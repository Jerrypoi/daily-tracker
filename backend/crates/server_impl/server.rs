
#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use hyper::service::{service_fn, Service};
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

use diesel::prelude::*;
use db_model::establish_connection;
use db_model::models::DailyTrack;
use db_model::schema::daily_track;
use std::convert::TryInto;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use openapi::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr: SocketAddr = addr.parse().expect("Failed to parse bind address");
    let listener = TcpListener::bind(&addr).await.unwrap();

    let server = Server::new();

    let service = MakeService::new(server);
    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        openapi::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();

            info!("Starting a server (with https)");
            loop {
                if let Ok((tcp, addr)) = listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        http1::Builder::new()
                            .serve_connection(TokioIo::new(tls), service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        info!("Starting a server (over http, so no TLS)");
        println!("Listening on http://{}", addr);

        loop {
            // When an incoming TCP connection is received grab a TCP stream for
            // client<->server communication.
            //
            // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
            // .await point allows the Tokio runtime to pull the task off of the thread until the task
            // has work to do. In this case, a connection arrives on the port we are listening on and
            // the task is woken up, at which point the task is then put back on a thread, and is
            // driven forward by the runtime, eventually yielding a TCP stream.
            let (tcp_stream, addr) = listener.accept().await.expect("Failed to accept connection");

            let service = service.call(addr).await.unwrap();
            let io = TokioIo::new(tcp_stream);
            // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
            // current task without waiting for the processing of the HTTP1 connection we just received
            // to finish
            tokio::task::spawn(async move {
                // Handle the connection from the client using HTTP1 and pass any
                // HTTP requests received on that connection to the `hello` function
                let result = http1::Builder::new()
                    .serve_connection(io, service)
                    .await;
                if let Err(err) = result
                {
                    println!("Error serving connection: {err:?}");
                }
            });
        }
    }
}

// #[derive(Copy)]
pub struct Server<C> {
    marker: PhantomData<C>,
    connection: Arc<Mutex<MysqlConnection>>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        // add database connection here
        let connection = establish_connection();
        Server{marker: PhantomData, connection: Arc::new(Mutex::new(connection))}
    }
}

impl<C> Clone for Server<C> {
    fn clone(&self) -> Self {
        Self {
            marker: PhantomData,
            connection: self.connection.clone(),
        }
    }
}


use jsonwebtoken::{decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use swagger::auth::Authorization;
use crate::server_auth;


use openapi::{
    Api,
    CreateDailyTrackResponse,
    GetDailyTracksResponse,
    GetDailyTrackByIdResponse,
    CreateTopicResponse,
    GetTopicsResponse,
    GetTopicByIdResponse,
};
use openapi::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Create a new daily track record
    async fn create_daily_track(
        &self,
        body: models::CreateDailyTrackRequest,
        context: &C) -> Result<CreateDailyTrackResponse, ApiError>
    {
        info!("create_daily_track({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        
        // Convert API types to database types
        // Convert i64 topic_id to Vec<u8> (8 bytes for UUID compatibility)
        let topic_id_bytes = if body.topic_id != 0 {
            Some(body.topic_id.to_le_bytes().to_vec())
        } else {
            None
        };
        
        // Convert DateTime<Utc> to NaiveDateTime
        let start_time_naive = body.start_time.naive_utc();
        // Lock the database connection
        let mut conn = self.connection.lock()
            .map_err(|e| ApiError(format!("Failed to lock database connection: {}", e)))?;
        
        // Create a new DailyTrack instance (this generates a UUID)
        let new_track = DailyTrack::new(
            start_time_naive,
            topic_id_bytes,
            body.comment.clone(),
        );
        // Insert into database
        diesel::insert_into(daily_track::table)
            .values(&new_track)
            .execute(&mut *conn).unwrap();
            // .map_err(|e| ApiError(format!("Database error: {}", e)))?;
        
        // Convert database model back to API model
        // Convert UUID (Vec<u8>) to i64 by taking first 8 bytes
        let id_as_i64 = if new_track.id.len() >= 8 {
            i64::from_le_bytes(
                new_track.id[0..8].try_into()
                    .map_err(|_| ApiError("Failed to convert UUID to i64".into()))?
            )
        } else {
            return Err(ApiError("Invalid UUID format".into()));
        };
        
        // Convert topic_id back from Vec<u8> to i64
        let topic_id_as_i64 = new_track.topic_id
            .and_then(|bytes| {
                if bytes.len() >= 8 {
                    Some(i64::from_le_bytes(
                        bytes[0..8].try_into().ok()?
                    ))
                } else {
                    None
                }
            });
        
        // Convert NaiveDateTime back to DateTime<Utc>
        let created_at_utc = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            new_track.created_at,
            chrono::Utc
        );
        let updated_at_utc = new_track.updated_at
            .map(|naive| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc));
        let start_time_utc = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            new_track.start_time,
            chrono::Utc
        );
        
        let data = CreateDailyTrackResponse::DailyTrackRecordCreatedSuccessfully(models::DailyTrack {
            id: id_as_i64,
            start_time: start_time_utc,
            topic_id: topic_id_as_i64.unwrap_or(0),
            comment: new_track.comment,
            created_at: created_at_utc,
            updated_at: updated_at_utc.unwrap_or(created_at_utc),
        });
        
        Ok(data)
    }

    /// Get daily track records
    async fn get_daily_tracks(
        &self,
        start_date: Option<chrono::naive::NaiveDate>,
        end_date: Option<chrono::naive::NaiveDate>,
        topic_id: Option<i64>,
        context: &C) -> Result<GetDailyTracksResponse, ApiError>
    {
        info!("get_daily_tracks({:?}, {:?}, {:?}) - X-Span-ID: {:?}", start_date, end_date, topic_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get a daily track record by ID
    async fn get_daily_track_by_id(
        &self,
        id: i64,
        context: &C) -> Result<GetDailyTrackByIdResponse, ApiError>
    {
        info!("get_daily_track_by_id({}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Create a new topic
    async fn create_topic(
        &self,
        body: models::CreateTopicRequest,
        context: &C) -> Result<CreateTopicResponse, ApiError>
    {
        info!("create_topic({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get all topics
    async fn get_topics(
        &self,
        parent_topic_id: Option<i64>,
        context: &C) -> Result<GetTopicsResponse, ApiError>
    {
        info!("get_topics({:?}) - X-Span-ID: {:?}", parent_topic_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get a topic by ID
    async fn get_topic_by_id(
        &self,
        id: i64,
        context: &C) -> Result<GetTopicByIdResponse, ApiError>
    {
        info!("get_topic_by_id({}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

}
