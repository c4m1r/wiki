document.addEventListener('DOMContentLoaded', () => {
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
});
