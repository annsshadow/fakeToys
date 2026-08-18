## SoundWire 涓殑闊抽娴?
闊抽娴佹槸鍦ㄤ互涓嬪璞′箣闂村缓绔嬬殑閫昏緫鎴栬櫄鎷熻繛鎺ワ細

  (1) 绯荤粺鍐呭瓨缂撳啿鍖轰笌 Codec

  (2) DSP 鍐呭瓨缂撳啿鍖轰笌 Codec

  (3) FIFO 涓?Codec

  (4) Codec 涓?Codec

閫氬父鐢?DMA 閫氶亾閫氳繃鏁版嵁閾捐矾椹卞姩銆備竴涓煶棰戞祦鍖呭惈涓€涓垨澶氫釜鏁版嵁閫氶亾銆傛祦涓殑鎵€鏈夐€氶亾蹇呴』鍏锋湁鐩稿悓鐨勯噰鏍风巼鍜岀浉鍚岀殑閲囨牱澶у皬銆?
鍋囪閫氳繃 SoundWire 鎺ュ彛鎵撳紑涓€涓叿鏈変袱涓€氶亾锛堝乏澹伴亾涓庡彸澹伴亾锛夌殑娴併€備互涓嬫槸娴佸湪 SoundWire 涓彲琛ㄧず鐨勮嫢骞叉柟寮忋€?
```

	-------------------------
	| L | R | L | R | L | R |
	-------------------------

```
绀轰緥 1锛氱敱 Master 娓叉煋銆佸寘鍚?L 鍜?R 閫氶亾鐨勭珛浣撳０娴侊紝娓叉煋鏂瑰悜浠?Master 鍒?```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|               |                                  |       1       |
	|               |                     Data Signal  |               |
	|    L  +  R    +----------------------------------+    L  +  R    |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+


```
绀轰緥 2锛氱敱 Slave 鎹曡幏銆佸寘鍚?L 鍜?R 閫氶亾鐨勭珛浣撳０娴侊紝鎹曡幏鏂瑰悜浠?Slave 鍒?```



	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|               |                                  |       1       |
	|               |                     Data Signal  |               |
	|    L  +  R    +----------------------------------+    L  +  R    |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  <-----------------------+       +---------------+



```
绀轰緥 3锛氱敱 Master 娓叉煋鐨勩€佸寘鍚?L 鍜?R 閫氶亾鐨勭珛浣撳０娴併€侺 鍜?R 閫氶亾鍒嗗埆鐢变袱涓笉鍚岀殑 Slave 鎺ユ敹銆侻aster 涓庝袱涓?Slave 涔嬮棿鐨勫叧绯诲
```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +---------+------------------------+     Slave     |
	|   Interface   |         |                        |   Interface   |
	|               |         |                        |       1       |
	|               |         |           Data Signal  |               |
	|    L  +  R    +---+------------------------------+       L       |
	|     (Data)    |   |     |    Data Direction      |     (Data)    |
	+---------------+   |     |   +------------->      +---------------+
	                    |     |
	                    |     |
	                    |     |                        +---------------+
	                    |     +----------------------> |     Slave     |
	                    |                              |   Interface   |
	                    |                              |       2       |
	                    |                              |               |
	                    +----------------------------> |       R       |
	                                                   |     (Data)    |
	                                                   +---------------+

```
绀轰緥 4锛氱敱 Master 娓叉煋鐨勩€佸寘鍚?L 鍜?R 閫氶亾鐨勭珛浣撳０娴併€侺 鍜?R 閫氶亾鍧囩敱涓や釜涓嶅悓鐨?Slave 鎺ユ敹銆侻aster 涓庝袱涓?Slave 鍧囬噰鐢ㄥ崟涓€绔彛澶勭悊
L+R銆傛瘡涓?Slave 璁惧閫氬父鍦ㄦ湰鍦板鐞?L + R 鏁版嵁锛屼竴鑸熀浜庨潤鎬侀厤缃垨鍔ㄦ€佹柟鍚戯紝骞跺彲鑳介┍鍔?```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +---------+------------------------+     Slave     |
	|   Interface   |         |                        |   Interface   |
	|               |         |                        |       1       |
	|               |         |           Data Signal  |               |
	|    L  +  R    +---+------------------------------+     L + R     |
	|     (Data)    |   |     |    Data Direction      |     (Data)    |
	+---------------+   |     |   +------------->      +---------------+
	                    |     |
	                    |     |
	                    |     |                        +---------------+
	                    |     +----------------------> |     Slave     |
	                    |                              |   Interface   |
	                    |                              |       2       |
	                    |                              |               |
	                    +----------------------------> |     L + R     |
	                                                   |     (Data)    |
	                                                   +---------------+

```
绀轰緥 5锛氬寘鍚?L 鍜?R 閫氶亾鐨勭珛浣撳０娴佺敱 Master 鐨勪袱涓笉鍚岀鍙ｆ覆鏌擄紝骞朵粎鐢?Slave 鐨勫崟涓€绔彛鎺ユ敹
```

	+--------------------+
	|                    |
	|     +--------------+                             +----------------+
	|     |             ||                             |                |
	|     |  Data Port  ||  L Channel                  |                |
	|     |      1      |------------+                 |                |
	|     |  L Channel  ||           |                 +-----+----+     |
	|     |   (Data)    ||           |   L + R Channel ||    Data |     |
	| Master  +----------+           | +---+---------> ||    Port |     |
	| Interface          |           |                 ||     1   |     |
	|     +--------------+           |                 ||         |     |
	|     |             ||           |                 +----------+     |
	|     |  Data Port  |------------+                 |                |
	|     |      2      ||  R Channel                  |     Slave      |
	|     |  R Channel  ||                             |   Interface    |
	|     |   (Data)    ||                             |       1        |
	|     +--------------+         Clock Signal        |     L  +  R    |
	|                    +---------------------------> |      (Data)    |
	+--------------------+                             |                |
							   +----------------+

```
绀轰緥 6锛氬寘鍚?L 鍜?R 閫氶亾鐨勭珛浣撳０娴佺敱 2 涓?Master 娓叉煋锛屾瘡涓?Master 娓叉煋涓€涓€氶亾锛屽苟鐢变袱涓笉鍚岀殑 Slave 鎺ユ敹锛屾瘡涓?Slave 鎺ユ敹
```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       1       |                                  |       1       |
	|               |                     Data Signal  |               |
	|       L       +----------------------------------+       L       |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       2       |                                  |       2       |
	|               |                     Data Signal  |               |
	|       R       +----------------------------------+       R       |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

```
绀轰緥 7锛氬寘鍚?L 鍜?R 閫氶亾鐨勭珛浣撳０娴佺敱 2 涓?Master 娓叉煋锛屾瘡涓?Master 娓叉煋涓や釜閫氶亾銆傛瘡涓?Slave 鎺ユ敹 L + R銆傝繖涓庣ず渚?4 鐨勫簲鐢ㄧ浉鍚岋紝鍙槸 Slave 鏀剧疆浜?```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       1       |                                  |       1       |
	|               |                     Data Signal  |               |
	|     L + R     +----------------------------------+     L + R     |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       2       |                                  |       2       |
	|               |                     Data Signal  |               |
	|     L + R     +----------------------------------+     L + R     |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

```
绀轰緥 8锛? 閫氶亾娴佺敱 2 涓?Master 娓叉煋锛屾瘡涓?Master 娓叉煋涓€涓?```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       1       |                                  |       1       |
	|               |                     Data Signal  |               |
	|    L1 + R1    +----------------------------------+    L1 + R1    |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       2       |                                  |       2       |
	|               |                     Data Signal  |               |
	|     L2 + R2   +----------------------------------+    L2 + R2    |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

```
娉?锛氬湪涓婅堪鐨勫閾捐矾鎯呭喌涓嬶紝涓轰簡鍔犻攣锛岄渶瑕佸厛鑾峰彇涓€涓叏灞€閿侊紝鐒跺悗鍐嶄緷娆￠攣瀹氬悇涓€荤嚎瀹炰緥銆備絾鍦ㄨ繖绉嶆儏鍐典笅锛岃皟鐢ㄦ柟妗嗘灦锛圓SoC DPCM锛変繚璇佸涓€寮犲０鍗′笂鐨勬祦鎿嶄綔濮嬬粓鏄覆琛屽寲鐨勩€傚洜姝や笉瀛樺湪绔炴€佹潯浠讹紝涔熷氨涓嶉渶瑕佸叏灞€閿併€?
娉?锛氫竴涓?Slave 璁惧鍙閰嶇疆涓烘帴鏀跺湪缁欏畾閾捐矾涓婁负鏌愪釜娴佷紶杈撶殑鎵€鏈夐€氶亾锛堢ず渚?4锛夛紝鎴栬€呬粎鍏朵腑涓€閮ㄥ垎鏁版嵁锛堢ず渚?3锛夈€係lave 璁惧鐨勯厤缃笉鐢?SoundWire 瀛愮郴缁?API 澶勭悊锛岃€屾槸鐢?snd_soc_dai_set_tdm_slot() API 澶勭悊銆傚钩鍙版垨鏈哄櫒椹卞姩閫氬父浼氶厤缃娇鐢ㄥ摢浜涙椂闅欙紙slot锛夈€傚浜庣ず渚?4锛屾墍鏈夎澶囧皢浣跨敤鐩稿悓鐨勬椂闅欙紱鑰屽浜庣ず渚?3锛孲lave Device1 灏嗕娇鐢ㄤ緥濡?Slot 0锛孲lave device2 浣跨敤 Slot 1銆?
娉?锛氬涓?Sink 绔彛鍙互浠?SoundWire 甯т腑鐩稿悓鐨?bitSlot 鎻愬彇鐩稿悓鐨勪俊鎭紝浣嗗涓?Source 绔彛蹇呴』閰嶇疆涓轰笉鍚岀殑 bitSlot銆傝繖涓?I2S/PCM TDM 鐨勪娇鐢ㄩ檺鍒剁浉鍚屻€?
## SoundWire 娴佺鐞嗘祦绋?
### 娴佸畾涔?
  (1) 褰撳墠娴侊紙Current stream锛夛細琚綊绫讳负闇€瑕佹墽琛?prepare銆乪nable銆乨isable銆乨e-prepare 绛夋搷浣滅殑娴併€?
  (2) 娲诲姩娴侊紙Active stream锛夛細琚綊绫讳负闄ゅ綋鍓嶆祦涔嬪銆佸凡缁忓湪鎬荤嚎涓婂浜庢椿鍔ㄧ姸鎬佺殑娴併€傛€荤嚎涓婂彲浠ュ瓨鍦ㄥ涓椿鍔ㄦ祦銆?
