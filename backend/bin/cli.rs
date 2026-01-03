//! CLI tool driving the API client
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use log::{debug, info};
// models may be unused if all inputs are primitive types
#[allow(unused_imports)]
use openapi_client::{
    models, ApiNoContext, Client, ContextWrapperExt,
    CreateDailyTrackResponse,
    GetDailyTracksResponse,
    GetDailyTrackByIdResponse,
    CreateTopicResponse,
    GetTopicsResponse,
    GetTopicByIdResponse,
};
use simple_logger::SimpleLogger;
use swagger::{AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

#[derive(Parser, Debug)]
#[clap(
    name = "Daily Tracker Backend API",
    version = "1.0.0",
    about = "CLI access to Daily Tracker Backend API"
)]
struct Cli {
    #[clap(subcommand)]
    operation: Operation,

    /// Address or hostname of the server hosting this API, including optional port
    #[clap(short = 'a', long, default_value = "http://localhost")]
    server_address: String,

    /// Path to the client private key if using client-side TLS authentication
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[clap(long, requires_all(&["client_certificate", "server_certificate"]))]
    client_key: Option<String>,

    /// Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[clap(long, requires_all(&["client_key", "server_certificate"]))]
    client_certificate: Option<String>,

    /// Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[clap(long)]
    server_certificate: Option<String>,

    /// If set, write output to file instead of stdout
    #[clap(short, long)]
    output_file: Option<String>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(Parser, Debug)]
