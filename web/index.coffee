#import core from '../soe/Cargo.toml'

SOE = {
  #
  # TODO
  #
  plop: 'this is plop'
  #
}

initSOE = ->
  if 'serviceWorker' in navigator then navigator.serviceWorker.register 'sw.js'
  #
  # TODO
  #
  console.log 'starting SOE ...'
  #
  #
  # soe => soe-core
  #

window.SOE = SOE

window.initSOE = initSOE

