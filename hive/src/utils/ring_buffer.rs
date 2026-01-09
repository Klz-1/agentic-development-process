//! Ring buffer for storing output lines

use std::collections::VecDeque;

/// A fixed-size ring buffer
#[derive(Debug)]
pub struct RingBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    /// Create a new ring buffer with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Push an item to the buffer, removing the oldest if at capacity
    pub fn push(&mut self, item: T) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item);
    }

    /// Get the number of items in the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Get an iterator over the items
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }

    /// Get the last N items
    pub fn last_n(&self, n: usize) -> impl Iterator<Item = &T> {
        let skip = self.buffer.len().saturating_sub(n);
        self.buffer.iter().skip(skip)
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }
}

impl<T> Default for RingBuffer<T> {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl<T: Clone> Clone for RingBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            capacity: self.capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_capacity() {
        let mut buf = RingBuffer::new(3);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4);

        assert_eq!(buf.len(), 3);
        assert_eq!(buf.iter().copied().collect::<Vec<_>>(), vec![2, 3, 4]);
    }

    #[test]
    fn test_last_n() {
        let mut buf = RingBuffer::new(5);
        for i in 1..=5 {
            buf.push(i);
        }

        assert_eq!(buf.last_n(2).copied().collect::<Vec<_>>(), vec![4, 5]);
    }
}
