## MSR 璺熻釜浜嬩欢


x86 鍐呮牳鏀寔璺熻釜澶у鏁?MSR锛圡odel Specific Register锛屾ā鍨嬬壒瀹氬瘎瀛樺櫒锛夎闂€?
鏈夊叧 Intel 绯荤粺涓?MSR 鐨勫畾涔夛紝璇峰弬瑙?SDM锛歨ttps://www.intel.com/sdm锛堢 3 鍗凤級

鍙敤鐨勮窡韪偣锛?

/sys/kernel/tracing/events/msr/

璺熻釜 MSR 璇伙細

read_msr

  - msr: MSR 缂栧彿
  - val: 鍐欏叆鐨勫€?
  - failed: 鑻ヨ闂け璐ュ垯涓?1锛屽惁鍒欎负 0


璺熻釜 MSR 鍐欙細

write_msr

  - msr: MSR 缂栧彿
  - val: 鍐欏叆鐨勫€?
  - failed: 鑻ヨ闂け璐ュ垯涓?1锛屽惁鍒欎负 0


璺熻釜鍐呮牳涓殑 RDPMC锛?

rdpmc

```

  cat /sys/kernel/tracing/trace | decode_msr.py /usr/src/linux/include/asm/msr-index.h

```
浠ユ坊鍔犵鍙峰寲鐨?MSR 鍚嶇О銆?
