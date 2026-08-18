## Linux 鍐呮牳 VFP 鏀寔浠ｇ爜鍙戝竷璇存槑


鏃ユ湡锛?004 骞?5 鏈?20 鏃?
浣滆€咃細Russell King

杩欐槸 Linux 鍐呮牳 VFP 鏀寔浠ｇ爜鐨勯娆″彂甯冦€傚畠鎻愪緵瀵逛粠 ARM926EJ-S 涓?VFP 纭欢
寮瑰洖鐨勫紓甯哥殑鏀寔銆?
姝ょ増鏈凡閽堝 John R. Hauser 鐨?SoftFloat-2b 搴擄紝浣跨敤 TestFloat-2a 娴嬭瘯濂椾欢
杩涜楠岃瘉銆傛湁鍏宠搴撲笌娴嬭瘯濂椾欢鐨勭粏鑺傚彲鍦ㄦ澶勬壘鍒帮細

   http://www.jhauser.us/arithmetic/SoftFloat.html

宸蹭娇鐢ㄦ鍖呮祴璇曠殑杩愮畻鏈夛細

 - fdiv
 - fsub
 - fadd
 - fmul
 - fcmp
 - fcmpe
 - fcvtd
 - fcvts
 - fsito
 - ftosi
 - fsqrt

涓婅堪鎵€鏈夎繍绠楀潎閫氳繃 softfloat 娴嬭瘯锛屼絾鏈変互涓嬩緥澶栵細

- fadd/fsub 鍦ㄨ緭鍏ユ搷浣滄暟绗﹀彿涓嶅悓鏃讹紝瀵?+0 / -0 缁撴灉鐨勫鐞嗘湁浜涘樊寮傘€?- 涓嬫孩寮傚父鐨勫鐞嗙暐鏈変笉鍚屻€傚鏋滀竴涓粨鏋滃湪鑸嶅叆鍓嶄笅婧紝浣嗗湪鑸嶅叆鍚庡彉涓鸿鑼冨寲
  鏁帮紝鎴戜滑涓嶄細鍙戝嚭涓嬫孩寮傚父銆?
鍏朵粬宸查€氳繃鍩烘湰绾眹缂栨祴璇曡繍绠楃殑鏈夛細

 - fcpy
 - fabs
 - fneg
 - ftoui
 - ftosiz
 - ftouiz

鏈祴璇曠殑缁勫悎杩愮畻鏈夛細

 - fmac
 - fnmac
 - fmsc
 - fnmsc
 - fnmul
