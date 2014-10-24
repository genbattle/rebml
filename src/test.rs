#[test]
// Test parsing of an EBML stream with only a header node and a size of zero.
fn test_identify_ebml() {
	let data = MemReader::new(vec!(0x1A, 0x45, 0xDF, 0xA3, 0x80));
}
