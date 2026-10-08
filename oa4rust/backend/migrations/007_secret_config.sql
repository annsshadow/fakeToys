-- program_init 模块初始化密钥持久化表
--
-- 单行逻辑表：保存系统初始化时设置的 secret（AES-256-GCM 加密后的密文）。
-- "已初始化"判定 = auth_person 存在任意启用用户 OR secret_config 存在记录，
-- 见 crates/program_init/src/lib.rs 的 check()。

CREATE TABLE IF NOT EXISTS secret_config (
    id VARCHAR(40) PRIMARY KEY,
    secret_encrypted TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);