(async () => {
  const p='http://127.0.0.1:8080/api/v1/me/advice/action';
  const tk=localStorage.getItem('lyp_token');
  async function go(q){const r=await fetch(p+'?'+q,{headers:{Authorization:'Bearer '+tk}});const j=await r.json();return {status:r.status,expected_reaction:j.expected_reaction,cand:j.candidate_id,empty:j.empty_reason&&j.empty_reason.code}}
  const noRx=await go('element=fire&source=reactions');
  const withRx=await go('element=fire&source=reactions&reaction=steam_burst');
  return JSON.stringify({noRx,withRx});
})()