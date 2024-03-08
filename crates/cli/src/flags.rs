#[derive(Debug, Default, clap::Args)]
pub struct GlobalFormatFlags {
    /// Enable JSON output, only supported on some commands
    #[arg(long, global = true, conflicts_with = "jsonl")]
    pub json: bool,
    /// Enable JSONL output, only supported on some commands
    #[arg(long, global = true, conflicts_with = "json")]
    pub jsonl: bool,
    #[cfg(feature = "remote_redis")]
    /// Enable Redis output, only supported on some commands
    #[arg(long, global = true, conflicts_with = "jsonl")]
    pub redis: bool,
    #[cfg(feature = "remote_pubsub")]
    /// Enable Google Cloud PubSub output, only supported on some commands
    #[arg(long, global = true)]
    pub pubsub: bool,
    /// Override the default log level (info)
    #[arg(long, global = true)]
    pub log_level: Option<log::LevelFilter>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OutputFormat {
    Standard,
    Json,
    Jsonl,
    #[cfg(feature = "remote_redis")]
    Redis,
    #[cfg(feature = "remote_pubsub")]
    PubSub,
    #[cfg(feature = "server")]
    Combined,
}

impl OutputFormat {
    /// Should the command always succeed, and should we show an error message?
    /// Returns (always_succeed, show_error)
    pub fn is_always_ok(&self) -> (bool, bool) {
        match self {
            OutputFormat::Standard => (false, false),
            OutputFormat::Json | OutputFormat::Jsonl => (true, true),
            #[cfg(feature = "remote_redis")]
            OutputFormat::Redis => (false, true),
            #[cfg(feature = "remote_pubsub")]
            OutputFormat::PubSub => (false, true),
            #[cfg(feature = "server")]
            OutputFormat::Combined => (false, true),
        }
    }
}

impl From<&GlobalFormatFlags> for OutputFormat {
    fn from(flags: &GlobalFormatFlags) -> Self {
        #[cfg(feature = "server")]
        if flags.pubsub && flags.redis {
            return OutputFormat::Combined;
        }
        #[cfg(feature = "remote_pubsub")]
        if flags.pubsub {
            return OutputFormat::PubSub;
        }
        #[cfg(feature = "remote_redis")]
        if flags.redis {
            return OutputFormat::Redis;
        }
        if flags.json {
            OutputFormat::Json
        } else if flags.jsonl {
            OutputFormat::Jsonl
        } else {
            OutputFormat::Standard
        }
    }
}

impl From<GlobalFormatFlags> for OutputFormat {
    fn from(flags: GlobalFormatFlags) -> Self {
        (&flags).into()
    }
}
