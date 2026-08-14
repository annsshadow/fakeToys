## PXA 鎽勫儚澶翠富鏈洪┍鍔?

浣滆€? Robert Jarzmik <robert.jarzmik@free.fr>

### 绾︽潫


a) YUV422P 鏍煎紡鐨勫浘鍍忓昂瀵?   鎵€鏈?YUV422P 鍥惧儚閮借寮哄埗瑕佹眰 width x height % 16 = 0銆?   杩欐槸鐢变簬 DMA 绾︽潫锛屽畠鍙紶杈?8 瀛楄妭鍊嶆暟鐨勫钩闈€?
### 鍏ㄥ眬瑙嗛宸ヤ綔娴?

a) QCI 宸插仠姝?   鏈€鍒濓紝QCI 鎺ュ彛鏄仠姝㈢殑銆?   褰撲竴涓紦鍐插尯琚帓闃熸椂锛岃皟鐢?start_streaming锛孮CI 鍚姩銆?
b) QCI 宸插惎鍔?   鍦?QCI 宸插惎鍔ㄧ殑鎯呭喌涓嬶紝鍙互鎺掗槦鏇村缂撳啿鍖鸿€屼笉浼氬仠姝㈡崟鑾枫€傛柊缂撳啿鍖鸿鈥滆拷鍔犫€濆埌 DMA 閾剧殑灏鹃儴锛屽苟
   骞虫粦鍦颁竴甯ф帴涓€甯ф崟鑾枫€?
   涓€鏃︿竴涓紦鍐插尯鍦?QCI 鎺ュ彛涓濉弧锛屽畠浼氳鏍囪涓衡€淒ONE鈥濆苟浠庢椿鍔ㄧ紦鍐插尯鍒楄〃涓Щ闄ゃ€傜劧鍚庡畠鍙互鐢辩敤鎴风┖闂村簲鐢ㄧ▼搴忛噸鏂版帓闃熸垨鍑洪槦銆?
   涓€鏃︽渶鍚庝竴涓紦鍐插尯琚～婊★紝QCI 鎺ュ彛鍋滄銆?
c) 鎹曡幏鍏ㄥ眬鏈夐檺鐘舵€佹満绀烘剰


	+----+                             +---+  +----+
	| DQ |                             | Q |  | DQ |
	|    v                             |   v  |    v
	+-----------+                     +------------------------+
	|   STOP    |                     | Wait for capture start |
	+-----------+         Q           +------------------------+
	+-> | QCI: stop | ------------------> | QCI: run               | <------------+
	|   | DMA: stop |                     | DMA: stop              |              |
	|   +-----------+             +-----> +------------------------+              |
	|                            /                            |                   |
	|                           /             +---+  +----+   |                   |
	|capture list empty        /              | Q |  | DQ |   | QCI Irq EOF       |
	|                         /               |   v  |    v   v                   |
	|   +--------------------+             +----------------------+               |
	|   | DMA hotlink missed |             |    Capture running   |               |
	|   +--------------------+             +----------------------+               |
	|   | QCI: run           |     +-----> | QCI: run             | <-+           |
	|   | DMA: stop          |    /        | DMA: run             |   |           |
	|   +--------------------+   /         +----------------------+   | Other     |
	|     ^                     /DMA still            |               | channels  |
	|     | capture list       /  running             | DMA Irq End   | not       |
	|     | not empty         /                       |               | finished  |
	|     |                  /                        v               | yet       |
	|   +----------------------+           +----------------------+   |           |
	|   |  Videobuf released   |           |  Channel completed   |   |           |
	|   +----------------------+           +----------------------+   |           |
	+-- | QCI: run             |           | QCI: run             | --+           |
	| DMA: run             |           | DMA: run             |               |
	+----------------------+           +----------------------+               |
		^                      /           |                           |
		|          no overrun /            | overrun                   |
		|                    /             v                           |
	+--------------------+         /   +----------------------+               |
	|  Frame completed   |        /    |     Frame overran    |               |
	+--------------------+ <-----+     +----------------------+ restart frame |
	| QCI: run           |             | QCI: stop            | --------------+
	| DMA: run           |             | DMA: stop            |
	+--------------------+             +----------------------+

	Legend锛堝浘渚嬶級: - 姣忎釜鏂规鏄竴涓?FSM 鐘舵€?  - 姣忎釜绠ご鏄浆鎹㈠埌鍙︿竴涓姸鎬佺殑鏉′欢
  - 甯︽敞閲婄殑绠ご鏄己鍒惰浆鎹紙鏃犳潯浠讹級
  - 绠ご "Q" 琛ㄧず锛氫竴涓紦鍐插尯宸茶鍏ラ槦
  - 绠ご "DQ" 琛ㄧず锛氫竴涓紦鍐插尯宸茶鍑洪槦
  - "QCI: stop" 琛ㄧず QCI 鎺ュ彛鏈娇鑳?  - "DMA: stop" 琛ㄧず鎵€鏈?3 涓?DMA 閫氶亾閮藉仠姝?  - "DMA: run" 琛ㄧず鑷冲皯鏈変竴涓?DMA 閫氶亾浠嶅湪杩愯

