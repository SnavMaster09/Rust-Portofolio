use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::env;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use flate2::read::GzDecoder;

fn compress(args: Vector<String>) -> std::io::Result<()>{
    let mut _input = BufReader::new(File::open(format!("src/{}",&args[2]))?);  // Wrap in BufReader
    let _output = File::create(format!("src/{}.gz",&args[3]))?;
    let mut encoder = GzEncoder::new(_output, Compression::default());
    copy(&mut _input, &mut encoder)?;
    let _output = encoder.finish()?;
    Ok(())
}

fn decompress() -> std::io::Result<String> {
    let mut _input = BufReader::new(File::open(format!("src/{}",&args[2]))?);  // Wrap in BufReader
    let _output = File::create(format!("src/{}.gz",&args[3]))?;
    let mut encoder = GzEncoder::new(_output, Compression::default());
    copy(&mut _input, &mut encoder)?;
    let _output = encoder.finish()?;
    Ok(())
}

fn main()  {
    let args: Vec<String> = env::args().collect();
    if(args[1] == "compress") {
        compress().unwrap();
    }
    else if(args[1] == "decompress") {
        decompress().unwrap();
    }
}
