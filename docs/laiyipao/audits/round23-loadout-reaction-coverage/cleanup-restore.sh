set -e
D=docs/laiyipao/audits/round23-loadout-reaction-coverage
rm -f "$D/auth-init.js"
rm -f "$D/screenshots/r23-02-coverage-card.png"
echo "AUTH_INIT_EXISTS=$(test -f "$D/auth-init.js" && echo yes || echo no)"
echo "DUP_EXISTS=$(test -f "$D/screenshots/r23-02-coverage-card.png" && echo yes || echo no)"
echo "=== SCREENSHOT SHA256 ==="
sha256sum "$D"/screenshots/*.png
echo "=== QA RESTORE ==="
docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < "$D/data/qa-before.dump" && echo RESTORE_EXIT=0
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT count(*) FROM battle_records;" | sed 's/^/BATTLES=/'
