-- plan002 U2 gap: oauth code/token persistence for organization_assemble_authentication
CREATE TABLE IF NOT EXISTS x_org_oauth_code (
    id CHARACTER VARYING(64) NOT NULL,
    code TEXT NOT NULL,
    client CHARACTER VARYING(64),
    person_id CHARACTER VARYING(64),
    scope TEXT,
    expire_time TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT x_org_oauth_code_pkey PRIMARY KEY (id)
);
