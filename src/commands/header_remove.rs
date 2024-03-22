use crate::requests::request::Request;

pub fn invoke(name: String, header: String) -> anyhow::Result<()> {
    let mut request = Request::get(&name)?;
    request.headers.remove(&header);
    request.save_to_file(&name)?;
    Ok(())
}
