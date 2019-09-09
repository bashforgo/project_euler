use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    rc::Rc,
};

const TRINGLE: &str = "\
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

type Value = u8;

#[derive(PartialEq, Eq, Hash)]
enum TreeImpl {
    Node {
        value: Value,
        left: Tree,
        right: Tree,
    },
    Leaf,
}
#[derive(PartialEq, Eq, Hash)]
struct Tree(Rc<RefCell<TreeImpl>>);

impl Tree {
    fn new_leaf() -> Tree {
        Tree(Rc::new(RefCell::new(TreeImpl::Leaf)))
    }

    fn new_node(value: Value) -> Tree {
        Tree(Rc::new(RefCell::new(TreeImpl::Node {
            value,
            left: Tree::new_leaf(),
            right: Tree::new_leaf(),
        })))
    }

    fn update_left(&self, new_left: Tree) {
        let mut parent = self.0.borrow_mut();
        if let TreeImpl::Node { left, .. } = &mut *parent {
            left.0 = new_left.0;
        }
    }

    fn update_right(&self, new_right: Tree) {
        let mut parent = self.0.borrow_mut();
        if let TreeImpl::Node { right, .. } = &mut *parent {
            right.0 = new_right.0;
        }
    }

    fn is_node(&self) -> bool {
        let inner = self.0.borrow();
        match *inner {
            TreeImpl::Leaf => false,
            _ => true,
        }
    }

    fn get_with<F: Fn(&TreeImpl) -> &T, T>(&self, choose: F) -> Option<Ref<T>> {
        if self.is_node() {
            let inner = self.0.borrow();
            Some(Ref::map(inner, choose))
        } else {
            None
        }
    }

    fn value(&self) -> Option<Ref<Value>> {
        self.get_with(|t| {
            if let TreeImpl::Node { value, .. } = t {
                value
            } else {
                unreachable!()
            }
        })
    }

    fn left(&self) -> Option<Ref<Tree>> {
        self.get_with(|t| {
            if let TreeImpl::Node { left, .. } = t {
                left
            } else {
                unreachable!()
            }
        })
    }

    fn right(&self) -> Option<Ref<Tree>> {
        self.get_with(|t| {
            if let TreeImpl::Node { right, .. } = t {
                right
            } else {
                unreachable!()
            }
        })
    }
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Tree(Rc::clone(&self.0))
    }
}

pub fn solve() -> String {
    let triangle = TRINGLE.to_string();
    let mut numbers = triangle.lines().map(|line| {
        line.split(' ')
            .map(|digits_str| digits_str.parse::<Value>().unwrap())
            .collect::<Vec<_>>()
    });

    let first = numbers.next().unwrap();
    let head = Tree::new_node(first[0]);
    let mut last_row = vec![Tree::clone(&head)];

    for line in numbers {
        let mut row = Vec::with_capacity(line.len());
        for (i, number) in (0..).zip(line) {
            let node = Tree::new_node(number);
            row.push(Tree::clone(&node));
            {
                let node = Tree::clone(&node);
                if i >= 1 {
                    let parent = &last_row[i - 1];
                    parent.update_right(node);
                }
            }
            {
                let node = Tree::clone(&node);
                if last_row.len() > i {
                    let parent = &last_row[i];
                    parent.update_left(node);
                }
            }
        }
        last_row = row;
    }

    fn ucs(start: Tree) -> Vec<Tree> {
        let mut frontier = HashSet::<Tree>::new();
        let mut explored = HashSet::new();
        let mut path_cost = HashMap::new();

        loop {
            if frontier.is_empty() {
                panic!("cannot find solution");
            }

            let node = frontier.iter().fold(None, |ret, n| {
                ret
            });

            // is goal

            explored.insert(node);

            
        }

        unimplemented!()
    }
    let path = ucs(Tree::clone(&head));
    path.iter().for_each(|t| {
        println!("{:?}", t.value());
    });

    unimplemented!()
}
