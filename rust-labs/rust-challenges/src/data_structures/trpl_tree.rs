/*
 * This is a Tree DS created as example in the TRPL Book Chapter 15
 * */

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });

        let branch = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    }
}
