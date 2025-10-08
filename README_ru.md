# NervaWeb

Статический генератор сайтов с поддержкой многоязычности, полностью написанный на Rust. Создавайте красивые многоязычные сайты из Markdown файлов с поддержкой публикации на GitHub/GitLab Pages и зеркал.

## 🎯 Что такое NervaWeb?

**NervaWeb** - это современный статический генератор сайтов, созданный специально для создания многоязычных проектов.

### Ключевые особенности

1. **Чистый Rust** - высокая производительность и безопасность
2. **Многоязычность** - поддержка 12+ языков в одном файле
3. **Публикация** - встроенная поддержка GitHub/GitLab Pages и зеркал
4. **Расширяемость** - легко добавлять новые языки и темы
5. **Простота использования** - интуитивные команды
6. **Гибкость** - полная свобода в настройке

### Для кого предназначен?

- Разработчиков, создающих многоязычную документацию
- Блогеров с международной аудиторией
- Команд, работающих над проектами на нескольких языках
- Пользователей GitHub/GitLab Pages для публикации сайтов
- Всех, кто хочет простой и мощный инструмент для генерации сайтов с поддержкой зеркал

## 🚀 Быстрый старт

### Установка NervaWeb

```bash
# Глобальная установка
cargo install --path src/bin

# Проверка установки
nervaweb --version
```

### Создание и сборка проекта

```bash
# Создать новый проект
nervaweb new my-website --desc "My awesome website"

# Собрать проект
nervaweb build my-website

# Посмотреть все проекты
nervaweb content

# Очистить сборку
nervaweb clear
```

### Просмотр результата

```bash
# Запустить локальный сервер
python3 -m http.server 8000 -d good2go/my-website
# или
npx serve good2go/my-website -p 8000
# или
simple-http-server good2go/my-website -p 8000
```

### Интерактивные скрипты

```bash
# Linux/macOS - интерактивное меню
./src/bin/build.sh

# Windows - интерактивное меню
src\bin\build.bat
```

### Настройка публикации

После создания проекта откройте `projects/my-website/config.toml` и настройте:

```toml
# Основной URL публикации
primary_deployment_url = "https://username.github.io/repository/"

# Дополнительные зеркала для резервирования
[[deployment_mirrors]]
name = "GitHub Pages"
url = "https://username.github.io/repository/"
enabled = true

[[deployment_mirrors]]
name = "Локальная разработка"
url = "http://localhost:8080/"
ip = "127.0.0.1"
port = 8080
enabled = false
```

### Развертывание

1. **GitHub Pages**: Скопируйте содержимое `good2go/my-website/` в папку `docs/` или настройте на ветку `gh-pages`
2. **GitLab Pages**: Скопируйте содержимое в папку `public/`
3. **Другой хостинг**: Загрузите содержимое `good2go/my-website/` на ваш сервер
4. **Зеркала**: Настройте дополнительные локации в `config.toml` для резервирования

### Примеры URL для разных платформ:

- **GitHub Pages**: `https://username.github.io/repository/`
- **GitLab Pages**: `https://username.gitlab.io/project/`
- **Netlify**: `https://amazing-site.netlify.app/`
- **Vercel**: `https://project-name.vercel.app/`
- **Custom domain**: `https://yoursite.com/` или `https://blog.yoursite.com/`

### Архитектура NervaWeb

