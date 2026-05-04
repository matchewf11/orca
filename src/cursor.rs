pub struct Cursor<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    pub fn take_while_slice<F>(&mut self, f: F) -> &'a str
    where
        F: FnMut(&char) -> bool,
    {
        let res = self
            .input
            .chars()
            .skip(self.pos)
            .take_while(f)
            .collect::<String>();
        let prev_pos = self.pos;
        self.pos += res.len();
        self.input.get(prev_pos..self.pos).unwrap()
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.pos);
        self.pos += 1;
        c
    }
}
