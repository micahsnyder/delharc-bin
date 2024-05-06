use clap::Parser;
use delharc::LhaDecodeReader;
use std::{io::Read, path::PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    target_path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut decoder = LhaDecodeReader::new(std::fs::File::open(args.target_path).unwrap()).unwrap();

    println!("Opened the LHA/LZH archive");

    loop {
        // Get the file header.
        let header = decoder.header();

        let filepath = header.parse_pathname();
        let filename = filepath.to_string_lossy();

        if header.is_directory() {
            println!("Skipping directory {filename}");
        } else {
            println!("Found file in LHA archive: {filename}");
            println!("Details: {header:?}");

            if !decoder.is_decoder_supported() {
                println!("err: unsupported compression method");
            } else {
                // Read the file into a buffer.
                let mut file_data = Vec::<u8>::new();

                match decoder.read_to_end(&mut file_data) {
                    Ok(bytes_read) => {
                        if bytes_read > 0 {
                            println!("Read {bytes_read} bytes from file {filename}");

                            // Verify the CRC check *after* reading the file.
                            match decoder.crc_check() {
                                Ok(crc) => {
                                    // CRC is valid.  Very likely it is indeed an LHA/LZH archive.
                                    println!("CRC check passed: CRC: {crc}");
                                }
                                Err(err) => {
                                    // Error checking CRC.
                                    println!(
                                        "An error occurred when checking the CRC of this LHA or LZH archive: {err}"
                                    );

                                    // Allow the scan to continue even with a CRC error, for now.
                                    // break;
                                }
                            }
                        } else {
                            println!("Read zero-byte file.");
                        }
                    }
                    err => {
                        println!("Error reading file {err:?}");
                    }
                }
            }
        }

        // Get the next file.
        match decoder.next_file() {
            Ok(true) => {
                println!("Found another file in the archive!");
            }
            Ok(false) => {
                println!("No more files in the archive.");
                break;
            }
            Err(err) => {
                // Error getting the next file.
                // Use println-level because may not actually be an LHA/LZH archive.
                // LHA/LZH does not have particularly identifiable magic bytes.
                println!("An error occurred when checking for the next file in this LHA or LZH archive: {err}");
                break;
            }
        }
    }
}
