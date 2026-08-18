## 鍐呮牳妯″紡璁剧疆 (KMS)


椹卞姩蹇呴』閫氳繃鍦?DRM 璁惧涓婅皟鐢?drmm_mode_config_init() 鏉ュ垵濮嬪寲妯″紡璁剧疆鏍稿績銆傝鍑芥暟鍒濆鍖?`struct drm_device <drm_device>` 鐨?mode_config 瀛楁锛屼笖姘歌繙涓嶄細澶辫触銆傚畬鎴愬悗锛屽繀椤婚€氳繃鍒濆鍖栦互涓嬪瓧娈垫潵寤虹珛妯″紡閰嶇疆銆?

- int min_width, min_height; int max_width, max_height;
   甯х紦鍐诧紙frame buffer锛夌殑鏈€灏忓拰鏈€澶у搴︿笌楂樺害锛屼互鍍忕礌涓哄崟浣嶃€?

- struct drm_mode_config_funcs \*funcs;
   妯″紡璁剧疆鍑芥暟銆?

## 姒傝堪


   :alt: KMS 鏄剧ず娴佹按绾?
   :caption: KMS 鏄剧ず娴佹按绾挎瑙?

   digraph "KMS" {
      node [shape=box]

      subgraph cluster_static {
          style=dashed
          label="Static Objects"

          node [bgcolor=grey style=filled]
          "drm_plane A" -> "drm_crtc"
          "drm_plane B" -> "drm_crtc"
          "drm_crtc" -> "drm_encoder A"
          "drm_crtc" -> "drm_encoder B"
      }

      subgraph cluster_user_created {
          style=dashed
          label="Userspace-Created"

          node [shape=oval]
          "drm_framebuffer 1" -> "drm_plane A"
          "drm_framebuffer 2" -> "drm_plane B"
      }

      subgraph cluster_connector {
          style=dashed
          label="Hotpluggable"

          "drm_encoder A" -> "drm_connector A"
          "drm_encoder B" -> "drm_connector B"
      }
   }

KMS 鍚戠敤鎴风┖闂村憟鐜扮殑鍩烘湰瀵硅薄缁撴瀯鐩稿綋绠€鍗曘€傚抚缂撳啿锛堢敱 `struct drm_framebuffer <drm_framebuffer>` 琛ㄧず锛屽弬瑙?`Frame Buffer Abstraction`_锛夎緭鍏ュ埌 plane 涓€侾lane 鐢?`struct drm_plane <drm_plane>` 琛ㄧず锛屾洿澶氱粏鑺傚弬瑙?`Plane Abstraction`_銆備竴涓垨澶氫釜锛堢敋鑷抽浂涓級plane 灏嗗叾鍍忕礌鏁版嵁閫佸叆涓€涓?CRTC锛堢敱 `struct drm_crtc <drm_crtc>` 琛ㄧず锛屽弬瑙?`CRTC Abstraction`_锛夎繘琛屾贩鍚堬紙blending锛夈€傜簿纭殑娣峰悎姝ラ鍦?`Plane Composition Properties`_ 鍙婄浉鍏崇珷鑺備腑鏈夋洿璇︾粏鐨勮鏄庛€?

鍦ㄨ緭鍑鸿矾鐢辨柟闈紝绗竴姝ユ槸 encoder锛堢敱 `struct drm_encoder <drm_encoder>` 琛ㄧず锛屽弬瑙?`Encoder Abstraction`_锛夈€傝繖浜涘疄闄呬笂鍙槸鐢ㄤ簬瀹炵幇 KMS 椹卞姩鐨勮緟鍔╁簱鐨勫唴閮ㄤ骇鐗┿€傞櫎姝や箣澶栵紝瀹冧滑璁╃敤鎴风┖闂存洿闅句互寮勬竻妤?CRTC 涓?connector 涔嬮棿鍝簺杩炴帴鏄彲鑳界殑銆佹敮鎸佷綍绉嶅厠闅嗭紙cloning锛夛紝瀹冧滑鍦ㄧ敤鎴风┖闂?API 涓鏃犵敤澶勩€傞仐鎲剧殑鏄?encoder 宸茬粡鏆撮湶缁欎簡鐢ㄦ埛绌洪棿锛屽洜姝ょ洰鍓嶆棤娉曠Щ闄ゅ畠浠€傛澶栵紝鏆撮湶鐨勯檺鍒剁粡甯镐細琚┍鍔ㄩ敊璇湴璁剧疆锛屽苟涓斿湪寰堝鎯呭喌涓嬩笉瓒充互琛ㄨ揪鐪熸鐨勯檺鍒躲€備竴涓?CRTC 鍙互杩炴帴鍒板涓?encoder锛岃€屽浜庝竴涓浜庢椿鍔ㄧ姸鎬佺殑 CRTC 鑰岃█锛屽繀椤昏嚦灏戞湁涓€涓?encoder銆?

