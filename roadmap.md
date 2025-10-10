# 🗺️ NervaWeb Development Roadmap

## 📋 Current Status & TODO List

### ✅ Completed Features
- [x] **Basic project structure** - Multi-project architecture with projects/ and good2go/ directories
- [x] **Custom Rust builder** - Replaced mdBook with custom NervaWeb generator
- [x] **Interactive console menu** - ASCII animation with spider web, multilingual support
- [x] **Emoji replacement** - All emoji replaced with ASCII symbols from user list
- [x] **Favicon integration** - Complete favicon set for all platforms (Apple, Android, Windows, PWA)
- [x] **Theme system** - hello-world and wiki themes with full asset copying
- [x] **Language blocks** - `<!-- LANG: xx -->` support in single Markdown files
- [x] **Widget configuration** - Enable/disable search, ticker, counter, language switcher
- [x] **Deployment configuration** - Primary URL and mirror support in config.toml

### 🚧 Critical Issues to Fix

#### 🔥 **HIGH PRIORITY - Template Processing Bug**
- [x] **Template variables not rendering** - `{{ path_to_root }}` appearing in HTML instead of actual paths ✅ FIXED
- [x] **CSS/JS links broken** - All asset references showing `{{ path_to_root }}css/variables.css` in browser ✅ FIXED
- [x] **Handlebars template processing** - Templates not being processed during site generation ✅ FIXED
- [x] **Debug template rendering** - Add logging to see what data is passed to templates ✅ NOT NEEDED

#### 🔧 **Asset Copying Issues**
- [x] **Theme assets not copying** - CSS, JS, fonts not being copied to output directory ✅ FIXED
- [x] **Missing FontAwesome** - FontAwesome/css/font-awesome.css referenced but not copied ✅ FIXED
- [x] **Missing highlight.js files** - tomorrow-night.css, ayu-highlight.css, etc. not found ✅ FIXED
- [x] **Missing editor assets** - ace.js, editor.js, mode-rust.js not copied ✅ FIXED

### 📝 **Feature Development Plan**

#### **Phase 1: Core Functionality (Current Focus)**
- [x] **Refactor main.rs** - Extract console, language, and variables modules ✅ COMPLETED
- [x] **Fix template processing** - Get basic HTML generation working ✅ COMPLETED
- [x] **Fix asset copying** - Ensure all CSS, JS, fonts are copied correctly ✅ COMPLETED
- [x] **Test local server** - Verify generated sites work in browser ✅ COMPLETED
- [x] **Clean up favicon_io folder** - Move files and remove temporary directory ✅ COMPLETED

#### **Phase 2: Enhanced Features** ✅ **COMPLETED**
- [x] **Search functionality** - Implemented local full-text search with ElasticLunr ✅ COMPLETED
- [x] **Language switching** - Client-side language switching (redirect-based) ✅ COMPLETED
- [x] **Theme switching** - Dynamic theme switching without page reload ✅ COMPLETED
- [x] **Visitor counter** - Implemented with CountAPI and CSS styling ✅ COMPLETED
- [x] **Ticker widget** - Implemented with live clock and page title ✅ COMPLETED
- [x] **Clipboard functionality** - Code copy buttons working with native APIs ✅ COMPLETED

#### **Phase 3: Advanced Features** ✅ **COMPLETED**
- [x] **PWA support** - Service worker, offline functionality, web app manifest ✅ COMPLETED
- [x] **Internationalization** - Complete i18n support for all UI elements with translations for EN, RU, ES, DE ✅ COMPLETED
- [x] **Plugin system** - Allow custom themes and extensions with CLI, manifests, and examples ✅ COMPLETED
- [ ] **Deployment automation** - GitHub Pages, Netlify, custom domain deployment
- [ ] **Performance optimization** - Asset minification, lazy loading

#### **Phase 4: Production Ready**
- [ ] **Comprehensive testing** - Unit tests, integration tests
- [ ] **Documentation** - Complete user and developer documentation
- [ ] **CI/CD pipeline** - Automated testing and deployment
- [ ] **Security audit** - Code review and security checks
- [ ] **Performance monitoring** - Analytics and performance tracking

### 🐛 **Known Bugs & Issues**

#### **Template Processing**
```
ERROR: Templates showing raw Handlebars syntax in browser:
- {{ path_to_root }}css/variables.css
- {{ path_to_root }}js/nervaweb.js
- {{ title }}, {{ description }}, etc.
```

