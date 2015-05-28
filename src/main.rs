use std::fs::File;
use std::io::Read;
use std::str;

const BUF_SIZE: usize = 4096;

struct Tokenizer<T: Read> {
	stream: T,
	buf: [u8; BUF_SIZE],
	mark: usize,
	pos: usize,
	limit: usize,
}

impl<T: Read> Tokenizer<T> {
	fn new(stream: T) -> Tokenizer<T> {
		Tokenizer {stream: stream, buf: [0u8; BUF_SIZE], mark: 0, pos: 0, limit: 0}
	}

	fn eof(&mut self) -> bool {
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

	fn getc(&mut self) -> u8 {
		// println!("{} <= {} <= {}", self.mark, self.pos, self.limit);
		self.pos += 1;
		self.buf[self.pos - 1]
	}

	fn peek(&self) -> u8 {
		self.buf[self.pos]
	}

	fn consume(&mut self, byte: u8) -> bool {
		if self.peek() != byte {
			return false;
		}
		self.pos += 1;
		true
	}

	fn mark(&mut self) -> &mut Tokenizer<T> {
		self.mark = self.pos;
		self
	}

	fn skip(&mut self) -> &mut Tokenizer<T> {
		while !self.eof() && b" \t\r\n\0".contains(&self.peek()) {
			self.pos += 1;
		}
		self
	}

	fn until(&mut self, delim: &[u8]) -> &mut Tokenizer<T> {
		while !self.eof() && !delim.contains(&self.peek()) {
			self.pos += 1;
		}
		self
	}

	fn token(&self) -> &[u8] {
		&self.buf[self.mark..self.pos]
	}

	fn next(&mut self, delim: &[u8]) -> &[u8] {
		self.skip().mark().until(delim).token()
	}

	fn nextf(&mut self) -> f32 {
		self.skip().consume(b',');
		let tmp = unsafe { str::from_utf8_unchecked(self.next(b" \t\r\n\0,")) };
		println!("<{}>", tmp);
		tmp.parse::<f32>().unwrap()
	}
}

fn text(text: &[u8]) {
	println!("Text: {}", str::from_utf8(text).unwrap());
}

fn tag(name: &[u8]) {
	println!("Tag: {}", str::from_utf8(name).unwrap());
}

fn closetag() {
	println!("Closetag");
}

fn attr<T: Read>(name: &str, scanner: &mut Tokenizer<T>)  {
	println!("Attr: {}", name);
	match name {
		"d" => parse_d(scanner),
		_ => { scanner.next(b"'\""); },
	}
}

fn parse_d<T: Read>(scanner: &mut Tokenizer<T>) {
	let mut cmd = b'M';
	loop {
		match scanner.skip().peek() {
			b'\'' | b'"' => break,
			b'm' | b'z' | b'l' | b'h' | b'v' | b'c' | b's' | b'q' | b't' | b'a' |
			b'M' | b'Z' | b'L' | b'H' | b'V' | b'C' | b'S' | b'Q' | b'T' | b'A' => cmd = scanner.getc(),
			_ => {},
		}
		match cmd & b'_' {
			b'M' => println!("{} {} moveto", scanner.nextf(), scanner.nextf()),
			b'L' => println!("{} {} lineto", scanner.nextf(), scanner.nextf()),
			b'Z' => println!("closepath"),
			b'H' => println!("{} horz", scanner.nextf()),
			b'V' => println!("{} vert", scanner.nextf()),
			b'C' => println!("{} {} {} {} {} {} curveto", scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf()),
			b'S' => println!("{} {} {} {} smoothto", scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf()),
			b'Q' => println!("{} {} {} {} quadto", scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf()),
			b'T' => println!("{} {} smoothquadto", scanner.nextf(), scanner.nextf()),
			b'A' => println!("{} {} {} {} {} {} {} arcto", scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf(), scanner.nextf()),
			_ => panic!("Unknown path command: {}", cmd),
		}
	}
}

fn parse(file: &str) {
	let mut scanner = Tokenizer::new(File::open(file).unwrap());
	// Skip the xml declaration
	assert_eq!(scanner.next(b" \t\r\n\0"), b"<?xml");
	scanner.until(b">");
	loop {
		assert_eq!(scanner.getc(), b'>');
		if scanner.skip().eof() { break }
		text(scanner.next(b"<"));
		assert_eq!(scanner.getc(), b'<');
		if scanner.consume(b'!') {
			// Skip doctypes and comments
			scanner.until(b">");
			continue;
		}
		if scanner.consume(b'/') {
			closetag();
			scanner.until(b">");
			continue;
		}
		tag(scanner.next(b" \t\r\n\0"));
		loop {
			if scanner.skip().consume(b'/') {
				closetag();
			}
			if scanner.peek() == b'>' { break; }
			let name = str::from_utf8(scanner.next(b" \t\r\n\0=")).unwrap().to_string();
			assert_eq!(scanner.skip().getc(), b'=');
			let quote = scanner.skip().getc();
			assert!(quote == b'\'' || quote == b'"');
			attr(&name, &mut scanner);
			assert_eq!(scanner.getc(), quote);
		}
	}
}

fn main() {
	for i in 0..10000 {
		parse("/home/grimy/src/snippets/xml/huge.svg");
	}
}
