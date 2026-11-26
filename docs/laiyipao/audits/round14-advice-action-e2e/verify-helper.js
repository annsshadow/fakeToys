(async () => {
  const m = await import('/src/pages/_shared/r14AdviceAction.ts');
  const base = {schema_version:'advice-action-v1', badge:'AI', element:'fire', expected_reaction:'', based_on:['battle_records.elements_used','/config(v=x)'], kind:'skill', candidate_id:3, replaced_id:4, candidate_name:'温压弹', replaced_name:'干冰弹', params:{kind:'skill',id:3,replace:4}, preview_route:'/pages/skills/skills', empty_reason:null, evidence:[], impact:null};
  const noRx = m.normalizeAdviceAction(base);
  const withRx = m.normalizeAdviceAction(Object.assign({},base,{expected_reaction:'steam_burst'}));
  return JSON.stringify({noRx:noRx===null?null:{status:noRx.status,cand:noRx.candidateId}, withRx:withRx===null?null:{status:withRx.status,cand:withRx.candidateId}});
})()