## Linux 鍗虫彃鍗崇敤锛圥lug and Play锛夋枃妗?


:Author: Adam Belay <ambx1@neo.rr.com>
:Last updated: Oct. 16, 2002


### 姒傝堪


鍗虫彃鍗崇敤锛圥lug and Play锛夋彁渚涗簡涓€绉嶆娴嬪苟涓轰紶缁熻澶囨垨鍏朵粬涓嶅彲閰嶇疆璁惧
璁剧疆璧勬簮鐨勬墜娈点€侺inux 鍗虫彃鍗崇敤灞傚悜鍏煎鐨勯┍鍔ㄦ彁渚涜繖浜涙湇鍔°€?


### 鐢ㄦ埛鐣岄潰


Linux 鍗虫彃鍗崇敤鐨勭敤鎴风晫闈负閭ｄ簺涓嶆敮鎸?Linux 鍗虫彃鍗崇敤鐨勪紶缁熼┍鍔ㄤ笌鐢ㄦ埛鎬侀┍鍔?
鎻愪緵浜嗕竴绉嶆縺娲?PnP 璁惧鐨勬墜娈点€傝鐢ㄦ埛鐣岄潰闆嗘垚鍦?sysfs 涓€?

闄や簡鏍囧噯鐨?sysfs 鏂囦欢澶栵紝杩樹細鍦ㄦ瘡涓澶囩殑鐩綍涓嬪垱寤轰互涓嬫枃浠讹細
- id 鈥斺€?鏄剧ず鎵€鏀寔鐨?EISA ID 鍒楄〃
- options 鈥斺€?鏄剧ず鍙兘鐨勮祫婧愰厤缃?
- resources 鈥斺€?鏄剧ず褰撳墠宸插垎閰嶇殑璧勬簮锛屽苟鍏佽鏇存敼璧勬簮

##### 婵€娲昏澶?

```

	# echo "auto" > resources

```
杩欏皢璋冪敤鑷姩璧勬簮閰嶇疆绯荤粺鏉ユ縺娲昏璁惧

##### 鎵嬪姩婵€娲昏澶?

```

	# echo "manual <depnum> <mode>" > resources

	<depnum> - the configuration number
	<mode> - static or dynamic
		 static = for next boot
		 dynamic = now

```
##### 绂佺敤璁惧

```

	# echo "disable" > resources


```
绀轰緥锛?

鍋囪浣犻渶瑕佹縺娲昏蒋鐩樻帶鍒跺櫒銆?

1. 鍒囨崲鍒版纭殑鐩綍锛屽湪鎴戣繖閲岃鐩綍涓?

```

	# cd /driver/bus/pnp/devices/00:0f
	# cat name
	PC standard floppy disk controller

```
```

	# cat resources
	DISABLED

  - Notice the string "DISABLED".  This means the device is not active.

```
```

	# cat options
	Dependent: 01 - Priority acceptable
	    port 0x3f0-0x3f0, align 0x7, size 0x6, 16-bit address decoding
	    port 0x3f7-0x3f7, align 0x0, size 0x1, 16-bit address decoding
	    irq 6
	    dma 2 8-bit compatible
	Dependent: 02 - Priority acceptable
	    port 0x370-0x370, align 0x7, size 0x6, 16-bit address decoding
	    port 0x377-0x377, align 0x0, size 0x1, 16-bit address decoding
	    irq 6
	    dma 2 8-bit compatible

```
```

	# echo "auto" > resources

```
```

	# cat resources
	io 0x3f0-0x3f5
	io 0x3f7-0x3f7
	irq 6
	dma 2

```
```

	pnp_reserve_irq=irq1[,irq2] ....
	pnp_reserve_dma=dma1[,dma2] ....
	pnp_reserve_io=io1,size1[,io2,size2] ....
	pnp_reserve_mem=mem1,size1[,mem2,size2] ....



```
### 缁熶竴鐨勫嵆鎻掑嵆鐢ㄥ眰


鎵€鏈夊嵆鎻掑嵆鐢ㄩ┍鍔ㄣ€佸崗璁笌鏈嶅姟閮藉湪涓€涓О涓衡€滃嵆鎻掑嵆鐢ㄥ眰鈥濈殑涓績浣嶇疆姹囧悎銆傝灞?
璐熻矗鍦?PnP 椹卞姩涓?PnP 鍗忚涔嬮棿浜ゆ崲淇℃伅锛屽洜姝や細鑷姩灏嗗懡浠よ浆鍙戠粰鐩稿簲鐨勫崗璁€?
杩欎娇寰楃紪鍐?PnP 椹卞姩鍙樺緱瀹规槗寰楀銆?

鍗虫彃鍗崇敤灞傛彁渚涗互涓嬪嚱鏁帮細

pnp_get_protocol
  灏嗕娇鐢ㄨ鏁板姞涓€

pnp_put_protocol
  灏嗕娇鐢ㄨ鏁板噺涓€

pnp_register_protocol
  鐢ㄤ簬娉ㄥ唽涓€涓柊鐨?PnP 鍗忚

pnp_register_driver
  灏嗕竴涓?PnP 椹卞姩娣诲姞鍒板嵆鎻掑嵆鐢ㄥ眰

  鍏朵腑鍖呭惈椹卞姩妯″瀷鐨勯泦鎴?
  鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鍙凤紱鑻ヤ綘鎯充簡瑙ｆ湁澶氬皯涓澶囩粦瀹氬埌璇ラ┍鍔紝鍙粺璁″ .add() 鏂规硶鐨勮皟鐢ㄦ鏁?

pnp_unregister_driver
  浠庡嵆鎻掑嵆鐢ㄥ眰涓Щ闄や竴涓?PnP 椹卞姩



