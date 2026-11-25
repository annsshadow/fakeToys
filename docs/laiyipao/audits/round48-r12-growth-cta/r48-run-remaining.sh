#!/usr/bin/env bash
set -uo pipefail
A="docs/laiyipao/audits/round48-r12-growth-cta"
timeout 90 bash "$A/r48-one-scene.sh" equipment me-equipment.json "http://127.0.0.1:5201/#/pages/equipment/equipment" ".r3-filter .r3-chip:nth-child(2)"
echo EQ_RC=$?
timeout 90 bash "$A/r48-one-scene.sh" reaction me-reaction.json "http://127.0.0.1:5201/#/pages/bag/bag" ""
echo RE_RC=$?
echo R48_TWO_SCENES_DONE=YES
