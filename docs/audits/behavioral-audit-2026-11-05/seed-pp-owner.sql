INSERT INTO x_task (id, title, work, person, task_status, create_time, creator) VALUES ('task-admin-1','Admin task 1','w-admin-1','person-admin','pending','2026-11-03 10:00:00','person-admin');
INSERT INTO x_task (id, title, work, person, task_status, create_time, creator) VALUES ('task-admin-2','Admin task 2','w-admin-2','person-admin','pending','2026-11-04 10:00:00','person-admin');
INSERT INTO x_work (id, title, process, creator, create_time, person) VALUES ('work-admin-1','Admin work 1','p1','person-admin','2026-11-03 10:00:00','person-admin');
INSERT INTO x_read (id, work_id, person, scope, read_time) VALUES ('read-admin-1','work-admin-1','person-admin','ALL','2026-11-03 10:00:00');
SELECT 'task' AS t, id, person FROM x_task WHERE person='person-admin';
