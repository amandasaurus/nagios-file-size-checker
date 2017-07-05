use std::env::args;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 4 {
        println!("Usage: nagios-file-size-checker FILENAME MIN_SIZE_BYTES MAX_SIZE_BYTES");
        println!("\nChecks if FILENAME has a filesize greater than MIN_SIZE_BYTES and less than\nMAX_SIZE_BYTES, and returns the appropriate nagios plugin status codes (and stdout\ndetails) if that's not the case. If the file doesn't exist, then it's OK.\n");
        exit(3);

    }
    let ref filename = args[1];
    let min_size: u64 = args[2].parse().unwrap_or_else(|_| {
        println!("UNKNOWN: Argument 2 (MIN_SIZE_BYTES) must be an integer");
        exit(3);
    });
    let max_size: u64 = args[3].parse().unwrap_or_else(|_| {
        println!("UNKNOWN: Argument 3 (MAX_SIZE_BYTES) must be an integer");
        exit(3);
    });

    let metadata = fs::metadata(&filename).unwrap_or_else(|_| {
        println!("OK: File {} doesn't exist", filename);
        exit(0);
    });

    if ! metadata.is_file() {
        println!("UNKNOWN: Filename {} is not a file", filename);
        exit(3);
    }

    let size = metadata.len();
    
    if size < min_size {
        println!("CRITICAL: File {} is {} bytes, which is less than the min size of {}", filename, size, min_size);
        exit(2);
    } else if size > max_size {
        println!("CRITICAL: File {} is {} bytes, which is greater than max size of {}", filename, size, max_size);
        exit(2);
    } else {
        println!("OK: File {} is {} bytes, which is between {} and {}", filename, size, min_size, max_size);
        exit(0);
    }


}
