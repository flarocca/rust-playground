use std::mem;

type Link<'a, T> = Option<Box<BinaryTree<'a, T>>>;

#[derive(PartialEq, Debug)]
struct BinaryTree<'a, T> {
    elem: &'a T,
    left: Link<'a, T>,
    right: Link<'a, T>,
}

impl<'a, T: PartialEq + PartialOrd> BinaryTree<'a, T> {
    pub fn insert(&mut self, new_elem: &'a T) {
        if self.elem == new_elem {
            return;
        }

        let target_node = if new_elem < self.elem {
            &mut self.left
        } else {
            &mut self.right
        };

        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_elem),
            &mut None => {
                let new_node = BinaryTree {
                    elem: new_elem,
                    left: None,
                    right: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}
