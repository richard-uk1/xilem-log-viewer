use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct LogRecord {
    pub timestamp: OffsetDateTime,
    pub level: Level,
    pub message: String,
    pub target: String,
}

impl<'de> Deserialize<'de> for LogRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = LogRecordRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

/// A log record from
#[derive(Debug, Deserialize)]
struct LogRecordRaw {
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub level: Level,
    pub fields: Fields,
    pub target: String,
}

impl From<LogRecordRaw> for LogRecord {
    fn from(value: LogRecordRaw) -> Self {
        Self {
            timestamp: value.timestamp,
            level: value.level,
            message: value.fields.message,
            target: value.target,
        }
    }
}

/// Log level
#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Level {
    Error,
    Warn,
    Debug,
    Info,
    Trace,
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    pub message: String,
}
