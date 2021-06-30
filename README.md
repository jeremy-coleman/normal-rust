<h3><code>Wasm written Computation of Normals</code></h3>

## About
This was a test of replacing some functionality in BabylonJS with a function written in Rust / WebAssembly.  It worked, producing identical results, but massively slower (22x).  Might have a lot to do with 2 `Float32Arrays` & 1 `Uint32Array` with the call.  Used the Wee allocator, instead of the default, might try again without it.


### Originally built with `wasm-pack build --target no-modules`

using a local copy of vite plgin rsw because its output not configurable, and want to use --release flag since we're benching

