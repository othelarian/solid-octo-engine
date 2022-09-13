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

option '-n', '--nostart', 'cancel nwjs start (ONLY for `cake heavy`'
option '-r', '--release', 'set compilation mode to release'

# GLOBAL VARS #########################

cfg = require('./config.default').cfg

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
        style = if cfg.envRelease then 'compressed' else 'expanded'
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
  inOpts = {input: "#{cfg.webSources}/#{inFile}", plugins: [rollStatic name]}
  outOpts = {file: "#{cfg.dest}/#{outFile}", exports: 'default', format: 'cjs'}
  if cfg.watching
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
  cfgpath = './config.coffee'
  try
    fse.accessSync cfgpath
    cfgov = require(cfgpath).cfg
    for key, value of cfgov
      cfg[key] = value
  cfg.envRelease = if options.release? then true else false
  cfg.watching = false
  cfg.dest = cfg.dest_path[if cfg.envRelease then 'release' else 'debug']

compileJs = (cb) ->
  inOpts =
    input: "#{cfg.webSources}/index.coffee"
    plugins: [rollCoffee(), rust {debug: not cfg.envRelease}]
  outOpts =
    file: "./#{cfg.dest}/index.js"
    format: 'iife'
    assetFileNames: 'wasm/[name][extname]'
    plugins: (if cfg.envRelease then [terser()] else [])
  bundle = await rollup inOpts
  await bundle.write outOpts
  traceExec 'coffee/wasm'
  cb null, 0

compilePug = (cb) -> rollExec 'index.pug', 'index.html', 'pug', cb

compileSass = (cb) -> rollExec 'style.sass', 'style.css', 'sass', cb

createDir = (cb) ->
  try
    await fse.mkdirs "./#{cfg.dest}/static"
    await fse.copy './static', "./#{cfg.dest}/static"
    cb null, 0
  catch e
    if e.code = 'EEXIST'
      if not cfg.envRelease
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

task_cleandesc =
  "rm ./#{cfg.dest_path.debug} or ./#{cfg.dest_path.release} (debug or release)"
task 'clean', task_cleandesc, (options) ->
  checkEnv options
  console.log "cleaning `#{cfg.dest}`..."
  rimraf "./#{cfg.dest}", (e) ->
    if e? then console.log e
    else console.log "`#{cfg.dest}` removed successfully"

task 'itch', 'generate the bundle for itch.io', (options) ->
  checkEnv {release: true}
  filepath = "./soe_itch-#{(require './package').version}.zip"
  await fse.access filepath, fse.constants.F_OK, (e) -> if not e? then fse.rmSync filepath
  console.log 'itch building...'
  finalStep = ->
    console.log 'zipping...'
    zip = new admzip()
    zip.addLocalFolder cfg.dest
    zip.writeZip filepath
    console.log "======> itch zip ready: #{filepath}"
  try
    fse.accessSync cfg.dest
    finalStep()
  catch
    building (e, _) ->
      if e?
        console.log 'Something went wrong'
        console.log e
      else
        finalStep()

task 'heavy', 'build the nw/electron version', (options) ->
  checkEnv options
  #
  # TODO: si le dossier n'existe pas, c'est que l'app n'a pas été compilée
  #
  firstStep = try
    fse.accessSync cfg.dest
    (cb) -> cb null, 0
  catch
    building
  #
  #
  # TODO: copie et renommage du package.json spécial nwjs
  #
  sndStep = (cb) ->
    #
    #
    console.log 'not ready (snd step)'
    #
    cb null, 0
    #
  #
  #
  # TODO: compilation du coffee
  #
  #
  console.log require('./package').version
  #
  #
  # TODO: lancement de l'app
  #
  launchApp = (cb) ->
    #
    # TODO
    #
    cb null, 0
    #
  #
  (bach.series firstStep, sndStep, launchApp) (e, _) -> if e? then console.log e
  #

task 'serve', 'launch a micro server and watch files', (options) ->
  checkEnv options
  if cfg.envRelease
    console.error 'Impossible to use `serve` in `release` mode!'
  else
    cfg.watching = true
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
