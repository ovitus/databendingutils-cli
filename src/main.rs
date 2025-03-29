use std::env;
use std::process;

mod fromhbmp;
mod tohbmp;
mod fromsbr;
mod tosbr;

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  databendingutils tosbr <input_file> <output_file> <header_file>");
    eprintln!("  databendingutils fromsbr <input_file> <header_file> <output_file>");
    eprintln!("  databendingutils tohbmp <input_file> <output_file> <header_file>");
    eprintln!("  databendingutils fromhbmp <input_file_path> <header_file> <output_file>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 5 {
        print_usage();
        process::exit(1);
    }

    let command = args[1].as_str();

    let result: Result<(), Box<dyn std::error::Error>> = match command {
        "to-sbr" => tosbr::convert_to_sbr(&args[2], &args[3], &args[4]).map_err(|e| e.into()),
        "from-sbr" => fromsbr::convert_from_sbr(&args[2], &args[3], &args[4]).map_err(|e| e.into()),
        "to-hbmp" => tohbmp::convert_to_hbmp(&args[2], &args[3], &args[4]).map_err(|e| e.into()),
        "from-hbmp" => fromhbmp::convert_from_hbmp(&args[2], &args[3], &args[4]).map_err(|e| e.into()),
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
