#!/bin/bash
S="lyp-posth5b"
AB="agent-browser --session $S"
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/e2e-battle-nav.js 2>&1
