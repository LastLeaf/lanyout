/* global mergeInto: false, LibraryManager: false, __lanyoutLib__: false */

mergeInto(LibraryManager.library, {
  init_lib: function() {
    __lanyoutLib__.initLib()
  },
  bind_canvas: function(index) {
    __lanyoutLib__.bindCanvas(index)
  }
})
