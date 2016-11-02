/// Binary Search Tree

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, elem: T) -> bool {        
        let mut cur_link = &mut self.root;
        
        loop {
            let tmp = cur_link;
            if let &mut Some(ref mut node) = tmp {
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

        *cur_link = Some(box Node {
            elem: elem,
            left: None,
            right:None,
        });

        true
     }

    pub fn search(&self, elem: T) -> bool {
        let mut cur_link = &self.root;
        
        while let &Some(ref node) = cur_link {
            if elem < node.elem {
                cur_link = &node.left;
            } else if elem == node.elem {
                return true
            } else {
                cur_link = &node.right
            }
        }

        false
    }
}

impl<'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            next : self.root.as_mut().map(|node| {
                &mut **node
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_mut().map(|node| {
                &mut **node
            });
            
            &mut node.elem
        })
    }
}

pub struct IterMut<'a, T:'a> {
    next: Option<&'a mut Node<T>>
}


impl<'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            next : self.root.as_ref().map(|node| {
                &**node
            })
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.right.as_ref().map(|node| {
                &**node
            });
            
            &node.elem
        })
    }
}

pub struct Iter<'a, T:'a> {
    next: Option<&'a Node<T>>
}

impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(BST<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.root.take().map(|box node| {
            self.0.root = node.right;
            node.elem
        })
    }
}

impl<T> Drop for BST<T> {
    fn drop(&mut self) {
        let mut stack = vec![];
        stack.push(self.root.take());

        while let Some(Some(mut node)) = stack.pop() {
            if node.left.is_some() {
                stack.push(node.left.take());
            }

            if node.right.is_some() {
                stack.push(node.right.take());
            }
        }
    }
}

impl<T> Default for BST<T> {
    fn default() -> Self {
        BST { root: None }
    }
}


#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_iter_mut() {
        let mut bst = BST::new();
        
        for i in 0..100 {
            bst.insert(i);
        }
        
        for (i, r) in (&mut bst).into_iter().enumerate() {
            assert!(i == *r);
            *r += 10;
        }

        for (i, r) in (&mut bst).into_iter().enumerate() {
            assert!(i + 10 == *r);
        }

    }

    #[test]
    fn test_iter() {
        let mut bst = BST::new();
        for i in 0..100 {
            bst.insert(i);
        }

        for (i, r) in (&bst).into_iter().enumerate() {
            assert!(i == *r);
        }
    }

    #[test]
    fn test_into_iter() {
        let mut bst = BST::new();
        for i in 0..100 {
            bst.insert(i);
        }

        for (i, r) in bst.into_iter().enumerate() {
            assert!(i == r);
        }
    }

    #[test]
    fn test_insert() {
        let mut bst = BST::new();
        assert!(bst.insert(1));
        assert!(!bst.insert(1));
        assert!(bst.insert(10));
        assert!(bst.insert(4));
        assert!(bst.insert(7));
        assert!(bst.insert(2));


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
