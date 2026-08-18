## 浣跨敤 Linux 鍐呮牳璺熻釜鐐癸紙Tracepoints锛?


:Author: Mathieu Desnoyers


鏈枃妗ｄ粙缁?Linux 鍐呮牳璺熻釜鐐瑰強鍏朵娇鐢ㄦ柟娉曘€傚畠鎻愪緵浜嗗浣曞湪鍐呮牳涓彃鍏ヨ窡韪偣銆佸苟灏嗘帰娴嬪嚱鏁拌繛鎺ュ埌杩欎簺璺熻釜鐐圭殑绀轰緥锛屽悓鏃剁粰鍑轰簡涓€浜涙帰娴嬪嚱鏁扮殑渚嬪瓙銆?


### 璺熻釜鐐圭殑鐢ㄩ€?


鏀剧疆鍦ㄤ唬鐮佷腑鐨勮窡韪偣鎻愪緵浜嗕竴涓挬瀛愶紝鐢ㄤ簬璋冪敤涓€涓綘鍙互鍦ㄨ繍琛屾椂鎻愪緵鐨勫嚱鏁帮紙鎺㈤拡锛夈€備竴涓窡韪偣鍙互鏄€滃紑鍚€濓紙宸茶繛鎺ユ帰閽堬級鎴栤€滃叧闂€濓紙鏈檮鍔犳帰閽堬級鐘舵€併€傚綋璺熻釜鐐逛负鈥滃叧闂€濇椂锛屽畠闄や簡甯︽潵寰皬鐨勬椂闂村紑閿€锛堟鏌ュ垎鏀潯浠讹級鍜岀┖闂村紑閿€锛堝湪琚彃妗╃殑鍑芥暟鏈熬娣诲姞鐢ㄤ簬鍑芥暟璋冪敤鐨勫嚑涓瓧鑺傦紝骞跺湪涓€涓嫭绔嬬殑娈典腑娣诲姞鏁版嵁缁撴瀯锛夊锛屾病鏈夊叾瀹冨奖鍝嶃€傚綋璺熻釜鐐逛负鈥滃紑鍚€濇椂锛屼綘鎻愪緵鐨勫嚱鏁颁細鍦ㄦ瘡娆¤窡韪偣鎵ц鏃惰璋冪敤锛屼笖澶勪簬璋冪敤鑰呯殑鎵ц涓婁笅鏂囦腑銆傚綋鎵€鎻愪緵鍑芥暟鎵ц缁撴潫鏃讹紝瀹冧細杩斿洖鍒拌皟鐢ㄨ€咃紙浠庤窡韪偣浣嶇疆缁х画鎵ц锛夈€?

浣犲彲浠ュ湪浠ｇ爜涓殑閲嶈浣嶇疆鏀剧疆璺熻釜鐐广€傚畠浠槸杞婚噺绾х殑閽╁瓙锛屽彲浠ヤ紶閫掍换鎰忔暟閲忕殑鍙傛暟锛屽叾鍘熷瀷鍦ㄦ斁鍦ㄥご鏂囦欢涓殑璺熻釜鐐瑰０鏄庨噷鎻忚堪銆?

瀹冧滑鍙敤浜庤拷韪拰鎬ц兘缁熻銆?


### 鐢ㄦ硶


浣跨敤璺熻釜鐐归渶瑕佷袱涓绱狅細

- 鏀剧疆鍦ㄥご鏂囦欢涓殑璺熻釜鐐瑰畾涔夈€?
- 浣嶄簬 C 浠ｇ爜涓殑璺熻釜鐐硅鍙ャ€?

涓轰簡浣跨敤璺熻釜鐐癸紝浣犲簲璇ュ寘鍚?linux/tracepoint.h銆?

