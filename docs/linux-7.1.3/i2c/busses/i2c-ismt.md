## 鍐呮牳椹卞姩 i2c-ismt


鏀寔鐨勯€傞厤鍣細
  - Intel S12xx 绯诲垪 SOCs

浣滆€咃細
	Bill Brown <bill.e.brown@intel.com>


### 妯″潡鍙傛暟


- bus_speed锛堟棤绗﹀彿鏁村瀷锛?
鐢ㄤ簬鏇存敼鎬荤嚎閫熷害銆傞€氬父鎬荤嚎閫熷害鐢?BIOS 璁惧畾锛屾棤闇€鏇存敼銆備絾鍦ㄨ皟璇曟湡闂达紝鏌愪簺 SMBus 鍒嗘瀽鍣ㄩ€熷害杩囨參锛屾棤娉曠洃娴嬫€荤嚎锛屽洜姝ら渶瑕佹妯″潡鍙傛暟銆傝浠?kHz 涓哄崟浣嶆寚瀹氭€荤嚎閫熷害銆?
鍙敤鐨勬€荤嚎棰戠巼璁剧疆锛?
  ====   =========
  0      鏃犲彉鍖?  80     kHz
  100    kHz
  400    kHz
  1000   kHz
  ====   =========


### 鎻忚堪


S12xx 绯诲垪 SOCs 闆嗘垚浜嗕竴瀵?SMBus 2.0 鎺у埗鍣紝涓昏闈㈠悜寰湇鍔″櫒涓庡瓨鍌ㄥ競鍦恒€?
S12xx 绯诲垪鍖呭惈涓€瀵?PCI functions銆俵spci 鐨勮緭鍑哄皢鏄剧ず锛?```
  00:13.0 System peripheral: Intel Corporation Centerton SMBus 2.0 Controller 0
  00:13.1 System peripheral: Intel Corporation Centerton SMBus 2.0 Controller 1
```
