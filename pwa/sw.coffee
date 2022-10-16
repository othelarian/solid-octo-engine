OFFLINE_VERSION = 1
CACHE_NAME = 'soe_offline'
OFFLINE_URL = 'index.html'

self.addEventListener 'install', (evt) =>
  evt.waitUntil(( =>
    cache = await cache.open CACHE_NAME
    await cache.add new Request OFFLINE_URL, {cache: 'reload'}
  )())

self.addEventListener 'activate', (evt) =>
  evt.waitUntil(( =>
    if 'navigationPreload' in self.registration
      await self.registration.navigationPreload.enable()
  )())