```

	#undef TRACE_SYSTEM
	#define TRACE_SYSTEM subsys

	#if !defined(_TRACE_SUBSYS_H) || defined(TRACE_HEADER_MULTI_READ)
	#define _TRACE_SUBSYS_H

	#include <linux/tracepoint.h>

	DECLARE_TRACE(subsys_eventname,
		TP_PROTO(int firstarg, struct task_struct *p),
		TP_ARGS(firstarg, p));

	#endif /* _TRACE_SUBSYS_H */

	/* This part must be outside protection */
	#include <trace/define_trace.h>

```
```

	#include <trace/events/subsys.h>

	#define CREATE_TRACE_POINTS
	DEFINE_TRACE(subsys_eventname);

	void somefct(void)
	{
		...
		trace_subsys_eventname_tp(arg, task);
		...
	}

```
鍏朵腑锛?
  - subsys_eventname 鏄綘鐨勪簨浠朵腑鍞竴鐨勬爣璇嗙

    - subsys 鏄綘鐨勫瓙绯荤粺鍚嶇О銆?
    - eventname 鏄杩借釜鐨勪簨浠跺悕绉般€?

  - `TP_PROTO(int firstarg, struct task_struct *p)` 鏄璺熻釜鐐规墍璋冪敤鍑芥暟鐨勫師鍨嬨€?

  - `TP_ARGS(firstarg, p)` 鏄弬鏁板悕绉帮紝涓庡師鍨嬩腑鐨勭浉鍚屻€?

  - 濡傛灉浣犲湪澶氫釜婧愭枃浠朵腑浣跨敤璇ュご鏂囦欢锛宍#define CREATE_TRACE_POINTS` 搴旇鍙嚭鐜板湪涓€涓簮鏂囦欢涓€?

灏嗕竴涓嚱鏁帮紙鎺㈤拡锛夎繛鎺ュ埌涓€涓窡韪偣锛屾槸閫氳繃涓虹壒瀹氳窡韪偣鎻愪緵涓€涓帰閽堬紙瑕佽皟鐢ㄧ殑鍑芥暟锛夋潵瀹屾垚鐨勶紝浣跨敤 register_trace_subsys_eventname()銆傜Щ闄ゆ帰閽堝垯閫氳繃 unregister_trace_subsys_eventname() 瀹屾垚锛涘畠浼氱Щ闄よ鎺㈤拡銆?

蹇呴』鍦ㄦā鍧楅€€鍑哄嚱鏁扮粨鏉熶箣鍓嶈皟鐢?tracepoint_synchronize_unregister()锛屼互纭繚娌℃湁璋冪敤鑰呬粛鍦ㄤ娇鐢ㄨ鎺㈤拡銆傝繖涓€鐐癸紝鍔犱笂鍦ㄦ帰閽堣皟鐢ㄥ懆鍥寸鐢ㄤ簡鎶㈠崰锛屼繚璇佷簡鎺㈤拡绉婚櫎鍜屾ā鍧楀嵏杞界殑瀹夊叏鎬с€?

璺熻釜鐐规満鍒舵敮鎸佹彃鍏ュ悓涓€涓窡韪偣鐨勫涓疄渚嬶紝浣嗗繀椤诲鏁翠釜鍐呮牳涓殑缁欏畾璺熻釜鐐瑰悕绉板彧鍋氫竴娆″畾涔夛紝浠ョ‘淇濅笉浼氬彂鐢熺被鍨嬪啿绐併€傝窡韪偣鐨勫悕绉版敼鍐欙紙name mangling锛変娇鐢ㄥ師鍨嬫潵瀹屾垚锛屼互纭繚绫诲瀷姝ｇ‘銆傛帰娴嬬被鍨嬫纭€х殑楠岃瘉鐢辩紪璇戝櫒鍦ㄦ敞鍐屽瀹屾垚銆傝窡韪偣鍙互鏀惧湪鍐呰仈鍑芥暟銆佸唴鑱旈潤鎬佸嚱鏁般€佸睍寮€寰幆浠ュ強甯歌鍑芥暟涓€?

杩欓噷寤鸿閲囩敤鈥渟ubsys_event鈥濆懡鍚嶆柟妗堜綔涓轰竴绉嶇害瀹氾紝浠ラ檺鍒跺悕绉板啿绐併€傝窡韪偣鍚嶇О瀵规暣涓唴鏍告槸鍏ㄥ眬鐨勶細鏃犺瀹冧滑浣嶄簬鏍稿績鍐呮牳鏄犲儚杩樻槸妯″潡涓紝閮借瑙嗕负鐩稿悓鐨勩€?

