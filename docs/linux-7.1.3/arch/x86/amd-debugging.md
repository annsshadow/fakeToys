
Debugging AMD Zen systems
+++++++++++++++++++++++++

## 绠€浠?

鏈枃妗ｆ弿杩颁簡鍙敤浜庤皟璇?AMD Zen 绯荤粺闂鐨勬妧鏈€傚畠闈㈠悜寮€鍙戣€呭拰鎶€鏈汉鍛橈紝浠ュ府鍔╀粬浠瘑鍒拰瑙ｅ喅闂銆?
## S3 涓?s2idle


鍦?AMD 绯荤粺涓婏紝鏃犳硶鍚屾椂鏀寔鎸傝捣鍒?RAM锛圫3锛夊拰鎸傝捣鍒扮┖闂诧紙s2idle锛夈€傝纭浣犵殑绯荤粺鏀寔鍝妯″紡锛屽彲浠ユ煡鐪?`cat /sys/power/mem_sleep`銆傚鏋滃畠鏄剧ず `s2idle [deep]`锛屽垯鏀寔 **S3**锛涘鏋滄樉绀?`[s2idle]`锛屽垯鏀寔 **s2idle**銆?
鍦ㄦ敮鎸?**S3** 鐨勭郴缁熶笂锛屽浐浠跺皢琚敤鏉ュ皢鎵€鏈夌‖浠剁疆浜庨€傚綋鐨勪綆鍔熻€楃姸鎬併€?
鍦ㄦ敮鎸?**s2idle** 鐨勭郴缁熶笂锛屽唴鏍稿皢璐熻矗灏嗚澶囪浆鎹㈠埌閫傚綋鐨勪綆鍔熻€楃姸鎬併€傚綋鎵€鏈夎澶囬兘澶勪簬閫傚綋鐨勪綆鍔熻€楃姸鎬佹椂锛岀‖浠跺皢杞崲鍒扮‖浠朵紤鐪犵姸鎬併€?
鍦ㄤ竴涓寕璧峰懆鏈熶箣鍚庯紝浣犲彲浠ラ€氳繃鏌ョ湅 `cat /sys/power/suspend_stats/last_hw_sleep` 鏉ヤ簡瑙ｅ湪纭欢浼戠湢鐘舵€佷腑鑺辫垂浜嗗灏戞椂闂淬€?
姝ゆ祦绋嬪浘璇存槑浜?AMD s2idle 鎸傝捣娴佺▼鏄浣曞伐浣滅殑銆?

姝ゆ祦绋嬪浘璇存槑浜?AMD s2idle 鎭㈠娴佺▼鏄浣曞伐浣滅殑銆?

## s2idle 璋冭瘯宸ュ叿


鐢变簬闂鍙兘鍑虹幇鍦ㄨ澶氬湴鏂癸紝鍥犳宸茬粡鍒涘缓浜嗕竴涓皟璇曞伐鍏凤紝浣嶄簬
`amd-debug-tools <https://git.kernel.org/pub/scm/linux/kernel/git/superm1/amd-debug-tools.git/about/>`_锛?瀹冨彲浠ュ府鍔╂祴璇曞父瑙侀棶棰樺苟鎻愪緵寤鸿銆?
濡傛灉浣犳湁 s2idle 闂锛屾渶濂戒粠杩欓噷寮€濮嬶紝骞堕伒寰叾鍙戠幇缁撴灉涓殑璇存槑銆傚鏋滀綘浠嶇劧鏈夐棶棰橈紝璇峰甫鐫€姝よ剼鏈敓鎴愮殑鎶ュ憡锛屽悜
`drm/amd gitlab <https://gitlab.freedesktop.org/drm/amd/-/issues/new?issuable_template=s2idle_BUG_TEMPLATE>`_
鎻愪氦涓€涓己闄枫€?
## 鏉ヨ嚜 IRQ 鐨勪吉 s2idle 鍞ら啋


浼敜閱掗€氬父浼氭湁涓€涓?IRQ 琚缃埌 `/sys/power/pm_wakeup_irq`銆傝繖鍙互鍖归厤鍒?`/proc/interrupts` 鏉ョ‘瀹氭槸浠€涔堣澶囧敜閱掍簡绯荤粺銆?
濡傛灉杩欒繕涓嶈冻浠ヨ皟璇曢棶棰橈紝閭ｄ箞鍙互浣跨敤浠ヤ笅 sysfs 鏂囦欢

