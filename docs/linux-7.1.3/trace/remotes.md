
## 杩借釜 Remotes锛堣繙绋嬭拷韪級


:Author: Vincent Donnefort <vdonnefort@google.com>

## 姒傝堪

鍥轰欢鍜岃櫄鎷熸満鐩戞帶鍣紙hypervisor锛夊鍐呮牳鏉ヨ閮芥槸榛戠洅銆傝兘澶熺湅鍒板畠浠湪鍋氫粈涔堬紝瀵硅皟璇曚袱鑰呴兘寰堟湁鐢ㄣ€傝繖姝ｆ槸杩滅▼杩借釜缂撳啿鍖猴紙remote tracing buffer锛夌殑鐢ㄦ涔嬪湴銆傝繙绋嬭拷韪紦鍐插尯鏄敱鍥轰欢鎴栬櫄鎷熸満鐩戞帶鍣ㄥ湪鏄犲皠鍒颁富鏈哄唴鏍哥殑鍐呭瓨涓墽琛岀殑涓€涓幆褰㈢紦鍐插尯銆傝繖绫讳技浜庣敤鎴风┖闂村唴瀛樻槧灏勫唴鏍哥幆褰㈢紦鍐插尯鐨勬柟寮忥紝浣嗗湪杩欑鎯呭喌涓嬪唴鏍告壆婕旂殑鏄敤鎴风┖闂寸殑瑙掕壊锛岃€屽浐浠舵垨铏氭嫙鏈虹洃鎺у櫒鍒欐槸"鍐呮牳"涓€渚с€傚€熷姪杩滅▼杩借釜鐜舰缂撳啿鍖猴紝鍥轰欢鍜岃櫄鎷熸満鐩戞帶鍣ㄥ彲浠ヨ褰曚簨浠讹紝涓绘満鍐呮牳鑳藉鐪嬪埌杩欎簺浜嬩欢骞跺皢鍏舵毚闇茬粰鐢ㄦ埛绌洪棿銆?
## 娉ㄥ唽涓€涓?remote

涓€涓?remote 蹇呴』鎻愪緵涓€缁勫洖璋冨嚱鏁?`struct trace_remote_callbacks`锛屽叾鎻忚堪瑙佷笅鏂囥€傝繖浜涘洖璋冨厑璁?Tracefs 鍚敤鍜岀鐢ㄨ拷韪笌浜嬩欢銆佸姞杞藉拰鍗歌浇杩借釜缂撳啿鍖猴紙涓€缁勭幆褰㈢紦鍐插尯锛夛紝浠ュ強涓庡ご椤典氦鎹竴涓鍙栧櫒椤碉紝浠庤€屽疄鐜版秷璐瑰紡璇诲彇銆?
涓€鏃︽敞鍐岋紝璇?remote 鐨勪竴涓疄渚嬪氨浼氬嚭鐜板湪 Tracefs 鐩綍 **remotes/** 涓嬨€傜劧鍚庡彲浠ヤ娇鐢ㄥ父瑙勭殑 Tracefs 鏂囦欢 **trace_pipe** 鍜?**trace** 鏉ヨ鍙栫紦鍐插尯銆?
## 澹版槑涓€涓?remote 浜嬩欢

鎻愪緵浜嗕竴浜涘畯鏉ョ畝鍖?remote 浜嬩欢鐨勫０鏄庯紝鍏舵柟寮忎笌鍐呮牳鍐呬簨浠剁被浼笺€傚０鏄庡繀椤绘彁渚?ID銆佷簨浠跺弬鏁扮殑鎻忚堪浠ュ強浜嬩欢鐨勬墦鍗版柟寮忥細

	REMOTE_EVENT(foo, EVENT_FOO_ID,
		RE_STRUCT(
			re_field(u64, bar)
		),
		RE_PRINTK("bar=%lld", __entry->bar)
	);

鐒跺悗蹇呴』鍦?C 鏂囦欢涓娇鐢ㄤ互涓嬪唴瀹瑰０鏄庤繖浜涗簨浠讹細

	#define REMOTE_EVENT_INCLUDE_FILE foo_events.h
	#include <trace/define_remote_events.h>

杩欎細鎻愪緵涓€涓?`struct remote_event remote_event_foo`锛屽彲浠ユ妸瀹冧紶缁?`trace_remote_register`銆?
宸叉敞鍐岀殑浜嬩欢浼氬嚭鐜板湪 remote 鐩綍涓嬬殑 **events/** 涓€?
## 绠€鍗曠幆褰㈢紦鍐插尯

涓€涓幆褰㈢紦鍐插尯鍐欏叆绔殑绠€鍗曞疄鐜板彲浠ュ湪 kernel/trace/simple_ring_buffer.c 涓壘鍒般€?