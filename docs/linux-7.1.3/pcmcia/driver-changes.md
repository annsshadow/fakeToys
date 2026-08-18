## 椹卞姩鍙樻洿


鏈枃浠惰杩颁簡 2.6 涓奖鍝?PCMCIA 鍗￠┍鍔ㄤ綔鑰呯殑鍙樻洿锛?
- pcmcia_loop_config() 涓庤嚜鍔ㄩ厤缃紙鑷?2.6.36 璧凤級
   濡傛灉鐩稿簲璁剧疆浜?`struct pcmcia_device *p_dev->config_flags`锛?   pcmcia_loop_config() 鐜板湪浼氳嚜鍔ㄨ缃煇浜涢厤缃€硷紝灏界椹卞姩浠嶅彲鍦ㄥ洖璋冨嚱鏁颁腑
   瑕嗙洊杩欎簺璁剧疆銆傜洰鍓嶆彁渚涗互涓嬭嚜鍔ㄩ厤缃€夐」锛?
 - CONF_AUTO_CHECK_VCC : 妫€鏌ュ尮閰嶇殑 Vcc
 - CONF_AUTO_SET_VPP   : 璁剧疆 Vpp
 - CONF_AUTO_AUDIO     : 鑻ラ渶瑕佸垯鑷姩鍚敤闊抽绾? - CONF_AUTO_SET_IO    : 璁剧疆 ioport 璧勬簮锛?>resource[0,1]锛? - CONF_AUTO_SET_IOMEM : 璁剧疆绗竴涓?iomem 璧勬簮锛?>resource[^2^]锛?
- pcmcia_request_configuration -> pcmcia_enable_device锛堣嚜 2.6.36 璧凤級
   pcmcia_request_configuration() 宸查噸鍛藉悕涓?pcmcia_enable_device()锛屽洜涓?   瀹冧笌 pcmcia_disable_device() 鐩稿搴斻€傞厤缃缃幇鍦ㄥ瓨鍌ㄥ湪 struct
   pcmcia_device 涓紝渚嬪 config_flags銆乧onfig_index銆乧onfig_base銆乿pp
   绛夊瓧娈点€?
- pcmcia_request_window 鍙樻洿锛堣嚜 2.6.36 璧凤級
   椹卞姩鐜板湪涓嶅啀浣跨敤 win_req_t锛岃€屾槸闇€瑕佸～鍐?`struct pcmcia_device
   *p_dev->resource[2,3,4,5]` 浠ユ敮鎸佹渶澶氬洓涓?ioport 鑼冨洿銆傝皟鐢?   pcmcia_request_window() 鍚庯紝鎵惧埌鐨勫尯鍩熶細琚繚鐣欙紝骞跺彲绔嬪嵆浣跨敤鈥斺€旂洿鍒拌皟鐢?   pcmcia_release_window()銆?
- pcmcia_request_io 鍙樻洿锛堣嚜 2.6.36 璧凤級
   椹卞姩鐜板湪涓嶅啀浣跨敤 io_req_t锛岃€屾槸闇€瑕佸～鍐?`struct pcmcia_device
   *p_dev->resource[0,1]` 浠ユ敮鎸佹渶澶氫袱涓?ioport 鑼冨洿銆傝皟鐢?pcmcia_request_io()
   鍚庯紝鎵惧埌鐨勭鍙ｄ細琚繚鐣欙紱鍦ㄨ皟鐢?pcmcia_request_configuration() 鍚庯紝瀹冧滑鏂瑰彲
   浣跨敤銆?
- 涓嶅啀鏈?dev_info_t锛屼笉鍐嶆湁 cs_types.h锛堣嚜 2.6.36 璧凤級
   dev_info_t 浠ュ強鍙﹀鍑犱釜 typedef 宸茶绉婚櫎銆備笉瑕佸啀鍦?PCMCIA 璁惧椹卞姩涓娇鐢?   瀹冧滑銆傚悓鏃讹紝涓嶈鍖呭惈 pcmcia/cs_types.h锛屽洜涓鸿鏂囦欢宸蹭笉瀛樺湪銆?
- 涓嶅啀鏈?dev_node_t锛堣嚜 2.6.35 璧凤級
   涓嶅啀闇€瑕佸～鍐?"dev_node_t" 缁撴瀯銆?
- 鏂扮殑 IRQ 璇锋眰瑙勫垯锛堣嚜 2.6.35 璧凤級
   椹卞姩鐜板湪涓嶅啀浣跨敤鏃х殑 pcmcia_request_irq() 鎺ュ彛锛岃€屾槸鍙互鍦ㄤ互涓嬩袱鑰呴棿閫夋嫨锛?
   - 鐩存帴璋冪敤 request_irq/free_irq銆備娇鐢ㄦ潵鑷?`*p_dev->irq` 鐨?IRQ銆?   - 浣跨敤 pcmcia_request_irq(p_dev, handler_t)锛汸CMCIA 鏍稿績浼氬湪璋冪敤
     pcmcia_disable_device() 鎴栬澶囧脊鍑烘椂鑷姩娓呯悊銆?
