DELETE FROM x_cms_form WHERE id = 'seed-leave-form';
DELETE FROM x_cms_appinfo WHERE id = 'seed-app';
UPDATE x_process_definition
    SET process_definition = '{}'::jsonb
    WHERE id = 'seed-approval-flow';
DELETE FROM x_process_definition WHERE id = 'seed-approval-flow' AND version = 1 AND creator = 'system';