SoundWire 鎬荤嚎绠＄悊鍦?SoundWire 鎬荤嚎涓婃覆鏌?鎹曡幏鐨勬瘡涓祦鐨勬搷浣溿€傛湰鑺傝鏄庢€荤嚎瀵瑰湪鎬荤嚎涓婂垎閰?閲婃斁鐨勬瘡涓祦鎵€鎵ц鐨勬搷浣溿€備互涓嬫槸鎬荤嚎涓烘瘡涓煶棰戞祦缁存姢鐨勬祦鐘舵€併€?
### SoundWire 娴佺姸鎬?
```

	+-----------+     +------------+     +----------+     +----------+
	| ALLOCATED +---->| CONFIGURED +---->| PREPARED +---->| ENABLED  |
	|   STATE   |     |    STATE   |     |  STATE   |     |  STATE   |
	+-----------+     +------------+     +---+--+---+     +----+-----+
	                                         ^  ^              ^
				                 |  |              |
				               __|  |___________   |
				              |                 |  |
	                                      v                 |  v
	         +----------+           +-----+------+        +-+--+-----+
	         | RELEASED |<----------+ DEPREPARED |<-------+ DISABLED |
	         |  STATE   |           |   STATE    |        |  STATE   |
	         +----------+           +------------+        +----------+

```
娉ㄦ剰锛氫粎褰?ALSA/ASoC 灞傞潰鏀寔 INFO_PAUSE 鏍囧織鏃讹紝`SDW_STREAM_ENABLED` 涓?`SDW_STREAM_DISABLED` 涔嬮棿鐨勭姸鎬佽浆鎹㈡墠鐩稿叧銆傚悓鏍凤紝`SDW_DISABLED_STATE` 涓?`SDW_PREPARED_STATE` 涔嬮棿鐨勮浆鎹㈠彇鍐充簬 INFO_RESUME 鏍囧織銆?
娉?锛氳妗嗘灦瀹炵幇浜嗗熀鏈殑鐘舵€佽浆鎹㈡鏌ワ紝浣嗗苟涓嶄細锛堜緥濡傦級妫€鏌ヤ粠 DISABLED 鍒?ENABLED 鐨勮浆鎹㈠湪鐗瑰畾骞冲彴涓婃槸鍚︽湁鏁堛€傛绫绘祴璇曢渶瑕佸湪 ALSA/ASoC 灞傞潰娣诲姞銆?
### 娴佺姸鎬佹搷浣?
浠ヤ笅灏忚妭璇存槑浣滀负娴佺姸鎬佽浆鎹㈢殑涓€閮ㄥ垎锛屾€荤嚎鍦?Master 涓?Slave 涓婃墍鎵ц鐨勬搷浣溿€?
#### SDW_STREAM_ALLOCATED

