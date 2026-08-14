
## CTU CAN FD 椹卞姩


浣滆€咃細Martin Jerabek <martin.jerabek01@gmail.com>


### 鍏充簬 CTU CAN FD IP 鏍?

`CTU CAN FD <https://gitlab.fel.cvut.cz/canbus/ctucanfd_ip_core>`_
鏄竴涓敤 VHDL 缂栧啓鐨勫紑婧愯蒋鏍搞€?瀹冭捣婧愪簬 2015 骞?Ondrej Ille 鍦?`CTU <https://www.cvut.cz/en>`_ 鐨?`鐢垫皵宸ョ▼瀛﹂櫌锛團EE锛?<http://www.fel.cvut.cz/en/>`_ 鐨?`娴嬮噺绯?<https://meas.fel.cvut.cz/>`_
鐨勯」鐩€?
閽堝鍩轰簬 Xilinx Zynq SoC 鐨?MicroZed 鏉垮崱鐨?SocketCAN 椹卞姩
`Vivado 闆嗘垚 <https://gitlab.fel.cvut.cz/canbus/zynq/zynq-can-sja1000-top>`_
浠ュ強鍩轰簬 Intel Cyclone V 5CSEMA4U23C6 鐨?DE0-Nano-SoC Terasic 鏉垮崱鐨?`QSys 闆嗘垚 <https://gitlab.fel.cvut.cz/canbus/intel-soc-ctucanfd>`_
宸茬粡瀹屾垚寮€鍙戯紝鍚屾椂杩樺寘鎷璇ユ牳鐨?`PCIe 闆嗘垚 <https://gitlab.fel.cvut.cz/canbus/pcie-ctucanfd>`_
鐨勬敮鎸併€?
瀵逛簬 Zynq锛岃鏍搁€氳繃 APB 绯荤粺鎬荤嚎杩炴帴锛岃鎬荤嚎涓嶆敮鎸佽澶囨灇涓撅紝鍥犳
蹇呴』鍦?Device Tree 涓寚瀹氳璁惧銆傝繖绫昏澶囧湪 kernel 涓О涓?platform device
锛堝钩鍙拌澶囷級锛岀敱 platform device driver锛堝钩鍙拌澶囬┍鍔級澶勭悊銆?
CTU CAN FD 澶栬鐨勫熀鏈姛鑳芥ā鍨嬪凡琚?QEMU 涓荤嚎鎺ュ彈銆傚弬瑙?QEMU 鐨?`CAN 浠跨湡鏀寔 <https://www.qemu.org/docs/master/system/devices/can.html>`_
浜嗚В CAN FD 鎬荤嚎銆佷富鏈鸿繛鎺ヤ互鍙?CTU CAN FD 鏍哥殑浠跨湡銆備豢鐪熸敮鎸佺殑寮€鍙?鐗堟湰鍙互浠?QEMU 鏈湴寮€鍙?`浠撳簱 <https://gitlab.fel.cvut.cz/canbus/qemu-canbus>`_
鐨?ctu-canfd 鍒嗘敮鍏嬮殕寰楀埌銆?

### 鍏充簬 SocketCAN


SocketCAN 鏄?Linux 鍐呮牳涓?CAN 璁惧鐨勬爣鍑嗛€氱敤鎺ュ彛銆傞【鍚嶆€濅箟锛岃鎬荤嚎
閫氳繃 socket 璁块棶锛岀被浼间簬甯歌鐨勭綉缁滆澶囥€傚叾鑳屽悗鐨勫師鐞嗗湪
`Linux SocketCAN <https://www.kernel.org/doc/html/latest/networking/can.html>`_
涓湁娣卞叆鎻忚堪銆傜畝鑰岃█涔嬶紝瀹冩彁渚涗簡涓€绉?鍦?CAN 涔嬩笂瀹炵幇鍜屼娇鐢ㄩ珮灞傚崗璁殑鑷劧鏂瑰紡锛?涓庝緥濡備互澶綉涔嬩笂杩愯 UDP/IP 鐨勬柟寮忕浉鍚屻€?

#### 璁惧鎺㈡祴锛圖evice probe锛?

鍦ㄨ缁嗕粙缁?CAN 鎬荤嚎璁惧椹卞姩鐨勭粨鏋勪箣鍓嶏紝鎴戜滑鍏堥噸鐢充竴涓嬪唴鏍哥┒绔?鏄浣曞緱鐭ヨ澶囧瓨鍦ㄧ殑銆傛煇浜涙€荤嚎锛屽 PCI 鎴?PCIe锛屾敮鎸佽澶囨灇涓俱€備篃灏辨槸璇达紝
绯荤粺鍚姩鏃讹紝浼氬彂鐜版€荤嚎涓婄殑鎵€鏈夎澶囧苟璇诲彇瀹冧滑鐨勯厤缃€傚唴鏍搁€氳繃鍏?vendor ID 鍜?device ID 鏉ヨ瘑鍒澶囷紝濡傛灉瀛樺湪涓鸿鏍囪瘑绗︾粍鍚堟敞鍐岀殑椹卞姩锛?灏变細璋冪敤鍏?probe 鏂规硶鏉ヤ负璇ョ‖浠跺～鍏呴┍鍔ㄧ殑瀹炰緥銆俇SB 鐨勬儏鍐电被浼硷紝鍙笉杩?瀹冨厑璁歌澶囩儹鎻掓嫈銆?
瀵逛簬鐩存帴鍐呭祵鍦?SoC 涓苟杩炴帴鍒板唴閮ㄧ郴缁熸€荤嚎锛圓XI銆丄PB銆丄valon 绛夛級
鐨勫璁撅紝鎯呭喌鍒欎笉鍚屻€傝繖浜涙€荤嚎涓嶆敮鎸佹灇涓撅紝鍥犳鍐呮牳蹇呴』浠庡叾浠栧湴鏂?鑾风煡璁惧淇℃伅銆傝繖姝ｆ槸 Device Tree 鐨勭敤閫旀墍鍦ㄣ€?

#### 璁惧鏍戯紙Device tree锛?

