mod args;

use std::error::Error;
use std::f32::consts::E;

use args::SearchCommand;
use clap::Parser;

use std::fs::File;
use std::io::copy;

use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_json;

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
            return download(download_command.id);
        }
    }
}

#[derive(Deserialize, Debug)]
struct Resource {
    name: String,
    tag: String,
    id: u32,
}

fn search(search_command: &SearchCommand) -> Result<(), Box<dyn Error>> {
    if search_command.query.len() == 0 {
        return Err("query must be specified. Check 'spigotfly search --help'".into());
    }

    let mut url = format!(
        "https://api.spiget.org/v2/search/resources/{}",
        search_command.query.join("%20")
    );
    if search_command.tag {
        url.push_str("?field=tag");
    }
    let resp = reqwest::blocking::get(url)?.json::<Vec<Resource>>()?;

    for resource in resp {
        println!("{} | {} | {}", resource.id, resource.name, resource.tag)
    }
    Ok(())
}

fn download(id: u32) -> Result<(), Box<dyn Error>> {
    let url = format!("https://api.spiget.org/v2/resources/{}/download", id);
    let resp = reqwest::blocking::get(url)?;

    match resp.status().as_u16() {
        302 => {
            return Err("File found Redirect to the file direct download OR the url of externally hosted resources".into());
        }

        404 => {
            return Err("Resource not found / File not Found".into());
        }
        _ => {}
    }

    // Apri un file per scrivere il contenuto del plugin
    let mut file = File::create(format!("{}.jar", id))?;

    // Copia il contenuto della risposta HTTP nel file
    let _ = copy(&mut resp.bytes()?.as_ref(), &mut file)?;

    Ok(())
}