```

  # echo 1 | sudo tee /sys/power/pm_debug_messages
  # echo 1 | sudo tee /sys/power/pm_print_times

```
鍦ㄨ繘琛岃繖浜涙洿鏀逛箣鍚庯紝鍐呮牳灏嗘樉绀哄彲浠ュ洖婧埌鍐呮牳 s2idle 寰幆浠ｇ爜鐨勬秷鎭紝骞跺湪鍞ら啋鏃舵樉绀轰换浣曟椿璺冪殑
GPIO 鏉ユ簮銆?
濡傛灉鍞ら啋鏄敱 ACPI SCI 寮曡捣鐨勶紝鍙兘闇€瑕侀澶栫殑 ACPI 璋冭瘯

```

  # echo enable | sudo tee /sys/module/acpi/parameters/trace_state
  # echo 1 | sudo tee /sys/module/acpi/parameters/aml_debug_output
  # echo 0x0800000f | sudo tee /sys/module/acpi/parameters/debug_level
  # echo 0xffff0000 | sudo tee /sys/module/acpi/parameters/debug_layer

```
## 鏉ヨ嚜 GPIO 鐨勪吉 s2idle 鍞ら啋


濡傛灉鍦ㄥ敜閱掔郴缁熸椂鏌愪釜 GPIO 澶勪簬娲昏穬鐘舵€侊紝鐞嗘兂鎯呭喌涓嬩綘搴旇鏌ョ湅鍘熺悊鍥炬潵纭畾瀹冧笌浠€涔堣澶囩浉鍏宠仈銆傚鏋滃師鐞嗗浘涓嶅彲鐢紝鍙︿竴绉嶇瓥鐣ユ槸鏌ョ湅 ACPI _EVT() 鏉＄洰锛屼互纭畾褰撹 GPIO 娲昏穬鏃朵細閫氱煡浠€涔堣澶囥€?
涓句竴涓亣璁剧殑渚嬪瓙锛屽亣璁?GPIO 59 鍞ら啋浜嗙郴缁熴€備綘鍙互鏌ョ湅 SSDT 鏉ョ‘瀹?GPIO 59 娲昏穬鏃朵細閫氱煡浠€涔堣澶囥€?
```

  $ python3 -c "print(hex(59))"
  0x3b

```

```

  $ sudo grep EVT /sys/firmware/acpi/tables/SSDT*
  grep: /sys/firmware/acpi/tables/SSDT27: binary file matches

```

```

  $ sudo cp /sys/firmware/acpi/tables/SSDT27 .
  $ sudo iasl -d SSDT27

```

```

  Case (0x3B)
  {
      M000 (0x393B)
      M460 ("    Notify (\\_SB.PCI0.GP17.XHC1, 0x02)\n", Zero, Zero, Zero, Zero, Zero, Zero)
      Notify (\_SB.PCI0.GP17.XHC1, 0x02) // Device Wake
  }

```
浣犲彲浠ョ湅鍒帮紝鍦ㄨ繖绉嶆儏鍐典笅锛屽綋 GPIO 59 娲昏穬鏃朵細閫氱煡璁惧 `\_SB.PCI0.GP17.XHC1`銆傛樉鐒惰繖鏄竴涓?XHCI 鎺у埗鍣紝浣嗚鏇磋繘涓€姝ワ紝浣犲彲浠ラ€氳繃灏嗗畠涓庝互涓嬪唴瀹瑰尮閰嶆潵纭畾瀹冩槸鍝釜 XHCI 鎺у埗鍣?
```

  $ grep "PCI0.GP17.XHC1" /sys/bus/acpi/devices/*/path
  /sys/bus/acpi/devices/device:2d/path:\_SB_.PCI0.GP17.XHC1
  /sys/bus/acpi/devices/device:2e/path:\_SB_.PCI0.GP17.XHC1.RHUB
  /sys/bus/acpi/devices/device:2f/path:\_SB_.PCI0.GP17.XHC1.RHUB.PRT1
  /sys/bus/acpi/devices/device:30/path:\_SB_.PCI0.GP17.XHC1.RHUB.PRT1.CAM0
  /sys/bus/acpi/devices/device:31/path:\_SB_.PCI0.GP17.XHC1.RHUB.PRT1.CAM1
  /sys/bus/acpi/devices/device:32/path:\_SB_.PCI0.GP17.XHC1.RHUB.PRT2
  /sys/bus/acpi/devices/LNXPOWER:0d/path:\_SB_.PCI0.GP17.XHC1.PWRS

```
杩欓噷浣犲彲浠ョ湅鍒板畠鍖归厤鍒颁簡 `device:2d`銆傛煡鐪?`physical_node`

