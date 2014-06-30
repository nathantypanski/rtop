use std::io::File;

pub fn read_meminfo(meminfo: &Path) -> Option<String> {
    let mut file = File::open(meminfo);
    match file.read_to_end() {
        Ok(bytes) => {
            bytes.into_ascii_opt().map(|b| b.into_str())
        },
        Err(_) => { None },
    }
}
