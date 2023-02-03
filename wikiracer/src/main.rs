use anyhow::Result;

mod wiki_page_name;
mod wiki_racer;

use crate::wiki_page_name::WikiPage;

#[tokio::main]
async fn main() -> Result<()> {
    let initial_page = WikiPage::new("Mystery_Seeker");

    initial_page
        .get_linked_pages()
        .await?
        .into_iter()
        .for_each(|x| println!("{}", x.name()));

    Ok(())
}
