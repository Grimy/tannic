/// main.rs

mod lexer;
mod path;

use lexer::Lexer;
use path::{Path,PathVisitor,PSExporter};
use path::Segment::*;

use std::fs::File;
use std::io::{Read,stdout};
use std::str;
use std::f32::NAN;

struct SVGParser {
	path: Path,
	out: Box<PathVisitor>,
}

impl SVGParser {
	fn text(&mut self, text: &[u8]) {
		// println!("Text: {}", str::from_utf8(text).unwrap());
	}

	fn tag(&mut self, name: &[u8]) {
		// println!("Tag: {}", str::from_utf8(name).unwrap());
		self.out.visit_all(&mut self.path);
		self.path.clear();
	}

	fn closetag(&mut self) {
		// path.clear();
	}

	fn attr(&mut self, name: &str, lexer: &mut Lexer) {
		match name {
			"d" => self.parse_d(lexer),
			_ => { lexer.next(b"'\""); },
		}
	}

	fn parse_d(&mut self, lexer: &mut Lexer) {
		let mut cmd = b'M';
		loop {
			match lexer.skip().peek() {
				b'\'' | b'"' => break,
				b'm' | b'z' | b'l' | b'h' | b'v' | b'c' | b's' | b'q' | b't' | b'a' |
				b'M' | b'Z' | b'L' | b'H' | b'V' | b'C' | b'S' | b'Q' | b'T' | b'A' => cmd = lexer.getc(),
				_ => {},
			}
			self.path.visit(match cmd & b'_' {
				b'M' => MoveTo([lexer.nextf(), lexer.nextf()]),
				b'L' => LineTo([lexer.nextf(), lexer.nextf()]),
				b'Z' => ClosePath,
				b'H' => LineTo([lexer.nextf(), NAN]),
				b'V' => LineTo([NAN, lexer.nextf()]),
				b'C' => CurveTo([lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf()]),
				b'S' => CurveTo([NAN, NAN, lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf()]),
				b'Q' => QuadTo([lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf()]),
				b'T' => QuadTo([NAN, NAN, lexer.nextf(), lexer.nextf()]),
				b'A' => ArcTo([lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf(), lexer.nextf()]),
				_ => panic!("Unknown path command: {}", cmd),
			})
		}
	}

	fn parse(&mut self, input: Box<Read>) {
		let mut lexer = Lexer::new(input);
		// Skip the xml declaration
		assert_eq!(lexer.next(b" \t\r\n\0"), b"<?xml");
		lexer.until(b">");
		loop {
			assert_eq!(lexer.getc(), b'>');
			if lexer.skip().eof() { break }
			self.text(lexer.next(b"<"));
			assert_eq!(lexer.getc(), b'<');
			if lexer.consume(b'!') {
				// Skip doctypes and comments
				lexer.until(b">");
				continue;
			}
			if lexer.consume(b'/') {
				self.closetag();
				lexer.until(b">");
				continue;
			}
			self.tag(lexer.next(b" \t\r\n\0>"));
			while lexer.skip().peek() != b'>' {
				if lexer.consume(b'/') {
					self.closetag();
					break;
				}
				let name = str::from_utf8(lexer.next(b" \t\r\n\0=")).unwrap().to_string();
				assert_eq!(lexer.skip().getc(), b'=');
				let quote = lexer.skip().getc();
				assert!(quote == b'\'' || quote == b'"');
				self.attr(&name, &mut lexer);
				assert_eq!(lexer.getc(), quote);
			}
		}
	}
}

fn main() {
	let mut parser = SVGParser {
		path: Path::new(),
		out: Box::new(PSExporter::new(Box::new(stdout())))
	};
	parser.parse(Box::new(File::open("/home/grimy/src/scar/huge.svg").unwrap()));
}
