## octeontx2 devlink 鏀寔


鏈枃妗ｆ弿杩颁簡 `octeontx2 AF銆丳F 鍜?VF` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## 鍙傛暟


`octeontx2 PF 鍜?VF` 椹卞姩瀹炵幇浜嗕互涓嬮┍鍔ㄧ壒瀹氱殑鍙傛暟銆?
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `mcam_count`
     - u16
     - runtime
     - 閫夋嫨涓烘煇涓帴鍙ｅ垎閰嶇殑鍖归厤 CAM 鏉＄洰鏁伴噺銆?       璇ユ暟閲忓悓鏍风敤浜庤鎺ュ彛鐨?ntuple 杩囨护鍣ㄣ€傜敱 PF 鍜?VF 椹卞姩鏀寔銆?
`octeontx2 AF` 椹卞姩瀹炵幇浜嗕互涓嬮┍鍔ㄧ壒瀹氱殑鍙傛暟銆?
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `dwrr_mtu`
     - u32
     - runtime
     - 鐢ㄤ簬璁剧疆纭欢鍦ㄤ紶杈撻槦鍒椾箣闂磋皟搴︽椂浣跨敤鐨勯噺瀛愶紙quantum锛夈€?       纭欢浣跨敤鍔犳潈 DWRR 绠楁硶鍦ㄦ墍鏈変紶杈撻槦鍒椾箣闂磋繘琛岃皟搴︺€?   - - `npc_mcam_high_zone_percent`
     - u8
     - runtime
     - 鐢ㄤ簬璁剧疆鐢ㄦ埛鍙湪 NPC MCAM 涓垎閰嶇殑楂樹紭鍏堢骇鍖哄煙鏉＄洰鏁伴噺锛屼粠 high銆乵id 鍜?low
       涓変釜浼樺厛绾у尯鍩熺被鍒腑鍒掑垎銆?   - - `npc_def_rule_cntr`
     - bool
     - runtime
     - 鐢ㄤ簬鍚敤鎴栫鐢?NPC MCAM 涓粯璁よ鍒欑殑鍛戒腑璁℃暟鍣ㄣ€?       涓嶈兘淇濊瘉璁℃暟鍣ㄤ細琚惎鐢ㄥ苟鏄犲皠鍒版墍鏈夐粯璁よ鍒欙紝鍥犱负璁℃暟鍣ㄧ█缂猴紝椹卞姩閲囩敤灏藉姏鑰屼负鐨勬柟寮忋€?       榛樿瑙勫垯浣滀负鐗瑰畾 PF 鎴?VF 鐨勪富瑕佹暟鎹寘瀵煎悜锛坰teering锛夎鍒欙紝鍩轰簬鍏剁敱 AF 椹卞姩鍦ㄥ垵濮嬪寲
       鏃跺畨瑁呯殑 DMAC 鍦板潃銆備粠 debugfs 璇诲彇榛樿瑙勫垯鍛戒腑璁℃暟鍣ㄧ殑绀轰緥鍛戒护濡備笅锛?       cat /sys/kernel/debug/cn10k/npc/mcam_rules
   - - `nix_maxlf`
     - u16
     - runtime
     - 鐢ㄤ簬璁剧疆 NIX 纭欢鍧椾腑 LF 鐨勬渶澶ф暟閲忋€傝繖鏈夊姪浜庡鍔犲垎閰嶇粰宸插惎鐢?LF锛堜緥濡?MCAM 鏉＄洰锛?       鐨勯粯璁よ祫婧愮殑鍙敤鎬с€?
`octeontx2 PF` 椹卞姩瀹炵幇浜嗕互涓嬮┍鍔ㄧ壒瀹氱殑鍙傛暟銆?
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `unicast_filter_count`
     - u8
     - runtime
     - 璁剧疆鍙负璇ヨ澶囩紪绋嬬殑鍗曟挱杩囨护鍣ㄧ殑鏈€澶ф暟閲忋€傝繖鍙敤浜庡疄鐜版洿濂界殑璁惧璧勬簮鍒╃敤锛?       閬垮厤杩囬噺娑堣€楁湭浣跨敤鐨?MCAM 琛ㄦ潯鐩€?