璁惧鏍戜腑鐨勪竴涓潯鐩０鏄庝簡绯荤粺涓瓨鍦ㄤ竴涓澶囥€佸畠濡備綍琚闂紙浣嶄簬
鍝潯鎬荤嚎涓婏級浠ュ強瀹冪殑閰嶇疆鈥斺€斿瘎瀛樺櫒鍦板潃銆佷腑鏂瓑绛夈€傛绫昏澶囨爲
鐨勪竴涓ず渚嬪涓嬨€?

```

           / {
               /* ... */
               amba: amba {
                   #address-cells = <1>;
                   #size-cells = <1>;
                   compatible = "simple-bus";

                   CTU_CAN_FD_0: CTU_CAN_FD@43c30000 {
                       compatible = "ctu,ctucanfd";
                       interrupt-parent = <&intc>;
                       interrupts = <0 30 4>;
                       clocks = <&clkc 15>;
                       reg = <0x43c30000 0x10000>;
                   };
               };
           };


```


#### 椹卞姩缁撴瀯


璇ラ┍鍔ㄥ彲浠ュ垎涓轰袱閮ㄥ垎鈥斺€斾笌骞冲彴鐩稿叧鐨勮澶囧彂鐜颁笌鍒濆鍖栵紝浠ュ強涓庡钩鍙?鏃犲叧鐨?CAN 缃戠粶璁惧瀹炵幇銆?

##### 骞冲彴璁惧椹卞姩


瀵逛簬 Zynq锛岃鏍搁€氳繃 AXI 绯荤粺鎬荤嚎杩炴帴锛岃鎬荤嚎涓嶆敮鎸佹灇涓撅紝鍥犳璁惧
蹇呴』鍦?Device Tree 涓寚瀹氥€傝繖绫昏澶囧湪 kernel 涓О涓?**platform device**
锛堝钩鍙拌澶囷級锛岀敱 **platform device driver**锛堝钩鍙拌澶囬┍鍔級\  [^1^]_ 澶勭悊銆?
涓€涓钩鍙拌澶囬┍鍔ㄦ彁渚涗互涓嬪唴瀹癸細

- 涓€涓?**probe** 鍑芥暟

- 涓€涓?**remove** 鍑芥暟

- 涓€寮犺椹卞姩鑳藉澶勭悊鐨?**compatible**锛堝吋瀹癸級璁惧琛?
**probe** 鍑芥暟鍦ㄨ澶囧嚭鐜版椂锛堟垨椹卞姩鍔犺浇鏃讹紝浠ヨ緝鏅氳€呬负鍑嗭級琚伆濂借皟鐢ㄤ竴娆°€?濡傛灉鍚屼竴涓┍鍔ㄥ鐞嗗涓澶囷紝鍒欎細瀵规瘡涓澶囪皟鐢ㄤ竴娆?**probe** 鍑芥暟銆?瀹冪殑浣滅敤鏄垎閰嶅苟鍒濆鍖栧鐞嗚澶囨墍闇€鐨勮祫婧愶紝浠ュ強涓轰笌骞冲彴鏃犲叧鐨?灞傝缃簳灞傚嚱鏁帮紝渚嬪 **read_reg** 鍜?**write_reg**銆?涔嬪悗锛岄┍鍔ㄥ皢璁惧娉ㄥ唽鍒版洿楂樺眰锛屽湪鏈緥涓敞鍐屼负 **network device**锛堢綉缁滆澶囷級銆?
**remove** 鍑芥暟鍦ㄨ澶囨秷澶辨垨椹卞姩鍗冲皢鍗歌浇鏃惰璋冪敤銆傚畠鐢ㄤ簬閲婃斁
鍦?**probe** 涓垎閰嶇殑璧勬簮锛屽苟灏嗚澶囦粠鏇撮珮灞傛敞閿€銆?
鏈€鍚庯紝**compatible** 璁惧琛ㄥ０鏄庝簡璇ラ┍鍔ㄨ兘澶熷鐞嗙殑璁惧銆侱evice Tree
鏉＄洰 `compatible` 浼氫笌鎵€鏈?**platform drivers**锛堝钩鍙伴┍鍔級鐨勮〃杩涜鍖归厤銆?

           ```c
           /** Match table for OF platform binding **/
           static const struct of_device_id ctucan_of_match[] = {
               { .compatible = "ctu,canfd-2", },
               { .compatible = "ctu,ctucanfd", },
               { /** end of list **/ },
           };
           MODULE_DEVICE_TABLE(of, ctucan_of_match);

           static int ctucan_probe(struct platform_device *pdev);
           static int ctucan_remove(struct platform_device *pdev);

           static struct platform_driver ctucanfd_driver = {
               .probe  = ctucan_probe,
               .remove = ctucan_remove,
               .driver = {
                   .name = DRIVER_NAME,
                   .of_match_table = ctucan_of_match,
               },
           };
           module_platform_driver(ctucanfd_driver);
           ```



##### 缃戠粶璁惧椹卞姩


姣忎釜缃戠粶璁惧蹇呴』鑷冲皯鏀寔浠ヤ笅鎿嶄綔锛?
- 鍚姩璁惧锛歚ndo_open`

- 鍏抽棴璁惧锛歚ndo_close`

- 鍚戣澶囨彁浜?TX 甯э細`ndo_start_xmit`

- 鍚戠綉缁滃瓙绯荤粺鎶ュ憡 TX 瀹屾垚涓庨敊璇細ISR

- 鍚戠綉缁滃瓙绯荤粺鎻愪氦 RX 甯э細ISR 涓?NAPI

