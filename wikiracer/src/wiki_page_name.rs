use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct WikiPage {
    name: String,
}

impl FromStr for WikiPage {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        let valid_page_name_str =
            value.starts_with("/wiki/") && !value.contains("Main_Page") && !value.contains(':');

        if !valid_page_name_str {
            return Err(anyhow!(
                "This link is unable to be parsed as an wikipedia page name"
            ));
        }

        Ok(WikiPage {
            name: value[6..].to_string(),
        })
    }
}

impl WikiPage {
    const WIKI_LINK_BASE: &str = "https://en.wikipedia.org/wiki/";

    pub fn new(page_name: &str) -> WikiPage {
        WikiPage {
            name: page_name.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_link(&self) -> String {
        format!("{}{}", WikiPage::WIKI_LINK_BASE, self.name)
    }
}
