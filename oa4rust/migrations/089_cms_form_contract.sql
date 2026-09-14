-- W4/W5 CMS form runtime contract support.
-- Runtime lookup handlers resolve a document's form through form_id.
ALTER TABLE "x_cms_data_document"
    ADD COLUMN IF NOT EXISTS "form_id" VARCHAR(255);

CREATE INDEX IF NOT EXISTS "idx_x_cms_data_document_form_id"
    ON "x_cms_data_document" ("form_id");
