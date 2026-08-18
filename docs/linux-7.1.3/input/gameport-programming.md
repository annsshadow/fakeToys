#### 缂栧啓 gameport 椹卞姩


#### 涓€涓熀鏈殑缁忓吀 gameport


濡傛灉 gameport 娌℃湁鎻愪緵瓒呭嚭 inb()/outb() 涔嬪鐨勫姛鑳斤紝
```

	struct gameport gameport;

	gameport.io = MY_IO_ADDRESS;
	gameport_register_port(&gameport);

```
纭繚 struct gameport 鐨勫叾浠栨墍鏈夊瓧娈甸兘鍒濆鍖栦负 0銆俫ameport 閫氱敤浠ｇ爜浼氳礋璐ｅ鐞嗗叾浣欓儴鍒嗐€?

濡傛灉浣犵殑纭欢鏀寔澶氫釜 io 鍦板潃锛屽苟涓斾綘鐨勯┍鍔ㄥ彲浠ラ€夋嫨灏嗙‖浠剁紪绋嬪埌鍝竴涓紝閭ｄ箞浠庤緝涓嶅父瑙?
鐨勫湴鍧€寮€濮嬫槸鏇村ソ鐨勯€夋嫨锛屽洜涓轰笌鏍囧噯 0x201 鍦板潃鍙戠敓鍐茬獊鐨勫彲鑳芥€ф洿灏忋€?

渚嬪锛屽鏋滀綘鐨勯┍鍔ㄦ敮鎸佸湴鍧€ 0x200銆?x208銆?x210 鍜?0x218锛岄偅涔?0x218 灏嗘槸棣栭€夊湴鍧€銆?

濡傛灉浣犵殑纭欢鏀寔鐨?gameport 鍦板潃鏈槧灏勫埌 ISA io 绌洪棿锛堥珮浜?0x1000锛夛紝鍒欎娇鐢ㄨ鍦板潃锛?
骞朵笖涓嶈鏄犲皠 ISA 闀滃儚銆?

姝ゅ锛屽簲濮嬬粓瀵?gameport 鍗犵敤鐨勬暣涓?io 绌洪棿鎵ц request_region()銆傚敖绠＄湡姝ｄ娇鐢ㄧ殑鍙湁
涓€涓?ioport锛屼絾 gameport 閫氬父鍗犵敤 io 绌洪棿涓粠 1 鍒?16 涓湴鍧€銆?

璇峰悓鏃惰€冭檻鍦?->open() 鍥炶皟涓湪鍗′笂鍚敤 gameport锛堝鏋?io 鏄犲皠鍒?ISA 绌洪棿锛夆€斺€旇繖鏍峰畠灏?
浠呭湪鏈夌湡姝ｄ娇鐢ㄥ畠鐨勬椂鍊欐墠鍗犵敤 io 绌洪棿銆傚湪 ->close() 鍥炶皟涓啀娆＄鐢ㄥ畠銆備綘涔熷彲浠ュ湪
->open() 鍥炶皟涓€夋嫨 io 鍦板潃锛岃繖鏍峰綋鏌愪簺鍙兘鐨勫湴鍧€宸茶鍏朵粬 gameport 鍗犵敤鏃跺畠涔熶笉浼氬け璐ャ€?

#### 鍐呭瓨鏄犲皠鐨?gameport


褰撲竴涓?gameport 鍙互閫氳繃 MMIO 璁块棶鏃讹紝浼樺厛閲囩敤杩欑鏂瑰紡锛屽洜涓哄畠鏇村揩锛屽厑璁告瘡绉掓洿澶氱殑
璇诲彇娆℃暟銆傛敞鍐岃繖鏍蜂竴涓?gameport
```

	struct gameport gameport;

	void my_trigger(struct gameport *gameport)
	{
		my_mmio = 0xff;
	}

	unsigned char my_read(struct gameport *gameport)
	{
		return my_mmio;
	}

	gameport.read = my_read;
	gameport.trigger = my_trigger;
	gameport_register_port(&gameport);

```

