use std::str::FromStr;
use std::collections::HashMap;

use anyhow::Context;

use crate::requests::request::{Kind, Request};

pub(crate) fn invoke(
    name: String,
    url: Option<String>,
    method: Option<String>,
    body: Option<String>,
) -> anyhow::Result<()> {
    let request = Request {
        url,
        // method: Kind::from_str(&method.unwrap()).ok(),
        method: match method {
            Some(m) => Some(Kind::from_str(&m)?),
            None => None
        },
        body,
        headers: HashMap::new(),
    };
    request
        .save_to_file(&name)
        .context("Error saving to file")?;
    Ok(())
}
