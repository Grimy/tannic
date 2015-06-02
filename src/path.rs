/// path.rs

use std::io::Write;

#[derive(Clone)]
pub enum Segment {
	MoveTo([f32; 2]),
	LineTo([f32; 2]),
	QuadTo([f32; 4]),
	CurveTo([f32; 6]),
	ArcTo([f32; 7]),
	ClosePath,
}

pub type Path = Vec<Segment>;

pub trait PathVisitor {
	fn visit(&mut self, segment: Segment);
	fn visit_all(&mut self, path: &mut Path) {
		for segment in path {
			// TODO: use path.drain() instead
			self.visit(segment.clone());
		}
	}
}

impl PathVisitor for Path {
	fn visit(&mut self, segment: Segment) {
		self.push(segment);
	}
}

pub struct PSExporter {
	out: Box<Write>,
}

impl PSExporter {
	pub fn new(out: Box<Write>) -> PSExporter {
		PSExporter { out: out }
	}

	fn format(&mut self, slice: &[f32], text: &[u8]) {
		for elem in slice {
			self.out.write((*elem as i64).to_string().as_bytes());
			self.out.write(b" ");
		}
		self.out.write(text);
		self.out.write(b"\n");
	}
}

impl PathVisitor for PSExporter {
	fn visit(&mut self, segment: Segment) {
		match segment {
			Segment::MoveTo(p) => self.format(&p, b"moveto"),
			Segment::LineTo(p) => self.format(&p, b"lineto"),
			Segment::QuadTo(p) => self.format(&p, b"quadto"),
			Segment::CurveTo(p) => self.format(&p, b"curveto"),
			Segment::ArcTo(p) => self.format(&p, b"arcto"),
			Segment::ClosePath => self.format(&[], b"closepath"),
		};
	}
}