### DMA 浣跨敤


a) DMA 娴?     - 绗竴涓帓闃熺殑鎹曡幏缂撳啿鍖?       涓€鏃︾涓€涓紦鍐插尯琚帓闃熺敤浜庢崟鑾凤紝QCI 鍚姩锛屼絾鏁版嵁浼犺緭鏈惎鍔ㄣ€傚湪鈥滃抚缁撴潫锛圗nd Of Frame锛夆€濅腑鏂椂锛宨rq 澶勭悊绋嬪簭
       鍚姩 DMA 閾俱€?     - 涓€涓?videobuffer 鐨勬崟鑾?       DMA 閾惧紑濮嬪皢鏁版嵁浼犺緭鍒?videobuffer 鐨?RAM 椤典腑銆?       褰撴墍鏈夐〉閮戒紶杈撳畬姣曟椂锛屽湪 鈥淓NDINTR鈥?鐘舵€佷笅寮曞彂 DMA irq
     - 瀹屾垚涓€涓?videobuffer
       DMA irq 澶勭悊绋嬪簭灏?videobuffer 鏍囪涓衡€渄one鈥濓紝骞跺皢鍏朵粠娲诲姩杩愯闃熷垪涓Щ闄?       鍚屾椂锛屼笅涓€涓?videobuffer锛堝鏋滄湁锛夌敱 DMA 浼犺緭
     - 瀹屾垚鏈€鍚庝竴涓?videobuffer
       鍦ㄦ渶鍚庝竴涓?videobuffer 鐨?DMA irq 涓婏紝QCI 鍋滄銆?
b) 鍑嗗濂界殑 DMA 缂撳啿鍖哄皢鍏锋湁濡備笅缁撴瀯


     +------------+-----+---------------+-----------------+
     | desc-sg[^0^] | ... | desc-sg[last] | finisher/linker |
     +------------+-----+---------------+-----------------+

璇ョ粨鏋勭敱 dma->sg_cpu 鎸囧悜銆?鎻忚堪绗︾殑鐢ㄦ硶濡備笅锛?
- desc-sg[i]: 绗?i 涓弿杩扮锛屽皢绗?i 涓?sg 鍏冪礌浼犺緭鍒拌棰戠紦鍐插尯鐨勫垎鏁?鑱氶泦
- finisher: 鍏锋湁 ddadr=DADDR_STOP, dcmd=ENDIRQEN
- linker: 鍏锋湁 ddadr= 涓嬩竴涓棰戠紦鍐插尯鐨?desc-sg[^0^]锛宒cmd=0

瀵逛簬涓嬩竴涓ず鎰忓浘锛屽亣璁?d0=desc-sg[^0^] .. dN=desc-sg[N]锛?鈥渇鈥?浠ｈ〃 finisher锛屸€渓鈥?浠ｈ〃 linker銆?涓€涓吀鍨嬬殑杩愯閾炬槸锛?

         Videobuffer 1         Videobuffer 2
     +---------+----+---+  +----+----+----+---+
     | d0 | .. | dN | l |  | d0 | .. | dN | f |
     +---------+----+-|-+  ^----+----+----+---+
                      |    |
                      +----+