娴佺殑鍒嗛厤鐘舵€併€傝繖鏄祦鐨勫叆鍙ｇ姸鎬併€傚湪杩涘叆姝ょ姸鎬佷箣鍓嶆墽琛岀殑鎿嶄綔锛?
  (1) 涓烘祦鍒嗛厤涓€涓祦杩愯鏃讹紙stream runtime锛夈€傛娴佽繍琛屾椂鐢ㄤ綔瀵硅娴佹墽琛岀殑鎵€鏈夋搷浣滅殑寮曠敤銆?
  (2) 鍒嗛厤骞跺垵濮嬪寲鐢ㄤ簬淇濆瓨娴佽繍琛屾椂淇℃伅鐨勮祫婧愩€傚叾淇濆瓨鎵€鏈変笌娴佺浉鍏崇殑淇℃伅锛屼緥濡傛祦绫诲瀷锛圥CM/PDM锛夊強鍙傛暟銆佷笌娴佸叧鑱旂殑 Master 涓?Slave 鎺ュ彛銆佹祦鐘舵€佺瓑銆?
涓婅堪鎵€鏈夋搷浣滄垚鍔熷悗锛屾祦鐘舵€佽璁剧疆涓?`SDW_STREAM_ALLOCATED`銆?
鎬荤嚎瀹炵幇浜嗕互涓嬬敤浜庡垎閰嶆祦鐨?API锛屾瘡涓祦闇€璋冪敤涓€娆°€傚湪 ASoC DPCM 妗嗘灦涓紝姝ゆ祦鐘舵€佸彲鑳戒笌 .startup() 鎿嶄綔鐩稿叧鑱斻€?

  int sdw_alloc_stream(char * stream_name, enum sdw_stream_type type);

