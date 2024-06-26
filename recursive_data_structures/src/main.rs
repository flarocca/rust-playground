use linked_list::LinkedList;

mod common;
mod double_linked_list;
mod linked_list;

fn main() {
    let mut list = LinkedList::<i32>::new();

    list.push(5);
    list.push(10);
    list.insert_at(15, 2);

    assert_eq!(list.len(), 2);

    let elem = list.pop();

    assert_eq!(list.len(), 1);
    assert_eq!(elem.unwrap(), 10);

    let _ = list.remove_at(0);

    for (pos, elem) in list.iter() {
        println!("Element {elem} at position {pos}");
    }

    println!("Hello world!!!");
}
