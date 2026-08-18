-- Component assemble control configuration table.
-- Stores the single-row control configuration updated via
-- /jaxrs/component_assemble_control/update/control/config and mirrored by
-- get_control_config. Mirrors the other *assemble_control_config tables.
CREATE TABLE IF NOT EXISTS "x_component_assemble_control_config" (
    "id" TEXT PRIMARY KEY,
    "enabled" BOOLEAN,
    "max_component_count" BIGINT,
    "allow_custom_components" BOOLEAN,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);
