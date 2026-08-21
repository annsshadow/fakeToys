# o2server

## Responsibility

系统初始化模块，负责系统初始化检查、密钥设置和系统初始状态管理。

## Core Classes and Interfaces

- com.x.program.init.ApplicationServletContextListener
- com.x.program.init.ExceptionMissionExecute
- com.x.program.init.MissionExternalDataSources
- com.x.program.init.MissionRestore
- com.x.program.init.Missions
- com.x.program.init.MissionSetSecret
- com.x.program.init.ThisApplication
- com.x.program.init.jaxrs.ActionApplication
- com.x.program.init.jaxrs.externaldatasources.ActionCheck
- com.x.program.init.jaxrs.externaldatasources.ActionList

## Key Flows

- 初始化判定：`GET .../secret/check` initialized = auth_person 存在启用用户（locked=false AND deleted_at IS NULL）OR `secret_config` 存在记录
- 密钥设置：`POST .../secret/set` 校验非空且 ≤1024 字符、系统未初始化（已有启用用户则拒绝）；SecretCipher 从 SECRET_ENCRYPTION_KEY 派生 AES-128-GCM 密钥加密后 UPSERT `secret_config`（固定主键 init-secret）
- 密钥轮换/清除：更换环境变量后重跑 set 即用新密钥重写密文；`GET .../secret/set/cancel` 在无启用用户时 DELETE `secret_config` WHERE id='init-secret'
- 密文格式：base64(nonce(12B) || ciphertext+tag)，nonce 取 uuid v4 前 12 字节

## Dependencies



- x_base_core_project
- h2migrationtool

**Rust（oa4rust/crates/program_init）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、aes-gcm、sha2、base64、uuid

## REST Endpoints



- `GET /jaxrs/secret/check`
- `POST /jaxrs/secret/set`
- `GET /jaxrs/secret/set/cancel`
