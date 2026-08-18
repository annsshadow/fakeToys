## 鎺堟潈锛堟垨涓嶆巿鏉冿級浣犵殑 USB 璁惧杩炴帴鍒扮郴缁?

Copyright (C) 2007 Inaky Perez-Gonzalez <inaky@linux.intel.com> Intel Corporation

姝ょ壒鎬у厑璁镐綘鎺у埗涓€涓?USB 璁惧鏄惁鍙互鍦ㄧ郴缁熶腑浣跨敤锛堟垨涓嶈浣跨敤锛夈€傛鐗规€у皢鍏佽浣?瀹炵幇鐢辩敤鎴风┖闂村畬鍏ㄦ帶鍒剁殑 USB 璁惧閿佸畾锛坙ock-down锛夈€?
鎴嚦鐩墠锛屽綋杩炴帴涓€涓?USB 璁惧鏃讹紝瀹冧細琚厤缃紝鍏舵帴鍙ｄ細绔嬪嵆瀵圭敤鎴峰彲鐢ㄣ€傜粡杩囨淇敼锛?鍙湁褰?root 鎺堟潈璇ヨ澶囪繘琛岄厤缃悗锛屾墠鑳戒娇鐢ㄥ畠銆?
## 鐢ㄦ硶


```

	$ echo 1 > /sys/bus/usb/devices/DEVICE/authorized

```
```

	$ echo 0 > /sys/bus/usb/devices/DEVICE/authorized

```
榛樿灏嗚繛鎺ュ埌 hostX 鐨勬柊璁惧璁句负榛樿涓嶆巿鏉冿紙鍗筹細
```

	$ echo 0 > /sys/bus/usb/devices/usbX/authorized_default

```
```

	$ echo 1 > /sys/bus/usb/devices/usbX/authorized_default

```
榛樿鎯呭喌涓嬶紝鎵€鏈?USB 璁惧閮芥槸琚巿鏉冪殑銆傚悜 authorized_default 灞炴€у啓鍏?"2" 浼氫娇鍐呮牳
榛樿浠呮巿鏉冭繛鎺ュ埌鍐呴儴 USB 绔彛鐨勮澶囥€?
### 绯荤粺閿佸畾绀轰緥锛堢畝闄嬬増锛?

璁炬兂浣犳兂瀹炵幇涓€绉嶉攣瀹氾紝浣垮緱鍙湁 XYZ 绫诲瀷鐨勮澶囧彲浠ヨ繛鎺ワ紙渚嬪锛岃繖鏄竴鍙板甫鏈夊彲瑙?```

  鍚姩
  rc.local ->

   for host in /sys/bus/usb/devices/usb*
   do
      echo 0 > $host/authorized_default
   done

```
```

  if device_is_my_type $DEV
  then
    echo 1 > $device_path/authorized
  done


```
鐜板湪锛宒evice_is_my_type() 鎵嶆槸閿佸畾鐪熸鐨勭簿楂撴墍鍦ㄣ€備粎浠呮鏌?class銆乼ype 涓?protocol 鏄惁
鍖归厤鏌愪簨鐗╋紝鏄綘鎵€鑳藉仛鐨勬渶绯熺硶鐨勫畨鍏ㄩ獙璇侊紙鎴栬€呭浜庢兂绐佺牬瀹冪殑浜烘潵璇存槸鏈€濂界殑锛夈€傚鏋滀綘闇€瑕?瀹夊叏鐨勬柟妗堬紝璇蜂娇鐢ㄥ姞瀵嗕笌璇佷功璁よ瘉鎴栫被浼兼墜娈点€傚浜庡瓨鍌ㄥ瘑閽ヨ繖绫荤畝鍗曞満鏅?```

  function device_is_my_type()
  {
    echo 1 > authorized		# 涓存椂鎺堟潈瀹?                                # FIXME锛氱‘淇濇棤浜鸿兘鎸傝浇瀹?    mount DEVICENODE /mntpoint
    sum=$(md5sum /mntpoint/.signature)
    if [ $sum = $(cat /etc/lockdown/keysum) ]
    then
         echo "We are good, connected"
         umount /mntpoint
         # 鍏朵粬鎿嶄綔浠ヤ究浠栦汉鍙互浣跨敤瀹?    else
         echo 0 > authorized
    fi
  }


```
褰撶劧锛岃繖寰堢畝闄嬶紝浣犱細鎯宠鐢?PKI 鍋氱湡姝ｇ殑璇佷功楠岃瘉锛岃繖鏍峰氨涓嶄緷璧栧叡浜瘑閽ョ瓑绛夛紝浣嗘€濊矾灏辨槸杩欐牱銆?浠讳綍鑳藉鎺ヨЕ鍒拌澶囧皬宸ュ叿濂椾欢鐨勪汉閮藉彲浠ヤ吉閫犳弿杩扮涓庤澶囦俊鎭€備笉瑕佷俊浠昏繖浜涖€備笉瀹㈡皵銆?
### 鎺ュ彛鎺堟潈


鏈変竴绉嶇被浼肩殑鏂规硶鍙互鍏佽鎴栨嫆缁濈壒瀹氱殑 USB 鎺ュ彛銆傝繖鍏佽鍙睆钄戒竴涓?USB 璁惧鐨勫瓙闆嗐€?
```

	$ echo 1 > /sys/bus/usb/devices/INTERFACE/authorized

```
```

	$ echo 0 > /sys/bus/usb/devices/INTERFACE/authorized

```
鍦ㄧ壒瀹?USB 鎬荤嚎涓婏紝鏂版帴鍙ｇ殑榛樿鍊间篃鍙互琚洿鏀广€?
```

	$ echo 1 > /sys/bus/usb/devices/usbX/interface_authorized_default

```
```

	$ echo 0 > /sys/bus/usb/devices/usbX/interface_authorized_default

```
榛樿鎯呭喌涓?interface_authorized_default 浣嶄负 1銆?鍥犳鎵€鏈夋帴鍙ｉ粯璁ら兘浼氳鎺堟潈銆?
娉ㄦ剰锛?  濡傛灉涓€涓鍙栨秷鎺堟潈鐨勬帴鍙ｈ閲嶆柊鎺堟潈锛屽垯椹卞姩鎺㈡祴蹇呴』閫氳繃灏?INTERFACE 鍐欏叆
  /sys/bus/usb/drivers_probe 鎵嬪姩瑙﹀彂銆?
瀵逛簬闇€瑕佸涓帴鍙ｇ殑椹卞姩锛屽簲棣栧厛鎺堟潈鎵€鏈夐渶瑕佺殑鎺ュ彛銆備箣鍚庡啀杩涜椹卞姩鎺㈡祴銆?杩欐牱鍙互閬垮厤鍓綔鐢ㄣ€?