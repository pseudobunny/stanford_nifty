use anyhow::{Ok, Result};
use std::{collections::HashSet, str::FromStr};
use tokio::task::JoinSet;

use select::{document::Document, predicate::Name};

use crate::wiki_page_name::WikiPage;

pub async fn get_linked_pages_request(page_url: &str) -> Result<HashSet<WikiPage>> {
    let res = reqwest::get(page_url).await?.text().await?;

    let linked_pages = Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .flat_map(WikiPage::from_str)
        .collect::<Vec<WikiPage>>();

    Ok(HashSet::from_iter(linked_pages))
}

const BATCH_SIZE: usize = 100;
pub async fn get_all_sublinks(
    link_set: HashSet<WikiPage>,
) -> Result<Vec<(WikiPage, HashSet<WikiPage>)>> {
    let link_vec = link_set.into_iter().collect::<Vec<WikiPage>>();
    let windowed_set = link_vec.chunks(BATCH_SIZE);

    let mut result = vec![];
    for window in windowed_set {
        result.append(&mut batched_request(window).await?)
    }

    Ok(result)
}

async fn batched_request(link_batch: &[WikiPage]) -> Result<Vec<(WikiPage, HashSet<WikiPage>)>> {
    let mut batch_request_set = JoinSet::new();

    for page in link_batch.iter().cloned() {
        batch_request_set.spawn(async move {
            (
                page.clone(),
                get_linked_pages_request(&page.get_link()).await,
            )
        });
    }

    let mut results = vec![];
    while let Some(page_tuple_result) = batch_request_set.join_next().await {
        let page_tuple = page_tuple_result?;
        let page = page_tuple.0;
        let linked_pages = page_tuple.1?;

        results.push((page, linked_pages));
    }

    Ok(results)
}
