(function(){
  // 合成config缓存(零真值): 2技能+1配方+1装备+1宝石, 使bag页面渲染link入口
  var sk=function(id,name,tier){return {id:id,code:'Q'+id,name:name,family:'焰',element:'fire',kind:'active',descr:'合成技能',base_damage:100,heat_cost:10,cooldown_ms:1000,pierce:0,aoe_radius:0,apply_element:'',apply_stacks:0,projectile_speed:1000,chain:0,unlock_level:1,tier:tier||1}};
  var cfg={version:'r99-qa-v1',require_login:false,server_time:new Date().toISOString(),
    levels:[],enemies:[],skills:[sk(1,'焰击'),sk(2,'冰刺')],composite_skills:[],
    recipes:[{output:9001,a:1,b:2,out_tier:2}],
    equipment:[{id:1,name:'测试护甲',slot:'armor',defense:10,element:'fire'}],
    gems:[{id:1,name:'测试宝石',slot:'gem',element:'fire',quality:1,affixes:[]}],
    gem_qualities:[{name:'普通',affix_count:1,mult:1}],skins:[],mastery_families:[],reactions:[],chapters:[],
    drop_rates:{},rating_weights:{element_coverage:20,reaction_coverage:20,mastery_done:20,equipment_synergy:20,mechanic_depth:20},score_rules:{},skill_rules:{coef_permille:100,max_level:5},banners:[],announcements:[]};
  localStorage.setItem('lyp_config_cache_v1',JSON.stringify({version:'r99-qa-v1',config:cfg}));
  // 占位令牌(零真值), 使uni.request带Bearer但api无后端请求仍会fail(用XHR hook拦截)
  localStorage.setItem('lyp_access_token','r99-qa-placeholder-token');
  window.__qaHits=[];
  var OP=XMLHttpRequest.prototype.open,SP=XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.open=function(m,u){this.__m=m;this.__u=u;return OP.apply(this,arguments)};
  XMLHttpRequest.prototype.send=function(body){
    var u=this.__u||'',m=this.__m||'GET';
    if(window.__qaHits)window.__qaHits.push(m+' '+u);
    var self=this,resp=null;
    if(u.indexOf('/config')>=0){resp=cfg}
    else if(u.indexOf('/me')>=0&&u.indexOf('/me/')<0){resp={id:10,nickname:'QA玩家',userId:10,is_guest:true,gold:10000,stars:0,build_rating:{total:30,element_coverage:0,reaction_coverage:0,mastery_done:0,equipment_synergy:0,mechanic_depth:0,weaknesses:[]}}}
    else if(u.indexOf('/me/nickname')>=0){resp={ok:true}}
    else if(u.indexOf('/auth/')>=0){resp={access_token:'r99-qa-placeholder-token',refresh_token:'',expires_at:''}}
    else if(u.indexOf('/me/battles/')>=0){resp={id:9001,level_id:9,result:'win',stars:3,score:1200,kills:30,leaked:0,hp_left:80,wave_reached:10,duration_ms:60000,shots:100,hits:80,reactions:5,heat_max:60,elements_used:{fire:10},reactions_used:{overload:3},terrain_used:['t1'],loot:{coin:100},replay_hash:'QA_HASH_9001',created_at:'2026-11-27T00:00:00Z'}}
    else if(u.indexOf('/me/battles')>=0){resp={items:[{id:9001,level_id:9,result:'win',stars:3,score:1200,kills:30,leaked:0,replay_hash:'QA_HASH_9001',created_at:'2026-11-27T00:00:00Z'}],total:1,next_cursor:0}}
    else if(u.indexOf('/announcements')>=0){resp={items:[]}}
    else if(u.indexOf('/battles')>=0){resp={items:[],total:0}}
    else if(u.indexOf('/presets')>=0){resp={items:[],total:0}}
    else if(u.indexOf('/equipment')>=0){resp={items:[],total:0}}
    else if(u.indexOf('/gems')>=0){resp={items:[],total:0}}
    else if(u.indexOf('/skills')>=0){resp={items:[],total:0}}
    else if(u.indexOf('/redeem')>=0){resp={error:{code:'ok',message:'QA'}}}
    else {resp={ok:true}};
    setTimeout(function(){
      Object.defineProperty(self,'readyState',{value:4,configurable:true});
      Object.defineProperty(self,'status',{value:200,configurable:true});
      Object.defineProperty(self,'responseText',{value:JSON.stringify(resp),configurable:true});
      Object.defineProperty(self,'response',{value:resp,configurable:true});
      if(self.onreadystatechange)try{self.onreadystatechange()}catch(e){}
      if(self.onload)try{self.onload()}catch(e){}
    },50);
    return SP.call(this,body);
  };
  // 同时patch fetch覆盖bootstrap通道(index页request:fail根因)
  var OF=window.fetch;
  window.fetch=function(input,init){
    var u=(typeof input==='string')?input:(input&&input.url)||'';
    var m=(init&&init.method)||'GET';
    if(window.__qaHits)window.__qaHits.push('F:'+m+' '+u);
    var resp=null;
    if(u.indexOf('/config')>=0){resp=cfg}
    else if(u.indexOf('/me/battles/')>=0){resp={id:9001,level_id:9,result:'win',stars:3,score:1200,kills:30,leaked:0,hp_left:80,wave_reached:10,duration_ms:60000,shots:100,hits:80,reactions:5,heat_max:60,elements_used:{fire:10},reactions_used:{overload:3},terrain_used:['t1'],loot:{coin:100},replay_hash:'QA_HASH_9001',created_at:'2026-11-27T00:00:00Z'}}
    else if(u.indexOf('/me/battles')>=0){resp={items:[{id:9001,level_id:9,result:'win',stars:3,score:1200,kills:30,leaked:0,replay_hash:'QA_HASH_9001',created_at:'2026-11-27T00:00:00Z'}],total:1,next_cursor:0}}
    else if(u.indexOf('/announcements')>=0){resp={items:[]}}
    else if(u.indexOf('/me')>=0&&u.indexOf('/me/')<0){resp={id:10,user_id:10,nickname:'QA玩家',userId:10,is_guest:true,gold:10000,stars:0,power:100,build_rating:{total:30,element_coverage:0,reaction_coverage:0,mastery_done:0,equipment_synergy:0,mechanic_depth:0,weaknesses:[]}}}
    else if(u.indexOf('/auth/')>=0){resp={access_token:'r99-qa-placeholder-token',refresh_token:'',expires_at:''}}
    else if(u.indexOf('/wallet')>=0){resp={coin:10000,gem:100,energy:50,keys:5,tokens:{gem_wash_token:3}}}
    else {resp={ok:true}};
    return Promise.resolve({ok:true,status:200,json:function(){return Promise.resolve(resp)},text:function(){return Promise.resolve(JSON.stringify(resp))}});
  };
  return {injected:true,cfgVersion:cfg.version,fetchPatched:true};
})()
