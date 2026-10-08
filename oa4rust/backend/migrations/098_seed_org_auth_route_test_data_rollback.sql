-- Rollback seed data for organization_assemble_authentication route tests.
DELETE FROM x_org_identity WHERE id = 'test-identity-id';
DELETE FROM auth_person WHERE id = 'test-person-id';
