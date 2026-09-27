// 生产 H5 不需要 Vue devtools 运行时：pinia 的 devtools-api 引用链在
// rolldown 严格解析下缺传递依赖，统一指向此空实现。
export default {}
export const setupDevtoolsPlugin = (): void => {}
