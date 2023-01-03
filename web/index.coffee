import core from '../soe/Cargo.toml'

getId = (id) -> document.getElementById id

SOE = {
  clearMenu: ->
    #
    # TODO
    console.log 'reached clearMenu'
    #
    (getId 'soe-menu').innerText = ''
  showScreen: -> (getId 'soe-veil').style.display = 'block'
  hideScreen: -> (getId 'soe-veil').style.display = 'none'
}

initSOE = ->
  if navigator.serviceWorker?
    navigator.serviceWorker.register 'sw.js', {scope: '/solid-octo-engine/'}
  app = await core()
  glzone = getId 'soe-glzone'
  app.start_soe()

window.SOE = SOE

window.initSOE = initSOE

