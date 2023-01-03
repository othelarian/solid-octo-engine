icndir = 'icons'
pwapath = 'pwa'
webpath = 'web'

exports.cfg =
  dest_path:
    debug: 'dist'
    release: 'out'
    github: 'docs'
  icon:
    dir: icndir
    src: ["#{pwapath}/icon.pug", "#{webpath}/logo.pug"]
    out: "#{icndir}/icon.svg"
  pwa:
    background_color: '#fff'
    description: 'Un jeu de Simon, mais sur internet'
    display: 'standalone'
    icon_sizes: [128, 192, 256, 512]
    icon_mask: [false, false, true, true]
    icon_svg: '72x72 96x96 1024x1024'
    lang: 'fr'
    name: 'Solid-Octo-Engine'
    path: pwapath
    service_worker: {src: "#{pwapath}/sw.coffee", out: 'sw.js'}
    'short-name': 'soe'
    start_url: 'index.html'
    theme_color: '#fff'
    scope: '/solid-octo-engine/'
  static: 'static'
  web:
    html:
      src: ("#{webpath}/#{file}" for file in ['index.pug', 'logo.pug'])
      out: 'index.html'
    path: webpath
    sass:
      src: "#{webpath}/style.sass"
      out: 'style.css'
    coffee:
      src: "#{webpath}/index.coffee"
      out: 'index.js'

