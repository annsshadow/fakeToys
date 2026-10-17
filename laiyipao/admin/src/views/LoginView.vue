<script setup lang="ts">
/**
 * 管理员登录。
 *
 * 刻意做得极简：只有一个表单。
 * 运营后台是内部工具，登录页不需要展示任何游戏内容 ——
 * 多一个能渲染的模块，就多一处可能泄漏信息的地方。
 */
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { adminLogin } from '@/api'
import { setToken, confirmTokenValid, invalidateTokenVerification } from '@/api/client'

const route = useRoute()
const router = useRouter()
const username = ref('')
const password = ref('')
const loading = ref(false)
const errMsg = ref('')

/**
 * 守卫 `toLogin(to)` 把目标路径写进 `?redirect=`。登录成功后应跳回那里。
 *
 * ⚠️ 第 155 轮起消费它。但 `redirect` 是**用户可改**的 query 参数，
 * 若不限制，`/admin/login?redirect=https://evil.com` 就成了开放重定向。
 * 判据：必须**以 `/admin` 开头**（后台路由都在这个前缀下），
 * 其余（外部 URL、协议相对 `//evil.com`、`/wiki` 等壳外路径）一律落到看板。
 * 这是「同源约束」，不是「白名单」—— 后台新增页面时自动生效，不用改这里。
 */
function redirectTarget(): string {
  const r = route.query.redirect
  if (typeof r === 'string' && r.startsWith('/admin')) return r
  return '/admin/dashboard'
}

async function submit() {
  if (!username.value.trim() || !password.value) {
    errMsg.value = '请填写用户名与密码'
    return
  }
  loading.value = true
  errMsg.value = ''
  try {
    const res = await adminLogin(username.value.trim(), password.value)
    setToken(res.access_token)
    // 登进来自成一次「重新验真」：把上一次会话留下的验真缓存作废，
    // 否则换号登录后会直接拿到上一个账号的「已验过」结论。
    invalidateTokenVerification()
    // 顺手把新令牌验一次，失败就停在登录页而不是进壳后才报错。
    if (await confirmTokenValid()) {
      ElMessage.success(`欢迎回来，${res.admin.username}`)
      router.replace(redirectTarget())
    }
  } catch (e) {
    errMsg.value = (e as Error).message
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  // 已有令牌就直接验一次，验过就跳过登录页。
  // 与路由守卫同一个判据（confirmTokenValid）—— 之前这里各自写一份，
  // 而守卫那份只看「有没有字符串」，于是两种入口行为不一致。
  if (!(await confirmTokenValid())) return
  router.replace(redirectTarget())
})
</script>

<template>
  <div class="login-page">
    <div class="login-card">
      <div class="brand">
        <div class="brand-mark">炮</div>
        <div>
          <div class="brand-name">来一炮</div>
          <div class="brand-sub">运营后台</div>
        </div>
      </div>

      <el-form label-position="top" @submit.prevent="submit">
        <el-form-item label="用户名">
          <el-input v-model="username" placeholder="admin" autocomplete="username" />
        </el-form-item>
        <el-form-item label="密码">
          <el-input
            v-model="password"
            type="password"
            placeholder="至少 10 位"
            autocomplete="current-password"
            show-password
            @keyup.enter="submit"
          />
        </el-form-item>

        <el-alert
          v-if="errMsg"
          type="error"
          :closable="false"
          show-icon
          :title="errMsg"
          style="margin-bottom: 12px"
        />

        <el-button type="primary" :loading="loading" style="width: 100%" @click="submit">
          登录
        </el-button>
      </el-form>

      <p class="hint">
        默认账号 <code>admin</code>。密码由环境变量
        <code>BOOTSTRAP_ADMIN_PASS</code> 决定，<strong>少于 10 位会被拒绝引导</strong>。
      </p>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--lyp-bg);
}
.login-card {
  width: 340px;
  background: #10151c;
  border: 1px solid var(--lyp-border);
  border-radius: 10px;
  padding: 26px;
}
.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 20px;
}
.brand-mark {
  width: 38px;
  height: 38px;
  border-radius: 9px;
  background: linear-gradient(135deg, #ff6b35, #ff3d71);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 19px;
  color: #fff;
}
.brand-name {
  font-weight: 600;
  font-size: 16px;
}
.brand-sub {
  font-size: 11px;
  color: var(--lyp-muted);
}
.hint {
  margin: 16px 0 0;
  font-size: 11px;
  color: #6e7681;
  line-height: 1.7;
}
code {
  font-family: ui-monospace, monospace;
  background: #1c2430;
  padding: 1px 4px;
  border-radius: 3px;
}
</style>
