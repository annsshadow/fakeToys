<script setup lang="ts">
/**
 * 管理员登录。
 *
 * 刻意做得极简：只有一个表单。
 * 运营后台是内部工具，登录页不需要展示任何游戏内容 ——
 * 多一个能渲染的模块，就多一处可能泄漏信息的地方。
 */
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { adminLogin, adminMe } from '@/api'
import { getToken, setToken } from '@/api/client'

const router = useRouter()
const username = ref('')
const password = ref('')
const loading = ref(false)
const errMsg = ref('')

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
    ElMessage.success(`欢迎回来，${res.admin.username}`)
    router.replace('/admin/dashboard')
  } catch (e) {
    errMsg.value = (e as Error).message
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  // 已有令牌就直接验一次，验过就跳过登录页
  if (!getToken()) return
  try {
    await adminMe()
    router.replace('/admin/dashboard')
  } catch {
    setToken('')
  }
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