```

  $ ls -l /sys/bus/acpi/devices/device:2d/physical_node
  lrwxrwxrwx 1 root root 0 Feb 12 13:22 /sys/bus/acpi/devices/device:2d/physical_node -> ../../../../../pci0000:00/0000:00:08.1/0000:c2:00.4

```
浜庢槸鐪熺浉澶х櫧锛氫笌姝?GPIO 鍞ら啋鐩稿叧鑱旂殑 PCI 璁惧鏄?`0000:c2:00.4`銆?
`amd_s2idle.py` 鑴氭湰灏嗕负浣犳崟鑾峰ぇ閮ㄥ垎杩欎簺宸ヤ欢銆?
## s2idle PM 璋冭瘯娑堟伅


鍦?AMD 绯荤粺鐨?s2idle 娴佺▼涓紝ACPI LPS0 椹卞姩璐熻矗妫€鏌ユ墍鏈?uPEP 绾︽潫銆傛湭婊¤冻 uPEP 绾︽潫骞朵笉浼氶樆姝?s0i3 杩涘叆銆傝繖鎰忓懗鐫€濡傛灉鏈変竴浜涚害鏉熸湭婊¤冻锛屽嵆浣垮瓨鍦ㄦ煇浜涘凡鐭ラ棶棰橈紝鍐呮牳浠嶅彲鑳藉皾璇曡繘鍏?s2idle銆?
瑕佹縺娲?PM 璋冭瘯锛屽彲浠ュ湪寮曞鏃舵寚瀹?`pm_debug_messagess` 鍐呮牳鍛戒护琛岄€夐」锛屾垨鑰呭啓鍏?`/sys/power/pm_debug_messages`銆傛湭婊¤冻鐨勭害鏉熶細鏄剧ず鍦ㄥ唴鏍告棩蹇椾腑锛屽苟鍙互閫氳繃澶勭悊鍐呮牳鐜舰缂撳啿鍖虹殑鏃ュ織宸ュ叿锛堝 `dmesg` 鎴?`journalctl`锛夋煡鐪嬨€?
濡傛灉绯荤粺鍦ㄥ埛鏂拌繖浜涙秷鎭箣鍓嶅湪杩涘嚭鏃跺喕缁擄紝涓€涓湁鐢ㄧ殑璋冭瘯绛栫暐鏄В缁?`amd_pmc` 椹卞姩锛屼互闃绘鍚戝钩鍙板彂鍑哄紑濮?s0i3 杩涘叆鐨勯€氱煡銆傝繖灏嗛樆姝㈢郴缁熷湪杩涘叆鎴栭€€鍑烘椂鍐荤粨锛屽苟璁╀綘鏌ョ湅鎵€鏈夊け璐ョ殑

```

  cd /sys/bus/platform/drivers/amd_pmc
  ls | grep AMD | sudo tee unbind

```

```

  ACPI: LPI: Constraint not met; min power state:%s current power state:%s

```
## s2idle 闂鐨勫巻鍙茬ず渚?

涓轰簡甯姪鐞嗚В鍙兘鍙戠敓鐨勯棶棰樼被鍨嬩互鍙婂浣曡皟璇曞畠浠紝杩欓噷鎻愪緵涓€浜涘凡瑙ｅ喅鐨?s2idle 闂鐨勫巻鍙茬ず渚嬨€?
### 鏍稿績绂荤嚎鍖栵紙Core offlining锛?

