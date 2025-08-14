document.addEventListener('DOMContentLoaded', () => {
  const form = document.getElementById('es-search-form');
  const input = document.getElementById('es-search-input');
  const overlay = document.getElementById('es-search-overlay');

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
});

function performSearch(query) {
  const esHost = 'https://example.com'; // replace with your Elasticsearch endpoint
  fetch(`${esHost}/wiki/_search?q=${encodeURIComponent(query)}&size=50`)
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
