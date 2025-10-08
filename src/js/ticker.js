document.addEventListener('DOMContentLoaded', () => {
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
});
