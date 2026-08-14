## Leds BlinkM 椹卞姩


leds-blinkm 椹卞姩鏀寔 BlinkM 绯诲垪鐨勮澶囥€?
瀹冧滑鏄?RGB-LED 妯″潡锛岀敱 (AT)tiny 寰帶鍒跺櫒椹卞姩锛屽苟閫氳繃 I2C 閫氫俊銆傝繖浜涙ā鍧楃殑榛樿
鍦板潃鏄?0x09锛屼絾鍙互閫氳繃鍛戒护鏇存敼銆傝繖鏍蜂綘鍙互鍦ㄤ竴鏉?I2C 鎬荤嚎涓婁互鑿婅姳閾炬柟寮忚繛鎺ユ渶澶?127 涓?BlinkM銆?
璇ヨ澶囬€氳繃鐙珛鐨勫懡浠ゆ帴鍙?RGB 涓?HSB 棰滆壊鍊笺€備綘涔熷彲浠ュ湪鎺у埗鍣ㄤ腑鎶婇棯鐑佸簭鍒楀瓨鍌ㄤ负
鈥滆剼鏈€濓紙scripts锛夊苟杩愯瀹冧滑銆傛笎鍙橈紙fading锛変篃鏄竴涓彲閫夐」銆?
鏈┍鍔ㄦ彁渚涚殑鎺ュ彛鏈変笁灞傦細

# a) 鐢ㄤ簬閰嶅悎瑙﹀彂鍣ㄤ娇鐢ㄧ殑 LED 澶氳壊绫绘帴鍙?

```

  blinkm-<i2c-bus-nr>-<i2c-device-nr>:rgb:indicator

  $ ls -h /sys/class/leds/blinkm-1-9:rgb:indicator
  brightness  device  max_brightness  multi_index  multi_intensity  power  subsystem  trigger  uevent

```
鑹茬浉锛圚ue锛夌敱 multi_intensity 鏂囦欢鎺у埗锛屼寒搴︼紙lightness锛夌敱 brightness 鏂囦欢鎺у埗銆?
鍐欏叆寮哄害鍊肩殑椤哄簭鍙互鍦?multi_index 涓壘鍒般€傚繀椤诲悜 multi_intensity 鍐欏叆姝ｅソ涓変釜
浠嬩簬 0 鍒?255 涔嬮棿鐨勫€硷紝浠ワ細

```

  $ echo 255 100 50 > multi_intensity

```
閫氳繃鍚?brightness 鏂囦欢鍐欏叆涓€涓粙浜?0 鍒?255 涔嬮棿鐨勫€硷紝鍙互鏀瑰彉鏁翠綋浜害銆?
# b) 鐢ㄤ簬閰嶅悎瑙﹀彂鍣ㄤ娇鐢ㄧ殑 LED 绫绘帴鍙?

```

  blinkm-<i2c-bus-nr>-<i2c-device-nr>-<color>

  $ ls -h /sys/class/leds/blinkm-6-*
  /sys/class/leds/blinkm-6-9-blue:
  brightness  device  max_brightness  power  subsystem  trigger  uevent

  /sys/class/leds/blinkm-6-9-green:
  brightness  device  max_brightness  power  subsystem  trigger  uevent

  /sys/class/leds/blinkm-6-9-red:
  brightness  device  max_brightness  power  subsystem  trigger  uevent

```
锛?sys/bus/i2c/devices/6-0009/leds 涓浉鍚岋級

鎴戜滑鍙互灏嗛鑹叉媶鍒嗕负绾€佺豢銆佽摑鍒嗗埆鎺у埗锛屽苟涓烘瘡绉嶉鑹插垎閰嶈Е鍙戝櫒銆?
```

  $ cat blinkm-6-9-blue/brightness
  05

  $ echo 200 > blinkm-6-9-blue/brightness
  $

  $ modprobe ledtrig-heartbeat
  $ echo heartbeat > blinkm-6-9-green/trigger
  $


```
# b) 鐢ㄤ簬鎺у埗 rgb銆乫ade銆乭sb銆乻cripts 鐨?Sysfs 缁?...


姝ゆ墿灞曟帴鍙ｄ綔涓?blinkm 鏂囦欢澶癸紝浣嶄簬 I2C 璁惧鐨?sysfs 鏂囦欢澶逛腑銆備緥濡備綅浜?/sys/bus/i2c/devices/6-0009/blinkm 涓?
  $ ls -h /sys/bus/i2c/devices/6-0009/blinkm/
  blue  green  red  test

鐩墠浠呮敮鎸佽缃孩銆佺豢銆佽摑浠ュ強涓€涓祴璇曞簭鍒椼€?
```

  $ cat *
  00
  00
  00
  #Write into test to start test sequence!#

  $ echo 1 > test
  $

  $ echo 255 > red
  $



```
鎴嚦 2024 骞?07 鏈?
dl9pf <at> gmx <dot> de
jstrauss <at> mailbox <dot> org
