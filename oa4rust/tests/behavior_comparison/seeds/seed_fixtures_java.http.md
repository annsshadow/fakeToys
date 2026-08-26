# oa4rust 行为对比 · 组织人员域共享种子（Java 侧调用序列）

- 目标容器：`oa4rust-o2server` → 宿主 `http://127.0.0.1:18080`
- 认证：`POST /x_organization_assemble_authentication/jaxrs/authentication`
  体 `{"credential":"xadmin","password":"o2oa@2022"}` → `data.token`；
  后续请求头 `x-token: <token>`、`Content-Type: application/json`。
- 标识符字面值依据：全量驱动 `tests/behavior_compare.rs` 不做占位符替换，
  `{flag}`/`{personFlag}` 等**原样进入 URL**（percent-encode 传输、服务端 decode），
  故种子实体的标识符就是这些字面字符串。
- Java flag 匹配规则（实测 v9.5.2）：
  - unit / group / role：按 **name** 命中；
  - person：按 **unique** 命中（name 不参与），故创建时必须显式传 `unique`；
  - identity：创建时 person/unit 参数按上述规则解析。
- 幂等性：创建类调用**非幂等** —— 重放时返回 HTTP 500
  `ExceptionDuplicateName`（如 "组织 {flag} 在同一层级下必须唯一"）。
  该 500 即"种子已就位"的标志，可直接忽略；如需完全重放可先执行文末清理段。

## 1. 登录取 token

```http
POST /x_organization_assemble_authentication/jaxrs/authentication
Content-Type: application/json

{"credential":"xadmin","password":"o2oa@2022"}
```

## 2. 单元 ×2（支撑 unit/list/{flag}/… 与 unitattribute/list/unit/{flag}）

```http
POST /x_organization_assemble_control/jaxrs/unit
x-token: <token>
Content-Type: application/json

{"name":"{flag}","typeList":["部门"]}
```

```http
POST /x_organization_assemble_control/jaxrs/unit
x-token: <token>
Content-Type: application/json

{"name":"{unitFlag}"}
```

## 3. 人员 ×2（必须带 unique；支撑 identity/role/group/personattribute 的 *list/person|unit*）

```http
POST /x_organization_assemble_control/jaxrs/person
x-token: <token>
Content-Type: application/json

{"name":"{personFlag}","mobile":"13900000001","unique":"{personFlag}"}
```

```http
POST /x_organization_assemble_control/jaxrs/person
x-token: <token>
Content-Type: application/json

{"name":"{flag}","mobile":"13900000002","unique":"{flag}"}
```

> 注意：若此前存在同名但无 unique 的旧记录，先 DELETE
> `/jaxrs/person/{旧id}` 再重建（本次实施时即如此修正）。

## 4. 群组 ×2（支撑 group/list/{flag}/… 与 role/list/group/{groupFlag}）

```http
POST /x_organization_assemble_control/jaxrs/group
x-token: <token>
Content-Type: application/json

{"name":"{flag}"}
```

```http
POST /x_organization_assemble_control/jaxrs/group
x-token: <token>
Content-Type: application/json

{"name":"{groupFlag}"}
```

## 5. 角色 ×1（支撑 group/list/role/{roleFlag}）

```http
POST /x_organization_assemble_control/jaxrs/role
x-token: <token>
Content-Type: application/json

{"name":"{roleFlag}"}
```

## 6. 身份 ×1（支撑 unitduty/list/identity/{identityFlag}；
   同时使 identity/list/unit/{unitFlag}、identity/list/person/{personFlag}
   在两侧各返回 array[1]，保持列表长度对称）

```http
POST /x_organization_assemble_control/jaxrs/identity
x-token: <token>
Content-Type: application/json

{"name":"{identityFlag}","person":"{personFlag}","unit":"{unitFlag}"}
```

> Rust 侧对称物为影子行：`x_org_identity`(unit_id='{unitFlag}',
> creator=person_id='{personFlag}')，见 seed_org.sql。

## 清理段（可选，完整重放前执行；`<id>` 用 GET 对应 list 接口查得）

```http
DELETE /x_organization_assemble_control/jaxrs/identity/{identityId}
DELETE /x_organization_assemble_control/jaxrs/role/{roleId}
DELETE /x_organization_assemble_control/jaxrs/group/{groupId}    ← 共 2 个
DELETE /x_organization_assemble_control/jaxrs/person/{personId}  ← 共 2 个
DELETE /x_organization_assemble_control/jaxrs/unit/{unitId}      ← 共 2 个
```

## U4 探测记录（2026-08-26）

### 已验证可用的端点

| 端点 | 状态 | 说明 |
|------|------|------|
| `POST /x_organization_assemble_authentication/jaxrs/authentication` | ✓ 200 | 认证正常 |
| `GET /x_cms_assemble_control/jaxrs/appinfo/list/all` | ✓ 200 | CMS appinfo 正常（46条） |

### 不可用端点（返回404）

以下端点在探测时返回404，可能原因：o2server 容器需要重新初始化，或 v9 路由规则变更。

- `/x_organization_assemble_control/jaxrs/unit/list/all` 及所有 org control 端点
- `/x_cms_assemble_control/jaxrs/categoryinfo/list/app/{id}`
- `/x_bbs_assemble_control/jaxrs/forum/list/all`
- `/x_meeting_assemble_control/jaxrs/building/list/all`
- `/x_processplatform_assemble_surface/jaxrs/application/list/summary`（返回405，方法不支持GET）

### 待办

- 重新初始化 o2server 容器后重跑探测矩阵
- 补充 category/forum 的 Java 侧创建路径验证
- 验证 meeting 的 flag 解析链（仅认 UUID 主键的实体标记为"不可种"）
