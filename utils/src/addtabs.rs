use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    name: String,
    globalpw: String,
    #[structopt(parse(from_os_str))]
    package_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let files = fs::read_dir(args.package_path.clone())?.map(|file| {
        let file = file.unwrap().path().clone();
        let content = fs::read_to_string(&file).unwrap();
        let name = file.file_stem().unwrap().to_str().unwrap().to_string();
        (name, content)
    }).collect::<HashMap<String, String>>();
    let params = (args.name.clone(), args.globalpw.clone(), files);

    let client = reqwest::Client::new();
    client.post("http://localhost:8000/create_package").json(&params).send().await?;
    Ok(())
}
