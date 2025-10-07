document.addEventListener('DOMContentLoaded', () => {
  // Проверяем, находимся ли мы на корневой странице
  if (window.location.pathname === '/' || window.location.pathname === '') {
    // Перенаправляем на главную страницу русской версии по умолчанию
    window.location.href = '/ru_index.html';
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
    button.textContent = `${flag} ${langName}`;
    button.style.cssText = `
      background: var(--bg);
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
      button.style.background = 'var(--bg)';
    });

    button.addEventListener('click', () => {
      // Определяем текущий путь
      const currentPath = window.location.pathname;

      // Маппинг путей для разных языков
      const pathMappings = {
        '/': '/',
        '/en/': '/',
        '/ru/': '/',
        '/es/': '/',
        '/glpi/': '/glpi/',
        '/rust-renderers/': '/rust-renderers/',
        '/comments/': '/comments/',
        '/multilanguage/': '/multilanguage/',
        '/toolkit/': '/toolkit/',
        '/updates/': '/updates/',
        '/hidden-pages/': '/hidden-pages/',
        '/story/01-intro/': '/story/01-intro/',
        '/story/_02-secret/': '/story/_02-secret/'
      };

      // Проверяем, есть ли прямое соответствие
      let newPath = pathMappings[currentPath];
      if (!newPath) {
        // Если нет прямого соответствия, пробуем найти по паттерну
        if (currentPath.startsWith('/ru/')) {
          newPath = currentPath.replace('/ru/', '/');
        } else if (currentPath.startsWith('/en/')) {
          newPath = currentPath.replace('/en/', '/');
        } else if (currentPath.startsWith('/es/')) {
          newPath = currentPath.replace('/es/', '/');
        } else if (currentPath.includes('/ru/')) {
          newPath = currentPath.replace('/ru/', '/');
        } else if (currentPath.includes('/en/')) {
          newPath = currentPath.replace('/en/', '/');
        } else if (currentPath.includes('/es/')) {
          newPath = currentPath.replace('/es/', '/');
        } else {
          newPath = currentPath;
        }
      }

      // Добавляем префикс языка к пути
      if (langCode !== 'default') {
        newPath = '/' + langCode + newPath;
      }

      // Переходим на новую страницу
      if (newPath !== currentPath) {
        window.location.href = newPath;
      }
    });

    return button;
  }

  // Создаем кнопки для каждого языка
  const languages = [
    { code: 'default', name: 'EN', flag: '🇺🇸' },
    { code: 'ru', name: 'RU', flag: '🇷🇺' },
    { code: 'es', name: 'ES', flag: '🇪🇸' }
  ];

  languages.forEach(lang => {
    const button = createLanguageButton(lang.code, lang.name, lang.flag);
    languageContainer.appendChild(button);
  });

  // Добавляем контейнер в body
  document.body.appendChild(languageContainer);
});
