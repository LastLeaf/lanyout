import { canvases } from './canvas_store'

export const imageLoadUrl = function(canvasIndex, id, url, reqPtr) {
  const {imageElementMap} = canvases[canvasIndex]
  const imgElem = document.createElement('img')
  imgElem.onload = function() {
    imageElementMap[id] = imgElem
    __lanyoutAsm__._callback(reqPtr, 0)
  }
  imgElem.onerror = imgElem.onabort = function(){
    __lanyoutAsm__._callback(reqPtr, -1)
  }
  imgElem.src = __lanyoutAsm__.UTF8ToString(url)
}
