#!/bin/bash
D=docs/laiyipao/audits/round40-wiki-rating-weights/data
agent-browser --session r40pub close 2>&1 | head -n 2
agent-browser --session r40pub open "http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating" > "$D/open.txt" 2>&1
echo OPEN_RC=$?
sleep 6
agent-browser --session r40pub --json eval "(() => ({url: location.href, title: document.title, hash: location.hash, text: document.body.innerText.slice(0,600), hasRating: !!document.querySelector('.r40-rating-mechanics')}))()" > "$D/public-check.json" 2>&1
echo EVAL_RC=$?
echo PROBE_OPEN_DONE
