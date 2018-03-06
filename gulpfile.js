var execFile = require('child_process').execFile
var gulp = require('gulp')
var concat = require('gulp-concat')
var sourcemaps = require('gulp-sourcemaps')
var webpack = require('webpack')

var libCompilerConfig = require('./lib/webpack.config')

var execShellScript = function(path, cwd, cb) {
  execFile('sh', [path], {cwd: cwd}, function(err, stdout, stderr) {
    if (err) {
      console.error(stderr)
      cb(err)
      return
    }
    cb()
  })
}

var convertSourceMapPath = function() {
  return sourcemaps.mapSources(function(sourcePath, file) {
    if (sourcePath.match(/^webpack:\/\/__lanyoutLib__\/webpack\//)) {
      return sourcePath.replace('webpack://__lanyoutLib__/webpack/', '__internal__/webpack/')
    }
    if (sourcePath.match(/^webpack:\/\/__lanyoutLib__\/lib\//)) {
      return sourcePath.replace('webpack://__lanyoutLib__/lib/', 'lib/src/')
    }
    if (sourcePath.match(/^..\//)) {
      return sourcePath.replace(/^(..\/)+/, '__internal__/')
    }
    if (sourcePath.match(/^src\//)) {
      return sourcePath.replace('src/', 'asm/src/')
    }
    return '__internal__/' + sourcePath
  })
}

gulp.task('clean', function(cb) {
  execShellScript('scripts/clean.sh', 'asm', cb)
})

gulp.task('compile-lib-debug', function(cb) {
  webpack(libCompilerConfig[0]).run(cb)
});

gulp.task('compile-lib-release', function(cb) {
  webpack(libCompilerConfig[1]).run(cb)
});

gulp.task('watch-lib', function(cb) {
  webpack(libCompilerConfig[0]).watch({}, function(err, stats) {
    console.log(stats)
  })
})

gulp.task('compile-asm-debug', function(cb) {
  execShellScript('scripts/build-asmjs-debug.sh', 'asm', cb)
})

gulp.task('compile-asm-release', function(cb) {
  execShellScript('scripts/build-asmjs-release.sh', 'asm', cb)
})

gulp.task('build-release', ['compile-lib-release', 'compile-asm-release'], function(cb) {
  return gulp.src(['./lib/bin/lanyout-lib.min.js', './asm/target/asmjs-unknown-emscripten/release/lanyout-asm.js'])
    .pipe(concat('lanyout.min.js'))
    .pipe(gulp.dest('./bin/'))
})

gulp.task('build-debug-with-sourcemap', ['compile-lib-debug', 'compile-asm-debug'], function(cb) {
  return gulp.src(['./lib/bin/lanyout-lib.js', './asm/target/asmjs-unknown-emscripten/debug/deps/lanyout_asm-*.js'])
    .pipe(sourcemaps.init({loadMaps: true}))
    .pipe(concat('lanyout.js'))
    .pipe(convertSourceMapPath())
    .pipe(sourcemaps.write('.', {includeContent: true}))
    .pipe(gulp.dest('./bin/'))
})

gulp.task('build-debug', ['compile-lib-debug', 'compile-asm-debug'], function(cb) {
  return gulp.src(['./lib/bin/lanyout-lib.js', './asm/target/asmjs-unknown-emscripten/debug/deps/lanyout_asm-*.js'])
    .pipe(sourcemaps.init({loadMaps: true}))
    .pipe(concat('lanyout.js'))
    .pipe(convertSourceMapPath())
    .pipe(sourcemaps.write('.', {includeContent: false, sourceRoot: '..'}))
    .pipe(gulp.dest('./bin/'))
})

gulp.task('watch', ['build-debug'], function() {
  gulp.watch(['asm/src/*.rs', 'asm/src/**/*.rs', 'lib/*.js',  'lib/src/*.js', 'lib/src/**/*.js'], ['build-debug'])
    .on('change', function(event) {
      console.log('file: ' + event.path + ' was ' + event.type);
    })
})

gulp.task('default', ['build-release'])
