use super::Solve;
use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Fragment {
    File(usize),
    Free,
}

impl Fragment {
    fn into_id(&self) -> Option<usize> {
        match self {
            Self::File(id) => Some(*id),
            Self::Free => None,
        }
    }
}

#[derive(Default)]
pub struct Solution {
    raw: Vec<char>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day09/input.txt");
        sol
    }

    fn generate_disk_state(&self) -> Vec<Fragment> {
        let mut id = 0;
        let mut f_type = Fragment::File(id);
        let mut disk = vec![];

        for frag in self.raw.iter() {
            let sum = frag.to_digit(10).unwrap();
            for _ in 0..sum {
                disk.push(f_type);
            }

            if f_type == Fragment::Free {
                id += 1;
                f_type = Fragment::File(id);
            } else {
                f_type = Fragment::Free;
            }
        }

        disk
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        self.raw = std::fs::read_to_string(path)?
            .trim()
            .chars()
            .collect::<Vec<char>>();

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let mut disk = self.generate_disk_state();
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

        println!("Part 1: {total}");

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let mut disk = self.generate_disk_state();
        let file_idx = (disk.len() - 1)
            - disk
                .iter()
                .rev()
                .position(|&f| f != Fragment::Free)
                .unwrap();

        let mut file_id = disk[file_idx].into_id().unwrap();

        while file_id > 0 {
            let (s_idx, e_idx) = get_file_block_by_id(&disk, file_id);
            if let Some((free_start, free_end)) = get_first_free_block(&disk, e_idx - s_idx, s_idx)
            {
                for (file, free) in (s_idx..e_idx)
                    .into_iter()
                    .zip((free_start..free_end).into_iter())
                {
                    disk.swap(file, free);
                }
            }

            file_id -= 1;
        }

        let total: usize = disk
            .iter()
            .enumerate()
            .filter_map(|(idx, block)| match block {
                Fragment::File(id) => Some(idx * id),
                Fragment::Free => None,
            })
            .sum();

        println!("Part 2: {total}");

        Ok(())
    }
}

fn get_file_block_by_id(disk: &Vec<Fragment>, id: usize) -> (usize, usize) {
    let end_idx = (disk.len() - 1)
        - disk
            .iter()
            .rev()
            .position(|&f| f == Fragment::File(id))
            .unwrap();

    let mut start_idx = end_idx;
    while disk[start_idx - 1] == Fragment::File(id) {
        start_idx -= 1;
    }

    (start_idx, end_idx + 1)
}

fn get_first_free_block(disk: &Vec<Fragment>, size: usize, limit: usize) -> Option<(usize, usize)> {
    let mut start = None;
    let mut length = 0;

    for (idx, frag) in disk.iter().enumerate() {
        if *frag == Fragment::Free {
            if start.is_none() {
                start = Some(idx);
            }

            length += 1;

            if length >= size && idx < limit {
                return start.map(|s| (s, s + length));
            }
        } else {
            start = None;
            length = 0;
        }
    }

    None
}
