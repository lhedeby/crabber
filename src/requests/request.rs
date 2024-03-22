use anyhow::{anyhow, Context};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    str::FromStr,
};

use crate::FOLDER_PATH;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub url: Option<String>,
    pub method: Option<Kind>,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Kind {
    POST,
    GET,
}

impl FromStr for Kind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "POST" => Ok(Kind::POST),
            "GET" => Ok(Kind::GET),
            _ => Err(anyhow!("Error parsing Kind")),
        }
    }
}

impl Request {
    pub fn save_to_file(&self, name: &str) -> anyhow::Result<()> {
        let seralized = serde_json::to_string(self).context("error serializing request object")?;

        if !Path::new(FOLDER_PATH).exists() {
            fs::create_dir(FOLDER_PATH).context("error creating root folder")?;
        }

        if let Ok(mut file) = File::create(format!("{}/{}", FOLDER_PATH, &name)) {
            file.write_all(seralized.as_bytes())
                .context("Error writing to request file")?;
        }
        Ok(())
    }

    pub fn get(name: &str) -> anyhow::Result<Self> {
        if !Path::new(FOLDER_PATH).exists() {
            fs::create_dir(FOLDER_PATH).context("error creating root folder")?;
        }
        let text = fs::read_to_string(format!("{}/{}", FOLDER_PATH, name))?;
        let res = serde_json::from_str(&text)?;
        Ok(res)
    }

    pub fn print(&self) -> anyhow::Result<()> {
        let seralized =
            serde_json::to_string_pretty(self).context("error serializing request object")?;
        println!("{}", seralized);
        Ok(())
    }

    pub fn run(self) -> anyhow::Result<()> {
        let client = reqwest::blocking::Client::new();
        let method = self.method.expect("method missing");
        let url = self.url.expect("url missing");
        let mut headers = HeaderMap::new();
        for (key, value) in self.headers {
            if let Ok(key) = HeaderName::from_str(&key) {
                if let Ok(value) = HeaderValue::from_str(&value) {
                    headers.insert(key, value);
                }
            }
        }

        match method {
            Kind::GET => {
                let res = client.get(url).headers(headers).send()?;
                println!("res: {}", res.text()?);
            }
            Kind::POST => {
                let body = self.body.unwrap_or("".to_string());
                let res = client.post(url).body(body).send()?;
                println!("res: {}", res.text()?);
            }
        }
        Ok(())
    }
}