- 涓嶅啀鏈?cs_error / CS_CHECK / CONFIG_PCMCIA_DEBUG锛堣嚜 2.6.33 璧凤級
   璇蜂娇鐢?Linux 椋庢牸鐨勬鏌ヨ繑鍥炲€肩殑鏂瑰紡锛屾潵浠ｆ浛 cs_error() 鍥炶皟鎴?CS_CHECK()
   瀹忥紱濡傛湁蹇呰锛岃皟璇曚俊鎭浣跨敤 "dev_dbg()" 鎴?"pr_debug()"銆?
- 鏂扮殑 CIS 鍏冪粍璁块棶锛堣嚜 2.6.33 璧凤級
   椹卞姩搴斾娇鐢?"pcmcia_get_tuple()"锛堝鏋滃彧瀵瑰崟涓紙鍘熷锛夊厓缁勬劅鍏磋叮锛夋垨
   "pcmcia_loop_tuple()"锛堝鏋滃鏌愪竴绫诲瀷鐨勬墍鏈夊厓缁勬劅鍏磋叮锛夛紝鏉ヤ唬鏇?   pcmcia_get_{first,next}_tuple()銆乸cmcia_get_tuple_data() 鍜?   pcmcia_parse_tuple()銆備负浜嗕粠 CISTPL_FUNCE 瑙ｇ爜 MAC锛屾柊澧炰簡杈呭姪鍑芥暟
   "pcmcia_get_mac_from_cis()"銆?
- 鏂扮殑閰嶇疆寰幆杈呭姪鍑芥暟锛堣嚜 2.6.28 璧凤級
   閫氳繃璋冪敤 pcmcia_loop_config()锛岄┍鍔ㄥ彲浠ラ亶鍘嗘墍鏈夊彲鐢ㄧ殑閰嶇疆閫夐」銆傚湪椹卞姩鐨?   probe() 闃舵锛屽湪澶у鏁帮紙濡傛灉涓嶆槸鍏ㄩ儴锛夋儏鍐典笅锛岄兘鏃犻渶鐩存帴浣跨敤
   pcmcia_get_{first,next}_tuple銆乸cmcia_get_tuple_data 鍜?pcmcia_parse_tuple銆?
- 鏂扮殑閲婃斁杈呭姪鍑芥暟锛堣嚜 2.6.17 璧凤級
   鐜板湪涓嶅啀闇€瑕佽皟鐢?pcmcia_release_{configuration,io,irq,win}锛屽彧闇€璋冪敤
   pcmcia_disable_device 鍗冲彲銆傜敱浜庡凡娌℃湁鍚堢悊鐨勭悊鐢卞幓璋冪敤 pcmcia_release_io
   鍜?pcmcia_release_irq锛屽畠浠殑瀵煎嚭宸茶绉婚櫎銆?
- 缁熶竴 detach 涓?REMOVAL 浜嬩欢浠ｇ爜锛屼互鍙?attach 涓?INSERTION 浜嬩欢浠ｇ爜

```
       void (*remove)          (struct pcmcia_device *dev);
       int (*probe)            (struct pcmcia_device *dev);

```
```

       int (*suspend)          (struct pcmcia_device *dev);
       int (*resume)           (struct pcmcia_device *dev);

  should be initialized in struct pcmcia_driver, and handle
  (SUSPEND == RESET_PHYSICAL) and (RESUME == CARD_RESET) events

```
- 浜嬩欢澶勭悊绋嬪簭鍦?struct pcmcia_driver 涓殑鍒濆鍖栵紙鑷?2.6.13 璧凤級
   浜嬩欢澶勭悊绋嬪簭浼氭敹鍒版墍鏈変簨浠剁殑閫氱煡锛屽苟涓斿繀椤讳綔涓洪┍鍔?struct pcmcia_driver
   涓殑 event() 鍥炶皟杩涜鍒濆鍖栥€?
- 涓嶅簲鍐嶄娇鐢?pcmcia/version.h锛堣嚜 2.6.13 璧凤級
   璇ユ枃浠舵渶缁堝皢琚Щ闄ゃ€?
- 鍐呮牳鍐呯殑璁惧<->椹卞姩鍖归厤锛堣嚜 2.6.13 璧凤級
   PCMCIA 璁惧鍙婂叾姝ｇ‘鐨勯┍鍔ㄧ幇鍦ㄥ彲浠ュ湪鍐呮牳绌洪棿涓繘琛屽尮閰嶃€傝瑙?   'devicetable.txt'銆?
