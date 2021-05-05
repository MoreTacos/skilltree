use std::error::Error;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    globalpw: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let mut params = HashMap::new();
    params.insert("name", "Toronto Gymnastics International");
    params.insert("email", "tgi@gmail.com");
    params.insert("pw", "password");
    params.insert("globalpw", &args.globalpw);

    let client = reqwest::Client::new();
    let url = format!("http://localhost:8000/create_gym");
    client.post(url).form(&params).send().await?;
    Ok(())
}
