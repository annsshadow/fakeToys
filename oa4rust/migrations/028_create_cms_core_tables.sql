-- 028: create x_cms_category and x_cms_article tables for cms_core_entity.
-- The cms_core_entity SeaORM entities map to table_name "x_cms_category" and
-- "x_cms_article", but migration 023 only created x_cms_categoryinfo (for the
-- separate cms_assemble_control module) and never created x_cms_article.
-- As a result /jaxrs/cms/category/list and /jaxrs/cms/article/list 500'd on a
-- missing relation. Create both tables here to match the entity column set.

CREATE TABLE IF NOT EXISTS "x_cms_category" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" CHARACTER VARYING(255) NOT NULL,
    "parent_id" CHARACTER VARYING(255),
    "sort_order" INTEGER NOT NULL DEFAULT 0,
    "status" CHARACTER VARYING(50) NOT NULL DEFAULT 'enabled',
    "create_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_cms_category_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_cms_article" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "category_id" CHARACTER VARYING(255) NOT NULL,
    "title" CHARACTER VARYING(255) NOT NULL,
    "content" TEXT,
    "author_id" CHARACTER VARYING(255) NOT NULL,
    "status" CHARACTER VARYING(50) NOT NULL DEFAULT 'published',
    "publish_time" TIMESTAMP,
    "create_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_cms_article_pkey" PRIMARY KEY ("id")
);
