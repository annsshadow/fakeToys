## 涓插彛 UART 16450/16550 MIDI 椹卞姩


adaptor 妯″潡鍙傛暟鍏佽浣犻€夋嫨浠ヤ笅涔嬩竴锛?

- 0 - Roland Soundcanvas 鏀寔锛堥粯璁わ級
- 1 - Midiator MS-124T 鏀寔锛?锛?
- 2 - Midiator MS-124W S/A 妯″紡锛?锛?
- 3 - MS-124W M/B 妯″紡鏀寔锛?锛?
- 4 - 鏀寔澶氳緭鍏ョ殑閫氱敤璁惧锛?锛?

瀵逛簬 Midiator MS-124W锛屼綘蹇呴』灏?Midiator 涓婄殑鐗╃悊 M-S 涓?A-B 寮€鍏宠缃緱涓庝綘鎵€閫夋嫨鐨勯┍鍔ㄦā寮忕浉鍖归厤銆?

鍦?Roland Soundcanvas 妯″紡涓嬶紝鏀寔澶氫釜 ALSA raw MIDI 瀛愭祦锛坢idiCnD0-midiCnD15锛夈€傛瘡褰撲綘鍐欏叆涓€涓笉鍚岀殑瀛愭祦鏃讹紝椹卞姩浼氬彂閫侀潪鏍囧噯鐨?MIDI 鍛戒护搴忓垪 F5 NN锛屽叾涓?NN 涓哄瓙娴佺紪鍙峰姞 1銆俁oland 妯″潡浣跨敤姝ゅ懡浠ゅ湪涓嶅悓鈥滃０閮ㄢ€濓紙part锛変箣闂村垏鎹紝鍥犳璇ョ壒鎬ц浣犲彲浠ュ皢姣忎釜澹伴儴褰撲綔涓€涓嫭绔嬬殑 raw MIDI 瀛愭祦瀵瑰緟銆傞┍鍔ㄦ病鏈夋彁渚涘彂閫?F5 00锛堜笉閫夋嫨锛夋垨涓嶅彂閫?F5 NN 鍛戒护搴忓垪鐨勬柟寮忥紱鎴栬搴斿綋鎻愪緵銆?

绠€鍗曚覆鍙ｈ浆鎹㈠櫒鐨勪娇鐢ㄧず渚嬶細
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 speed=115200

```
甯?4 涓?MIDI 绔彛鐨?Roland SoundCanvas 浣跨敤绀轰緥锛?
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 outs=4

```
鍦?MS-124T 妯″紡涓嬶紝鏀寔涓€涓?raw MIDI 瀛愭祦锛坢idiCnD0锛夛紱outs 妯″潡鍙傛暟浼氳嚜鍔ㄨ涓?1銆傞┍鍔ㄥ皢鐩稿悓鐨勬暟鎹彂閫佸埌鍏ㄩ儴鍥涗釜 MIDI Out 鎺ュ彛銆傚皢 A-B 寮€鍏冲拰 speed 妯″潡鍙傛暟璁剧疆涓哄尮閰嶏紙A=19200锛孊=9600锛夈€?

A-B 寮€鍏冲浜?A 浣嶇殑 MS-124T 浣跨敤绀轰緥锛?
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 adaptor=1 \
			speed=19200

