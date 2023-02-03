use anyhow::Result;
use std::{collections::HashSet, num::NonZeroUsize, str::FromStr};

use lru::LruCache;
use select::{document::Document, predicate::Name};

use crate::wiki_page_name::WikiPage;

pub type WikiPageRequestCache = LruCache<String, HashSet<WikiPage>>;

const CAPACITY: usize = 1000;

pub fn new_request_cache() -> WikiPageRequestCache {
    LruCache::new(NonZeroUsize::new(CAPACITY).unwrap())
}

pub async fn get_linked_pages(
    request_cache: &mut WikiPageRequestCache,
    page_url: &str,
) -> Result<HashSet<WikiPage>> {
    match request_cache.get(&page_url.to_string()) {
        Some(result) => Ok(result.clone()),
        None => {
            let request_result = get_linked_pages_request(page_url).await?;
            request_cache.put(page_url.to_string(), request_result.clone());

            Ok(request_result)
        }
    }
}

async fn get_linked_pages_request(page_url: &str) -> Result<HashSet<WikiPage>> {
    let res = reqwest::get(page_url).await?.text().await?;

    let linked_pages = Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .flat_map(|l| WikiPage::from_str(l))
        .collect::<Vec<WikiPage>>();

    Ok(HashSet::from_iter(linked_pages))
}
