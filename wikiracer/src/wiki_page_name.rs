use anyhow::{anyhow, Result};
use select::{document::Document, predicate::Name};
use std::{collections::HashSet, str::FromStr};

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

    pub fn name(self) -> String {
        self.name
    }

    pub fn get_link(self) -> String {
        format!("{}{}", WikiPage::WIKI_LINK_BASE, self.name)
    }

    pub async fn get_linked_pages(self) -> Result<HashSet<WikiPage>> {
        let res = reqwest::get(self.get_link()).await?.text().await?;

        let linked_pages = Document::from(res.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .flat_map(|l| WikiPage::from_str(l))
            .collect::<Vec<WikiPage>>();

        Ok(HashSet::from_iter(linked_pages))
    }
}
