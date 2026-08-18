
## 鍐呮牳椹卞姩 lp5812


- TI/National Semiconductor LP5812 LED 椹卞姩
- Datasheet: https://www.ti.com/product/LP5812#tech-docs

Authors: Jared Zhou <jared-zhou@ti.com>

## 鎻忚堪


LP5812 鏄竴涓?4x3 鐭╅樀 LED 椹卞姩锛屾敮鎸佹墜鍔ㄥ拰鑷富鍔ㄧ敾鎺у埗銆傝椹卞姩鎻愪緵 sysfs 鎺ュ彛鏉ユ帶鍒跺拰閰嶇疆 LP5812 璁惧鍙婂叾 LED 閫氶亾銆?

## Sysfs 鎺ュ彛


璇ラ┍鍔ㄤ娇鐢?Documentation/ABI/testing/sysfs-class-led-multicolor.rst 涓畾涔夌殑鏍囧噯澶氳壊 LED class 鎺ュ彛銆?

姣忎釜 LP5812 LED 杈撳嚭鍑虹幇鍦?`/sys/class/leds/` 涓嬶紝甯︽湁鍏跺垎閰嶇殑鏍囩锛堜緥濡?`LED_A`锛夈€?

鏆撮湶浠ヤ笅灞炴€э細
  - multi_intensity锛氭瘡閫氶亾 RGB 寮哄害鎺у埗
  - brightness锛氭爣鍑嗕寒搴︽帶鍒讹紙0-255锛?

## 鑷富鎺у埗妯″紡


璇ラ┍鍔ㄨ繕鏀寔閫氳繃璁惧鏍戜腑瀹氫箟鐨勬ā寮忛厤缃紙渚嬪 direct銆乼cmscan 鎴?mixscan 妯″紡锛夎繘琛岃嚜涓绘帶鍒躲€傞厤缃悗锛孡P5812 鍙互鍦ㄦ病鏈?CPU 骞查鐨勬儏鍐典笅鐢熸垚杩囨浮鍜岄鑹叉晥鏋溿€?

鏈夊叧鏈夋晥鐨勬ā寮忓瓧绗︿覆鍜岄厤缃ず渚嬶紝璇峰弬闃呰澶囨爲缁戝畾鏂囨。銆?

## 绀轰緥浣跨敤


```
    # 璁剧疆 RGB 寮哄害锛圧=50, G=50, B=50锛?
    echo 50 50 50 > /sys/class/leds/LED_A/multi_intensity
    # 璁剧疆鏁翠綋浜害涓烘渶澶?
    echo 255 > /sys/class/leds/LED_A/brightness

```
