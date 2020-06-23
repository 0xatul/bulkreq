use futures::stream::{StreamExt};
use md5::digest::generic_array::typenum::U16;
use md5::digest::generic_array::GenericArray;
use md5::{Digest, Md5};
use reqwest::redirect;
use tokio::time::Instant;
use std::io::{self, Read};
//big thing :upside_down:
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main(core_threads = 4)]
async fn main() -> Result<()> {
    concurrent_fetches().await;
    Ok(())
}

fn md5_hash(input: &[u8]) -> GenericArray<u8, U16> {
    let mut hasher = Md5::new();
    hasher.input(input);
    hasher.result()
}

async fn fetch(url: String) -> Result<String> {
  //println!("Started Request");
  let custom = redirect::Policy::none();
  let client = reqwest::Client::builder()
    .redirect(custom)
    .build()?;
  let resp = client.get(&url).send().await?;
  let status = resp.status();
  // if we don't clone the headermap, when we call resp.bytes().await?
  // the ownership of the resp will be moved and dropped, then we would
  // have a the mime value ponting to nothing.
  let headers = resp.headers().clone();
  //let mime = ct.to_str().unwrap();
  let mime = if headers.contains_key("Content-Type"){
    let ct = &headers["Content-Type"];
    &ct.to_str().unwrap()
  } else {
    "None"
  };

  let protocol = resp.version();
  let bytes = resp.bytes().await?;
  let hash = md5_hash(&bytes);
  let serv = if headers.contains_key("Server"){
    &headers["Server"].to_str().unwrap()
  } else {
    "unknown"
  };
  if headers.contains_key("Location") {
    let redir = &headers["Location"].to_str().unwrap();
    println!("{} {} {} {:?} {} {:x} --> {}",
             &status, &url, &mime, &protocol, &serv, &hash, &redir

    )
  } else {
  println!("{} {} {} {:?} {} {:x}",
           &status, &url, &mime, &protocol,&serv, &hash,
         
  );
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


async fn concurrent_fetches() {
    // in my other code i set this to like 50 or sometimes 250 for waybackurls
    // because im fetching over 700 pages sometimes.
    const ACTIVE_REQUESTS: usize = 100;
    let _before = Instant::now();
    let urls = read_stdin().unwrap();
//    let urls = vec![
//        "https://hackerone.com",
//        "https://google.com",
//        "http://google.com",
//    ];

    // here we turn our urls into a stream of futures, and spawn a task for each of of the urls
    // at a limit of 2 requests at a time. You can think of tokio::tasks kinda like goroutines
  let responses = futures::stream::iter(
    urls.into_iter()
        .map(|url| tokio::spawn(async move { fetch(url.to_string()).await })),
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

   // println!("elapsed time: {:.2?}", _before.elapsed());
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
