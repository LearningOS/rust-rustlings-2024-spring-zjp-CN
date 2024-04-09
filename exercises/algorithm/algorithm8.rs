/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
enum Stack {
    Q1,
    Q2,
}

pub struct myStack<T> {
    //TODO
    stack: Stack,
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            stack: Stack::Q1,
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.stack_queue(|stack, queue| {
            if let Ok(last) = stack.dequeue() {
                queue.enqueue(last);
            }
            stack.enqueue(elem);
        });
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        let last = self.stack_queue(|stack, queue| {
            let last = stack.dequeue().map_err(|_| "Stack is empty")?;
            for _ in 0..queue.size().saturating_sub(1) {
                // move the whole queue except the last element
                stack.enqueue(queue.dequeue().map_err(|_| "Stack is empty")?);
            }
            Ok(last)
        })?;
        // flip the stack status
        self.flip_stack();
        Ok(last)
    }
    pub fn is_empty(&self) -> bool {
        //TODO
        self.q1.is_empty() && self.q2.is_empty()
    }
    fn stack_queue<U>(&mut self, f: impl FnOnce(&mut Queue<T>, &mut Queue<T>) -> U) -> U {
        let [stack, queue] = match self.stack {
            Stack::Q1 => [&mut self.q1, &mut self.q2],
            Stack::Q2 => [&mut self.q2, &mut self.q1],
        };
        f(stack, queue)
    }
    fn flip_stack(&mut self) {
        self.stack = match self.stack {
            Stack::Q1 => Stack::Q2,
            Stack::Q2 => Stack::Q1,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}

