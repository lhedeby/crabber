use crate::requests::request::Request;

pub(crate) fn invoke(name: String) -> anyhow::Result<()> {
    let request = Request::get(&name)?;
    request.print()?;
    Ok(())
}
