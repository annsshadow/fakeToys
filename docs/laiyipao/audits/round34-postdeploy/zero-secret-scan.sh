#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round34-postdeploy
BAD=$(grep -rInE 'Bearer[[:space:]]+eyJ|eyJ[A-Za-z0-9_-]{10,}|[A-Za-z]:[/\\](Users|WORKSPACE|home)' "$D" --include='*.sh' --include='*.js' --include='*.py' 2>/dev/null | grep -v '/data/' | grep -v 'mint-token.sh' || true)
if [ -n "$BAD" ]; then echo BAD_FOUND:; echo "$BAD"; else echo BAD_COUNT=0; fi
SYNTAX=0; for f in "$D"/*.sh; do bash -n "$f" || SYNTAX=1; done; echo SYNTAX_RC=$SYNTAX
git check-ignore -v "$D/data/config.json" "$D/data/" || echo NO_IGNORE
git ls-files | grep -E '(^|/)(data/|admin-auth-init)' || echo NO_DATA_TRACKED
