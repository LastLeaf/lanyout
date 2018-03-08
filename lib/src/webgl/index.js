import { canvases } from './canvas_store'

import imgVs from './glsl/img.v.glsl'
import imgFs from './glsl/img.f.glsl'

const initCanvas = function(canvas, gl) {
  gl.enable(gl.BLEND)
  gl.enable(gl.DEPTH_TEST)
  gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
  gl.viewport(0, 0, canvas.width, canvas.height)
  gl.clearColor(1.0, 1.0, 1.0, 0.0)
  gl.clearDepth(0)
  gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT)
}

export const bindCanvas = function(canvasIndex) {
  const elem = document.querySelector('canvas[lanyout="' + canvasIndex + '"]')
  const gl = elem.getContext('webgl') || elem.getContext('experimental-webgl')
  canvases[canvasIndex] = {
    canvas: elem,
    gl,
  }
  initCanvas(elem, gl)
}

export const unbindCanvas = function(canvasIndex) {
  delete canvases[canvasIndex]
}

export const setClearColor = function(canvasIndex, r, g, b, a) {
  const {gl} = canvases[canvasIndex]
  gl.clearColor(r, g, b, a)
}

export const clear = function(canvasIndex) {
  const {gl} = canvases[canvasIndex]
  gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT)
}