### 鍗虫彃鍗崇敤鍗忚


鏈妭闈㈠悜 PnP 鍗忚寮€鍙戣€呮彁渚涚浉鍏充俊鎭€?

褰撳墠璁＄畻涓栫晫涓彲鐢ㄧ殑鍗忚濡備笅锛?

- PNPBIOS:
    鐢ㄤ簬涓插彛銆佸苟鍙ｇ瓑绯荤粺璁惧銆?
- ISAPNP:
    涓?ISA 鎬荤嚎鎻愪緵 PnP 鏀寔
- ACPI:
    鍦ㄥ叾浼楀鐢ㄩ€斾腑锛孉CPI 鎻愪緵鍏充簬绯荤粺绾ц澶囩殑淇℃伅銆?

瀹冩棬鍦ㄥ彇浠?PNPBIOS銆侺inux 鍗虫彃鍗崇敤鐩墠灏氭湭鏀寔瀹冿紝浣嗚鍒掑湪涓嶄箙鐨勫皢鏉ュ疄鐜般€?


Linux PnP 鍗忚鐨勮姹傦細
1. 鍗忚蹇呴』浣跨敤 EISA ID
2. 鍗忚蹇呴』鍚?PnP 灞傛姤鍛婅澶囧綋鍓嶇殑閰嶇疆

- 璁剧疆璧勬簮鐨勮兘鍔涙槸鍙€夌殑锛屼絾鎺ㄨ崘浣跨敤銆?

浠ヤ笅鏄笌 PnP 鍗忚鐩稿叧鐨勫嚱鏁帮細

pnp_add_device
  浣跨敤姝ゅ嚱鏁板皢涓€涓?PnP 璁惧娣诲姞鍒?PnP 灞?

  浠呭綋 pnp_dev 缁撴瀯涓殑鎵€鏈夋湡鏈涘瓧娈甸兘宸茶缃椂鎵嶈皟鐢ㄦ鍑芥暟

pnp_init_device
  璋冪敤瀹冩潵鍒濆鍖?PnP 缁撴瀯

pnp_remove_device
  璋冪敤瀹冧粠鍗虫彃鍗崇敤灞傜Щ闄よ澶囥€?
  鑻ヨ澶囦粛鍦ㄤ娇鐢ㄥ垯浼氬け璐ャ€?
  浼氳嚜鍔ㄩ噴鏀捐澶囧強鐩稿叧缁撴瀯鎵€鍗犵敤鐨勫唴瀛?

pnp_add_id
  灏嗕竴涓?EISA ID 娣诲姞鍒版寚瀹氳澶囨墍鏀寔鐨?ID 鍒楄〃涓?

鏇村淇℃伅璇峰弬鑰冩煇涓崗璁殑婧愮爜锛屼緥濡?
/drivers/pnp/pnpbios/core.c銆?



### Linux 鍗虫彃鍗崇敤椹卞姩


鏈妭闈㈠悜 Linux PnP 椹卞姩寮€鍙戣€呮彁渚涚浉鍏充俊鎭€?

##### 鏂版柟寮?


1. 棣栧厛鍒楀嚭鎵€鏀寔鐨?EISA ID

```

	static const struct pnp_id pnp_dev_table[] = {
		/* Standard LPT Printer Port */
		{.id = "PNP0400", .driver_data = 0},
		/* ECP Printer Port */
		{.id = "PNP0401", .driver_data = 0},
		{.id = ""}
	};

   Please note that the character 'X' can be used as a wild card in the function
   portion (last four characters).

   ex::

	/* Unknown PnP modems */
	{	"PNPCXXX",		UNKNOWN_DEV	},

   Supported PnP card IDs can optionally be defined.
   ex::

	static const struct pnp_id pnp_card_table[] = {
		{	"ANYDEVS",		0	},
		{	"",			0	}
	};

```
2. 鍙€夊湴瀹氫箟 probe 涓?remove 鍑芥暟銆傚鏋滈┍鍔ㄥ凡缁忔嫢鏈夊彲闈犵殑璧勬簮妫€娴嬫柟娉曪紙渚嬪 parport_pc 椹卞姩锛夛紝
   涓嶅畾涔夎繖浜涘嚱鏁版槸鍚堢悊鐨勩€?

```

	static int
	serial_pnp_probe(struct pnp_dev * dev, const struct pnp_id *card_id, const
			struct pnp_id *dev_id)
	{
	. . .

   ex::

	static void serial_pnp_remove(struct pnp_dev * dev)
	{
	. . .

   consult /drivers/serial/8250_pnp.c for more information.

```
3. 鍒涘缓椹卞姩缁撴瀯

```

	static struct pnp_driver serial_pnp_driver = {
		.name		= "serial",
		.card_id_table	= pnp_card_table,
		.id_table	= pnp_dev_table,
		.probe		= serial_pnp_probe,
		.remove		= serial_pnp_remove,
	};

   * name and id_table cannot be NULL.

```
4. 娉ㄥ唽椹卞姩

```

	static int __init serial8250_pnp_init(void)
	{
		return pnp_register_driver(&serial_pnp_driver);
	}

```
##### 鏃ф柟寮?


鍒涘缓浜嗕竴绯诲垪鍏煎鍑芥暟锛屼互渚夸簬杞崲 ISAPNP 椹卞姩銆傚畠浠彧搴斾綔涓轰复鏃舵柟妗堜娇鐢ㄣ€?

```

	struct pnp_dev *pnp_find_dev(struct pnp_card *card,
				     unsigned short vendor,
				     unsigned short function,
				     struct pnp_dev *from)


```