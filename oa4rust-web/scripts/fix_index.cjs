const fs = require('fs');
const filePath = 'D:/WORKSPACE/fakeToys/oa4rust-web/packages/apis/src/index.ts';
let content = fs.readFileSync(filePath, 'utf8');

// Truncate at incomplete fileControlApi if present
const fcIdx = content.indexOf('\n// ─────────────────────────────────────────────────────────────\n// 文件控制 (file_assemble_control');
if (fcIdx !== -1) {
  // Find the start of this comment block
  const startOfBlock = content.lastIndexOf('\n// ─────────────────────────────────────────────────────────────\n', fcIdx);
  if (startOfBlock !== -1) {
    content = content.slice(0, startOfBlock + 1);
  } else {
    content = content.slice(0, fcIdx);
  }
}

// Also check if oa4rustApis closing brace is missing
if (!content.trimEnd().endsWith('};')) {
  const lastBrace = content.lastIndexOf('};');
  if (lastBrace !== -1) {
    content = content.slice(0, lastBrace + 2);
  }
}

const remaining = `
// ─────────────────────────────────────────────────────────────
// 工作流表面 (processplatform_assemble_surface — 963 routes)
// ─────────────────────────────────────────────────────────────
export const processplatformSurfaceApi = {
  openapi: () => api.get("/jaxrs/processplatform/assemble/surface/openapi"),
  get: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/get/\${id}\`),
  sign: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/sign/\${id}\`),
  snap: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/snap/\${id}\`),
  task: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/task/\${id}\`),
  work: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/work/\${id}\`),
  draft: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/draft/\${id}\`),
  route: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/route/\${id}\`),
  form: (flag: string) => api.get(\`/jaxrs/processplatform/assemble/surface/form/\${flag}\`),
  review: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/review/\${id}\`),
  preview: (id: string) => api.get(\`/jaxrs/processplatform/assemble/surface/preview/\${id}\`),
  handover: (id: string) => api.post(\`/jaxrs/processplatform/assemble/surface/handover/\${id}\`, null),
  create: (data: unknown) => api.post("/jaxrs/processplatform/assemble/surface/create", data),
  save: (id: string, data: unknown) => api.put(\`/jaxrs/processplatform/assemble/surface/save/\${id}\`, data),
  delete: (id: string) => api.delete(\`/jaxrs/processplatform/assemble/surface/\${id}\`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/processplatform/assemble/surface" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// CMS 内容管理 (cms_assemble_control — 405 routes)
// ─────────────────────────────────────────────────────────────
export const cmsApi = {
  log: (id: string) => api.get(\`/jaxrs/cms/log/\${id}\`),
  file: (id: string) => api.get(\`/jaxrs/cms/file/\${id}\`),
  form: (id: string) => api.get(\`/jaxrs/cms/form/\${id}\`),
  view: (id: string) => api.get(\`/jaxrs/cms/view/\${id}\`),
  script: (id: string) => api.get(\`/jaxrs/cms/script/\${id}\`),
  outputList: () => api.get("/jaxrs/cms/output/list"),
  create: (type: string, data: unknown) => api.post(\`/jaxrs/cms/\${type}\`, data),
  update: (type: string, id: string, data: unknown) => api.put(\`/jaxrs/cms/\${type}/\${id}\`, data),
  delete: (type: string, id: string) => api.delete(\`/jaxrs/cms/\${type}/\${id}\`),
  comment: (id: string) => api.get(\`/jaxrs/cms/comment/\${id}\`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/cms" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 组织控制 (organization_assemble_control — 235 routes)
// ─────────────────────────────────────────────────────────────
export const organizationControlApi = {
  identity: (flag: string) => api.get(\`/jaxrs/organization/assemble/control/identity/\${flag}\`),
  role: (flag: string) => api.get(\`/jaxrs/organization/assemble/control/role/\${flag}\`),
  unit: (flag: string) => api.get(\`/jaxrs/organization/assemble/control/unit/\${flag}\`),
  group: (flag: string) => api.get(\`/jaxrs/organization/assemble/control/group/\${flag}\`),
  person: (flag: string) => api.get(\`/jaxrs/organization/assemble/control/person/\${flag}\`),
  getRoot: () => api.get("/jaxrs/organization/assemble/control/unit/get/root"),
  listTop: () => api.get("/jaxrs/organization/assemble/control/unit/list/top"),
  roleListLike: () => api.get("/jaxrs/organization/assemble/control/role/list/like"),
  unitListLike: () => api.get("/jaxrs/organization/assemble/control/unit/list/like"),
  groupListLike: () => api.get("/jaxrs/organization/assemble/control/group/list/like"),
  identityListLike: () => api.get("/jaxrs/organization/assemble/control/identity/list/like"),
  create: (type: string, data: unknown) => api.post(\`/jaxrs/organization/assemble/control/\${type}\`, data),
  update: (type: string, flag: string, data: unknown) => api.put(\`/jaxrs/organization/assemble/control/\${type}/\${flag}\`, data),
  delete: (type: string, flag: string) => api.delete(\`/jaxrs/organization/assemble/control/\${type}/\${flag}\`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/organization/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 考勤控制 (attendance_assemble_control — 228 routes)
// ─────────────────────────────────────────────────────────────
export const attendanceControlApi = {
  ruleList: () => api.get("/jaxrs/attendance/assemble/control/rule/list"),
  v2Config: () => api.get("/jaxrs/attendance/assemble/control/v2/config"),
  uuid: () => api.get("/jaxrs/attendance/assemble/control/uuid/random"),
  statistic: () => api.post("/jaxrs/attendance/assemble/control/statistic/do", null),
  v2Group: (id: string) => api.get(\`/jaxrs/attendance/assemble/control/v2/group/\${id}\`),
  v2MyVersion: () => api.get("/jaxrs/attendance/assemble/control/v2/my/version"),
  v2Shift: (id: string) => api.get(\`/jaxrs/attendance/assemble/control/v2/shift/\${id}\`),
  qywxSyncList: () => api.get("/jaxrs/attendance/assemble/control/qywx/sync/list"),
  workplace: (data: unknown) => api.post("/jaxrs/attendance/assemble/control/workplace", data),
  toggleRule: (id: string) => api.put(\`/jaxrs/attendance/assemble/control/rule/\${id}/toggle\`, null),
  auditAppeal: (id: string, data: unknown) => api.put(\`/jaxrs/attendance/assemble/control/attendanceappealInfo/audit/\${id}\`, data),
  checkDetail: (params: unknown) => api.post("/jaxrs/attendance/assemble/control/attendancedetail/filter/list", params),
  deleteWorkplace: (id: string) => api.delete(\`/jaxrs/attendance/assemble/control/workplace/\${id}\`),
  deleteAdmin: (id: string) => api.delete(\`/jaxrs/attendance/assemble/control/attendanceadmin/\${id}\`),
  deleteDetail: (id: string) => api.delete(\`/jaxrs/attendance/assemble/control/attendancedetail/\${id}\`),
  dingdingSync: () => api.delete("/jaxrs/attendance/assemble/control/dingding/all"),
  qywxSync: () => api.delete("/jaxrs/attendance/assemble/control/qywx/all"),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/attendance/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 文件控制 (file_assemble_control — 182 routes)
// ─────────────────────────────────────────────────────────────
export const fileControlApi = {
  shareList: () => api.get("/jaxrs/file/assemble/control/share/list"),
  share: (id: string) => api.get(\`/jaxrs/file/assemble/control/share/\${id}\`),
  top: () => api.get("/jaxrs/file/assemble/control/complex/top"),
  editorList: () => api.get("/jaxrs/file/assemble/control/editor/list"),
  folder: (id: string) => api.get(\`/jaxrs/file/assemble/control/folder/\${id}\`),
  fileId: (id: string) => api.get(\`/jaxrs/file/assemble/control/file/id/\${id}\`),
  folder2: (id: string) => api.get(\`/jaxrs/file/assemble/control/folder2/\${id}\`),
  recycleList: () => api.get("/jaxrs/file/assemble/control/recycle/list"),
  shareCreate: (data: unknown) => api.post("/jaxrs/file/assemble/control/share", data),
  config: (data: unknown) => api.post("/jaxrs/file/assemble/control/config", data),
  folderCreate: (data: unknown) => api.post("/jaxrs/file/assemble/control/folder", data),
  folder2Create: (data: unknown) => api.post("/jaxrs/file/assemble/control/folder2", data),
  folderUpdate: (id: string, data: unknown) => api.put(\`/jaxrs/file/assemble/control/folder/\${id}\`, data),
  folder2Update: (id: string, data: unknown) => api.put(\`/jaxrs/file/assemble/control/folder2/\${id}\`, data),
  emptyRecycle: () => api.delete("/jaxrs/file/assemble/control/recycle/empty"),
  deleteShare: (id: string) => api.delete(\`/jaxrs/file/assemble/control/share/\${id}\`),
  deleteFolder: (id: string) => api.delete(\`/jaxrs/file/assemble/control/folder/\${id}\`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/file/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 会议控制 (meeting_assemble_control — 109 routes)
// ─────────────────────────────────────────────────────────────
export const meetingControlApi = {
  roomList: () => api.get("/jaxrs/meeting/assemble/control/room/list"),
  room: (id: string) => api.get(\`/jaxrs/meeting/assemble/control/room/\${id}\`),
  openmeeting: () => api.get("/jaxrs/meeting/assemble/control/openmeeting"),
  meeting: (id: string) => api.get(\`/jaxrs/meeting/assemble/control/meeting/\${id}\`),
  buildingList: () => api.get("/jaxrs/meeting/assemble/control/building/list"),
  building: (id: string) => api.get(\`/jaxrs/meeting/assemble/control/building/\${id}\`),
  attachment: (id: string) => api.get(\`/jaxrs/meeting/assemble/control/attachment/\${id}\`),
  roomCreate: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/room", data),
  config: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/config", data),
  create: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/create", data),
  meetingCreate: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/meeting", data),
  buildingCreate: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/building", data),
  roomUpdate: (id: string, data: unknown) => api.put(\`/jaxrs/meeting/assemble/control/room/\${id}\`, data),
  meetingUpdate: (id: string, data: unknown) => api.put(\`/jaxrs/meeting/assemble/control/meeting/\${id}\`, data),
  buildingUpdate: (id: string, data: unknown) => api.put(\`/jaxrs/meeting/assemble/control/building/\${id}\`, data),
  roomDelete: (id: string) => api.delete(\`/jaxrs/meeting/assemble/control/room/\${id}\`),
  meetingDelete: (id: string) => api.delete(\`/jaxrs/meeting/assemble/control/meeting/\${id}\`),
  buildingDelete: (id: string) => api.delete(\`/jaxrs/meeting/assemble/control/building/\${id}\`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/meeting/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 门户表面 (portal_assemble_surface — 72 routes)
// ─────────────────────────────────────────────────────────────
export const portalSurfaceApi = {
  list: () => api.get("/jaxrs/portal/assemble/surface/list"),
  preview: (id: string) => api.get(\`/jaxrs/portal/assemble/surface/\${id}/preview\`),
  get: (id: string) => api.get(\`/jaxrs/portal/assemble/surface/get/\${id}\`),
  getLayout: () => api.get("/jaxrs/portal/assemble/surface/get/layout"),
  file: (flag: string) => api.get(\`/jaxrs/portal/assemble/surface/file/\${flag}\`),
  script: (id: string) => api.get(\`/jaxrs/portal/assemble/surface/script/\${id}\`),
  widget: (id: string) => api.get(\`/jaxrs/portal/assemble/surface/widget/\${id}\`),
  publish: (data: unknown) => api.post("/jaxrs/portal/assemble/surface/publish", data),
  create: (data: unknown) => api.post("/jaxrs/portal/assemble/surface/create", data),
  saveLayout: (data: unknown) => api.put("/jaxrs/portal/assemble/surface/save/layout", data),
  deleteLayout: (data: unknown) => api.delete("/jaxrs/portal/assemble/surface/delete/layout", data),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/portal/assemble/surface" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 通用控制 (general_assemble_control — 94 routes)
// ─────────────────────────────────────────────────────────────
export const generalControlApi = {
  status: () => api.get("/jaxrs/general/assemble/control/status"),
  areaList: () => api.get("/jaxrs/general/assemble/control/area/list"),
  area: (id: string) => api.get(\`/jaxrs/general/assemble/control/area/\${id}\`),
  qrCodeList: () => api.get("/jaxrs/general/assemble/control/qrcode/list"),
  qrCode: (id: string) => api.get(\`/jaxrs/general/assemble/control/qrcode/\${id}\`),
  attendScopeList: () => api.get("/jaxrs/general/assemble/control/attendscope/list"),
  office: (data: unknown) => api.post("/jaxrs/general/assemble/control/office", data),
  qrCodeCreate: (data: unknown) => api.post("/jaxrs/general/assemble/control/qrcode", data),
  areaCreate: (data: unknown) => api.post("/jaxrs/general/assemble/control/area/create", data),
  invoiceCreate: (data: unknown) => api.post("/jaxrs/general/assemble/control/invoice/create", data),
  statusUpdate: (data: unknown) => api.put("/jaxrs/general/assemble/control/status/update", data),
  areaUpdate: (id: string, data: unknown) => api.put(\`/jaxrs/general/assemble/control/area/update/\${id}\`, data),
  areaDelete: (id: string) => api.delete(\`/jaxrs/general/assemble/control/area/delete/\${id}\`),
  qrCodeDelete: (id: string) => api.delete(\`/jaxrs/general/assemble/control/qrcode/delete/\${id}\`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/general/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 消息通信 (message_assemble_communicate — 78 routes)
// ─────────────────────────────────────────────────────────────
export const messageCommunicateApi = {
  connector: () => api.get("/jaxrs/message/assemble/communicate/connector"),
  mass: (id: string) => api.get(\`/jaxrs/message/assemble/communicate/mass/\${id}\`),
  wsList: () => api.get("/jaxrs/message/assemble/communicate/ws/list/person"),
  wsCount: () => api.get("/jaxrs/message/assemble/communicate/ws/count/person"),
  imManagerConfig: () => api.get("/jaxrs/message/assemble/communicate/im/manager/config"),
  receive: (consume: string) => api.get(\`/jaxrs/message/assemble/communicate/receive/\${consume}\`),
  imMsgList: () => api.get("/jaxrs/message/assemble/communicate/im/msg/list/object"),
  imMsgRevoke: (id: string) => api.get(\`/jaxrs/message/assemble/communicate/im/msg/revoke/\${id}\`),
  ws: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/ws", data),
  massCreate: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/mass", data),
  send: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/send", data),
  imMsg: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/im/msg", data),
  markRead: (id: string) => api.post(\`/jaxrs/message/assemble/communicate/mark_read/\${id}\`, null),
  imConversation: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/im/conversation", data),
  imConversationUpdate: (id: string, data: unknown) => api.put(\`/jaxrs/message/assemble/communicate/im/conversation/\${id}\`, data),
  imConversationRead: (id: string) => api.put(\`/jaxrs/message/assemble/communicate/im/conversation/\${id}/read\`, null),
  massDelete: (id: string) => api.delete(\`/jaxrs/message/assemble/communicate/mass/\${id}\`),
  imConversationGroup: (id: string) => api.delete(\`/jaxrs/message/assemble/communicate/im/conversation/\${id}/group\`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/message/assemble/communicate" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};
`;

content += remaining;
fs.writeFileSync(filePath, content);

// Count APIs
const count = (content.match(/export const \w+Api = \{/g) || []).length;
console.log('Done! Total lines:', content.split('\n').length);
console.log('API modules:', count);
