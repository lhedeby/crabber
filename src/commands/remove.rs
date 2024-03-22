use std::fs;
use crate::FOLDER_PATH;

pub(crate) fn invoke(name: String) -> anyhow::Result<()> {
    fs::remove_file(format!("{}/{}", FOLDER_PATH, &name))?;
    Ok(())
}
