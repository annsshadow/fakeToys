
### Apple 瑙︽懜鏉块┍鍔紙appletouch锛?

:Copyright: |copy| 2005 Stelian Pop <stelian@popies.net>

appletouch 鏄竴涓?Linux 鍐呮牳椹卞姩锛岀敤浜?2005 骞?2 鏈堝拰 2005 骞?10 鏈堜箣鍚庡嚭浜х殑 Apple Aluminium Powerbook 涓婃墍閰嶅鐨?USB 瑙︽懜鏉裤€?
璇ラ┍鍔ㄦ淳鐢熻嚜 Johannes Berg 鐨?appletrackpad 椹卞姩 [#f1]_锛屼絾鍦ㄤ竴浜涙柟闈㈣繘琛屼簡鏀硅繘锛?
 - appletouch 鏄竴涓畬鏁寸殑鍐呮牳椹卞姩锛屾棤闇€浠讳綍鐢ㄦ埛绌洪棿绋嬪簭
 - appletouch 鍙互涓?synaptics X11 椹卞姩瀵规帴锛屼粠鑰岃幏寰楄Е鎽告澘鍔犻€熴€佹粴鍔ㄧ瓑鍔熻兘

鎰熻阿 Johannes Berg 瀵硅Е鎽告澘鍗忚杩涜鐨勯€嗗悜宸ョ▼锛孎rank Arnold 鐨勮繘涓€姝ュ畬鍠勶紝浠ュ強 Alex Harper 鎻愪緵鐨勫叧浜庤Е鎽告澘浼犳劅鍣ㄥ唴閮ㄥ伐浣滃師鐞嗙殑棰濆淇℃伅銆侻ichael Hanselmann 澧炲姞浜嗗 2005 骞?10 鏈堝瀷鍙风殑鏀寔銆?
### 鐢ㄦ硶


瑕佸湪鍩烘湰妯″紡涓嬩娇鐢ㄨЕ鎽告澘锛岀紪璇戦┍鍔ㄥ苟鍔犺浇妯″潡銆傜郴缁熷皢妫€娴嬪埌涓€涓柊鐨勮緭鍏ヨ澶囷紝浣犲彲浠ヤ粠 /dev/input/mice 璇诲彇榧犳爣鏁版嵁锛堜娇鐢?gpm 鎴?X11锛夈€?
鍦?X11 涓紝浣犲彲浠ュ皢瑙︽懜鏉块厤缃负浣跨敤 synaptics X11 椹卞姩锛屼粠鑰岃幏寰楅澶栫殑鍔熻兘锛屽鍔犻€熴€佹粴鍔ㄣ€佸弻鎸囩偣鍑绘ā鎷熶腑閿€佷笁鎸囩偣鍑绘ā鎷熷彸閿瓑銆備负姝わ紝璇风‘淇濅綘浣跨敤鐨勬槸杈冩柊鐗堟湰鐨?synaptics 椹卞姩锛堝凡鍦?0.14.2 涓婃祴璇曪紝鍙粠 [#f2]_ 鑾峰彇锛夛紝骞跺湪浣犵殑 X11 閰嶇疆鏂囦欢涓厤缃竴涓柊鐨勮緭鍏ヨ澶囷紙閰嶇疆绀轰緥瑙佷笅

```

	Section "InputDevice"
		Identifier      "Synaptics Touchpad"
		Driver          "synaptics"
		Option          "SendCoreEvents"        "true"
		Option          "Device"                "/dev/input/mice"
		Option          "Protocol"              "auto-dev"
		Option		"LeftEdge"		"0"
		Option		"RightEdge"		"850"
		Option		"TopEdge"		"0"
		Option		"BottomEdge"		"645"
		Option		"MinSpeed"		"0.4"
		Option		"MaxSpeed"		"1"
		Option		"AccelFactor"		"0.02"
		Option		"FingerLow"		"0"
		Option		"FingerHigh"		"30"
		Option		"MaxTapMove"		"20"
		Option		"MaxTapTime"		"100"
		Option		"HorizScrollDelta"	"0"
		Option		"VertScrollDelta"	"30"
		Option		"SHMConfig"		"on"
	EndSection

	Section "ServerLayout"
		...
		InputDevice	"Mouse"
		InputDevice	"Synaptics Touchpad"
	...
	EndSection

```

### 鎶栧姩闂


瑙︽懜鏉夸紶鎰熷櫒瀵圭儹閲忛潪甯告晱鎰燂紝褰撴俯搴﹀彉鍖栨椂浼氫骇鐢熷ぇ閲忓櫔澹般€傞娆＄粰绗旇鏈數鑴戜笂鐢垫椂灏ゅ叾鏄庢樉銆?
appletouch 椹卞姩浼氬皾璇曞鐞嗘鍣０骞惰嚜鍔ㄩ€傚簲锛屼絾瀹冨苟闈炲畬缇庛€傚鏋滄墜鎸囩Щ鍔ㄤ笉鍐嶈璇嗗埆锛岃灏濊瘯閲嶆柊鍔犺浇椹卞姩銆?
浣犲彲浠ヤ娇鐢?'debug' 妯″潡鍙傛暟寮€鍚皟璇曘€傚€间负 0 鍏抽棴鎵€鏈夎皟璇曪紝1 寮€鍚鏃犳晥閲囨牱鐨勮窡韪紝2 寮€鍚?
```

	modprobe appletouch debug=1

```

```

	echo "1" > /sys/module/appletouch/parameters/debug


```
