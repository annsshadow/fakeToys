## 鏄犲皠璁惧鐨勬棭鏈熷垱寤?

鍙互閫氳繃涓ょ鏂瑰紡灏?device-mapper 璁惧閰嶇疆涓虹郴缁熺殑鏍硅澶囥€?
绗竴绉嶆槸鏋勫缓涓€涓垵濮嬪唴瀛樼洏锛坕nitramfs锛夛紝瀹冨紩瀵煎埌涓€涓渶灏忕敤鎴风┖闂达紝璇ョ敤鎴风┖闂撮厤缃ソ璁惧锛岀劧鍚?pivot_root(8) 杩涘叆鍏朵腑銆?
绗簩绉嶆槸閫氳繃鍐呮牳鍚姩鍛戒护琛屽弬鏁帮紝浣跨敤妯″潡鍙傛暟 "dm-mod.create=" 鍒涘缓涓€涓垨澶氫釜 device-mapper銆?
鍏舵牸寮忔寚瀹氫负涓€涓敱閫楀彿鍒嗛殧銆佸彲閫変娇鐢ㄥ垎鍙风殑鏁版嵁瀛楃涓诧紝鍏朵腑锛?
 - 閫楀彿鐢ㄤ簬鍒嗛殧瀛楁锛屽 name銆乽uid銆乫lags 鍜?table锛堟寚瀹氫竴涓澶囷級
 - 鍒嗗彿鐢ㄤ簬鍒嗛殧璁惧銆?
```

 dm-mod.create=<name>,<uuid>,<minor>,<flags>,<table>[,<table>+][;<name>,<uuid>,<minor>,<flags>,<table>[,<table>+]+]

```
```

	<name>		::= 璁惧鍚嶇О銆?	<uuid>		::= xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | ""
	<minor>		::= 璁惧娆¤澶囧彿 | ""
	<flags>		::= "ro" | "rw"
	<table>		::= <start_sector> <num_sectors> <target_type> <target_args>
	<target_type>	::= "verity" | "linear" | ...锛堣涓嬭〃锛?
```
dm 琛屽簲绛変环浜?dmsetup 宸ュ叿浣跨敤 `--concise` 鍙傛暟鏃舵墍鐢ㄧ殑涓€琛屻€?
## 鐩爣绫诲瀷


骞堕潪鎵€鏈夌洰鏍囩被鍨嬮兘鍙敤锛屽洜涓哄湪鏈厛浣跨敤鐢ㄦ埛绌洪棿宸ュ叿妫€鏌ョ浉鍏冲厓鏁版嵁鏈夋晥鎬у氨婵€娲绘煇浜?DM 鐩爣鏃讹紝瀛樺湪
涓ラ噸椋庨櫓銆?
======================= =======================================================
`cache`			鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉缂撳瓨璁惧
`crypt`			鍏佽
`delay`			鍏佽
`era`			鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉鍏冩暟鎹澶?`flakey`		鍙楅檺锛岀敤浜庢祴璇?`linear`		鍏佽
`log-writes`		鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉鍏冩暟鎹澶?`mirror`		鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉涓?闀滃儚璁惧
`raid`			鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉鍏冩暟鎹澶?`snapshot`		鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉婧?鐩爣璁惧
`snapshot-origin`	鍏佽
`snapshot-merge`		鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉婧?鐩爣璁惧
`striped`		鍏佽
`switch`		鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉璁惧璺緞
`thin`			鍙楅檺锛岄渶瑕佹潵鑷敤鎴风┖闂寸殑 dm target 娑堟伅
`thin-pool`		鍙楅檺锛岄渶瑕佹潵鑷敤鎴风┖闂寸殑 dm target 娑堟伅
`verity`		鍏佽
`writecache`		鍙楅檺锛岀敤鎴风┖闂村簲楠岃瘉缂撳瓨璁惧
`zero`			鍙楅檺锛屼笉鐢ㄤ簬鏍规枃浠剁郴缁?======================= =======================================================

濡傛灉鐩爣绫诲瀷鏈湪涓婇潰鍒楀嚭锛屽垯榛樿鍙楅檺锛堟湭缁忔祴璇曪級銆?
## 绀轰緥


涓€涓紩瀵煎埌涓€涓敱鐢ㄦ埛鎬?Linux 鍧楄澶囩粍鎴愮殑绾挎€ч樀鍒楃殑绀轰緥
```

  dm-mod.create="lroot,,,rw, 0 4096 linear 98:16 0, 4096 4096 linear 98:32 0" root=/dev/dm-0

```
杩欏皢寮曞鍒颁竴涓敱 8192 涓墖鍖虹粍鎴愮殑璇诲啓 dm-linear 鐩爣锛岃鐩爣璺ㄨ秺涓や釜閫氳繃鍏朵富:娆¤澶囧彿鏍囪瘑鐨勫潡璁惧銆?鍚姩鍚庯紝udev 浼氭牴鎹鍒欏皢姝ょ洰鏍囬噸鍛藉悕涓?/dev/mapper/lroot銆傛病鏈夊垎閰?uuid銆?
澶氫釜 device-mapper 鐨勭ず渚嬶紝dm-mod.create="..." 鐨勫唴瀹?```

  dm-linear,,1,rw,
    0 32768 linear 8:1 0,
    32768 1024000 linear 8:2 0;
  dm-verity,,3,ro,
    0 1638400 verity 1 /dev/sdc1 /dev/sdc2 4096 4096 204800 1 sha256
    ac87db56303c9c1da433d7209b5a6ef3e4779df141200cbd7c157dcb8dd89c42
    5ebfe87f7df3235b80a117ebc4078e44f55045487ad4a96581d1adb564615b51

```
鍏朵粬绀轰緥锛堟寜鐩爣绫诲瀷锛夛細

```

  dm-crypt,,8,ro,
    0 1048576 crypt aes-xts-plain64
    babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe 0
    /dev/sda 0 1 allow_discards

```
```

  dm-delay,,4,ro,0 409600 delay /dev/sda1 0 500

```
```

  dm-linear,,,rw,
    0 32768 linear /dev/sda1 0,
    32768 1024000 linear /dev/sda2 0,
    1056768 204800 linear /dev/sda3 0,
    1261568 512000 linear /dev/sda4 0

```
```

  dm-snap-orig,,4,ro,0 409600 snapshot-origin 8:2

```
```

  dm-striped,,4,ro,0 1638400 striped 4 4096
  /dev/sda1 0 /dev/sda2 0 /dev/sda3 0 /dev/sda4 0

```
```

  dm-verity,,4,ro,
    0 1638400 verity 1 8:1 8:2 4096 4096 204800 1 sha256
    fb1a5a0f00deb908d8b53cb270858975e76cf64105d412ce764225d53b8f3cfd
    51934789604d1b92399c52e7cb149d1b3a1b74bbbcb103b2a0aaacbed5c08584

```
瀵逛簬鍦ㄥ紓姝ユ帰娴嬬殑鍧楄澶囷紙MMC銆乁SB 绛夛級涔嬩笂浣跨敤 device-mapper 鐨勮缃紝鍙兘闇€瑕佸憡璇?dm-init
鍦ㄥ缓绔?device-mapper 琛ㄤ箣鍓嶆樉寮忕瓑寰呭畠浠彉涓哄彲鐢ㄣ€傝繖鍙互閫氳繃 "dm-mod.waitfor=" 瀹屾垚
```

  dm-mod.waitfor=<device1>[,..,<deviceN>]

```
