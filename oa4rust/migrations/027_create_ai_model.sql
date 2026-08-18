-- 027: create x_ai_model table.
-- Migration 011 renamed X_AI_MODEL -> x_ai_model, but X_AI_MODEL was never
-- created by any migration, so the rename was a no-op and /jaxrs/ai/core/entity/model/list
-- 500'd on a missing table. Create it here to match the ai_model SeaORM entity.

CREATE TABLE IF NOT EXISTS "x_ai_model" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" CHARACTER VARYING(255) NOT NULL,
    "provider" CHARACTER VARYING(255) NOT NULL DEFAULT '',
    "enabled" BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT "x_ai_model_pkey" PRIMARY KEY ("id")
);