SoundWire 鏍稿績鎻愪緵浜嗕竴涓?sdw_startup_stream() 杈呭姪鍑芥暟锛岄€氬父鍦?dailink .startup() 鍥炶皟鏈熼棿璋冪敤锛岀敤浜庢墽琛屾祦鍒嗛厤骞朵负杩炴帴鍒版煇涓祦鐨勬墍鏈?DAI 璁剧疆娴佹寚閽堛€?
#### SDW_STREAM_CONFIGURED

娴佺殑閰嶇疆鐘舵€併€傚湪杩涘叆姝ょ姸鎬佷箣鍓嶆墽琛岀殑鎿嶄綔锛?
  (1) 鍦?SDW_STREAM_ALLOCATED 鐘舵€佷腑涓烘祦淇℃伅鍒嗛厤鐨勮祫婧愬湪姝ゅ琚洿鏂般€傝繖鍖呮嫭娴佸弬鏁般€佷笌褰撳墠娴佸叧鑱旂殑 Master 涓?Slave 杩愯鏃朵俊鎭€?
  (2) 涓庡綋鍓嶆祦鍏宠仈鐨勬墍鏈?Master 涓?Slave 鍚戞€荤嚎鎻愪緵绔彛淇℃伅锛屽寘鎷敱 Master 涓?Slave 涓哄綋鍓嶆祦鍒嗛厤鐨勭鍙ｅ彿鍙婂叾閫氶亾鎺╃爜銆?
