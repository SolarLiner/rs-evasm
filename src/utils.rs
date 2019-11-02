use std::io::Error;
use std::{fs, io};

pub enum Input {
	Standard(io::Stdin),
	File(fs::File),
}

pub enum Output {
	Standard(io::Stdout),
	File(fs::File),
}

impl Input {
	pub fn stdin() -> Self {
		Input::Standard(io::stdin())
	}

	pub fn file(path: &str) -> io::Result<Self> {
		fs::OpenOptions::new()
			.read(true)
			.open(path)
			.map(Input::File)
	}

	pub fn from_arg(arg: Option<&str>) -> io::Result<Self> {
		match arg {
			None | Some("-") => Ok(Self::stdin()),
			Some(fname) => Self::file(fname),
		}
	}
}

impl io::Read for Input {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Input::Standard(ref mut s) => s.read(buf),
			Input::File(ref mut f) => f.read(buf),
		}
	}
}

impl Output {
	pub fn stdout() -> Self {
		Output::Standard(io::stdout())
	}

	pub fn file(path: &str) -> io::Result<Self> {
		fs::OpenOptions::new()
			.write(true)
			.open(path)
			.map(Output::File)
	}

	pub fn from_arg(arg: Option<&str>) -> io::Result<Self> {
		match arg {
			None | Some("-") => Ok(Self::stdout()),
			Some(fname) => Self::file(fname),
		}
	}
}

impl io::Write for Output {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		match self {
			Output::Standard(ref mut s) => s.write(buf),
			Output::File(ref mut f) => f.write(buf),
		}
	}

	fn flush(&mut self) -> Result<(), Error> {
		match self {
			Output::Standard(ref mut s) => s.flush(),
			Output::File(ref mut f) => f.flush(),
		}
	}
}
