use std::{fs::File, io::Read, str::from_utf8};

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tendermint_rpc::endpoint::{block, block_results};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockAndTransactions {
    pub block: block::Response,
    pub block_results: block_results::Response,
}

fn main() -> anyhow::Result<()> {
    let data: BlockAndTransactions = read_data()?;

    let config = bincode::config::standard();
    let encoded: Vec<u8> = bincode::serde::encode_to_vec(&data, config)?;
    // THIS IS THE LINE THAT FAILS
    let (_decoded, _): (BlockAndTransactions, _) =
        bincode::serde::decode_from_slice(&encoded, config)?;

    Ok(())
}

pub fn read_data<T: DeserializeOwned>() -> anyhow::Result<T> {
    let filename = "1206335.json.xz";

    let file = File::open(filename)?;
    let mut decompressor = lzma::LzmaReader::new_decompressor(file)?;

    let mut s = Vec::new();
    decompressor.read_to_end(&mut s)?;

    let string = from_utf8(&s)?;
    Ok(serde_json::from_str::<T>(string).with_context(|| "Can't decode json data")?)
}
