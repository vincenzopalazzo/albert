//! Albert stream is an abstraction of token
//! stream where it is possible implement
//! stream of tokens with the basic method
//! for a simple compiler front-end.

use std::fmt;

#[derive(Debug)]
/// Basic Stream that allow to have out of the box a stream of
/// tokens with access to the current position and the tot
/// size of the stream.
pub struct BasicStream<T> {
    /// current position in the stream
    pub pos: usize,
    /// current stream
    pub stream: Vec<T>,
    /// tot size of the stream
    pub size: usize,
}

impl<T> Stream<T> for BasicStream<T>
where
    T: fmt::Display + Clone,
{
    fn new(tokens: &Vec<T>) -> Self {
        BasicStream::new_with_pos(tokens, 0)
    }

    fn new_with_pos(stream: &Vec<T>, pos: usize) -> Self {
        BasicStream {
            pos,
            stream: stream.to_vec(),
            size: stream.len(),
        }
    }

    /// advance the position and return the previous element
    /// in position - 1
    fn advance<'c>(&'c mut self) -> &'c T {
        self.next();
        if self.is_end() {
            return &self.stream.last().unwrap();
        }
        self.prev()
    }

    fn lookup<'c>(&'c self, step: usize) -> Option<&'c T> {
        if self.size > self.pos + step {
            return None;
        }
        Some(&self.stream[self.pos + step])
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    fn prev<'c>(&'c self) -> &'c T {
        assert!(self.pos < self.size, "prev: out of bound");
        &self.stream[self.pos - 1]
    }

    /// return he token at the current position
    fn peek<'c>(&'c self) -> &'c T {
        assert!(self.pos < self.size);
        &self.stream[self.pos]
    }

    fn match_tok(&self, tok: &str) -> bool {
        self.peek().to_string() == tok
    }

    /// check if it is reach the end of the stream
    fn is_end(&self) -> bool {
        if self.size == 0 {
            return true;
        }
        self.pos > self.size - 1
    }
}

pub trait Stream<T> {
    fn new(tokens: &Vec<T>) -> Self;

    fn new_with_pos(stream: &Vec<T>, pos: usize) -> Self;

    /// advance the position and return the previous element
    /// in position - 1
    fn advance<'c>(&'c mut self) -> &'c T;

    fn lookup<'c>(&'c self, step: usize) -> Option<&'c T>;

    fn next(&mut self);

    fn prev<'c>(&'c self) -> &'c T;

    /// return he token at the current position
    fn peek<'c>(&'c self) -> &'c T;

    fn match_tok(&self, tok: &str) -> bool;

    /// check if it is reach the end of the stream
    fn is_end(&self) -> bool;
}
