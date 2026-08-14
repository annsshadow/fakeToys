
## 鍚姩 Linux/LoongArch


:Author: Yanteng Si <siyanteng@loongson.cn>
:Date:   18 Nov 2022

## 浠庡紩瀵煎姞杞界▼搴忎紶閫掔粰鍐呮牳鐨勪俊鎭?


LoongArch 鏀寔 ACPI 鍜?FDT銆傞渶瑕佷紶閫掔粰鍐呮牳鐨勪俊鎭寘鎷?memmap銆乮nitrd銆佸懡浠よ锛屼互鍙婂彲閫夌殑 ACPI/FDT 琛ㄧ瓑銆?

鍐呮牳鍦?`kernel_entry` 澶勬帴鏀朵互涓嬪弬鏁帮細

      - a0 = efi_boot锛歚efi_boot` 鏄竴涓爣蹇楋紝鎸囩ず姝ゅ紩瀵肩幆澧冩槸鍚﹀畬鍏ㄧ鍚?UEFI銆?

      - a1 = cmdline锛歚cmdline` 鏄寚鍚戝唴鏍稿懡浠よ鐨勬寚閽堛€?

      - a2 = systemtable锛歚systemtable` 鎸囧悜 EFI 绯荤粺琛ㄣ€傛闃舵娑夊強鐨勬墍鏈夋寚閽堥兘鏄墿鐞嗗湴鍧€銆?

## Linux/LoongArch 鍐呮牳鏄犲儚澶撮儴


Linux/LoongArch 鍐呮牳鏄犲儚鏄?EFI 鏄犲儚銆備綔涓?PE 鏂囦欢锛屽畠浠叿鏈?
```
	u32	MZ_MAGIC                /* "MZ"锛孧S-DOS 澶撮儴 */
	u32	res0 = 0                /* 淇濈暀 */
	u64	kernel_entry            /* 鍐呮牳鍏ュ彛鐐?*/
	u64	_end - _text            /* 鍐呮牳鏄犲儚鏈夋晥澶у皬 */
	u64	load_offset             /* 鍐呮牳鏄犲儚璺?RAM 璧峰鐨勫姞杞藉亸绉?*/
	u64	res1 = 0                /* 淇濈暀 */
	u64	res2 = 0                /* 淇濈暀 */
	u64	res3 = 0                /* 淇濈暀 */
	u32	LINUX_PE_MAGIC          /* 榄旀暟 */
	u32	pe_header - _head       /* 鍒?PE 澶撮儴鐨勫亸绉?*/
```
