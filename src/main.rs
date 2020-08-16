use serde_json::Value;
use simd_json;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    input: PathBuf,
    #[structopt(short, long)]
    disable_simd: bool,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let f = File::open(opt.input.clone())?;

    let reader = BufReader::new(f);
    let _: Value = 
        if opt.disable_simd {
            serde_json::from_reader(reader)?
        } else {
            simd_json::serde::from_reader(reader)?
        };

    Ok(())
}
