#![cfg_attr(test, feature(inclusive_range_syntax))]
#![no_std]


#[derive(Debug)]
pub struct Stack<'a, T: 'a> where T: Clone {
    data: &'a mut [T],
    len: usize
}

impl<'a, T: 'a> Stack<'a, T> {
    /// takes storage and creates a Stack data structure
    /// init throws away/forgets about values contained in data
    pub fn init(data: &'a mut [T]) -> Stack<'a, T> {
        Stack {
            data,
            len: 0usize
        }
    }

    /// Creates Stack data structure with length len.
    /// new_from() keeps the values contained in data
    pub fn make(data: &'a mut [T], len: usize) -> Stack<'a, T> {
        Stack { data, len }
    }

    pub fn capacity(&self) -> usize { self.data.len() }

    pub fn len(&self) -> usize { self.len }

    /// Shortens the stack, to keep the 0..len elements.
    pub fn truncate(&mut self, len: usize) {
        if len < self.len() {
            self.len = len;
        }
    }

    /// Consumes self, returns the length of the Stack, not the length of the backing storage
    pub fn into_slice(self) -> &mut [T] {
        &mut self.data[0..self.len]
    }

    ///
    pub fn as_slice(&self) -> &[T] {
        &self.data[0..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[0..self.len]
    }

    /// checks if stack is empty
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// check if stack is full
    pub fn is_full(&self) -> bool { self.len == self.capacity() }

    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if !self.is_full() {
            self.data[len] = value;
            self.len += 1;
            Ok(self.len)
        } else {
            Err
        }
    }
}

impl<'a, T: Clone + 'a> Stack<'a, T> {
    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.len -= 1;
            let i = self.len;
            Some(self.data[i].clone())
        } else {
            None
        }
    }
}