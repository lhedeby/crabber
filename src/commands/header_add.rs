
use crate::requests::request::Request;

pub fn invoke(name: String, header: String) -> anyhow::Result<()> {
    let mut request = Request::get(&name)?;
    if let Some((key, value)) = header.split_once(':') {
        request.headers.insert(key.to_string(), value.to_string());
    }
    request.save_to_file(&name)?;
    Ok(())
}
