use std::fs;

pub fn read(path: &str) -> String {
    fs::read_to_string(path).expect("file read error")
}

pub fn write(path: &str, content: &str) {
    fs::write(path, content).expect("file write error");
}