enum Operation {
    /// Create a new daily track record
    CreateDailyTrack {
        /// Daily track record to be created
        #[clap(value_parser = parse_json::<models::CreateDailyTrackRequest>)]
        body: models::CreateDailyTrackRequest,
    },
    /// Get daily track records
    GetDailyTracks {
        /// Filter records starting from this date (inclusive). Format: YYYY-MM-DD
        start_date: Option<chrono::naive::NaiveDate>,
        /// Filter records up to this date (inclusive). Format: YYYY-MM-DD
        end_date: Option<chrono::naive::NaiveDate>,
        /// Filter records by topic ID
        topic_id: Option<i64>,
    },
    /// Get a daily track record by ID
    GetDailyTrackById {
        /// ID of the daily track record to retrieve
        id: i64,
    },
    /// Create a new topic
    CreateTopic {
        /// Topic object to be created
        #[clap(value_parser = parse_json::<models::CreateTopicRequest>)]
        body: models::CreateTopicRequest,
    },
    /// Get all topics
    GetTopics {
        /// Filter topics by parent topic ID.
        parent_topic_id: Option<i64>,
    },
    /// Get a topic by ID
    GetTopicById {
        /// ID of the topic to retrieve
        id: i64,
    },
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
fn create_client(args: &Cli, context: ClientContext) -> Result<Box<dyn ApiNoContext<ClientContext>>> {
    if args.client_certificate.is_some() {
        debug!("Using mutual TLS");
        let client = Client::try_new_https_mutual(
            &args.server_address,
            args.server_certificate.clone().unwrap(),
            args.client_key.clone().unwrap(),
            args.client_certificate.clone().unwrap(),
        )
        .context("Failed to create HTTPS client")?;
        Ok(Box::new(client.with_context(context)))
    } else if args.server_certificate.is_some() {
        debug!("Using TLS with pinned server certificate");
        let client =
            Client::try_new_https_pinned(&args.server_address, args.server_certificate.clone().unwrap())
                .context("Failed to create HTTPS client")?;
        Ok(Box::new(client.with_context(context)))
    } else {
        debug!("Using client without certificates");
        let client =
            Client::try_new(&args.server_address).context("Failed to create HTTP(S) client")?;
        Ok(Box::new(client.with_context(context)))
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
fn create_client(args: &Cli, context: ClientContext) -> Result<Box<dyn ApiNoContext<ClientContext>>> {
    let client =
        Client::try_new(&args.server_address).context("Failed to create HTTP(S) client")?;
    Ok(Box::new(client.with_context(context)))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    if let Some(log_level) = args.verbosity.log_level() {
        SimpleLogger::new().with_level(log_level.to_level_filter()).init()?;
    }

    debug!("Arguments: {:?}", &args);

    let auth_data: Option<AuthData> = None;

    #[allow(trivial_casts)]
    let context = swagger::make_context!(
        ContextBuilder,
        EmptyContext,
        auth_data,
        XSpanIdString::default()
    );

    let client = create_client(&args, context)?;

    let result = match args.operation {
        Operation::CreateDailyTrack {
            body,
        } => {
            info!("Performing a CreateDailyTrack request");

            let result = client.create_daily_track(
                body,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                CreateDailyTrackResponse::DailyTrackRecordCreatedSuccessfully
                (body)
                => "DailyTrackRecordCreatedSuccessfully\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                CreateDailyTrackResponse::InvalidInput
                (body)
                => "InvalidInput\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                CreateDailyTrackResponse::ReferencedTopicNotFound
                (body)
                => "ReferencedTopicNotFound\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                CreateDailyTrackResponse::ARecordAlreadyExistsForThisTimePeriod
                (body)
                => "ARecordAlreadyExistsForThisTimePeriod\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                CreateDailyTrackResponse::InternalServerError
                (body)
                => "InternalServerError\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::GetDailyTracks {
            start_date,
            end_date,
            topic_id,
        } => {
            info!("Performing a GetDailyTracks request");

            let result = client.get_daily_tracks(
                start_date,
                end_date,
                topic_id,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetDailyTracksResponse::SuccessfulOperation
                (body)
                => "SuccessfulOperation\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                GetDailyTracksResponse::InvalidDateFormatOrParameters
                (body)
                => "InvalidDateFormatOrParameters\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                GetDailyTracksResponse::InternalServerError
                (body)
                => "InternalServerError\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::GetDailyTrackById {
            id,
        } => {
            info!("Performing a GetDailyTrackById request on {:?}", (
                &id
            ));

            let result = client.get_daily_track_by_id(
                id,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetDailyTrackByIdResponse::SuccessfulOperation
                (body)
                => "SuccessfulOperation\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                GetDailyTrackByIdResponse::DailyTrackRecordNotFound
                (body)
                => "DailyTrackRecordNotFound\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                GetDailyTrackByIdResponse::InternalServerError
                (body)
                => "InternalServerError\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::CreateTopic {
            body,
        } => {
            info!("Performing a CreateTopic request");

            let result = client.create_topic(
                body,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                CreateTopicResponse::TopicCreatedSuccessfully
                (body)
                => "TopicCreatedSuccessfully\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                CreateTopicResponse::InvalidInput
                (body)
                => "InvalidInput\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                CreateTopicResponse::TopicNameAlreadyExists
                (body)
                => "TopicNameAlreadyExists\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                CreateTopicResponse::InternalServerError
                (body)
                => "InternalServerError\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::GetTopics {
            parent_topic_id,
        } => {
            info!("Performing a GetTopics request");

            let result = client.get_topics(
                parent_topic_id,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetTopicsResponse::SuccessfulOperation
                (body)
                => "SuccessfulOperation\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                GetTopicsResponse::InternalServerError
                (body)
                => "InternalServerError\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::GetTopicById {
            id,
        } => {
            info!("Performing a GetTopicById request on {:?}", (
                &id
            ));

            let result = client.get_topic_by_id(
                id,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetTopicByIdResponse::SuccessfulOperation
                (body)
                => "SuccessfulOperation\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                GetTopicByIdResponse::TopicNotFound
                (body)
                => "TopicNotFound\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
                GetTopicByIdResponse::InternalServerError
                (body)
                => "InternalServerError\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
    };

    if let Some(output_file) = args.output_file {
        std::fs::write(output_file, result)?
    } else {
        println!("{}", result);
    }
    Ok(())
}

// May be unused if all inputs are primitive types
#[allow(dead_code)]
fn parse_json<T: serde::de::DeserializeOwned>(json_string: &str) -> Result<T> {
    serde_json::from_str(json_string).map_err(|err| anyhow!("Error parsing input: {}", err))
}
