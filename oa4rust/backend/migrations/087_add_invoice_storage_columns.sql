-- 为 x_general_invoice 补齐 StorageObject 字段，支撑
-- processplatform_assemble_surface 的 attachment/invoice 与
-- attachment/download/invoice 端点（对齐 o2server ActionGetInvoiceInfo / ActionDownloadInvoice）。
ALTER TABLE x_general_invoice
  ADD COLUMN IF NOT EXISTS xname TEXT,
  ADD COLUMN IF NOT EXISTS xstorage TEXT,
  ADD COLUMN IF NOT EXISTS xextension TEXT,
  ADD COLUMN IF NOT EXISTS xperson TEXT,
  ADD COLUMN IF NOT EXISTS xfiletype TEXT,
  ADD COLUMN IF NOT EXISTS xlength BIGINT;
