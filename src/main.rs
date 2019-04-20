use std::fs;
use std::io::Read;
use std::io::Write;

use argparse::{ArgumentParser, Store, StoreTrue};

mod bit_packer;
mod delta;
mod lz77;

fn read_file_to_bytes(filename: &str) -> Vec<u8> {
    let mut res = Vec::new();

    let mut input = fs::File::open(filename).expect("File could not be opened.");
    input
        .read_to_end(&mut res)
        .expect("File could not be read.");

    res
}

const DISTANCE_BITS: usize = 12;
const LENGTH_BITS: usize = 4;

fn main() -> std::io::Result<()> {
    // let args: Vec<String> = std::env::args().collect();

    // if args.len() < 3 {
    //     print_usage();
    //     std::process::exit(1);
    // }

    // let decompress = args[1] == "-d";
    // let delta_encode = true;

    // if decompress && args.len() < 4 {
    //     print_usage();
    //     std::process::exit(1);
    // }

    let mut decompress = false;
    let mut delta_coding = false;
    let mut input_name = String::new();
    let mut output_name = String::new();

    {
        let mut parser = ArgumentParser::new();

        parser.set_description(
            "Compress or decompress a file, possibly using delta encoding (for 64-bit unsigned integers).");

        parser.refer(&mut decompress).add_option(
            &["-d", "--decompress"],
            StoreTrue,
            "Decompress (default is compress).",
        );

        parser.refer(&mut delta_coding).add_option(
            &["-e", "--delta"],
            StoreTrue,
            "Use delta coding.",
        );

        parser
            .refer(&mut input_name)
            .required()
            .add_argument("input_file", Store, "The input file name.");

        parser
            .refer(&mut output_name)
            .required()
            .add_argument("output_file", Store, "The output file name.");

        parser.parse_args_or_exit();
    }

    let input_bytes = read_file_to_bytes(&input_name);

    let res = if decompress {
        let option = lz77::lz77_decode(&input_bytes, DISTANCE_BITS, LENGTH_BITS);
        if let Some(result) = option {
            if delta_coding {
                let u64s = delta::le_bytes_to_u64s(&result);
                let delta_decoded = delta::from_deltas(&u64s);
                delta::u64s_to_le_bytes(&delta_decoded)
            } else {
                result
            }
        } else {
            println!("Corrupt compressed file.");
            std::process::exit(2);
        }
    } else {
        // Compressing.
        let to_compress = if delta_coding {
            let u64s = delta::le_bytes_to_u64s(&input_bytes);
            let delta_encoded = delta::to_deltas(&u64s);
            let delta_bytes = delta::u64s_to_le_bytes(&delta_encoded);
            delta_bytes
        } else {
            input_bytes
        };

        lz77::lz77_encode(&to_compress, DISTANCE_BITS, LENGTH_BITS)
    };

    let mut output = fs::File::create(output_name)?;
    output.write_all(&res)?;

    Ok(())
}
