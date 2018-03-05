var gulp = require('gulp')
var concat = require('gulp-concat')
var sourcemaps = require('gulp-sourcemaps')
var webpack = require('webpack')

var libCompilerConfig = require('./lib/webpack.config')

gulp.task('compile-lib-debug', function(callback) {
  webpack(libCompilerConfig[0]).run(callback)
});

gulp.task('compile-lib-release', function(callback) {
  webpack(libCompilerConfig[1]).run(callback)
});

gulp.task('watch-lib', function(callback) {
  webpack(libCompilerConfig[0]).watch({}, function(err, stats) {
    console.log(stats)
  })
})

gulp.task('compile-asm-debug', function(callback) {
  callback()
})

gulp.task('compile-asm-release', function(callback) {
  callback()
})

gulp.task('build-release', ['compile-lib-release', 'compile-asm-release'], function(callback) {
  return gulp.src(['./lib/bin/lanyout-lib.min.js', './asm/target/asmjs-unknown-emscripten/release/lanyout-asm.js'])
    .pipe(concat('lanyout.min.js'))
    .pipe(gulp.dest('./bin/'))
})

gulp.task('build-debug', ['compile-lib-debug', 'compile-asm-debug'], function(callback) {
  return gulp.src(['./lib/bin/lanyout-lib.js', './asm/target/asmjs-unknown-emscripten/debug/lanyout-asm.js'])
    .pipe(sourcemaps.init({loadMaps: true}))
    .pipe(concat('lanyout.js'))
    .pipe(sourcemaps.write('.'))
    .pipe(gulp.dest('./bin/'))
})

gulp.task('watch', ['build-debug'], function() {
  gulp.watch(['asm/*.js', 'asm/**/*.js', 'lib/*.js', 'lib/**/*.js'], ['build-debug'])
    .on('change', function(event) {
      console.log('file: ' + event.path + ' was ' + event.type);
    })
})

gulp.task('default', ['build-release'])