鏄剧ず閾句腑鏈€缁堢殑銆佷篃鏄湡姝ｇ殑绔偣鏄?connector锛堢敱 `struct drm_connector <drm_connector>` 琛ㄧず锛屽弬瑙?`Connector Abstraction`_锛夈€侰onnector 鍙互鏈変笉鍚岀殑鍙敤 encoder锛屼絾鍐呮牳椹卞姩浼氫负姣忎釜 connector 閫夋嫨浣跨敤鍝釜 encoder銆傚叾鐢ㄤ緥鏄?DVI锛屽畠鍙互鍦ㄦā鎷熷拰鏁板瓧 encoder 涔嬮棿鍒囨崲銆侲ncoder 涔熷彲浠ラ┍鍔ㄥ涓笉鍚岀殑 connector銆傛瘡涓椿鍔?encoder 鎭板ソ瀵瑰簲涓€涓椿鍔?connector銆?

鍦ㄥ唴閮紝杈撳嚭娴佹按绾胯绋嶅井澶嶆潅涓€浜涳紝骞朵笖鏇磋创杩戝綋浠婄殑纭欢锛?

   :alt: KMS 杈撳嚭娴佹按绾?
   :caption: KMS 杈撳嚭娴佹按绾?

   digraph "Output Pipeline" {
      node [shape=box]

      subgraph {
          "drm_crtc" [bgcolor=grey style=filled]
      }

      subgraph cluster_internal {
          style=dashed
          label="Internal Pipeline"
          {
              node [bgcolor=grey style=filled]
              "drm_encoder A";
              "drm_encoder B";
              "drm_encoder C";
          }

          {
              node [bgcolor=grey style=filled]
              "drm_encoder B" -> "drm_bridge B"
              "drm_encoder C" -> "drm_bridge C1"
              "drm_bridge C1" -> "drm_bridge C2";
          }
      }

      "drm_crtc" -> "drm_encoder A"
      "drm_crtc" -> "drm_encoder B"
      "drm_crtc" -> "drm_encoder C"


      subgraph cluster_output {
          style=dashed
          label="Outputs"

          "drm_encoder A" -> "drm_connector A";
          "drm_bridge B" -> "drm_connector B";
          "drm_bridge C2" -> "drm_connector C";

          "drm_panel"
      }
   }

鍦ㄥ唴閮ㄨ繕鏈変袱涓澶栫殑杈呭姪瀵硅薄鍙戞尌浣滅敤銆傞鍏堬紝涓轰簡鑳藉鍦?encoder 涔嬮棿鍏变韩浠ｇ爜锛堟湁鏃跺湪鍚屼竴 SoC 涓婏紝鏈夋椂鍦ㄧ墖澶栵級锛屽彲浠ュ皢涓€涓垨澶氫釜 drm_bridge锛堢敱 :c:type:`struct drm_bridge <drm_bridge>` 琛ㄧず锛夐摼鎺ュ埌鏌愪釜 encoder銆傝閾炬帴鏄潤鎬佺殑锛屾棤娉曟洿鏀癸紝杩欐剰鍛崇潃浜ゅ弶寮€鍏筹紙cross-bar锛屽鏋滄湁鐨勮瘽锛夊繀椤绘槧灏勫埌 CRTC 涓庝换浣?encoder 涔嬮棿銆傞€氬父鍦ㄥ甫鏈?bridge 鐨勯┍鍔ㄤ腑锛宔ncoder 灞傞潰宸茬粡娌℃湁浠ｇ爜鍓╀笅銆侫tomic 椹卞姩鍙互鐪佸幓鎵€鏈?encoder 鍥炶皟锛屼粠鑰屽疄璐ㄤ笂鍙暀涓嬩竴涓搼璺敱锛坉ummy routing锛夊璞★紝鐢变簬 encoder 宸叉毚闇茬粰鐢ㄦ埛绌洪棿锛岃瀵硅薄闇€瑕佷繚鐣欎互瀹炵幇鍚戝悗鍏煎銆?

