import { canvases } from './canvas_store'
import { textImageDataMap } from './text'
import { imageElementMap } from './image'

import imgVs from './glsl/img.v.glsl'
import imgFs from './glsl/img.f.glsl'

const GL_DRAW_RECT_MAX = 4096

export const createTexManager = function(canvas, gl) {
  const texSize = gl.MAX_TEXTURE_SIZE
  const texCount = gl.getParameter(gl.MAX_TEXTURE_IMAGE_UNITS)

  // init img draw program
  const imgShaderProgram = gl.createProgram()
  let shaderLog = ''
  let shader = gl.createShader(gl.VERTEX_SHADER)
  gl.shaderSource(shader, imgVs)
  gl.compileShader(shader)
  if(!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    shaderLog = gl.getShaderInfoLog(shader)
    gl.deleteShader(shader)
    throw new Error('Failed initializing WebGL vertex shader: ' + shaderLog)
  }
  gl.attachShader(imgShaderProgram, shader)
  shader = gl.createShader(gl.FRAGMENT_SHADER)
  gl.shaderSource(shader, imgFs)
  gl.compileShader(shader)
  if(!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    shaderLog = gl.getShaderInfoLog(shader)
    gl.deleteShader(shader)
    throw new Error('Failed initializing WebGL fragment shader: ' + shaderLog)
  }
  gl.attachShader(imgShaderProgram, shader)
  gl.linkProgram(imgShaderProgram)
  if(!gl.getProgramParameter(imgShaderProgram, gl.LINK_STATUS)) {
    throw new Error('Failed initializing WebGL shader program.')
  }
  gl.useProgram(imgShaderProgram)

  // the texture position buffer
  const texPosGLBuf = gl.createBuffer()
  const texPosBuf = new Float32Array(8 * GL_DRAW_RECT_MAX)
  gl.bindBuffer(gl.ARRAY_BUFFER, texPosGLBuf)
  gl.bufferData(gl.ARRAY_BUFFER, texPosBuf, gl.DYNAMIC_DRAW)
  const aTexPos = gl.getAttribLocation(imgShaderProgram, 'aTexPos')
  gl.enableVertexAttribArray(aTexPos)
  gl.vertexAttribPointer(aTexPos, 2, gl.FLOAT, false, 0, 0)

  // the draw position buffer
  const drawPosGLBuf = gl.createBuffer()
  const drawPosBuf = new Float32Array(8 * GL_DRAW_RECT_MAX)
  gl.bindBuffer(gl.ARRAY_BUFFER, drawPosGLBuf)
  gl.bufferData(gl.ARRAY_BUFFER, drawPosBuf, gl.DYNAMIC_DRAW)
  const aDrawPos = gl.getAttribLocation(imgShaderProgram, 'aDrawPos')
  gl.enableVertexAttribArray(aDrawPos)
  gl.vertexAttribPointer(aDrawPos, 2, gl.FLOAT, false, 0, 0)

  // the texture position buffer
  const texIndexGLBuf = gl.createBuffer()
  const texIndexBuf = new Float32Array(GL_DRAW_RECT_MAX)
  gl.bindBuffer(gl.ARRAY_BUFFER, texIndexGLBuf)
  gl.bufferData(gl.ARRAY_BUFFER, texIndexBuf, gl.DYNAMIC_DRAW)
  const aTexIndex = gl.getAttribLocation(imgShaderProgram, 'aTexIndex')
  gl.enableVertexAttribArray(aTexIndex)
  gl.vertexAttribPointer(aTexIndex, 2, gl.FLOAT, false, 0, 0)

  return {
    width: 1,
    height: 1,
    texSize,
    texCount,
    imgShaderProgram,
    texPosGLBuf,
    texPosBuf,
    drawPosGLBuf,
    drawPosBuf,
    texIndexGLBuf,
    texIndexBuf,
    textures,
  }
}

export const setTexDrawSize = function(ctx, texManager, w, h) {
  texManager.width = w
  texManager.height = h
  const uAreaSize = ctx.getUniformLocation(texManager.imgShaderProgram, 'uAreaSize')
  ctx.uniform2f(uAreaSize, w, h)
}

