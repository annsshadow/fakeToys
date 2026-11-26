(async function(){
  var p=window.__LYP_PROBE__||{}; var recs=p.recResponses||[];
  if(!recs.length) return JSON.stringify({error:"no_rec_url_captured"});
  var sample=recs[recs.length-1].url;
  var t=localStorage.getItem("lyp_token")||"";
  var ctxs=["equipment","reactions","skills"];
  var out=[];
  for(var i=0;i<ctxs.length;i++){
    var u=sample.replace(/context=[^&]+/, "context="+ctxs[i]);
    try{ var r=await fetch(u,{headers:{Authorization:"Bearer "+t}}); var j=await r.json().catch(function(){return {}});
      out.push({context:ctxs[i],status:r.status,schema:j.schema_version,empty:(j.empty_reason&&j.empty_reason.code)||null,insights:(j.insights||[]).length});
    }catch(e){ out.push({context:ctxs[i],err:String(e)}); }
  }
  return JSON.stringify(out);
})()