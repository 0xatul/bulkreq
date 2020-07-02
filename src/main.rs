use clap::{App, AppSettings, Arg};
use futures::stream::StreamExt;
use md5::digest::generic_array::typenum::U16;
use md5::digest::generic_array::GenericArray;
use md5::{Digest, Md5};
use reqwest::redirect;
use std::fs;
use std::io::{self, Read};
use std::time::Duration;
use tokio::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main(core_threads = 4)]
async fn main() -> Result<()> {
    let args = create_clap_app("v0.1.0");
    let matches = args.get_matches();
    let mut verbose = false;
    let timeout: u64 = matches.value_of("timeout").unwrap().parse()?;
    let mut urls: Vec<String> = Vec::new();

    if matches.is_present("verbose") {
        verbose = true;
    }

    if matches.is_present("file") {
        let input = matches.value_of("input").unwrap();
        let contents = fs::read_to_string(input)?;
        urls = contents.lines().map(|l| l.to_string()).collect();
    } else {
        urls = read_stdin()?;
    }

    concurrent_fetches(urls, verbose, timeout).await;
    Ok(())
}

fn create_clap_app(version: &str) -> clap::App {
    // Add support to not include subdomains.
    let app = App::new("bulkreq")
        .version(version)
        .about("Make lots of requests quickly")
        .usage("bulkreq -f <list_of_urls> or bulkreq < /path/to/urls.txt")
        .arg(Arg::with_name("input").index(1).required(false))
        .arg(
            Arg::with_name("file")
                .help("read urls from a file")
                .short("f")
                .long("file"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("print some extra information")
                .short("v")
                .long("verbose"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("set timeout (default: 3)")
                .short("t")
                .long("timeout")
                .takes_value(true)
                .required(false)
                .default_value("3"),
        );

    app
}

fn md5_hash(input: &[u8]) -> GenericArray<u8, U16> {
    let mut hasher = Md5::new();
    hasher.input(input);
    hasher.result()
}

async fn fetch(url: String, verbose: bool, timeout:u64) -> Result<String> {
    let custom = redirect::Policy::none();
    let client = reqwest::Client::builder().redirect(custom).timeout(Duration::from_secs(timeout)).build()?;
    let resp = client.get(&url).send().await?;
    let status = resp.status();
    let headers = resp.headers().clone();

    let mime = if headers.contains_key("Content-Type") {
        let ct = &headers["Content-Type"];
        &ct.to_str().unwrap()
    } else {
        "None"
    };

    let protocol = resp.version();
    let bytes = resp.bytes().await?;
    let hash = md5_hash(&bytes);
    let serv = if headers.contains_key("Server") {
        &headers["Server"].to_str().unwrap()
    } else {
        "unknown"
    };

    if verbose {
        if headers.contains_key("Location") {
            let redir = &headers["Location"].to_str().unwrap();
            println!(
                "{} {} {} {:?} {} {:x} --> {}",
                &status, &url, &mime, &protocol, &serv, &hash, &redir
            )}
        else {
            println!(
                "{} {} {} {:?} {} {:x}",
                &status, &url, &mime, &protocol, &serv, &hash,
            );
        }
    }
        else {
        if headers.contains_key("Location"){
            let redir = &headers["Location"].to_str().unwrap();
            println!("{}\t{} --> {}",&status,&url,&redir)
        }
            else {
                println!("{}\t{}",&status,&url);
            }
        }

    // just debug that we are actually running multiple threads and tasks per thread.
    let res = format!(
        "current thread {:?} | thread name {}",
        std::thread::current().id(),
        std::thread::current()
            .name()
            .get_or_insert("default_thread_name"),
    );
    
    Ok(res)
}

async fn concurrent_fetches(urls: Vec<String>, verbose: bool, timeout: u64) {
    const ACTIVE_REQUESTS: usize = 100;
    let _before = Instant::now();
    // here we turn our urls into a stream of futures, and spawn a task for each of of the urls
    // at a limit of 2 requests at a time. You can think of tokio::tasks kinda like goroutines
    let responses = futures::stream::iter(
        urls.into_iter()
            .map(|url| tokio::spawn(async move { fetch(url.to_string(), verbose, timeout).await })),
    )
    .buffer_unordered(ACTIVE_REQUESTS) // this is your concurrency threshold
    .map(|_r| {
        /*println!(
          "finished request: {}",
          match r.unwrap() {
          Ok(rr) => rr,
          Err(_) => String::from("Bad"),
        }
        );*/
    })
    .collect::<Vec<_>>();
    responses.await;
}

fn read_stdin() -> Result<Vec<String>> {
    let mut buffer = String::new();
    let mut res = Vec::new();
    io::stdin().read_to_string(&mut buffer)?;
    for line in buffer.split_whitespace() {
        res.push(line.to_string())
    }
    Ok(res)
}
