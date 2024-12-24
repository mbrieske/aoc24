use itertools::Itertools;
use petgraph::graph::Graph;
use petgraph::prelude::*;
use petgraph::Undirected;
use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(23);

fn generate_graph(input: &str) -> Graph<String, (), Undirected> {
    let mut graph: Graph<String, (), Undirected> = Graph::new_undirected();
    let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split('-').collect();

        let node1 = parts[0];
        let node2 = parts[1];

        let &mut index1 = node_map
            .entry(node1)
            .or_insert_with(|| graph.add_node(node1.to_string()));

        let &mut index2 = node_map
            .entry(node2)
            .or_insert_with(|| graph.add_node(node2.to_string()));

        graph.add_edge(index1, index2, ());
    }
    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let g = generate_graph(input);

    let mut cycles = HashSet::new();

    g.node_indices()
        .filter(|node| g[*node].starts_with("t"))
        .for_each(|node| {
            g.neighbors(node).combinations(2).for_each(|pair| {
                if g.neighbors(pair[0]).contains(&pair[1]) {
                    let mut cycle = vec![node, pair[0], pair[1]];
                    cycle.sort();
                    cycles.insert(cycle);
                }
            });
        });

    Some(cycles.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(7))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
