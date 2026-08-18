DO $$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname='o2server') THEN
    CREATE ROLE o2server LOGIN PASSWORD 'password' SUPERUSER;
  END IF;
END
$$;

ALTER DATABASE oa4rust OWNER TO o2server;
