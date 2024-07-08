#[derive(Debug, Clone)]
pub enum LinkedList<T> {
    Empty,
    More(Box<Node<T>>),
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    elem: T,
    next: LinkedList<T>,
}

impl<T: Clone + Copy> LinkedList<T> {
    pub fn new() -> Self {
        Self::Empty
    }

    fn new_with_elem(elem: T) -> Self {
        Self::new_with_elem_and_next(elem, LinkedList::Empty)
    }

    fn new_with_elem_and_next(elem: T, next: LinkedList<T>) -> Self {
        Self::More(Box::new(Node { elem, next }))
    }

    fn priv_insert_at(&mut self, elem: T, position: usize, current: usize) -> bool {
        match self {
            LinkedList::Empty if current < position => false,
            LinkedList::Empty => {
                self.push(elem);
                true
            }
            LinkedList::More(node) if current + 1 == position => {
                let mut new =
                    LinkedList::new_with_elem_and_next(elem, std::mem::take(&mut node.next));

                std::mem::swap(&mut node.next, &mut new);
                true
            }
            LinkedList::More(node) => {
                Self::priv_insert_at(&mut node.next, elem, position, current + 1)
            }
        }
    }

    fn priv_remove_at(&mut self, position: usize, current: usize) -> Option<T> {
        match self {
            LinkedList::Empty => None,
            LinkedList::More(_) if current == position => self.pop(),
            LinkedList::More(node) if current + 1 == position => {
                if let LinkedList::More(next) = &node.next {
                    let result = next.elem;
                    node.next = next.next.clone();
                    return Some(result);
                }

                None
            }
            LinkedList::More(node) => Self::priv_remove_at(&mut node.next, position, current + 1),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            LinkedList::Empty => 0,
            LinkedList::More(node) => 1 + Self::len(&node.next),
        }
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: &self,
            position: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        match self {
            LinkedList::Empty => {
                let _ = std::mem::replace(self, LinkedList::new_with_elem(elem));
            }
            LinkedList::More(node) => {
                Self::push(&mut node.next, elem);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            LinkedList::Empty => None,
            LinkedList::More(node) if node.is_last() => {
                let result = Some(node.elem);
                let _ = std::mem::replace(self, LinkedList::Empty);

                result
            }
            LinkedList::More(node) => Self::pop(&mut node.next),
        }
    }

    pub fn insert_at(&mut self, elem: T, position: usize) -> bool {
        self.priv_insert_at(elem, position, 0)
    }

    pub fn remove_at(&mut self, position: usize) -> Option<T> {
        self.priv_remove_at(position, 0)
    }
}

impl<T: Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T: Clone> Node<T> {
    fn is_last(&self) -> bool {
        matches!(self.next, LinkedList::Empty)
    }
}

pub struct LinkedListIterator<'a, T: Clone> {
    current: &'a LinkedList<T>,
    position: usize,
}

