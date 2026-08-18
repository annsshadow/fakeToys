## USB 绔彛 LED 瑙﹀彂鍣?

璇?LED 瑙﹀彂鍣ㄥ彲鐢ㄤ簬鍚戠敤鎴锋寚绀虹粰瀹氱鍙ｄ笂鏄惁瀛樺湪 USB 璁惧銆傚畠浼氬湪璁惧
鍑虹幇鏃剁偣浜?LED锛屽苟鍦ㄨ澶囨秷澶辨椂鐔勭伃銆?
瀹冮渶瑕侀€夋嫨瑕佽瀵熺殑 USB 绔彛銆傛墍鏈夊彲鐢ㄧ鍙ｉ兘浣滀负鐙珛鏉＄洰鍒楀湪 "ports"
瀛愮洰褰曚腑銆傞€夋嫨閫氳繃灏?"1" 鍐欏叆鎵€閫夌鍙ｆ潵瀹屾垚銆?
璇锋敞鎰忥紝璇ヨЕ鍙戝櫒鍏佽涓哄崟涓?LED 閫夋嫨澶氫釜 USB 绔彛銆?
杩欏湪涓ょ鎯呭喌涓嬫湁鐢細

## 1) 鍏锋湁鍗曚釜 USB LED 涓庡皯閲忕墿鐞嗙鍙ｇ殑璁惧


鍦ㄨ繖绉嶆儏鍐典笅锛屽彧瑕佸瓨鍦ㄨ嚦灏戜竴涓凡杩炴帴鐨?USB 璁惧锛孡ED 灏变細鐐逛寒銆?
## 2) 鐢卞皯閲忔帶鍒跺櫒澶勭悊鐨勭墿鐞嗙鍙ｇ殑璁惧


鏌愪簺璁惧鍙兘姣忎釜 PHY 鏍囧噯鏈変竴涓帶鍒跺櫒銆備緥濡?USB 3.0 鐗╃悊绔彛鍙兘鐢?ohci-platform銆乪hci-platform 鍜?xhci-hcd 澶勭悊銆傚鏋滃彧鏈変竴涓?LED锛岀敤鎴?寰堝彲鑳藉笇鏈涘垎閰嶆潵鑷叏閮?3 涓泦绾垮櫒鐨勭鍙ｃ€?

璇ヨЕ鍙戝櫒鍙粠鐢ㄦ埛绌洪棿鍦?led class 璁惧涓婃縺娲伙紝濡備笅鎵€绀?```

  echo usbport > trigger

```
杩欎細鍚?LED 娣诲姞鍦ㄤ互涓嬩綅缃枃妗ｅ寲鐨?sysfs 灞炴€э細
Documentation/ABI/testing/sysfs-class-led-trigger-usbport
```

  echo usbport > trigger
  echo 1 > ports/usb1-port1
  echo 1 > ports/usb2-port1
  cat ports/usb1-port1
  echo 0 > ports/usb1-port1

```