浜嬩欢鏉ユ簮鏈変袱绉嶅彲鑳斤細璁惧鍜岀綉缁滃瓙绯荤粺銆傝澶囦簨浠堕€氬父閫氳繃涓柇鍙戝嚭淇″彿锛?鐢变腑鏂湇鍔＄▼搴忥紙ISR锛夊鐞嗐€傛簮鑷綉缁滃瓙绯荤粺鐨勪簨浠跺鐞嗙▼搴忓垯鍦?`struct net_device_ops` 涓寚瀹氥€?
褰撹澶囪鍚姩鏃讹紝渚嬪閫氳繃璋冪敤 `ip link set can0 up`锛?浼氳皟鐢ㄩ┍鍔ㄧ殑 `ndo_open` 鍑芥暟銆傚畠搴斿綋鏍￠獙鎺ュ彛閰嶇疆骞堕厤缃拰鍚敤璁惧銆?鐩稿弽鐨勬搷浣滄槸 `ndo_close`锛屽湪璁惧琚叧闂椂璋冪敤锛屾棤璁烘槸鏄惧紡杩樻槸闅愬紡銆?
褰撶郴缁熼渶瑕佸彂閫佷竴涓抚鏃讹紝瀹冮€氳繃璋冪敤 `ndo_start_xmit` 鏉ュ疄鐜帮紝璇ュ嚱鏁板皢
甯у叆闃熷埌璁惧銆傚鏋滆澶囩殑 HW 闃熷垪锛團IFO銆侀偖绠辨垨浠讳綍瀹炵幇鏂瑰紡锛夊彉婊★紝
`ndo_start_xmit` 鐨勫疄鐜颁細閫氱煡缃戠粶瀛愮郴缁熷畠搴斿綋鍋滄 TX 闃熷垪
锛堥€氳繃 `netif_stop_queue`锛夈€備箣鍚庡綋璁惧鍐嶆鏈夊彲鐢ㄧ┖闂村苟鑳藉鍏ラ槦
鍙︿竴涓抚鏃讹紝浼氬湪 ISR 涓噸鏂板惎鐢ㄩ槦鍒椼€?
鎵€鏈夎澶囦簨浠堕兘鍦?ISR 涓鐞嗭紝鍏蜂綋鍖呮嫭锛?
#. **TX 瀹屾垚**銆傚綋璁惧鎴愬姛瀹屾垚涓€涓抚鐨勫彂閫佹椂锛岃甯т細鍦ㄦ湰鍦板洖鏄俱€?   鍙戠敓閿欒鏃讹紝鍒欐敼涓哄悜缃戠粶瀛愮郴缁熷彂閫佷竴涓俊鎭€ч敊璇抚 [^2^]_銆?   鍦ㄨ繖涓ょ鎯呭喌涓嬶紝杞欢 TX 闃熷垪閮戒細琚仮澶嶏紝浠ヤ究鍙互鍙戦€佹洿澶氬抚銆?
#. **閿欒鐘舵€?*銆傚鏋滃嚭閿欙紙渚嬪璁惧杩涘叆 bus-off 鐘舵€佹垨鍙戠敓 RX 婧㈠嚭锛夛紝
   閿欒璁℃暟鍣ㄤ細琚洿鏂帮紝淇℃伅鎬ч敊璇抚浼氳鍏ラ槦鍒?SW RX 闃熷垪銆?
#. **RX 缂撳啿鍖洪潪绌?*銆傚湪杩欑鎯呭喌涓嬶紝璇诲彇 RX 甯у苟灏嗗叾鍏ラ槦鍒?SW RX 闃熷垪銆?   閫氬父浣跨敤 NAPI 浣滀负涓棿灞傦紙鍙傝 锛夈€?

#### NAPI


浼犲叆甯х殑棰戠巼鍙兘寰堥珮锛岃€屾瘡甯ч兘璋冪敤涓柇鏈嶅姟绋嬪簭鐨勫紑閿€浼氶€犳垚鏄捐憲鐨?绯荤粺璐熻浇銆侺inux 鍐呮牳涓湁澶氱鏈哄埗鏉ュ鐞嗚繖绉嶆儏鍐点€傚畠浠槸闅忕潃 Linux
鍐呮牳澶氬勾鐨勫彂灞曞拰鏀硅繘鑰屾紨杩涘嚭鏉ョ殑銆傚浜庣綉缁滆澶囷紝褰撳墠鐨勬爣鍑嗘槸
NAPI鈥斺€?*New API锛堟柊 API锛?*銆傚畠绫讳技浜庣粡鍏哥殑 top-half/bottom-half
涓柇澶勭悊锛屽嵆瀹冧粎鍦?ISR 涓‘璁や腑鏂紝骞惰〃鏄庡叾浣欏鐞嗗簲鍦?softirq
涓婁笅鏂囦腑瀹屾垚銆傛澶栵紝瀹冭繕鎻愪緵浜嗗湪涓€娈垫椂闂村唴 **杞锛坧oll锛?* 鏂板抚鐨勫彲鑳芥€с€?杩欐湁鍙兘閬垮厤鍚敤涓柇銆佸湪 ISR 涓鐞嗕紶鍏?IRQ銆侀噸鏂板惎鐢?softirq 浠ュ強
灏嗕笂涓嬫枃鍒囨崲鍥?softirq 杩欎竴浠ｄ环楂樻槀鐨勮繃绋嬨€?
鏇村淇℃伅鍙傝 Documentation/networking/napi.rst <napi>銆?

### 灏嗘牳闆嗘垚鍒?Xilinx Zynq


璇ユ牳鎺ュ彛鐨勬槸 Avalon 鎬荤嚎鐨勪竴涓畝鍗曞瓙闆?锛堝弬瑙?Intel **Avalon Interface Specifications**锛夛紝
鍥犱负瀹冩渶鍒濈敤浜?Altera FPGA 鑺墖涓婏紝鑰?Xilinx 鍘熺敓浣跨敤 AXI 鎺ュ彛
锛堝弬瑙?ARM **AMBA AXI and ACE Protocol Specification AXI3,
AXI4, and AXI4-Lite, ACE and ACE-Lite**锛夈€?鏈€鏄庢樉鐨勮В鍐虫柟妗堟槸浣跨敤涓€涓?Avalon/AXI 妗ユ垨瀹炵幇鏌愮绠€鍗曠殑杞崲瀹炰綋銆?鐒惰€岋紝璇ユ牳鐨勬帴鍙ｆ槸鍗婂弻宸ョ殑锛屾病鏈夋彙鎵嬩俊鍙凤紝鑰?AXI 鏄叏鍙屽伐鐨勶紝
鍏锋湁鍙屽悜淇″彿銆傛澶栵紝鍗充究鏄?AXI-Lite 浠庢帴鍙ｄ篃鐩稿綋娑堣€楄祫婧愶紝鑰?CAN
鏍稿苟涓嶉渶瑕?AXI 鐨勭伒娲绘€т笌閫熷害銆?
鍥犳閫夋嫨浜嗕竴涓畝鍗曞緱澶氱殑鎬荤嚎鈥斺€擜PB锛圓dvanced Peripheral Bus锛岄珮绾у璁炬€荤嚎锛?锛堝弬瑙?ARM **AMBA APB Protocol Specification**锛夈€?APB-AXI 妗ュ湪 Xilinx Vivado 涓洿鎺ュ彲鐢紝鎺ュ彛閫傞厤瀹炰綋鍙槸涓€缁勭畝鍗曠殑
缁勫悎閫昏緫璧嬪€笺€?
鏈€鍚庯紝涓轰簡鑳藉灏嗚鏍镐綔涓鸿嚜瀹氫箟 IP 鍖呭惈鍦ㄦ鍥句腑锛屾牳杩炲悓 APB 鎺ュ彛
涓€璧疯鎵撳寘涓?Vivado 缁勪欢銆?