涓婅堪鎵€鏈夋搷浣滄垚鍔熷悗锛屾祦鐘舵€佽璁剧疆涓?`SDW_STREAM_CONFIGURED`銆?
鎬荤嚎瀹炵幇浜嗕互涓嬬敤浜?CONFIG 鐘舵€佺殑 API锛岄渶瑕佺敱涓庢祦鍏宠仈鐨勭浉搴?Master 涓?Slave 璋冪敤銆傝繖浜?API 鍙兘鐢辩浉搴旂殑 Master 涓?Slave 鍚勮皟鐢ㄤ竴娆°€傚湪 ASoC DPCM 妗嗘灦涓紝姝ゆ祦鐘舵€佷笌 .hw_params() 鎿嶄綔鐩稿叧鑱斻€?

  int sdw_stream_add_master(struct sdw_bus * bus,
		struct sdw_stream_config * stream_config,
		const struct sdw_ports_config * ports_config,
		struct sdw_stream_runtime * stream);

  int sdw_stream_add_slave(struct sdw_slave * slave,
		struct sdw_stream_config * stream_config,
		const struct sdw_ports_config * ports_config,
		struct sdw_stream_runtime * stream);


#### SDW_STREAM_PREPARED

娴佺殑鍑嗗鐘舵€併€傚湪杩涘叆姝ょ姸鎬佷箣鍓嶆墽琛岀殑鎿嶄綔锛?
  (0) 鍦ㄦ仮澶嶏紙resume锛夋搷浣滅殑鎯呭喌涓嬬渷鐣ユ楠?1 鍜?2锛屾鏃舵€荤嚎甯﹀宸茬煡銆?
  (1) 鎬荤嚎鍙傛暟锛堝甯﹀銆佸抚褰㈢姸銆佹椂閽熼鐜囷級鏍规嵁褰撳墠娴佷互鍙婃€荤嚎涓婂凡鏈夌殑娲诲姩娴佽繘琛岃绠椼€傞渶瑕侀噸鏂拌绠椾互瀹圭撼鎬荤嚎涓婄殑褰撳墠娴併€?
  (2) 鎵€鏈?Master 涓?Slave 绔彛鐨勪紶杈擄紙transport锛変笌绔彛鍙傛暟锛屾牴鎹楠?1 璁＄畻鍑虹殑甯у舰鐘朵笌鏃堕挓棰戠巼锛岄拡瀵瑰綋鍓嶆祦浠ュ強宸叉湁娲诲姩娴佽繘琛岃绠椼€?
  (3) 璁＄畻鍑虹殑鎬荤嚎涓庝紶杈撳弬鏁拌缂栫▼鍒?Master 涓?Slave 鐨勫瘎瀛樺櫒涓€傚奖瀛愬瘎瀛樺櫒锛坆anked registers锛夌殑缂栫▼鍦ㄥ鐢?bank锛堝綋鍓嶆湭浣跨敤鐨?bank锛変笂杩涜銆傚凡鏈夌殑娲诲姩娴佺殑绔彛鍦ㄥ鐢?bank锛堝綋鍓嶆湭浣跨敤鐨?bank锛変笂琚惎鐢ㄣ€傝繖鏍峰仛鏄负浜嗕笉鎵撴柇宸叉湁鐨勬椿鍔ㄦ祦銆?
  (4) 涓€鏃︽墍鏈夊€艰缂栫▼锛屾€荤嚎鍙戣捣鍒囨崲鍒板鐢?bank锛屾墍鏈夋柊缂栫▼鐨勫€煎嵆鐢熸晥銆?
  (5) 褰撳墠娴佺殑 Master 涓?Slave 绔彛閫氳繃缂栫▼ PrepareCtrl 瀵勫瓨鍣ㄨ繘琛屽噯澶囥€?
