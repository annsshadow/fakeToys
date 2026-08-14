## Qualcomm Datacenter Technologies L3 缂撳瓨鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?

璇ラ┍鍔ㄦ敮鎸?Qualcomm Datacenter Technologies Centriq SoC 涓殑 L3 缂撳瓨 PMU銆?杩欎簺 SoC 涓婄殑 L3 缂撳瓨鐢卞涓垏鐗囩粍鎴愶紝鐢辨彃妲藉唴鐨勬墍鏈夋牳蹇冨叡浜€傛瘡涓垏鐗囦綔涓?鐙珛鐨勯潪鏍?perf PMU 鏆撮湶锛岃澶囧悕涓?l3cache_<socket>_<instance>銆傜敤鎴风┖闂?璐熻矗璺ㄥ垏鐗囪仛鍚堛€?
璇ラ┍鍔ㄥ湪 sysfs 涓彁渚涘叾鍙敤浜嬩欢涓庨厤缃€夐」鐨勬弿杩帮紝瑙?/sys/bus/event_source/devices/l3cache*銆傞壌浜庤繖浜涙槸闈炴牳 PMU锛岄┍鍔ㄨ繕鏆撮湶涓€涓?"cpumask" sysfs 灞炴€э紝鍏朵腑鍖呭惈姣忎釜鎻掓Ы涓€涓?CPU 鐨勬帺鐮侊紝灏嗙敤浜庡鐞嗚鎻掓Ы涓婄殑
鎵€鏈?PMU 浜嬩欢銆?
纭欢瀹炵幇 32 浣嶄簨浠惰鏁板櫒锛屽苟閫氳繃 "event" 鏍煎紡灞炴€ф毚闇蹭竴涓墎骞崇殑 8 浣嶄簨浠剁┖闂淬€?闄や簡 32 浣嶇墿鐞嗚鏁板櫒澶栵紝椹卞姩杩橀€氳繃浣跨敤纭欢璁℃暟鍣ㄩ摼寮忚繛鎺ユ敮鎸佽櫄鎷?64 浣嶇‖浠?璁℃暟鍣ㄣ€傝鐗规€ч€氳繃 "lc"锛堥暱璁℃暟鍣級鏍煎紡鏆撮湶
```

  perf stat -e l3cache_0_0/read-miss,lc/

```
閴翠簬杩欎簺鏄潪鏍?PMU锛岄┍鍔ㄤ笉鏀寔閲囨牱锛屽洜姝?"perf record" 灏嗘棤娉曞伐浣溿€備笉鏀寔
姣忎换鍔＄殑 perf 浼氳瘽銆?