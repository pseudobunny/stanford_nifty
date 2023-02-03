use anyhow::{Ok, Result};
use std::collections::HashSet;

use crate::wiki_page_name::WikiPage;
use priority_queue::PriorityQueue;

type WikiLadder = Vec<WikiPage>;

struct WikiRacer {
    ladder_queue: PriorityQueue<WikiLadder, usize>,
    visited_pages: HashSet<WikiPage>,
    target_page: WikiPage,
    target_page_linked_pages: HashSet<WikiPage>,
}

impl WikiRacer {
    pub async fn new(initial_page: WikiPage, target_page: WikiPage) -> Result<WikiRacer> {
        let mut ladder_queue = PriorityQueue::new();
        ladder_queue.push(vec![initial_page.clone()], 0);

        let mut visited_pages = HashSet::new();
        visited_pages.insert(initial_page);

        let racer = WikiRacer {
            ladder_queue,
            visited_pages,
            target_page: target_page.clone(),
            target_page_linked_pages: target_page.get_linked_pages().await?,
        };

        Ok(racer)
    }

    fn determine_priority(self, linked_pages: &HashSet<WikiPage>) -> usize {
        self.target_page_linked_pages
            .intersection(linked_pages)
            .count()
    }
}
