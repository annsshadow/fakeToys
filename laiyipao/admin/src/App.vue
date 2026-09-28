<script setup lang="ts">
import zhCn from 'element-plus/es/locale/lang/zh-cn'

// 根组件：只提供路由容器与环境（AdminLayout / WikiLayout 里）
//
// ⚠️ 这里的 `<el-config-provider>` 是**按需引入模式下语言包的唯一正确注入方式**。
//
// 之前语言包是靠 `app.use(ElementPlus, { locale: zhCn })` 注入的，
// 那行同时导致 element-plus 被**整包**打进产物（901 KB，占 45%）。
// 去掉整包注册后若不接上这里，页面**不会报错**，
// 只会把分页、日期选择器、表格工具栏的文案从中文悄悄变成英文 ——
// 一类「不红但错了」的退化，所以必须显式写明。
//
// 为什么要设：运营看的是中文报表，`el-pagination` 出现
// 「Total 0 / 前往 1 页」和「共 0 条 / 前往 1 页」是能直接看出来的差别。
const locale = zhCn
</script>

<template>
  <el-config-provider :locale="locale">
    <router-view />
  </el-config-provider>
</template>
