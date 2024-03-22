use std::{fs, path::Path};

use crate::FOLDER_PATH;

pub(crate) fn invoke() -> anyhow::Result<()> {
    if !Path::new(FOLDER_PATH).exists() {
        fs::create_dir(FOLDER_PATH).expect("Error creating dir");
    }
    if let Ok(paths) = fs::read_dir(FOLDER_PATH) {
        println!("All saved requests located in {}", FOLDER_PATH);
        for p in paths {
            println!("path: {}", p.unwrap().file_name().into_string().unwrap());
        }
    }
    Ok(())
}