### CTU CAN FD 椹卞姩璁捐


CAN 璁惧椹卞姩鐨勪竴鑸粨鏋勫凡鍦?涓粙缁嶈繃銆傛帴涓嬫潵鐨勬钀藉皢鍏蜂綋鎻愪緵瀵?CTU
CAN FD 鏍搁┍鍔ㄧ殑鏇磋缁嗘弿杩般€?

#### 搴曞眰椹卞姩


璇ユ牳骞堕潪浠呬緵 SocketCAN 浣跨敤锛屽洜姝ゆ渶濂芥嫢鏈変竴涓笌 OS 鏃犲叧鐨勫簳灞傞┍鍔ㄣ€?杩欎釜搴曞眰椹卞姩闅忓悗鍙互鐢ㄤ簬 OS 椹卞姩鐨勫疄鐜颁腑锛屾垨鑰呯洿鎺ョ敤浜庤８鏈烘垨
鐢ㄦ埛绌洪棿搴旂敤绋嬪簭涓€傚彟涓€涓紭鐐规槸锛屽鏋滅‖浠剁暐鏈夊彉鍖栵紝鍙渶淇敼
搴曞眰椹卞姩鍗冲彲銆?
浠ｇ爜 [^3^]_ 閮ㄥ垎鐢卞伐鍏疯嚜鍔ㄧ敓鎴愶紝閮ㄥ垎鐢辨牳浣滆€呮墜宸ョ紪鍐欙紝骞跺寘鍚鏂?浣滆€呯殑璐＄尞銆傚簳灞傞┍鍔ㄦ敮鎸佽濡備互涓嬫搷浣滐細璁剧疆浣嶆椂搴忋€佽缃帶鍒跺櫒妯″紡銆?鍚敤/绂佺敤銆佽鍙?RX 甯с€佸啓鍏?TX 甯х瓑绛夈€?

#### 閰嶇疆浣嶆椂搴?

鍦?CAN 涓紝姣忎釜浣嶈鍒嗕负鍥涗釜娈碉細SYNC銆丳ROP銆丳HASE1 鍜?PHASE2銆傚畠浠殑
鎸佺画鏃堕棿浠ユ椂闂撮噺瀛愶紙Time Quantum锛夌殑鍊嶆暟琛ㄧず
锛堣瑙?`CAN Specification, Version 2.0 <http://esd.cs.ucr.edu/webres/can20.pdf>`_ 绗?8 绔狅級銆?閰嶇疆娉㈢壒鐜囷紙bitrate锛夋椂锛屾墍鏈夋鐨勬寔缁椂闂达紙浠ュ強鏃堕棿閲忓瓙锛夊繀椤绘牴鎹?娉㈢壒鐜囧拰閲囨牱鐐癸紙Sample Point锛夎绠楀緱鍑恒€傚浜?CAN FD锛屾爣绉版尝鐗圭巼
锛圢ominal bitrate锛夊拰鏁版嵁娉㈢壒鐜囷紙Data bitrate锛夋槸鍒嗗埆鐙珛璁＄畻鐨勩€?
SocketCAN 鐩稿綋鐏垫椿锛屾棦鍙互閫氳繃鎵嬪姩璁剧疆鎵€鏈夋鐨勬寔缁椂闂存潵鎻愪緵楂樺害
鑷畾涔夌殑閰嶇疆锛屼篃鍙互閫氳繃浠呰缃尝鐗圭巼鍜岄噰鏍风偣鏉ユ彁渚涗究鎹风殑閰嶇疆
锛堝鏋滄湭鎸囧畾锛岀敋鑷充細鏍规嵁 Bosch 寤鸿鑷姩閫夋嫨锛夈€傜劧鑰岋紝姣忎釜 CAN 鎺у埗鍣?鍙兘鍏锋湁涓嶅悓鐨勫熀鍑嗘椂閽熼鐜囧拰涓嶅悓鐨勬鎸佺画鏃堕棿瀵勫瓨鍣ㄥ搴︺€傚洜姝ょ畻娉?闇€瑕佹寔缁椂闂寸殑锛堜互鍙婃椂閽熼鍒嗛鍣ㄧ殑锛夋渶灏忓€煎拰鏈€澶у€硷紝骞跺皾璇曚紭鍖栬繖浜?鏁板€间互鍚屾椂婊¤冻绾︽潫鏉′欢鍜屾墍璇锋眰鐨勫弬鏁般€?

           ```c
           struct can_bittiming_const {
               char name[^16^];      /** Name of the CAN controller hardware **/
               __u32 tseg1_min;    /** Time segment 1 = prop_seg + phase_seg1 **/
               __u32 tseg1_max;
               __u32 tseg2_min;    /** Time segment 2 = phase_seg2 **/
               __u32 tseg2_max;
               __u32 sjw_max;      /** Synchronisation jump width **/
               __u32 brp_min;      /** Bit-rate prescaler **/
               __u32 brp_max;
               __u32 brp_inc;
           };
           ```


