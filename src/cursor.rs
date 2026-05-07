pub struct Cursor<'a, T> {
    input: &'a [T],
    pos: usize,
}

impl<'a, T> Cursor<'a, T> {
    pub fn new(input: &'a [T]) -> Self {
        Self { input, pos: 0 }
    }

    pub fn peek(&self) -> Option<&'a T> {
        self.input.get(self.pos)
    }

    #[allow(dead_code)]
    pub fn peek_n(&self, n: usize) -> Option<&'a T> {
        self.input.get(self.pos + n)
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn peek_while<F>(&mut self, mut f: F) -> &'a [T]
    where
        F: FnMut(&T) -> bool,
    {
        let start = self.pos;
        let end = self.pos
            + self.input[self.pos..]
                .iter()
                .take_while(|item| f(item))
                .count();
        &self.input[start..end]
    }

    #[allow(dead_code)]
    pub fn peek_while_map<F, M>(&mut self, mut f: F) -> Vec<M>
    where
        F: FnMut(&T) -> Option<M>,
    {
        let start = self.pos;
        let end = self.pos
            + self.input[self.pos..]
                .iter()
                .take_while(|item| f(item).is_some())
                .count();
        self.input[start..end].iter().filter_map(f).collect()
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
