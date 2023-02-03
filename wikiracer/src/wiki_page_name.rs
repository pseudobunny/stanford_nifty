use anyhow::{Result, anyhow};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash)]
pub struct WikiPageName {
    name: String
}

impl FromStr for WikiPageName {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        let valid_page_name_str = value.starts_with("/wiki/") && !value.contains("Main_Page") && !value.contains(':');

        if !valid_page_name_str {
            return Err(anyhow!("This link is unable to be parsed as an wikipedia page name"));
        }

        Ok(WikiPageName{ name: value[6..].to_string() })
    }
}

impl WikiPageName {
    const WIKI_LINK_BASE: &str = "https://en.wikipedia.org/wiki/";
    
    pub fn new(page_name: &str) -> WikiPageName {
        WikiPageName { name: page_name.to_string() }
    }

    pub fn name(self) -> String {
        self.name
    }

    pub fn get_link(self) -> String {
        format!("{}{}", WikiPageName::WIKI_LINK_BASE, self.name)
    }
}