(function() {
    'use strict';

    // Global NervaWeb API object
    window.NervaWeb = window.NervaWeb || {
        version: '1.0.0',
        modules: [],
        config: {},
        // Utility functions
        isModuleLoaded: function(moduleName) {
            return this.modules.includes(moduleName);
        },
        loadConfig: function() {
            // Try to load config from meta tag or global variable
            const configMeta = document.querySelector('meta[name="nervaweb-config"]');
            if (configMeta) {
                try {
                    this.config = JSON.parse(configMeta.getAttribute('content') || '{}');
                } catch (e) {
                    console.warn('Failed to parse NervaWeb config:', e);
                }
            }
        }
    };

    // Load configuration on DOM ready
    document.addEventListener('DOMContentLoaded', function() {
        window.NervaWeb.loadConfig();
    });

    // --- VISITOR COUNTER MODULE ---
    if (window.NervaWeb.config.enable_visitor_counter !== false) {
        document.addEventListener('DOMContentLoaded', function() {
            const el = document.getElementById('visitor-counter');
            if (!el) return;

            const key = window.location.pathname.replace(/\//g, '_') || 'home';
            fetch(`https://api.countapi.xyz/hit/c4m1r_wiki/${key}`)
                .then(res => res.json())
                .then(data => {
                    el.textContent = `Visitors: ${data.value}`;
                })
                .catch(() => {
                    el.textContent = 'Visitors: N/A';
                });

            window.NervaWeb.modules.push('visitorCounter');
        });
    }

    // --- LANGUAGE SWITCHER MODULE ---
    if (window.NervaWeb.config.enable_language_switcher !== false) {
        document.addEventListener('DOMContentLoaded', function() {
            // Определяем текущий язык из URL параметров
            function getCurrentLanguage() {
                const urlParams = new URLSearchParams(window.location.search);
                return urlParams.get('lang') || 'ru'; // По умолчанию русский
            }

            // Получаем текущий язык
            const currentLang = getCurrentLanguage();

            // Проверяем, находимся ли мы на корневой странице
            if (window.location.pathname === '/' || window.location.pathname === '' || window.location.pathname.endsWith('index.html')) {
                // Перенаправляем на главную страницу с параметром языка
                window.location.href = `/index.html?lang=${currentLang}`;
                return;
            }

            // Создаем контейнер для языковых кнопок
            const languageContainer = document.createElement('div');
            languageContainer.id = 'language-switcher';
            languageContainer.style.cssText = `
                position: fixed;
                top: 20px;
                right: 20px;
                z-index: 1000;
                display: flex;
                gap: 8px;
                background: var(--bg);
                padding: 8px;
                border-radius: 8px;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
                border: 1px solid var(--fg);
            `;

            // Функция для создания языковой кнопки
            function createLanguageButton(langCode, langName, flag) {
                const button = document.createElement('button');
                button.textContent = `${flag} ${langName}${langCode === currentLang ? ' ✓' : ''}`;
                button.style.cssText = `
                    background: ${langCode === currentLang ? 'var(--sidebar-bg)' : 'var(--bg)'};
                    color: var(--fg);
                    border: 1px solid var(--fg);
                    padding: 6px 12px;
                    border-radius: 4px;
                    cursor: pointer;
                    font-size: 12px;
                    transition: all 0.2s;
                `;

                button.addEventListener('mouseenter', () => {
                    button.style.background = 'var(--sidebar-bg)';
                });

                button.addEventListener('mouseleave', () => {
                    button.style.background = langCode === currentLang ? 'var(--sidebar-bg)' : 'var(--bg)';
                });

                button.addEventListener('click', () => {
                    // Получаем текущий путь без расширения
                    const currentPath = window.location.pathname.replace(/\.html$/, '');

                    // Определяем новый URL с параметром языка
                    const newUrl = new URL(window.location.href);
                    newUrl.searchParams.set('lang', langCode);

                    // Переходим на новую страницу
                    window.location.href = newUrl.toString();
                });

                return button;
            }

            // Создаем кнопки для каждого языка (используем конфиг если доступен)
            const languages = window.NervaWeb.config.enabled_languages || [
                { code: 'en', name: 'EN', flag: '🇺🇸' },
                { code: 'ru', name: 'RU', flag: '🇷🇺' },
                { code: 'es', name: 'ES', flag: '🇪🇸' }
            ];

            languages.forEach(lang => {
                const button = createLanguageButton(lang.code, lang.name, lang.flag);
                languageContainer.appendChild(button);
            });

            // Добавляем контейнер в body
            document.body.appendChild(languageContainer);

            window.NervaWeb.modules.push('languageSwitcher');
        });
    }

    // --- TICKER MODULE ---
    if (window.NervaWeb.config.enable_ticker !== false) {
        document.addEventListener('DOMContentLoaded', function() {
            const ticker = document.getElementById('ticker-text');
            if (!ticker) return;

            function update() {
                const now = new Date();
                const time = now.toLocaleTimeString();
                const date = now.toLocaleDateString();
                const page = document.title;
                ticker.textContent = `${time} ${date} - ${page}`;
            }

            update();
            setInterval(update, 1000);

            window.NervaWeb.modules.push('ticker');
        });
    }

    // --- SEARCH MODULE ---
    if (window.NervaWeb.config.enable_search !== false) {
        document.addEventListener('DOMContentLoaded', function() {
            const form = document.getElementById('es-search-form');
            const input = document.getElementById('es-search-input');
            const overlay = document.getElementById('es-search-overlay');
            const soundToggle = document.getElementById('sound-toggle');

            let soundEnabled = false;
            let soundCount = 0;
            let audioCtx;

            function playTone(freq) {
                if (!audioCtx) {
                    audioCtx = new (window.AudioContext || window.webkitAudioContext)();
                }
                const osc = audioCtx.createOscillator();
                const gain = audioCtx.createGain();
                gain.gain.value = 0.1;
                osc.frequency.value = freq;
                osc.connect(gain);
                gain.connect(audioCtx.destination);
                osc.start();
                osc.stop(audioCtx.currentTime + 0.1);
            }

            if (soundToggle && input) {
                soundToggle.addEventListener('click', () => {
                    soundEnabled = !soundEnabled;
                    soundCount = 0;
                    soundToggle.classList.toggle('active', soundEnabled);
                });

                input.addEventListener('keydown', (e) => {
                    if (!soundEnabled) return;
                    if (e.key.length === 1) {
                        const pos = soundCount % 5;
                        if (pos < 3) {
                            playTone(440);
                        } else {
                            playTone(660);
                        }
                        soundCount++;
                    }
                });
            }

            if (form && input && overlay) {
                input.addEventListener('input', () => {
                    overlay.textContent = input.value;
                });
                form.addEventListener('submit', (e) => {
                    e.preventDefault();
                    const query = input.value.trim();
                    if (query) {
                        window.location.href = (typeof path_to_root !== 'undefined' ? path_to_root : '') + 'search.html?q=' + encodeURIComponent(query);
                    }
                });
            }

            const resultsContainer = document.getElementById('search-results');
            if (resultsContainer) {
                const params = new URLSearchParams(window.location.search);
                const q = params.get('q') || '';
                if (input) {
                    input.value = q;
                    overlay.textContent = q;
                }
                if (q) {
                    performSearch(q);
                }
            }

            window.NervaWeb.modules.push('search');
        });

        function performSearch(query) {
            const esHost = 'https://example.com'; // replace with your Elasticsearch endpoint
            fetch(`${esHost}/content/_search?q=${encodeURIComponent(query)}&size=50`)
                .then((res) => res.json())
                .then((data) => {
                    const hits = data.hits && data.hits.hits ? data.hits.hits : [];
                    const resultsContainer = document.getElementById('search-results');
                    const topContainer = document.getElementById('top-articles');
                    resultsContainer.innerHTML = '';
                    const counts = {};
                    hits.forEach((hit) => {
                        const source = hit._source || {};
                        const title = source.title || hit._id;
                        const url = source.url || '#';
                        const count = hit._score || 1;
                        counts[title] = (counts[title] || 0) + count;
                        const item = document.createElement('div');
                        const link = document.createElement('a');
                        link.href = url;
                        link.textContent = title;
                        item.appendChild(link);
                        resultsContainer.appendChild(item);
                    });

                    const sorted = Object.entries(counts).sort((a, b) => b[1] - a[1]).slice(0, 10);
                    const max = sorted.length ? sorted[0][1] : 1;
                    const min = sorted.length ? sorted[sorted.length - 1][1] : 0;
                    const range = Math.max(max - min, 1);
                    topContainer.innerHTML = '';
                    sorted.forEach(([title, count]) => {
                        const item = document.createElement('div');
                        const size = 1 + 0.1 * (count - min) / range;
                        item.style.fontSize = size + 'em';
                        item.textContent = `${title} (${count})`;
                        topContainer.appendChild(item);
                    });
                })
                .catch(() => {
                    const resultsContainer = document.getElementById('search-results');
                    if (resultsContainer) {
                        resultsContainer.textContent = 'Search error';
                    }
                });
        }
    }

})();
