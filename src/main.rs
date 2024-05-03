mod config;

use teloxide::prelude::*;
use teloxide::types::Recipient;
use tokio::sync::mpsc;
use tokio_schedule::Job;

async fn update_dns(
    tx: mpsc::Sender<String>,
    ikuai_address: &str,
    ikuai_username: &str,
    ikuai_password: &str,
    cloudflare_token: &str,
    cloudflare_zone_id: &str,
) {
    log::info!("Updating DNS.");
    let ikuai_client =
        match ikuai::IkuaiClient::login(ikuai_address, ikuai_username, ikuai_password).await {
            Ok(v) => v,
            Err(e) => {
                log::error!("IKuai login failed.");
                let _ = tx.send(e.to_string()).await;
                return ();
            }
        };

    let address = match ikuai_client.get_wan_ip().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Get WAN IP failed.");
            let _ = tx.send(e.to_string()).await;
            return ();
        }
    };

    let cloudflare_client =
        match cloudflare_ddns::CloudflareClient::new(cloudflare_token, cloudflare_zone_id) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Get cloudflare failed.");
                let _ = tx.send(e.to_string()).await;
                return ();
            }
        };

    match cloudflare_client
        .update_ip("dom.hakuya.moe", "A", &address)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            log::error!("Update IPv4 DNS failed.");
            let _ = tx.send(e.to_string()).await;
            return ();
        }
    };

    match cloudflare_client
        .update_ip("v4.dom.hakuya.moe", "A", &address)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            log::error!("Update IPv4 DNS failed.");
            let _ = tx.send(e.to_string()).await;
            return ();
        }
    };

    log::info!("Update IPv4 DDNS success");
    let _ = tx.send(String::from("Update IPv4 DDNS success")).await;

    let resp = match reqwest::get("https://api6.ipify.org").await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Get IPv6 address failed.");
            let _ = tx.send(e.to_string()).await;
            return ();
        }
    };
    let address = match resp.text().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Parse IPv6 address failed.");
            let _ = tx.send(e.to_string()).await;
            return ();
        }
    };

    let address: Vec<String> = address.split("\n").map(|s| String::from(s)).collect();

    match cloudflare_client
        .update_ip("dom.hakuya.moe", "AAAA", &address)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            log::error!("Update IPv6 DNS failed.");
            let _ = tx.send(e.to_string()).await;
            return ();
        }
    };

    match cloudflare_client
        .update_ip("v6.dom.hakuya.moe", "AAAA", &address)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            log::error!("Update IPv6 DNS failed.");
            let _ = tx.send(e.to_string()).await;
            return ();
        }
    };

    log::info!("Update IPv6 DDNS success");
    let _ = tx.send(String::from("Update IPv6 DDNS success")).await;
}

#[tokio::main]
async fn main() -> Result<(), String> {
    env_logger::init();
    dotenv::dotenv().ok();
    let res = dotenv::from_path("/etc/kagura.conf");
    if let Err(_) = res {
        log::error!("File \"/etc/kagura.conf\" not found.");
    }

    let configs = config::Config::from_env()?;

    log::info!("Init finished.");

    let bot = Bot::from_env();
    let (tx, mut rx) = mpsc::channel(512);

    let tg_task = tokio::spawn(async move {
        loop {
            let message = tokio::select! {
                message = rx.recv() => message,
                else => {break}
            };

            if let Some(message) = message {
                log::debug!("Recv message: {}", message);
                let res = bot
                    .send_message(Recipient::from(ChatId(configs.chat_id.clone())), message)
                    .await;

                if let Err(e) = res {
                    log::error!("Send message failed: {}", e);
                }
            }
        }
    });

    let update_dns_task = tokio_schedule::every(1)
        .day()
        .at(20, 0, 0)
        .in_timezone(&chrono::Utc)
        .perform(|| async {
            update_dns(
                tx.clone(),
                &configs.ikuai_address,
                &configs.ikuai_username,
                &configs.ikuai_password,
                &configs.cloudflare_api_token,
                &configs.cloudflare_zone_id,
            )
            .await
        });

    update_dns_task.await;
    let _ = tg_task.await;

    Ok(())
}
