// https://github.com/jakearchibald/svgomg/blob/main/src/js/utils/storage.js
const storage = (() => {
  let dbInstance;

  function getDB() {
    if (dbInstance) return dbInstance;

    dbInstance = new Promise((resolve, reject) => {
      const openreq = indexedDB.open('my-keyval', 2);

      openreq.onerror = () => {
        reject(openreq.error);
      };

      openreq.onupgradeneeded = () => {
        // First time setup: create an empty object store
        openreq.result.createObjectStore('keyval');
      };

      openreq.onsuccess = () => {
        resolve(openreq.result);
      };
    });

    return dbInstance;
  }

  async function withStore(type, callback) {
    const db = await getDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction('keyval', type);
      transaction.oncomplete = () => resolve();
      transaction.onerror = () => reject(transaction.error);
      callback(transaction.objectStore('keyval'));
    });
  }

  return {
    async get(key) {
      let request;
      await withStore('readonly', (store) => {
        request = store.get(key);
      });
      return request.result;
    },
    set(key, value) {
      return withStore('readwrite', (store) => {
        store.put(value, key);
      });
    },
    delete(key) {
      return withStore('readwrite', (store) => {
        store.delete(key);
      });
    },
  };
})();


// Modified from https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Guides/Offline_and_background_operation

const CACHE_NAME = "v2";

const putInCache = async (request, response) => {
  const cache = await caches.open(CACHE_NAME);
  await cache.put(request, response);
};

let dbInstance;

const failbackResponse = new Response("Network error happened", {
  status: 408,
  headers: { "Content-Type": "text/plain" },
});

const cacheFirst = async (request) => {
  // First try to get the resource from the cache.
  const responseFromCache = await caches.open(CACHE_NAME).then((cache) => cache.match(request));
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
  const responseFromCache = await caches.open(CACHE_NAME).then((cache) => cache.match(request));;
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