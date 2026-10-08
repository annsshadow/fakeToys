-- plan002 U2 冲刺收尾（重试批次）：cms_assemble_control 缺口端点闭合 migration。
-- 支持 Java 对齐端点：
--   x_cms_data_document.batch_name
--     -> DELETE /document/batch/{batchName}            (DocumentAction.persist_deleteWithBatchName)
--     -> GET   /document/batch/{batchName}/status      (DocumentAction.query_checkImportStatus)
--     -> GET   /document/batch/{batchName}/mockdeletetoget
--     -> GET   /document/batch/status                  (DocumentAction.query_checkAllImportStatus)
--     -> PUT   /document/batch/data/modify             (DocumentAction.persist_batchDataModify)
-- 其余新增端点均落在既有表/列（x_cms_log / x_cms_viewrecord / x_cms_permission /
-- x_cms_data_document_field / x_cms_commend / x_cms_correlation / ext_content 等）。
--
-- 幂等（IF NOT EXISTS），可重复执行；回滚见 078_cms_u2_batch3_rollback.sql。

ALTER TABLE "x_cms_data_document"
    ADD COLUMN IF NOT EXISTS "batch_name" TEXT;

CREATE INDEX IF NOT EXISTS "idx_x_cms_data_document_batch_name"
    ON "x_cms_data_document" ("batch_name");
