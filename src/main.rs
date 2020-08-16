use serde_json::Value;
use simd_json;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    input: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let f = File::open(opt.input.clone())?;

    let reader = BufReader::new(f);
    let _: Value = simd_json::serde::from_reader(reader)?;

    Ok(())
}