涓€浣嶆渶缁堢敤鎴锋姤鍛婅锛屽皢涓€涓牳蹇冪绾夸細闃绘绯荤粺姝ｇ‘杩涘叆 s0i3銆傝繖閫氳繃浣跨敤鍐呴儴 AMD 宸ュ叿璋冭瘯鏉ユ崟鑾峰拰鏄剧ず鏉ヨ嚜纭欢鐨勪竴涓叉寚鏍囷紝鏄剧ず浜嗘牳蹇冪绾挎椂鍙戠敓浜嗕粈涔堝彉鍖栥€傜‘瀹氱殑鏄紝纭欢娌℃湁鏀跺埌绂荤嚎鏍稿績宸茶繘鍏ユ渶娣辩姸鎬佺殑閫氱煡锛屽洜姝ゅ畠闃绘浜?CPU 杩涘叆鏈€娣辩姸鎬併€傝闂琚皟璇曚负涓€涓己澶辩殑鍛戒护鈥斺€斿湪绂荤嚎鏃惰鏍稿績杩涘叆 C3 鐘舵€併€?
`commit d6b88ce2eb9d2 ("ACPI: processor idle: Allow playing dead in C3 state") <https://git.kernel.org/torvalds/c/d6b88ce2eb9d2>`_

### 鎭㈠鍚庢崯鍧忥紙Corruption after resume锛?

Rembrandt 鍑虹幇鐨勪竴涓ぇ闂鏄仮澶嶅悗鍥惧舰鎹熷潖銆傝繖鏄敱浜?PSP 鍜岄┍鍔ㄨ亴璐ｄ箣闂寸殑閿欎綅閫犳垚鐨勩€侾SP 浼氫繚瀛樺拰鎭㈠ DMCUB锛屼絾椹卞姩鍋囧畾瀹冮渶瑕佸湪鎭㈠鏃堕噸缃?DMCUB銆傚疄闄呬笂锛岃繖绉嶉敊浣嶅湪鏇存棭鐨勭鐗囦笂涔熷瓨鍦ㄤ簬锛屽彧鏄病鏈夎瑙傚療鍒般€?
`commit 79d6b9351f086 ("drm/amd/display: Don't reinitialize DMCUB on s0ix resume") <https://git.kernel.org/torvalds/c/79d6b9351f086>`_

### 杩炵画鎸傝捣澶辫触锛圔ack to Back suspends fail锛?

褰撲娇鐢ㄤ竴涓Е鍙?IRQ 鏉ュ敜閱掔殑鍞ら啋婧愭椂锛宲inctrl-amd 椹卞姩涓殑涓€涓己闄峰彲鑳戒細鎹曡幏鍒?IRQ 鐨勯敊璇姸鎬侊紝浠庤€岄樆姝㈢郴缁熸纭洖鍒扮潯鐪犵姸鎬併€?
`commit b8c824a869f22 ("pinctrl: amd: Don't save/restore interrupt status and wake status bits") <https://git.kernel.org/torvalds/c/b8c824a869f22>`_

### 5 鍒嗛挓鍚庣殑浼畾鏃跺櫒鍞ら啋锛圫purious timer based wakeup after 5 minutes锛?

HPET 鏇捐鐢ㄦ潵涓虹郴缁熺紪绋嬪敜閱掓簮锛岀劧鑰岃繖瀵艰嚧浜?5 鍒嗛挓鍚庣殑浼敜閱掋€傛纭娇鐢ㄧ殑闂归挓搴旇鏄?ACPI 闂归挓銆?
`commit 3d762e21d5637 ("rtc: cmos: Use ACPI alarm for non-Intel x86 systems too") <https://git.kernel.org/torvalds/c/3d762e21d5637>`_

### 鎭㈠鍚庣鐩樻秷澶憋紙Disk disappears after resume锛?

浠?s2idle 鎭㈠鍚庯紝NVME 纾佺洏浼氭秷澶便€傝繖鏄敱浜?BIOS 娌℃湁鎸囧畾 _DSD StorageD3Enable 灞炴€ч€犳垚鐨勩€傝繖瀵艰嚧 NVME 椹卞姩娌℃湁鍦ㄦ寕璧锋椂灏嗙鐩樼疆浜庨鏈熺姸鎬侊紝骞跺湪鎭㈠鏃跺け璐ャ€?
`commit e79a10652bbd3 ("ACPI: x86: Force StorageD3Enable on more products") <https://git.kernel.org/torvalds/c/e79a10652bbd3>`_

