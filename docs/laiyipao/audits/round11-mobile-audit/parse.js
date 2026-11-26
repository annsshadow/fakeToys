var fs=require('fs');
var files=['docs/laiyipao/audits/round11-mobile-audit/data/batch1.log','docs/laiyipao/audits/round11-mobile-audit/data/batch2.log'];
var issues=[];var rows=[];
files.forEach(function(f){
  fs.readFileSync(f,'utf8').split(String.fromCharCode(10)).forEach(function(line){
    if(line.indexOf('url')<0||line.indexOf('{')<0||line.indexOf('vw')<0)return;
    var j=null;
    try{j=JSON.parse(JSON.parse(line))}catch(e){
      try{j=JSON.parse(line)}catch(e2){return}
    }
    if(!j||!j.url)return;
    var m=j.url.match(/pages\/([^\/]+)\//);var pg=m?m[1]:'?';
    var issue=(j.vertical.length>0?'VERT ':'')+(j.hOverflow>0?'HOV ':'')+(j.outView.length>0?'OUT ':'')+(j.probe.pe>0?'PE':'')+(j.probe.ce>0?'CE':'');
    console.log([pg,'vw='+j.vw,'hOv='+j.hOverflow,'vert='+j.vertical.length,'out='+j.outView.length,'fixed='+j.fixedEls.length,'empty='+j.empty,'p/c/a/f='+[j.probe.pe,j.probe.ce,j.probe.af,j.probe.fr].join('/'),'txt='+j.textLen].join(' | ')+(issue?('  <<< '+issue):''));
    if(j.vertical.length)console.log('   VERT='+JSON.stringify(j.vertical).slice(0,220));
    if(j.outView.length)console.log('   OUT='+JSON.stringify(j.outView).slice(0,260));
    if(j.hOverflow>0)console.log('   HOV_EL='+JSON.stringify(j.overflowEls).slice(0,260));
    if(issue)issues.push(pg+':'+issue);
  });
});
console.log('=== ISSUE_PAGES='+issues.length+' ===');
console.log(issues.join(' ; '));