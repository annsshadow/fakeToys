
## 鏈畾涔夎涓烘秷姣掑墏 鈥?UBSAN


UBSAN 鏄竴涓繍琛屾湡鏈畾涔夎涓烘鏌ュ櫒銆?
UBSAN 浣跨敤缂栬瘧鏈熸彃妗╂潵鎹曡幏鏈畾涔夎涓猴紙UB锛夈€傜紪璇戝櫒鍦ㄥ彲鑳藉紩鍙?UB 鐨勬搷浣滀箣鍓嶆彃鍏ユ墽琛?鐗瑰畾绫诲瀷妫€鏌ョ殑浠ｇ爜銆傚鏋滄鏌ュけ璐ワ紙鍗虫娴嬪埌 UB锛夛紝浼氳皟鐢?__ubsan_handle_* 鍑芥暟
鏉ユ墦鍗伴敊璇俊鎭€?
GCC 浠?4.9.x [1_] 寮€濮嬪叿鏈夎鐗规€э紙鍙傝 `-fsanitize=undefined` 閫夐」鍙婂叾瀛愰€夐」锛夈€?GCC 5.x 瀹炵幇浜嗘洿澶氭鏌ュ櫒 [2_]銆?
### 鎶ュ憡绀轰緥


```

	 ================================================================================
	 UBSAN: Undefined behaviour in ../include/linux/bitops.h:110:33
	 shift exponent 32 is to large for 32-bit type 'unsigned int'
	 CPU: 0 PID: 0 Comm: swapper Not tainted 4.4.0-rc1+ #26
	  0000000000000000 ffffffff82403cc8 ffffffff815e6cd6 0000000000000001
	  ffffffff82403cf8 ffffffff82403ce0 ffffffff8163a5ed 0000000000000020
	  ffffffff82403d78 ffffffff8163ac2b ffffffff815f0001 0000000000000002
	 Call Trace:
	  [<ffffffff815e6cd6>] dump_stack+0x45/0x5f
	  [<ffffffff8163a5ed>] ubsan_epilogue+0xd/0x40
	  [<ffffffff8163ac2b>] __ubsan_handle_shift_out_of_bounds+0xeb/0x130
	  [<ffffffff815f0001>] ? radix_tree_gang_lookup_slot+0x51/0x150
	  [<ffffffff8173c586>] _mix_pool_bytes+0x1e6/0x480
	  [<ffffffff83105653>] ? dmi_walk_early+0x48/0x5c
	  [<ffffffff8173c881>] add_device_randomness+0x61/0x130
	  [<ffffffff83105b35>] ? dmi_save_one_device+0xaa/0xaa
	  [<ffffffff83105653>] dmi_walk_early+0x48/0x5c
	  [<ffffffff831066ae>] dmi_scan_machine+0x278/0x4b4
	  [<ffffffff8111d58a>] ? vprintk_default+0x1a/0x20
	  [<ffffffff830ad120>] ? early_idt_handler_array+0x120/0x120
	  [<ffffffff830b2240>] setup_arch+0x405/0xc2c
	  [<ffffffff830ad120>] ? early_idt_handler_array+0x120/0x120
	  [<ffffffff830ae053>] start_kernel+0x83/0x49a
	  [<ffffffff830ad120>] ? early_idt_handler_array+0x120/0x120
	  [<ffffffff830ad386>] x86_64_start_reservations+0x2a/0x2c
	  [<ffffffff830ad4f3>] x86_64_start_kernel+0x16b/0x17a
	 ================================================================================

```
### 浣跨敤


```

  CONFIG_UBSAN=y

```
```

  UBSAN_SANITIZE_main.o := n

```
```

  UBSAN_SANITIZE := n

```
```

  UBSAN_SANITIZE_main.o := y

```
瀵规湭瀵归綈璁块棶鐨勬娴嬬敱鍗曠嫭鐨勯€夐」 CONFIG_UBSAN_ALIGNMENT 鎺у埗銆傚湪鏀寔鏈榻愯闂殑
鏋舵瀯涓婏紙CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS=y锛夛紝瀹冮粯璁ゅ叧闂€備粛鐒跺彲浠ュ湪閰嶇疆涓?鍚敤瀹冿紝鍙槸瑕佹敞鎰忓畠浼氫骇鐢熷ぇ閲?UBSAN 鎶ュ憡銆?
### 鍙傝€冭祫鏂?