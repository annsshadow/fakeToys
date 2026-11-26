#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D="docs/laiyipao/audits/round36-wiki-composite-skills"
BAD=$(grep -rInE 'Bearer[[:space:]]+eyJ|eyJ[A-Za-z0-9_-]{10,}|(admin|root|guest)[0-9]{4,}|[A-Za-z]:[/\\](Users|WORKSPACE|home)|[/]home[/][a-z]+/' "$D" --include='*.sh' --include='*.js' --include='*.py' 2>/dev/null | grep -v '/data/' || true)
if [ -n "$BAD" ]; then echo BAD_FOUND:; echo "$BAD"; else echo BAD_COUNT=0; fi
SYNTAX=0; for f in "$D"/*.sh; do bash -n "$f" || SYNTAX=1; done; echo SYNTAX_RC=$SYNTAX
git check-ignore -v "$D/data/config.json" "$D/data/admin-auth-init.js" "$D/admin-auth-init.js" 2>&1 || echo NO_IGNORE
if [ -f "$D/data/admin-auth-init.js" ]; then echo AUTH_BEFORE=YES; else echo AUTH_BEFORE=NO; fi
