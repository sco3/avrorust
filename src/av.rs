
pub struct AvroReader<'a> {
	pub buf: &'a [u8],
	pub len: usize,
	pub read: usize,
}

#[allow(clippy::needless_return)]
impl AvroReader<'_> {
	fn read_byte(&mut self, c: &mut u8) -> i32 {
		if self.len - self.read < 1 {
			return -1;
		}
		*c = self.buf[self.read];
		self.read += 1;
		return 0;
	}
}

#[allow(clippy::needless_return)]
pub fn read_long(reader: &mut AvroReader, result: &mut i64) -> i32 {
	let mut i = 0u64;
	let mut c: u8 = 0;

	let mut j = 0;
	loop {
		if j > 9 {
			return -2;
		}
		reader.read_byte(&mut c);
		i |= (u64::from(c & 0x7F)) << (j * 7);
		if (c >> 7) == 0 {
			break;
		} else {
			j += 1;
		}
	}
	if i & 0x1 == 0 {
		*result = (i >> 1) as i64
	} else {
		*result = !(i >> 1) as i64
	}
	return 0;
}