濡傛灉璺熻釜鐐硅鍦ㄥ唴鏍告ā鍧椾腑浣跨敤锛屽彲浠ヤ娇鐢?EXPORT_TRACEPOINT_SYMBOL_GPL() 鎴?EXPORT_TRACEPOINT_SYMBOL() 鏉ュ鍑哄凡瀹氫箟鐨勮窡韪偣銆?

濡傛灉浣犻渶瑕佷负鏌愪釜璺熻釜鐐瑰弬鏁板仛涓€鐐瑰伐浣滐紝鑰岃宸ヤ綔浠呯敤浜庤璺熻釜鐐癸紝鍒欏彲浠ュ皢璇ュ伐浣滃皝瑁?
```

	if (trace_foo_bar_enabled()) {
		int i;
		int tot = 0;

		for (i = 0; i < count; i++)
			tot += calculate_nuggets();

		trace_foo_bar_tp(tot);
	}

```
鎵€鏈?trace_<tracepoint>_tp() 璋冪敤閮芥湁涓€涓尮閰嶇殑 trace_<tracepoint>_enabled() 鍑芥暟锛屽綋璺熻釜鐐瑰惎鐢ㄦ椂杩斿洖 true锛屽惁鍒欒繑鍥?false銆倀race_<tracepoint>_tp() 搴斿缁堜綅浜?if (trace_<tracepoint>_enabled()) 鍧楀唴閮紝浠ラ槻姝㈣窡韪偣琚惎鐢ㄤ笌妫€鏌ヨ瑙傚療鍒颁箣闂村彂鐢熺珵鎬併€?

浣跨敤 trace_<tracepoint>_enabled() 鐨勪紭鍔垮湪浜庯紝瀹冨埄鐢ㄨ窡韪偣鐨?static_key 浣?if 璇彞鍙互閫氳繃璺宠浆鏍囩锛坖ump labels锛夊疄鐜帮紝浠庤€岄伩鍏嶆潯浠跺垎鏀€?

      瀹氫箟璺熻釜鐐广€傛敞鎰忥紝DECLARE_TRACE(foo) 浼氬垱寤轰竴涓悕涓?"trace_foo_tp()" 鐨勫嚱鏁帮紝鑰?TRACE_EVENT(foo) 浼氬垱寤轰竴涓悕涓?"trace_foo()" 鐨勫嚱鏁帮紝鍚屾椂杩樹細鍦?/sys/kernel/tracing/events 鐩綍涓嬪皢璇ヨ窡韪偣浣滀负璺熻釜浜嬩欢鏆撮湶鍑烘潵銆傛洿澶氱粏鑺傝鍙傞槄 http://lwn.net/Articles/379903銆乭ttp://lwn.net/Articles/381064 鍜?http://lwn.net/Articles/383362 绯诲垪鏂囩珷銆?

濡傛灉浣犻渶瑕佷粠澶存枃浠朵腑璋冪敤璺熻釜鐐癸紝涓嶅缓璁洿鎺ヨ皟鐢ㄦ垨浣跨敤 trace_<tracepoint>_enabled() 鍑芥暟璋冪敤锛屽洜涓哄綋澶存枃浠惰璁剧疆浜?CREATE_TRACE_POINTS 鐨勬枃浠跺寘鍚椂锛屽ご鏂囦欢涓殑璺熻釜鐐瑰彲鑳戒骇鐢熷壇浣滅敤锛岃€屼笖 trace_<tracepoint>() 鍐呰仈鍑芥暟骞朵笉绠楀皬锛屽鏋滆鍏跺畠鍐呰仈鍑芥暟浣跨敤浼氫娇鍐呮牳鑶ㄨ儉銆傜浉鍙嶏紝搴斿綋鍖呭惈 tracepoint-defs.h 骞朵娇鐢?tracepoint_enabled()銆?

```

	void do_trace_foo_bar_wrapper(args)
	{
		trace_foo_bar_tp(args); // for tracepoints created via DECLARE_TRACE
					//   or
		trace_foo_bar(args);    // for tracepoints created via TRACE_EVENT
	}

```
```

	DECLARE_TRACEPOINT(foo_bar);

	static inline void some_inline_function()
	{
		[..]
		if (tracepoint_enabled(foo_bar))
			do_trace_foo_bar_wrapper(args);
		[..]
	}

```