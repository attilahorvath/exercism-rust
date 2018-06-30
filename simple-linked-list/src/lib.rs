use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

pub struct LinkedListIter<'a, T: 'a> {
    node: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(node) = self.node {
            self.node = &node.next;
            Some(&node.data)
        } else {
            None
        }
    }
}

pub struct LinkedListIntoIter<T> {
    node: Option<Box<Node<T>>>,
}

impl<T> Iterator for LinkedListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(mut node) = self.node.take() {
            self.node = node.next.take();
            Some(node.data)
        } else {
            None
        }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node::new(element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut head) = self.head.take() {
            self.head = head.next.take();
            Some(head.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.iter().next()
    }

    fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter { node: &self.head }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = SimpleLinkedList::new();

        for i in iter {
            list.push(i);
        }

        list
    }
}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIntoIter<T>;

    fn into_iter(self) -> LinkedListIntoIter<T> {
        LinkedListIntoIter { node: self.head }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        self.iter().cloned().collect()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        item.iter().cloned().collect()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = self.into_iter().collect::<Vec<_>>();

        vec.reverse();
        vec
    }
}
