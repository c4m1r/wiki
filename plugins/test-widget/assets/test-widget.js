// NervaWeb Widget Plugin: test-widget
(function() {
    'use strict';

    // Widget initialization
    document.addEventListener('DOMContentLoaded', function() {
        console.log('Widget test-widget loaded');

        // Create widget container
        const widget = document.createElement('div');
        widget.id = 'test-widget-widget';
        widget.className = 'nervaweb-plugin-widget';
        widget.innerHTML = `
            <div class="widget-header">
                <h3>test-widget</h3>
            </div>
            <div class="widget-content">
                <p>This is a custom widget from plugin test-widget</p>
                <button onclick="alert('Hello from test-widget!')">Click me</button>
            </div>
        `;

        // Add widget styles
        const style = document.createElement('style');
        style.textContent = `
            #test-widget-widget {
                position: fixed;
                top: 100px;
                right: 10px;
                width: 250px;
                background: var(--bg);
                border: 1px solid var(--border);
                border-radius: 8px;
                padding: 15px;
                z-index: 1000;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            }
            #test-widget-widget .widget-header {
                margin-bottom: 10px;
            }
            #test-widget-widget .widget-content button {
                background: var(--links);
                color: white;
                border: none;
                padding: 5px 10px;
                border-radius: 4px;
                cursor: pointer;
            }
        `;
        document.head.appendChild(style);
        document.body.appendChild(widget);
    });

    // Register widget
    if (window.NervaWeb) {
        window.NervaWeb.widgets = window.NervaWeb.widgets || {};
        window.NervaWeb.widgets['test-widget'] = {
            name: 'test-widget',
            version: '1.0.0',
            init: function() {
                console.log('Initializing widget test-widget');
            }
        };
    }
})();
