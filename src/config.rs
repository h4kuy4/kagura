use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub chat_id: i64,
    pub ikuai_address: String,
    pub ikuai_username: String,
    pub ikuai_password: String,
    pub cloudflare_api_token: String,
    pub cloudflare_zone_id: String,
}

impl Config {
    pub fn from_env() -> Result<Config, String> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(|err| err.to_string())?;

        let cfg: Config = cfg.try_deserialize().map_err(|err| err.to_string())?;

        Ok(cfg)
    }
}
