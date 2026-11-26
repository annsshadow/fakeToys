#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
curl -s --max-time 25 "https://$DOMAIN/laiyipao-api/api/v1/config" -o docs/laiyipao/audits/round3-postdeploy/data/config-env.json
echo "CONFIG_BYTES=$(wc -c < docs/laiyipao/audits/round3-postdeploy/data/config-env.json)"
python3 -c "import json; d=json.load(open('docs/laiyipao/audits/round3-postdeploy/data/config-env.json',encoding='utf-8')); sk=d.get('skills',[]); print('SKILLS_COUNT',len(sk)); print('SKILL1_BASE_DAMAGE', [ (s.get('id'),s.get('base_damage'),s.get('name')) for s in sk if s.get('id')==1 ]); print('LEVELS',len(d.get('levels',[])),'RECIPES',len(d.get('recipes',[])))"
