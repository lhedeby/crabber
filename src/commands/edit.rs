use std::str::FromStr;

use crate::requests::request::{Kind, Request};

pub(crate) fn invoke(name: String, url: Option<String>, method: Option<String>, body: Option<String>) -> anyhow::Result<()>{
    let mut request = Request::get(&name)?;
    if let Some(_) = url {
        request.url = url;
    }
    if let Some(method) = method {
        request.method = Some(Kind::from_str(&method)?);
    }
    if let Some(_) = body {
        request.body = body;
    }
    request.save_to_file(&name)?;
    Ok(())
}

