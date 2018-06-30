struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

struct LinkedListIterator<'a, T: 'a> {
    node: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<&'a Node<T>> {
        if let Some(node) = self.node {
            self.node = &node.next;
            Some(&node)
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
        if let Some(head) = &self.head {
            Some(&head.data)
        } else {
            None
        }
    }

    fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator { node: &self.head }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();

        for i in self.iter() {
            list.push(i.data.clone());
        }

        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();

        for i in item {
            list.push(i.clone());
        }

        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut node = self.head;

        while let Some(mut n) = node {
            node = n.next.take();
            vec.push(n.data);
        }

        vec.reverse();
        vec
    }
}
