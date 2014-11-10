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
use std::io;

mod rebml {
    mod incremental {
        pub struct Parser {
            input: &Reader,
            id: u32,
            size: u64
        }

        impl Parser {
            // Create new EBML Parser and read in the first tag, if any. Consumes the input stream.
            fn new(&self) -> IoResult<Parser> {

            }

            /*
            Get next the next node after this one in the given input stream.
            */
            fn next(&self) -> Option<IoError> {
            }

            /*
            Get the subnode below this one, if any exists (returns IoError on failure)
            */
            fn down(&self) -> Option<IoError> {
            }

            /*
            Get the current depth in the Element tree
            */
            fn level(&self) -> u32 {}

            /*
            Get the value of this node
            */
            fn value(input: &Reader) -> IoResult<Value> {
            }
        }
    }
}
