/*

doubly linked list of heap-allocated strings
A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.
 */
use std::fmt::Debug;

trait ILinkedList {
    fn new() -> Self;
    fn get_head(&self) -> Option<&Node<String>>;
    fn get_tail(&self) -> Option<&Node<String>>;
    fn insert(&mut self, value: String) -> bool;
    fn find(&self, value: &str) -> Option<&Node<String>>;
    fn delete(&mut self, value: &str) -> Option<Node<String>>;
    fn len(&self) -> u32;
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node<String>>>,
    tail: Option<Box<Node<String>>>,
    len: u32
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None
        }
    }
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0
        }
    }
}

impl ILinkedList for LinkedList {
    fn new() -> Self {
        Self::new()
    }

    fn get_head(&self) -> Option<&Node<String>> {
        self.head.as_deref()
    }

    fn get_tail(&self) -> Option<&Node<String>> {
        self.tail.as_deref()
    }

    fn insert(&mut self, value: String) -> bool {
        let new_node = Box::new(Node::new(value));
        
        match self.head.take() {
            None => {
                // Empty list
                self.head = Some(new_node);
                self.tail = self.head.clone();
                self.len = 1;
            }
            Some(old_head) => {
                // Non-empty list
                let mut new_node = new_node;
                new_node.next = Some(old_head);
                self.head = Some(new_node);
                self.len += 1;
            }
        }
        true
    }

    fn find(&self, value: &str) -> Option<&Node<String>> {
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            if node.value == value {
                return Some(node);
            }
            current = node.next.as_deref();
        }
        None
    }

    fn delete(&mut self, value: &str) -> Option<Node<String>> {
        if self.head.is_none() {
            return None;
        }

        let current = self.head.take();
        
        // Check if head is the target
        if let Some(ref head) = current {
            if head.value == value {
                self.len -= 1;
                self.head = head.next.clone();
                return Some(*current.unwrap());
            }
        }
        
        // If head wasn't the target, restore it
        self.head = current.clone();
        
        // Search through the rest of the list
        let mut current_ref = self.head.as_mut();
        while let Some(node) = current_ref {
            if let Some(next) = node.next.as_mut() {
                if next.value == value {
                    let target = node.next.take();
                    node.next = target.as_ref().unwrap().next.clone();
                    self.len -= 1;
                    return Some(*target.unwrap());
                }
            }
            current_ref = node.next.as_mut();
        }
        None
    }

    fn len(&self) -> u32 {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list = LinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.get_head().is_none());
        assert!(list.get_tail().is_none());
    }

    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        assert!(list.insert(String::from("first")));
        assert_eq!(list.len(), 1);
        assert_eq!(list.get_head().unwrap().value, "first");
        
        assert!(list.insert(String::from("second")));
        assert_eq!(list.len(), 2);
        assert_eq!(list.get_head().unwrap().value, "second");
    }

    #[test]
    fn test_find() {
        let mut list = LinkedList::new();
        list.insert(String::from("first"));
        list.insert(String::from("second"));
        list.insert(String::from("third"));

        assert!(list.find("second").is_some());
        assert!(list.find("fourth").is_none());
        assert_eq!(list.find("first").unwrap().value, "first");
    }

    #[test]
    fn test_delete() {
        let mut list = LinkedList::new();
        list.insert(String::from("first"));
        list.insert(String::from("second"));
        list.insert(String::from("third"));

        assert!(list.delete("second").is_some());
        assert_eq!(list.len(), 2);
        assert!(list.find("second").is_none());

        assert!(list.delete("fourth").is_none());
        assert_eq!(list.len(), 2);

        assert!(list.delete("first").is_some());
        assert_eq!(list.len(), 1);
        assert!(list.find("first").is_none());
    }
}