涓婅堪鎵€鏈夋搷浣滄垚鍔熷悗锛屾祦鐘舵€佽璁剧疆涓?`SDW_STREAM_PREPARED`銆?
鎬荤嚎瀹炵幇浜嗕互涓嬬敤浜?PREPARE 鐘舵€佺殑 API锛屾瘡涓祦闇€璋冪敤涓€娆°€傚湪 ASoC DPCM 妗嗘灦涓紝姝ゆ祦鐘舵€佷笌 .prepare() 鎿嶄綔鐩稿叧鑱斻€傜敱浜?.trigger() 鎿嶄綔鍙兘骞朵笉璺熼殢 .prepare()锛屽洜姝ゅ厑璁镐粠
`SDW_STREAM_PREPARED` 鐩存帴杞崲鍒?`SDW_STREAM_DEPREPARED`銆?

  int sdw_prepare_stream(struct sdw_stream_runtime * stream);


#### SDW_STREAM_ENABLED

娴佺殑浣胯兘鐘舵€併€傛暟鎹鍙ｅ湪杩涘叆姝ょ姸鎬佹椂鍚敤銆傚湪杩涘叆姝ょ姸鎬佷箣鍓嶆墽琛岀殑鎿嶄綔锛?
  (1) 鍦?SDW_STREAM_PREPARED 鐘舵€佽绠楀嚭鐨勬墍鏈夊€艰缂栫▼鍒板鐢?bank锛堝綋鍓嶆湭浣跨敤鐨?bank锛夈€傝繖鍚屾牱鍖呮嫭宸叉湁娲诲姩娴佺殑缂栫▼銆?
  (2) 褰撳墠娴佺殑鎵€鏈?Master 涓?Slave 绔彛閫氳繃缂栫▼ ChannelEn 瀵勫瓨鍣ㄥ湪澶囩敤 bank锛堝綋鍓嶆湭浣跨敤鐨?bank锛変笂鍚敤銆?
  (3) 涓€鏃︽墍鏈夊€艰缂栫▼锛屾€荤嚎鍙戣捣鍒囨崲鍒板鐢?bank锛屾墍鏈夋柊缂栫▼鐨勫€煎嵆鐢熸晥锛屽苟涓庡綋鍓嶆祦鍏宠仈鐨勭鍙ｈ鍚敤銆?
涓婅堪鎵€鏈夋搷浣滄垚鍔熷悗锛屾祦鐘舵€佽璁剧疆涓?`SDW_STREAM_ENABLED`銆?
鎬荤嚎瀹炵幇浜嗕互涓嬬敤浜?ENABLE 鐘舵€佺殑 API锛屾瘡涓祦闇€璋冪敤涓€娆°€傚湪 ASoC DPCM 妗嗘灦涓紝姝ゆ祦鐘舵€佷笌 .trigger() start 鎿嶄綔鐩稿叧鑱斻€?

  int sdw_enable_stream(struct sdw_stream_runtime * stream);

#### SDW_STREAM_DISABLED

