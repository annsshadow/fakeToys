## 闈㈠悜 Intel M.2 PCIe 璋冨埗瑙ｈ皟鍣ㄧ殑 IOSM 椹卞姩


IOSM锛圛PC over Shared Memory锛屽熀浜庡叡浜唴瀛樼殑 IPC锛夐┍鍔ㄦ槸涓€涓?WWAN PCIe 涓绘満
椹卞姩锛屼负 Linux 鎴?Chrome 骞冲彴寮€鍙戯紝鐢ㄤ簬鍦ㄤ富鏈哄钩鍙颁笌 Intel M.2 璋冨埗瑙ｈ皟鍣ㄤ箣闂?閫氳繃 PCIe 鎺ュ彛浜ゆ崲鏁版嵁銆傝椹卞姩鎻愪緵绗﹀悎 MBIM 鍗忚 [^1^] 鐨勬帴鍙ｃ€備换浣曞墠绔簲鐢?绋嬪簭锛堝 Modem Manager锛夐兘鍙互杞绘澗绠＄悊璇?MBIM 鎺ュ彛锛屼互鍚敤閫氬線 WWAN 鐨勬暟鎹?閫氫俊銆?
## 鍩烘湰鐢ㄦ硶


鏈彈绠＄悊鏃讹紝MBIM 鍔熻兘澶勪簬闈炴椿鍔ㄧ姸鎬併€侷OSM 椹卞姩浠呮彁渚涗竴涓敤鎴锋€佹帴鍙?MBIM
鈥淲WAN PORT鈥濓紝浠ｈ〃 MBIM 鎺у埗閫氶亾锛屽苟涓嶅弬涓庡姛鑳界殑绠＄悊銆傛娴嬬鍙ｆ灇涓惧苟鍚敤
MBIM 鍔熻兘鏄敤鎴锋€佸簲鐢ㄧ▼搴忕殑鑱岃矗銆?
姝ょ被鐢ㄦ埛鎬佸簲鐢ㄧ▼搴忕殑渚嬪瓙鏈夛細
- mbimcli锛堥殢 libmbim [^2^] 搴撲竴鍚屾彁渚涳級锛屼互鍙?- Modem Manager [^3^]

绠＄悊搴旂敤绋嬪簭闇€瑕佹墽琛屼互涓嬪繀瑕佹搷浣滀互寤虹珛 MBIM IP 浼氳瘽锛?- 鎵撳紑 MBIM 鎺у埗閫氶亾
- 閰嶇疆缃戠粶杩炴帴璁剧疆
- 杩炴帴鍒扮綉缁?- 閰嶇疆 IP 缃戠粶鎺ュ彛

## 绠＄悊搴旂敤绋嬪簭寮€鍙?

椹卞姩涓庣敤鎴锋€佹帴鍙ｆ弿杩板涓嬨€侻BIM 鍗忚鍦?[^1^] Mobile Broadband Interface
Model v1.0 Errata-1 涓弿杩般€?
### MBIM 鎺у埗閫氶亾鐢ㄦ埛鎬?ABI


#### /dev/wwan0mbim0 瀛楃璁惧


璇ラ┍鍔ㄩ€氳繃瀹炵幇 MBIM WWAN Port 鍚?MBIM 鍔熻兘鏆撮湶涓€涓?MBIM 鎺ュ彛銆傛帶鍒堕€氶亾绠￠亾鐨?鐢ㄦ埛鎬佷竴绔槸 /dev/wwan0mbim0 瀛楃璁惧銆傚簲鐢ㄧ▼搴忓簲浣跨敤姝ゆ帴鍙ｈ繘琛?MBIM 鍗忚
閫氫俊銆?
#### 鍒嗙墖锛團ragmentation锛?

鐢ㄦ埛鎬佸簲鐢ㄧ▼搴忚礋璐ｆ寜鐓?MBIM 瑙勮寖杩涜鎵€鏈夋帶鍒舵秷鎭殑鍒嗙墖涓庨噸缁勩€?
#### /dev/wwan0mbim0 write()


鏉ヨ嚜绠＄悊搴旂敤绋嬪簭鐨?MBIM 鎺у埗娑堟伅涓嶅緱瓒呰繃鍗忓晢鐨勬帶鍒舵秷鎭ぇ灏忋€?
#### /dev/wwan0mbim0 read()


绠＄悊搴旂敤绋嬪簭蹇呴』鎺ュ彈绛変簬鍗忓晢鎺у埗娑堟伅澶у皬鐨勬帶鍒舵秷鎭€?
### MBIM 鏁版嵁閫氶亾鐢ㄦ埛鎬?ABI


#### wwan0-X 缃戠粶璁惧


IOSM 椹卞姩涓?IP 娴侀噺鏆撮湶涓€涓被鍨嬩负 鈥渨wan鈥?鐨?IP 閾捐矾鎺ュ彛 鈥渨wan0-X鈥濄€侷proute
缃戠粶宸ュ叿鐢ㄤ簬鍒涘缓 鈥渨wan0-X鈥?缃戠粶鎺ュ彛骞跺皢鍏跺叧鑱斿埌 MBIM IP 浼氳瘽銆傝椹卞姩鏀寔
鏈€澶?8 涓?IP 浼氳瘽浠ヨ繘琛屽苟鍙?IP 閫氫俊銆?
鐢ㄦ埛鎬佺鐞嗗簲鐢ㄧ▼搴忚礋璐ｅ湪寤虹珛 SessionId 澶т簬 0 鐨?MBIM IP 浼氳瘽涔嬪墠鍒涘缓鏂扮殑
IP 閾捐矾銆?
渚嬪锛屼负 SessionId 涓?1 鐨?MBIM IP 浼氳瘽鍒涘缓鏂扮殑 IP 閾捐矾锛?
  ip link add dev wwan0-1 parentdev-name wwan0 type wwan linkid 1

椹卞姩灏嗚嚜鍔ㄦ妸 鈥渨wan0-1鈥?缃戠粶璁惧鏄犲皠鍒?MBIM IP 浼氳瘽 1銆?
## 鍙傝€?

[^1^] "MBIM (Mobile Broadband Interface Model) Errata-1"
      - https://www.usb.org/document-library/

[^2^] libmbim - "a glib-based library for talking to WWAN modems and
      devices which speak the Mobile Interface Broadband Model (MBIM)
      protocol"
      - http://www.freedesktop.org/wiki/Software/libmbim/

[^3^] Modem Manager - "a DBus-activated daemon which controls mobile
      broadband (2G/3G/4G) devices and connections"
      - http://www.freedesktop.org/wiki/Software/ModemManager/