```
鍦?MS-124W S/A 妯″紡涓嬶紝鏀寔涓€涓?raw MIDI 瀛愭祦锛坢idiCnD0锛夛紱outs 妯″潡鍙傛暟浼氳嚜鍔ㄨ涓?1銆傞┍鍔ㄤ互鍏ㄩ€?MIDI 閫熺巼灏嗙浉鍚岀殑鏁版嵁鍙戦€佸埌鍏ㄩ儴鍥涗釜 MIDI Out 鎺ュ彛銆?

S/A 妯″紡浣跨敤绀轰緥锛?
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 adaptor=2

```
鍦?MS-124W M/B 妯″紡涓嬶紝椹卞姩鏀寔 16 涓?ALSA raw MIDI 瀛愭祦锛沷uts 妯″潡鍙傛暟浼氳嚜鍔ㄨ涓?16銆傚瓙娴佺紪鍙风粰鍑烘暟鎹簲鍙戦€佸埌鐨?MIDI Out 鎺ュ彛鐨勪綅鎺╃爜锛屽叾涓?midiCnD1 鍙戦€佸埌 Out 1锛宮idiCnD2 鍒?Out 2锛宮idiCnD4 鍒?Out 3锛宮idiCnD8 鍒?Out 4銆傚洜姝?midiCnD15 灏嗘暟鎹彂閫佸埌鍏ㄩ儴 4 涓鍙ｃ€備綔涓轰竴绉嶇壒娈婃儏鍐碉紝midiCnD0 涔熶細鍙戦€佸埌鎵€鏈夌鍙ｏ紝鍥犱负鍚戞棤绔彛鍙戦€佹暟鎹苟鏃犵敤澶勩€侻/B 妯″紡鏈夐澶栧紑閿€鏉ヤ负姣忎釜瀛楄妭閫夋嫨 MIDI Out锛屽洜姝ゅ洓涓?MIDI Out 涓婄殑鎬绘暟鎹€熺巼鏈€澶氫负姣忎釜瀛楄妭 520 寰涓€娆★紝鑰屽叏閫?MIDI 鏁版嵁閫熺巼涓烘瘡绔彛姣忓瓧鑺?320 寰涓€娆°€?

M/B 妯″紡浣跨敤绀轰緥锛?
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 adaptor=3

```
MS-124W 纭欢鐨?M/A 妯″紡鐩墠涓嶅彈鏀寔銆傝妯″紡鍏佽 MIDI Out 浠?M/B 涓ゅ€嶇殑鎬诲悶鍚愮嫭绔嬪伐浣滐紝浣嗕笉鍏佽灏嗗悓涓€瀛楄妭鍚屾椂鍙戦€佸埌澶氫釜 MIDI Out銆侻/A 鍗忚瑕佹眰椹卞姩鍦ㄦ椂搴忕害鏉熶笅鎷ㄥ姩璋冨埗瑙ｈ皟鍣ㄦ帶鍒剁嚎锛屽洜姝ゅ疄鐜拌捣鏉ユ瘮鍏朵粬妯″紡绋嶅鏉傘€?

闄?MS-124W 鍜?MS-124T 涔嬪鐨?Midiator 鍨嬪彿鐩墠涓嶅彈鏀寔銆傝娉ㄦ剰鍚庣紑瀛楁瘝鏄湁鎰忎箟鐨勶紱MS-124 涓?MS-124B 涓嶅吋瀹癸紝鍏朵粬宸茬煡鍨嬪彿 MS-101銆丮S-101B銆丮S-103 鍜?MS-114 涔嬮棿涔熶笉鍏煎銆傛垜鎵嬪ご鏈夛紙tim.mann@compaq.com锛夐儴鍒嗘兜鐩栬繖浜涘瀷鍙风殑鏂囨。锛屼絾娌℃湁鍙緵璇曢獙鐨勫疄鐗┿€侻S-124W 鏀寔宸茬敤鐪熷疄璁惧娴嬭瘯杩囥€侻S-124T 鏀寔鏈粡娴嬭瘯锛屼絾搴斿綋鍙敤銆?

閫氱敤椹卞姩閫氳繃鍗曚釜涓插彛鏀寔澶氫釜杈撳叆鍜岃緭鍑哄瓙娴併€備笌 Roland Soundcanvas 妯″紡绫讳技锛屼娇鐢?F5 NN 鏉ラ€夋嫨閫傚綋鐨勮緭鍏ユ垨杈撳嚭娴侊紙鍙栧喅浜庢暟鎹柟鍚戯級銆傛澶栵紝CTS 淇″彿鐢ㄤ簬璋冭妭鏁版嵁娴併€傝緭鍏ョ殑鏁伴噺鐢?ins 鍙傛暟鎸囧畾銆?
