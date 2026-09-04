<template>
  <div class="login-container">
    <!-- 粒子背景 -->
    <canvas ref="canvasRef" class="particle-canvas"></canvas>

    <div class="login-wrapper">
      <div class="login-card glass-card">
        <!-- Logo -->
        <div class="login-header">
          <div class="logo-mark">◆</div>
          <h1 class="logo-title">OA4RUST</h1>
          <p class="logo-subtitle">下一代智能办公平台</p>
        </div>

        <!-- 登录表单 -->
        <form @submit.prevent="handleLogin" class="login-form">
          <div class="form-group">
            <label class="form-label">用户名</label>
            <input
              v-model="username"
              type="text"
              class="form-input"
              placeholder="请输入用户名"
              autofocus
              :disabled="loading"
            />
          </div>

          <div class="form-group">
            <label class="form-label">密码</label>
            <div class="password-wrapper">
              <input
                v-model="password"
                :type="showPassword ? 'text' : 'password'"
                class="form-input"
                placeholder="请输入密码"
                :disabled="loading"
              />
              <button
                type="button"
                class="eye-btn"
                @click="showPassword = !showPassword"
              >
                {{ showPassword ? '🙈' : '👁' }}
              </button>
            </div>
          </div>

          <!-- 验证码（需要时显示） -->
          <div v-if="showCaptcha" class="form-group">
            <label class="form-label">验证码</label>
            <div class="captcha-row">
              <input
                v-model="captchaAnswer"
                type="text"
                class="form-input captcha-input"
                placeholder="请输入验证码"
                :disabled="loading"
              />
              <img
                :src="captchaUrl"
                alt="验证码"
                class="captcha-img"
                @click="refreshCaptcha"
              />
            </div>
          </div>

          <div v-if="error" class="error-msg">{{ error }}</div>

          <button
            type="submit"
            class="login-btn neon-button"
            :disabled="loading || !username || !password"
          >
            <span v-if="loading" class="spinner"></span>
            <span v-else>登 录</span>
          </button>
        </form>

        <!-- 第三方登录 -->
        <div class="oauth-section">
          <div class="divider"><span>其他登录方式</span></div>
          <div class="oauth-buttons">
            <button
              v-for="provider in oauthProviders"
              :key="provider.id"
              class="oauth-btn"
              @click="handleOauth(provider)"
              :title="provider.name"
            >
              <span class="oauth-icon">{{ provider.icon }}</span>
            </button>
          </div>
        </div>

        <!-- 系统状态 -->
        <div v-if="systemUninitialized" class="init-warning">
          ⚠ 系统尚未初始化，请先设置管理员密码
        </div>
      </div>

      <!-- 右侧装饰 -->
      <div class="login-decoration">
        <div class="deco-content">
          <h2>智慧办公 · 无限可能</h2>
          <p>Workflow · Organization · Collaboration · Intelligence</p>
          <div class="stats-row">
            <div class="stat-item">
              <div class="stat-number">3000+</div>
              <div class="stat-label">API 接口</div>
            </div>
            <div class="stat-item">
              <div class="stat-number">40+</div>
              <div class="stat-label">业务模块</div>
            </div>
            <div class="stat-item">
              <div class="stat-number">98%</div>
              <div class="stat-label">Java 兼容</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useSession } from '@oa4rust/sdk';

const router = useRouter();
const route = useRoute();
const session = useSession();

const username = ref('');
const password = ref('');
const captchaAnswer = ref('');
const showPassword = ref(false);
const loading = ref(false);
const error = ref('');
const systemUninitialized = ref(false);
const showCaptcha = ref(false);
const captchaUrl = ref('');

const canvasRef = ref<HTMLCanvasElement>();
let animFrame: number;

// OAuth 提供商
const oauthProviders = [
  { id: 'qywx', name: '企业微信', icon: '💼', redirect: '/oauth/callback/qywx' },
  { id: 'dingding', name: '钉钉', icon: '📌', redirect: '/oauth/callback/dingding' },
  { id: 'mpweixin', name: '微信', icon: '💚', redirect: '/oauth/callback/mpweixin' },
  { id: 'sso', name: '统一认证', icon: '🔐', redirect: '/sso' },
];

