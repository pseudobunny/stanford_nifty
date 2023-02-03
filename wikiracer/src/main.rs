use anyhow::Result;
use wiki_page_request_cache::{get_linked_pages, new_request_cache};

mod wiki_page_name;
mod wiki_page_request_cache;
mod wiki_racer;

use crate::wiki_page_name::WikiPage;

#[tokio::main]
async fn main() -> Result<()> {
    let initial_page = WikiPage::new("Mystery_Seeker");

    let mut request_cache = new_request_cache();

    get_linked_pages(&mut request_cache, &initial_page.get_link())
        .await?
        .into_iter()
        .for_each(|x| println!("{}", x.name()));

    Ok(())
}
