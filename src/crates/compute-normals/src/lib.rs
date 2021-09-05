
use wasm_bindgen::prelude::*;

// #[cfg(not(target_feature = "simd128"))]
// compile_error!("Simd not enabled, please check your configuration");


// Compute normals for given positions and indices
// @param positions an array of vertex positions, [...., x, y, z, ......]
// @param indices an array of indices in groups of three for each triangular facet, [...., i, j, k, ......]
// @param normals an array of vertex normals, [...., x, y, z, ......]
// https://github.com/BabylonJS/Babylon.js/blob/master/src/Meshes/mesh.vertexData.ts#L1102



#[wasm_bindgen]
pub fn ComputeNormals(positions: &[f32], indices: &[u32], normals: &mut[f32]) {

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
    let mut v1x: usize;                    // vector1 x index in the positions array
    let mut v1y: usize;                    // vector1 y index in the positions array
    let mut v1z: usize;                    // vector1 z index in the positions array
    let mut v2x: usize;                    // vector2 x index in the positions array
    let mut v2y: usize;                    // vector2 y index in the positions array
    let mut v2z: usize;                    // vector2 z index in the positions array
    let mut v3x: usize;                    // vector3 x index in the positions array
    let mut v3y: usize;                    // vector3 y index in the positions array
    let mut v3z: usize;                    // vector3 z index in the positions array

    // reset the normals
    normals.fill(0.);

    // Loop : 1 indice triplet = 1 facet
    let nbFaces = (indices.len() / 3) | 0;
    for index in 0..nbFaces {

        // get the indexes of the coordinates of each vertex of the facet
        v1x = indices[index as usize * 3] as usize * 3;
        v1y = v1x + 1;
        v1z = v1x + 2;
        v2x = indices[index as usize * 3 + 1] as usize * 3;
        v2y = v2x + 1;
        v2z = v2x + 2;
        v3x = indices[index as usize * 3 + 2]  as usize * 3;
        v3y = v3x + 1;
        v3z = v3x + 2;

        p1p2x = positions[v1x] - positions[v2x];          // compute two vectors per facet : p1p2 and p3p2
        p1p2y = positions[v1y] - positions[v2y];
        p1p2z = positions[v1z] - positions[v2z];

        p3p2x = positions[v3x] - positions[v2x];
        p3p2y = positions[v3y] - positions[v2y];
        p3p2z = positions[v3z] - positions[v2z];

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
        normals[v1x] = normals[v1x] + faceNormalx;                         // accumulate all the normals per face
        normals[v1y] = normals[v1y] + faceNormaly;
        normals[v1z] = normals[v1z] + faceNormalz;
        normals[v2x] = normals[v2x] + faceNormalx;
        normals[v2y] = normals[v2y] + faceNormaly;
        normals[v2z] = normals[v2z] + faceNormalz;
        normals[v3x] = normals[v3x] + faceNormalx;
        normals[v3y] = normals[v3y] + faceNormaly;
        normals[v3z] = normals[v3z] + faceNormalz;
    }
    // last normalization of each normal
    for index in 0..normals.len() / 3 {
        faceNormalx = normals[index * 3];
        faceNormaly = normals[index * 3 + 1];
        faceNormalz = normals[index * 3 + 2];

        length = (faceNormalx * faceNormalx + faceNormaly * faceNormaly + faceNormalz * faceNormalz).sqrt();
        length = if length == 0. { 1.0 } else { length };
        faceNormalx /= length;
        faceNormaly /= length;
        faceNormalz /= length;

        normals[index * 3] = faceNormalx;
        normals[index * 3 + 1] = faceNormaly;
        normals[index * 3 + 2] = faceNormalz;
    }
}
