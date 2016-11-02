/// Binary Search Tree

use std::mem;

#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl Default for BST {
    fn default() -> Self {
        BST { root: Link::Empty }
    }
}

impl BST {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, elem: i32) -> bool {        
        let mut cur_link = &mut self.root;
        
        loop {
            let tmp = cur_link;
            if let Link::More(ref mut node) = *tmp {
                if elem < node.elem {
                    cur_link = &mut node.left;
                } else if elem == node.elem {
                    return false
                } else {
                    cur_link = &mut node.right;
                }
            } else {
                cur_link = tmp;
                break
            }
        }
        
        *cur_link = Link::More(Box::new(Node {
            elem: elem,
            left: Link::Empty,
            right: Link::Empty,
        }));

        true
     }

    pub fn search(&self, elem: i32) -> bool {
        let mut cur_link = &self.root;

        while let Link::More(ref node) = *cur_link {
            if elem < node.elem {
                cur_link = &node.left;
            } else if elem == node.elem {
                return true
            } else {
                cur_link = &node.right;
            }
        }

        false
    }
}

impl Drop for BST {
    fn drop(&mut self) {
        let mut stack = vec![];
        stack.push(mem::replace(&mut self.root, Link::Empty));

        while let Some(Link::More(mut node)) = stack.pop() {
            if let Link::More(_) = node.left {
                stack.push(mem::replace(&mut node.left, Link::Empty));
            }

            if let Link::More(_) = node.right {
                stack.push(mem::replace(&mut node.right, Link::Empty));
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_insert() {
        let mut bst = BST::new();

        for i in 0 .. 10000 {
            assert!(bst.insert(i));
        }

        for i in 0 .. 10000 {
            assert!(bst.search(i));
        }        
    }

    #[test]
    fn test_search() {
        let mut bst = BST::new();
        assert!(bst.insert(1));
        assert!(bst.insert(10));
        assert!(bst.insert(4));
        assert!(bst.insert(7));
        assert!(bst.insert(2));

        assert!(bst.search(1));
        assert!(!bst.search(11));
        assert!(bst.search(7));
        assert!(bst.search(10));
        assert!(!bst.search(13));
        assert!(bst.search(4));
        assert!(bst.search(2));
        assert!(bst.search(10));
    }
}
