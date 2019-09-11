use std::collections::{HashMap, HashSet};

const TRIANGLE: &str = "\
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

type Value = u32;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Location(usize, usize);

pub fn maximum_path_sum(triangle: &str) -> Value {
    let triangle = triangle.to_string();
    let numbers = triangle
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|digits_str| digits_str.parse::<Value>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let bottom_index = numbers.len() - 1;

    let mut node: Location = Location(0, 0);

    let mut came_from: HashMap<Location, Location> = HashMap::new();

    let mut path_cost = HashMap::new();
    path_cost.insert(node.clone(), 99 - numbers[0][0]);

    let mut frontier: HashSet<Location> = HashSet::new();
    frontier.insert(node.clone());

    let mut explored: HashSet<Location> = HashSet::new();

    loop {
        if frontier.is_empty() {
            panic!("fail");
        }

        node = frontier
            .iter()
            .fold(None, |acc, loc| {
                if acc.is_none()
                    || path_cost.get(&loc).unwrap() < path_cost.get(acc.unwrap()).unwrap()
                {
                    Some(loc)
                } else {
                    acc
                }
            })
            .unwrap()
            .clone();
        frontier.remove(&node);

        let Location(a, b) = node.clone();
        if a == bottom_index {
            break;
        }

        explored.insert(node.clone());

        let left = Location(a + 1, b);
        let right = Location(a + 1, b + 1);
        let children = vec![left, right];

        for child in children {
            let Location(x, y) = child.clone();

            if numbers.get(x).and_then(|_| numbers.get(y)).is_none() {
                continue;
            }

            let child_path_cost = path_cost.get(&node).unwrap() + (99 - numbers[x][y]);

            if frontier.contains(&child) {
                if *path_cost.get(&child).unwrap() > child_path_cost {
                    path_cost.insert(child.clone(), child_path_cost);
                    came_from.insert(child, node.clone());
                }
            } else if !explored.contains(&child) {
                frontier.insert(child.clone());
                path_cost.insert(child.clone(), child_path_cost);
                came_from.insert(child, node.clone());
            }
        }
    }

    let mut path = vec![];
    loop {
        path.push(node.clone());
        if let Some(parent) = came_from.get(&node) {
            node = parent.clone();
        } else {
            break;
        }
    }

    path.into_iter()
        .map(|Location(x, y)| numbers[x][y])
        .sum::<Value>()
}

pub fn solve() -> String {
    let max = maximum_path_sum(TRIANGLE);
    max.to_string()
}
