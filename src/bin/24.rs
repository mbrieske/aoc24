use std::collections::HashMap;
use std::collections::VecDeque;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
enum Gate<'a> {
    And(Option<bool>, &'a str),
    Or(Option<bool>, &'a str),
    Xor(Option<bool>, &'a str),
}

pub fn part_one(input: &str) -> Option<usize> {
    let (nets, gates) = input.split_once("\n\n").unwrap();
    let mut nets = nets
        .lines()
        .map(|line| {
            let (net, v) = line.split_once(": ").unwrap();
            let v = match v {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            };
            (net, v)
        })
        .collect::<VecDeque<_>>();
    let mut gates_vec = Vec::new();
    let mut gates_map: HashMap<&str, Vec<usize>> = HashMap::new();
    for line in gates.lines() {
        let (inp, outp) = line.split_once(" -> ").unwrap();
        let inp = inp.split(" ").collect::<Vec<_>>();
        let gate = match inp[1] {
            "AND" => Gate::And(None, outp),
            "OR" => Gate::Or(None, outp),
            "XOR" => Gate::Xor(None, outp),
            _ => unreachable!(),
        };

        gates_vec.push(gate);
        gates_map
            .entry(inp[0])
            .or_default()
            .push(gates_vec.len() - 1);
        gates_map
            .entry(inp[2])
            .or_default()
            .push(gates_vec.len() - 1);
    }

    let mut zs: Vec<Option<bool>> = Vec::new();

    while let Some((net, v1)) = nets.pop_front() {
        if let Some(connected_gates) = gates_map.get(net) {
            for gate_idx in connected_gates {
                let gate = gates_vec.get_mut(*gate_idx).unwrap();
                match gate {
                    Gate::And(None, outp) => {
                        *gate = Gate::And(Some(v1), outp);
                    }
                    Gate::Or(None, outp) => {
                        *gate = Gate::Or(Some(v1), outp);
                    }
                    Gate::Xor(None, outp) => {
                        *gate = Gate::Xor(Some(v1), outp);
                    }
                    Gate::And(Some(v2), outp) => {
                        let v_out = v1 & *v2;
                        nets.push_back((outp, v_out));
                    }
                    Gate::Or(Some(v2), outp) => {
                        let v_out = v1 | *v2;
                        nets.push_back((outp, v_out));
                    }
                    Gate::Xor(Some(v2), outp) => {
                        let v_out = v1 ^ *v2;
                        nets.push_back((outp, v_out));
                    }
                }
            }
        }
        if net.starts_with("z") {
            let (_, nbr) = net.split_once("z").unwrap();
            let nbr = nbr.parse::<usize>().unwrap();
            if nbr >= zs.len() {
                zs.resize(nbr + 1, None);
            }
            zs[nbr] = Some(v1);
        }
    }

    let res = zs.iter().enumerate().fold(0, |acc, (idx, v)| {
        let v = v.unwrap() as usize;
        acc | v << idx
    });
    Some(res as usize)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(4))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(2024))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<usize>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<usize>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
