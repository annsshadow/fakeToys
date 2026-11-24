#!/bin/bash
D=docs/laiyipao/audits/round28-live-reaction-attribution
echo '=== PRE CTA ==='
grep -o '\"helpVisible\":[^,}]*' "$D/data/pre-cta-probe.json" | tail -1
echo '=== POST CTA ==='
grep -o '\"helpVisible\":[^,}]*' "$D/data/post-cta-probe.json" | tail -1
echo '=== FINAL ==='
cat "$D/data/final.json"
