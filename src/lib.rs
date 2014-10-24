/*
 * A basic EBML parser for Rust. Gives methods for both approaches; full parsing
 * of an EBML file in-memory or incremental sequential parsing of nodes for
 * environments that are more memory-constrained.
 * 
 * The full parser allows you to get all sub-elements of a node in one call, 
 * because each node has a reference to the data it contains.
 * 
 * The PartialElement only contains a reference to the data source, not the data
 * itself. So if the data source is a file the full parser will load the whole
 * file to memory, while the partial parser will only load at most one leaf
 * node's value plus its id and size.
 * 
 * The partial parser stores the offset of each node in the input stream, which
 * is then used to jump to the node's data in the stream. This may cause the
 * partial parser to do more random seeking.
 */

use std::io::Memreader;
use std::time::Timespec;

pub mod rebml {
	// Error struct to give information when something goes wrong
	struct ParseError {
		error: String,
	}
	
	// Element for a full reader
	struct Element {
		id: u32,
		size: u64,
		data: &[u8]
	}
	
	// Element for an incremental reader
	struct PartialElement {
		id: u32,
		size: u64
	}
	
	enum Value {
		SubElement(Element),
		Uint(u64),
		Int(i64),
		Str(String),
		Date(Timespec),
		Bin(&[u8]),
		Err(EbmlError)
	}
	
	fn full_parse(input: Reader) -> Result<Element> {
		let mut ele: Element;
		// Attempt to read root element
		match (input.read_be_u64()) {
			Ok(val) => ele.id = val,
			Err(err) => println!("Failed to get root element"),
		}
	}
	
	fn incremental_parse(input: Reader) -> Result<PartialElement> {
		let mut ele: PartialElement;
	}
	
	impl Element {
		// TODO: method to return vec of sub-elements
		// TODO: get properties
	}
	
	impl PartialElement {
		// TODO: method to read sub/adjacent PartialElement
		// TODO: get properties
	}
}
