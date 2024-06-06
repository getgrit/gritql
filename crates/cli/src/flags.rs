#[derive(Debug, Default, clap::Args, Clone)]
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
    /// Override the default .grit directory location
    #[arg(long, global = true)]
    pub grit_dir: Option<std::path::PathBuf>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OutputFormat {
    Standard,
    /// Print every transformed file back out in full, with no other output
    Transformed,
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
    /// Gets the OutputFormat from the GlobalFormatFlags
    /// A default should be provided based on other CLI flags
    pub fn from_flags(flags: &GlobalFormatFlags, default: OutputFormat) -> Self {
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
            default
        }
    }
}

impl OutputFormat {
    /// Should the command always succeed, and should we show an error message?
    /// Returns (always_succeed, show_error)
    pub fn is_always_ok(&self) -> (bool, bool) {
        match self {
            OutputFormat::Standard => (false, false),
            OutputFormat::Transformed => (false, false),
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
        OutputFormat::from_flags(flags, OutputFormat::Standard)
    }
}

impl From<GlobalFormatFlags> for OutputFormat {
    fn from(flags: GlobalFormatFlags) -> Self {
        (&flags).into()
    }
}
