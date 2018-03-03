/* global mergeInto: false, LibraryManager: false */

mergeInto(LibraryManager.library, {
  sample_callback: function(arg) {
    console.info('Sample callback: ' + arg);
  }
});
