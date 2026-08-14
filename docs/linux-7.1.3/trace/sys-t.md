
## 閫氳繃 STP 鐨?MIPI SyS-T


MIPI SyS-T 鍗忚椹卞姩鍙互涓?STM 绫昏澶囦竴璧蜂娇鐢紝浠ョ敓鎴愭爣鍑嗗寲鐨勮窡韪祦锛坱race stream锛夈€傞櫎浜嗕綔涓烘爣鍑嗕箣澶栵紝瀹冭繕鎻愪緵鏇村ソ鐨勮窡韪簮璇嗗埆涓庢椂闂存埑鍏宠仈锛坱imestamp correlation锛夈€?
涓轰簡灏?MIPI SyS-T 鍗忚椹卞姩鐢ㄤ簬浣犵殑 STM 璁惧锛岄鍏堜綘闇€瑕?CONFIG_STM_PROTO_SYS_T銆?
鐜板湪锛屼綘鍙互鍦ㄤ负 STM 璁惧鍒涘缓绛栫暐锛坧olicy锛夋椂锛岄€氳繃鍦ㄧ瓥鐣ュ悕绉颁腑鎸囧畾鏉ラ€夋嫨瑕佷娇鐢ㄧ殑鍗忚椹卞姩锛?
# mkdir /config/stp-policy/dummy_stm.0:p_sys-t.my-policy/

鎹㈠彞璇濊锛岀瓥鐣ュ悕绉版牸寮忔墿灞曞涓嬶細

  <device_name>:<protocol_name>.<policy_name>

鍥犳锛屼娇鐢?Intel TH 鏃跺畠鍙兘鐪嬭捣鏉ュ儚 "0-sth:p_sys-t.my-policy"銆?
濡傛灉鐪佺暐鍗忚鍚嶇О锛孲TM 绫诲皢閫夋嫨鏈€鍏堝姞杞界殑閭ｄ釜鍗忚椹卞姩銆?
浣犱篃鍙互閫氳繃浠ヤ笅鏂瑰紡鍐嶆纭涓€鍒囨寜棰勬湡宸ヤ綔锛?
# cat /config/stp-policy/dummy_stm.0:p_sys-t.my-policy/protocol
p_sys-t

鐜板湪锛屼娇鐢?MIPI SyS-T 鍗忚椹卞姩鏃讹紝configfs 涓殑姣忎釜绛栫暐鑺傜偣閮戒細鑾峰緱涓€浜涢澶栫殑灞炴€э紝瀹冧滑鍐冲畾浜嗙壒瀹氫簬璇ュ崗璁殑姣忔簮锛坧er-source锛夊弬鏁帮細

# mkdir /config/stp-policy/dummy_stm.0:p_sys-t.my-policy/default
# ls /config/stp-policy/dummy_stm.0:p_sys-t.my-policy/default
channels
clocksync_interval
do_len
masters
ts_interval
uuid

鍏朵腑鏈€閲嶈鐨勬槸 "uuid"锛屽畠鍐冲畾浜嗙敤浜庢爣璁版潵鑷婧愮殑鎵€鏈夋暟鎹殑 UUID銆傚綋鍒涘缓涓€涓柊鑺傜偣鏃跺畠浼氳嚜鍔ㄧ敓鎴愶紝浣嗕綘寰堝彲鑳戒細鎯宠鏇存敼瀹冦€?
do_len 寮€鍚?鍏抽棴 MIPI SyS-T 娑堟伅澶翠腑鐨勯檮鍔犫€減ayload length锛堣礋杞介暱搴︼級鈥濆瓧娈点€傞粯璁ゅ叧闂紝鍥犱负 STP 宸茬粡鏍囪浜嗘秷鎭竟鐣屻€?
ts_interval 涓?clocksync_interval 鍒嗗埆鍐冲畾浜嗗湪娑堟伅澶翠腑鍖呭惈鍗忚锛堣€岄潪浼犺緭锛屽嵆 STP锛夋椂闂存埑鎴栧彂閫?CLOCKSYNC 鍖呬箣鍓嶏紝鍙互缁忚繃澶氬皯姣鏃堕棿銆?
璇﹁ Documentation/ABI/testing/configfs-stp-policy-p_sys-t銆?
- [^1^] https://www.mipi.org/specifications/sys-t
