const CACHE_NAME = 'oa4rust-v1'
const ASSETS = ['/', '/index.html', '/favicon.svg', '/manifest.json']

// Install: cache core assets
self.addEventListener('install', (event) => {
  event.waitUntil(caches.open(CACHE_NAME).then((cache) => cache.addAll(ASSETS)))
  self.skipWaiting()
})

// Activate: clean old caches
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) => Promise.all(keys.filter((k) => k !== CACHE_NAME).map((k) => caches.delete(k)))),
  )
  self.clients.claim()
})

// Fetch: network first, fallback to cache
self.addEventListener('fetch', (event) => {
  if (event.request.method !== 'GET') return
  event.respondWith(
    fetch(event.request)
      .then((networkResponse) => {
        // Cache successful responses
        const responseClone = networkResponse.clone()
        caches.open(CACHE_NAME).then((cache) => cache.put(event.request, responseClone))
        return networkResponse
      })
      .catch(() => caches.match(event.request)),
  )
})

// Message: handle skip waiting
// 安全审计（2026-10 二轮）：纵深防御——仅接受来自同源（本 SW 注册源）页面的
// 消息，防同域既有 XSS 借 SKIP_WAITING 强制激活 pending SW（版本错位降级面）。
self.addEventListener('message', (event) => {
  if (event.origin !== self.location.origin) return
  if (event.data?.type === 'SKIP_WAITING') self.skipWaiting()
})