[lst:can_bittiming_const]

缁嗗績鐨勮鑰呬細娉ㄦ剰鍒帮紝PROP_SEG 鍜?PHASE_SEG1 娈电殑鎸佺画鏃堕棿涓嶆槸鍒嗗埆纭畾鐨勶紝
鑰屾槸鍏堝悎骞讹紝鐒跺悗榛樿鎯呭喌涓嬪皢寰楀埌鐨?TSEG1 鍦?PROP_SEG 鍜?PHASE_SEG1 涔嬮棿
骞冲潎鍒嗛厤銆傚疄闄呬笂杩欏嚑涔庢病鏈変粈涔堝奖鍝嶏紝鍥犱负閲囨牱鐐逛綅浜?PHASE_SEG1 鍜?PHASE_SEG2 涔嬮棿銆傜劧鑰屽湪 CTU CAN FD 涓紝`PROP` 鍜?`PH1` 鎸佺画鏃堕棿瀵勫瓨鍣?鍏锋湁涓嶅悓瀹藉害锛堝垎鍒负 6 浣嶅拰 7 浣嶏級锛屽洜姝よ嚜鍔ㄨ绠楃殑鍊煎彲鑳戒細婧㈠嚭杈冪煭鐨?瀵勫瓨鍣紝浠庤€屽繀椤诲湪涓よ€呬箣闂撮噸鏂板垎閰?[^4^]_銆?

#### 澶勭悊 RX


甯ф帴鏀跺湪 NAPI 闃熷垪涓鐞嗭紝褰?RXNE锛圧X FIFO Not Empty锛孯X FIFO 闈炵┖锛?浣嶈缃綅鏃讹紝鐢?ISR 鍚敤銆傚抚琚€愪釜璇诲彇锛岀洿鍒?RX FIFO 涓病鏈夊墿浣欏抚锛?鎴?NAPI 杞杩愯杈惧埌鏈€澶у伐浣滈厤棰濓紙鍙傝 锛夈€傜劧鍚庢瘡甯ц浼犻€掔粰缃戠粶
鎺ュ彛 RX 闃熷垪銆?
浼犲叆鐨勫抚鍙兘鏄?CAN 2.0 甯ф垨 CAN FD 甯с€傚湪鍐呮牳涓尯鍒嗚繖涓よ€呯殑鏂规硶鏄?鍒嗛厤 `struct can_frame` 鎴?`struct canfd_frame`锛屼袱鑰呭ぇ灏忎笉鍚屻€?鍦ㄦ帶鍒跺櫒涓紝鍏充簬甯х被鍨嬬殑淇℃伅瀛樺偍鍦?RX FIFO 鐨勭涓€涓瓧涓€?
杩欏氨缁欐垜浠甫鏉ヤ簡涓€涓厛鏈夐浮杩樻槸鍏堟湁铔嬬殑闂锛氭垜浠笇鏈涗负甯у垎閰?`skb`锛?骞朵笖鍙湁鍦ㄥ垎閰嶆垚鍔熸椂鎵嶄粠 FIFO 涓彇鍑哄抚锛涘惁鍒欏皢鍏朵繚鐣欏湪閭ｉ噷绋嶅悗澶勭悊銆?浣嗘槸涓轰簡鑳藉鍒嗛厤姝ｇ‘鐨?`skb`锛屾垜浠繀椤诲厛浠?FIFO 涓彇鍑虹涓€涓瓧銆傛湁鍑犵
鍙兘鐨勮В鍐虫柟妗堬細

#. 璇诲彇璇ュ瓧锛岀劧鍚庡垎閰嶃€傚鏋滃け璐ワ紝鍒欎涪寮冨抚鐨勫叾浣欓儴鍒嗐€傚綋绯荤粺鍐呭瓨
   涓嶈冻鏃讹紝鎯呭喌鏈潵灏卞緢绯熺硶銆?
#. 棰勫厛濮嬬粓鍒嗛厤瓒冲澶т互瀹圭撼 FD 甯х殑 `skb`銆傜劧鍚庤皟鏁?`skb` 鍐呴儴锛屼娇鍏?   鐪嬭捣鏉ュ儚鏄负杈冨皬鐨?CAN 2.0 甯у垎閰嶇殑銆?
#. 澧炲姞绐ヨ锛坧eek锛塅IFO 鑰岄潪娑堣垂璇ュ瓧鐨勯€夐」銆?
#. 濡傛灉鍒嗛厤澶辫触锛屽皢璇诲彇鐨勫瓧瀛樺叆椹卞姩鐨勬暟鎹腑銆備笅娆″皾璇曟椂锛屼娇鐢?   瀛樺偍鐨勫瓧鑰屼笉鏄啀娆¤鍙栥€?
鏂规 1 瓒冲绠€鍗曪紝浣嗗鏋滄垜浠兘鍋氬緱鏇村ソ锛屽畠灏变笉澶护浜烘弧鎰忋€傛柟妗?2
涓嶅彲鎺ュ彈锛屽洜涓哄畠闇€瑕佷慨鏀逛竴涓畬鏁村唴鏍哥粨鏋勭殑绉佹湁鐘舵€併€傜暐寰鍔犵殑
鍐呭瓨娑堣€椾笉杩囨槸鈥滆泲绯曗€濅笂鐨勮櫄鎷熸ū妗冦€傛柟妗?3 闇€瑕佷笉灏忕殑纭欢鏀瑰姩锛?浠庣‖浠惰搴︽潵鐪嬩篃涓嶇悊鎯炽€?
鏂规 4 浼间箮鏄竴涓笉閿欑殑鎶樹腑锛屽叾缂虹偣鏄儴鍒嗗抚鍙兘浼氬湪 FIFO 涓仠鐣?杈冮暱鏃堕棿銆傚敖绠″姝わ紝RX FIFO 鍙兘鍙湁涓€涓嫢鏈夎€咃紝鍥犳鍏朵粬浠讳綍浜洪兘
涓嶅簲鐪嬪埌璇ラ儴鍒嗗抚锛堝拷鐣ユ煇浜涚壒娈婄殑璋冭瘯鍦烘櫙锛夈€傛澶栵紝椹卞姩鍦ㄥ垵濮嬪寲鏃?浼氶噸缃牳锛屽洜姝よ閮ㄥ垎甯т篃鏃犳硶琚€滄敹鍏烩€濄€傛渶缁堥€夋嫨浜嗘柟妗?4 [^5^]_銆?

