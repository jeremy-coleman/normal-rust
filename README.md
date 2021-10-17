note to j, check a diff impl
https://github.com/xinaesthete/shp-contour-wasm/blob/main/src/math.rs

<h3><code>Wasm written Computation of Normals</code></h3>

## About
This was a test of replacing some functionality in BabylonJS with a function written in Rust / WebAssembly.
First attempt using js-sys was massively slower (22x).
Changing to rust types, its now ~1.5x faster.


### Originally built with `wasm-pack build --target no-modules`

using a local copy of vite plgin rsw because its input flags are not configurable to use --release, which we want to since we're benching

note: I changed codegen-units to 1 from 16 in Cargo.toml
