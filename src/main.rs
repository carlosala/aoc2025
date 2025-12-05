use chrono::Datelike;
use clap::{Args, Parser};
use std::io::Read;

mod solutions;
extern crate lazy_static;

#[derive(Parser, Debug)]
enum CLIArgs {
    Run(RunCommand),
    Download(DownloadCommand),
}

#[derive(Parser, Debug)]
struct RunCommand {
    day: Option<usize>,
    #[clap(long, short, default_value_t = 3)]
    part: usize,
    #[clap(long, short)]
    test: bool,
}

#[derive(Args, Debug)]
struct DownloadCommand {
    day: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let args = CLIArgs::parse();

    match args {
        CLIArgs::Run(c) => {
            let day = get_day(c.day);
            solutions::solve(day, c.part, c.test);

            Ok(())
        }
        CLIArgs::Download(c) => download(get_day(c.day)).await,
    }
}

async fn download(selected_day: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();

    let token = std::env::var("TOKEN");
    if token.is_err() {
        panic!("Missing TOKEN env variable");
    }
    let cookie = format!("session={}", token?);
    headers.insert("cookie", reqwest::header::HeaderValue::from_str(&cookie)?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let body = client
        .get(format!(
            "https://adventofcode.com/2025/day/{selected_day}/input"
        ))
        .send()
        .await?
        .text()
        .await?;

    if body.contains("Please don't repeatedly request this endpoint") {
        println!("Day {:02} not yet ready", selected_day);
        return Ok(());
    }

    std::fs::write(format!("inputs/{:02}", selected_day), body)?;

    if copy_template(selected_day)? {
        append_day(selected_day)?;
    }

    Ok(())
}

fn copy_template(selected_day: usize) -> Result<bool, std::io::Error> {
    if std::fs::exists(format!("src/solutions/day{:02}.rs", selected_day)).unwrap() {
        println!(
            "PATH src/solutions/day{:02}.rs already exists",
            selected_day
        );
        return Ok(false);
    }
    match std::fs::copy(
        "src/solutions/template.rs",
        format!("src/solutions/day{:02}.rs", selected_day),
    ) {
        Ok(_) => return Ok(true),
        Err(e) => return Err(e),
    }
}

fn append_day(selected_day: usize) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open("src/solutions/mod.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let new_mod = format!("mod day{:02};", selected_day);
    contents = contents.replace("mod solver;", &format!("{new_mod}\nmod solver;"));

    let new_match = format!(
        "{selected_day} => day{:02}::Problem.solve(filename, parts),",
        selected_day
    );
    contents = contents.replace("_ => panic!", &format!("{new_match}\n        _ => panic!"));

    std::fs::write("src/solutions/mod.rs", contents)
}

fn get_day(day: Option<usize>) -> usize {
    let default_day = chrono::Utc::now().day() as usize;
    day.unwrap_or(default_day)
}
