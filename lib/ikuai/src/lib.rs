mod model;

use base64::engine::general_purpose::STANDARD as base64;
use base64::Engine;
use reqwest::Client;
use serde_json::json;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
pub struct IkuaiClient {
    address: String,
    cookie: String,
    client: Client,
}

impl IkuaiClient {
    pub async fn login(address: &str, username: &str, password: &str) -> Result<IkuaiClient> {
        let req = json!({
            "username": &username,
            "passwd": format!("{:x}", md5::compute(&password)),
            "pass": base64.encode(format!("salt_11{}", &password)),
            "remember_password": ""
        });

        let client = Client::new();
        let res = client
            .post(format!("http://{}/Action/login", address))
            .body(req.to_string())
            .send()
            .await
            .map_err(|err| err.to_string())?;

        let cookie = res
            .headers()
            .get("Set-Cookie")
            .ok_or(String::from("Login Failed"))?
            .to_str()
            .map_err(|err| err.to_string())?;

        Ok(IkuaiClient {
            address: String::from(address),
            cookie: String::from(cookie),
            client,
        })
    }

    pub async fn get_wan_ip(&self) -> Result<Vec<String>> {
        let req = json!({
            "action": "show",
            "func_name": "lan",
            "param": {
                "TYPE": "ether_info,snapshoot"
            }
        });

        let resp = self
            .client
            .post(format!("http://{}/Action/call", &self.address))
            .header(reqwest::header::COOKIE, &self.cookie)
            .body(req.to_string())
            .send()
            .await
            .map_err(|err| err.to_string())?;

        let resp: model::Response = resp.json().await.map_err(|err| err.to_string())?;

        let mut ret: Vec<String> = Vec::new();

        for wan in resp.data.snapshoot_wan {
            if wan.ip_addr.starts_with("10") {
                continue;
            }

            if wan.ip_addr == "" {
                continue;
            }

            ret.push(wan.ip_addr);
        }

        Ok(ret)
    }
}