娴佺殑绂佺敤鐘舵€併€傛暟鎹鍙ｅ湪閫€鍑烘鐘舵€佹椂绂佺敤銆傚湪杩涘叆姝ょ姸鎬佷箣鍓嶆墽琛岀殑鎿嶄綔锛?
  (1) 褰撳墠娴佺殑鎵€鏈?Master 涓?Slave 绔彛閫氳繃缂栫▼ ChannelEn 瀵勫瓨鍣ㄥ湪澶囩敤 bank锛堝綋鍓嶆湭浣跨敤鐨?bank锛変笂绂佺敤銆?
  (2) 鎬荤嚎鐨勬墍鏈夊綋鍓嶉厤缃互鍙婃椿鍔ㄦ祦琚紪绋嬪埌澶囩敤 bank锛堝綋鍓嶆湭浣跨敤鐨?bank锛夈€?
  (3) 涓€鏃︽墍鏈夊€艰缂栫▼锛屾€荤嚎鍙戣捣鍒囨崲鍒板鐢?bank锛屾墍鏈夋柊缂栫▼鐨勫€煎嵆鐢熸晥锛屽苟涓庡綋鍓嶆祦鍏宠仈鐨勭鍙ｈ绂佺敤銆?
涓婅堪鎵€鏈夋搷浣滄垚鍔熷悗锛屾祦鐘舵€佽璁剧疆涓?`SDW_STREAM_DISABLED`銆?
鎬荤嚎瀹炵幇浜嗕互涓嬬敤浜?DISABLED 鐘舵€佺殑 API锛屾瘡涓祦闇€璋冪敤涓€娆°€傚湪 ASoC DPCM 妗嗘灦涓紝姝ゆ祦鐘舵€佷笌 .trigger() stop 鎿嶄綔鐩稿叧鑱斻€?
褰撴敮鎸?INFO_PAUSE 鏍囧織鏃讹紝鍏佽鐩存帴杞崲鍒?`SDW_STREAM_ENABLED`銆?
瀵逛簬 ASoC 灏嗕娇鐢?.prepare() 鍥炶皟鐨勬仮澶嶆搷浣滐紝娴佸彲浠ヤ粠 `SDW_STREAM_DISABLED` 杞崲鍒?`SDW_STREAM_PREPARED`锛屾仮澶嶆墍鏈夊繀闇€璁剧疆锛屼絾涓嶆洿鏂板甫瀹戒笌姣旂壒鍒嗛厤銆?

  int sdw_disable_stream(struct sdw_stream_runtime * stream);


#### SDW_STREAM_DEPREPARED

娴佺殑鍘诲噯澶囩姸鎬併€傚湪杩涘叆姝ょ姸鎬佷箣鍓嶆墽琛岀殑鎿嶄綔锛?
  (1) 褰撳墠娴佺殑鎵€鏈?Master 涓?Slave 绔彛閫氳繃缂栫▼ PrepareCtrl 瀵勫瓨鍣ㄨ繘琛屽幓鍑嗗銆?
  (2) 褰撳墠娴佺殑杞借嵎甯﹀浠庢€荤嚎鎬诲甫瀹介渶姹備腑鎵ｅ噺锛屽苟閫氳繃鎵ц bank 鍒囨崲绛夋柟寮忚绠楀苟搴旂敤鏂板弬鏁般€?
涓婅堪鎵€鏈夋搷浣滄垚鍔熷悗锛屾祦鐘舵€佽璁剧疆涓?`SDW_STREAM_DEPREPARED`銆?
鎬荤嚎瀹炵幇浜嗕互涓嬬敤浜?DEPREPARED 鐘舵€佺殑 API锛屾瘡涓祦闇€璋冪敤涓€娆°€侫LSA/ASoC 娌℃湁鈥滃幓鍑嗗锛坉eprepare锛夆€濈殑姒傚康锛屽洜姝や粠姝ゆ祦鐘舵€佸埌 ALSA/ASoC 鎿嶄綔鐨勬槧灏勫彲鑳芥槸瀹炵幇鐩稿叧鐨勩€?
褰撴敮鎸?INFO_PAUSE 鏍囧織鏃讹紝娴佺姸鎬佷笌 .hw_free() 鎿嶄綔鐩稿叧鑱斺€斺€斿湪 TRIGGER_STOP 鏃朵笉浼氬幓鍑嗗璇ユ祦銆?
鍏朵粬瀹炵幇鍙兘浼氬湪 TRIGGER_STOP 鏃惰浆鎹㈠埌 `SDW_STREAM_DEPREPARED` 鐘舵€侊紝鍓嶆彁鏄畠浠渶瑕佺粡鐢?`SDW_STREAM_PREPARED` 鐘舵€佽繘琛岃浆鎹€?

  int sdw_deprepare_stream(struct sdw_stream_runtime * stream);


