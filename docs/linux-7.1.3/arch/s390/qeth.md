## IBM s390 QDIO 浠ュお缃戦┍鍔?

## OSA 鍜?HiperSockets 妗ユ帴绔彛锛圔ridge Port锛夋敮鎸?

### Uevents


瑕佺敓鎴愯繖浜涗簨浠讹紝璁惧蹇呴』琚祴浜堜富锛坧rimary锛夋垨娆★紙secondary锛夋ˉ鎺ョ鍙ｇ殑瑙掕壊銆傛洿澶氫俊鎭紝璇峰弬瑙?z/VM Connectivity, SC24-6174"銆?
褰撹繍琛屽湪 OSA 鎴?HiperSockets 妗ユ帴鑳藉姏绔彛纭欢涓婏紝骞朵笖閫氶亾涓婃煇涓凡閰嶇疆鐨勬ˉ鎺ョ鍙ｈ澶囩姸鎬佸彂鐢熷彉鍖栨椂锛屼細浠ｈ〃鐩稿簲鐨?ccwgroup 璁惧鍙戝嚭涓€涓?ACTION=CHANGE 鐨?udev 浜嬩欢銆傝浜嬩欢鍏锋湁浠ヤ笅灞炴€э細

BRIDGEPORT=statechange
  琛ㄧず璇ユˉ鎺ョ鍙ｈ澶囨敼鍙樹簡鍏剁姸鎬併€?
ROLE={primary|secondary|none}
  璧嬩簣璇ョ鍙ｇ殑瑙掕壊銆?
STATE={active|standby|inactive}
  璇ョ鍙ｆ柊閲囩撼鐨勭姸鎬併€?
褰撹繍琛屽湪鍚敤浜嗕富鏈哄湴鍧€閫氱煡鐨?HiperSockets 妗ユ帴鑳藉姏绔彛纭欢涓婃椂锛屼細鍙戝嚭涓€涓?ACTION=CHANGE 鐨?udev 浜嬩欢銆傚綋鏌愪釜涓绘満鎴?VLAN 鍦ㄨ璁惧鎵€鏈嶅姟鐨勭綉缁滀笂娉ㄥ唽鎴栨敞閿€鏃讹紝浼氫唬琛ㄧ浉搴旂殑 ccwgroup 璁惧鍙戝嚭璇ヤ簨浠躲€傝浜嬩欢鍏锋湁浠ヤ笅灞炴€э細

BRIDGEDHOST={reset|register|deregister|abort}
  涓绘満鍦板潃
  閫氱煡閲嶆柊鍚姩銆佸湪妗ユ帴绔彛 HiperSockets 閫氶亾涓婃敞鍐屾垨娉ㄩ攢涓€涓柊鐨勪富鏈烘垨 VLAN锛屾垨鑰呬腑姝㈠湴鍧€閫氱煡銆?
VLAN=numeric-vlan-id
  浜嬩欢鍙戠敓鎵€鍦ㄧ殑 VLAN ID銆傚鏋滀簨浠朵笉娑夊強 VLAN锛屽垯涓嶅寘鍚椤广€?
MAC=xx:xx:xx:xx:xx:xx
  姝ｅ湪琚敞鍐屾垨娉ㄩ攢鐨勪富鏈虹殑 MAC 鍦板潃銆傚鏋滀簨浠舵姤鍛婄殑鏄?VLAN 鐨勫垱寤烘垨閿€姣侊紝鍒欎笉鎶ュ憡姝ら」銆?
NTOK_BUSID=x.y.zzzz
  璁惧鎬荤嚎 ID锛圕SSID銆丼SID 鍜?device number锛夈€?
NTOK_IID=xx
  璁惧 IID銆?
NTOK_CHPID=xx
  璁惧 CHPID銆?
NTOK_CHID=xxxx
  璁惧閫氶亾 ID銆?
璇锋敞鎰忥紝`NTOK_*` 灞炴€ф寚鐨勬槸涓庤繍琛岃 OS 鐨勭郴缁熸墍杩炴帴璁惧涓嶅悓鐨勮澶囥€?