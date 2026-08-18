## 瀹㈡埛鏈哄仠鏈鸿疆璇紙Guest halt polling锛?

cpuidle_haltpoll 椹卞姩閰嶅悎 haltpoll 璋冨害鍣紙governor锛夛紝鍏佽瀹㈡埛鏈?vcpu 鍦?鍋滄満锛坔alt锛変箣鍓嶈疆璇竴娈垫寚瀹氱殑鏃堕棿銆?
杩欑粰涓绘満涓€渚х殑杞甯︽潵浜嗕互涓嬪ソ澶勶細

 1) 鍦ㄨ疆璇㈡墽琛屾湡闂翠細璁剧疆 POLL 鏍囧織锛屼娇寰楄繙绋?vCPU 鍦ㄦ墽琛屽敜閱掓椂鍙互閬垮厤
	   鍙戦€?IPI锛堜互鍙婂鐞嗚 IPI 鐨勭浉鍏冲紑閿€锛夈€?
 2) 鍙互閬垮厤 VM-exit 鐨勫紑閿€銆?
瀹㈡埛鏈轰竴渚ц疆璇㈢殑缂虹偣鍦ㄤ簬锛屽嵆浣夸富鏈轰笂杩樻湁鍏跺畠鍙繍琛屼换鍔★紝涔熶細鎵ц杞銆?
鍩烘湰閫昏緫濡備笅锛氱敱涓€涓叏灞€鍊?guest_halt_poll_ns 鐢辩敤鎴烽厤缃紝琛ㄧず鍏佽杞鐨?鏈€闀挎椂闂淬€傝鍊兼槸鍥哄畾鐨勩€?
姣忎釜 vcpu 閮芥湁涓€涓彲璋冩暣鐨?guest_halt_poll_ns锛堚€滄瘡 cpu 鐨?guest_halt_poll_ns鈥濓級锛?鐢辩畻娉曟牴鎹簨浠讹紙濡備笅鎵€杩帮級杩涜璋冩暣銆?
## 妯″潡鍙傛暟


haltpoll 璋冨害鍣ㄦ湁 5 涓彲璋冩暣鐨勬ā鍧楀弬鏁帮細

1) guest_halt_poll_ns锛?
杞鍦ㄥ仠鏈哄墠鎵ц鐨勬渶闀挎椂闂达紙鍗曚綅绾崇锛夈€?
榛樿鍊硷細200000

2) guest_halt_poll_shrink锛?
褰撳敜閱掍簨浠跺彂鐢熷湪鍏ㄥ眬 guest_halt_poll_ns 涔嬪悗鏃讹紝鐢ㄤ簬鏀剁缉姣?cpu 鐨?guest_halt_poll_ns 鐨勯櫎娉曞洜瀛愩€?
榛樿鍊硷細2

3) guest_halt_poll_grow锛?
褰撲簨浠跺彂鐢熷湪姣?cpu 鐨?guest_halt_poll_ns 涔嬪悗銆佷絾鍦ㄥ叏灞€ guest_halt_poll_ns
涔嬪墠鏃讹紝鐢ㄤ簬澧為暱姣?cpu 鐨?guest_halt_poll_ns 鐨勪箻娉曞洜瀛愩€?
榛樿鍊硷細2

4) guest_halt_poll_grow_start锛?
鍦ㄧ┖闂茬郴缁熺殑鎯呭喌涓嬶紝姣?cpu 鐨?guest_halt_poll_ns 鏈€缁堜細闄嶅埌闆躲€傝鍊艰缃簡
澧為暱鏃剁殑鍒濆姣?cpu 鐨?guest_halt_poll_ns銆傚彲浠ヤ粠 10000 璧峰澶э紝浠ラ伩鍏嶅湪
鍒濆澧為暱闃舵鍑虹幇閬楁紡锛?
10k銆?0k銆?0k銆佲€︹€︼紙绀轰緥鍋囪 guest_halt_poll_grow=2锛夈€?
榛樿鍊硷細50000

5) guest_halt_poll_allow_shrink锛?
鍏佽鏀剁缉鐨勫竷灏斿弬鏁般€傝涓?N 鍙伩鍏嶆敹缂╋紙涓€鏃﹁揪鍒板叏灞€ guest_halt_poll_ns 鍊硷紝
姣?cpu 鐨?guest_halt_poll_ns 灏嗕繚鎸佽緝楂橈級銆?
榛樿鍊硷細Y

```

	/sys/module/haltpoll/parameters/

```
## 杩涗竴姝ヨ鏄?

- 璁剧疆 guest_halt_poll_ns 鍙傛暟鏃跺簲灏忓績锛屽洜涓鸿緝澶х殑鍊兼湁鍙兘灏嗕竴鍙版湰搴斿嚑涔?  瀹屽叏绌洪棽鐨勬満鍣ㄧ殑 cpu 浣跨敤鐜囨帹楂樺埌 100%銆?