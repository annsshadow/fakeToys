DELETE FROM x_attendance_appeal_info WHERE id='appeal-admin';
DELETE FROM x_attendance_selfholiday WHERE id='hol-admin';
DELETE FROM x_attendance_statisticshow WHERE id='stat-admin';
INSERT INTO x_attendance_appeal_info (id, person_id, appeal_status, creator, create_time) VALUES ('appeal-admin','person-admin','pending','person-admin','2026-11-01 10:00:00');
INSERT INTO x_attendance_selfholiday (id, person_id, holiday_date, reason, creator, create_time) VALUES ('hol-admin','person-admin','2026-11-02','admin leave','person-admin','2026-11-01 10:00:00');
INSERT INTO x_attendance_statisticshow (id, person_id, year, month, status, create_time) VALUES ('stat-admin','person-admin','2026','11','normal','2026-11-01 10:00:00');
SELECT 'appeal' AS t, person_id FROM x_attendance_appeal_info WHERE id='appeal-admin'
UNION ALL SELECT 'holiday', person_id FROM x_attendance_selfholiday WHERE id='hol-admin'
UNION ALL SELECT 'stat', person_id FROM x_attendance_statisticshow WHERE id='stat-admin';
