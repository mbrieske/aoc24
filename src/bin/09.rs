use std::vec;

advent_of_code::solution!(9);

#[derive(Debug, PartialEq)]
enum DiskSpace {
    Data(usize, u64),
    Empty(usize),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut id: u64 = 0;
    let mut disk = input
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .flat_map(|(i, c)| {
            let is_data = i % 2 == 0;
            let data = if is_data { Some(id) } else { None };
            let datavec = vec![data; c.to_digit(10).unwrap() as usize];
            if is_data {
                id += 1
            };
            datavec
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut j = disk.len() - 1;

    while disk[i].is_some() {
        i += 1;
    }
    while disk[j].is_none() {
        j -= 1;
    }
    while i < j {
        disk.swap(i, j);
        while disk[i].is_some() {
            i += 1;
        }
        while disk[j].is_none() {
            j -= 1;
        }
    }
    Some(
        disk.iter()
            .filter_map(|d| *d)
            .enumerate()
            .fold(0, |acc, (i, d)| acc + i as u64 * d),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut id: u64 = 0;
    let mut disk = input
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(i, c)| {
            let is_data = i % 2 == 0;
            let len = c.to_digit(10).unwrap() as usize;
            let data = if is_data {
                DiskSpace::Data(len, id)
            } else {
                DiskSpace::Empty(len)
            };
            if is_data {
                id += 1
            };
            data
        })
        .collect::<Vec<_>>();

    let id = if let DiskSpace::Data(_, id) = disk.last().unwrap() {
        id
    } else {
        panic!()
    };

    (1..=*id).rev().for_each(|id| {
        let (data_pos, &data_size) = &disk
            .iter()
            .enumerate()
            .find_map(|(i, d)| {
                if let DiskSpace::Data(len, data) = d {
                    if *data == id {
                        Some((i, len))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap();

        let empty_candidate = &disk.iter().enumerate().find_map(|(i, e)| {
            if let DiskSpace::Empty(space_available) = e {
                if *space_available >= data_size && i < *data_pos {
                    Some((i, space_available))
                } else {
                    None
                }
            } else {
                None
            }
        });

        if let Some((empty_pos, &empty_len)) = empty_candidate {
            disk.swap(*data_pos, *empty_pos);
            if empty_len > data_size {
                if let DiskSpace::Empty(_) = disk[*data_pos] {
                    disk[*data_pos] = DiskSpace::Empty(data_size);
                } else {
                    unreachable!("swap should have put an empty space in the data position");
                }
                if let DiskSpace::Empty(len) = disk[empty_pos + 1] {
                    disk[empty_pos + 1] = DiskSpace::Empty(len + empty_len - data_size);
                } else {
                    disk.insert(empty_pos + 1, DiskSpace::Empty(empty_len - data_size))
                };
            }
        }
    });

    let crc = disk
        .iter()
        .flat_map(|d| match d {
            DiskSpace::Data(len, id) => std::iter::repeat(Some(*id)).take(*len),
            DiskSpace::Empty(len) => std::iter::repeat(None).take(*len),
        })
        .enumerate()
        .fold(0, |acc, (i, d)| acc + i as u64 * d.unwrap_or(0));

    Some(crc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(1928))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(2858))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
