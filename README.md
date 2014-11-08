# REBML #
## A Rust EBML Library ##

This library aims to provide an interface to read and write files in the EBML format.

Currently the goal is to provide interfaces to both parse an entire file into memory or incrementally parse through a file. Full parsing may make it easier to handle the data in some circumstances, while incremental parsing will be less resource intensive. The intention is to let developers pick the best one for their use case, rather than having to compromise.

This library is currently a work-in-progress built against rust-nightly, so expect some breakage from time to time. Feel free to log an issue or pull request against this project for any such breakage.