### 浼?IRQ1锛圫purious IRQ1锛?

璁稿 Renoir銆丩ucienne銆丆ezanne 鍜?Barcelo 骞冲彴瀛樺湪涓€涓钩鍙板浐浠剁己闄凤紝鍗冲湪 s0i3 鎭㈠鏈熼棿瑙﹀彂 IRQ1銆?
璇ラ棶棰樺凡鍦ㄥ钩鍙板浐浠朵腑淇锛屼絾璁稿绯荤粺涓嶅啀鎺ユ敹浠讳綍骞冲彴鍥轰欢鏇存柊銆?
`commit 8e60615e89321 ("platform/x86/amd: pmc: Disable IRQ1 wakeup for RN/CZN") <https://git.kernel.org/torvalds/c/8e60615e89321>`_

### 纭欢瓒呮椂锛圚ardware timeout锛?

纭欢闄や簡鎺ュ彈鏉ヨ嚜 amd-pmc 椹卞姩鐨勫€间箣澶栵紝杩樻墽琛岃澶氭搷浣溿€傜敱浜庝笌纭欢鐨勯€氫俊璺緞鏄竴涓偖绠憋紝瀹冨彲鑳芥棤娉曡冻澶熷揩鍦板搷搴斻€?
```

  PM: dpm_run_callback(): acpi_subsys_suspend_noirq+0x0/0x50 returns -110
  amd_pmc AMDI0005:00: PM: failed to suspend noirq: error -110

```
璁℃椂闂鏄€氳繃姣旇緝绌洪棽鎺╃爜鐨勫€兼潵纭畾鐨勩€?
`commit 3c3c8e88c8712 ("platform/x86: amd-pmc: Increase the response register timeout") <https://git.kernel.org/torvalds/c/3c3c8e88c8712>`_

### 闈㈡澘寮€鍚椂鏃犳硶杩涘叆纭欢浼戠湢鐘舵€侊紙Failed to reach hardware sleep state with panel on锛?

鍦ㄤ竴浜?Strix 绯荤粺涓婏紝瑙傚療鍒版煇浜涢潰鏉夸細鍦ㄥ唴閮ㄩ潰鏉垮紑鍚椂闃绘绯荤粺杩涘叆纭欢浼戠湢鐘舵€併€?
灏界闈㈡澘鍦ㄦ寕璧锋湡闂磋鍏抽棴锛屼絾瀹冩毚闇蹭簡涓€涓鏃堕棶棰橈細涓€涓腑鏂鑷存樉绀虹‖浠跺敜閱掑苟闃绘浜嗕綆鍔熻€楃姸鎬佺殑杩涘叆銆?
`commit 40b8c14936bd2 ("drm/amd/display: Disable unneeded hpd interrupts during dm_init") <https://git.kernel.org/torvalds/c/40b8c14936bd2>`_

## 杩愯鏃跺姛鑰楅棶棰?

杩愯鏃跺姛鑰楀彈璁稿鍥犵礌褰卞搷锛屽寘鎷絾涓嶉檺浜?PCIe 涓诲姩鐘舵€佺數婧愮鐞嗭紙ASPM锛夌殑閰嶇疆銆佹樉绀轰寒搴︺€丆PU 鐨?EPP 绛栫暐锛屼互鍙婅澶囩殑鐢垫簮绠＄悊銆?
### ASPM


涓轰簡鑾峰緱鏈€浣崇殑杩愯鏃跺姛鑰楋紝ASPM 搴旇鎸夌収纭欢鍘傚晢鐨?BIOS 棰勬湡杩涜缂栫▼銆備负浜嗗疄鐜拌繖涓€鐐癸紝Linux 鍐呮牳搴旇浠?`CONFIG_PCIEASPM_DEFAULT` 璁句负 `y` 鐨勬柟寮忕紪璇戯紝骞朵笖涓嶅簲淇敼 sysfs 鏂囦欢 `/sys/module/pcie_aspm/parameters/policy`銆?
鏈€鍊煎緱娉ㄦ剰鐨勬槸锛屽鏋滀换浣曡澶囩殑 L1.2 娌℃湁姝ｇ‘閰嶇疆锛孲oC 灏嗘棤娉曡繘鍏ユ渶娣辩殑绌洪棽鐘舵€併€?
### EPP 绛栫暐


