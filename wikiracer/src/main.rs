use anyhow::Result;
use wiki_racer::WikiRacer;

mod wiki_page_name;
mod wiki_page_request;
mod wiki_racer;

use crate::wiki_page_name::WikiPage;

#[tokio::main]
async fn main() -> Result<()> {
    let initial_page = WikiPage::new("Belle_Delphine");
    let target_page = WikiPage::new("Communism");

    let mut wiki_racer = WikiRacer::new(initial_page, target_page).await?;

    wiki_racer
        .find_best_wiki_ladder()
        .await?
        .iter()
        .for_each(|p| println!("{}", p.name()));

    Ok(())
}