#### Cooked 妯″紡 gameport


鏈変簺 gameport 鑳藉灏嗚酱鍊间互鏁板瓧褰㈠紡涓婃姤锛岃繖鎰忓懗鐫€椹卞姩涓嶅繀鍐嶄互鏃ф柟寮忔祴閲忓畠浠€斺€斾竴涓?ADC
宸插唴缃湪
```

	struct gameport gameport;

	int my_cooked_read(struct gameport *gameport, int *axes, int *buttons)
	{
		int i;

		for (i = 0; i < 4; i++)
			axes[i] = my_mmio[i];
		buttons[0] = my_mmio[4];
	}

	int my_open(struct gameport *gameport, int mode)
	{
		return -(mode != GAMEPORT_MODE_COOKED);
	}

	gameport.cooked_read = my_cooked_read;
	gameport.open = my_open;
	gameport.fuzz = 8;
	gameport_register_port(&gameport);

```
杩欓噷鍞竴浠や汉鍥版儜鐨勬槸 fuzz 鍊笺€傛渶濂介€氳繃瀹為獙纭畾锛屽畠鏄?ADC 鏁版嵁涓殑鍣０閲忋€傚畬缇庣殑
gameport 鍙互灏嗗叾璁句负 0锛屾渶甯歌鐨?fuzz 鍦?8 鍒?32 涔嬮棿銆傛湁鍏?fuzz 鐨勫鐞嗭紝璇峰弬瑙?
analog.c 鍜?input.c鈥斺€攆uzz 鍊煎喅瀹氫簡涓€涓敤浜庢秷闄ゆ暟鎹腑鍣０鐨勯珮鏂护娉㈠櫒绐楀彛鐨勫ぇ灏忋€?

#### 鏇村鏉傜殑 gameport


gameport 鍙互鍚屾椂鏀寔 raw 鍜?cooked 妯″紡銆傚湪杩欑鎯呭喌涓嬶紝灏嗙ず渚?1+2 鎴?1+3 缁勫悎鍗冲彲銆?
gameport 鍙互鏀寔鍐呴儴鏍″噯鈥斺€旇涓嬫枃锛屼互鍙?lightning.c 鍜?analog.c 浜嗚В鍏跺伐浣滄柟寮忋€傚鏋?
浣犵殑椹卞姩鍚屾椂鏀寔澶氫釜 gameport 瀹炰緥锛岃浣跨敤 gameport 缁撴瀯浣撶殑 ->private 鎴愬憳鎸囧悜浣犵殑鏁版嵁銆?

#### 娉ㄩ攢涓€涓?gameport


```

    gameport_unregister_port(&gameport);

```
#### gameport 缁撴瀯浣?


```

    struct gameport {

	void *port_data;

```
渚?gameport 椹卞姩鑷敱浣跨敤鐨勭鏈夋寚閽堛€傦紙涓嶆槸鎽囨潌椹卞姩锛侊級

```

	char name[32];

```
鐢遍┍鍔ㄨ皟鐢?gameport_set_name() 璁剧疆鐨勯┍鍔ㄥ悕绉般€備粎鐢ㄤ簬淇℃伅灞曠ず銆?

```

	char phys[32];

```
鐢遍┍鍔ㄨ皟鐢?gameport_set_phys() 璁剧疆鐨?gameport 鐗╃悊鍚嶇О/鎻忚堪銆備粎鐢ㄤ簬淇℃伅灞曠ず銆?

```

	int io;

```
鐢ㄤ簬 raw 妯″紡鐨?I/O 鍦板潃銆傚鏋滀綘鐨?gameport 鏀寔 raw 妯″紡锛屼綘蹇呴』灏嗘鎴?->read() 璁剧疆涓?
鏌愪釜鍊笺€?

```

	int speed;

```
gameport 璇诲彇鐨?raw 妯″紡閫熷害锛屼互姣忕鍗冩璇诲彇璁°€?

