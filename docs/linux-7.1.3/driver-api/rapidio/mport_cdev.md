## RapidIO 瀛愮郴缁?mport 瀛楃璁惧椹卞姩锛坮io_mport_cdev.c锛?

## 1. 姒傝堪


璇ヨ澶囬┍鍔ㄦ槸 RapidIO.org 杞欢浠诲姟缁勶紙STG锛夊唴 Texas Instruments銆丗reescale銆?Prodrive Technologies銆丯okia Networks銆丅AE 涓?IDT 涔嬮棿鍗忎綔鐨勬垚鏋溿€傝繕鏀跺埌浜?鏉ヨ嚜 RapidIO.org 鍏朵粬鎴愬憳鐨勫叾浠栬緭鍏ャ€傚叾鐩爣鏄垱寤轰竴涓瓧绗︽ā寮忛┍鍔ㄦ帴鍙ｏ紝
浠ュ厑璁镐紬澶氫笖鍚勫紓鐨?RapidIO 瀹炵幇鑳藉浜掓搷浣滅殑鏂瑰紡锛屽皢 RapidIO 璁惧鐨?鑳藉姏鐩存帴鏆撮湶缁欏簲鐢ㄧ▼搴忋€?
璇ラ┍鍔紙MPORT_CDEV锛変负鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鎻愪緵瀵瑰熀鏈?RapidIO 瀛愮郴缁熸搷浣滅殑
璁块棶銆傚ぇ澶氭暟 RapidIO 鎿嶄綔閫氳繃 'ioctl' 绯荤粺璋冪敤鏀寔銆?
鍔犺浇璇ヨ澶囬┍鍔ㄥ悗锛屽畠浼氫负姣忎竴涓凡娉ㄥ唽鐨?RapidIO mport 璁惧鍦?/dev 鐩綍涓?鍒涘缓鍚嶄负 rio_mportX 鐨勬枃浠剁郴缁熻妭鐐广€傝妭鐐瑰悕涓殑 'X' 涓庡垎閰嶇粰姣忎釜鏈湴
mport 璁惧鐨勫敮涓€绔彛 ID 鐩稿尮閰嶃€?
浣跨敤鍙敤鐨勪竴缁?ioctl 鍛戒护锛岀敤鎴风┖闂村簲鐢ㄧ▼搴忓彲浠ユ墽琛屼互涓?RapidIO 鎬荤嚎涓?瀛愮郴缁熸搷浣滐細

- 浠?鍚?mport 璁惧鐨勯厤缃瘎瀛樺櫒璇诲彇鍜屽啓鍏?  锛圧IO_MPORT_MAINT_READ_LOCAL/RIO_MPORT_MAINT_WRITE_LOCAL锛?- 浠?鍚戣繙绋?RapidIO 璁惧鐨勯厤缃瘎瀛樺櫒璇诲彇鍜屽啓鍏ャ€?  杩欎簺鎿嶄綔鍦?RIO 瑙勮寖涓瀹氫箟涓?RapidIO 缁存姢璇?鍐欍€?  锛圧IO_MPORT_MAINT_READ_REMOTE/RIO_MPORT_MAINT_WRITE_REMOTE锛?- 涓?mport 璁惧璁剧疆 RapidIO 鐩爣 ID锛圧IO_MPORT_MAINT_HDID_SET锛?- 涓?mport 璁惧璁剧疆 RapidIO 缁勪欢鏍囩锛圕omponent Tag锛?  锛圧IO_MPORT_MAINT_COMPTAG_SET锛?- 鏌ヨ mport 璁惧鐨勯€昏緫绱㈠紩锛圧IO_MPORT_MAINT_PORT_IDX_GET锛?- 鏌ヨ mport 璁惧鐨勮兘鍔涗笌 RapidIO 閾捐矾閰嶇疆
  锛圧IO_MPORT_GET_PROPERTIES锛?- 鍚敤/绂佺敤鍚戠敤鎴风┖闂村簲鐢ㄧ▼搴忔姤鍛?RapidIO 闂ㄩ搩锛坉oorbell锛変簨浠?  锛圧IO_ENABLE_DOORBELL_RANGE/RIO_DISABLE_DOORBELL_RANGE锛?- 鍚敤/绂佺敤鍚戠敤鎴风┖闂村簲鐢ㄧ▼搴忔姤鍛?RIO 绔彛鍐欙紙port-write锛変簨浠?  锛圧IO_ENABLE_PORTWRITE_RANGE/RIO_DISABLE_PORTWRITE_RANGE锛?- 鏌ヨ/鎺у埗閫氳繃璇ラ┍鍔ㄦ姤鍛婄殑浜嬩欢绫诲瀷锛氶棬閾冦€佺鍙ｅ啓鎴栦袱鑰?  锛圧IO_SET_EVENT_MASK/RIO_GET_EVENT_MASK锛?- 涓虹壒瀹氬ぇ灏忋€丷apidIO 鐩爣 ID銆佽烦鏁帮紙hopcount锛変笌璇锋眰绫诲瀷閰嶇疆/鏄犲皠 mport 鐨?  鍑虹珯璇锋眰绐楀彛锛圧IO_MAP_OUTBOUND/RIO_UNMAP_OUTBOUND锛?- 涓虹壒瀹氬ぇ灏忋€丷apidIO 鍩哄湴鍧€涓庢湰鍦板唴瀛樺熀鍦板潃閰嶇疆/鏄犲皠 mport 鐨?  鍏ョ珯璇锋眰绐楀彛锛圧IO_MAP_INBOUND/RIO_UNMAP_INBOUND锛?- 涓轰笌杩滅▼ RapidIO 璁惧鐨?DMA 鏁版嵁浼犺緭鍒嗛厤/閲婃斁杩炵画鐨?DMA 涓€鑷存€у唴瀛樼紦鍐插尯
  锛圧IO_ALLOC_DMA/RIO_FREE_DMA锛?- 鍙戣捣涓庤繙绋?RapidIO 璁惧鐨?DMA 鏁版嵁浼犺緭锛圧IO_TRANSFER锛夈€?  鏀寔闃诲銆佸紓姝ヤ笌 posted锛堝嵆鈥滃彂灏勫悗涓嶇鈥濓級鏁版嵁浼犺緭妯″紡銆?- 妫€鏌?绛夊緟寮傛 DMA 鏁版嵁浼犺緭瀹屾垚锛圧IO_WAIT_FOR_ASYNC锛?- 绠＄悊 RapidIO 瀛愮郴缁熸敮鎸佺殑璁惧瀵硅薄锛圧IO_DEV_ADD/RIO_DEV_DEL锛夈€?  杩欏厑璁稿皢鍚勭 RapidIO 缁撴瀯锛坒abric锛夋灇涓剧畻娉曞疄鐜颁负鐢ㄦ埛绌洪棿搴旂敤绋嬪簭锛?  鍚屾椂浣跨敤鍐呮牳 RapidIO 瀛愮郴缁熸彁渚涚殑鍏朵綑鍔熻兘銆?
