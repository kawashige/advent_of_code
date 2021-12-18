use proconio::input;
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: Option<usize>,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: usize) -> Self {
        TreeNode {
            val: Some(val),
            left: None,
            right: None,
        }
    }
}

fn parse(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let mut stack = vec![vec![]];
    for c in s.chars() {
        match c {
            '[' => {
                stack.push(Vec::new());
            }
            ']' => {
                let mut pair = stack.pop().unwrap();
                (*stack.last_mut().unwrap()).push(TreeNode {
                    val: None,
                    right: Some(Rc::new(RefCell::new(pair.pop().unwrap()))),
                    left: Some(Rc::new(RefCell::new(pair.pop().unwrap()))),
                });
            }
            ',' => {
                // noop
            }
            _ => {
                (*stack.last_mut().unwrap()).push(TreeNode::new(c.to_digit(10).unwrap() as usize));
            }
        }
    }
    Some(Rc::new(RefCell::new(stack.pop().unwrap().pop().unwrap())))
}

fn solve(numbers: &Vec<String>) -> usize {
    let mut number = parse(&numbers[0]);
    for i in 1..numbers.len() {
        number = add(number, parse(&numbers[i]));
    }

    calc_magnitude(&number)
}

fn solve2(numbers: &Vec<String>) -> usize {
    (0..numbers.len())
        .map(|i| {
            (0..numbers.len())
                .map(|j| {
                    let x = if i == j {
                        0
                    } else {
                        let y = add(parse(&numbers[i]), parse(&numbers[j]));
                        calc_magnitude(&y)
                    };

                    x
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn add(
    num1: Option<Rc<RefCell<TreeNode>>>,
    num2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut added = Some(Rc::new(RefCell::new(TreeNode {
        val: None,
        left: num1,
        right: num2,
    })));

    while reduce(&mut added, 1).is_some() || split(&mut added) {}

    added
}

fn reduce(number: &mut Option<Rc<RefCell<TreeNode>>>, depth: usize) -> Option<(usize, usize)> {
    if let Some(num) = number {
        let mut n = num.borrow_mut();
        if depth >= 5
            && n.val.is_none()
            && n.left.as_ref().unwrap().borrow().val.is_some()
            && n.right.as_ref().unwrap().borrow().val.is_some()
        {
            n.val = Some(0);
            let left_val = n.left.as_ref().unwrap().borrow().val.unwrap();
            let right_val = n.right.as_ref().unwrap().borrow().val.unwrap();
            n.left = None;
            n.right = None;

            Some((left_val, right_val))
        } else {
            if let Some((l_l, l_r)) = reduce(&mut n.left, depth + 1) {
                add_left(&mut n.right, l_r);
                Some((l_l, 0))
            } else {
                if let Some((r_l, r_r)) = reduce(&mut n.right, depth + 1) {
                    add_right(&mut n.left, r_l);
                    Some((0, r_r))
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn split(number: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(num) = number {
        let mut n = num.borrow_mut();
        if n.val.is_some() {
            if n.val.unwrap() > 9 {
                let v = n.val.unwrap();
                n.left = Some(Rc::new(RefCell::new(TreeNode::new(v / 2))));
                n.right = Some(Rc::new(RefCell::new(TreeNode::new((v + 1) / 2))));
                n.val = None;
                true
            } else {
                false
            }
        } else {
            split(&mut n.left) || split(&mut n.right)
        }
    } else {
        false
    }
}
fn add_right(number: &mut Option<Rc<RefCell<TreeNode>>>, val: usize) {
    if let Some(num) = number {
        let mut n = num.borrow_mut();
        if n.val.is_some() {
            n.val = Some(n.val.unwrap() + val);
        } else {
            add_right(&mut n.right, val);
        }
    }
}

fn add_left(number: &mut Option<Rc<RefCell<TreeNode>>>, val: usize) {
    if let Some(num) = number {
        let mut n = num.borrow_mut();
        if n.val.is_some() {
            n.val = Some(n.val.unwrap() + val);
        } else {
            add_left(&mut n.left, val);
        }
    }
}

fn calc_magnitude(number: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    if let Some(num) = number {
        let n = num.borrow();
        if let Some(v) = n.val {
            v
        } else {
            calc_magnitude(&n.left) * 3 + calc_magnitude(&n.right) * 2
        }
    } else {
        0 // unreachable
    }
}

fn main() {
    input! {
        n: usize,
        numbers: [String; n]
    }

    println!("part1: {}", solve(&numbers));
    println!("part2: {}", solve2(&numbers));
}
