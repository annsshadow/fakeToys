## 鏃у唴鏍镐腑浠庣敤鎴风┖闂存帶鍒?I2C 璁惧椹卞姩缁戝畾


   娉ㄦ剰锛氭湰鑺備粎鍦ㄤ綘澶勭悊鍦ㄥ唴鏍?2.6 涓彂鐜扮殑涓€浜涙棫浠ｇ爜鏃剁浉鍏炽€傚鏋滀綘浣跨敤鐨勬槸杈冩柊鐨勫唴鏍革紝鍙互瀹夊叏鍦拌烦杩囨湰鑺傘€?
鍦ㄥ唴鏍?2.6.32 涔嬪墠锛岃澶?I2C 椹卞姩浣跨敤 <linux/i2c.h> 鎻愪緵鐨勮緟鍔╁畯锛岃繖浜涘畯鍒涘缓浜嗘爣鍑嗙殑妯″潡鍙傛暟锛岃鐢ㄦ埛鍙互鎺у埗椹卞姩濡備綍鎺㈡祴 I2C 鎬荤嚎骞堕檮鍔犲埌璁惧銆傝繖浜涘弬鏁拌绉颁负 `probe`锛堣椹卞姩鎺㈡祴涓€涓澶栫殑鍦板潃锛夈€乣force`锛堝己鍒跺皢椹卞姩闄勫姞鍒扮粰瀹氳澶囷級鍜?`ignore`锛堥樆姝㈤┍鍔ㄦ帰娴嬬粰瀹氬湴鍧€锛夈€?
闅忕潃 I2C 瀛愮郴缁熷悜鏍囧噯璁惧椹卞姩缁戝畾妯″瀷杞崲锛岃繖浜涙瘡妯″潡鍙傛暟鍙樺緱涓嶅啀闇€瑕侊紝骞朵笖闆嗕腑寮忓疄鐜版垚涓哄彲鑳姐€傛柊鐨勩€佸熀浜?sysfs 鐨勬帴鍙ｅ湪 Documentation/i2c/instantiating-devices.rst 鐨?Method 4: Instantiate from user-space"涓€鑺備腑鎻忚堪銆?
涓嬮潰鏄棫妯″潡鍙傛暟鍒版柊鎺ュ彛鐨勬槧灏勩€?
### 灏嗛┍鍔ㄩ檮鍔犲埌 I2C 璁惧


```

  # modprobe <driver> probe=1,0x2d
  # modprobe <driver> force=1,0x2d
  # modprobe <driver> force_<device>=1,0x2d

```

```
  # echo <device> 0x2d > /sys/bus/i2c/devices/i2c-1/new_device

```
### 闃绘椹卞姩闄勫姞鍒?I2C 璁惧


```

  # modprobe <driver> ignore=1,0x2f

```

```
  # echo dummy 0x2f > /sys/bus/i2c/devices/i2c-1/new_device
  # modprobe <driver>

```
褰撶劧锛岄噸瑕佺殑鏄湪鍔犺浇椹卞姩涔嬪墠瀹炰緥鍖?`dummy` 璁惧銆俤ummy 璁惧灏嗙敱 i2c-core 鑷韩澶勭悊锛屼粠鑰岄樆姝㈠叾浠栭┍鍔ㄧ◢鍚庣粦瀹氬埌瀹冦€傚鏋滈棶棰樺湴鍧€澶勬湁涓€涓湡瀹炶澶囷紝骞朵笖浣犲笇鏈涘彟涓€涓┍鍔ㄧ粦瀹氬埌瀹冿紝閭ｄ箞鍙渶浼犲叆鐩稿叧璁惧鐨勫悕绉拌€屼笉鏄?`dummy`銆?