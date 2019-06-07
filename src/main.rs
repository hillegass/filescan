use filescan::file_digest;
use std::collections::HashMap;
use std::env::args_os;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use bytefmt; // For human-readable memory amounts: "56.2 GB"
use ctrlc; // For catching control-c

fn main() {
    // Set a handler for control-c
    ctrlc::set_handler(move || {
        unsafe { filescan::file_digest::set_interrupted(true) };
    })
    .expect("Error setting Ctrl-C handler");

    // What are the command line arguments?
    let mut args: Vec<OsString> = args_os().skip(1).collect();

    // Is the command line empty?
    if args.len() == 0 {
        // Scan the working directory
        args.push(OsString::from("."));
    }

    // Create an empty hash map of file digests and their corresponding paths
    let mut digests: HashMap<file_digest::FileDigest, Vec<PathBuf>> = HashMap::new();

    // Go through the command-line processing making file digests for every file in the directories listed
    for arg in &args {
        let p = Path::new(arg);
        file_digest::process_dir(&p, &mut digests);
    }

    // Print the total number of unique file digests
    println!("Total digests: {}", digests.len());

    // Keep only possible duplicates
    digests.retain(|_, value| value.len() > 1);
    println!("Possible duplicates: {}", digests.len());

    // Make a list of the file digests ordered from largest to smallest
    let mut keys: Vec<&file_digest::FileDigest> = digests.keys().collect();
    keys.sort_by(|a, b| b.file_length.partial_cmp(&a.file_length).unwrap());

    // For each unique file digest, list the files that match
    for key in keys {
        println!("{}:", bytefmt::format(key.file_length as u64));
        for value in digests.get(&key).unwrap() {
            println!("    {}", value.to_str().unwrap());
        }
    }
}
