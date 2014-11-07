/*
 * A basic EBML parser for Rust. Gives methods for both approaches; full parsing
 * of an EBML file in-memory or incremental sequential parsing of nodes for
 * environments that are more memory-constrained.
 * 
 * The full parser allows you to get all sub-elements of a node in one call, 
 * because each node has a reference to the data it contains. All of the data is
 * loaded into memory when the root node is parsed.
 * 
 * The PartialElement only reads the data it needs to get the header of any 
 * single element. So if the data source is a file the full parser will load the
 * whole file to memory, while the partial parser will only load at most one leaf
 * node's value plus its id and size.
 * 
 * The partial parser stores the offset of each node in the input stream, which
 * is then used to jump to the node's data in the stream. This may cause the
 * partial parser to do more random seeking.
 */

extern crate time;
//use std::io::Memreader;
//use time::Timespec;
use std::io::IoResult;

pub mod rebml {
	// Element for an incremental reader
	struct ElementHeader {
		id: u32,
		size: u64
	}
	
	/*// Element for a full reader
	struct Element {
		header: ElementHeader,
		data: &[u8]
	}*/
	
	enum Value {
		SubElement(ElementHeader),
		Uint(u64),
		Int(i64),
		Str(String),
		Date(time::Timespec),
		Bin(&[u8]),
		Nil, // Empty element
	}
	
	enum Type {
		
	}
	
	/*fn full_parse(input: &Reader) -> IoResult<Element> {
		let mut ele: Element;
		// Attempt to read root element
		match (input.read_be_u64()) {
			Ok(val) => ele.header.id = val,
			Error(err) => return Error(err),
		}
	}*/
	
	fn partial_parse(input: &Reader) -> IoResult<ElementHeader> {
	}
	
	/*impl Element {
		fn sub_elements() -> IoResult<Vec<Element>> {
		}
		
		fn value() -> IoResult<Option<Value>> {
			
		}
		// TODO: get properties
	}*/
	
	impl ElementHeader {
		/*
		 * Get next the next node after this one in the given input stream.
		 */
		fn next(input: &Reader) -> IoResult<ElementHeader> {
		}
		
		/*
		 * Get the value of this node
		 */
		fn value(input: &Reader) -> IoResult<Value> {
		}
		// TODO: method to read sub/adjacent PartialElement
		// TODO: get properties
	}
}
