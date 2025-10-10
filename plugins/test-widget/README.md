# test-widget Plugin

This is an example widget plugin for NervaWeb.

## Installation

1. Place this plugin in the `plugins/test-widget` directory
2. Enable the plugin in your NervaWeb configuration

## Usage

This plugin provides UI widgets functionality to extend NervaWeb.

## Configuration

Add to your `config.toml`:

```toml
[plugins]
test-widget = { enabled = true }
```

## Development

To modify this plugin:
1. Edit `plugin.toml` for metadata
2. Edit `assets/test-widget.js` for functionality
3. Add additional assets in the `assets/` directory
