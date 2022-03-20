# path-tracer
Implementation of a path tracer in Rust

# Features

- In-memory buffer of image canvas data
- Utility methods to iterate over each `(x, y)` pixel
- Buffered write of image data, reaching ≈9.5k pixels-per-milisecond (p/ms) on i7-6700K
