use super::Solve;
use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Fragment {
    File(usize),
    Free,
}

#[derive(Default)]
pub struct Solution {
    disk: Vec<Fragment>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day09/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        let frags = std::fs::read_to_string(path)?
            .trim()
            .chars()
            .collect::<Vec<char>>();

        let mut id = 0;
        let mut f_type = Fragment::File(id);

        for frag in frags {
            let sum = frag.to_digit(10).unwrap();
            for _ in 0..sum {
                self.disk.push(f_type);
            }

            if f_type == Fragment::Free {
                id += 1;
                f_type = Fragment::File(id);
            } else {
                f_type = Fragment::Free;
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let mut disk = self.disk.clone();
        let mut start_idx = disk
            .iter()
            .position(|&frag| frag == Fragment::Free)
            .unwrap();

        let mut end_idx = (disk.len() - 1)
            - disk
                .iter()
                .rev()
                .position(|&frag| frag != Fragment::Free)
                .unwrap();

        while start_idx < end_idx {
            disk.swap(start_idx, end_idx);
            while disk[start_idx] != Fragment::Free {
                start_idx += 1;
            }

            while disk[end_idx] == Fragment::Free {
                end_idx -= 1;
            }
        }

        let total: usize = disk
            .iter()
            .enumerate()
            .filter_map(|(idx, block)| match block {
                Fragment::File(id) => Some(idx * id),
                Fragment::Free => None,
            })
            .sum();

        println!("Total: {total}");

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        Ok(())
    }
}
