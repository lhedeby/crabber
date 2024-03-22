use std::fs;
use crate::FOLDER_PATH;

pub(crate) fn invoke(name: String, new_name: String) -> anyhow::Result<()> {
    fs::rename(format!("{}/{}", FOLDER_PATH, &name), format!("{}/{}", FOLDER_PATH, &new_name))?;
    Ok(())
}
