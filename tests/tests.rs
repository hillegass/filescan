#[cfg(test)]
use filescan::file_digest;
use std::path::Path;

mod tests {
    use super::*;

    fn digest_for_path(p:&str) -> file_digest::FileDigest {
       let a_path = Path::new(p);
      let a_metadata = a_path.metadata().unwrap();
      let a_len: usize = a_metadata.len() as usize;
      let a_digest = match file_digest::file_digest(&a_path, a_len) {
                        Err(e) => {
                            panic!("Unable to hash {}", e);
                        }
                        Ok(f) => f,
                    };
		    a_digest
		    }

    #[test]
    fn equal_files_equal_digests() {
      let a_digest = digest_for_path("tests/a.txt");
      let b_digest = digest_for_path("tests/equaltoa.txt");
      assert!(a_digest == b_digest);         
    }

    #[test]
    fn unequal_files_unequal_digests() {
      let a_digest = digest_for_path("tests/a.txt");
      let b_digest = digest_for_path("tests/notequaltoa.txt");
      assert!(a_digest != b_digest);         
    }
}