import { canvases } from './canvas_store'

import imgVs from './glsl/img.v.glsl'
import imgFs from './glsl/img.f.glsl'

const initCanvas = function(canvas, gl) {
  gl.enable(gl.BLEND)
  gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
  gl.viewport(0, 0, canvas.width, canvas.height)
  gl.clearColor(0.0, 0.0, 0.0, 1.0)
  gl.clearDepth(1)
  gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT)
}

export const bindCanvas = function(canvasIndex) {
  const elem = document.querySelector('canvas[lanyout="' + canvasIndex + '"]')
  const gl = elem.getContext('webgl') || elem.getContext('experimental-webgl')
  canvases[canvasIndex] = {
    elem,
    gl,
  }
  initCanvas(elem, gl)

  console.info(imgVs)
  console.info(imgFs)
}
