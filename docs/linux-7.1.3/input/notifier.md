## 閿洏閫氱煡鍣?

鍙互浣跨敤 register_keyboard_notifier 鍦ㄩ敭鐩樹簨浠跺彂鐢熸椂鑾峰緱鍥炶皟
锛堣瑙?kbd_keycode() 鍑芥暟锛夈€備紶鍏ョ殑缁撴瀯浣撲负 keyboard_notifier_param
锛堝弬瑙?<linux/keyboard.h>锛夛細

- 'vc' 濮嬬粓鎻愪緵璇ラ敭鐩樹簨浠舵墍閫傜敤鐨勮櫄鎷熸帶鍒跺彴锛圴C锛夛紱
- 'down' 瀵逛簬鎸夐敭浜嬩欢涓?1锛屽浜庢澗寮€浜嬩欢涓?0锛?- 'shift' 涓哄綋鍓嶄慨楗伴敭鐘舵€侊紝鎺╃爜浣嶇储寮曚负 KG_*锛?- 'ledstate' 涓哄綋鍓?LED 鐘舵€侊紱
- 'value' 鍙栧喅浜庝簨浠剁被鍨嬨€?
- KBD_KEYCODE 浜嬩欢鎬绘槸鍦ㄥ叾浠栦簨浠朵箣鍓嶅彂閫侊紝value 涓洪敭鐮併€?- KBD_UNBOUND_KEYCODE 浜嬩欢鍦ㄩ敭鐮佹湭缁戝畾鍒版煇涓?keysym 鏃跺彂閫併€?  value 涓洪敭鐮併€?- KBD_UNICODE 浜嬩欢鍦?閿爜 -> keysym 杞崲浜х敓涓€涓?  unicode 瀛楃鏃跺彂閫併€倂alue 涓鸿 unicode 鍊笺€?- KBD_KEYSYM 浜嬩欢鍦?閿爜 -> keysym 杞崲浜х敓涓€涓?  闈?unicode 瀛楃鏃跺彂閫併€倂alue 涓鸿 keysym銆?- KBD_POST_KEYSYM 浜嬩欢鍦ㄥ鐞嗗畬闈?unicode keysym 涔嬪悗鍙戦€併€?  杩欏厑璁镐緥濡傛鏌ユ渶缁堝緱鍒扮殑 LED銆?
瀵逛簬闄ゆ渶鍚庝竴绉嶅鐨勬瘡绉嶄簨浠讹紝鍥炶皟鍙互杩斿洖 NOTIFY_STOP 浠モ€滃悆鎺夆€濊浜嬩欢锛?閫氱煡寰幆琚仠姝紝閿洏浜嬩欢琚涪寮冦€?
```

    kbd_keycode(keycode) {
	...
	params.value = keycode;
	if (notifier_call_chain(KBD_KEYCODE,&params) == NOTIFY_STOP)
	    || !bound) {
		notifier_call_chain(KBD_UNBOUND_KEYCODE,&params);
		return;
	}

	if (unicode) {
		param.value = unicode;
		if (notifier_call_chain(KBD_UNICODE,&params) == NOTIFY_STOP)
			return;
		emit unicode;
		return;
	}

	params.value = keysym;
	if (notifier_call_chain(KBD_KEYSYM,&params) == NOTIFY_STOP)
		return;
	apply keysym;
	notifier_call_chain(KBD_POST_KEYSYM,&params);
    }

```