const setTex = function(canvasIndex, drawIndex, img, texIndex, texX, texY, texW, texH){
  const {ctx, texManager} = canvases[canvasIndex]
  ctx.bindTexture(ctx.TEXTURE_2D, texIndex)
  ctx.texSubImage2D(ctx.TEXTURE_2D, 0, texX, texY, texW, texH, ctx.RGBA, ctx.UNSIGNED_BYTE, img)
  ctx.texParameteri(ctx.TEXTURE_2D, ctx.TEXTURE_MIN_FILTER, ctx.LINEAR)
  ctx.texParameteri(ctx.TEXTURE_2D, ctx.TEXTURE_WRAP_S, ctx.CLAMP_TO_EDGE)
  ctx.texParameteri(ctx.TEXTURE_2D, ctx.TEXTURE_WRAP_T, ctx.CLAMP_TO_EDGE)
  ctx.bindTexture(ctx.TEXTURE_2D, null)
}

// TODO impl
export const texDraw = function(canvasIndex) {
  const {ctx, texManager} = canvases[canvasIndex]
  const {
    texPosBuf,
    drawPosBuf,
    texIndexBuf,
  } = texManager
  const drawIndex8 = drawIndex * 8
  texPosBuf[drawIndex8 + 0] = texX / texW
  texPosBuf[drawIndex8 + 1] = texY / texH
  texPosBuf[drawIndex8 + 2] = texX / texW
  texPosBuf[drawIndex8 + 3] = (texY + texDy) / texH
  texPosBuf[drawIndex8 + 4] = (texX + texDx) / texW
  texPosBuf[drawIndex8 + 5] = (texY + texDy) / texH
  texPosBuf[drawIndex8 + 6] = (texX + texDx) / texW
  texPosBuf[drawIndex8 + 7] = texY
  drawPosBuf[drawIndex8 + 0] = x
  drawPosBuf[drawIndex8 + 1] = y
  drawPosBuf[drawIndex8 + 2] = x
  drawPosBuf[drawIndex8 + 3] = y + dY
  drawPosBuf[drawIndex8 + 4] = x + dX
  drawPosBuf[drawIndex8 + 5] = y + dY
  drawPosBuf[drawIndex8 + 6] = x + dX
  drawPosBuf[drawIndex8 + 7] = y
  texIndexBuf[drawIndex] = texIndex
}

// TODO impl
export const texDrawEnd = function(canvasIndex) {
  const {ctx, texManager} = canvases[canvasIndex]
  const {texPosGLBuf,
    texCount,
    texPosBuf,
    drawPosGLBuf,
    drawPosBuf,
    texIndexGLBuf,
    texIndexBuf,
  } = texManager
  ctx.bindBuffer(ctx.ARRAY_BUFFER, texPosGLBuf)
  ctx.bufferData(ctx.ARRAY_BUFFER, texPosBuf, ctx.DYNAMIC_DRAW)
  ctx.bindBuffer(ctx.ARRAY_BUFFER, drawPosGLBuf)
  ctx.bufferData(ctx.ARRAY_BUFFER, drawPosBuf, ctx.DYNAMIC_DRAW)
  ctx.bindBuffer(ctx.ARRAY_BUFFER, texIndexGLBuf)
  ctx.bufferData(ctx.ARRAY_BUFFER, texIndexBuf, ctx.DYNAMIC_DRAW)
  for (let i = 0; i < texCount; i++) {
    ctx.activeTexture(ctx[TEXTURE + i])
    ctx.bindTexture(ctx.TEXTURE_2D, textures[i])
  }
  ctx.drawArrays(gl.TRIANGLE_FAN, 0, 4)
}

export const texSetText = function(canvasIndex, drawIndex, id, texIndex, texX, texY, texWidth, texHeight) {
  const imageData = textImageDataMap[id]
  setTex(canvasIndex, drawIndex, imageData, texIndex, texX, texY, texWidth, texHeight, x, y, w, h)
}

export const texSetImage = function(canvasIndex, drawIndex, id, texIndex, texX, texY, texWidth, texHeight) {
  const imgElem = imageElementMap[id]
  setTex(canvasIndex, drawIndex, imgElem, texIndex, texX, texY, texWidth, texHeight, x, y, w, h)
}
