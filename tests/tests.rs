#[cfg(test)]

mod ../src/lib;
use filescan::file_digest::FileDigest;
use filescan::file_digest;

mod tests {
    use super::*;

    #[test]
    fn equal_files_equal_digests() {
      let a_path = Path::new("tests/a.txt");
      let a_metadata = a_path.metadata().unwrap();
      let a_len: usize = metadata.len() as usize;
      let a_digest = match file_digest(&a_path, a_len) {
                        Err(e) => {
                            eprintln!("Unable to hash {}", e);
                            continue;
                        }
                        Ok(f) => f,
                    };

      assert!(5 > 2)         
    }
}