pub struct StackSet {
    occupied: Vec<bool>,
    stack: Vec<usize>,
}

impl StackSet {
    pub fn new(max_len: usize) -> Self {
        Self {
            occupied: vec![false; max_len],
            stack: Vec::new(),
        }
    }

    pub fn full(max_len: usize) -> Self {
        let mut stack = Vec::new();
        for i in 0..max_len {
            stack.push(i);
        }

        Self {
            occupied: vec![true; max_len],
            stack,
        }
    }

    pub fn push(&mut self, value: usize) {
        if !self.occupied[value] {
            self.occupied[value] = true;
            self.stack.push(value);
        }
    }

    pub fn pop(&mut self) -> Option<usize> {
        if let Some(value) = self.stack.pop() {
            self.occupied[value] = false;
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_behaves_like_a_stack_with_unique_elements() {
        let mut stack_set = StackSet::new(5);
        stack_set.push(2);
        stack_set.push(2);
        stack_set.push(2);
        stack_set.push(2);
        stack_set.push(0);
        stack_set.push(0);
        stack_set.push(1);
        stack_set.push(1);
        stack_set.push(4);
        stack_set.push(4);
        stack_set.push(3);
        stack_set.push(3);

        assert_eq!(stack_set.pop(), Some(3));
        assert_eq!(stack_set.pop(), Some(4));
        assert_eq!(stack_set.pop(), Some(1));
        assert_eq!(stack_set.pop(), Some(0));
        assert_eq!(stack_set.pop(), Some(2));
        assert_eq!(stack_set.pop(), None);
    }
}
