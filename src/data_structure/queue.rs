use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    // Creates a new empty Queue
    pub fn new() -> Queue<T> {
        Queue {
            elements: LinkedList::new(),
        }
    }

    // Adds and element to the back of the queue
    // 如果这里不使用 引用，在向队列中 加入一个元素后，就不能再使用这个队列了，因为所有权已经传递给这个函数了
    // pub fn enqueue(mut self, value: T) {
    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value);
    }

    // Removes and returns the front element from the queue, or None if empty
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    // Returns a reference to the front element of the queue, or None if empty
    // 为什么这里返回引用 &T，因为如果返回 T，会将所有权转移走，这里只是读取
    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    // Returns a reference to the back element of the queue, or None if empty
    pub fn peek_back(&self) -> Option<&T> {
        self.elements.back()
    }

    // Returns the number of the elements in the queue
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    // Checks if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Clears all elements from the queue
    pub fn drain(&mut self) {
        self.elements.clear();
    }
}

// Implementing the Default trait for Queue
impl<T> Default for Queue<T> {
    fn default() -> Self {
        Queue::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_functionality() {
        let mut queue = Queue::<usize>::default();

        assert!(queue.is_empty());

        queue.enqueue(8);
        queue.enqueue(16);
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.peek_front(), Some(&8));
        assert_eq!(queue.peek_back(), Some(&16));

        assert_eq!(queue.dequeue(), Some(8));
        assert_eq!(queue.len(), 1);

        assert_eq!(queue.peek_front(), Some(&16));
        assert_eq!(queue.peek_back(), Some(&16));

        queue.drain();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_work() {
        let mut queue = Queue::<usize>::new();

        queue.enqueue(8);
    }
}
