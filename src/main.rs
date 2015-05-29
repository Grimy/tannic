/// main.rs

mod lexer;
use lexer::Lexer;
use std::fs::File;
use std::str;

fn text(text: &[u8]) {
	println!("Text: {}", str::from_utf8(text).unwrap());
}

fn tag(name: &[u8]) {
	println!("Tag: {}", str::from_utf8(name).unwrap());
}

fn closetag() {
	println!("Closetag");
}

fn attr(name: &str, scanner: &mut Lexer) {
	println!("Attr: {}", name);
	match name {
		"d" => parse_d(scanner),
		_ => { scanner.next(b"'\""); },
	}
}

fn parse_d(scanner: &mut Lexer) {
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
	let mut scanner = Lexer::new(Box::new(File::open(file).unwrap()));
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
