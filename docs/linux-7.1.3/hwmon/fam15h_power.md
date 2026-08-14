## 鍐呮牳椹卞姩 fam15h_power


鏀寔鐨勮姱鐗囷細

- AMD Family 15h 澶勭悊鍣?
- AMD Family 16h 澶勭悊鍣?
  鍓嶇紑: 'fam15h_power'

  鎵弿鐨勫湴鍧€: PCI 绌洪棿

  鏁版嵁鎵嬪唽:

  - AMD Family 15h 澶勭悊鍣ㄧ殑 BIOS 鍜屽唴鏍稿紑鍙戣€呮寚鍗楋紙BKDG锛?  - AMD Family 16h 澶勭悊鍣ㄧ殑 BIOS 鍜屽唴鏍稿紑鍙戣€呮寚鍗楋紙BKDG锛?  - AMD64 鏋舵瀯绋嬪簭鍛樻墜鍐?绗?2 鍗凤細绯荤粺缂栫▼

Author: Andreas Herrmann <herrmann.der.user@googlemail.com>

### 鎻忚堪


1) 澶勭悊鍣?TDP锛堢儹璁捐鍔熻€楋紝Thermal design power锛?
鍦ㄧ粰瀹氱殑鍥哄畾棰戠巼鍜岀數鍘嬩笅锛屽鐞嗗櫒鐨勫姛鑰楁牴鎹墍鎵ц鐨勫伐浣滆礋杞借€屽彉鍖栥€傞檷棰濆姛鑰楋紙derated power锛?鏄繍琛岀壒瀹氬簲鐢ㄧ▼搴忔椂娑堣€楃殑鍔熺巼銆傜儹璁捐鍔熻€楋紙TDP锛夊氨鏄檷棰濆姛鑰楃殑涓€涓緥瀛愩€?
璇ラ┍鍔ㄥ厑璁搁€氳繃 TDP 绠楁硶璇诲彇鎻愪緵 AMD Family 15h 鍜?16h 澶勭悊鍣ㄥ姛鑰椾俊鎭殑瀵勫瓨鍣ㄣ€?
瀵逛簬 AMD Family 15h 鍜?16h 澶勭悊鍣紝鍙互浣跨敤涓嶅悓鐨勫鐞嗗櫒鍖楁ˉ鍔熻兘瀵勫瓨鍣ㄨ绠椾互涓嬪姛鐜囧€硷細

- BasePwrWatts:
    浠ョ摝鐗规寚瀹氬鐞嗗櫒涓?NB 鍜屾牳蹇冨閮ㄩ€昏緫娑堣€楃殑鏈€澶у姛鐜囥€?
- ProcessorPwrWatts:
    浠ョ摝鐗规寚瀹氬鐞嗗櫒鍙互鏀寔鐨勬渶澶у姛鐜囥€?- CurrPwrWatts:
    浠ョ摝鐗规寚瀹氬鐞嗗櫒褰撳墠姝ｅ湪娑堣€楃殑鍔熺巼銆?
璇ラ┍鍔ㄦ彁渚?ProcessorPwrWatts 鍜?CurrPwrWatts锛?
- power1_crit (ProcessorPwrWatts)
- power1_input (CurrPwrWatts)

鍦ㄥ鑺傜偣澶勭悊鍣ㄤ笂锛岃绠楀€兼槸閽堝鏁翠釜灏佽锛坧ackage锛夌殑锛岃€屼笉鏄拡瀵瑰崟涓妭鐐广€傚洜姝よ椹卞姩浠呬负
澶氳妭鐐瑰鐞嗗櫒鐨勫唴閮?node0 鍒涘缓 sysfs 灞炴€с€?
2) 绱Н鍔熺巼鏈哄埗

璇ラ┍鍔ㄨ繕寮曞叆浜嗕竴绉嶇畻娉曪紝鐢ㄤ簬璁＄畻澶勭悊鍣ㄥ湪娴嬮噺闂撮殧 Tm 鍐呮秷鑰楃殑骞冲潎鍔熺巼銆傜疮绉姛鐜囨満鍒剁殑鐗规€х敱
CPUID Fn8000_0007_EDX[^12^] 鎸囩ず銆?
- Tsample:
	璁＄畻鍗曞厓鍔熺巼绱姞鍣ㄩ噰鏍峰懆鏈?
- Tref:
	PTSC 璁℃暟鍣ㄥ懆鏈?
- PTSC:
	鎬ц兘鏃堕棿鎴宠鏁板櫒

- N:
	璁＄畻鍗曞厓鍔熺巼绱姞鍣ㄩ噰鏍峰懆鏈熶笌 PTSC 鍛ㄦ湡鐨勬瘮鐜?
- Jmax:
	鏈€澶ц绠楀崟鍏冪疮绉姛鐜囷紝鐢?MaxCpuSwPwrAcc MSR C001007b 鎸囩ず

- Jx/Jy:
	璁＄畻鍗曞厓绱Н鍔熺巼锛岀敱 CpuSwPwrAcc MSR C001007a 鎸囩ず
- Tx/Ty:
	鎬ц兘鏃堕棿鎴宠鏁板櫒鐨勫€硷紝鐢?CU_PTSC MSR C0010280 鎸囩ず

- PwrCPUave:
	CPU 骞冲潎鍔熺巼

i. 鎵ц CPUID Fn8000_0007 浠ョ‘瀹?Tsample 涓?Tref 鐨勬瘮鐜囥€?
	N = CPUID Fn8000_0007_ECX[CpuPwrSampleTimeRatio[15:0]] 鐨勫€笺€?
ii. 浠庢柊鐨?MSR MaxCpuSwPwrAcc 璇诲彇绱Н鑳介噺鍊肩殑瀹屾暣鑼冨洿銆?
	Jmax = 杩斿洖鐨勫€笺€?
iii. 鍦ㄦ椂鍒?x锛孲W 璇诲彇 CpuSwPwrAcc MSR 骞堕噰鏍?PTSC銆?
	Jx = 浠?CpuSwPwrAcc 璇诲彇鐨勫€硷紝Tx = 浠?PTSC 璇诲彇鐨勫€笺€?
iv. 鍦ㄦ椂鍒?y锛孲W 璇诲彇 CpuSwPwrAcc MSR 骞堕噰鏍?PTSC銆?
	Jy = 浠?CpuSwPwrAcc 璇诲彇鐨勫€硷紝Ty = 浠?PTSC 璇诲彇鐨勫€笺€?
v. 璁＄畻涓€涓绠楀崟鍏冨湪涓€娈垫椂闂村唴鐨勫钩鍧囧姛鑰?```

	if (Jy < Jx) // 鍙戠敓浜嗗洖缁?		Jdelta = (Jy + Jmax) - Jx
	else
		Jdelta = Jy - Jx
	PwrCPUave = N * Jdelta * 1000 / (Ty - Tx)

```
璇ラ┍鍔ㄦ彁渚?PwrCPUave 鍜岄棿闅旓紙榛樿涓?10 姣锛屾渶澶т负 1 绉掞級锛?
- power1_average (PwrCPUave)
- power1_average_interval (Interval)

power1_average_interval 鍙互鍦?/etc/sensors3.conf 鏂囦欢涓洿鏂帮紝濡備笅鎵€绀猴細

chip `fam15h_power-*`
	set power1_average_interval 0.01

鐒跺悗浣跨敤 鈥渟ensors -s鈥?淇濆瓨瀹冦€?