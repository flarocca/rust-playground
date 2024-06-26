#[derive(Debug, Clone)]
enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

impl<T> Link<T> {
    fn new_with_elem(elem: T) -> Self {
        Self::More(Box::new(Node {
            elem,
            next: Link::Empty,
        }))
    }

    fn new_with_elem_and_next(elem: T, next: Link<T>) -> Self {
        Self::More(Box::new(Node { elem, next }))
    }
}

impl<T> Default for Link<T> {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn is_last(&self) -> bool {
        matches!(self.next, Link::Empty)
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    length: usize,
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: Link::Empty,
            length: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut current = &mut self.head;
        loop {
            match current {
                Link::Empty => {
                    let _ = std::mem::replace(current, Link::new_with_elem(elem));
                    break;
                }
                Link::More(node) if node.is_last() => {
                    let _ = std::mem::replace(&mut node.next, Link::new_with_elem(elem));
                    break;
                }
                Link::More(node) => {
                    current = &mut node.next;
                }
            }
        }
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        loop {
            match current {
                Link::Empty => {
                    return None;
                }
                Link::More(node) if node.is_last() => {
                    let result = Some(node.elem);
                    let _ = std::mem::replace(current, Link::Empty);

                    self.length -= 1;
                    return result;
                }
                Link::More(node) => {
                    current = &mut node.next;
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn insert_at(&mut self, elem: T, position: usize) -> bool {
        let mut current_position = 0;
        let mut current = &mut self.head;
        loop {
            match current {
                Link::Empty if current_position < position => return false,
                Link::Empty => {
                    self.push(elem);
                    return true;
                }
                Link::More(node) if current_position + 1 == position => {
                    let mut new =
                        Link::new_with_elem_and_next(elem, std::mem::take(&mut node.next));

                    std::mem::swap(&mut node.next, &mut new);
                    self.length += 1;
                    return true;
                }
                Link::More(node) => {
                    current = &mut node.next;
                }
            }

            current_position += 1;
        }
    }

    pub fn remove_at(&mut self, position: usize) -> Option<T> {
        let mut current_position = 0;
        let mut current = &mut self.head;
        loop {
            match current {
                Link::Empty if current_position < position => return None,
                Link::Empty => {
                    return self.pop();
                }
                Link::More(node) if current_position + 1 == position => {
                    if let Link::More(next) = &node.next {
                        let result = next.elem.clone();
                        node.next = next.next.clone();
                        self.length -= 1;
                        return Some(result);
                    }
                }
                Link::More(node) => {
                    current = &mut node.next;
                }
            }

            current_position += 1;
        }
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: &self.head,
            position: 0,
        }
    }
}

impl<T: Copy> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LinkedListIterator<'a, T> {
    current: &'a Link<T>,
    position: usize,
}

impl<'a, T: Copy> Iterator for LinkedListIterator<'a, T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Link::Empty => None,
            Link::More(node) => {
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
