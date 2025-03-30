use std::env;
use std::process;

mod tosbr;
mod fromsbr;
mod tohbmp;
mod fromhbmp;

fn print_usage() {
    eprintln!("Usage: databendingutils <tosbr/fromsbr/tohbmp/fromhbmp> <input_file> <output_file> <header_file>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 5 {
        print_usage();
        process::exit(1);
    }

    let command = args[1].as_str();

    let result: Result<(), Box<dyn std::error::Error>> = match command {
        "tosbr" => tosbr::convert_to_sbr(&args[2], &args[3], &args[4]).map_err(|e| e.into()),
        "fromsbr" => fromsbr::convert_from_sbr(&args[2], &args[4], &args[3]).map_err(|e| e.into()),
        "tohbmp" => tohbmp::convert_to_hbmp(&args[2], &args[3], &args[4]).map_err(|e| e.into()),
        "fromhbmp" => fromhbmp::convert_from_hbmp(&args[2], &args[4], &args[3]).map_err(|e| e.into()),
        _ => {
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    } else {
        println!("Successfully executed {}", command);
    }
}