```
📁 nervaweb-workspace/          # Рабочая директория NervaWeb
├── 📁 projects/               # Все ваши проекты
│   └── 📁 my-website/         # Конкретный проект
│       ├── 📁 content/        # Markdown статьи
│       │   ├── hello-world.md # Главная страница (создается автоматически)
│       │   └── about.md       # Другие страницы
│       └── config.toml        # Конфигурация проекта
├── 📁 good2go/                # Собранные сайты (готовые к развертыванию)
│   └── 📁 my-website/         # Готовый HTML сайт
│       ├── index.html
│       ├── hello-world.html
│       ├── content/
│       ├── css/
│       ├── js/
│       ├── themes/            # Все доступные темы
│       └── fonts/
├── 📁 src/                    # Исходный код генератора
│   ├── 📁 themes/             # Темы с шаблонами и контентом
│   │   ├── 📁 hello-world/    # Простая тема (по умолчанию)
│   │   │   ├── index.hbs      # Шаблон HTML
│   │   │   ├── default.md     # Шаблон контента для новых проектов
│   │   │   └── fonts/, css/   # Стили и шрифты
│   │   └── 📁 wiki/          # Полнофункциональная тема
│   ├── 📁 css/               # Глобальные стили
│   ├── 📁 js/                # JavaScript файлы
│   │   ├── nervaweb.js       # Объединенная JS библиотека
│   │   ├── README.md          # Документация JS
│   │   └── [исходные файлы]   # Отдельные модули
│   └── 📁 bin/               # Rust код
│       ├── main.rs           # Основная логика
│       ├── logic.rs          # Общая логика проектов
│       ├── themes.rs         # Модуль управления темами
│       ├── themes.md         # Документация тем
│       ├── build.sh          # Интерактивный скрипт Linux/macOS
│       └── build.bat         # Интерактивный скрипт Windows
├── target/                    # Скомпилированные бинарники (новое расположение)
├── nervaweb.exe                     # Исполняемый файл генератора
└── README.md
```

### Особенности

- 🌍 **Многоязычная поддержка** - 12+ языков в одном файле
- 🎨 **Кастомные темы** - полная свобода в оформлении
- 🔄 **Динамическое переключение языков** - без перезагрузки страницы
- 📱 **Адаптивный дизайн** - работает на всех устройствах
- ⚡ **Чистый Rust** - высокая производительность и надежность
- 🏗️ **Расширяемая архитектура** - легко добавлять новые возможности

## 📚 Содержание

- [Главная страница](./src/content/index.md) - обзор всех категорий
- [Инструменты для скачивания](./src/content/toolkit.md) - полезные программы
- [Обновления](./src/content/updates.md) - последние изменения
- [Комментарии](./src/content/comments.md) - системы комментирования
- [Скрытые страницы](./src/content/hidden-pages.md) - демонстрация скрытого контента

## 🛠️ Команды NervaWeb

### Основные команды

```bash
nervaweb new <name>         # Создать новый проект
nervaweb build [project]    # Собрать проект(ы)
nervaweb clear              # Очистить все сборки
nervaweb content            # Показать все проекты со статистикой
nervaweb help               # Показать справку
nervaweb version            # Показать версию
```

### Создание проектов

```bash
nervaweb new my-blog                    # Создать проект с именем my-blog
nervaweb new portfolio --desc "My Work" # Создать с описанием
nervaweb new site --theme wiki          # Создать с выбранной темой
nervaweb new blog --desc "My Blog" --theme hello-world # С описанием и темой
```

### Сборка проектов

```bash
nervaweb build                 # Собрать все проекты
nervaweb build my-blog         # Собрать конкретный проект
nervaweb build my-blog --lang en --theme wiki --quiet  # С английским языком, темой wiki, тихо
```

### Опции команд

```bash
nervaweb new <name> [опции]
  --desc, -d <DESC>     # Описание проекта
  --theme, -t <THEME>   # Тема для контента (hello-world, wiki) - по умолчанию hello-world

nervaweb build [project] [опции]
  --lang, -l <LANG>     # Язык сборки (ru, en, es, de, fr, it, pt, zh, ja, ko, ar, hi)
  --theme, -t <THEME>   # Тема (hello-world, wiki) - по умолчанию hello-world
  --quiet, -q           # Тихий режим

nervaweb content [опции]
  --quiet, -q           # Только количество файлов
```

### Примеры использования

```bash
# Создание проектов
nervaweb new blog --desc "My personal blog"
nervaweb new docs --desc "Project documentation"

# Сборка
nervaweb build                    # Все проекты
nervaweb build blog               # Только блог
nervaweb build docs --lang en     # Документацию на английском
nervaweb build blog --theme dark  # Блог с темной темой

# Управление
nervaweb content                  # Посмотреть все проекты
nervaweb clear                    # Очистить все сборки

# Интерактивные скрипты
./src/bin/build.sh          # Меню для Linux/macOS
src\bin\build.bat           # Меню для Windows
```

