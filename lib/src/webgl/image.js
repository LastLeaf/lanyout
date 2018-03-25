export const imageElementMap = []

export const imageLoadUrl = function(id, url, reqPtr) {
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
