use std::env;

use anyhow::{anyhow, Result};
use wiki_racer::WikiRacer;

mod wiki_page_name;
mod wiki_page_request;
mod wiki_racer;

use crate::wiki_page_name::WikiPage;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(anyhow!("You must provide a start and target page!"));
    }

    let initial_page = WikiPage::new(&args[1]);
    let target_page = WikiPage::new(&args[2]);

    let mut wiki_racer = WikiRacer::new(initial_page, target_page).await?;

    wiki_racer
        .find_best_wiki_ladder()
        .await?
        .iter()
        .for_each(|p| println!("{}", p.name()));

    Ok(())
}
