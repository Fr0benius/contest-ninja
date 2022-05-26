mod tester;

use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::{error, info};
use serde::Deserialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str;

#[derive(Debug, Deserialize)]
struct TestCase {
    input: String,
    output: String,
}

#[derive(Debug, Deserialize)]
struct Problem {
    name: String,
    #[allow(unused)]
    group: String,
    #[allow(unused)]
    url: String,
    tests: Vec<TestCase>,
}

async fn listen(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let full_body = hyper::body::to_bytes(req.into_body()).await?;
    let data = str::from_utf8(&full_body).unwrap();
    println!("{}", data);
    match parse_problem(&full_body) {
        Ok(problem) => {
            if let Err(err) = save_test_cases(&problem) {
                error!("{}", err);
            }
        }
        Err(error) => {
            error!("Could not parse problem: {}", error)
        }
    }
    Ok(Response::new("No response needed".into()))
}

fn parse_problem(msg: &[u8]) -> serde_json::Result<Problem> {
    let problem: Problem = serde_json::from_slice(msg)?;
    dbg!(&problem);
    Ok(problem)
}

fn save_test_cases(problem: &Problem) -> std::io::Result<()> {
    // TODO: More sophisticated short name
    let short_name: String = problem
        .name
        .chars()
        .take_while(|c| c.is_alphanumeric())
        .collect();

    info!("Saving test cases for problem {}", short_name);
    for (i, test_case) in problem.tests.iter().enumerate() {
        let prefix = format!("{}-{}", short_name, i + 1);
        let input_path = format!("{}.in", prefix);
        let output_path = format!("{}.out", prefix);
        info!("Writing {}", input_path);
        std::fs::write(input_path, &test_case.input)?;
        info!("Writing {}", output_path);
        std::fs::write(output_path, &test_case.output)?;
    }
    Ok(())
}

/// A programming contest assistant
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Subcommand, Debug)]
enum Cmd {
    Download {
        #[clap(long, default_value_t = 1327)]
        port: u16,
    },
    Test {
        /// Path to executable
        #[clap(parse(from_os_str))]
        executable: PathBuf,
        /// Problem name - test cases must have the pattern <name>-*.{in, out}
        #[clap()]
        name: String,
    },
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let args = Args::parse();
    match args.cmd {
        Cmd::Download { port } => {
            info!("Listening for contest problems on port {}...", port);
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            let make_svc =
                make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(listen)) });
            let server = Server::bind(&addr).serve(make_svc);
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        }
        Cmd::Test { executable, name } => {
            if let Err(err) = tester::test_run(executable, &name) {
                error!("{}", err);
            }
        }
    }
}
