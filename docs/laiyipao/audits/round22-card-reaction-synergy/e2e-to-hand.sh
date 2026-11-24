#!/bin/bash
AB=agent-browser; S=r22-e2e; D=docs/laiyipao/audits/round22-card-reaction-synergy
for i in $(seq 1 60); do
  R=$($AB eval $S "document.querySelectorAll('.hand-card').length" 2>/dev/null | tr -d '\r\n' | tail -1)
  case "$R" in *1*) echo "HAND_AT=$i"; break;; esac
  sleep 1
done
B64=$(node -e "process.stdout.write(Buffer.from(require('fs').readFileSync('"$D"/probe.js')).toString('base64'))")
$AB eval $S -b "$B64" > "$D/data/probe-user10.json" 2>&1
echo PROBE_DONE; head -c 400 "$D/data/probe-user10.json"