## 2. 纭欢鍏煎鎬?

璇ヨ澶囬┍鍔ㄤ娇鐢ㄥ唴鏍?RapidIO 瀛愮郴缁熷畾涔夌殑鏍囧噯鎺ュ彛锛屽洜姝ゅ畠鍙互涓庝换浣曠敱
RapidIO 瀛愮郴缁熸敞鍐岀殑 mport 璁惧椹卞姩涓€璧蜂娇鐢紝闄愬埗鐢卞彲鐢ㄧ殑 mport 瀹炵幇璁剧疆銆?
鐩墠鏈€甯歌鐨勯檺鍒舵槸鐗瑰畾 mport 璁惧鏄惁鏈夊彲鐢ㄧ殑 RapidIO 涓撶敤
DMA 寮曟搸妗嗘灦銆傜敤鎴峰湪璁″垝浣跨敤璇ラ┍鍔ㄦ椂搴旈獙璇佸叾骞冲彴鍙敤鍔熻兘锛?
- IDT Tsi721 PCIe 鍒?RapidIO 妗ユ帴璁惧鍙婂叾 mport 璁惧椹卞姩涓庤椹卞姩瀹屽叏鍏煎銆?- Freescale SoC 鐨?'fsl_rio' mport 椹卞姩娌℃湁瀹炵幇 RapidIO 涓撶敤 DMA 寮曟搸鏀寔锛?  鍥犳 mport_cdev 椹卞姩鐨?DMA 鏁版嵁浼犺緭涓嶅彲鐢ㄣ€?
## 3. 妯″潡鍙傛暟


- 'dma_timeout'
      - DMA 浼犺緭瀹屾垚瓒呮椂锛堜互姣璁★紝榛樿鍊?3000锛夈€?        璇ュ弬鏁拌缃?SYNC 妯″紡 DMA 浼犺緭璇锋眰涓?RIO_WAIT_FOR_ASYNC
        ioctl 璇锋眰鐨勬渶澶у畬鎴愮瓑寰呮椂闂淬€?
- 'dbg_level'
      - 璇ュ弬鏁板厑璁告帶鍒惰璁惧椹卞姩鐢熸垚鐨勮皟璇曚俊鎭噺銆傝鍙傛暟鐢变竴缁?        瀵瑰簲浜庣壒瀹氬姛鑳藉潡鐨勪綅鎺╃爜鏋勬垚銆?        鏈夊叧鎺╃爜瀹氫箟璇峰弬瑙?'drivers/rapidio/devices/rio_mport_cdev.c'
        璇ュ弬鏁板彲浠ュ姩鎬佹洿鏀广€?        浣跨敤 CONFIG_RAPIDIO_DEBUG=y 浠ュ湪椤跺眰鍚敤璋冭瘯杈撳嚭銆?
## 4. 宸茬煡闂


  鏃犮€?
## 5. 鐢ㄦ埛绌洪棿搴旂敤绋嬪簭涓?API


浣跨敤姝よ澶囬┍鍔ㄧ殑 API 搴撲笌搴旂敤绋嬪簭鍙粠 RapidIO.org 鑾峰彇銆?
## 6. 寰呭姙鍒楄〃锛圱ODO List锛?

- 娣诲姞瀵瑰彂閫?鎺ユ敹鈥滃師濮嬧€漅apidIO 娑堟伅鏁版嵁鍖呯殑鏀寔銆?- 褰?RapidIO 涓撶敤 DMA 涓嶅彲鐢ㄦ椂锛屾坊鍔犲唴瀛樻槧灏勭殑 DMA 鏁版嵁浼犺緭浣滀负閫夐」銆?