## 🛠️ Технические детали

**NervaWeb** - это полностью оригинальный статический генератор сайтов, написанный на чистом Rust без использования сторонних библиотек генерации сайтов.

### Система многоязычности

Статьи хранятся в единых файлах с блоками для каждого языка:

```markdown
---
title:
  ru: Добро пожаловать
  en: Welcome
  es: Bienvenido
---

<!-- LANG: ru -->
Русский контент здесь...
<!-- END_LANG -->

<!-- LANG: en -->
English content here...
<!-- END_LANG -->

<!-- LANG: es -->
Contenido español aquí...
<!-- END_LANG -->
```

### Преимущества подхода

- ✅ **Единое управление контентом** - один файл для всех языков
- ✅ **Нет дублирования** - изменения автоматически применяются ко всем языкам
- ✅ **Легкое добавление языков** - просто добавьте новый блок LANG
- ✅ **Четкая структура** - языки явно разделены в коде

### Архитектура

```
src/bin/
├── main.rs           # Точка входа, обработка команд
├── config.rs         # Конфигурация проекта и языков
└── preprocessor.rs   # Обработка Markdown и многоязычности
```

### Поддерживаемые языки

NervaWeb поддерживает 12 языков из коробки:
- 🇷🇺 **Русский** (ru) - основной язык
- 🇺🇸 **English** (en)
- 🇪🇸 **Español** (es)
- 🇩🇪 **Deutsch** (de)
- 🇫🇷 **Français** (fr)
- 🇮🇹 **Italiano** (it)
- 🇵🇹 **Português** (pt)
- 🇨🇳 **中文** (zh)
- 🇯🇵 **日本語** (ja)
- 🇰🇷 **한국어** (ko)
- 🇸🇦 **العربية** (ar)
- 🇮🇳 **हिन्दी** (hi)

### Темы

NervaWeb поддерживает систему тем, которые полностью настраивают внешний вид сайта:

- **hello-world** (по умолчанию) - Простая тема с градиентным фоном, демонстрирующая все возможности Markdown
- **wiki** - Полнофункциональная тема с неоновой сеткой, языковым переключателем и расширенными возможностями

#### Структура темы

```
themes/[theme-name]/
├── index.hbs          # Шаблон HTML страницы
├── default.md         # Шаблон контента (опционально)
├── fonts/             # Шрифты
├── css/               # Стили темы
└── favicon.*          # Иконки
```

#### Создание собственной темы

1. Создайте папку `src/themes/my-theme/`
2. Добавьте `index.hbs` с вашим HTML шаблоном
3. Добавьте `default.md` с шаблоном контента (опционально)
4. Добавьте тему в `config.toml` проекта
5. Соберите с `--theme my-theme`

**Примечание**: Если в теме есть `default.md`, он будет использоваться как шаблон для новых проектов при выборе этой темы.

### 📦 Публикация тем через Crates.io