- 璁惧妯″瀷闆嗘垚锛堣嚜 2.6.11 璧凤級
   struct pcmcia_device 浼氭敞鍐屽埌璁惧妯″瀷鏍稿績锛屽苟鍙€氳繃
   handle_to_dev(client_handle_t * handle) 浣跨敤锛堜緥濡傜敤浜?SET_NETDEV_DEV锛夈€?
- 灏嗗唴閮?I/O 绔彛鍦板潃杞崲涓?unsigned int锛堣嚜 2.6.11 璧凤級
   鍦?PCMCIA 鍗￠┍鍔ㄤ腑锛宨oaddr_t 搴旀浛鎹负 unsigned int銆?
- irq_mask 涓?irq_list 鍙傛暟锛堣嚜 2.6.11 璧凤級
   irq_mask 鍜?irq_list 鍙傛暟涓嶅簲鍐嶅湪 PCMCIA 鍗￠┍鍔ㄤ腑浣跨敤銆傜浉鍙嶏紝纭畾搴斾娇鐢?   鍝釜 IRQ 鏄?PCMCIA 鏍稿績鐨勮亴璐ｃ€傚洜姝わ紝link->irq.IRQInfo2 浼氳蹇界暐銆?
- client->PendingEvents 宸茬Щ闄わ紙鑷?2.6.11 璧凤級
   client->PendingEvents 涓嶅啀鍙敤銆?
- client->Attributes 宸茬Щ闄わ紙鑷?2.6.11 璧凤級
   client->Attributes 鏈浣跨敤锛屽洜姝ゅ凡浠庢墍鏈?PCMCIA 鍗￠┍鍔ㄤ腑绉婚櫎

- 鏍稿績鍑芥暟涓嶅啀鍙敤锛堣嚜 2.6.11 璧凤級
   浠ヤ笅鍑芥暟宸蹭粠鍐呮牳婧愮爜涓Щ闄わ紝鍥犱负鎵€鏈夊唴鏍稿唴椹卞姩閮戒笉浣跨敤瀹冧滑锛屼笖娌℃湁澶栭儴

```
	pcmcia_get_first_region()
	pcmcia_get_next_region()
	pcmcia_modify_window()
	pcmcia_set_event_mask()
	pcmcia_get_first_window()
	pcmcia_get_next_window()

```
- 妯″潡绉婚櫎鏃剁殑璁惧鍒楄〃閬嶅巻锛堣嚜 2.6.10 璧凤級
   鍦ㄦā鍧楃Щ闄ゆ椂锛屼笉鍐嶉渶瑕侀亶鍘嗛┍鍔ㄧ殑鍐呴儴瀹㈡埛绔垪琛ㄥ苟璋冪敤 ->detach() 鍑芥暟銆?
- 璧勬簮绠＄悊銆傦紙鑷?2.6.8 璧凤級
   灏界 PCMCIA 瀛愮郴缁熶細涓哄崱鍒嗛厤璧勬簮锛屼絾瀹冧笉鍐嶅皢杩欎簺璧勬簮鏍囪涓哄繖銆傝繖鎰忓懗鐫€
   椹卞姩浣滆€呯幇鍦ㄦ湁璐ｄ换鍍?Linux 涓殑鍏朵粬椹卞姩涓€鏍峰０鏄庢偍鐨勮祫婧愩€傛偍搴斾娇鐢?   request_region() 灏嗘偍鐨?IO 鍖哄煙鏍囪涓轰娇鐢ㄤ腑锛屽苟浣跨敤 request_mem_region()
   灏嗘偍鐨勫唴瀛樺尯鍩熸爣璁颁负浣跨敤涓€俷ame 鍙傛暟搴旀槸鎸囧悜鎮ㄩ┍鍔ㄥ悕绉扮殑鎸囬拡銆備緥濡傦紝瀵逛簬
   pcnet_cs锛宯ame 搴旀寚鍚戝瓧绗︿覆 "pcnet_cs"銆?
- CardServices 宸茬Щ闄?   2.4 涓殑 CardServices() 鍙槸涓€涓敤浜庤皟鐢ㄥ悇绉嶆湇鍔＄殑澶?switch 璇彞銆傚湪 2.6
   涓紝鎵€鏈夎繖浜涘叆鍙ｇ偣閮借瀵煎嚭骞剁洿鎺ヨ皟鐢紙pcmcia_report_error() 闄ゅ锛岀洿鎺ユ敼鐢?   cs_error() 鍗冲彲锛夈€?
- struct pcmcia_driver
   鎮ㄩ渶瑕佷娇鐢?struct pcmcia_driver 鍜?pcmcia_{un,}register_driver锛岃€屼笉鏄?   {un,}register_pccard_driver