```

	int fuzz;

```
濡傛灉 gameport 鏀寔 cooked 妯″紡锛屽垯搴斿皢鍏惰缃负琛ㄧず鏁版嵁涓櫔澹伴噺鐨勪竴涓€笺€傚弬瑙?
gameport_pgm_cooked_mode銆?

```

	void (*trigger)(struct gameport *);

```
瑙﹀彂鍣紙Trigger锛夈€傛鍑芥暟搴旇Е鍙?ns558 鍗曟閲囨牱锛坥neshots锛夈€傚鏋滆涓?NULL锛屽垯灏嗕娇鐢?
outb(0xff, io)銆?

```

	unsigned char (*read)(struct gameport *);

```
璇诲彇鎸夐敭鍜?ns558 鍗曟閲囨牱浣嶃€傚鏋滆涓?NULL锛屽垯灏嗘敼鐢?inb(io)銆?

```

	int (*cooked_read)(struct gameport *, int *axes, int *buttons);

```
濡傛灉 gameport 鏀寔 cooked 妯″紡锛屽垯搴斿皢姝ゆ寚鍚戝叾 cooked 璇诲彇鍑芥暟銆傚畠搴斿皢 axes[0..3] 濉厖涓?
鎽囨潌鍥涗釜杞寸殑鍊硷紝骞跺皢 buttons[^0^] 濉厖涓鸿〃绀烘寜閿殑鍥涗釜浣嶃€?

```

	int (*calibrate)(struct gameport *, int *axes, int *max);

```
鐢ㄤ簬鏍″噯 ADC 纭欢鐨勫嚱鏁般€傝皟鐢ㄦ椂锛宎xes[0..3] 搴旂敱璋冪敤鑰呯敤 cooked 鏁版嵁棰勫～鍏咃紝max[0..3]
搴旂敤姣忎釜杞寸殑棰勬湡鏈€澶у€奸濉厖銆俢alibrate() 鍑芥暟搴旇缃?ADC 纭欢鐨勭伒鏁忓害锛屼娇鏈€澶у€艰兘钀藉叆
鍏堕噺绋嬶紝骞堕噸鏂拌绠?axes[] 鍊间互鍖归厤鏂扮殑鐏垫晱搴︼紝鎴栭噸鏂颁粠纭欢璇诲彇瀹冧滑浠ョ粰鍑烘湁鏁堝€笺€?

```

	int (*open)(struct gameport *, int mode);

```
open() 鏈変袱涓敤閫斻€傞鍏堬紝椹卞姩浠?raw 鎴?cooked 妯″紡鎵撳紑绔彛锛宱pen() 鍥炶皟鍙互鍐冲畾鏀寔
鍝簺妯″紡銆傚叾娆★紝璧勬簮鍒嗛厤鍙互鍦ㄦ澶勮繘琛屻€傜鍙ｄ篃鍙互鍦ㄦ澶勫惎鐢ㄣ€傚湪姝ゆ璋冪敤涔嬪墠锛実ameport
缁撴瀯浣撶殑鍏朵粬瀛楁锛堝嵆 io 鎴愬憳锛夋棤闇€鏈夋晥銆?

```

	void (*close)(struct gameport *);

```
close() 搴旈噴鏀剧敱 open 鍒嗛厤鐨勮祫婧愶紝骞跺彲鑳界鐢?gameport銆?

```

	struct timer_list poll_timer;
	unsigned int poll_interval;     /* in msecs */
	spinlock_t timer_lock;
	unsigned int poll_cnt;
	void (*poll_handler)(struct gameport *);
	struct gameport *parent, *child;
	struct gameport_driver *drv;
	struct mutex drv_mutex;		/* protects serio->drv so attributes can pin driver */
	struct device dev;
	struct list_head node;

```
渚?gameport 灞傚唴閮ㄤ娇鐢ㄣ€?

```

    };

```
绁濅娇鐢ㄦ剦蹇紒
