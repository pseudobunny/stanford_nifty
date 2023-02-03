use anyhow::{anyhow, Ok, Result};
use std::collections::HashSet;

use crate::wiki_page_request_cache::{get_linked_pages, WikiPageRequestCache};
use crate::{wiki_page_name::WikiPage, wiki_page_request_cache::new_request_cache};
use priority_queue::PriorityQueue;

type WikiLadder = Vec<WikiPage>;

pub struct WikiRacer {
    ladder_queue: PriorityQueue<WikiLadder, usize>,
    visited_pages: HashSet<WikiPage>,
    target_page: WikiPage,
    request_cache: WikiPageRequestCache,
}

impl WikiRacer {
    pub fn new(initial_page: WikiPage, target_page: WikiPage) -> Result<WikiRacer> {
        let mut ladder_queue = PriorityQueue::new();
        ladder_queue.push(vec![initial_page.clone()], 0);

        let mut visited_pages = HashSet::new();
        visited_pages.insert(initial_page);

        let racer = WikiRacer {
            ladder_queue,
            visited_pages,
            target_page: target_page.clone(),
            request_cache: new_request_cache(),
        };

        Ok(racer)
    }

    pub async fn find_best_wiki_ladder(&mut self) -> Result<WikiLadder> {
        while !self.ladder_queue.is_empty() {
            if let Some(ladder) = self.parse_highest_priority_ladder().await? {
                return Ok(ladder);
            }
        }

        Ok(vec![])
    }

    async fn parse_highest_priority_ladder(&mut self) -> Result<Option<WikiLadder>> {
        let ladder = self
            .ladder_queue
            .pop()
            .ok_or(anyhow!("Could not pop a ladder from the queue"))?
            .0;

        let page_to_process = ladder
            .last()
            .ok_or(anyhow!("The ladder did not have a last page"))?
            .clone();

        let page_to_process_linked_pages = self
            .get_linked_pages_from_cache(&page_to_process.get_link())
            .await?;

        let visited_pages = &self.visited_pages.clone();

        let new_pages_for_ladder = page_to_process_linked_pages.difference(visited_pages);

        for page in new_pages_for_ladder.cloned() {
            let linked_pages = self.get_linked_pages_from_cache(&page.get_link()).await?;

            let mut new_ladder = ladder.clone();
            new_ladder.push(page);

            if linked_pages.contains(&self.target_page) {
                new_ladder.push(self.target_page.clone());

                return Ok(Some(new_ladder));
            }

            let priority = self.determine_priority(&linked_pages).await?;

            self.ladder_queue.push(new_ladder, priority);
        }

        Ok(None)
    }

    async fn determine_priority(&mut self, linked_pages: &HashSet<WikiPage>) -> Result<usize> {
        Ok(self
            .get_linked_pages_from_cache(&self.target_page.get_link())
            .await?
            .intersection(linked_pages)
            .count())
    }

    async fn get_linked_pages_from_cache(&mut self, page_url: &str) -> Result<HashSet<WikiPage>> {
        get_linked_pages(&mut self.request_cache, page_url).await
    }
}
