#[allow(dead_code)]

pub mod file_digest {

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

    const BUF_SIZE: usize = 4096; // Size of buffer read for digest
    const BYTES_IN_DIGEST: usize = 4; // Size of the digest
    const MAX_BYTES_SCANNED: usize = BUF_SIZE * 3; // Only scan this much of the file

    static mut INTERRUPTED: bool = false; // Has control-c been pressed?

    // Unsafe methods for accessing INTERRUPTED
    pub unsafe fn is_interrupted() -> bool {
        INTERRUPTED
    }

    pub unsafe fn set_interrupted(yn: bool) {
        INTERRUPTED = yn;
    }

    // We record the size and digest (or "hash") of each file scanned
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub struct FileDigest {
        pub file_length: usize,
        pub file_digest: [u8; BYTES_IN_DIGEST],
    }

    // Get the create a FileDigest for the file at 'p', which we think is 'flen' bytes long.
    pub fn file_digest(p: &Path, flen: usize) -> std::io::Result<FileDigest> {
        let mut f = std::fs::File::open(p)?;
        let mut digest = [0u8; BYTES_IN_DIGEST];
        let mut buffer = [0u8; BUF_SIZE];
        let mut read_len: usize = BUF_SIZE;
        let mut bytes_read = 0;

        // Read a few buffers worth and make a digest using bitwise XOR
        while read_len > 0 && bytes_read < MAX_BYTES_SCANNED {
            read_len = f.read(&mut buffer)?;
            bytes_read = bytes_read + read_len;
            let steps: usize = read_len / BYTES_IN_DIGEST;
            for i in 0..steps {
                let start_index = i * BYTES_IN_DIGEST;
                for j in 0..BYTES_IN_DIGEST {
                    digest[j] = digest[j] ^ buffer[start_index + j];
                }
            }
        }

        let f_dig = FileDigest {
            file_length: flen,
            file_digest: digest,
        };
        Ok(f_dig)
    }

    pub fn process_dir(p: &Path, digests: &mut HashMap<FileDigest, Vec<PathBuf>>) {
        // As we encounter directories, add them to the list
        let mut dirs_to_process: Vec<PathBuf> = Vec::new();

        // Start with the directory at p.
        // Is it a directory?
        if !p.is_dir() {
            eprintln!("{:?} is not a directory", p);
            return;
        }

        // Add it to the list of directories to scan
        dirs_to_process.push(p.to_path_buf());

        // Keep scanning until there are no more directories or the user pressed control-C
        while !dirs_to_process.is_empty() && unsafe { !is_interrupted() } {
            // Pop the first directory of the beginning of the list
            let current_dir = dirs_to_process.remove(0);

            // Open it
            let rd = match fs::read_dir(&current_dir) {
                Err(e) => {
                    eprintln!("Error reading {:?}: {}", &current_dir, e);
                    continue;
                }
                Ok(f) => f,
            };

            // Go through each entry in the directory
            for entry in rd {
                let entry = match entry {
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                    Ok(f) => f,
                };
                let path_buf = entry.path();
                let path_type: std::fs::FileType = entry.file_type().unwrap();

                // Do not follow symlinks
                if path_type.is_symlink() {
                    continue;
                }

                // Is it a directory?
                if path_type.is_dir() {
                    // Add it to the list we are processing
                    dirs_to_process.push(path_buf);
                    continue;
                }

                // Is it a regular file?
                if path_type.is_file() {
                    let metadata = entry.metadata().unwrap();
                    let flen: usize = metadata.len() as usize;

                    // Skip little files
                    if flen < 100 {
                        continue;
                    }

                    // Make a FileDigest for it
                    let current_digest = match file_digest(&path_buf, flen) {
                        Err(e) => {
                            eprintln!("Unable to hash {}", e);
                            continue;
                        }
                        Ok(f) => f,
                    };

                    // Look up the FileDigest to see if an exact match exists
                    // If so, add it to the list of paths
                    // If not, create a vector and put it in the hash map
                    digests
                        .entry(current_digest)
                        .or_insert_with(Vec::new)
                        .push(path_buf);
                }
            }
        }
    }
}
