/*

doubly linked list of heap-allocated strings
A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.
 */
trait ILinkedList<T> {
    fn get_head()-> Node<Node<T>>;
    fn get_tail()-> Node<Node<T>>;
    fn insert()-> bool; // we will keep it simple by returning a boolean but we could have make another, more comprehensive behaviour like return the new total size or node position
    fn find(value: Node<String>)-> Node<Node<T>>;
    fn delete(value: Node<String>)-> Node<Node<T>>;
    
}
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct LinkedLinkedList {
    value: Node<String>,
    next: Node<Node<String>>, 
    prev: Node<Node<String>>, 
}


impl Node<String> {
    fn new(value: String) -> Self {
        todo!()
    }
}

impl LinkedLinkedList {
    fn new() -> Self {
       todo!() 
    }
}