绗簩涓璞＄敤浜庨潰鏉匡紙panel锛夛紝鐢?:c:type:`struct drm_panel <drm_panel>` 琛ㄧず锛屽弬瑙?drm_panel_helper銆傞潰鏉挎病鏈夊浐瀹氱殑缁戝畾鐐癸紝浣嗛€氬父閾炬帴鍒板唴宓屼簡 `struct drm_connector <drm_connector>` 鐨勯┍鍔ㄧ鏈夌粨鏋勩€?

娉ㄦ剰锛岀洰鍓?bridge 鐨勯摼寮忚繛鎺ヤ互鍙婁笌 connector 鍜?panel 鐨勪氦浜掍粛澶勪簬鍙樺姩涔嬩腑锛屽皻鏈湡姝ｅ畬鍏ㄧ悊娓呫€?

## KMS 鏍稿績缁撴瀯浣撲笌鍑芥暟


   :internal:

   :export:


## Modeset Base Object Abstraction


   :alt: 妯″紡瀵硅薄涓庡睘鎬?
   :caption: 妯″紡瀵硅薄涓庡睘鎬?

   digraph {
      node [shape=box]

      "drm_property A" -> "drm_mode_object A"
      "drm_property A" -> "drm_mode_object B"
      "drm_property B" -> "drm_mode_object A"
   }

鎵€鏈?KMS 瀵硅薄鐨勫熀缁撴瀯鏄?:c:type:`struct drm_mode_object <drm_mode_object>`銆傚畠鎻愪緵鐨勫熀纭€鏈嶅姟涔嬩竴鏄窡韪睘鎬э紙property锛夛紝杩欏浜?atomic IOCTL 灏や负閲嶈锛堝弬瑙?`Atomic Mode Setting`_锛夈€傝繖閲屾湁鐐瑰嚭浜烘剰鏂欑殑鏄紝灞炴€у苟闈炵洿鎺ュ湪姣忎釜瀵硅薄涓婂疄渚嬪寲锛岃€屾槸鏈韩鏄嫭绔嬬殑妯″紡瀵硅薄锛岀敱 `struct drm_property <drm_property>` 琛ㄧず锛屽畠鍙瀹氫簡灞炴€х殑绫诲瀷鍜屽彇鍊艰寖鍥淬€備换浣曠粰瀹氱殑灞炴€ч兘鍙互閫氳繃 drm_object_attach_property() 澶氭闄勫姞鍒颁笉鍚屽璞′笂銆?

   :internal:

   :export:

## Atomic Mode Setting



   :alt: 妯″紡瀵硅薄涓庡睘鎬?
   :caption: 妯″紡瀵硅薄涓庡睘鎬?

   digraph {
      node [shape=box]

      subgraph cluster_state {
          style=dashed
          label="Free-standing state"

          "drm_atomic_state" -> "duplicated drm_plane_state A"
          "drm_atomic_state" -> "duplicated drm_plane_state B"
          "drm_atomic_state" -> "duplicated drm_crtc_state"
          "drm_atomic_state" -> "duplicated drm_connector_state"
          "drm_atomic_state" -> "duplicated driver private state"
      }

      subgraph cluster_current {
          style=dashed
          label="Current state"

          "drm_device" -> "drm_plane A"
          "drm_device" -> "drm_plane B"
          "drm_device" -> "drm_crtc"
          "drm_device" -> "drm_connector"
          "drm_device" -> "driver private object"

          "drm_plane A" -> "drm_plane_state A"
          "drm_plane B" -> "drm_plane_state B"
          "drm_crtc" -> "drm_crtc_state"
          "drm_connector" -> "drm_connector_state"
          "driver private object" -> "driver private state"
      }

      "drm_atomic_state" -> "drm_device" [label="atomic_commit"]
      "duplicated drm_plane_state A" -> "drm_device"[style=invis]
   }

Atomic 鎻愪緵浜嬪姟鎬х殑妯″紡璁剧疆锛堝寘鎷?plane锛夋洿鏂帮紝浣嗕笌閫氬父鐨?try-commit 鍔?rollback 鐨勪簨鍔℃柟寮忕暐鏈変笉鍚岋細

