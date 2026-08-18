## 鍐呮牳椹卞姩 k8temp


鏀寔鐨勮姱鐗囷細

  - AMD Athlon64/FX 鎴?Opteron CPU

    Prefix: 'k8temp'

    Addresses scanned: PCI 绌洪棿

    Datasheet: https://www.amd.com/system/files/TechDocs/32559.pdf

Author: Rudolf Marek

Contact: Rudolf Marek <r.marek@assembler.cz>

### 鎻忚堪


璇ラ┍鍔ㄥ厑璁歌鍙栧唴缃簬 AMD K8 绯诲垪 CPU锛圓thlon64/FX銆丱pteron锛変腑鐨勬俯搴︿紶鎰熷櫒鐨勮鏁般€傚畼鏂规枃妗ｇО鍏朵粠 K8 鏍稿績鐨?F 淇鐗堝紑濮嬪伐浣滐紝浣嗗疄闄呬笂瀹冧技涔庡湪闄ゅ墠涓や釜淇鐗堬紙SH-B0 涓?SH-B3锛変箣澶栫殑鎵€鏈?K8 淇鐗堜笂閮芥湁瀹炵幇銆?
璇锋敞鎰忥紝浣犺嚦灏戦渶瑕?lm-sensors 2.10.1 鎵嶈兘鑾峰緱姝ｅ父鐨勭敤鎴风┖闂存敮鎸併€?
鍗曚釜 CPU 鍐呮渶澶氬彲鏈夊洓涓俯搴︿紶鎰熷櫒銆傝椹卞姩浼氳嚜鍔ㄦ娴嬩紶鎰熷櫒锛屽苟鍙樉绀哄凡瀹炵幇浼犳劅鍣ㄧ殑娓╁害銆?
/sys 鏂囦欢鐨勬槧灏勫涓嬶細

============= ===================================
temp1_input   Core 0 涓?鈥滀綅缃€?0 鐨勬俯搴?temp2_input   Core 0 涓?鈥滀綅缃€?1 鐨勬俯搴?temp3_input   Core 1 涓?鈥滀綅缃€?0 鐨勬俯搴?temp4_input   Core 1 涓?鈥滀綅缃€?1 鐨勬俯搴?============= ===================================

娓╁害浠ユ憚姘忓害娴嬮噺锛屾祴閲忓垎杈ㄧ巼涓?1 搴?C銆傞璁℃湭鏉ョ殑 CPU 浼氭湁鏇村ソ鐨勫垎杈ㄧ巼銆傛俯搴︽瘡绉掓洿鏂颁竴娆°€傛湁鏁堟俯搴﹁寖鍥翠负 -49 鍒?206 搴?C銆?
绉颁负 TCaseMax 鐨勬俯搴︽槸閽堝淇鐗?E 鍙婁箣鍓嶇殑澶勫櫒鎸囧畾鐨勩€傝娓╁害瀹氫箟涓烘暎鐑墖锛坔eat-spreader锛変笌 CPU 澶栧３涔嬮棿鐨勬俯搴︼紝鍥犳璇ラ┍鍔ㄦ彁渚涚殑 CPU 鍐呴儴娓╁害鍙兘鏇撮珮銆傛病鏈夌畝鍗曠殑鏂规硶娴嬮噺涓?TCaseMax 娓╁害鐩稿叧鑱旂殑娓╁害銆?
瀵逛簬杈冩柊淇鐗堢殑 CPU锛坮ev F锛宻ocket AM2锛夛紝鏈変竴涓暟瀛﹁绠楀緱鍒扮殑娓╁害绉颁负 TControl锛屽畠蹇呴』浣庝簬 TControlMax銆?
鍏崇郴濡備笅锛?
	temp1_input - TjOffset*2 < TControlMax,

TjOffset 灏氭湭鐢遍┍鍔ㄥ鍑猴紝TControlMax 閫氬父涓?70 搴?C銆傜粡楠屾硶鍒?-> CPU 娓╁害涓嶅簲杩囧瓒呰繃 60 搴?C銆?