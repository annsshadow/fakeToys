import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import prettier from 'eslint-config-prettier'

/**
 * ESLint flat config（ESLint 10 + typescript-eslint 8）
 *
 * 几点容易踩的：
 * - `eslint-plugin-react-hooks` v7 的 `configs.recommended` 是 **legacy eslintrc 格式**
 *   （`plugins` 是字符串数组），flat config 会直接报错；必须用 `configs.flat.recommended`。
 * - `eslint-plugin-react-refresh` 0.5.x **没有 `configs` 导出**，只能手工注册插件。
 * - `eslint-config-prettier` 必须放在最后，否则关不掉与 Prettier 冲突的格式规则。
 */
export default tseslint.config(
  { ignores: ['dist/**', 'coverage/**', 'node_modules/**'] },

  js.configs.recommended,
  tseslint.configs.recommended,

  {
    files: ['**/*.{ts,tsx}'],
    languageOptions: {
      ecmaVersion: 2020,
      globals: { ...globals.browser },
    },
    plugins: { 'react-hooks': reactHooks, 'react-refresh': reactRefresh },
    rules: {
      // 只启用经典的两条 hooks 规则，不用 `reactHooks.configs.flat.recommended`。
      //
      // 原因：v7 的 recommended 额外包含一批 **React Compiler 专用**规则
      // （immutability / purity / static-components / preserve-manual-memoization /
      // set-state-in-effect / refs / globals …）。本项目没有采用 React Compiler
      // （React 18，无 babel-plugin-react-compiler），这批规则报的是
      // "which prevents the compiler from …"（编译器无法优化），**不是缺陷**。
      // 实测 10 处 immutability 全是同一形态：`useEffect` 里调用了在其下方用
      // `const` 声明的函数——运行时完全正常，因为 effect 回调在挂载后才执行。
      // 等真正引入 React Compiler 时再把 recommended 整体打开。
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',
      'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],
    },
  },

  // 服务层与测试层必须零 any —— 这两层有测试兜底，且是新增代码的主要落点。
  {
    files: ['src/services/**/*.{ts,tsx}', 'src/test/**/*.{ts,tsx}'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'error',
    },
  },

  // 页面/组件层豁免 no-explicit-any（**显式豁免，不是静默关闭**）。
  //
  // 实测该层有 44 处 any，其中约 30 处是 `useState<any>` 承载 API 响应载荷 ——
  // 前端目前**没有响应类型层**，要消除就得为约 24 个端点逐一发明接口定义，
  // 在不知道后端确切响应形状的情况下很可能发明错，反而把错误假设固化下来。
  // 这是一项独立的类型层工程，已登记为规划里的 T1.15，不塞进"引入 lint"这一次改动。
  // 其余规则（hooks 规则、未使用变量等）在该层仍然全部生效。
  {
    files: ['src/pages/**/*.{ts,tsx}', 'src/components/**/*.{ts,tsx}'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
    },
  },

  // 配置文件运行在 Node 环境，不是浏览器
  {
    files: ['*.config.{js,ts}', 'eslint.config.js'],
    languageOptions: {
      globals: { ...globals.node },
    },
  },

  // 关闭所有与 Prettier 冲突的规则，必须最后一项
  prettier,
)
