use regex::Regex;

advent_of_code::solution!(5);

fn rule_applies(update: &[u32], rule: (u32, u32)) -> bool {
    let (a, b) = rule;
    let ai = update
        .iter()
        .enumerate()
        .find(|&(_, &p)| p == a)
        .map(|(i, _)| i);
    if let Some(ai) = ai {
        let bi = update
            .iter()
            .enumerate()
            .find(|&(_, &p)| p == b)
            .map(|(i, _)| i);
        if let Some(bi) = bi {
            return ai < bi;
        }
    }
    true
}

fn fix_update(update: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let mut fixed_updates: Vec<Vec<u32>> = vec![Vec::new()];
    update.iter().for_each(|&p| {
        // for each version of fixed updates currently in the fixed_updates list,
        // insert p at all possible positions and then check if all rules apply
        // set fixed_updates to the list of all valid updates with p inserted
        fixed_updates = fixed_updates
            .clone()
            .into_iter()
            .flat_map(|fixed_update| {
                (0..=fixed_update.len())
                    .map(|i| {
                        let mut fixed_update_candidate = fixed_update.clone();
                        fixed_update_candidate.insert(i, p);
                        fixed_update_candidate
                    })
                    .filter(|update| rules.iter().all(|&rule| rule_applies(update, rule)))
                    .collect::<Vec<_>>()
            })
            .collect();
    });
    fixed_updates.pop().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (split_at, _) = input
        .lines()
        .enumerate()
        .find(|&(_i, line)| line.is_empty())
        .unwrap();
    let l: Vec<_> = input.lines().collect();
    let (rules, updates) = l.split_at(split_at);
    let updates = &updates[1..];
    let rules: Vec<_> = rules
        .iter()
        .map(|rule| rule.split_once("|").unwrap())
        .map(|(a, b)| {
            let re_str = format!(r"(^|,){},(?:\d+,)*{}(,|$)", b, a);
            Regex::new(&re_str).unwrap()
        })
        .collect();
    Some(
        updates
            .iter()
            .filter(|update| !rules.iter().any(|rule| rule.is_match(update)))
            .map(|update| {
                let pages = update.split(",").collect::<Vec<_>>();
                pages[(pages.len() - 1) / 2].parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (split_at, _) = input
        .lines()
        .enumerate()
        .find(|&(_i, line)| line.is_empty())
        .unwrap();
    let l: Vec<_> = input.lines().collect();
    let (rules, updates) = l.split_at(split_at);
    let rules: Vec<(u32, u32)> = rules
        .iter()
        .map(|rule| {
            let (a, b) = rule.split_once("|").unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    let updates = &updates[1..];
    let regexes: Vec<_> = rules
        .iter()
        .map(|(a, b)| {
            let re_str = format!(r"(^|,){},(?:\d+,)*{}(,|$)", b, a);
            Regex::new(&re_str).unwrap()
        })
        .collect();

    let invalid_updates: Vec<Vec<u32>> = updates
        .iter()
        .filter(|update| regexes.iter().any(|rule| rule.is_match(update)))
        .map(|update| {
            update
                .split(",")
                .map(|p| p.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    Some(
        invalid_updates
            .into_iter()
            .map(|update| {
                let fixed_update = fix_update(&update, &rules);
                fixed_update[(fixed_update.len() - 1) / 2]
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(143))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(123))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&[75,97,47,61,53], &[(47, 53), (97, 13), (97, 61), (97, 47), (75, 29), (61, 13), (75, 53), (29, 13),
    (97, 29), (53, 29), (61, 53), (97, 53), (61, 29), (47, 13), (75, 47), (97, 75),
    (47, 61), (75, 61), (47, 29), (75, 13), (53, 13)], &[97,75,47,61,53])]
    #[case(&[61,13,29], &[(47, 53), (97, 13), (97, 61), (97, 47), (75, 29), (61, 13), (75, 53), (29, 13),
    (97, 29), (53, 29), (61, 53), (97, 53), (61, 29), (47, 13), (75, 47), (97, 75),
    (47, 61), (75, 61), (47, 29), (75, 13), (53, 13)], &[61,29,13])]
    #[case(&[97,13,75,29,47], &[(47, 53), (97, 13), (97, 61), (97, 47), (75, 29), (61, 13), (75, 53), (29, 13),
    (97, 29), (53, 29), (61, 53), (97, 53), (61, 29), (47, 13), (75, 47), (97, 75),
    (47, 61), (75, 61), (47, 29), (75, 13), (53, 13)], &[97,75,47,29,13])]
    fn test_fix_update(
        #[case] update: &[u32],
        #[case] rules: &[(u32, u32)],
        #[case] expected: &[u32],
    ) {
        let result = fix_update(update, rules);
        assert_eq!(result, expected);
    }
}