- 棣栧厛锛屽綋鎻愪氦锛坈ommit锛変細澶辫触鏃讹紝涓嶅厑璁歌繘琛屼换浣曠‖浠舵洿鏀广€傝繖浣挎垜浠兘澶熷疄鐜?DRM_MODE_ATOMIC_TEST_ONLY 妯″紡锛岃鐢ㄦ埛绌洪棿鑳藉璇曟帰鏌愪簺閰嶇疆鏄惁鍙銆?

- 杩欎粛鐒跺厑璁稿彧璁剧疆鍜屽洖婊氳蒋浠剁姸鎬侊紝绠€鍖栦簡瀵圭幇鏈夐┍鍔ㄧ殑杞崲銆備絾鍦ㄨ繖绉嶆儏鍐典笅锛屽璁￠┍鍔ㄧ殑 atomic_check 浠ｇ爜姝ｇ‘鎬у彉寰楅潪甯稿洶闅撅細鍒板鍥炴粴鏁版嵁缁撴瀯涓殑鏀瑰姩寰堥毦鍋氬銆?

- 鏈€鍚庯紝涓轰簡鍚戝悗鍏煎骞舵敮鎸佹墍鏈夌敤渚嬶紝atomic 鏇存柊闇€瑕佹槸澧為噺鐨勶紝骞朵笖瑕佽兘澶熷苟琛屾墽琛屻€傜‖浠跺苟闈炴€昏兘鍋氬埌杩欎竴鐐癸紝浣嗗湪鍙兘鐨勬儏鍐典笅锛屼笉鍚?CRTC 涓婄殑 plane 鏇存柊涓嶅簲鐩镐簰骞叉壈锛屼篃涓嶅簲鍥犱负涓嶅悓 CRTC 涓婄殑杈撳嚭璺敱鍙樺寲鑰屽仠婊炪€?

缁煎悎璧锋潵锛宎tomic 璁捐鏈変袱鐐瑰悗鏋滐細

- 鏁翠綋鐘舵€佽鎷嗗垎涓哄熀浜庢瘡涓璞＄殑 state 缁撴瀯锛歱lane 瀵瑰簲 `struct drm_plane_state <drm_plane_state>`锛孋RTC 瀵瑰簲 :c:type:`struct drm_crtc_state <drm_crtc_state>`锛宑onnector 瀵瑰簲 :c:type:`struct drm_connector_state <drm_connector_state>`銆傝繖浜涙槸鍞竴鍏锋湁鐢ㄦ埛绌洪棿鍙涓斿彲璁剧疆鐘舵€佺殑瀵硅薄銆傚浜庡唴閮ㄧ姸鎬侊紝椹卞姩鍙互閫氳繃鍐呭祵锛坋mbedding锛夋潵瀛愮被鍖栬繖浜涚粨鏋勶紝鎴栬€呬负瀹冧滑鍏ㄥ眬鍏变韩鐨勭‖浠跺姛鑳芥坊鍔犲叏鏂扮殑鐘舵€佺粨鏋勶紝鍙傝 :c:type:`struct drm_private_state<drm_private_state>`銆?

- 涓€涓?atomic 鏇存柊琚粍瑁呭苟楠岃瘉涓?`drm_atomic_state <drm_atomic_state>` 瀹瑰櫒鍐呬竴缁勫畬鍏ㄧ嫭绔嬬殑锛坒ree-standing锛夌粨鏋勩€傞┍鍔ㄧ鏈夌姸鎬佺粨鏋勪篃鍦ㄥ悓涓€缁撴瀯涓窡韪紱鍙傝涓嬩竴绔犮€傚彧鏈夊綋鏌愪釜鐘舵€佽鎻愪氦鏃讹紝鎵嶄細灏嗗叾搴旂敤鍒伴┍鍔ㄥ拰妯″紡璁剧疆瀵硅薄銆傝繖鏍凤紝鍥炴粴涓€娆℃洿鏂板氨褰掔粨涓洪噴鏀惧唴瀛樺苟瑙ｉ櫎瀵瑰抚缂撳啿绛夊璞＄殑寮曠敤銆?

