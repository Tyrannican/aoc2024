use super::Solve;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Default)]
pub struct Solution {
    rules: HashMap<u8, Vec<u8>>,
    pages: Vec<Vec<u8>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day05/input.txt");
        sol
    }

    pub fn is_ordered(&self, pages: &[u8]) -> bool {
        let mut printed = vec![];
        for page in pages {
            if let Some(rules) = self.rules.get(page) {
                for rule in rules {
                    if printed.contains(rule) {
                        return false;
                    }
                }
            }
            printed.push(*page);
        }
        true
    }

    pub fn order_page(&self, pages: &[u8]) -> usize {
        let mut ordered = pages
            .iter()
            .map(|page| {
                let rules = match self.rules.get(page) {
                    Some(rs) => rs
                        .iter()
                        .filter(|r| pages.contains(r))
                        .collect::<Vec<&u8>>(),
                    None => vec![],
                };

                (*page, rules)
            })
            .collect::<Vec<(u8, Vec<&u8>)>>();

        ordered.sort_by(|a, b| b.1.len().partial_cmp(&a.1.len()).unwrap());

        ordered[ordered.len() / 2].0 as usize
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        let input: Vec<String> = std::fs::read_to_string(path)?
            .trim()
            .split("\n\n")
            .map(|s| s.to_string())
            .collect();

        let _: Vec<_> = input[0]
            .split("\n")
            .map(|rule| {
                if let Some((page, other)) = rule.split_once('|') {
                    let page = page.parse::<u8>().unwrap();
                    let other = other.parse::<u8>().unwrap();
                    self.rules
                        .entry(page)
                        .and_modify(|pages| pages.push(other))
                        .or_insert(vec![other]);
                }
            })
            .collect();

        let _: Vec<_> = input[1]
            .split("\n")
            .map(|page_list| {
                self.pages.push(
                    page_list
                        .split(",")
                        .map(|p| p.parse::<u8>().unwrap())
                        .collect(),
                );
            })
            .collect();
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let total: usize = self
            .pages
            .iter()
            .filter_map(|page| {
                if self.is_ordered(page) {
                    return Some(page[page.len() / 2] as usize);
                }
                None
            })
            .sum();

        println!("Part 1: {total}");

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let total: usize = self
            .pages
            .iter()
            .filter_map(|page| {
                if !self.is_ordered(page) {
                    return Some(self.order_page(page));
                }
                None
            })
            .sum();

        println!("Part 2: {total}");

        Ok(())
    }
}
