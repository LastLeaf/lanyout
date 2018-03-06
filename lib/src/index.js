import { createBackgroundCanvas } from './webgl/canvas_store'

export * from './webgl/index'

export const initLib = function() {
  createBackgroundCanvas()
  console.log('Lanyout initialized.')
}
