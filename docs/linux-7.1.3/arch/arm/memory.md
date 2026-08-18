## ARM Linux 涓婄殑鍐呮牳鍐呭瓨甯冨眬


		Russell King <rmk@arm.linux.org.uk>

		     2005 骞?11 鏈?17 鏃?(2.6.15)

鏈枃妗ｆ弿杩颁簡 Linux 鍐呮牳鐢ㄤ簬 ARM 澶勭悊鍣ㄧ殑铏氭嫙鍐呭瓨甯冨眬銆傚畠鎸囧嚭浜嗗摢浜涘尯鍩熷彲渚涘钩鍙颁娇鐢紝
鍝簺鍖哄煙鐢遍€氱敤浠ｇ爜浣跨敤銆?
ARM CPU 鏈€澶氬彲瀵诲潃 4GB 铏氭嫙鍐呭瓨绌洪棿锛岃繖蹇呴』鍦ㄧ敤鎴风┖闂磋繘绋嬨€佸唴鏍镐互鍙婄‖浠惰澶囦箣闂村叡浜€?
闅忕潃 ARM 鏋舵瀯鐨勬垚鐔燂紝鏈夊繀瑕佷负鏂扮殑鍔熻兘淇濈暀鏌愪簺 VM 绌洪棿鍖哄煙锛涘洜姝ゆ湰鏂囨。鍙兘浼氶殢鐫€鏃堕棿
鎺ㄧЩ淇濈暀鏇村鐨?VM 绌洪棿銆?
=============== =============== ===============================================
Start		End		Use
=============== =============== ===============================================
ffff8000	ffffffff	copy_user_page / clear_user_page 浣跨敤銆?				瀵逛簬 SA11xx 鍜?Xscale锛岀敤浜?				寤虹珛 minicache 鏄犲皠銆?
ffff4000	ffffffff	ARMv6 鍙婃洿鏂?CPU 涓婄殑缂撳瓨鍒悕锛坈ache aliasing锛夈€?
ffff1000	ffff7fff	淇濈暀鍖恒€?				骞冲彴涓嶅緱浣跨敤姝ゅ湴鍧€鑼冨洿銆?
ffff0000	ffff0fff	CPU 鍚戦噺椤点€?				濡傛灉 CPU 鏀寔鍚戦噺閲嶅畾浣嶏紙鎺у埗
				瀵勫瓨鍣?V 浣嶏級锛屽垯 CPU 鍚戦噺鏄犲皠浜庢銆?
fffe0000	fffeffff	XScale 缂撳瓨鍒锋柊鍖哄煙銆傝繖鐢ㄤ簬
				proc-xscale.S 涓互鍒锋柊鏁翠釜鏁版嵁
				缂撳瓨銆傦紙XScale 娌℃湁 TCM銆傦級

fffe8000	fffeffff	CPU 鍐呯疆 DTCM 鐨勫钩鍙扮殑 DTCM 鏄犲皠鍖哄煙銆?
fffe0000	fffe7fff	CPU 鍐呯疆 ITCM 鐨勫钩鍙扮殑 ITCM 鏄犲皠鍖哄煙銆?
ffc80000	ffefffff	Fixmap 鏄犲皠鍖哄煙銆俧ix_to_virt() 鎻愪緵鐨?				鍦板潃灏嗕綅浜庢鍖哄煙銆?
ffc00000	ffc7ffff	淇濇姢鍖哄煙锛圙uard region锛?
ff800000	ffbfffff	鍥轰欢鎻愪緵鐨?DT blob 鐨勬案涔呫€佸浐瀹氬彧璇绘槧灏?
fee00000	feffffff	PCI I/O 绌洪棿鐨勬槧灏勩€傝繖鏄?vmalloc 绌洪棿鍐?				鐨勪竴涓潤鎬佹槧灏勩€?
VMALLOC_START	VMALLOC_END-1	vmalloc() / ioremap() 绌洪棿銆?				鐢?vmalloc/ioremap 杩斿洖鐨勫唴瀛樺皢
				琚姩鎬佹斁缃湪璇ュ尯鍩熶腑銆傛満鍣ㄧ壒瀹氱殑
				闈欐€佹槧灏勪篃閫氳繃 iotable_init() 浣嶄簬姝ゅ銆?				VMALLOC_START 鍩轰簬 high_memory 鍙橀噺鐨勫€硷紝
				VMALLOC_END 绛変簬 0xff800000銆?
PAGE_OFFSET	high_memory-1	鍐呮牳鐩存帴鏄犲皠鐨?RAM 鍖哄煙銆?				瀹冩槧灏勫钩鍙扮殑 RAM锛岄€氬父浠?1:1 鐨勫叧绯?				鏄犲皠鎵€鏈夊钩鍙?RAM銆?
PKMAP_BASE	PAGE_OFFSET-1	姘镐箙鍐呮牳鏄犲皠
				灏?HIGHMEM 椤垫槧灏勫埌鍐呮牳绌洪棿鐨?				涓€绉嶆柟寮忋€?
MODULES_VADDR	MODULES_END-1	鍐呮牳妯″潡绌洪棿
				閫氳繃 insmod 鎻掑叆鐨勫唴鏍告ā鍧椾娇鐢?				鍔ㄦ€佹槧灏勬斁缃簬姝ゃ€?
TASK_SIZE	MODULES_VADDR-1	鍚敤 KASan 鏃剁殑 KASan 褰卞瓙鍐呭瓨銆?				浠?MODULES_VADDR 鍒板唴瀛橀《绔殑
				鑼冨洿鍦ㄦ澶勪互姣忓瓧鑺傚唴瀛?1 浣嶇殑鏂瑰紡
				琚槧灏勪负褰卞瓙銆?
00001000	TASK_SIZE-1	鐢ㄦ埛绌洪棿鏄犲皠
				姣忕嚎绋嬫槧灏勯€氳繃 mmap() 绯荤粺璋冪敤
				鏀剧疆浜庢銆?
00000000	00000fff	CPU 鍚戦噺椤?/ 绌烘寚閽堥櫡闃?				涓嶆敮鎸佸悜閲忛噸鏄犲皠鐨?CPU 灏嗗叾鍚戦噺椤?				鏀剧疆浜庢銆傚唴鏍稿拰鐢ㄦ埛绌洪棿鐨?NULL 鎸囬拡
				瑙ｅ紩鐢ㄤ篃閫氳繃姝ゆ槧灏勮鎹曡幏銆?=============== =============== ===============================================

璇锋敞鎰忥紝涓庝笂杩板尯鍩熷啿绐佺殑鏄犲皠鍙兘瀵艰嚧鍐呮牳鏃犳硶鍚姩锛屾垨瀵艰嚧鍐呮牳鍦ㄨ繍琛屾椂锛堟渶缁堬級鍙戠敓 panic銆?
鐢变簬鏈潵鐨?CPU 鍙兘浼氬奖鍝嶅唴鏍告槧灏勫竷灞€锛岀敤鎴风▼搴忎笉寰楄闂叾 0x0001000 鍒?TASK_SIZE
鍦板潃鑼冨洿涔嬪浠讳綍鏈鏄犲皠鐨勫唴瀛樸€傚鏋滃笇鏈涜闂繖浜涘尯鍩燂紝蹇呴』閫氳繃 open() 涓?mmap() 鑷
寤虹珛鏄犲皠銆?