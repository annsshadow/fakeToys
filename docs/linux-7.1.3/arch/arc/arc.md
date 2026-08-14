
######## 闈㈠悜 ARC 澶勭悊鍣ㄧ殑 Linux 鍐呮牳


# 鍏朵粬淇℃伅鏉ユ簮


浠ヤ笅鏄竴浜涘彲浠ヨ幏鍙栧叧浜?ARC 澶勭悊鍣ㄥ強鐩稿叧寮€婧愰」鐩洿澶氫俊鎭殑璧勬簮銆?
- `<https://embarc.org>`_ - ARC 涓婂紑婧愯蒋浠剁殑绀惧尯闂ㄦ埛銆?  瀵绘壘鐩稿叧 FOSS 椤圭洰銆佸伐鍏烽摼鍙戝竷銆佹柊闂荤瓑鍐呭鐨勮壇濂借捣鐐广€?
- `<https://github.com/foss-for-synopsys-dwc-arc-processors>`_ -
  ARC 澶勭悊鍣ㄥ紑婧愰」鐩墍鏈夊紑鍙戞椿鍔ㄧ殑鎵€鍦ㄥ湴銆傚叾涓竴浜涢」鐩槸鍚勭涓婃父椤圭洰鐨勫垎鏀紝
  鍦ㄦ彁浜ゅ埌涓婃父椤圭洰涔嬪墠锛屸€滆繘琛屼腑鐨勫伐浣溾€濅細鎵樼浜庢銆傚叾浠栭」鐩垯鐢?Synopsys 寮€鍙戯紝
  骞朵綔涓哄紑婧愭彁渚涚粰绀惧尯鍦?ARC 澶勭悊鍣ㄤ笂浣跨敤銆?
- `Synopsys ARC 澶勭悊鍣ㄥ畼鏂圭綉绔?  <https://www.synopsys.com/designware-ip/processor-solutions.html>`_ -
  璇ョ珯鐐瑰彲鑾峰彇閮ㄥ垎 IP 鏂囨。锛坄Programmer's Reference
  Manual锛屽嵆 ARC HS 澶勭悊鍣?PRM
  <https://www.synopsys.com/dw/doc.php/ds/cc/programmers-reference-manual-ARC-HS.pdf>`_)
  浠ュ強閮ㄥ垎鍟嗕笟宸ュ叿鐨勫厤璐圭増鏈紙`Free nSIM
  <https://www.synopsys.com/cgi-bin/dwarcnsim/req1.cgi>`_ 涓?  `MetaWare Light Edition <https://www.synopsys.com/cgi-bin/arcmwtk_lite/reg1.cgi>`_锛夈€?  浣嗚娉ㄦ剰锛岃闂繖浜涙枃妗ｅ拰宸ュ叿閮介渶瑕佹敞鍐屻€?
# 鍏充簬 ARC 澶勭悊鍣ㄥ彲閰嶇疆鎬х殑閲嶈璇存槑


ARC 澶勭悊鍣ㄥ叿鏈夐珮搴﹀彲閰嶇疆鎬э紝Linux 鏀寔鑻ュ共鍙厤缃€夐」銆傚叾涓竴浜涢€夐」瀵硅蒋浠舵槸閫忔槑鐨?锛堜緥濡傜紦瀛樺嚑浣曠粨鏋勶紝鏈変簺鍙互鍦ㄨ繍琛屾椂琚帰娴嬪苟鐩稿簲閰嶇疆鍜屼娇鐢級锛岃€屽彟涓€浜涘垯闇€瑕佸湪
鍐呮牳鐨勯厤缃伐鍏凤紙鍗斥€渕ake menuconfig鈥濓級涓樉寮忛€夋嫨鎴栭厤缃€?
鐒惰€岋紝骞堕潪鎵€鏈夊彲閰嶇疆閫夐」鍦?ARC 澶勭悊鍣ㄨ繍琛?Linux 鏃堕兘鍙楁敮鎸併€係oC 璁捐鍥㈤槦搴斿弬鑰?ARC HS Databook 涓殑鈥淎ppendix E: Configuration for ARC Linux鈥濅互鑾峰彇鍙厤缃€ф寚鍗椼€?
閬靛惊杩欎簺鎸囧崡骞堕鍏堥€夋嫨鏈夋晥鐨勯厤缃€夐」锛屽浜庡府鍔╅伩鍏?SoC 鍚姩锛坆ringup锛変互鍙婅蒋浠?寮€鍙戣繃绋嬩腑浠讳綍涓嶅繀瑕佺殑闂鑷冲叧閲嶈銆?
# 涓?ARC 澶勭悊鍣ㄦ瀯寤?Linux 鍐呮牳


