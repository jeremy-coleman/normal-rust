import React from "react"
import { MHRef } from "./DefaultMKHumanGeo.js"

// ===================================== WASM Load =====================================
import loadWasm, { ComputeNormals } from "compute-normals"

loadWasm()


// ================================= Results Arrays ====================================
const len = MHRef.positions.length
const rustResults = new Float32Array(len)
const bjsResults = new Float32Array(len)

// ===================================== Test 1 ========================================
function test1() {
  ComputeNormals(MHRef.positions, MHRef.indices, rustResults)
  BABYLON.VertexData.ComputeNormals(MHRef.positions, MHRef.indices, bjsResults)

  let nFailed = 0
  for (let i = 0; i < len; i++) {
    if (Math.abs(rustResults[i] - bjsResults[i]) > 0.0001) nFailed++
  }
  document.getElementById("test1PassedCheck").checked = nFailed === 0
  if (nFailed > 0) {
    alert(`${nFailed} values failed to be be within 4 decimals`)
  }
}

// ===================================== Test 1 ========================================
function test2() {
  const attempts = parseInt(document.getElementById("attempts").value)

  ComputeNormals(MHRef.positions, MHRef.indices, rustResults) // warm up, off the clock
  let startTime = BABYLON.Tools.Now
  for (let i = 0; i < attempts; i++) {
    ComputeNormals(MHRef.positions, MHRef.indices, rustResults)
  }
  const wasmTime = BABYLON.Tools.Now - startTime
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  BABYLON.VertexData.ComputeNormals(MHRef.positions, MHRef.indices, bjsResults) // warm up, off the clock
  startTime = BABYLON.Tools.Now
  for (let i = 0; i < attempts; i++) {
    BABYLON.VertexData.ComputeNormals(
      MHRef.positions,
      MHRef.indices,
      bjsResults
    )
  }
  const bjsTime = BABYLON.Tools.Now - startTime
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  if (wasmTime < bjsTime) {
    document.getElementById("test2SpeedUp").value = (
      bjsTime / wasmTime
    ).toFixed(1)
  } else {
    alert(`Wasm was actually slower by ${(wasmTime / bjsTime).toFixed(1)}X`)
  }
}

export function TestApp() {
  return (
    <>
      <h1>Testing of BabylonJS's ComputeNormals Written in Rust</h1>
      <h3>Using The Default MakeHuman Geometry (26,756 Faces / Triangles)</h3>
      Tests done without a render loop, or even loading a scene
      <br />
      <br />
      <hr />
      Test 1: Values within 4 decimals of Babylon's values for Normals
      <br />
      <button onClick={test1}>Perform</button>
      , Passed <input type="checkbox" id="test1PassedCheck" disabled />
      <hr />
      Test 2: Performance comparison
      <br />
      # of Attempts:
      <input type="text" className="number" id="attempts" defaultValue="1000" />,
      <button onClick={test2}>Perform</button>
      , Speed Up Factor:
      <input type="text" className="number" id="test2SpeedUp" disabled />X
    </>
  )
}
