use hickory_resolver::TokioResolver;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::proto::rr::rdata::MX;
use lettre::AsyncTransport;
use lettre::Message;
use lettre::AsyncSmtpTransport;
use lettre::Tokio1Executor;

pub async fn send_verification_email(to: &str, username: &str, code: &str) -> Result<(), String> {
    let sender = std::env::var("SENDER_EMAIL")
        .unwrap_or_else(|_| "noreply@dailytracker.app".to_string());

    let domain = to
        .rsplit_once('@')
        .map(|(_, d)| d)
        .ok_or_else(|| format!("Invalid email address: {}", to))?;

    // Resolve MX records
    let resolver = TokioResolver::builder_with_config(
        ResolverConfig::default(),
        TokioConnectionProvider::default(),
    )
    .build();
    let mx_lookup = resolver
        .mx_lookup(domain)
        .await
        .map_err(|e| format!("MX lookup failed for {}: {}", domain, e))?;

    let mx_host = mx_lookup
        .iter()
        .min_by_key(|mx: &&MX| mx.preference())
        .ok_or_else(|| format!("No MX records found for {}", domain))?
        .exchange()
        .to_ascii();

    // Build the email
    let email = Message::builder()
        .from(sender.parse().map_err(|e| format!("Invalid sender address: {}", e))?)
        .to(to.parse().map_err(|e| format!("Invalid recipient address: {}", e))?)
        .subject("Your verification code")
        .body(format!(
            "Hi {},\n\nYour email verification code is: {}\n\nThis code expires in 30 minutes.\n",
            username, code
        ))
        .map_err(|e| format!("Failed to build email: {}", e))?;

    // Connect directly to the MX host on port 25
    let transport: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&mx_host)
            .port(25)
            .build();

    transport
        .send(email)
        .await
        .map_err(|e| format!("Failed to send email to {}: {}", mx_host, e))?;

    Ok(())
}
