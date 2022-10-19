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
  if 'serviceWorker' in navigator then navigator.serviceWorker.register 'sw.js'
  app = await core()
  app.start_soe getId 'soe-glzone'

window.SOE = SOE

window.initSOE = initSOE

