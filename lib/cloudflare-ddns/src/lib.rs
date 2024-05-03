mod model;

use reqwest::Client;
use serde_json::json;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
pub struct CloudflareClient {
    token: String,
    zone: String,
    client: Client,
}

impl CloudflareClient {
    pub fn new(token: &str, zone: &str) -> Result<Self> {
        let client = Client::new();

        Ok(Self {
            token: String::from(token),
            zone: String::from(zone),
            client,
        })
    }

    pub async fn get_record_id_by_name(&self, name: &str, t: &str) -> Result<Vec<model::Record>> {
        let resp: model::Response = self
            .client
            .get(format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}&type={}",
                self.zone, name, t
            ))
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", &self.token),
            )
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())?;

        if !resp.success {
            return Err(String::from("Record Not Found"));
        }

        Ok(resp.result)
    }

    pub async fn update_ip(&self, name: &str, t: &str, address: &[String]) -> Result<()> {
        let records = self.get_record_id_by_name(name, t).await?;

        for i in 0..records.len() {
            if i >= address.len() {
                break;
            }
            let req = json!({
                "content": &address[i],
                "name": &name,
                "type": &t
            });

            self.client
                .put(format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                    self.zone, records[i].id
                ))
                .header(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", &self.token),
                )
                .body(req.to_string())
                .send()
                .await
                .map_err(|err| err.to_string())?;
        }

        Ok(())
    }
}
