mod av;

fn main() {
	// -333444555
	let bytes: [u8; 5] = [0x95, 0xd7, 0xff, 0xbd, 0x02];
	let mut reader = av::AvroReader {
		buf: &bytes,
		len: bytes.len(),
		read: 0,
	};
	let mut i: i64 = 0;

	println!("Zigzag: {} {}", av::read_long(&mut reader, &mut i), i)
}
