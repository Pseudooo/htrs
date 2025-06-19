use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, value_name = "URL")]
    url: String
}

fn main() {
    let parsed_args = Cli::parse();

    let client = reqwest::blocking::Client::new();
    let response_result = client.get(&parsed_args.url).send();
    match response_result {
        Ok(response) => {
            println!("Response: {}", response.status());
        },
        Err(error) => {
            panic!("{}", error);
        }
    }
}
