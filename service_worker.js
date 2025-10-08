// Modified from https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Guides/Offline_and_background_operation
import { idbKeyval as storage } from './js_utils/storage.js';


const putInCache = async (request, response) => {
  const cache = await caches.open("v1");
  await cache.put(request, response);
};

let dbInstance;

const failbackResponse = new Response("Network error happened", {
  status: 408,
  headers: { "Content-Type": "text/plain" },
});

const cacheFirst = async (request) => {
  // First try to get the resource from the cache.
  const responseFromCache = await caches.match(request);
  if (responseFromCache) {
    // still update cache for next time
    fetch(request)
      .then(putInCache.bind(null, request))
      .catch((error) => {
        console.error("Background update failed:", error, request.url);
      });
    return responseFromCache;
  }

  // If the response was not found in the cache,
  // try to get the resource from the network.
  try {
    const responseFromNetwork = await fetch(request);
    // If the network request succeeded, clone the response:
    // - put one copy in the cache, for the next time
    // - return the original to the app
    // Cloning is needed because a response can only be consumed once.
    putInCache(request, responseFromNetwork.clone());
    return responseFromNetwork;
  } catch (error) {
    console.error(
      "Fetching failed:",
      error,
      request.url,
      "Returning offline page instead."
    );
    return failbackResponse;
  }
};

const cacheByTime = async (request, expire) => {
  // store cache time in indexedDB
  const responseFromCache = await caches.match(request);
  if (responseFromCache) {
    const ts = await storage.get(request.url);
    if (ts && (Date.now() - ts < expire * 1000)) {
      return responseFromCache;
    }
  }
  try {
    const responseFromNetwork = await fetch(request);
    await storage.set(request.url, Date.now());
    putInCache(request, responseFromNetwork.clone());
    return responseFromNetwork;
  } catch (error) {
    console.error("Fetching failed:", error);
    return responseFromCache || failbackResponse;
  }
};

self.addEventListener("fetch", (event) => {
  const request = event.request;

  if (request.url.startsWith(self.location.origin)) {
    // Use cache strategy for same-origin assets
    event.respondWith(cacheFirst(request));
  } else if (request.url.startsWith("https://hitscounter.dev/api/hit")) {
    event.respondWith(cacheByTime(request, 300));
  } else if (request.url.startsWith("https://gcore.jsdelivr.net/")) {
    // cache CDN resources
    event.respondWith(cacheByTime(request, 3600 * 24));
  }
});