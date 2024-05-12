use clap::{builder::Str, Args, Parser, Subcommand};

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
pub struct SearchCommand {
    /// Search by tag
    #[clap(long, short, action)]
    pub tag: bool,

    /// Text to be searched
    pub query: Vec<String>,
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    /// Text to be searched
    pub id: u32,
}
