use regex::Regex;
use std::io::fs;

/*
 * Returns true if the path is to a process's addr in procfs.
 */
fn is_proc(dir: &Path) -> Option<Path> {
    let mut result = None;
    if dir.is_dir() {
        let fname_str = dir.filename_str().expect("");
        let re = match Regex::new(r"^[0-9]+$") {
            Ok(re) => re,
            Err(err) => fail!("{}", err),
        };
        result = match re.is_match(fname_str) {
            true => Some(dir.clone()),
            false => None,
        };
    }
    result
}

pub fn read_processes(procfs: &Path) {
    match fs::readdir(procfs) {
        Ok(diri) => {
            for dir in diri.iter() {
                match is_proc(dir) {
                    Some(procdir) => {
                        println!("{}", procdir.filename_display());
                    }
                    None => {}
                }
            }

        }
        Err(_) => {}
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_is_proc() {
    }
}