`energy_performance_preference` sysfs 鏂囦欢鍙敤浜庝负 CPU 璁剧疆鍋忓悜鏁堢巼鎴栨€ц兘銆傚綋瀹冩洿鍋忓悜鎬ц兘鏃讹紝涓庣數姹犵画鑸椂闂存湁鐩存帴鍏崇郴銆?

## BIOS 璋冭瘯娑堟伅


澶у鏁?OEM 鏈哄櫒娌℃湁鐢ㄤ簬杈撳嚭鍐呮牳鎴?BIOS 璋冭瘯娑堟伅鐨勪覆鍙?UART銆傜劧鑰?BIOS 璋冭瘯娑堟伅瀵逛簬鐞嗚В BIOS 缂洪櫡浠ュ強璋冪敤 BIOS AML 鐨?Linux 鍐呮牳椹卞姩缂洪櫡寰堟湁鐢ㄣ€?
鐢变簬澶у鏁?OEM AMD 绯荤粺涓婄殑 BIOS 鍩轰簬 AMD 鍙傝€?BIOS锛岀敤浜庡鍑鸿皟璇曟秷鎭殑鍩虹璁炬柦閫氬父涓?AMD 鍙傝€?BIOS 鐩稿悓銆?
### 鎵嬪姩瑙ｆ瀽锛圡anually Parsing锛?

閫氬父鏈変竴涓?ACPI 鏂规硶 `\M460`锛孉ML 鐨勪笉鍚岃矾寰勪細璋冪敤瀹冩潵鍚?BIOS 涓茶鏃ュ織鍙戝嚭涓€鏉℃秷鎭€傛鏂规硶鎺ュ彈
7 涓弬鏁帮紝绗竴涓槸瀛楃涓诧紝鍏朵綑鏄彲閫夌殑

```

  Method (M460, 7, Serialized)

```

```

  M460 ("  OEM-ASL-PCIe Address (0x%X)._REG (%d %d)  PCSA = %d\n", DADR, Arg0, Arg1, PCSA, Zero, Zero)

```
閫氬父鎵ц鏃讹紝`\M460` 鏂规硶浼氬皢闄勫姞鍙傛暟濉厖鍒板瓧绗︿覆涓€備负浜嗕粠 Linux 鍐呮牳鑾峰彇杩欎簺娑堟伅锛孉CPICA 涓?鍔犲叆浜嗕竴涓挬瀛愶紝瀹冨彲浠ユ崟鑾峰彂閫佺粰 `\M460` 鐨?*鍙傛暟**骞跺皢鍏舵墦鍗板埌鍐呮牳鐜舰缂撳啿鍖恒€?
```

  extrace-0174 ex_trace_args         :  "  OEM-ASL-PCIe Address (0x%X)._REG (%d %d)  PCSA = %d\n", ec106000, 2, 1, 1, 0, 0

```
涓轰簡鑾峰彇杩欎簺娑堟伅锛屼綘闇€瑕佷互 `CONFIG_ACPI_DEBUG` 缂栬瘧锛岀劧鍚庢墦寮€浠ヤ笅 ACPICA 璺熻釜鍙傛暟銆?杩欏彲浠ュ湪鍐呮牳鍛戒护琛屾垨杩愯鏃跺畬鎴愶細

- `acpi.trace_method_name=\M460`
- `acpi.trace_state=method`

娉ㄦ剰锛氳繖浜涘湪寮曞鏃跺彲鑳介潪甯稿槇鏉傘€傚鏋滀綘鍦ㄥ唴鏍稿懡浠よ涓婃墦寮€杩欎簺鍙傛暟锛岃鍚屾椂鑰冭檻灏?`CONFIG_LOG_BUF_SHIFT` 璋冨ぇ鍒版洿澶х殑鍊硷紙濡?17锛夛紝浠ラ伩鍏嶄涪澶辨棭鏈熷紩瀵兼秷鎭€?
### 宸ュ叿杈呭姪瑙ｆ瀽锛圱ool assisted Parsing锛?

