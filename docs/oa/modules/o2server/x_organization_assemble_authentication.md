# o2server

## Responsibility

组织认证模块，负责用户登录、登出、会话管理、OAuth 第三方登录及验证码功能。

## Core Classes and Interfaces

- com.x.organization.assemble.authentication.AbstractFactory
- com.x.organization.assemble.authentication.ApplicationServletContextListener
- com.x.organization.assemble.authentication.Business
- com.x.organization.assemble.authentication.ThisApplication
- com.x.organization.assemble.authentication.factory.BindFactory
- com.x.organization.assemble.authentication.factory.IdentityFactory
- com.x.organization.assemble.authentication.factory.PersonFactory
- com.x.organization.assemble.authentication.factory.RoleFactory
- com.x.organization.assemble.authentication.jaxrs.ActionApplication
- com.x.organization.assemble.authentication.jaxrs.AndFxJaxrsFilter

## Key Flows

- 登录：`POST /jaxrs/authentication`（兼容 `/login`）→ 查 `auth_person` 校验锁定状态，LDAP 认证优先、失败或未启用时回退 bcrypt/MD5/DES 密码校验并对旧哈希自动 rehash 升级 → 联查角色与身份列表后由 SessionManager 签发 UUID 会话令牌（Bearer）
- 刷新与登出：`POST /jaxrs/authentication/refresh` → 校验 header token 与 body old_token 一致并验证会话 → 签发新令牌、移除旧会话；`DELETE /jaxrs/authentication` 移除会话完成登出
- 双因素登录：`GET /jaxrs/authentication/code/credential/{credential}` 校验凭据存在（防枚举）后经 CodeStore 签发 6 位验证码（5 分钟 TTL）→ `POST /jaxrs/authentication/code` 验证 TempToken 绑定与验证码 → 通过后查 `auth_person` 签发完整会话

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express

**Rust（oa4rust/crates/auth）：**

- 内部 path 依赖：shared、ldap、captcha_store、sms
- 关键外部依赖：axum、deadpool-postgres、bcrypt（另以 rsa/sha2/hmac 支撑 SSO 加密）

## REST Endpoints



- `GET /jaxrs/organization/assemble/authentication/identity/{id}`
- `GET /jaxrs/organization/assemble/authentication/person/{id}/icon`
