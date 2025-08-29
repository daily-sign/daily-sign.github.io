var cacheName = 'yew-pwa';
var filesToCache = [
  './',
  './daily-sign.js',
  './daily-sign.wasm',
  '//gcore.jsdelivr.net/npm/bootstrap@5.3.8/dist/js/bootstrap.bundle.min.js',
  '//gcore.jsdelivr.net/npm/bootstrap@5.3.8/dist/css/bootstrap.min.css',
];


/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function(e) {
  e.waitUntil(
    caches.open(cacheName).then(function(cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function(e) {
  e.respondWith(
    caches.match(e.request).then(function(response) {
      return response || fetch(e.request);
    })
  );
});