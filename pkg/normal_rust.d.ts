/* tslint:disable */
/* eslint-disable */
/**
* @param {Float32Array} positions
* @param {Uint32Array} indices
* @param {Float32Array} normals
*/
export function ComputeNormals(positions: Float32Array, indices: Uint32Array, normals: Float32Array): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly ComputeNormals: (a: number, b: number, c: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
