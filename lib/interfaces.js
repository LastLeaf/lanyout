/* global mergeInto: false, LibraryManager: false */

mergeInto(LibraryManager.library, {
  init_lib: function() {
    window.__lanyoutLib__.initLib()
  },
  bind_canvas: function(index) {
    window.__lanyoutLib__.bindCanvas(index)
  }
})
