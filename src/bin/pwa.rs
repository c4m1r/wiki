use std::fs;
use std::path::Path;
use serde_json::json;
use crate::logic::NervaConfig;

pub fn generate_web_app_manifest(output_dir: &Path, config: &NervaConfig) {
    let manifest = json!({
        "name": config.name,
        "short_name": config.name,
        "description": config.description,
        "start_url": "/",
        "display": "standalone",
        "background_color": "#ffffff",
        "theme_color": "#000000",
        "orientation": "portrait-primary",
        "scope": "/",
        "lang": config.default_language,
        "categories": ["documentation", "education", "productivity"],
        "icons": [
            {
                "src": "favicon/android-icon-36x36.png",
                "sizes": "36x36",
                "type": "image/png",
                "density": "0.75"
            },
            {
                "src": "favicon/android-icon-48x48.png",
                "sizes": "48x48",
                "type": "image/png",
                "density": "1.0"
            },
            {
                "src": "favicon/android-icon-72x72.png",
                "sizes": "72x72",
                "type": "image/png",
                "density": "1.5"
            },
            {
                "src": "favicon/android-icon-96x96.png",
                "sizes": "96x96",
                "type": "image/png",
                "density": "2.0"
            },
            {
                "src": "favicon/android-icon-144x144.png",
                "sizes": "144x144",
                "type": "image/png",
                "density": "3.0"
            },
            {
                "src": "favicon/android-icon-192x192.png",
                "sizes": "192x192",
                "type": "image/png",
                "density": "4.0"
            },
            {
                "src": "favicon/favicon-32x32.png",
                "sizes": "32x32",
                "type": "image/png"
            },
            {
                "src": "favicon/favicon-16x16.png",
                "sizes": "16x16",
                "type": "image/png"
            }
        ]
    });

    let manifest_path = output_dir.join("site.webmanifest");
    if let Ok(json_content) = serde_json::to_string_pretty(&manifest) {
        let _ = fs::write(manifest_path, json_content);
    }
}

pub fn generate_service_worker(output_dir: &Path, config: &NervaConfig) {
    let service_worker_js = format!(r#"// NervaWeb Service Worker for PWA
const CACHE_NAME = 'nervaweb-v1-{}';
const STATIC_CACHE_NAME = 'nervaweb-static-v1-{}';

// Files to cache immediately
const STATIC_ASSETS = [
    '/',
    '/index.html',
    '/css/variables.css',
    '/css/general.css',
    '/css/chrome.css',
    '/css/print.css',
    '/js/nervaweb.js',
    '/favicon.ico',
    '/favicon/favicon.ico',
    '/FontAwesome/css/font-awesome.css',
    '/highlight.css',
    '/highlight.js',
    '/search_index.json'
];

// Install event - cache static assets
self.addEventListener('install', event => {{
    console.log('[SW] Install');
    event.waitUntil(
        caches.open(STATIC_CACHE_NAME)
            .then(cache => cache.addAll(STATIC_ASSETS))
            .then(() => self.skipWaiting())
    );
}});

// Activate event - clean old caches
self.addEventListener('activate', event => {{
    console.log('[SW] Activate');
    event.waitUntil(
        caches.keys().then(cacheNames => {{
            return Promise.all(
                cacheNames.map(cacheName => {{
                    if (cacheName !== CACHE_NAME && cacheName !== STATIC_CACHE_NAME) {{
                        console.log('[SW] Deleting old cache:', cacheName);
                        return caches.delete(cacheName);
                    }}
                }})
            );
        }}).then(() => self.clients.claim())
    );
}});

// Fetch event - serve from cache or network
self.addEventListener('fetch', event => {{
    const url = new URL(event.request.url);

    // Skip non-GET requests
    if (event.request.method !== 'GET') return;

    // Skip external requests
    if (!url.origin.includes(location.origin)) return;

    // Handle API requests differently
    if (url.pathname.startsWith('/api/')) {{
        event.respondWith(
            fetch(event.request)
                .then(response => {{
                    // Cache API responses
                    const responseClone = response.clone();
                    caches.open(CACHE_NAME).then(cache => {{
                        cache.put(event.request, responseClone);
                    }});
                    return response;
                }})
                .catch(() => {{
                    // Return cached API response if available
                    return caches.match(event.request);
                }})
        );
        return;
    }}

    // Handle static assets
    event.respondWith(
        caches.match(event.request)
            .then(cachedResponse => {{
                if (cachedResponse) {{
                    return cachedResponse;
                }}

                return fetch(event.request).then(response => {{
                    // Don't cache non-successful responses
                    if (!response.ok) return response;

                    // Cache successful responses
                    const responseClone = response.clone();
                    caches.open(CACHE_NAME).then(cache => {{
                        cache.put(event.request, responseClone);
                    }});

                    return response;
                }});
            }})
            .catch(() => {{
                // Return offline fallback for HTML pages
                if (event.request.headers.get('accept').includes('text/html')) {{
                    return caches.match('/index.html');
                }}
            }})
    );
}});
"#, config.name, config.name);

    let sw_path = output_dir.join("sw.js");
    let _ = fs::write(sw_path, service_worker_js);
}
