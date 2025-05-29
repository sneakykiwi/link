import { Metric } from 'web-vitals';

export function register() {
  if (typeof window !== 'undefined' && process.env.NODE_ENV === 'production') {
    import('web-vitals').then(({ onCLS, onINP, onFCP, onLCP, onTTFB }) => {
      onCLS(sendToAnalytics);
      onINP(sendToAnalytics);
      onFCP(sendToAnalytics);
      onLCP(sendToAnalytics);
      onTTFB(sendToAnalytics);
    });
  }
}

function sendToAnalytics(metric: Metric) {
  const body = JSON.stringify({
    name: metric.name,
    value: metric.value,
    id: metric.id,
    delta: metric.delta,
    rating: metric.rating,
    url: window.location.href,
    timestamp: Date.now(),
  });

  if (navigator.sendBeacon) {
    navigator.sendBeacon('/api/analytics/web-vitals', body);
  } else {
    fetch('/api/analytics/web-vitals', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body,
      keepalive: true,
    }).catch(() => {});
  }
} 