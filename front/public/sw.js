const CACHE_NAME = "news";
const CACHE_MAX_AGE = 3600 * 1000; // 1 hour in milliseconds.

self.addEventListener("install", (event) => {
  event.waitUntil(self.skipWaiting());
});

self.addEventListener("activate", (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener("fetch", (event) => {
  if (
    event.request.method === "POST" &&
    event.request.url.includes("/graphql")
  ) {
    event.respondWith(handleGraphQLRequest(event.request));
  } else {
    event.respondWith(fetch(event.request));
  }
});

async function handleGraphQLRequest(request) {
  const cache = await caches.open(CACHE_NAME);
  const cacheResponse = await cache.match(request);

  if (cacheResponse) {
    const cachedTime = new Date(cacheResponse.headers.get("sw-cached-time"));
    const isExpired = Date.now() - cachedTime > CACHE_MAX_AGE;

    if (!isExpired) {
      return cacheResponse;
    }
  }

  try {
    const networkResponse = await fetch(request.clone());
    const responseClone = networkResponse.clone();
    const headers = new Headers(networkResponse.headers);
    headers.set("sw-cached-time", new Date().toISOString());

    const cacheResponseWithHeaders = new Response(responseClone.body, {
      status: responseClone.status,
      statusText: responseClone.statusText,
      headers: headers,
    });

    cache.put(request, cacheResponseWithHeaders);
    return networkResponse;
  } catch (error) {
    if (cacheResponse) {
      return cacheResponse;
    } else {
      return new Response(
        JSON.stringify({ error: "Network error and no cached data available" }),
        { status: 503, statusText: "Service Unavailable" },
      );
    }
  }
}