##### 涓?RX 甯ф墦鏃堕棿鎴?

CTU CAN FD 鏍镐細鎶ュ憡甯ц鎺ユ敹鐨勭‘鍒囨椂闂存埑銆傛椂闂存埑榛樿鍦?EOF 鏈€鍚庝竴浣嶇殑
閲囨牱鐐规崟鑾凤紝浣嗗彲閰嶇疆涓哄湪 SOF 浣嶆崟鑾枫€傛椂闂存埑婧愬湪鏍稿閮紝瀹藉害鍙揪 64 浣嶃€?鍦ㄦ挵鍐欐湰鏂囨椂锛屽皢鏃堕棿鎴充粠鍐呮牳浼犻€掑埌鐢ㄦ埛绌洪棿鐨勫姛鑳藉皻鏈疄鐜帮紝浣嗚鍒掑湪
鏈潵瀹屾垚銆?

#### 澶勭悊 TX


CTU CAN FD 鏍告湁 4 涓嫭绔嬬殑 TX 缂撳啿鍖猴紝姣忎釜閮芥湁鑷繁鐨勭姸鎬佸拰浼樺厛绾с€傚綋
鏍告兂瑕佸彂閫佹椂锛屼細閫夋嫨澶勪簬 Ready 鐘舵€佷笖浼樺厛绾ф渶楂樼殑 TX 缂撳啿鍖恒€?
浼樺厛绾ф槸瀵勫瓨鍣?TX_PRIORITY 涓殑 3 浣嶆暟鍊硷紙nibble 瀵归綈锛夈€傚浜庡ぇ澶氭暟
鐢ㄤ緥锛岃繖搴旇瓒冲鐏垫椿銆傜劧鑰岋紝SocketCAN 浠呬负浼犲嚭甯ф敮鎸佷竴涓?FIFO 闃熷垪 [^6^]_銆?缂撳啿鍖轰紭鍏堢骇鍙敤浜庢ā鎷?FIFO 琛屼负锛屾柟娉曟槸涓烘瘡涓紦鍐插尯鍒嗛厤涓嶅悓鐨勪紭鍏堢骇锛?骞跺湪涓€甯т紶杈撳畬鎴愬悗 **杞浆锛坮otating锛?* 浼樺厛绾с€?
闄や簡浼樺厛绾ц疆杞箣澶栵紝SW 杩樺繀椤荤淮鎶ゆ寚鍚戠敱 TX 缂撳啿鍖虹粍鎴愮殑 FIFO 鐨勫ご灏炬寚閽堬紝
浠ヤ究纭畾涓嬩竴涓抚搴斾娇鐢ㄥ摢涓紦鍐插尯锛坄txb_head`锛変互鍙婂摢涓紦鍐插尯搴旀槸鏈€鍏?瀹屾垚鐨勶紙`txb_tail`锛夈€傚疄闄呯殑缂撳啿鍖虹储寮曪紙鏄剧劧锛夋槸妯?4 鐨勶紙TX 缂撳啿鍖烘暟閲忥級锛?浣嗘寚閽堝繀椤昏嚦灏戝涓€浣嶏紝浠ヤ究鍖哄垎 FIFO 婊″拰 FIFO 绌衡€斺€斿湪杩欑鎯呭喌涓嬶紝
`txb\_head \equiv txb\_tail\ (\textrm{mod}\ 4)`銆備笅闈㈢粰鍑轰簡濡備綍缁存姢
FIFO 浠ュ強浼樺厛绾ц疆杞殑绀轰緥


|

+------+---+---+---+---+
| TXB# | 0 | 1 | 2 | 3 |
+======+===+===+===+===+
| Seq  | A | B | C |   |
+------+---+---+---+---+
| Prio | 7 | 6 | 5 | 4 |
+------+---+---+---+---+
|      |   | T |   | H |
+------+---+---+---+---+

|

+------+---+---+---+---+
| TXB# | 0 | 1 | 2 | 3 |
+======+===+===+===+===+
| Seq  |   | B | C |   |
+------+---+---+---+---+
| Prio | 4 | 7 | 6 | 5 |
+------+---+---+---+---+
|      |   | T |   | H |
+------+---+---+---+---+

|

+------+---+---+---+---+----+
| TXB# | 0 | 1 | 2 | 3 | 0鈥?|
+======+===+===+===+===+====+
| Seq  | E | B | C | D |    |
+------+---+---+---+---+----+
| Prio | 4 | 7 | 6 | 5 |    |
+------+---+---+---+---+----+
|      |   | T |   |   | H  |
+------+---+---+---+---+----+

|

   TX 缂撳啿鍖虹殑鐘舵€佸強鍏跺彲鑳界殑杞崲


##### 涓?TX 甯ф墦鏃堕棿鎴?

鍚?TX 缂撳啿鍖烘彁浜ゅ抚鏃讹紝鍙互鎸囧畾璇ュ抚搴旇鍙戦€佺殑鏃堕棿鎴炽€傚抚鐨勫彂閫佸彲鑳戒細
鏇存櫄寮€濮嬶紝浣嗕笉浼氭洿鏃┿€傛敞鎰忥紝鏃堕棿鎴充笉鍙備笌缂撳啿鍖轰紭鍏堢骇鎺掑簭鈥斺€旇繖瀹屽叏
鐢变笂杩版満鍒跺喅瀹氥€?
瀵瑰熀浜庢椂闂寸殑鎶ユ枃鍙戦€佺殑鏀寔鏈€杩戝凡琚悎骞跺埌 Linux v4.19
`Time-based packet transmission <https://lwn.net/Articles/748879/>`_锛?浣嗚繖椤瑰姛鑳藉浜?CAN 鏄惁瀹炵敤浠嶆湁寰呯爺绌躲€?
鍚屾牱绫讳技浜庤幏鍙?RX 甯х殑鏃堕棿鎴筹紝璇ユ牳涔熸敮鎸佽幏鍙?TX 甯х殑鏃堕棿鎴斥€斺€斿嵆甯?琚垚鍔熷彂閫佺殑鏃堕棿銆傚叾缁嗚妭涓庝负 RX 甯ф墦鏃堕棿鎴抽潪甯哥浉浼硷紝骞跺湪 涓弿杩般€?

