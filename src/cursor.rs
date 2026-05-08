pub struct Cursor<'a, T> {
    input: &'a [T],
    pos: usize,
}

impl<'a, T: PartialEq> Cursor<'a, T> {
    pub fn is_prefix(&self, prefix: &[T]) -> bool {
        self.input.get(self.pos..self.pos + prefix.len()) == Some(prefix)
    }

    pub fn expect_or<E>(&mut self, v: &T, e: E) -> Result<(), E> {
        match self.next() {
            Some(e) if e == v => Ok(()),
            _ => Err(e),
        }
    }
}

impl<'a, T> Cursor<'a, T> {
    pub fn new(input: &'a [T]) -> Self {
        Self { input, pos: 0 }
    }

    pub fn peek(&self) -> Option<&'a T> {
        self.input.get(self.pos)
    }

    pub fn peek_n(&self, n: usize) -> Option<&'a T> {
        self.input.get(self.pos + n)
    }

    pub fn eat_n(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn eat_while<F>(&mut self, mut f: F) -> &'a [T]
    where
        F: FnMut(&T) -> bool,
    {
        let start = self.pos;
        self.pos += self.input[self.pos..]
            .iter()
            .take_while(|item| f(item))
            .count();
        &self.input[start..self.pos]
    }
}

impl<'a, T> Iterator for Cursor<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.peek()?;
        self.pos += 1;
        Some(c)
    }
}
