use std::fs;

pub fn import_nano_source(path: &str) -> String {
    let file_read_result = fs::read_to_string(path);

    match file_read_result {
        Ok(content) => {
            println!("{}", content);
            return content;
        },
        Err(error) => panic!("File '{}' couldn't be loaded {:?}.", path, error)
    }
}