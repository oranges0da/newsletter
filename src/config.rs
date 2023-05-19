use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub app_port: u16,
    pub db: DBSettings,
}

#[derive(Deserialize)]
pub struct DBSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
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
