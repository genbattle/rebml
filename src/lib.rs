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

//extern crate time;
//use std::io::Memreader;
//use time::Timespec;

mod rebml {
    use time::Timespec;
    enum Value {
		Uint(u64),
		Int(i64),
		Str(String),
		DateTimespec),
		Bin(&[u8]),
        SubElement,
		Nil, // Empty element
	}

    mod incremental {
        use std::io::{IoResult,IoError};
        pub struct Parser {
            input: &Reader,
            id: u32,
            size: u64
        }

        impl Parser {
            /*
            Create new EBML Parser and read in the first tag, if any. Expects the stream to start with the top-level
            EBML tag [1A][45][DF][A3]. Consumes the input stream.
            */
            pub fn new(input: Reader) -> IoResult<Parser> {
                let mut parser: Parser;
                parser.input = input;
                match parser.read_id() {
                    Ok(val) => parser.id = val,
                    Err(err) => return err,
                }
                match parser.read_size() {
                    Ok(val) => parser.size = val,
                    Err(err) => return err
                }
            }

            /*
            Get next the next node after this one in the given input stream.
            */
            pub fn next(&self) -> Option<IoError> {
            }

            /*
            Get the subnode below this one, if any exists (returns IoError on failure)
            */
            pub fn down(&self) -> Option<IoError> {
            }

            /*
            Get the current depth in the Element tree
            */
            pub fn level(&self) -> u32 {}

            /*
            Get the value of this node
            */
            pub fn value(input: &Reader) -> IoResult<Value> {
            }

            /*
            Helper function to read an id from the stream
            */
            fn read_id(&self) -> IoResult<u32> {
                // read first byte
                let u8 = match self.input.read_byte() {
                    Ok(val) => val,
                    Err(err) => return err, // TODO: will this actually return from fn? is there a better way to format this block?
                }
                // TODO: Count zeroes to get number of trailing bytes
                // TODO: Read trailing bytes
            }

            /*
            Helper function to read size from the stream
            */
            fn read_size(&self) -> IoResult<u64> {
            }
        }
    }
}
