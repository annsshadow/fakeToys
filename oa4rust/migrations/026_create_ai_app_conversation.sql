-- 026: create AI core entity tables referenced by ai_core_entity SeaORM entities.
-- These tables were never created by any prior migration, causing the
-- /jaxrs/ai/core/entity/app/list and /conversation/list handlers to 500.

CREATE TABLE IF NOT EXISTS "x_ai_app" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" CHARACTER VARYING(255) NOT NULL,
    "description" TEXT,
    "status" CHARACTER VARYING(255) NOT NULL DEFAULT '',
    "create_time" TIMESTAMP WITHOUT TIME ZONE,
    CONSTRAINT "x_ai_app_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_ai_conversation" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "title" CHARACTER VARYING(255) NOT NULL,
    "user_id" CHARACTER VARYING(255) NOT NULL,
    "create_time" TIMESTAMP WITHOUT TIME ZONE,
    CONSTRAINT "x_ai_conversation_pkey" PRIMARY KEY ("id")
);