Atomic state 缁撴瀯鐨勫姞閿佸湪鍐呴儴浣跨敤 :c:type:`struct drm_modeset_lock <drm_modeset_lock>`銆備竴鑸師鍒欐槸鍔犻攣涓嶅簲鏆撮湶缁欓┍鍔紝鐩稿弽锛屼换浣曞鍒舵垨绐ヨ鏌愪釜鐘舵€佺殑鍑芥暟锛堜緥濡?drm_atomic_get_crtc_state()锛夐兘搴旇嚜鍔ㄨ幏鍙栨纭殑閿併€傚姞閿佸彧淇濇姢杞欢鏁版嵁缁撴瀯锛屽皢鐘舵€佸彉鏇存彁浜ゅ埌纭欢鐨勯『搴忓垯浣跨敤 `struct drm_crtc_commit <drm_crtc_commit>` 鏉ユ帓搴忋€?

鏈珷浠ュ強 drm_atomic_helper 涓繕鏈夋洿澶氬叧浜庡叿浣撲富棰樼殑璇︾粏浠嬬粛锛岃缁х画闃呰銆?

### 澶勭悊椹卞姩绉佹湁鐘舵€?


   :doc: handling driver private state

### 鍘熷瓙妯″紡璁剧疆鍑芥暟鍙傝€?


   :internal:

   :export:

### 鍘熷瓙妯″紡璁剧疆 IOCTL 涓?UAPI 鍑芥暟


   :doc: overview

   :export:

## CRTC Abstraction


   :doc: overview

### CRTC 鍑芥暟鍙傝€?


   :internal:

   :export:

### 鑹插僵绠＄悊鍑芥暟鍙傝€?


   :export:

   :internal:

## Frame Buffer Abstraction


   :doc: overview

### 甯х紦鍐插嚱鏁板弬鑰?


   :internal:

   :export:

## DRM Format Handling


   :doc: overview

### 鏍煎紡鍑芥暟鍙傝€?


   :internal:

   :export:


## Dumb Buffer Objects


   :doc: overview

## Plane Abstraction


   :doc: overview

### Plane 鍑芥暟鍙傝€?


   :internal:

   :export:

### Plane 鍚堟垚鍑芥暟鍙傝€?


   :export:

### Plane 鎹熷潖璺熻釜鍑芥暟鍙傝€?


   :export:

   :internal:

### Plane 绱ф€ユ樉绀虹壒鎬?


   :doc: overview

### Plane 绱ф€ユ樉绀虹壒鎬у嚱鏁板弬鑰?


   :internal:

   :export:

## Colorop Abstraction


   :doc: overview

### Colorop 鍑芥暟鍙傝€?


   :internal:

   :export:

## 鏄剧ず妯″紡鍑芥暟鍙傝€?


   :internal:

   :export:

## Connector Abstraction


   :doc: overview

### Connector 鍑芥暟鍙傝€?


   :internal:

   :export:

### Writeback Connectors


  :doc: overview

  :internal:

  :export:

## Encoder Abstraction


   :doc: overview

### Encoder 鍑芥暟鍙傝€?


   :internal:

   :export:

## KMS Locking


   :doc: kms locking

   :internal:

   :export:

## KMS Properties


鏈枃妗ｇ殑杩欎竴鑺備富瑕侀潰鍚戠敤鎴风┖闂村紑鍙戣€呫€傛湁鍏抽┍鍔?API锛岃鍙傝鍏朵粬绔犺妭銆?

### Requirements


KMS 椹卞姩鍙兘闇€瑕佹坊鍔犻澶栫殑灞炴€т互鏀寔鏂板姛鑳姐€傞櫎浜嗕笂闈㈡彁鍒扮殑涓€鐐逛箣澶栵紝椹卞姩涓紩鍏ョ殑姣忎釜鏂板睘鎬ц繕闇€瑕佹弧瓒充互涓嬪嚑涓姹傦細

- 瀹冨繀椤绘槸鏍囧噯鍖栫殑锛屽苟搴旇褰曪細

  - 瀹屾暣銆佸噯纭殑鍚嶇О瀛楃涓诧紱
  - 濡傛灉璇ュ睘鎬ф槸鏋氫妇锛屾墍鏈夊悎娉曠殑鍙栧€煎悕绉板瓧绗︿覆锛?
  - 鎺ュ彈鍝簺鍊硷紝浠ュ強杩欎簺鍊兼剰鍛崇潃浠€涔堬紱
  - 璇ュ睘鎬х殑浣滅敤浠ュ強濡備綍浣跨敤瀹冿紱
  - 璇ュ睘鎬у彲鑳藉浣曚笌鍏朵粬宸叉湁灞炴€т氦浜掋€?

