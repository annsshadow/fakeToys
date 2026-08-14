## Linux 鐩叉枃鎺у埗鍙?

瑕佸湪鐩叉枃璁惧涓婅幏鍙栨棭鏈熷惎鍔ㄦ秷鎭紙鍦ㄧ敤鎴风┖闂村睆骞曢槄璇诲櫒鍚姩涔嬪墠锛夛紝浣犻鍏堥渶瑕?缂栬瘧瀵瑰父瑙勪覆琛屾帶鍒跺彴鐨勬敮鎸侊紙瑙?Documentation/admin-guide/serial-console.rst
<serial_console>锛夛紝浠ュ強瀵圭洸鏂囪澶囩殑鏀寔锛堝湪 `Device Drivers --> Accessibility
support --> Console on braille device` 涓級銆?
鐒跺悗浣犻渶瑕佹寚瀹氫竴涓?`console=brl` 閫夐」鍦ㄥ唴鏍稿懡浠よ涓婏紝

```
	console=brl,serial_options...
```
鍏朵腑 `serial_options...` 涓?Documentation/admin-guide/serial-console.rst
<serial_console> 涓弿杩扮殑鐩稿悓銆?
渚嬪锛屽鏋滅洸鏂囪澶囪繛鎺ュ埌绗竴涓覆鍙ｏ紝浣犲彲浠ヤ娇鐢?`console=brl,ttyS0`锛涗娇鐢?`console=brl,ttyS0,115200` 鍙皢娉㈢壒鐜囪鐩栦负 115200锛岀瓑绛夈€?
榛樿鎯呭喌涓嬶紝鐩叉枃璁惧浠呮樉绀烘渶鍚庝竴鏉″唴鏍告秷鎭紙鎺у埗鍙版ā寮忥級銆傝鏌ョ湅鍏堝墠鐨勬秷鎭紝
鎸?Insert 閿垏鎹㈠埌 VT 瀹℃煡妯″紡銆傚湪瀹℃煡妯″紡涓嬶紝鏂瑰悜閿厑璁告祻瑙?VT 鍐呭锛?`PAGE-UP`/`PAGE-DOWN` 閿烦鍒板睆骞曢《閮?搴曢儴锛宍HOME` 閿洖鍒板厜鏍囧锛屼粠鑰屾彁渚?闈炲父鍩烘湰鐨勫睆骞曞鏌ュ姛鑳姐€?
鍙互閫氳繃娣诲姞 `braille_console.sound=1` 鍐呮牳鍙傛暟鏉ヨ幏寰楀０闊冲弽棣堛€?
涓虹畝鍗曡捣瑙侊紝鍙兘鍚敤涓€涓洸鏂囨帶鍒跺彴锛屽叾浠?`console=brl,...` 鐨勪娇鐢ㄥ皢琚涪寮冦€?杩樿娉ㄦ剰锛屽畠涓嶄細骞叉壈 Documentation/admin-guide/serial-console.rst
<serial_console> 涓弿杩扮殑鎺у埗鍙伴€夋嫨鏈哄埗銆?
鐩墠浠呮敮鎸?VisioBraille 璁惧銆?
Samuel Thibault <samuel.thibault@ens-lyon.org>
