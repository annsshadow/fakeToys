#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=lyp-gp-t42
sp=$(echo 'KCgpPT57Y29uc3QgY3M9Wy4uLmRvY3VtZW50LnF1ZXJ5U2VsZWN0b3JBbGwoJy5zcGVlZC1jaGlwJyldO2NvbnN0IGU9Y3MuZmluZCh4PT54LmlubmVyVGV4dC5pbmNsdWRlcygnOCcpKTtpZighZSlyZXR1cm4ge2ZvdW5kOmZhbHNlLHRleHRzOmNzLm1hcCh4PT54LmlubmVyVGV4dCl9O2Uuc2Nyb2xsSW50b1ZpZXcoe2Jsb2NrOidjZW50ZXInfSk7Y29uc3Qgcj1lLmdldEJvdW5kaW5nQ2xpZW50UmVjdCgpO3JldHVybiB7Zm91bmQ6dHJ1ZSx4Ok1hdGgucm91bmQoci54K3Iud2lkdGgvMikseTpNYXRoLnJvdW5kKHIueStyLmhlaWdodC8yKX19KSgp')
sp="$(agent-browser eval -b "$sp" 2>/dev/null)"
echo "SPEED_PROBE=$sp"
sx=$(printf '%s' "$sp" | sed -n 's/.*"x": \([0-9-]*\).*/\1/p')
sy=$(printf '%s' "$sp" | sed -n 's/.*"y": \([0-9-]*\).*/\1/p')
if [ -n "$sx" ] && [ -n "$sy" ]; then
  agent-browser mouse move "$sx" "$sy" >/dev/null
  sleep 0.2
  agent-browser mouse down >/dev/null
  sleep 0.2
  agent-browser mouse up >/dev/null
  sleep 0.5
fi
for i in $(seq 1 150); do
  st='KCgpPT57Y29uc3QgcmVzPSEhZG9jdW1lbnQucXVlcnlTZWxlY3RvcignLnJlc3VsdC10aXRsZScpO2NvbnN0IGNhcmQ9ZG9jdW1lbnQucXVlcnlTZWxlY3RvcignLmhhbmQtY2FyZCcpO2NvbnN0IHNraXA9Wy4uLmRvY3VtZW50LnF1ZXJ5U2VsZWN0b3JBbGwoJy5zaGVldCAuYnRuJyldLmZpbmQoeD0+eC5pbm5lclRleHQuaW5jbHVkZXMoJ+S4i+S4gOazoicpKTtsZXQgc3g9MCxzeT0wLHR4PTAsdHk9MDtpZihjYXJkKXtjb25zdCByPWNhcmQuZ2V0Qm91bmRpbmdDbGllbnRSZWN0KCk7dHg9TWF0aC5yb3VuZChyLngrci53aWR0aC8yKTt0eT1NYXRoLnJvdW5kKHIueStyLmhlaWdodC8yKX1pZihza2lwKXtjb25zdCByPXNraXAuZ2V0Qm91bmRpbmdDbGllbnRSZWN0KCk7c3g9TWF0aC5yb3VuZChyLngrci53aWR0aC8yKTtzeT1NYXRoLnJvdW5kKHIueStyLmhlaWdodC8yKX1jb25zdCBodWQ9ZG9jdW1lbnQuYm9keS5pbm5lclRleHQuc2xpY2UoMCwyNjApO3JldHVybiB7cmVzOnJlcyxjYXJkczohIWNhcmQsc3g6c3gsc3k6c3ksdHg6dHgsdHk6dHksaHVkOmh1ZH19KSgp'
  st="$(agent-browser eval -b "$st" 2>/dev/null)"
  case "$st" in
    *'"res": true'*) echo "FINAL_STATE=$st"; exit 0 ;;
  esac
  case "$st" in
    *'"cards": true'*)
      bx=$(printf '%s' "$st" | sed -n 's/.*"sx": \([0-9-]*\).*/\1/p')
      by=$(printf '%s' "$st" | sed -n 's/.*"sy": \([0-9-]*\).*/\1/p')
      if [ -n "$bx" ] && [ -n "$by" ]; then
        agent-browser mouse move "$bx" "$by" >/dev/null
        sleep 0.2
        agent-browser mouse down >/dev/null
        sleep 0.2
        agent-browser mouse up >/dev/null
        sleep 0.5
        echo "SKIP_CARD at $bx,$by"
      fi
      ;;
  esac
  if [ $((i % 15)) -eq 0 ]; then echo "POLL $i: $st"; fi
  sleep 1
done
echo 'TIMEOUT'
exit 1
