extern crate clap;
use clap::Clap;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::time::Instant;
use url::{ParseError, Url};

/// Cloudflare systems assignment
#[derive(Clap)]
#[clap(version = "1.0", author = "Jay Madden Cox <jaymaddencox@gmail.com>")]
struct Opts {
    /// Url to profile
    #[clap(short, long)]
    url: String,

    /// Amount of times to profile a given URL
    #[clap(short, long)]
    profile: Option<i32>,
}

#[derive(Debug)]
struct ReqTime {
    time: u128,
    response_size: u64,
    err_code: i32,
}

fn main() -> Result<(), ParseError> {
    let opts = Opts::parse();

    let url = Url::parse(&opts.url)?;

    let host = url.host_str().unwrap().to_owned();
    let port = String::from("80");
    let endpoint = url.path().to_owned();

    let profile = opts.profile.unwrap_or(1);

    let mut requests: Vec<ReqTime> = Vec::new();
    let mut failed_requests = 0;

    for i in 1..=profile {
        let t = Instant::now();
        let (size, code) = match make_req(i, &host, &port, &endpoint) {
            Ok(m) => m,
            Err(_) => {
                failed_requests += 1;
                continue;
            }
        };
        let elapsed = t.elapsed().as_millis();
        let req = ReqTime {
            time: elapsed,
            err_code: code,
            response_size: size as u64,
        };
        requests.push(req);
    }

    println!("{} Requests made to: {}\n", profile, &opts.url);

    let times = requests.iter().map(|x| x.time);
    let avg = times.clone().sum::<u128>() as f64 / requests.len() as f64;
    println!("The Average request time was: {}ms \n", avg);

    let min = times.clone().min();
    println!("The lowest request time: {}ms\n", min.unwrap_or(0));

    let max = times.clone().max();
    println!("The highest request time: {}ms\n", max.unwrap_or(0));

    if times.clone().len() > 0 {
        let mut sorted_list = times.clone().collect::<Vec<u128>>();
        sorted_list.sort();
        let med = sorted_list[sorted_list.len() / 2];
        println!("The median request times is {}ms\n", med);
    }

    let sizes = requests.iter().map(|x| x.response_size);
    let min = sizes.clone().min();
    println!("The lowest request size: {}bytes\n", min.unwrap_or(0));

    let max = sizes.clone().max();
    println!("The highest request size: {}bytes\n", max.unwrap_or(0));

    let err_codes = requests
        .iter()
        .filter(|x| x.err_code != 200)
        .map(|x| x.err_code)
        .collect::<Vec<i32>>();
    println!("Non 200 error codes: {:?}\n", err_codes);

    println!("Number of failed requests: {}\n", failed_requests);

    return Ok(());
}

fn make_req(
    req_num: i32,
    url: &String,
    port: &String,
    endpoint: &String,
) -> Result<(usize, i32), std::io::Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", url, port))?;
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;

    let req_data = format!("GET {end} HTTP/1.0\r\nHost: {host}\r\ncontent-type: text/plain;charset=UTF-8\r\nAccept: */*\r\n\r\n ",
        host=url, end=endpoint);

    println!("Sending request number {}: {}", req_num, req_data);

    stream.write_all(req_data.as_bytes())?;

    let mut buf = String::new();
    let result = stream.read_to_string(&mut buf)?;

    let ret_code = buf.split("\n").collect::<Vec<&str>>()[0]
        .split(" ")
        .collect::<Vec<&str>>()[1];

    return Ok((result, ret_code.parse::<i32>().unwrap()));
}