NervaWeb поддерживает публикацию тем как отдельных пакетов на [crates.io](https://crates.io):

1. **Создайте отдельный crate** для вашей темы:
   ```bash
   cargo new nervaweb-theme-mytheme
   cd nervaweb-theme-mytheme
   ```

2. **Настройте Cargo.toml**:
   ```toml
   [package]
   name = "nervaweb-theme-mytheme"
   version = "1.0.0"
   edition = "2021"
   description = "My awesome theme for NervaWeb"
   license = "MIT"
   repository = "https://github.com/username/nervaweb-theme-mytheme"
   keywords = ["nervaweb", "theme", "static-site"]
   categories = ["template-engine"]

   [dependencies]
   nervaweb-themes = "1.0"
   ```

3. **Организуйте структуру файлов**:
   ```
   nervaweb-theme-mytheme/
   ├── Cargo.toml
   ├── src/
   │   └── lib.rs
   └── themes/
       └── mytheme/
           ├── index.hbs
           ├── default.md
           └── assets/
   ```

4. **Опубликуйте на crates.io**:
   ```bash
   cargo publish
   ```

5. **Используйте тему** в проектах:
   ```toml
   [dependencies]
   nervaweb-theme-mytheme = "1.0"
   ```

Это позволяет создавать и распространять темы независимо от основного генератора NervaWeb.

## ❓ FAQ - Часто задаваемые вопросы

### 🔍 Где можно использовать команду `nw`?

**Команда `nw` работает ТОЛЬКО внутри папки проекта NervaWeb, где есть правильная структура файлов.**

#### ✅ Правильное использование:

   ```bash
# Перейти в папку проекта NervaWeb
cd /path/to/nervaweb-project

# Теперь все команды работают
nervaweb build     # ✅ Собрать сайт
nervaweb clear     # ✅ Очистить
nervaweb content   # ✅ Статистика

# Из любой другой папки
cd /any/other/folder
nervaweb build     # ❌ ОШИБКА - структура проекта не найдена
```

#### 🔧 Как работает определение проекта:

NervaWeb автоматически определяет корень проекта по расположению исполняемого файла:

1. **Если `nw.exe` в корне проекта** (рекомендуется):
   ```
   project/
   ├── nw.exe          ← Исполняемый файл здесь
   └── src/content/    ← Содержимое ищется относительно nw.exe
   ```

2. **Если `nw.exe` в `src/bin/target/release`** (при разработке):
   ```
   project/
   ├── src/
   │   └── bin/
   │       └── target/
   │           └── release/
   │               └── nw.exe  ← Автоматически поднимается вверх
   ```

#### 📁 Требуемая структура проекта:

```
nervaweb-project/
├── src/
│   ├── content/     # Статьи (.md файлы)
│   │   ├── index.md
│   │   ├── glpi/
│   │   ├── story/
│   │   └── ...
│   ├── theme/       # Шаблоны HTML
│   │   └── index.hbs
│   ├── css/         # Стили
│   └── bin/         # Исходный код
├── site/            # Выходная папка (создается nervaweb build)
└── nw.exe           # Исполняемый файл (создается установкой)
```

### 🚀 Как установить NervaWeb глобально?

```bash
# Перейти в папку проекта
cd /path/to/nervaweb-project

# Установить глобально
cargo install --path src/bin

# Теперь nervaweb доступен из любой папки
nervaweb --version
```

### 🌍 Как добавить новый язык?

1. **Добавить язык в `config.rs`:**
```rust
enabled_languages: vec![
    "ru".to_string(),
    "en".to_string(),
    "es".to_string(),
    "new_lang".to_string(), // Ваш новый язык
],
```

2. **Добавить название и флаг:**
```rust
pub fn get_language_name(&self, lang: &str) -> &'static str {
    match lang {
        "new_lang" => "New Language",
        // ... другие языки
    }
}
```

3. **Использовать в статьях:**
```markdown
<!-- LANG: new_lang -->
Контент на новом языке
<!-- END_LANG -->
```

### 🎨 Как создать новую тему?

1. **Создать папку темы:**
```
src/theme/
├── default/
├── dark/
└── my-theme/          # Новая тема
    ├── index.hbs      # Шаблон
    ├── css/           # Стили темы
    └── assets/        # Ресурсы
```

2. **Добавить тему в `config.rs`:**
```rust
themes: vec![
    "default".to_string(),
    "dark".to_string(),
    "my-theme".to_string(), // Новая тема
],
```

3. **Использовать:**
```bash
nervaweb build --theme my-theme
```

### ⚡ Как ускорить сборку?

```bash
# Тихий режим (минимальный вывод)
nervaweb build --quiet

# Использовать для автоматизации
nervaweb build --lang en --quiet
```

### 🔧 Как обновить NervaWeb?

```bash
# Если установлен глобально
cargo install --path src/bin --force

# Или пересобрать локально
cd src/bin
cargo build --release
cp target/release/nervaweb ../..
```

### 📊 Что показывает команда `content`?

```bash
nervaweb content        # Показать все файлы с путями
nervaweb content -q     # Только количество файлов
```

Пример вывода:
```
📊 Content Statistics:
📁 Content directory: src/content
📄 Total .md files: 42
📁 Total files: 156
```

### 🐛 Что делать при ошибках?

**Проблема:** `Failed to read template`
```
✅ Решение: Проверьте, что файл src/theme/index.hbs существует
```

**Проблема:** `Language not supported`
```
✅ Решение: Добавьте язык в config.rs или используйте поддерживаемый
```

**Проблема:** `Command not found`
```
✅ Решение: Установите NervaWeb: cargo install --path src/bin
```

### 🔄 Можно ли использовать с CI/CD?

Да! NervaWeb идеален для автоматизации:

```yaml
# .github/workflows/deploy.yml
name: Deploy Site
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install --path src/bin
      - run: nervaweb build --lang en --quiet
      - run: nervaweb build --lang ru --quiet
```

### 💻 Установка и использование на разных платформах

#### 🪟 Windows

**Требования:**
- Windows 10/11
- [Rust](https://rustup.rs/) установлен
- Git (опционально, для клонирования репозитория)

**Установка:**
```bash
# Клонируем репозиторий
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb

# Компилируем исполняемый файл
cargo build --release --manifest-path src\bin\Cargo.toml

# Копируем в корень проекта (рекомендуется)
copy src\bin\target\release\nervaweb.exe .\nervaweb.exe
```

**Использование:**
```bash
# Создаем проект
nervaweb.exe new my-site --desc "Мой сайт"

# Собираем проект
nervaweb.exe build my-site

# Запускаем локальный сервер
python -m http.server 8000 -d good2go\my-site
```

**Интерактивные скрипты:**
```cmd
# Запускаем скрипт сборки
src\bin\build.bat
```

#### 🐧 Linux

**Требования:**
- Ubuntu/Debian/CentOS/RHEL/Fedora или другой дистрибутив Linux
- [Rust](https://rustup.rs/) установлен
- Git (опционально, для клонирования репозитория)

**Установка:**
```bash
# Клонируем репозиторий
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb

# Компилируем исполняемый файл
cargo build --release --manifest-path src/bin/Cargo.toml

# Копируем в корень проекта (рекомендуется)
cp src/bin/target/release/nervaweb ./nervaweb

# Делаем исполняемым
chmod +x nervaweb
```

**Использование:**
```bash
# Создаем проект
./nervaweb new my-site --desc "Мой сайт"

# Собираем проект
./nervaweb build my-site

# Запускаем локальный сервер
python3 -m http.server 8000 -d good2go/my-site
```

**Интерактивные скрипты:**
```bash
# Запускаем скрипт сборки
./src/bin/build.sh
```

#### 🍎 macOS

**Требования:**
- macOS 10.15 или новее
- [Xcode Command Line Tools](https://developer.apple.com/xcode/) установлены
- [Rust](https://rustup.rs/) установлен
- Git (опционально, для клонирования репозитория)

**Установка:**
```bash
# Устанавливаем Xcode Command Line Tools (если не установлены)
xcode-select --install

# Клонируем репозиторий
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb

# Компилируем исполняемый файл
cargo build --release --manifest-path src/bin/Cargo.toml

# Копируем в корень проекта (рекомендуется)
cp src/bin/target/release/nervaweb ./nervaweb

# Делаем исполняемым
chmod +x nervaweb
```

**Использование:**
```bash
# Создаем проект
./nervaweb new my-site --desc "Мой сайт"

# Собираем проект
./nervaweb build my-site

# Запускаем локальный сервер
python3 -m http.server 8000 -d good2go/my-site
```

**Интерактивные скрипты:**
```bash
# Запускаем скрипт сборки
./src/bin/build.sh
```

#### 🐳 Docker (Кросс-платформенный)

**Требования:**
- [Docker](https://docker.com/) установлен

**Использование:**
```bash
# Клонируем репозиторий
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb

# Собираем Docker образ
docker build -t nervaweb .

# Создаем проект
docker run --rm -v $(pwd):/app nervaweb new my-site --desc "Мой сайт"

# Собираем проект
docker run --rm -v $(pwd):/app nervaweb build my-site

# Запускаем локальный сервер
docker run --rm -p 8000:8000 -v $(pwd)/good2go/my-site:/site \
  nginx:alpine -c /dev/null -p /site -l 8000
```

#### ☁️ GitHub Codespaces / GitPod / VS Code Remote

**Автоматическая настройка:**
Если вы используете GitHub Codespaces или подобную облачную среду разработки, Rust обычно предустановлен.

```bash
# Клонируем и собираем
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb
cargo build --release --manifest-path src/bin/Cargo.toml
cp src/bin/target/release/nervaweb ./nervaweb
chmod +x nervaweb

# Используем как обычно
./nervaweb new my-site
./nervaweb build my-site
```

#### 🔧 Глобальная установка (Все платформы)

**Через Cargo:**
```bash
# Устанавливаем глобально (работает на любой платформе с Rust)
cargo install --path src/bin

# Теперь nervaweb доступен системно
nervaweb --version
nervaweb new my-site
```

**Ручная установка:**
1. Скачайте готовый бинарный файл из [Releases](https://github.com/yourusername/nervaweb/releases)
2. Распакуйте в папку из PATH
3. Сделайте исполняемым (Linux/macOS): `chmod +x nervaweb`

### 🎯 Особенности по платформам

#### Windows
- Используйте `nervaweb.exe` вместо `nervaweb`
- Используйте `python` вместо `python3`
- Используйте обратные слеши `\` в путях внутри batch файлов
- Интерактивные скрипты: `build.bat`, `deploy.bat`, `clean.bat`

#### Linux
- Используйте `./nervaweb` для запуска исполняемого файла
- Используйте `python3` для локального сервера
- Интерактивные скрипты: `build.sh`, `deploy.sh`, `clean.sh`
- Может потребоваться дополнительные пакеты: `build-essential`, `pkg-config`

#### macOS
- То же что и Linux, но требуются Xcode Command Line Tools
- Используйте `python3` из Homebrew если системный Python старый
- Может потребоваться установка дополнительных инструментов через Homebrew

#### ARM устройства (Raspberry Pi и т.д.)
```bash
# Используем ARM скрипт сборки
./src/bin/build-arm.sh

# Или кросс-компиляция
rustup target add arm-unknown-linux-gnueabihf
cargo build --release --target arm-unknown-linux-gnueabihf --manifest-path src/bin/Cargo.toml
```

### 🔍 Устранение неисправностей по платформам

#### Проблемы Windows
```
Ошибка: 'nervaweb' is not recognized
Решение: Используйте 'nervaweb.exe' или добавьте в PATH
```

```
Ошибка: cargo not found
Решение: Установите Rust с https://rustup.rs/
```

#### Проблемы Linux
```
Ошибка: Permission denied
Решение: chmod +x nervaweb
```

```
Ошибка: libssl not found
Решение: sudo apt-get install libssl-dev pkg-config
```

#### Проблемы macOS
```
Ошибка: xcode-select not found
Решение: xcode-select --install
```

```
Ошибка: ld: library not found
Решение: Установите Xcode Command Line Tools
```

### 💡 Советы по использованию

1. **Всегда работайте в папке проекта** - `nw` ищет структуру относительно своего расположения
2. **Используйте `--quiet` для скриптов** - уменьшает вывод в логах
3. **Добавляйте языки постепенно** - переводите статьи поэтапно
4. **Тестируйте локально** - используйте `python3 -m http.server 8000` для проверки
5. **Делайте бэкапы** - `nervaweb clear` удаляет все сгенерированные файлы

### 📞 Поддержка

- 📖 **Документация:** README.md
- 🐛 **Баги:** Создавайте issues в репозитории
- 💡 **Идеи:** Предложения приветствуются
- 🤝 **Вклад:** PR всегда welcome!
