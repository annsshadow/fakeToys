## AppArmor


## 浠€涔堟槸 AppArmor锛?


AppArmor 鏄?Linux 鍐呮牳鐨?MAC锛堝己鍒惰闂帶鍒讹級椋庢牸瀹夊叏鎵╁睍銆傚畠瀹炵幇浜嗕竴涓互浠诲姟涓轰腑蹇冪殑绛栫暐锛屼换鍔＄殑鈥滈厤缃枃浠垛€濅粠鐢ㄦ埛绌洪棿鍒涘缓骞跺姞杞姐€傜郴缁熶笂娌℃湁涓哄叾瀹氫箟閰嶇疆鏂囦欢鐨勪换鍔′互鏃犵害鏉燂紙unconfined锛夌姸鎬佽繍琛岋紝杩欑瓑鍚屼簬鏍囧噯 Linux DAC 鏉冮檺銆?

## 濡備綍鍚敤/绂佺敤


璁剧疆 `CONFIG_SECURITY_APPARMOR=y`

```
   CONFIG_DEFAULT_SECURITY_APPARMOR=y
```
CONFIG_LSM 鍙傛暟绠＄悊 LSM 鐨勯『搴忓拰閫夋嫨銆傚湪鍒楄〃涓皢 apparmor 鎸囧畾涓虹涓€涓€滀富瑕佲€濇ā鍧楋紙渚嬪 AppArmor銆丼ELinux銆丼mack锛夈€?

鏋勫缓鍐呮牳

濡傛灉 AppArmor 涓嶆槸榛樿瀹夊叏妯″潡锛屽彲浠ラ€氳繃鍦ㄥ唴鏍稿懡浠よ涓婁紶閫?`security=apparmor` 鏉ュ惎鐢ㄣ€?

濡傛灉 AppArmor 鏄粯璁ゅ畨鍏ㄦā鍧楋紝鍙互閫氳繃鍦ㄥ唴鏍稿懡浠よ涓婁紶閫?`apparmor=0, security=XXXX`锛堝叾涓?`XXXX` 鏄湁鏁堢殑瀹夊叏妯″潡锛夋潵绂佺敤銆?

涓轰簡璁?AppArmor 寮哄埗鎵ц瓒呭嚭鏍囧噯 Linux DAC 鏉冮檺涔嬪鐨勪换浣曢檺鍒讹紝蹇呴』灏嗙瓥鐣ヤ粠鐢ㄦ埛绌洪棿鍔犺浇鍒板唴鏍镐腑锛堣鍙傞槄涓嬫柟鐨勬枃妗ｅ拰宸ュ叿閾炬帴锛夈€?

## 鏂囨。


鏂囨。鍙互鍦ㄤ笅鏂归摼鎺ョ殑 wiki 涓壘鍒般€?

## 閾炬帴


閭欢鍒楄〃 - apparmor@lists.ubuntu.com

Wiki - http://wiki.apparmor.net

鐢ㄦ埛绌洪棿宸ュ叿 - https://gitlab.com/apparmor

鍐呮牳妯″潡 - git://git.kernel.org/pub/scm/linux/kernel/git/jj/linux-apparmor
