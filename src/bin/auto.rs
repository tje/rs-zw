use structopt::StructOpt;
use std::io::{self, Read};

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "e", long = "encode")]
    encode: bool,
    #[structopt(short = "d", long = "decode")]
    decode: bool,
    input: Option<String>,
}

pub fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let input: String = match args.input {
        Some(s) => s,
        None if atty::isnt(atty::Stream::Stdin) => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        },
        _ => {
            eprintln!("Error: no input provided");
            std::process::exit(0x0100);
        },
    };

    let input = input.trim();
    if args.encode {
        println!("{}", zw::encode(input));
    } else if args.decode {
        println!("{}", zw::decode(input));
    } else {
        let ch = input.chars().next().unwrap();
        let out = match ch {
            zw::ZW_0 | zw::ZW_1 => zw::decode(input),
            _ => zw::encode(input),
        };
        println!("{}", out);
    }

    Ok(())
}
