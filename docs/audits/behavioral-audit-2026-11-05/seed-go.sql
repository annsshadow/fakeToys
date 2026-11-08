INSERT INTO users (guest_token, nickname, avatar_url, is_guest, status, ban_reason, created_at, last_login_at)
SELECT 'audit-token-' || g, 'audit_u' || g, '', true, 0, '', NOW() - (g || ' minutes')::interval, NOW()
FROM generate_series(1,250) g;
INSERT INTO battle_records (user_id, level_id, result, stars, created_at)
SELECT 1, 1, 'win', 3, timestamptz '2026-11-05 10:00:00+08'
FROM generate_series(1,5);
SELECT count(*) AS users FROM users;
SELECT count(*) AS battles, min(created_at) AS ct FROM battle_records WHERE user_id=1;
SELECT guest_token FROM users WHERE id=1;
