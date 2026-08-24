-- plan002 U2: processplatform_service_processing 端点闭合所需增量（对齐 Java jaxrs 契约）
--   ApplicationDictAction -> x_application_dict（数据字典 JSONB 存储）
--   DataAction            -> x_data（job/work/workcompleted 业务数据 JSONB，(scope,bundle) 唯一）
--   ReadAction            -> x_read.scope 区分运行中(work)/已完成(workcompleted)来源
--   TaskCompletedAction   -> x_task.next_task_identity（V2 nextTaskIdentity 编辑）
-- Idempotent: safe to run repeatedly.

CREATE TABLE IF NOT EXISTS "x_application_dict" (
    "id" VARCHAR(255) PRIMARY KEY,
    "name" VARCHAR(255),
    "category" VARCHAR(255),
    "data" JSONB DEFAULT '{}'::jsonb,
    "creator" VARCHAR(255),
    "create_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "update_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_data" (
    "scope" VARCHAR(50) NOT NULL,
    "bundle" VARCHAR(255) NOT NULL,
    "data" JSONB DEFAULT '{}'::jsonb,
    "create_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "update_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("scope", "bundle")
);

ALTER TABLE "x_read" ADD COLUMN IF NOT EXISTS "scope" VARCHAR(50) NOT NULL DEFAULT 'work';

ALTER TABLE "x_task" ADD COLUMN IF NOT EXISTS "next_task_identity" VARCHAR(255);
