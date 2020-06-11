use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "s", long = "size", default_value = "10")]
    size: u8,

    #[structopt(parse(from_os_str), default_value = "foo.txt")]
    output: PathBuf
}

fn zeroes(c: u8, size: u32) -> Vec<u8> {
    vec![c; size as usize]
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let path = Path::new(&opt.output);
    if path.exists() {
        fs::remove_file(path)?;
    }

    let mut file = File::create(path)?;

    for i in 1..=opt.size {
        let size = 1024 * 1024;
        let vec = zeroes(0x00 + i, size);

        file.write_all(vec.as_slice())?;
    }

    Ok(())
}
