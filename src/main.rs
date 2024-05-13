mod args;

use std::error::Error;

use args::DownloadCommand;
use tabled::{builder::Builder, settings::Style};

use args::SearchCommand;
use clap::Parser;
use reqwest::blocking::Response;
use reqwest::header::USER_AGENT;

use std::fs::File;
use std::io::copy;

use serde_derive::Deserialize;

use args::SpigotflyArgs;
use args::SpigotflyCommand;

fn main() {
    let args = SpigotflyArgs::parse();

    if let Err(err) = execute(&args) {
        println!("ERROR: {}", err);
    }
}

fn execute(args: &SpigotflyArgs) -> Result<(), Box<dyn Error>> {
    match &args.command {
        SpigotflyCommand::Search(search_command) => {
            return search(&search_command);
        }
        SpigotflyCommand::Download(download_command) => {
            return download(&download_command);
        }
    }
}

#[derive(Deserialize, Debug)]
struct Resource {
    id: u32,
    name: String,
    downloads: u64,
    rating: ResourceRating,
}

#[derive(Deserialize, Debug)]
struct ResourceRating {
    count: u32,
    average: f64,
}

#[derive(Deserialize, Debug)]
struct AuthorDetails {
    name: String,
}

fn new_request(url: &String) -> Result<Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client.get(url).header(USER_AGENT, "spigotfly").send()
}

fn search(search_command: &SearchCommand) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://api.spiget.org/v2/search/resources/{}?field={}&size={}&page={}",
        search_command.query.join("%20"),
        search_command.field.as_text(),
        search_command.size,
        search_command.page,
    );

    let mut resp = new_request(&url)?.json::<Vec<Resource>>()?;

    // sort by downloads amount
    resp.sort_by(|a, b| b.downloads.cmp(&a.downloads));

    let mut builder = Builder::new();

    builder.push_record(["Id", "Name", "Downloads", "Rating"]);
    for resource in resp {
        // let url = format!( "https://api.spiget.org/v2/authors/{}", resource.author.id);
        // let author = new_request(&url)?.json::<AuthorDetails>()?;
        builder.push_record([
            resource.id.to_string(),
            truncate(&resource.name, 50).to_string(),
            resource.downloads.to_string(),
            "O".repeat(resource.rating.average.round() as usize),
        ]);
    }
    let table = builder.build().with(Style::ascii_rounded()).to_string();
    println!("{}", table);

    Ok(())
}

fn download(download_command : &DownloadCommand) -> Result<(), Box<dyn Error>> {
    let url = format!("https://api.spiget.org/v2/resources/{}/download", download_command.id);
    let resp = new_request(&url)?;

    match resp.status().as_u16() {
        302 => {
            return Err("File found Redirect to the file direct download OR the url of externally hosted resources".into());
        }

        404 => {
            return Err("Resource not found / File not Found".into());
        }
        _ => {}
    }

    let file_name = match &download_command.output {
        Some(output) => format!("{}.jar", output),
        None => format!("{}.jar", download_command.id)
    };
    
    let mut file = File::create(file_name)?;

    // Copia il contenuto della risposta HTTP nel file
    let _ = copy(&mut resp.bytes()?.as_ref(), &mut file)?;

    Ok(())
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
