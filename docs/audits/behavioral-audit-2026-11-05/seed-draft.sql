INSERT INTO pp_c_draft (id, xid, xtitle, xperson, "xcreateTime")
VALUES
  ('audit-d1', 'audit-d1', 'Draft1', 'admin', '2026-11-01 10:00:00'),
  ('audit-d2', 'audit-d2', 'Draft2', 'admin', '2026-11-02 10:00:00'),
  ('audit-d3', 'audit-d3', 'Draft3', 'admin', '2026-11-03 10:00:00')
ON CONFLICT (id) DO NOTHING;
SELECT id, xid, "xcreateTime" FROM pp_c_draft ORDER BY "xcreateTime";
