DROP INDEX IF EXISTS "idx_x_cms_data_document_form_id";
ALTER TABLE "x_cms_data_document"
    DROP COLUMN IF EXISTS "form_id";
