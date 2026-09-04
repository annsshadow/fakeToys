/**
 * Service Worker 注册
 * 仅在 HTTPS 或 localhost 环境下生效
 */
export async function registerSW(): Promise<void> {
  if ('serviceWorker' in navigator && import.meta.env.PROD) {
    try {
      const registration = await navigator.serviceWorker.register('/sw.js', { scope: '/' });
      console.log('[PWA] ServiceWorker registered:', registration.scope);

      // 监听更新
      registration.addEventListener('updatefound', () => {
        const newSW = registration.installing;
        if (!newSW) return;
        newSW.addEventListener('statechange', () => {
          if (newSW.state === 'installed' && navigator.serviceWorker.controller) {
            // 有新版本，提示用户刷新
            console.log('[PWA] New content available; please refresh.');
            if (confirm('发现新版本，是否刷新？')) {
              window.location.reload();
            }
          }
        });
      });
    } catch (err) {
      console.warn('[PWA] ServiceWorker registration failed:', err);
    }
  }
}
