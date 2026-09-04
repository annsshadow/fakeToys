/**
 * 全局类型定义
 * 对应 oa4rust Session + 用户信息 + 布局系统
 */

/** 当前登录用户信息（来自 /jaxrs/authentication/who） */
export interface O2User {
  /** person_unique，等价于 o2web 的 distinguishedName */
  unique: string;
  name: string;
  icon?: string;
  mobile?: string;
  email?: string;
  groups?: OrgGroup[];
  roles?: string[];
  /** 用户权限等级 */
  permissionLevel?: number;
}

export interface OrgGroup {
  id: string;
  name: string;
  parent_id?: string;
  level?: number;
}

/** 会话状态 */
export interface SessionState {
  token: string | null;
  user: O2User | null;
  loading: boolean;
  /** 系统是否处于未初始化状态（首次安装需要设置 secret） */
  systemUninitialized: boolean;
}

/** 桌面布局上下文（等价于 o2web layout.desktop） */
export interface O2Desktop {
  /** 当前会话用户 */
  session: {
    user: O2User;
    token: string;
  };
  /** WebSocket 实例 */
  socket: O2WebSocket;
  /** 当前打开的应用 ID */
  currentApp?: string;
  /** 导航到指定应用 */
  navigate: (appId: string, params?: Record<string, unknown>) => void;
}

export interface O2WebSocket {
  /** 连接状态 */
  connected: boolean;
  /** 添加 IM 消息监听 */
  addImListener: (event: string, handler: (data: unknown) => void) => void;
  /** 发送消息 */
  send: (channel: string, data: unknown) => void;
  /** 断开连接 */
  close: () => void;
}

/** 应用初始化选项 */
export interface O2AppOptions {
  /** API 基础 URL，默认 /jaxrs */
  apiBase?: string;
  /** WebSocket 基础 URL，默认 ws:// 同域 /ws */
  wsBase?: string;
  /** 默认语言 */
  locale?: string;
  /** 默认主题 */
  theme?: 'dark' | 'light';
  /** 是否在登录页拦截未认证请求 */
  authGuard?: boolean;
}
