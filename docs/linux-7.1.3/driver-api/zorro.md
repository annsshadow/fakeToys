## 涓?Zorro 璁惧缂栧啓璁惧椹卞姩


:Author: Written by Geert Uytterhoeven <geert@linux-m68k.org>
:Last revised: September 5, 2003


### 绠€浠?

Zorro 鎬荤嚎鏄?Amiga 绯诲垪璁＄畻鏈轰腑浣跨敤鐨勬€荤嚎銆傚緱鐩婁簬 AutoConfig(tm)锛屽畠鏄?100%
鍗虫彃鍗崇敤锛圥lug-and-Play锛夌殑銆?
Zorro 鎬荤嚎鏈変袱绉嶇被鍨嬶紝Zorro II 涓?Zorro III锛?
  - Zorro II 鍦板潃绌洪棿鏄?24 浣嶇殑锛屼綅浜?Amiga 鍦板潃鏄犲皠鐨勫墠 16 MB 鍐呫€?
  - Zorro III 鏄?Zorro II 鐨?32 浣嶆墿灞曪紝鍚戝悗鍏煎 Zorro II銆俍orro III 鍦板潃绌洪棿
    浣嶄簬鍓?16 MB 涔嬪銆?

### 鎺㈡祴 Zorro 璁惧


閫氳繃璋冪敤 `zorro_find_device()` 鏉ュ彂鐜?Zorro 璁惧锛岃鍑芥暟杩斿洖鎸囧悜鍏锋湁鎸囧畾 Zorro ID
鐨刞涓嬩竴涓猔 Zorro 璁惧鐨勬寚閽堛€傛帰娴嬪惊鐜涓嬶細

```

    struct zorro_dev *z = NULL;

    while ((z = zorro_find_device(ZORRO_PROD_xxx, z))) {
	if (!zorro_request_region(z->resource.start+MY_START, MY_SIZE,
				  "My explanation"))
	...
    }

```
`ZORRO_WILDCARD` 鍏呭綋閫氶厤绗︼紝鍙互鎵惧埌浠绘剰 Zorro 璁惧銆傚鏋滀綘鐨勯┍鍔ㄥ涓嬶細

```

    struct zorro_dev *z = NULL;

    while ((z = zorro_find_device(ZORRO_WILDCARD, z))) {
	if (z->id != ZORRO_PROD_xxx1 && z->id != ZORRO_PROD_xxx2 && ...)
	    continue;
	if (!zorro_request_region(z->resource.start+MY_START, MY_SIZE,
				  "My explanation"))
	...
    }


```
### Zorro 璧勬簮


鍦ㄤ綘璁块棶 Zorro 璁惧鐨勫瘎瀛樺櫒涔嬪墠锛屽繀椤荤‘淇濆畠灏氭湭琚娇鐢ㄣ€傝繖鏄€氳繃 I/O 鍐呭瓨绌洪棿
璧勬簮绠＄悊瀹屾垚鐨勶細

```

    request_mem_region()
    release_mem_region()

```
```

    zorro_request_device
    zorro_release_device


```
### 璁块棶 Zorro 鍦板潃绌洪棿


Zorro 璁惧璧勬簮涓殑鍦板潃鍖哄煙鏄?Zorro 鎬荤嚎鍦板潃鍖哄煙銆傜敱浜?Zorro 鎬荤嚎涓婃€荤嚎-鐗╃悊鍦板潃鐨?鎭掔瓑鏄犲皠锛屽畠浠悓鏃朵篃鏄?CPU 鐗╃悊鍦板潃銆?
瀵硅繖浜涘尯鍩熺殑澶勭悊鍙栧喅浜?Zorro 绌洪棿鐨勭被鍨嬶細

  - Zorro II 鍦板潃绌洪棿鎬绘槸琚槧灏勭殑锛屼笉闇€瑕佷娇鐢?z_ioremap() 鏄惧紡鏄犲皠銆?
    浠庢€荤嚎/鐗╃悊 Zorro II 鍦板潃鍒板唴鏍歌櫄鎷熷湴鍧€鐨勮浆鎹細

```

	virt_addr = ZTWO_VADDR(bus_addr);
	bus_addr = ZTWO_PADDR(virt_addr);

  - Zorro III 鍦板潃绌洪棿蹇呴』鍏堜娇鐢?z_ioremap() 鏄惧紡鏄犲皠锛岀劧鍚庢墠鑳借闂?:

	virt_addr = z_ioremap(bus_addr, size);
	...
	z_iounmap(virt_addr);


```
### 鍙傝€冭祫鏂?

#. linux/include/linux/zorro.h
#. linux/include/uapi/linux/zorro.h
#. linux/include/uapi/linux/zorro_ids.h
#. linux/arch/m68k/include/asm/zorro.h
#. linux/drivers/zorro
#. /proc/bus/zorro
