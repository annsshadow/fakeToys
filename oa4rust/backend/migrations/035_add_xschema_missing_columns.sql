-- migration 035: add columns referenced by handlers (incl. x_ai_model etc.)
-- after X. -> x_ schema rename; idempotent via IF NOT EXISTS.

ALTER TABLE "x_ai_clue" ADD COLUMN IF NOT EXISTS "cnt" TEXT;
ALTER TABLE "x_ai_completion" ADD COLUMN IF NOT EXISTS "clueId" TEXT;
ALTER TABLE "x_ai_completion" ADD COLUMN IF NOT EXISTS "cnt" TEXT;
ALTER TABLE "x_ai_completion" ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE "x_ai_completion" ADD COLUMN IF NOT EXISTS "generateType" TEXT;
ALTER TABLE "x_ai_completion" ADD COLUMN IF NOT EXISTS "input" TEXT;
ALTER TABLE "x_ai_completion" ADD COLUMN IF NOT EXISTS "person" TEXT;
ALTER TABLE "x_ai_file" ADD COLUMN IF NOT EXISTS "xcreator" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "apiKey" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "asDefault" BOOLEAN;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "cnt" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "completionUrl" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "desc" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "enable" BOOLEAN;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "model" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "type" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "xapikey" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "xasdefault" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "xcompletionurl" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "xdesc" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "xenable" BOOLEAN;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "xmodel" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "xname" TEXT;
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "xtype" TEXT;
ALTER TABLE "x_express_info" ADD COLUMN IF NOT EXISTS "xcompany" TEXT;
ALTER TABLE "x_msg_message" ADD COLUMN IF NOT EXISTS "xbody" TEXT;
ALTER TABLE "x_msg_message" ADD COLUMN IF NOT EXISTS "xconsumer" TEXT;
ALTER TABLE "x_msg_message" ADD COLUMN IF NOT EXISTS "xcreateTime" TEXT;
ALTER TABLE "x_msg_message" ADD COLUMN IF NOT EXISTS "xid" TEXT;
ALTER TABLE "x_msg_message" ADD COLUMN IF NOT EXISTS "xperson" TEXT;
ALTER TABLE "x_msg_message" ADD COLUMN IF NOT EXISTS "xtitle" TEXT;
ALTER TABLE "x_msg_message" ADD COLUMN IF NOT EXISTS "xtype" TEXT;
