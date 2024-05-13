use std::ops::RangeInclusive;

use clap::{
    Args, Parser, Subcommand, ValueEnum
};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SpigotflyArgs {
    
    #[clap(subcommand)]
    pub command : SpigotflyCommand,
}

#[derive(Debug, Subcommand)]
pub enum SpigotflyCommand {
    /// Search a resource
    Search(SearchCommand),

    /// Download a resource by id
    Download(DownloadCommand)

}

/// Custom validator for the query field
fn validate_query(s: &str) -> Result<String, String> {
    println!("validating '{}'", s);
    let query : Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
    if query.len() > 0 {
        println!("returning");
        Ok("".to_string())
    } else {
        Err(String::from("The query must contain at least one word."))
    }
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct SearchCommand {
    // /// Text to be searched
    pub query : Vec<String>,

    /// Field to search in.
    #[clap(short, long, value_enum, default_value = "name")]
    pub field : Field,

    /// Size of the returned resources
    #[clap(short, long, default_value_t = 10)]
    pub size : u8,

    /// Page index
    #[clap(short, long, default_value_t = 0)]
    pub page : u8,
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

    /// Text to be searched
    pub id : u32
}
