#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round32-postfix-recheck
echo "=== zero-secret scan (audit scripts, excluding data/) ==="
BAD=$(grep -rInE 'Bearer eyJ|eyJ[A-Za-z0-9_-]{10,}|admin12345|D:/WORKSPACE|C:/Users|/home/[a-z]+/' "$D" --include='*.sh' --include='*.js' --include='*.py' 2>/dev/null | grep -v '/data/' | grep -v 'admin-auth-init' || true)
if [ -n "$BAD" ]; then echo "BAD_FOUND:"; echo "$BAD"; else echo BAD_COUNT=0; fi
echo "=== bash -n ==="
SYNTAX=0; for f in "$D"/*.sh; do bash -n "$f" || SYNTAX=1; done; echo SYNTAX_RC=$SYNTAX
echo "=== git check-ignore ==="
git check-ignore -v "$D/data/config.json" "$D/admin-auth-init.js" 2>&1 || echo NO_IGNORE
echo "=== auth token present ==="
if [ -f "$D/data/admin-auth-init.js" ]; then echo AUTH_BEFORE=YES; else echo AUTH_BEFORE=NO; fi
