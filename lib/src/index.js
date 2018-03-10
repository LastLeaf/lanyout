import { createBackgroundCanvas } from './webgl/canvas_store'

export * from './webgl/index'

/* global __lanyoutAsm__: false */

const initAnimationFrame = function() {
  requestAnimationFrame(function(timestamp) {
    initAnimationFrame()
    __lanyoutAsm__._animation_frame(timestamp)
  })
}

export const initLib = function() {
  if (typeof __lanyoutAsm__ === 'undefined') throw new Error('Lanyout asm module not found. Initializing failed.')
  createBackgroundCanvas()
  initAnimationFrame()
  console.log('Lanyout initialized.')
}
