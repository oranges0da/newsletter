use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub app_port: u16,
    pub db_settings: DBSettings,
}

#[derive(Deserialize)]
pub struct DBSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DBSettings {
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn get_database_url_without_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    // init our configuration reader
    let settings = config::Config::builder()
        // add configuration values from a file named `config.yaml`.
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()?;
    // try to convert the configuration values it read into our Settings struct
    settings.try_deserialize::<Settings>()
}
