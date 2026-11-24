#!/bin/bash
sleep 8
echo "--- hashes ---"
sha256sum laiyipao/admin/dist/assets/index-*.js laiyipao/admin/dist/assets/WikiView-*.js laiyipao/deploy/bin/lyp-server-amd64 laiyipao/miniapp/dist/build/h5/assets/index-*.js
echo "--- required strings in WikiView bundle ---"
grep -o "spread_stacks\|armor_shred_permille\|knockback\|层数传播\|削甲\|击退" laiyipao/admin/dist/assets/WikiView-*.js | sort | uniq -c
echo "--- health ---"
for u in http://127.0.0.1:8080/healthz http://127.0.0.1:8080/readyz http://127.0.0.1:5174/ http://127.0.0.1:5175/laiyipao-admin/; do
  code=$(curl -s -o /dev/null -w "%{http_code}" "$u")
  echo "$u=$code"
done