#### SDW_STREAM_RELEASED

娴佺殑閲婃斁鐘舵€併€傚湪杩涘叆姝ょ姸鎬佷箣鍓嶆墽琛岀殑鎿嶄綔锛?
  (1) 閲婃斁涓庡綋鍓嶆祦鍏宠仈鐨勬墍鏈?Master 涓?Slave 绔彛鐨勭鍙ｈ祫婧愩€?
  (2) 閲婃斁涓庡綋鍓嶆祦鍏宠仈鐨?Master 涓?Slave 杩愯鏃惰祫婧愩€?
  (3) 閲婃斁涓庡綋鍓嶆祦鍏宠仈鐨勬祦杩愯鏃惰祫婧愩€?
涓婅堪鎵€鏈夋搷浣滄垚鍔熷悗锛屾祦鐘舵€佽璁剧疆涓?`SDW_STREAM_RELEASED`銆?
鎬荤嚎瀹炵幇浜嗕互涓嬬敤浜?RELEASE 鐘舵€佺殑 API锛岄渶瑕佺敱涓庢祦鍏宠仈鐨勬墍鏈?Master 涓?Slave 璋冪敤銆傚湪 ASoC DPCM 妗嗘灦涓紝姝ゆ祦鐘舵€佷笌 .hw_free() 鎿嶄綔鐩稿叧鑱斻€?

  int sdw_stream_remove_master(struct sdw_bus * bus,
		struct sdw_stream_runtime * stream);
  int sdw_stream_remove_slave(struct sdw_slave * slave,
		struct sdw_stream_runtime * stream);


.shutdown() ASoC DPCM 鎿嶄綔璋冪敤浠ヤ笅鎬荤嚎 API 鏉ラ噴鏀句綔涓?ALLOCATED 鐘舵€佷竴閮ㄥ垎鍒嗛厤鐨勬祦銆?
鍦?.shutdown() 涓紝缁存姢娴佺姸鎬佺殑鏁版嵁缁撴瀯琚噴鏀俱€?

  void sdw_release_stream(struct sdw_stream_runtime * stream);

SoundWire 鏍稿績鎻愪緵浜嗕竴涓?sdw_shutdown_stream() 杈呭姪鍑芥暟锛岄€氬父鍦?dailink .shutdown() 鍥炶皟鏈熼棿璋冪敤锛岀敤浜庢竻闄よ繛鎺ュ埌鏌愪釜娴佺殑鎵€鏈?DAI 鐨勬祦鎸囬拡锛屽苟閲婃斁涓鸿娴佸垎閰嶇殑鍐呭瓨銆?
## 涓嶆敮鎸佺殑鎯呭喌

1. 鍏锋湁澶氫釜鍙楁敮鎸侀€氶亾鐨勫崟涓€绔彛涓嶈兘鐢ㄤ簬涓や釜娴佷箣闂存垨璺ㄦ祦浣跨敤銆備緥濡傦紝涓€涓叿鏈?4 涓€氶亾鐨勭鍙ｄ笉鑳界敤浜庡鐞?2 涓嫭绔嬬殑绔嬩綋澹版祦锛屽嵆浣垮湪鐞嗚涓?SoundWire 鏄彲琛岀殑銆?