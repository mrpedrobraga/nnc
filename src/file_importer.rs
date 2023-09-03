use std::fs;

pub fn import_nano_source(path: &str) -> Result<String, std::io::Error> {
    let file_read_result = fs::read_to_string(path);
    let _ = fs::create_dir(path);

    return file_read_result;
}