## 浣庡姛鑰楃┖闂茶〃锛圠ow Power Idle Table, LPIT锛?

涓轰簡鏋氫妇骞冲彴鐨勪綆鍔熻€楃┖闂茬姸鎬侊紝Intel 骞冲彴浣跨敤浜嗏€滀綆鍔熻€楃┖闂茶〃鈥濓紙Low Power Idle Table锛孡PIT锛夈€傛湁鍏宠琛ㄧ殑鏇村缁嗚妭鍙粠浠ヤ笅鍦板潃涓嬭浇锛?https://www.uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf

姣忎釜浣庡姛鑰楃姸鎬佺殑椹荤暀鏃堕棿锛圧esidency锛夊彲浠ラ€氳繃 FFH锛團unction fixed hardware锛屽姛鑳藉浐瀹氱‖浠讹級鎴栧唴瀛樻槧灏勶紙memory mapped锛夋帴鍙ｈ鍙栥€?
鍦ㄦ敮鎸?S0ix 鐫＄湢鐘舵€佺殑骞冲彴涓婏紝鍙兘瀛樺湪涓ょ绫诲瀷鐨勯┗鐣欐椂闂达細

  - CPU PKG C10锛堥€氳繃 FFH 鎺ュ彛璇诲彇锛?  - 骞冲彴鎺у埗鍣ㄤ腑鏋紙Platform Controller Hub锛孭CH锛塖LP_S0锛堥€氳繃鍐呭瓨鏄犲皠鎺ュ彛璇诲彇锛?
浠ヤ笅灞炴€т細琚姩鎬佸湴娣诲姞鍒?cpuidle 涓細
```
  /sys/devices/system/cpu/cpuidle/low_power_idle_cpu_residency_us
  /sys/devices/system/cpu/cpuidle/low_power_idle_system_residency_us
```

"low_power_idle_cpu_residency_us" 灞炴€ф樉绀?CPU 灏佽锛坧ackage锛夊浜?PKG C10 鐨勬椂闂淬€?
"low_power_idle_system_residency_us" 灞炴€ф樉绀?SLP_S0 鐨勯┗鐣欐椂闂达紝鍗?SLP_S0# 淇″彿琚疆浣嶆湡闂寸郴缁熸墍鑺辫垂鐨勬椂闂淬€傝繖鏄彲鑳界殑鏈€浣庣郴缁熷姛鑰楃姸鎬侊紝浠呭綋 CPU 澶勪簬 PKG C10 涓?PCH 涓殑鎵€鏈夊姛鑳芥ā鍧楅兘澶勪簬浣庡姛鑰楃姸鎬佹椂鎵嶅彲瀹炵幇銆?