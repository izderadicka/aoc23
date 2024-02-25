use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Working,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Token {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Token::Working),
            '#' => Ok(Token::Damaged),
            '?' => Ok(Token::Unknown),
            _ => anyhow::bail!("Unknown token: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct State {
    pos: usize,
    group_pos: usize,
    group_size: u16,
}

type Cache = std::collections::HashMap<State, u64>;

fn process_mem(cache: &mut Cache, pattern: &[Token], groups: &[u16], state: State) -> u64 {
    if let Some(res) = cache.get(&state) {
        // println!("For state {:?} result is {}", state, res);
        return *res;
    }
    let res = process_rec(cache, pattern, groups, state.clone());
    cache.insert(state, res);
    res
}

fn process_rec(cache: &mut Cache, pattern: &[Token], groups: &[u16], mut state: State) -> u64 {
    if state.pos >= pattern.len() {
        if state.group_pos == groups.len() && state.group_size == 0 {
            1
        } else if state.group_pos == groups.len() - 1 && state.group_size == groups[state.group_pos]
        {
            1
        } else {
            0
        }
    } else {
        let mut res = 0;

        let current = pattern[state.pos];
        state.pos += 1;
        if current == Token::Working || current == Token::Unknown {
            let mut state = state.clone();
            if state.group_size == 0 {
                res += process_mem(cache, pattern, groups, state);
            } else if state.group_pos < groups.len() && groups[state.group_pos] == state.group_size
            {
                state.group_pos += 1;
                state.group_size = 0;
                res += process_mem(cache, pattern, groups, state);
            }
        }
        if current == Token::Damaged || current == Token::Unknown {
            state.group_size += 1;
            res += process_mem(cache, pattern, groups, state);
        }
        res
    }
}

pub fn twelveth_task_1(f: impl BufRead) -> u64 {
    let mut sum = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();
        let pattern = iter.next().unwrap();
        let groups: Vec<_> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        let pattern2 = pattern
            .chars()
            .map(|x| x.try_into().unwrap())
            .collect::<Vec<_>>();
        let mut cache = std::collections::HashMap::new();
        let variants = process_rec(&mut cache, &pattern2, &groups, State::default());
        println!("{} {:?} => {}", pattern, groups, variants);
        sum += variants;
    }

    sum
}

pub fn twelveth_task_2(f: impl BufRead) -> u64 {
    let mut sum = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();
        let pattern = iter.next().unwrap();
        let mut ext_pattern = pattern.to_string();
        for _ in 0..4 {
            ext_pattern.push('?');
            ext_pattern.push_str(pattern);
        }
        let groups: Vec<_> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        let mut ext_groups = groups.clone();
        for _ in 0..4 {
            ext_groups.extend(groups.iter());
        }
        let ext_pattern = ext_pattern
            .chars()
            .map(|x| x.try_into().unwrap())
            .collect::<Vec<_>>();
        let mut cache = std::collections::HashMap::new();
        let variants = process_rec(&mut cache, &ext_pattern, &ext_groups, State::default());
        println!("{} {:?} => {}", pattern, groups, variants);
        sum += variants;
    }

    sum
}
