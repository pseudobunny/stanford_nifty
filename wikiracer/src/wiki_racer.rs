use anyhow::{anyhow, Ok, Result};
use std::collections::HashSet;

use crate::wiki_page_name::WikiPage;
use crate::wiki_page_request::{get_all_sublinks, get_linked_pages_request};
use priority_queue::PriorityQueue;

type WikiLadder = Vec<WikiPage>;

pub struct WikiRacer {
    ladder_queue: PriorityQueue<WikiLadder, usize>,
    visited_pages: HashSet<WikiPage>,
    target_page: WikiPage,
    target_page_links: HashSet<WikiPage>,
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
            target_page_links: get_linked_pages_request(&target_page.get_link()).await?,
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

        println!("Current Highest Priority Ladder: {:?}", ladder.clone());

        let page_to_process = ladder
            .last()
            .ok_or(anyhow!("The ladder did not have a last page"))?
            .clone();

        let page_to_process_linked_pages =
            get_linked_pages_request(&page_to_process.get_link()).await?;

        let visited_pages = &self.visited_pages.clone();

        let new_pages_for_ladder = page_to_process_linked_pages.difference(visited_pages);

        println!(
            "Number of subpages to process: {}",
            new_pages_for_ladder.clone().count()
        );

        let sub_pages_tuples =
            get_all_sublinks(new_pages_for_ladder.into_iter().cloned().collect()).await?;

        for (page, linked_pages) in sub_pages_tuples {
            let mut new_ladder = ladder.clone();
            self.visited_pages.insert(page.clone());
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

    async fn determine_priority(&self, linked_pages: &HashSet<WikiPage>) -> Result<usize> {
        Ok(self.target_page_links.intersection(linked_pages).count())
    }
}