涓?ARC 澶勭悊鍣ㄦ瀯寤哄唴鏍哥殑杩囩▼涓庝换浣曞叾浠栨灦鏋勭浉鍚岋紝鍙€氳繃涓ょ鏂瑰紡瀹屾垚锛?
- 浜ゅ弶缂栬瘧锛圕ross-compilation锛夛細鍦ㄥ鐞嗗櫒鏋舵瀯涓嶅悓鐨勫紑鍙戜富鏈猴紙閫氬父涓?x86_64/amd64锛変笂
  涓?ARC 鐩爣杩涜缂栬瘧鐨勮繃绋嬨€?- 鏈湴缂栬瘧锛圢ative compilation锛夛細鍦ㄨ鏈夊畬鏁村紑鍙戠幆澧冿紙GNU 宸ュ叿閾俱€乨tc銆乵ake 绛夛級鐨?  ARC 骞冲彴锛堢‖浠舵澘鍗℃垨 QEMU 涔嬬被鐨勬ā鎷熷櫒锛変笂涓?ARC 杩涜缂栬瘧鐨勮繃绋嬨€?
涓ょ鎯呭喌涓嬶紝閮介渶瑕佷富鏈轰笂鏈€鏂扮殑 ARC GNU 宸ュ叿閾俱€係ynopsys 鎻愪緵浜嗗彲鐢ㄤ簬姝ょ洰鐨勭殑棰勬瀯寤?宸ュ叿閾惧彂甯冪増鏈紝鍙粠浠ヤ笅浣嶇疆鑾峰彇锛?
- Synopsys GNU 宸ュ叿閾惧彂甯冿細
  `<https://github.com/foss-for-synopsys-dwc-arc-processors/toolchain/releases>`_

- Linux 鍐呮牳缂栬瘧鍣ㄩ泦鍚堬細
  `<https://mirrors.edge.kernel.org/pub/tools/crosstool>`_

- Bootlin 鐨勫伐鍏烽摼闆嗗悎锛歚<https://toolchains.bootlin.com>`_

宸ュ叿閾惧畨瑁呭埌绯荤粺鍚庯紝璇风‘淇濆叾鈥渂in鈥濇枃浠跺す宸插姞鍏ヤ綘鐨?`PATH` 鐜鍙橀噺銆傜劧鍚庤缃?`ARCH=arc` 涓?`CROSS_COMPILE=arc-linux`锛堟垨涓庝綘瀹夎鐨?ARC 宸ュ叿閾惧墠缂€鐩稿尮閰嶇殑鍊硷級锛屾帴鐫€鐓у父鎵ц
`make defconfig && make`銆?
杩欏皢鍦ㄥ唴鏍告簮鐮佹爲鏍圭洰褰曠敓鎴愬彲鐢ㄧ殑鈥渧mlinux鈥濇枃浠讹紝鍙敤浜庨€氳繃 JTAG 鍔犺浇鍒扮洰鏍囩郴缁熴€?濡傛灉浣犻渶瑕佷竴涓彲鐢ㄤ簬 U-Boot 寮曞鍔犺浇绋嬪簭鐨勯暅鍍忥紝璇锋墽琛?`make uImage`锛?`uImage` 灏嗗湪 `arch/arc/boot` 鏂囦欢澶逛腑鐢熸垚銆?