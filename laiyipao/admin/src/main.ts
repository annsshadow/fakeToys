import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import { router } from './router'
import './styles/main.css'

// ⚠️ 这里**不再** `app.use(ElementPlus)`。
//
// 原因：整包注册会把 element-plus 全量打进产物（实测 901 KB，占 45%）。
// 改为按需引入后，模板里用到的 `el-*` 由 `unplugin-vue-components`
// 在编译期自动 import（见 `vite.config.ts`）。
//
// **语言包**随之改由 `App.vue` 的 `<el-config-provider :locale>` 提供 ——
// 它是按需模式下唯一正确的注入方式。
//
// ⚠️ 移除整包注册有个**静默失败**风险：语言包如果没接上，
// 页面不会报错，只会把分页/日期选择器的文案从中文变成英文。
// `tests/DashboardView.test.ts` 那类断言中文文案的测试会先红，
// 但它们未必覆盖到 `el-pagination` 的内部文案 —— 所以这里写明。
const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
