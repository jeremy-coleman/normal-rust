  <h1><code>Wasm written Computation of Normals</code></h1>

## About
This was a test of replacing some functionality in BabylonJS with a function written in Rust / WebAssembly.  It worked, producing identical results, but massively slower (22x).  Might have a lot to do with 2 `Float32Arrays` & 1 `Uint32Array` with the call.  Used the Wee allocator, instead of the default, might try again without it.

### Built with `wasm-pack build --target no-modules`
