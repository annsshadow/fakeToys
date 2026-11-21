#!/bin/bash
AB="agent-browser --session lyp-r5after-h5"
BASE="http://127.0.0.1:5174/#"
DIR="docs/laiyipao/audits/round5-after"
$AB open "$BASE/pages/index/index" >/dev/null 2>&1
sleep 3
$AB screenshot "$DIR/screenshots/r5-h5-index-honest.png" >/dev/null 2>&1
$AB open "$BASE/pages/reactions/reactions" >/dev/null 2>&1
sleep 3
$AB screenshot "$DIR/screenshots/r5-h5-reactions-aoe.png" >/dev/null 2>&1
echo DONE