濡備笂鎵€杩帮紝鎵嬪姩瑙ｆ瀽鍙兘寰堢箒鐞愶紝灏ゅ叾鏄湪鏈夊ぇ閲忔秷鎭椂銆備负浜嗗府鍔╄В鍐宠繖涓棶棰橈紝宸茬粡鍒涘缓浜嗕竴涓伐鍏凤紝浣嶄簬
`amd-debug-tools <https://git.kernel.org/pub/scm/linux/kernel/git/superm1/amd-debug-tools.git/about/>`_锛?鐢ㄤ簬甯姪瑙ｆ瀽杩欎簺娑堟伅銆?
## 闅忔満閲嶅惎闂


褰撳彂鐢熼殢鏈洪噸鍚椂锛岄噸鍚殑楂樺眰鍘熷洜瀛樺偍鍦ㄤ竴涓瘎瀛樺櫒涓紝骞朵細淇濈暀鍒颁笅涓€娆″紩瀵笺€?
閲嶅惎鍘熷洜鍒嗕负 6 绫伙細
 - Software induced锛堣蒋浠跺紩鍙戯級
 - Power state transition锛堢數婧愮姸鎬佽浆鎹級
 - Pin induced锛堝紩鑴氬紩鍙戯級
 - Hardware induced锛堢‖浠跺紩鍙戯級
 - Remote reset锛堣繙绋嬪浣嶏級
 - Internal CPU event锛堝唴閮?CPU 浜嬩欢锛?
   :header: "Bit", "Type", "Reason"
   :align: left

   "0",  "Pin",      "鐑紩鑴?BP_THERMTRIP_L 琚Е鍙?
   "1",  "Pin",      "鐢垫簮鎸夐挳琚寜涓嬩簡 4 绉?
   "2",  "Pin",      "鍏虫満寮曡剼琚Е鍙?
   "4",  "Remote",   "鎺ユ敹鍒拌繙绋?ASF 鍏虫満鍛戒护"
   "9",  "Internal", "鍐呴儴 CPU 鐑檺鍒惰瑙﹀彂"
   "16", "Pin",      "绯荤粺澶嶄綅寮曡剼 BP_SYS_RST_L 琚Е鍙?
   "17", "Software", "杞欢鍙戝嚭浜?PCI 澶嶄綅"
   "18", "Software", "杞欢鍚戝浣嶆帶鍒跺瘎瀛樺櫒 0xCF9 鍐欏叆浜?0x4"
   "19", "Software", "杞欢鍚戝浣嶆帶鍒跺瘎瀛樺櫒 0xCF9 鍐欏叆浜?0x6"
   "20", "Software", "杞欢鍚戝浣嶆帶鍒跺瘎瀛樺櫒 0xCF9 鍐欏叆浜?0xE"
   "21", "ACPI-state", "鍙戠敓浜?ACPI 鐢垫簮鐘舵€佽浆鎹?
   "22", "Pin",      "閿洏澶嶄綅寮曡剼 KB_RST_L 琚Е鍙?
   "23", "Internal", "鍙戠敓浜嗗唴閮?CPU 鍏虫満浜嬩欢"
   "24", "Hardware", "绯荤粺鍦ㄥけ璐ュ惎鍔ㄥ畾鏃跺櫒鍒版湡鍓嶆湭鑳藉紩瀵?
   "25", "Hardware", "纭欢鐪嬮棬鐙楀畾鏃跺櫒鍒版湡"
   "26", "Remote",   "鎺ユ敹鍒拌繙绋?ASF 澶嶄綅鍛戒护"
   "27", "Internal", "涓€涓湭绾犳閿欒瀵艰嚧浜嗘暟鎹粐鐗╋紙data fabric锛夊悓姝ユ椽娉涗簨浠?
   "29", "Internal", "FCH 鍜?MP1 鏈兘瀹屾垚鐑浣嶆彙鎵?
   "30", "Internal", "鍙戠敓浜嗗鍋舵牎楠岄敊璇?
   "31", "Internal", "鍙戠敓浜嗚蒋浠跺悓姝ユ椽娉涗簨浠?

姝や俊鎭湪鍐呮牳寮曞鏃惰鍙栧苟鎵撳嵃鍒?syslog 涓€傚綋鍙戠敓闅忔満閲嶅惎鏃讹紝姝ゆ秷鎭湁鍔╀簬纭畾涓嬩竴涓璋冭瘯鐨勭粍浠躲€?