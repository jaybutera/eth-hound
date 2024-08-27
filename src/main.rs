use ethers_core::types::Address;
use std::str::FromStr;
use std::error::Error;
use structopt::StructOpt;
use color_eyre::eyre::Result;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    addrs: Option<std::path::PathBuf>,
    #[structopt(short, long)]
    api_key: Option<String>,
    #[structopt(short, long)]
    target: String,
}

fn load_addrs(filename: &str) -> Result<Vec<Address>> {
    let file = std::fs::read_to_string(filename)?;
    let addrs = file
        .lines()
        .map(|line| Address::from_str(line.trim()))
        .collect::<Result<Vec<Address>, _>>()?;
    Ok(addrs)
}

fn load_api_key(filename: &str) -> Result<String> {
    let file = std::fs::read_to_string(filename)?;
    Ok(file.trim().to_string())
}

use ethers_etherscan::{Client, account::TxListParams};
use ethers_core::types::Chain;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let api_key =
        if let Some(api_key) = args.api_key {
            api_key
        } else {
            load_api_key("api_key.txt")?
        };
    let target = Address::from_str(&args.target)?;
    let test_addrs = if let Some(addrs) = args.addrs {
        load_addrs(addrs.to_str().unwrap())?
    } else {
        load_addrs("addrs.txt")?
    };

    let chain = Chain::Mainnet;
    let client = Client::new(chain, api_key).unwrap();
    let params = TxListParams::default();

    let transactions = client.get_transactions(&target, Some(params)).await?;

    let from_addresses: Vec<Address> = transactions
        .into_iter()
        .filter_map(|tx| tx.from.value().cloned())
        .collect();

    for addr in from_addresses.iter() {
        if test_addrs.contains(addr) {
            println!("{:?}", addr);
        }
    }

    println!("Total unique 'from' addresses: {}", from_addresses.len());

    Ok(())
}
