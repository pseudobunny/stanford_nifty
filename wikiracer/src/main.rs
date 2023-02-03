
use std::hash::Hash;
use std::str::FromStr;
use std::collections::HashSet;

use select::document::Document;
use select::predicate::Name;
use anyhow::Result;

mod wiki_page_name;

use crate::wiki_page_name::WikiPageName;

async fn get_wiki_links_from_wiki_page(page: WikiPageName) -> Result<HashSet<WikiPageName>> {
    let res = reqwest::get(page.get_link())
        .await?
        .text()
        .await?;

    let linked_pages = Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .flat_map(|l| WikiPageName::from_str(l))
        .collect::<Vec<WikiPageName>>();

    Ok(HashSet::from_iter(linked_pages))
}

#[tokio::main]
async fn main() -> Result<()> {
    let initial_page = WikiPageName::new("Mystery_Seeker");

    get_wiki_links_from_wiki_page(initial_page)
        .await?
        .into_iter()
        .for_each(|x| println!("{}", x.name()));

    Ok(())
}
