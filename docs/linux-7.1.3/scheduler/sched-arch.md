## 闈㈠悜鏋舵瀯鐗瑰畾浠ｇ爜鐨?CPU 璋冨害鍣ㄥ疄鐜版彁绀?

	Nick Piggin, 2005

## 涓婁笅鏂囧垏鎹?
1. 杩愯闃熷垪锛圧unqueue锛夊姞閿?榛樿鎯呭喌涓嬶紝switch_to 鏋舵瀯鍑芥暟鏄湪杩愯闃熷垪鍔犻攣鐨勬儏鍐典笅璋冪敤鐨勩€傝繖閫氬父涓嶆槸闂锛岄櫎闈?switch_to 鍙兘闇€瑕佽幏鍙栬繍琛岄槦鍒楅攣銆傝繖閫氬父鏄敱浜庝笂涓嬫枃鍒囨崲涓殑鍞ら啋鎿嶄綔鎵€鑷淬€?
瑕佽姹傝皟搴﹀櫒鍦ㄦ湭鍔犻攣杩愯闃熷垪鐨勬儏鍐典笅璋冪敤 switch_to锛屼綘蹇呴』鍦ㄥご鏂囦欢锛堥€氬父鏄畾涔?switch_to
鐨勬枃浠讹級涓?`#define __ARCH_WANT_UNLOCKED_CTXSW`銆?
鏈姞閿佺殑涓婁笅鏂囧垏鎹㈠湪 CONFIG_SMP 鎯呭喌涓嬪彧浼氬鏍稿績璋冨害鍣ㄥ疄鐜板紩鍏ラ潪甯稿井灏忕殑鎬ц兘寮€閿€銆?
## CPU 绌洪棽

浣犵殑 cpu_idle 渚嬬▼闇€瑕侀伒瀹堜互涓嬭鍒欙細

1. 鎶㈠崰锛坧reempt锛夌幇鍦ㄥ簲鍦ㄧ┖闂蹭緥绋嬫湡闂翠繚鎸佺鐢ㄣ€傚彧搴斿湪璋冪敤 schedule() 鏃跺惎鐢紝闅忓悗鍐嶆绂佺敤銆?
2. need_resched/TIF_NEED_RESCHED 鍙細琚缃紝骞朵笖鍦ㄨ繍琛屼换鍔¤皟鐢?schedule() 涔嬪墠姘歌繙涓嶄細
   琚竻闄ゃ€傜┖闂茬嚎绋嬪彧闇€瑕佹煡璇?need_resched锛岃€岀粷涓嶅簲璁剧疆鎴栨竻闄ゅ畠銆?
3. 褰?cpu_idle 鍙戠幇锛坣eed_resched() == 'true'锛夋椂锛屽畠搴斿綋璋冪敤 schedule()銆傚湪鍏朵粬鎯呭喌涓?   涓嶅簲璋冪敤 schedule()銆?
4. 妫€鏌?need_resched 鏃堕渶瑕佺鐢ㄤ腑鏂殑鍞竴鏃舵満锛屾槸褰撴垜浠嵆灏嗚澶勭悊鍣ㄤ紤鐪犵洿鍒颁笅涓€娆′腑鏂椂
   锛堣繖骞朵笉鎻愪緵瀵?need_resched 鐨勪换浣曚繚鎶わ紝瀹冮槻姝涪澶变竴涓腑鏂級锛?
```

	        local_irq_disable();
	        if (!need_resched()) {
	                local_irq_enable();
	                *** resched interrupt arrives here ***
	                __asm__("sleep until next interrupt");
	        }

```
5. TIF_POLLING_NRFLAG 鍙敱绌洪棽渚嬬▼璁剧疆锛屽綋 need_resched 鍙橀珮鏃跺畠浠笉闇€瑕佷腑鏂潵鍞ら啋銆?   鎹㈠彞璇濊锛屽畠浠繀椤诲懆鏈熸€у湴杞 need_resched锛屽敖绠¤繘鍏ヨ緝浣庣殑 CPU 浼樺厛绾ф垨鍋氫竴浜涘悗鍙板伐浣?   鍙兘鏄悎鐞嗙殑銆?
      - 5a. 濡傛灉璁剧疆浜?TIF_POLLING_NRFLAG锛屽苟涓旀垜浠‘瀹炲喅瀹氳繘鍏ヤ腑鏂紤鐪狅紝鍒欓渶瑕佸厛娓呴櫎瀹冿紝
	鐒跺悗鍙戝嚭涓€涓唴瀛樺睆闅滐紙闅忓悗鍦ㄧ鐢ㄤ腑鏂殑鎯呭喌涓嬫祴璇?need_resched锛屽绗?3 鐐规墍杩帮級銆?
arch/x86/kernel/process.c 涓寘鍚疆璇笌浼戠湢涓ょ绌洪棽鍑芥暟鐨勭ず渚嬨€?

## 鍙兘鐨勬灦鏋勯棶棰?

鎴戝彂鐜扮殑鍙兘鐨勬灦鏋勯棶棰橈紙瑕佷箞灏濊瘯淇锛岃涔堟湭淇锛夛細

sparc - 姝ゆ椂涓柇宸插紑鍚紙?锛夛紝灏?local_irq_save 鏀逛负 _disable銆?      - TODO锛氶渶瑕佹瑕?CPU 绂佺敤鎶㈠崰锛堝弬瑙?#1锛?