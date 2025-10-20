# UVARINT

Zero-Copy Operations:

Read from byte slices without allocation
Write to pre-allocated buffers
std::io::Read and std::io::Write trait implementations

Important Features
Performance:

Inline functions for hot paths
Benchmarks comparing to existing libraries
Optional SIMD optimizations for batch operations
#[no_std] support for embedded systems

Ergonomics:

Trait-based API (e.g., VarIntEncode, VarIntDecode)
Helper functions like encoded_len(value) to predict size
Builder pattern for configuration if needed
Good type inference support

Compatibility:

Support multiple varint formats (LEB128, Protocol Buffers, etc.)
Feature flags for different encodings
Async I/O support (tokio::AsyncRead/AsyncWrite)

Nice-to-Have Features
Advanced:

Streaming decoder for parsing continuous streams
Batch encoding/decoding APIs
serde support for serialization frameworks
Zero-allocation iterators over varint sequences
Const functions where possible (for compile-time encoding)

Developer Experience:

Comprehensive examples in docs
Comparison guide with other libraries
Migration guide if competing with existing crates
Clear performance characteristics documented

## Resources

- https://techoverflow.net/2013/01/25/efficiently-encoding-variable-length-integers-in-cc/