const captchaId = ref('');

async function refreshCaptcha(): Promise<void> {
  try {
    const resp = await fetch('/jaxrs/authentication/captcha');
    const data = await resp.json() as { data: { image: string; id: string } };
    captchaUrl.value = `data:image/png;base64,${data.data.image}`;
    captchaId.value = data.data.id;
  } catch {
    showCaptcha.value = false;
  }
}

async function handleLogin(): Promise<void> {
  if (loading.value || !username.value || !password.value) return;

  loading.value = true;
  error.value = '';

  try {
    const params: Record<string, string> = {
      username: username.value,
      password: password.value,
    };
    if (showCaptcha.value && captchaId.value) {
      params.captchaId = captchaId.value;
      params.captchaAnswer = captchaAnswer.value;
    }

    await session.login(username.value, password.value, captchaId.value, captchaAnswer.value);

    // 登录后跳转
    const redirect = (route.query.redirect as string) || '/app/dashboard';
    router.replace(redirect);
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : '登录失败，请重试';
    error.value = msg;

    // 如果是 401 可能需要验证码
    if (msg.includes('captcha') || msg.includes('验证码')) {
      showCaptcha.value = true;
      refreshCaptcha();
    }
  } finally {
    loading.value = false;
  }
}

function handleOauth(provider: { id: string; name: string; icon: string; redirect: string }): void {
  window.location.href = `/jaxrs/authentication/oauth/login/${provider.id}/code/redirect`;
}

// 粒子背景动画
function initParticles(): void {
  const canvas = canvasRef.value;
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  interface Particle {
    x: number;
    y: number;
    vx: number;
    vy: number;
    size: number;
    opacity: number;
  }

  const particles: Particle[] = [];
  const count = Math.min(80, Math.floor(canvas.width * canvas.height / 15000));

  for (let i = 0; i < count; i++) {
    particles.push({
      x: Math.random() * canvas.width,
      y: Math.random() * canvas.height,
      vx: (Math.random() - 0.5) * 0.5,
      vy: (Math.random() - 0.5) * 0.5,
      size: Math.random() * 2 + 1,
      opacity: Math.random() * 0.5 + 0.2,
    });
  }

  function animate(): void {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // 绘制粒子
    for (const p of particles) {
      p.x += p.vx;
      p.y += p.vy;

      if (p.x < 0 || p.x > canvas.width) p.vx *= -1;
      if (p.y < 0 || p.y > canvas.height) p.vy *= -1;

      ctx.beginPath();
      ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
      ctx.fillStyle = `rgba(0, 212, 255, ${p.opacity})`;
      ctx.fill();
    }

    // 绘制连线
    for (let i = 0; i < particles.length; i++) {
      for (let j = i + 1; j < particles.length; j++) {
        const dx = particles[i].x - particles[j].x;
        const dy = particles[i].y - particles[j].y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (dist < 150) {
          ctx.beginPath();
          ctx.moveTo(particles[i].x, particles[i].y);
          ctx.lineTo(particles[j].x, particles[j].y);
          ctx.strokeStyle = `rgba(0, 212, 255, ${0.1 * (1 - dist / 150)})`;
          ctx.lineWidth = 0.5;
          ctx.stroke();
        }
      }
    }

    animFrame = requestAnimationFrame(animate);
  }

  animate();
}

function cleanupParticles(): void {
  if (animFrame) cancelAnimationFrame(animFrame);
}

onMounted(() => {
  initParticles();
  window.addEventListener('resize', () => {
    const canvas = canvasRef.value;
    if (canvas) {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    }
  });
});

onBeforeUnmount(() => {
  cleanupParticles();
});
</script>

<style scoped>
.login-container {
  position: relative;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background: var(--bg-base);
  display: flex;
  align-items: center;
  justify-content: center;
}

.particle-canvas {
  position: fixed;
  inset: 0;
  z-index: 0;
}

.login-wrapper {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 60px;
  padding: 40px;
  max-width: 1100px;
  width: 100%;
}