#### 澶勭悊 RX 缂撳啿鍖烘孩鍑?

褰撴帴鏀跺埌鐨勫抚鏃犳硶瀹屾暣鏀惧叆纭欢 RX FIFO 鏃讹紝RX FIFO 婧㈠嚭鏍囧織锛圫TATUS[DOR]锛?浼氳缃綅锛屽苟瑙﹀彂鏁版嵁婧㈠嚭涓柇锛圖OI锛夈€傚湪澶勭悊璇ヤ腑鏂椂锛屽繀椤绘敞鎰忓厛娓呴櫎
DOR 鏍囧織锛堥€氳繃 COMMAND[CDO]锛夛紝鐒跺悗鍐嶆竻闄?DOI 涓柇鏍囧織銆傚惁鍒欙紝璇ヤ腑鏂細
绔嬪嵆 [^7^]_ 閲嶆柊瑙﹀彂銆?
**娉ㄦ剰**锛氬湪寮€鍙戣繃绋嬩腑锛屾浘璁ㄨ杩囧唴閮?HW 娴佹按绾挎槸鍚︿細鎵颁贡杩欎釜娓呴櫎
椤哄簭锛屼互鍙婃槸鍚﹀湪娓呴櫎鏍囧織鍜屼腑鏂箣闂撮渶瑕侀澶栫殑绌哄懆鏈熴€傚湪 Avalon 鎺ュ彛涓婏紝
纭疄琚瘉鏄庢槸杩欐牱锛屼絾 APB 鏄畨鍏ㄧ殑锛屽洜涓哄畠浣跨敤 2 鍛ㄦ湡浜嬪姟銆傛湰璐ㄤ笂锛?DOR 鏍囧織浼氳娓呴櫎锛屼絾鍦?DOI 娓呴櫎璇锋眰涔熷簲鐢ㄧ殑閭ｄ釜鍛ㄦ湡锛堥€氳繃灏嗗瘎瀛樺櫒鐨?Reset 杈撳叆缃珮锛夛紝DOI 瀵勫瓨鍣ㄧ殑 Preset 杈撳叆浠嶇劧涓洪珮銆傜敱浜?Set 鐨勪紭鍏堢骇
楂樹簬 Reset锛孌OI 鏍囧織涓嶄細琚浣嶃€傝繖宸茬粡閫氳繃浜ゆ崲 Set/Reset 浼樺厛绾у緱鍒?淇锛堝弬瑙?issue #187锛夈€?

#### 鎶ュ憡 Error Passive 涓?Bus Off 鐘舵€?

鍙兘闇€瑕佸湪鑺傜偣杈惧埌 **Error Passive**銆?*Error Warning** 鍜?**Bus Off** 鐘舵€佹椂
杩涜鎶ュ憡銆傞┍鍔ㄩ€氳繃涓柇锛圗PI銆丒WLI锛夎幏鐭ラ敊璇姸鎬佺殑鍙樺寲锛岀劧鍚庤鍙栭敊璇?璁℃暟鍣ㄦ潵纭畾鏍哥殑閿欒鐘舵€併€?
鐒惰€岋紝杩欓噷瀛樺湪涓€涓交寰殑绔炴€佹潯浠垛€斺€旂姸鎬佽浆鎹㈠彂鐢燂紙浠ュ強涓柇琚Е鍙戯級
鐨勬椂闂翠笌璇诲彇閿欒璁℃暟鍣ㄧ殑鏃堕棿涔嬮棿瀛樺湪寤惰繜銆傚綋鏀跺埌 EPI 鏃讹紝鑺傜偣鍙兘
澶勪簬 **Error Passive** 鎴?**Bus Off** 鐘舵€併€傚鏋滆妭鐐硅繘鍏?**Bus Off**锛屽畠鏄剧劧
浼氫繚鎸佽鐘舵€佺洿鍒拌澶嶄綅銆傚惁鍒欙紝鑺傜偣 **褰撳墠鎴栨浘缁?* 澶勪簬 **Error Passive**銆?鐒惰€岋紝涔熸湁鍙兘璇诲彇鍒扮殑鐘舵€佹槸 **Error Warning** 鐢氳嚦 **Error Active**銆傚湪
杩欑鎯呭喌涓嬶紝鏄惁浠ュ強绌剁珶鎶ュ憡浠€涔堝彲鑳藉苟涓嶆槑纭紝浣嗘垜涓汉鍊惧悜浜庤涓?浠嶅簲鎶ュ憡杩囧幓鐨勯敊璇姸鎬併€傜被浼煎湴锛屽綋鏀跺埌 EWLI 浣嗛殢鍚庢娴嬪埌鐨勭姸鎬佹槸
**Error Passive** 鏃讹紝搴旀姤鍛?**Error Passive**銆?

### CTU CAN FD 椹卞姩婧愮爜鍙傝€?

   :internal:

   :internal:

   :internal:

   :internal:


### CTU CAN FD IP 鏍镐笌椹卞姩寮€鍙戣嚧璋?

- Odrej Ille <ondrej.ille@gmail.com>

  - 浣滀负 CTU 娴嬮噺绯荤殑瀛︾敓鍚姩浜嗚椤圭洰
  - 澶氬勾鏉ヤ负椤圭洰鎶曞叆浜嗗ぇ閲忎釜浜烘椂闂翠笌鐑儏
  - 鍙備笌浜嗘洿澶氬彈璧勫姪鐨勪换鍔?
- `Department of Measurement <https://meas.fel.cvut.cz/>`_銆?  `Faculty of Electrical Engineering <http://www.fel.cvut.cz/en/>`_銆?  `Czech Technical University <https://www.cvut.cz/en>`_

  - 澶氬勾鏉ユ槸璇ラ」鐩殑涓昏鎶曡祫鏂?  - 鍦ㄥ叾闈㈠悜 `Skoda Auto <https://www.skoda-auto.cz/>`_ 鐨?CAN/CAN FD 璇婃柇妗嗘灦涓娇鐢ㄨ椤圭洰

