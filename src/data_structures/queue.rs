use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type Link<'t, T> = Option<Rc<RefCell<Node<'t, T>>>>;

struct Node<'t, T>
where
    T: Debug,
{
    value: Option<&'t T>,
    next: Link<'t, T>,
}

impl<'t, T> Drop for Node<'t, T>
where
    T: Debug,
{
    fn drop(&mut self) {
        println!("{:?} is dropped", self.value);

        if let Some(next) = &self.next {
            println!("{:?}", next.borrow_mut().value);
        }
    }
}

/// # Description
///
/// This Queue uses linked list to handle queue. The reason why this is not a vector is that a linked list has constant O(1) complexity for both adding and taking operations.
/// Whereas vector will have O(1) for pushing and O(n) for popping from left.
/// So runtime cost for queue with a vector will increase with adding more items to the queue, whereas it's going to be constant for linked list.
pub struct Queue<'t, T>
where
    T: Debug,
{
    head: Link<'t, T>,
    tail: Link<'t, T>,
}

impl<'t, T> Queue<'t, T>
where
    T: Debug,
{
    #[must_use]
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }

    #[must_use]
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
            let value = first.borrow_mut().value;
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

impl<'t, T> Default for Queue<'t, T>
where
    T: Debug,
{
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

        println!("before 1");
        assert_eq!(Some(&1), queue.take());
        println!("after 1");

        println!("before 15");
        assert_eq!(Some(&15), queue.take());
        println!("before 15");

        println!("before 20");
        assert_eq!(Some(&20), queue.take());
        println!("before 20");

        println!("before 43");
        assert_eq!(Some(&43), queue.take());
        println!("before 43");

        assert_eq!(None, queue.take());
    }
}