.login-card {
  width: 420px;
  padding: 48px 40px;
  flex-shrink: 0;
}

.login-header {
  text-align: center;
  margin-bottom: 36px;
}

.logo-mark {
  font-size: 48px;
  color: var(--color-primary);
  text-shadow: 0 0 30px var(--color-primary-glow);
  margin-bottom: 12px;
  display: block;
}

.logo-title {
  font-family: 'Orbitron', sans-serif;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-primary);
  letter-spacing: 6px;
  margin-bottom: 8px;
  text-shadow: 0 0 20px var(--color-primary-glow);
}

.logo-subtitle {
  font-size: 13px;
  color: var(--text-muted);
  letter-spacing: 2px;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.form-input {
  background: var(--bg-elevated);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: 12px 16px;
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: all var(--transition-fast);
}

.form-input:focus {
  border-color: var(--color-primary);
  box-shadow: var(--border-glow);
}

.form-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.password-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.password-wrapper .form-input {
  width: 100%;
  padding-right: 44px;
}

.eye-btn {
  position: absolute;
  right: 12px;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 18px;
  opacity: 0.6;
  transition: opacity var(--transition-fast);
}

.eye-btn:hover {
  opacity: 1;
}

.captcha-row {
  display: flex;
  gap: 12px;
  align-items: center;
}

.captcha-input {
  flex: 1;
}

.captcha-img {
  width: 100px;
  height: 40px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  border: 1px solid var(--border-subtle);
  object-fit: cover;
}

.error-msg {
  background: var(--color-error-glow);
  border: 1px solid var(--color-error);
  color: var(--color-error);
  padding: 10px 14px;
  border-radius: var(--radius-md);
  font-size: 13px;
}

.login-btn {
  background: linear-gradient(135deg, var(--color-primary), var(--color-primary-deep));
  border: none;
  border-radius: var(--radius-md);
  padding: 14px;
  color: var(--text-inverse);
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 4px;
  cursor: pointer;
  transition: all var(--transition-fast);
  box-shadow: 0 4px 20px var(--color-primary-glow);
}

.login-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 6px 30px var(--color-primary-glow);
}

.login-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.spinner {
  display: inline-block;
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255,255,255,0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.oauth-section {
  margin-top: 28px;
}

.divider {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
  color: var(--text-muted);
  font-size: 12px;
}

.divider::before,
.divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border-subtle);
}

.oauth-buttons {
  display: flex;
  justify-content: center;
  gap: 12px;
}

.oauth-btn {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
  background: var(--bg-elevated);
  cursor: pointer;
  font-size: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
}

.oauth-btn:hover {
  border-color: var(--color-primary);
  background: var(--color-primary-soft);
  transform: translateY(-2px);
}

.init-warning {
  margin-top: 16px;
  padding: 10px 14px;
  background: var(--color-warning-glow);
  border: 1px solid var(--color-warning);
  color: var(--color-warning);
  border-radius: var(--radius-md);
  font-size: 13px;
  text-align: center;
}

/* 右侧装饰区 */
.login-decoration {
  flex: 1;
  max-width: 500px;
}

.deco-content h2 {
  font-family: 'Orbitron', sans-serif;
  font-size: 32px;
  font-weight: 700;
  color: var(--color-primary);
  margin-bottom: 12px;
  text-shadow: 0 0 20px var(--color-primary-glow);
}

.deco-content p {
  color: var(--text-secondary);
  font-size: 14px;
  letter-spacing: 2px;
  margin-bottom: 40px;
}

.stats-row {
  display: flex;
  gap: 32px;
}

.stat-item {
  text-align: center;
}

.stat-number {
  font-family: 'Orbitron', sans-serif;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-primary);
  text-shadow: 0 0 15px var(--color-primary-glow);
}

.stat-label {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 4px;
  letter-spacing: 1px;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .login-wrapper {
    flex-direction: column;
    padding: 20px;
    gap: 24px;
  }

  .login-card {
    width: 100%;
    max-width: 400px;
    padding: 32px 24px;
  }

  .login-decoration {
    display: none;
  }
}
</style>
