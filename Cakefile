admzip = require 'adm-zip'
bach = require 'bach'
coffee = require 'coffeescript'
connect = require 'connect'
fse = require 'fs-extra'
http = require 'http'
{ extname } = require 'path'
pug = require 'pug'
rimraf = require 'rimraf'
{ rollup, watch } = require 'rollup'
rust = require '@wasm-tool/rollup-plugin-rust'
sass = require 'sass'
serveStatic = require 'serve-static'
{ terser } = require 'rollup-plugin-terser'

# OPTIONS #############################

option '-r', '--release', 'set compilation mode to release'

# GLOBAL VARS #########################

dest = ''
envRelease = false
watching = false
webSources = 'web'

# ROLLUP PLUGINS ######################

rollCoffee = =>
  name: 'rolling-coffee'
  transform: (code, id) ->
    if extname(id) != '.coffee' then return null
    out = coffee.compile code
    code: out

rollStatic = (variant) =>
  rendered = ''
  name: 'rolling-static'
  transform: (_, file) ->
    switch variant
      when 'pug' then rendered = pug.renderFile file
      when 'sass'
        style = if envRelease then 'compressed' else 'expanded'
        rendered = (sass.compile file, {style}).css
    'export default ""'
  generateBundle: (options, bundle) ->
    Object.keys(bundle).forEach (filename) => delete bundle[filename]
    fse.writeFileSync options.file, rendered
    return null

# COMMON FUNS #########################

traceExec = (name) ->
  stmp = new Date().toLocaleString()
  console.log "#{stmp} => #{name} compilation done"

rollExec = (inFile, outFile, name, cb) ->
  inOpts = {input: "#{webSources}/#{inFile}", plugins: [rollStatic name]}
  outOpts = {file: "#{dest}/#{outFile}", exports: 'default', format: 'cjs'}
  if watching
    watcher = watch {inOpts..., output: outOpts}
    watcher.on 'event', (event) ->
      if event.code == 'ERROR' then console.log event.error
      else if event.code == 'END' then traceExec name
    cb null, 0
  else
    toExec = await rollup inOpts
    await toExec.write outOpts
    traceExec name
    cb null, 0

# ACTION FUNS #########################

checkEnv = (options) ->
  if options.release then envRelease = true
  dest = if envRelease then 'docs' else 'dist'

compileJs = (cb) ->
  rustCfg = {debug: not envRelease}
  inOpts =
    input: "#{webSources}/index.coffee"
    plugins: [rollCoffee(), rust rustCfg]
  outOpts =
    file: "./#{dest}/index.js"
    format: 'iife'
    assetFileNames: 'wasm/[name][extname]'
    plugins: (if envRelease then [terser()] else [])
  bundle = await rollup inOpts
  await bundle.write outOpts
  traceExec 'coffee/wasm'
  cb null, 0

compilePug = (cb) -> rollExec 'index.pug', 'index.html', 'pug', cb

compileSass = (cb) -> rollExec 'style.sass', 'style.css', 'sass', cb

createDir = (cb) ->
  try
    await fse.mkdirs "./#{dest}/static"
    await fse.copy './static', "./#{dest}/static"
    cb null, 0
  catch e
    if e.code = 'EEXIST'
      if not envRelease
        console.warn 'Warning: \'dist\' already exists'
      cb null, 1
    else cb e, null

launchServer = ->
  console.log 'launching server...'
  app = connect()
  app.use(serveStatic './dist')
  http.createServer(app).listen 5000
  console.log 'dev server launched'

building = bach.series createDir, compileSass, compilePug, compileJs

# TASKS ###############################

task 'build', 'build the app (core + static + wasm)', (options) ->
  checkEnv options
  console.log 'building...'
  building (e, _) ->
    if e?
      console.log 'Something went wrong'
      console.log e
    else console.log 'building => done'

task 'clean', 'rm ./dist or ./docs (debug or release)', (options) ->
  checkEnv options
  console.log "cleaning `#{dest}`..."
  rimraf "./#{dest}", (e) ->
    if e? then console.log e
    else console.log "`#{dest}` removed successfully"

task 'itch', 'generate the bundle for itch.io', (options) ->
  checkEnv {release: true}
  filepath = "./soe_itch-#{(require './package').version}.zip"
  await fse.access filepath, fse.constants.F_OK, (e) -> if not e? then fse.rmSync filepath
  console.log 'itch building...'
  building (e, _) ->
    if e?
      console.log 'Something went wrong'
      console.log e
    else
      console.log 'zipping...'
      zip = new admzip()
      zip.addLocalFolder './docs'
      zip.writeZip filepath
      console.log "=====> itch zip ready (#{filepath})"

task 'heavy', 'build the nw/electron version', (options) ->
  checkEnv options
  #
  # TODO
  #
  console.log require('./package').version
  #
  #
  console.log 'not ready'
  #

task 'serve', 'launch a micro server and watch files', (options) ->
  checkEnv options
  if envRelease
    console.error 'Impossible to use `serve` in `release` mode!'
  else
    watching = true
    serving = bach.series createDir, compileJs,
      (bach.parallel compileSass, compilePug, launchServer)
    serving (e, _) -> if e? then console.log e

task 'static', 'compile sass, and copy html + markdown', (options) ->
  checkEnv options
  compileStatic = bach.parallel compileSass, compilePug
  (bach.series createDir, compileStatic) (e, _) -> if e? then console.log e

task 'wasm', 'use rollup to compile wasm and coffee', (options) ->
  checkEnv options
  compileJs -> 42