impl<'a, T: Copy> Iterator for LinkedListIterator<'a, T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            LinkedList::Empty => None,
            LinkedList::More(node) => {
                self.current = &node.next;
                self.position += 1;
                return Some((self.position - 1, node.elem));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let list = LinkedList::<i32>::new();

        assert_eq!(list.len(), 0);
    }

    #[test]
    fn can_push_elements() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);
        list.push(10);
        list.push(15);

        assert_eq!(list.len(), 3);
    }

    #[test]
    fn can_pop_elements() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);
        list.push(10);
        list.push(15);

        assert_eq!(list.pop(), Some(15));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn popping_all_elements_empties_the_list() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);
        list.push(10);
        list.push(15);

        let _ = list.pop();
        let _ = list.pop();
        let _ = list.pop();

        assert_eq!(list.len(), 0);
    }

    #[test]
    fn can_iterate_elements() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);
        list.push(10);
        list.push(15);

        let mut elems = Vec::new();

        for elem in list.iter() {
            elems.push(elem);
        }

        assert_eq!(elems.iter().len(), 3);
        assert_eq!(elems[0], (0, 5));
        assert_eq!(elems[1], (1, 10));
        assert_eq!(elems[2], (2, 15));
    }

    #[test]
    fn can_insert_at() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);
        list.push(10);
        list.push(15);
        list.push(20);

        list.insert_at(12, 2);

        assert_eq!(list.len(), 5);

        let mut elems = Vec::new();

        for elem in list.iter() {
            elems.push(elem);
        }

        assert_eq!(elems[0], (0, 5));
        assert_eq!(elems[1], (1, 10));
        assert_eq!(elems[2], (2, 12));
        assert_eq!(elems[3], (3, 15));
        assert_eq!(elems[4], (4, 20));
    }

    #[test]
    fn can_insert_at_last_position() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);
        list.push(10);
        list.push(15);
        list.push(20);

        list.insert_at(12, 4);

        assert_eq!(list.len(), 5);

        let mut elems = Vec::new();

        for elem in list.iter() {
            elems.push(elem);
        }

        assert_eq!(elems[0], (0, 5));
        assert_eq!(elems[1], (1, 10));
        assert_eq!(elems[2], (2, 15));
        assert_eq!(elems[3], (3, 20));
        assert_eq!(elems[4], (4, 12));
    }

    #[test]
    fn can_insert_at_0_on_empty_list() {
        let mut list = LinkedList::<i32>::new();

        list.insert_at(12, 0);

        assert_eq!(list.len(), 1);

        let mut elems = Vec::new();

        for elem in list.iter() {
            elems.push(elem);
        }

        assert_eq!(elems[0], (0, 12));
    }

    #[test]
    fn cannot_insert_at_out_of_bound_on_empty_list() {
        let mut list = LinkedList::<i32>::new();

        let succeeded = list.insert_at(12, 1);

        assert_eq!(list.len(), 0);
        assert!(!succeeded);
    }

    #[test]
    fn cannot_insert_at_out_of_bound_on_non_empty_list() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);

        let succeeded = list.insert_at(12, 2);

        assert_eq!(list.len(), 1);
        assert!(!succeeded);
    }

    #[test]
    fn can_remove_at() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);
        list.push(10);
        list.push(12);
        list.push(15);
        list.push(20);

        let removed = list.remove_at(2);

        assert_eq!(list.len(), 4);
        assert_eq!(removed.unwrap(), 12);

        let mut elems = Vec::new();

        for elem in list.iter() {
            elems.push(elem);
        }

        assert_eq!(elems[0], (0, 5));
        assert_eq!(elems[1], (1, 10));
        assert_eq!(elems[2], (2, 15));
        assert_eq!(elems[3], (3, 20));
    }

    #[test]
    fn can_remove_at_last_position() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);
        list.push(10);
        list.push(12);
        list.push(15);
        list.push(20);

        let removed = list.remove_at(4);

        assert_eq!(list.len(), 4);
        assert_eq!(removed.unwrap(), 20);

        let mut elems = Vec::new();

        for elem in list.iter() {
            elems.push(elem);
        }

        assert_eq!(elems[0], (0, 5));
        assert_eq!(elems[1], (1, 10));
        assert_eq!(elems[2], (2, 12));
        assert_eq!(elems[3], (3, 15));
    }

    #[test]
    fn can_remove_at_0_on_list_with_one_element() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);

        let removed = list.remove_at(0);

        assert_eq!(list.len(), 0);
        assert_eq!(removed.unwrap(), 5);
    }

    #[test]
    fn cannot_remove_at_out_of_bound_on_empty_list() {
        let mut list = LinkedList::<i32>::new();

        let succeeded = list.remove_at(1);

        assert!(succeeded.is_none());
    }

    #[test]
    fn cannot_remove_at_out_of_bound_on_non_empty_list() {
        let mut list = LinkedList::<i32>::new();
        list.push(5);

        let succeeded = list.remove_at(2);

        assert_eq!(list.len(), 1);
        assert!(succeeded.is_none());
    }
}
