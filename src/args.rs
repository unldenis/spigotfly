use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SpigotflyArgs {
    #[clap(subcommand)]
    pub command: SpigotflyCommand,
}

#[derive(Debug, Subcommand)]
pub enum SpigotflyCommand {
    /// Search a resource
    Search(SearchCommand),

    /// Download a resource by id
    Download(DownloadCommand),
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct SearchCommand {
    // /// Text to be searched
    pub query: Vec<String>,

    /// Field to search in.
    #[clap(short, long, value_enum, default_value = "name")]
    pub field: Field,

    /// Size of the returned resources
    #[clap(short, long, default_value_t = 10)]
    pub size: u8,

    /// Page index
    #[clap(short, long, default_value_t = 0)]
    pub page: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Field {
    /// Search by resource name
    Name,
    /// Search by resource tag
    Tag,
}

impl Field {
    pub fn as_text(&self) -> &str {
        match *self {
            Field::Name => "name",
            Field::Tag => "tag",
        }
    }
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    /// Resource id to download
    pub id: u32,

    /// Plugin jar file name. The id is the default
    #[clap(short, long)]
    pub output: Option<String>,
}
