import { bgCanvas } from './canvas_store'

const fontFamilyMap = []
export const textImageDataMap = []

export const textBindFontFamily = function(id, fontFamily) {
  fontFamilyMap[id] = __lanyoutAsm__.UTF8ToString(fontFamily)
}

export const textUnbindFontFamily = function(id) {
  fontFamilyMap[id] = ''
}

export const textSetFont = function(fontSize, fontFamilyId) {
  const ctx = bgCanvas.ctx
  ctx.font = fontSize + 'px ' + fontFamilyMap[fontFamilyId]
}

export const textGetWidth = function(text) {
  const ctx = bgCanvas.ctx
  return ctx.measureText(text).width
}

export const textDrawInCanvas = function(text, width, fontSize, fontFamilyId) {
  const ctx = bgCanvas.ctx
  const height = fontSize
  ctx.clearRect(0, 0, width, height)
  ctx.textBaseline = 'top'
  ctx.fillStyle = 'black'
  ctx.fillText(text, 0, 0)
  var imgData = this.elem = ctx.getImageData(0, 0, width, height)
  this.width = imgData.width
  this.height = imgData.height
}

export const textSaveImageData = function(id, x, y, width, height) {
  const ctx = bgCanvas.ctx
  textImageDataMap[id] = ctx.getImageData(0, 0, width, height)
}

export const textRemoveImageData = function(id) {
  textImageDataMap[id] = null
}