- 瀹冨繀椤诲湪鏍稿績浠ｇ爜涓彁渚涗竴涓€氱敤杈呭姪鍑芥暟锛岀敤浜庡皢璇ュ睘鎬ф敞鍐屽埌瀹冩墍闄勫姞鐨勫璞′笂銆?

- 瀹冪殑鍐呭蹇呴』鐢辨牳蹇冧唬鐮佽В鐮侊紝骞舵彁渚涘埌瀵硅薄鍏宠仈鐨勭姸鎬佺粨鏋勪腑銆傝繖鍖呮嫭椹卞姩鍙兘鎯宠棰勮绠楃殑浠讳綍鍐呭锛屼緥濡?plane 鐨?struct drm_clip_rect銆?

- 瀹冪殑鍒濆鐘舵€佸繀椤讳笌璇ュ睘鎬у紩鍏ヤ箣鍓嶇殑琛屼负涓€鑷淬€傝繖鍙兘鏄竴涓笌纭欢瀹為檯琛屼负鐩稿尮閰嶇殑鍥哄畾鍊硷紝涔熷彲鑳芥槸浠庡浐浠跺湪鍚姩鏈熼棿鐣欑粰绯荤粺鐨勭姸鎬佺户鎵胯€屾潵銆?

- 鍦ㄥ悎鐞嗙殑鎯呭喌涓嬶紝蹇呴』鎻愪氦涓€涓?IGT 娴嬭瘯銆?

鐢变簬鍘嗗彶鍘熷洜锛屽瓨鍦ㄩ潪鏍囧噯鐨勩€侀┍鍔ㄧ壒瀹氱殑灞炴€с€傚鏋滄煇涓?KMS 椹卞姩鎯宠娣诲姞瀵瑰叾涓竴涓睘鎬х殑鏀寔锛屽垯搴斿湪鍙兘鐨勬儏鍐典笅閫傜敤鏂板睘鎬х殑鍚勯」瑕佹眰銆傛澶栵紝鏂囨。鍖栫殑琛屼负蹇呴』涓庤宸叉湁灞炴€х殑浜嬪疄璇箟鐩稿尮閰嶏紝浠ョ‘淇濆吋瀹规€с€傞涓坊鍔犺灞炴€х殑椹卞姩鐨勫紑鍙戣€呭簲褰撳崗鍔╁畬鎴愯繖浜涗换鍔★紝骞跺敖鍙兘 ACK 鏂囨。鍖栫殑琛屼负銆?

### 灞炴€х被鍨嬩笌 Blob 灞炴€ф敮鎸?


   :doc: overview

   :internal:

   :export:


### Standard Connector Properties


   :doc: standard connector properties

### HDMI 涓撶敤 Connector 灞炴€?


   :doc: HDMI connector properties

### 妯℃嫙鐢佃涓撶敤 Connector 灞炴€?


   :doc: Analog TV Connector Properties

### Standard CRTC Properties


   :doc: standard CRTC properties

### Standard Plane Properties


   :doc: standard plane properties


### Plane Composition Properties


   :doc: overview


### Damage Tracking Properties


   :doc: damage tracking

### Color Management Properties


   :doc: overview

### Tile Group Property


   :doc: Tile group

### Explicit Fencing Properties


   :doc: explicit fencing properties


### Variable Refresh Properties


   :doc: Variable refresh properties

### Cursor Hotspot Properties


   :doc: hotspot properties

### Existing KMS Properties


涓嬭〃鎻忚堪浜嗗悇涓ā鍧?椹卞姩鏆撮湶鐨?drm 灞炴€с€傜敱浜庤琛ㄩ潪甯哥閲嶏紝璇峰嬁鍦ㄦ澶勬坊鍔犱换浣曟柊灞炴€с€傝€屽簲鍦ㄤ笂闈㈢殑鏌愪釜灏忚妭涓褰曞畠浠€?

   :header-rows: 1
   :file: kms-properties.csv

## Vertical Blanking


   :doc: vblank handling

### 鍨傜洿娑堥殣涓庝腑鏂鐞嗗嚱鏁板弬鑰?


   :internal:

   :export:

## Vertical Blank Work


   :doc: vblank works

### 鍨傜洿娑堥殣宸ヤ綔鍑芥暟鍙傝€?


   :internal:

   :export:
