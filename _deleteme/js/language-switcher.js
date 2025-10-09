document.addEventListener('DOMContentLoaded', () => {
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

  // Создаем кнопки для каждого языка
  const languages = [
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
});
