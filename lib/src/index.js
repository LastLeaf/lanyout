import { createBackgroundCanvas } from './webgl/canvas_store'

export * from './webgl/index'

/* global __lanyoutAsm__: false */

let animationFrameEnabled = false
let animationFrameScheduled = false

const animationFrameFn = function(timestamp) {
  if (!animationFrameEnabled) {
    animationFrameScheduled = false
    return
  }
  requestAnimationFrame(animationFrameFn)
  __lanyoutAsm__._animation_frame(timestamp)
}

export const enableAnimationFrame = function() {
  animationFrameEnabled = true
  if (!animationFrameScheduled) {
    animationFrameScheduled = true
    requestAnimationFrame(animationFrameFn)
  }
}

export const disableAnimationFrame = function() {
  animationFrameEnabled = false
}

export const initLib = function() {
  if (typeof __lanyoutAsm__ === 'undefined') throw new Error('Lanyout asm module not found. Initializing failed.')
  createBackgroundCanvas()
  console.log('Lanyout initialized.')
}
