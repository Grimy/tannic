/// lexer.rs

use std::io::Read;
use std::str;

const BUF_SIZE: usize = 4096;

pub struct Lexer {
	stream: Box<Read>,
	buf: [u8; BUF_SIZE],
	mark: usize,
	pos: usize,
	limit: usize,
}

impl Lexer {
	pub fn new(stream: Box<Read>) -> Self {
		Lexer {stream: stream, buf: [0u8; BUF_SIZE], mark: 0, pos: 0, limit: 0}
	}

	pub fn eof(&mut self) -> bool {
		if self.pos < self.limit {
			return false;
		}
		for i in self.mark..self.limit {
			self.buf[i - self.mark] = self.buf[i];
		}
		self.pos = self.limit - self.mark;
		self.mark = 0;
		self.limit = self.pos + self.stream.read(&mut self.buf[self.pos..]).unwrap();
		self.pos >= self.limit
	}

	pub fn getc(&mut self) -> u8 {
		assert!(!self.eof());
		self.pos += 1;
		self.buf[self.pos - 1]
	}

	pub fn peek(&self) -> u8 {
		self.buf[self.pos]
	}

	pub fn consume(&mut self, byte: u8) -> bool {
		if self.eof() || self.peek() != byte {
			return false;
		}
		self.pos += 1;
		true
	}

	pub fn skip(&mut self) -> &mut Self {
		while !self.eof() && b" \t\r\n\0".contains(&self.peek()) {
			self.pos += 1;
		}
		self
	}

	pub fn until(&mut self, delim: &[u8]) -> &mut Self {
		while !self.eof() && !delim.contains(&self.peek()) {
			self.pos += 1;
		}
		self
	}

	pub fn next(&mut self, delim: &[u8]) -> &[u8] {
		self.skip();
		self.mark = self.pos;
		self.until(delim);
		&self.buf[self.mark..self.pos]
	}

	pub fn nextf(&mut self) -> f32 {
		self.skip().consume(b',');
		self.mark = self.pos;
		self.consume(b'-');
		self.until(b" \t\r\n\0,-'\"mzlhvcsqtaMZLHVCSQTA");
		let tmp = unsafe { str::from_utf8_unchecked(&self.buf[self.mark..self.pos]) };
		tmp.parse::<f32>().unwrap()
	}
}