#### **Asset Management**
```
MISSING FILES:
- FontAwesome CSS framework
- Highlight.js themes (tomorrow-night.css, ayu-highlight.css)
- Code editor assets (ace.js, editor.js)
- Search assets (elasticlunr.min.js, mark.min.js)
```

#### **Interactive Menu**
```
ISSUES:
- Console clearing not working properly on Windows
- Some menu options not fully implemented (change language/theme)
- Error handling in menu functions
```

### 🔧 **Technical Debt**

#### **Code Quality**
- [ ] **Remove unused imports** - HashMap, various unused variables
- [ ] **Fix warnings** - All compiler warnings should be addressed
- [ ] **Error handling** - Improve error messages and recovery
- [ ] **Logging** - Add proper logging throughout the application

#### **Architecture**
- [ ] **Separate concerns** - Better separation between logic, templates, assets
- [ ] **Configuration management** - Simplify config file handling
- [ ] **Path handling** - Robust path resolution across platforms
- [ ] **Memory management** - Optimize memory usage for large sites

### 📊 **Testing & Quality Assurance**

#### **Unit Tests**
- [ ] **Template processing** - Test Handlebars rendering
- [ ] **Asset copying** - Test file operations
- [ ] **Configuration parsing** - Test TOML parsing
- [ ] **Path resolution** - Test cross-platform path handling

#### **Integration Tests**
- [ ] **Full site generation** - End-to-end site building
- [ ] **Local server** - Test generated sites in browser
- [ ] **Multi-language support** - Test language block processing
- [ ] **Theme switching** - Test theme loading and assets

### 🚀 **Deployment & Distribution**

#### **Packaging**
- [ ] **Cross-platform binaries** - Windows, macOS, Linux builds
- [ ] **Installer creation** - MSI, DMG, DEB packages
- [ ] **Docker image** - Containerized deployment
- [ ] **NPM package** - JavaScript wrapper for Node.js users

#### **Documentation**
- [ ] **User guide** - Complete setup and usage instructions
- [ ] **Theme development** - How to create custom themes
- [ ] **API documentation** - Rust API docs with examples
- [ ] **Troubleshooting** - Common issues and solutions

### 📈 **Performance & Scalability**

#### **Optimization**
- [ ] **Build speed** - Parallel processing for large sites
- [ ] **Memory usage** - Stream processing for large files
- [ ] **Asset optimization** - Minification, compression
- [ ] **Caching** - Build cache for faster incremental builds

#### **Monitoring**
- [ ] **Build metrics** - Time, memory, file counts
- [ ] **Error tracking** - Better error reporting
- [ ] **Performance profiling** - Identify bottlenecks
- [ ] **User analytics** - Optional usage statistics

### 🎯 **Success Criteria**

#### **MVP (Minimum Viable Product)**
- [ ] **Working site generation** - Templates process correctly
- [ ] **All assets copied** - CSS, JS, fonts, images load
- [ ] **Local server works** - Sites display properly in browser
- [ ] **Basic themes work** - hello-world and wiki themes functional
- [ ] **Interactive menu** - All menu options working
- [ ] **Cross-platform** - Works on Windows, macOS, Linux

#### **Beta Release**
- [ ] **All widgets functional** - Search, counter, ticker, language switcher
- [ ] **Multi-language sites** - Full i18n support
- [ ] **Theme system complete** - Easy theme creation and switching
- [ ] **Deployment ready** - GitHub Pages, custom domains
- [ ] **Documentation complete** - User and developer docs
- [ ] **Stable API** - No breaking changes

#### **v1.0 Release**
- [ ] **Production tested** - Real-world usage validation
- [ ] **Performance optimized** - Fast builds, efficient runtime
- [ ] **Security reviewed** - Code security audit
- [ ] **Comprehensive testing** - 95%+ test coverage
- [ ] **Community feedback** - User testing and feedback incorporated

---

## 📅 **Timeline Estimate**

- **Week 1-2**: Fix template processing and asset copying (CRITICAL)
- **Week 3-4**: Complete widget functionality and testing
- **Week 5-6**: Documentation, packaging, and beta testing
- **Week 7-8**: Performance optimization and v1.0 preparation

---

*Last updated: October 9, 2025*
