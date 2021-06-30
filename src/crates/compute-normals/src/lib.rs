mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
// Compute normals for given positions and indices
// @param positions an array of vertex positions, [...., x, y, z, ......]
// @param indices an array of indices in groups of three for each triangular facet, [...., i, j, k, ......]
// @param normals an array of vertex normals, [...., x, y, z, ......]
// https://github.com/BabylonJS/Babylon.js/blob/master/src/Meshes/mesh.vertexData.ts#L1102
pub fn ComputeNormals(positions : js_sys::Float32Array, indices : js_sys::Uint32Array, normals : js_sys::Float32Array) {
    // temporary scalar variables
    let mut p1p2x: f32;                    // p1p2 vector x coordinate
    let mut p1p2y: f32;                    // p1p2 vector y coordinate
    let mut p1p2z: f32;                    // p1p2 vector z coordinate
    let mut p3p2x: f32;                    // p3p2 vector x coordinate
    let mut p3p2y: f32;                    // p3p2 vector y coordinate
    let mut p3p2z: f32;                    // p3p2 vector z coordinate
    let mut faceNormalx: f32;              // facet normal x coordinate
    let mut faceNormaly: f32;              // facet normal y coordinate
    let mut faceNormalz: f32;              // facet normal z coordinate
    let mut length: f32;                   // facet normal length before normalization
    let mut v1x: u32;                        // vector1 x index in the positions array
    let mut v1y: u32;                        // vector1 y index in the positions array
    let mut v1z: u32;                        // vector1 z index in the positions array
    let mut v2x: u32;                        // vector2 x index in the positions array
    let mut v2y: u32;                        // vector2 y index in the positions array
    let mut v2z: u32;                        // vector2 z index in the positions array
    let mut v3x: u32;                        // vector3 x index in the positions array
    let mut v3y: u32;                        // vector3 y index in the positions array
    let mut v3z: u32;                        // vector3 z index in the positions array

    // reset the normals
    normals.fill(0., 0, normals.length());

    // Loop : 1 indice triplet = 1 facet
    let nbFaces : u32 = (indices.length() / 3) | 0;
    for index in 0..nbFaces {

        // get the indexes of the coordinates of each vertex of the facet
        v1x = indices.get_index(index * 3) * 3;
        v1y = v1x + 1;
        v1z = v1x + 2;
        v2x = indices.get_index(index * 3 + 1) * 3;
        v2y = v2x + 1;
        v2z = v2x + 2;
        v3x = indices.get_index(index * 3 + 2) * 3;
        v3y = v3x + 1;
        v3z = v3x + 2;

        p1p2x = positions.get_index(v1x) - positions.get_index(v2x);          // compute two vectors per facet : p1p2 and p3p2
        p1p2y = positions.get_index(v1y) - positions.get_index(v2y);
        p1p2z = positions.get_index(v1z) - positions.get_index(v2z);

        p3p2x = positions.get_index(v3x) - positions.get_index(v2x);
        p3p2y = positions.get_index(v3y) - positions.get_index(v2y);
        p3p2z = positions.get_index(v3z) - positions.get_index(v2z);

        // compute the face normal with the cross product
        faceNormalx = p1p2y * p3p2z - p1p2z * p3p2y;
        faceNormaly = p1p2z * p3p2x - p1p2x * p3p2z;
        faceNormalz = p1p2x * p3p2y - p1p2y * p3p2x;
        // normalize this normal and store it in the array facetData
        length = (faceNormalx * faceNormalx + faceNormaly * faceNormaly + faceNormalz * faceNormalz).sqrt();
        length = if length == 0. { 1.0 } else { length };
        faceNormalx /= length;
        faceNormaly /= length;
        faceNormalz /= length;

        // compute the normals anyway
        normals.set_index(v1x, normals.get_index(v1x) + faceNormalx);                         // accumulate all the normals per face
        normals.set_index(v1y, normals.get_index(v1y) + faceNormaly);
        normals.set_index(v1z, normals.get_index(v1z) + faceNormalz);
        normals.set_index(v2x, normals.get_index(v2x) + faceNormalx);
        normals.set_index(v2y, normals.get_index(v2y) + faceNormaly);
        normals.set_index(v2z, normals.get_index(v2z) + faceNormalz);
        normals.set_index(v3x, normals.get_index(v3x) + faceNormalx);
        normals.set_index(v3y, normals.get_index(v3y) + faceNormaly);
        normals.set_index(v3z, normals.get_index(v3z) + faceNormalz);
    }
    // last normalization of each normal
    for index in 0..normals.length() / 3 {
        faceNormalx = normals.get_index(index * 3);
        faceNormaly = normals.get_index(index * 3 + 1);
        faceNormalz = normals.get_index(index * 3 + 2);

        length = (faceNormalx * faceNormalx + faceNormaly * faceNormaly + faceNormalz * faceNormalz).sqrt();
        length = if length == 0. { 1.0 } else { length };
        faceNormalx /= length;
        faceNormaly /= length;
        faceNormalz /= length;

        normals.set_index(index * 3, faceNormalx);
        normals.set_index(index * 3 + 1, faceNormaly);
        normals.set_index(index * 3 + 2, faceNormalz);
    }
}