- `Digiteq Automotive <https://www.digiteqautomotive.com/en>`_

  - 璧勫姪浜嗏€淐AN FD Open Cores Support Linux Kernel Based Systems鈥濋」鐩?  - 涓?CTU 鍗忓晢骞朵粯璐逛互鍏佽鍏紬璁块棶璇ラ」鐩?  - 涓鸿繖椤瑰伐浣滄彁渚涗簡棰濆璧勯噾

- `Department of Control Engineering <https://control.fel.cvut.cz/en>`_銆?  `Faculty of Electrical Engineering <http://www.fel.cvut.cz/en/>`_銆?  `Czech Technical University <https://www.cvut.cz/en>`_

  - 璐熻矗鈥淐AN FD Open Cores Support Linux Kernel Based Systems鈥濋」鐩?  - 鎻愪緵 GitLab 绠＄悊
  - 涓烘寔缁泦鎴愭彁渚涜櫄鎷熸湇鍔″櫒涓庤绠楄兘鍔?  - 涓?HIL 鎸佺画闆嗘垚娴嬭瘯鎻愪緵纭欢

- `PiKRON Ltd. <http://pikron.com/>`_

  - 涓哄惎鍔ㄩ」鐩紑婧愬噯澶囧伐浣滄彁渚涗簡灏戦噺璧勯噾

- Petr Porazil <porazil@pikron.com>

  - 璁捐 PCIe 鏀跺彂鍣ㄩ檮鍔犳澘骞剁粍瑁呮澘鍗?  - 涓哄熀浜?MicroZed/Zynq 鐨勭郴缁熻璁″拰缁勮 MZ_APO 鍩烘澘

- Martin Jerabek <martin.jerabek01@gmail.com>

  - Linux 椹卞姩寮€鍙?  - 鎸佺画闆嗘垚骞冲彴鏋舵瀯甯堜笌 GHDL 鏇存柊
  - 璁烘枃 `Open-source and Open-hardware CAN FD Protocol Support <https://dspace.cvut.cz/bitstream/handle/10467/80366/F3-DP-2019-Jerabek-Martin-Jerabek-thesis-2019-canfd.pdf>`_

- Jiri Novak <jnovak@fel.cvut.cz>

  - 鍦?CTU 娴嬮噺绯昏礋璐ｉ」鐩殑鍚姩銆佺鐞嗕笌浣跨敤

- Pavel Pisa <pisa@cmp.felk.cvut.cz>

  - 鍙戣捣寮€婧愶紝鍦?CTU 鎺у埗宸ョ▼绯昏礋璐ｉ」鐩崗璋冧笌绠＄悊

- Jaroslav Beran<jara.beran@gmail.com>

 - 璐熻矗 Intel SoC 鐨勭郴缁熼泦鎴愩€佹牳涓庨┍鍔ㄧ殑娴嬭瘯鍜屾洿鏂?
- Carsten Emde (`OSADL <https://www.osadl.org/>`_)

 - 鎻愪緵 OSADL 鐨勪笓涓氱煡璇嗕互璁ㄨ IP 鏍歌鍙? - 鎸囧嚭浜?LGPL 鍙兘鐨勬閿佷互鍙?CAN 鎬荤嚎鍙兘鐨勪笓鍒╅棶棰橈紝杩欎績浣?IP 鏍歌璁￠噸鏂版巿鏉冧负绫?BSD 璁稿彲

- Reiner Zitzmann and Holger Zeltwanger (`CAN in Automation <https://www.can-cia.org/>`_)

 - 鎻愪緵浜嗗缓璁拰甯姪浠ュ悜绀惧尯瀹ｄ紶璇ラ」鐩紝骞堕個璇锋垜浠弬鍔犲叧娉?CAN 鎬荤嚎鏈潵鍙戝睍鏂瑰悜鐨勬椿鍔?
- Jan Charvat

 - 涓?QEMU 瀹炵幇浜?CTU CAN FD 鍔熻兘妯″瀷锛岃妯″瀷宸查泦鎴愬埌 QEMU 涓荤嚎锛坄docs/system/devices/can.rst <https://www.qemu.org/docs/master/system/devices/can.html>`_锛? - 瀛﹀＋璁烘枃 Model of CAN FD Communication Controller for QEMU Emulator


### 娉ㄩ噴


   鍏朵粬鎬荤嚎鏈夎嚜宸辩殑鐗瑰畾椹卞姩鎺ュ彛鏉ヨ缃澶囥€?
   涓嶈涓?CAN Error Frame 娣锋穯銆傝繖鏄竴涓?`can_frame`锛屽叾 `CAN_ERR_FLAG`
   琚疆浣嶏紝骞跺湪鍏?`data` 瀛楁涓寘鍚竴浜涢敊璇俊鎭€?
   鍙湪 CTU CAN FD 浠撳簱
   `<https://gitlab.fel.cvut.cz/canbus/ctucanfd_ip_core>`_ 涓幏鍙?
   搴曞眰椹卞姩鍑芥暟 `ctucan_hw_set_nom_bittiming` 鍜?   `ctucan_hw_set_data_bittiming` 灏辨槸杩欐牱鍋氱殑銆?
   鍦ㄦ挵鍐欐湰璁烘枃鏃讹紝鏂规 1 浠嶅湪浣跨敤锛岃淇敼宸叉帓闃熷湪 gitlab issue #222 涓?
   涓ユ牸鏉ヨ锛岃嚜 v4.19 璧锋敮鎸佸涓?CAN TX 闃熷垪
   `can: enable multi-queue for SocketCAN devices <https://lore.kernel.org/patchwork/patch/913526/>`_
   浣嗗皻鏃犱富绾块┍鍔ㄤ娇鐢ㄥ畠浠€?
   鎴栬€呮洿纭垏鍦拌锛屽湪涓嬩竴涓椂閽熷懆鏈?