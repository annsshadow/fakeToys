var fs=require('fs');
var t=process.env.LYP_TOKEN||'';
if(!t){console.error('NO_TOKEN');process.exit(1)}
var lines=[
'// temp capture script with real token; sanitized after collection'
'(function(){',
'  try{localStorage.setItem("lyp_token","'+t+'");localStorage.removeItem("lyp_token_exp")}catch(e){}',
'  if(window.__LYP_PROBE__)return;',
'  var p={pageErrors:[],consoleErrors:[],failedRequests:[],apiFailures:[],startedAt:Date.now()};window.__LYP_PROBE__=p;',
'  window.addEventListener("error",function(e){try{p.pageErrors.push({msg:e.message,ts:Date.now()})}catch(_){}});',
'  window.addEventListener("unhandledrejection",function(e){try{p.pageErrors.push({msg:"ur: "+(e.reason&&e.reason.message?e.reason.message:String(e.reason)),ts:Date.now()})}catch(_){}});',
'  var oe=console.error;console.error=function(){try{p.consoleErrors.push([].map.call(arguments,function(a){return(a&&a.message)?a.message:String(a)}).join(" "))}catch(_){}return oe.apply(console,arguments)};',
')();'
];
fs.writeFileSync(process.argv[2],lines.join(String.fromCharCode(10)));
console.log('GEN_OK len='+t.length);