閾炬帴瀹屾垚鍚庯紝璇ラ摼鐪嬭捣鏉ュ儚锛?

         Videobuffer 1         Videobuffer 2         Videobuffer 3
     +---------+----+---+  +----+----+----+---+  +----+----+----+---+
     | d0 | .. | dN | l |  | d0 | .. | dN | l |  | d0 | .. | dN | f |
     +---------+----+-|-+  ^----+----+----+-|-+  ^----+----+----+---+
                      |    |                |    |
                      +----+                +----+
                                           new_link

c) DMA 鐑摼鎺ワ紙hot chaining锛夋椂闂寸墖闂

鐢变簬 DMA 閾炬帴鏄湪 DMA 杩愯鏈熼棿瀹屾垚鐨勶紝閾炬帴鍙兘鍙戠敓鍦?DMA 浠庝竴涓?Videobuffer 璺冲埌鍙︿竴涓椂銆傚湪绀烘剰鍥句笂锛屽鏋?閬囧埌浠ヤ笅搴忓垪锛岄偅灏嗘槸涓棶棰橈細

- DMA 閾炬槸 Videobuffer1 + Videobuffer2
- 璋冪敤 pxa_videobuf_queue() 鎺掗槦 Videobuffer3
- DMA 鎺у埗鍣ㄥ畬鎴?Videobuffer2锛孌MA 鍋滄


      =>
         Videobuffer 1         Videobuffer 2
     +---------+----+---+  +----+----+----+---+
     | d0 | .. | dN | l |  | d0 | .. | dN | f |
     +---------+----+-|-+  ^----+----+----+-^-+
                      |    |                |
                      +----+                +-- DMA DDADR 鍔犺浇 DDADR_STOP

- 璋冪敤 pxa_dma_add_tail_buf()锛孷ideobuffer2 鐨?鈥渇inisher鈥?琚?  鏇挎崲涓烘寚鍚?Videobuffer3 鐨?鈥渓inker鈥濓紙鍒涘缓 new_link锛?- pxa_videobuf_queue() 缁撴潫
- 璋冪敤 DMA irq 澶勭悊绋嬪簭锛屽畠缁堟 Videobuffer2
- Videobuffer3 鎹曡幏鏈瀹夋帓鍦?DMA 閾句笂锛堝洜涓哄畠鍋滄浜嗭紒锛侊紒锛?

         Videobuffer 1         Videobuffer 2         Videobuffer 3
     +---------+----+---+  +----+----+----+---+  +----+----+----+---+
     | d0 | .. | dN | l |  | d0 | .. | dN | l |  | d0 | .. | dN | f |
     +---------+----+-|-+  ^----+----+----+-|-+  ^----+----+----+---+
                      |    |                |    |
                      +----+                +----+
                                           new_link
                                          DMA DDADR 浠嶇劧鏄?DDADR_STOP

- 璋冪敤 pxa_camera_check_link_miss()
  杩欎細妫€鏌?DMA 鏄惁宸插畬鎴愪笖缂撳啿鍖轰粛鍦?pcdev->capture 鍒楄〃涓娿€傚鏋滄槸杩欐牱锛屾崟鑾峰皢琚噸鍚紝
  骞朵笖 Videobuffer3 琚畨鎺掑湪 DMA 閾句笂銆?- DMA irq 澶勭悊绋嬪簭缁撴潫


     濡傛灉鍦?pxa_camera_check_link_miss() 璇诲彇 DDADR() 鍊煎悗 DMA 鍒氬ソ鍋滄锛屾垜浠氨鏈変繚璇侊細褰?DMA 瀹屾垚璇ョ紦鍐插尯鏃讹紝
     DMA irq 澶勭悊绋嬪簭浼氳鍥炶皟锛屽苟涓?pxa_camera_check_link_miss() 灏嗚鍐嶆璋冪敤锛屼互閲嶆柊瀹夋帓 Videobuffer3銆?