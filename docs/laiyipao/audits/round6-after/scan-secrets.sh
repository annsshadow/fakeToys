#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DIR="docs/laiyipao/audits/round6-after"
leak=0
if [ -f "$KEYS" ]; then
  while IFS="=" read -r k v; do
    [ -z "$v" ] && continue
    case "$k" in DOMAIN|ADMIN_PASS|DB_DSN|JWT_SECRET)
      n=$(grep -rF -- "$v" "$DIR" 2>/dev/null | wc -l)
      if [ "$n" -gt 0 ]; then echo "LEAK:$k:$n"; leak=1; fi
      ;;
    esac
  done < "$KEYS"
fi
[ "$leak" -eq 0 ] && echo NO_LEAK_ALL
echo SCAN_DONE