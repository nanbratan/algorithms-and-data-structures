use std::cell::RefCell;
use std::rc::Rc;

type Link<'t, T> = Option<Rc<RefCell<Node<'t, T>>>>;

struct Node<'t, T> {
    value: Option<&'t T>,
    next: Link<'t, T>,
}

/// # Description
///
/// This Queue uses linked list to handle queue. The reason why this is not a vector is that a linked list has constant O(1) complexity for both adding and taking operations.
/// Whereas vector will have O(1) for pushing and O(n) for popping from left.
/// So runtime cost for queue with a vector will increase with adding more items to the queue, whereas it's going to be constant for linked list.
pub struct Queue<'t, T> {
    head: Link<'t, T>,
    tail: Link<'t, T>,
}

impl<'t, T> Queue<'t, T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }

    pub fn from(slice: &'t [T]) -> Self {
        let mut queue = Queue::new();

        queue.append(slice);

        queue
    }

    pub fn add(&mut self, value: &'t T) {
        let item = Rc::new(RefCell::new(Node {
            value: Some(value),
            next: None,
        }));

        match &self.tail {
            Some(last) => {
                last.borrow_mut().next = Some(Rc::clone(&item));
                self.tail = Some(item);
            }
            None => {
                self.head = Some(Rc::clone(&item));
                self.tail = Some(item);
            }
        }
    }

    pub fn append(&mut self, slice: &'t [T]) {
        for value in slice {
            self.add(value);
        }
    }

    pub fn take(&mut self) -> Option<&'t T> {
        if let Some(first) = &self.head {
            let value = first.borrow_mut().value.take();
            let next = first.borrow_mut().next.take();

            if next.is_none() {
                self.tail = None;
            }

            self.head = next;

            return value;
        }

        None
    }
}

impl<'t, T> Default for Queue<'t, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn should_add_and_take_from_queue() {
        let mut queue = Queue::from(&[1, 15, 20, 43]);

        assert_eq!(Some(&1), queue.take());
        assert_eq!(Some(&15), queue.take());
        assert_eq!(Some(&20), queue.take());
        assert_eq!(Some(&43), queue.take());
        assert_eq!(None, queue.take());
    }
}
