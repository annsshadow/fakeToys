
##  drm/komeda Arm 鏄剧ず椹卞姩


drm/komeda 椹卞姩鏀寔 Arm 鏄剧ず澶勭悊鍣?D71 鍙婁箣鍚庣殑浜у搧锛屾湰鏂囨。绠€瑕佹杩伴┍鍔?璁捐锛氬畠濡備綍宸ヤ綔锛屼互鍙婁负浣曞姝よ璁°€?
## D71 绫绘樉绀?IP 姒傝堪


浠?D71 寮€濮嬶紝Arm 鏄剧ず IP 寮€濮嬮噰鐢ㄧ伒娲汇€佹ā鍧楀寲鐨勬灦鏋勩€備竴鏉℃樉绀烘祦姘寸嚎鐢卞涓?鐙珛涓斿姛鑳藉寲鐨勬祦姘寸嚎闃舵锛堢О涓虹粍浠讹級缁勬垚锛屾瘡涓粍浠堕兘鏈変竴浜涚壒瀹氳兘鍔涳紝鍙
娴佺粡娴佹按绾跨殑鍍忕礌鏁版嵁鍋氱壒瀹氬鐞嗐€?
鍏稿瀷鐨?D71 缁勪欢锛?
### Layer锛堝浘灞傦級

Layer 鏄涓€涓祦姘寸嚎闃舵锛屼负涓嬩竴闃舵鍑嗗鍍忕礌鏁版嵁銆傚畠浠庡唴瀛樹腑鑾峰彇鍍忕礌锛?濡傛灉鏄?AFBC 鍒欒В鐮侊紝鏃嬭浆婧愬浘鍍忥紝灏?YUV 鍍忕礌瑙ｅ寘鎴栬浆鎹负璁惧鍐呴儴 RGB 鍍忕礌锛?鐒跺悗鍦ㄩ渶瑕佹椂瀵瑰儚绱犵殑 color_space锛堣壊褰╃┖闂达級杩涜璋冩暣銆?
### Scaler锛堢缉鏀惧櫒锛?
椤惧悕鎬濅箟锛宻caler 璐熻矗缂╂斁锛孌71 杩樻敮鎸侀€氳繃 scaler 杩涜鍥惧儚澧炲己銆?scaler 鐨勪娇鐢ㄩ潪甯哥伒娲伙紝鍙互杩炴帴鍒?layer 杈撳嚭浠ヨ繘琛屽浘灞傜缉鏀撅紝鎴栬繛鎺ュ埌
compositor锛堝悎鎴愬櫒锛夊苟缂╂斁鏁翠釜鏄剧ず甯э紝鐒跺悗灏嗚緭鍑烘暟鎹€佸叆 wb_layer锛岀敱鍚庤€?鍐欏叆鍐呭瓨銆?
### Compositor锛坈ompiz锛屽悎鎴愬櫒锛?
Compositor 灏嗗涓浘灞傛垨鍍忕礌鏁版嵁娴佹贩鍚堜负鍗曚竴鏄剧ず甯с€傚叾杈撳嚭甯у彲浠ラ€佸叆鍚庡浘鍍?澶勭悊鍣紙post image processor锛変互鍦ㄦ樉绀哄櫒涓婃樉绀猴紝鎴栧悓鏃堕€佸叆 wb_layer 骞跺啓鍏?鍐呭瓨銆傜敤鎴蜂篃鍙互鍦?compiz 涓?wb_layer 涔嬮棿鎻掑叆涓€涓?scaler锛屽厛瀵规樉绀哄抚杩涜
缂╁皬锛屽啀鍐欏叆鍐呭瓨銆?
### Writeback Layer锛坵b_layer锛屽洖鍐欏浘灞傦級

Writeback layer 鍋氫笌 Layer 鐩稿弽鐨勪簨鎯咃紝瀹冭繛鎺ュ埌 compiz锛屽苟灏嗗悎鎴愮粨鏋滃啓鍏?鍐呭瓨銆?
### Post image processor锛坕mproc锛屽悗鍥惧儚澶勭悊鍣級

Post image processor 璋冩暣甯ф暟鎹紝濡?gamma 鍜岃壊褰╃┖闂达紝浠ョ鍚堟樉绀哄櫒鐨勮姹傘€?
### Timing controller锛坱iming_ctrlr锛屾椂搴忔帶鍒跺櫒锛?
鏄剧ず娴佹按绾跨殑鏈€鍚庝竴涓樁娈碉紝Timing controller 涓嶅鐞嗗儚绱狅紝鍙敤浜庢帶鍒舵樉绀烘椂搴忋€?
### Merger锛堝悎骞跺櫒锛?
D71 鐨?scaler 涓?Layer 鐩告瘮锛屽ぇ澶氬彧鏈変竴鍗婄殑姘村钩鍜岃緭鍏ヨ緭鍑鸿兘鍔涳紝渚嬪濡傛灉
Layer 鏀寔 4K 杈撳叆灏哄锛宻caler 鍦ㄥ悓涓€鏃堕棿鍙兘鏀寔 2K 杈撳叆/杈撳嚭銆備负浜嗗疄鐜?瀹屾暣甯х缉鏀撅紝D71 寮曞叆浜?Layer Split锛屽畠灏嗘暣骞呭浘鍍忓垏鍒嗕负涓ゅ崐锛屽垎鍒€佸叆涓や釜
Layer A 鍜?B锛屽苟鐙珛杩涜缂╂斁銆傜缉鏀惧悗闇€瑕佸皢缁撴灉閫佸叆 merger 灏嗕袱涓儴鍒嗗浘鍍?鍚堝苟鍦ㄤ竴璧凤紝鐒跺悗灏嗗悎骞剁粨鏋滆緭鍑哄埌 compiz銆?
### Splitter锛堝垎鍓插櫒锛?
涓?Layer Split 绫讳技锛屼絾 Splitter 鐢ㄤ簬鍥炲啓锛屽畠灏?compiz 鐨勭粨鏋滃垏鍒嗕负涓ら儴鍒嗭紝
鐒跺悗鍒嗗埆閫佸叆涓や釜 scaler銆?
## D71 娴佹按绾垮彲鑳界殑鐢ㄦ硶


鍙楃泭浜庢ā鍧楀寲鏋舵瀯锛孌71 娴佹按绾垮彲浠ヨ交鏉捐皟鏁翠互閫傞厤涓嶅悓鐢ㄩ€斻€侱71 鏈変袱鏉℃祦姘寸嚎锛?鏀寔涓ょ宸ヤ綔妯″紡锛?
- Dual display mode锛堝弻鏄剧ず妯″紡锛?    涓ゆ潯娴佹按绾跨嫭绔嬨€佸垎鍒湴宸ヤ綔锛岄┍鍔ㄤ袱涓樉绀鸿緭鍑恒€?
- Single display mode锛堝崟鏄剧ず妯″紡锛?    涓ゆ潯娴佹按绾垮崗鍚屽伐浣滐紝浠呴┍鍔ㄤ竴涓樉绀鸿緭鍑恒€?
    鍦ㄦ妯″紡涓嬶紝pipeline_B 涓嶇嫭绔嬪伐浣滐紝鑰屾槸灏嗗叾鍚堟垚缁撴灉杈撳嚭鍒?pipeline_A锛?    鍏跺儚绱犳椂搴忎篃娲剧敓鑷?pipeline_A.timing_ctrlr銆俻ipeline_B 灏卞鍚?    pipeline_A锛坢aster锛屼富锛夌殑涓€涓?鈥渟lave鈥濓紙浠庯級銆?
### 鍗曟祦姘寸嚎鏁版嵁娴?

   :alt: 鍗曟祦姘寸嚎 digraph
   :caption: 鍗曟祦姘寸嚎鏁版嵁娴?
   digraph single_ppl {
      rankdir=LR;

      subgraph {
         "Memory";
         "Monitor";
      }

      subgraph cluster_pipeline {
          style=dashed
          node [shape=box]
          {
              node [bgcolor=grey style=dashed]
              "Scaler-0";
              "Scaler-1";
              "Scaler-0/1"
          }

         node [bgcolor=grey style=filled]
         "Layer-0" -> "Scaler-0"
         "Layer-1" -> "Scaler-0"
         "Layer-2" -> "Scaler-1"
         "Layer-3" -> "Scaler-1"

         "Layer-0" -> "Compiz"
         "Layer-1" -> "Compiz"
         "Layer-2" -> "Compiz"
         "Layer-3" -> "Compiz"
         "Scaler-0" -> "Compiz"
         "Scaler-1" -> "Compiz"

         "Compiz" -> "Scaler-0/1" -> "Wb_layer"
         "Compiz" -> "Improc" -> "Timing Controller"
      }

      "Wb_layer" -> "Memory"
      "Timing Controller" -> "Monitor"
   }

### 鍚敤 Slave 鐨勫弻娴佹按绾?

   :alt: Slave 娴佹按绾?digraph
   :caption: 鍚敤 Slave 娴佹按绾跨殑鏁版嵁娴?
   digraph slave_ppl {
      rankdir=LR;

      subgraph {
         "Memory";
         "Monitor";
      }
      node [shape=box]
      subgraph cluster_pipeline_slave {
          style=dashed
          label="Slave Pipeline_B"
          node [shape=box]
          {
              node [bgcolor=grey style=dashed]
              "Slave.Scaler-0";
              "Slave.Scaler-1";
          }

         node [bgcolor=grey style=filled]
         "Slave.Layer-0" -> "Slave.Scaler-0"
         "Slave.Layer-1" -> "Slave.Scaler-0"
         "Slave.Layer-2" -> "Slave.Scaler-1"
         "Slave.Layer-3" -> "Slave.Scaler-1"

         "Slave.Layer-0" -> "Slave.Compiz"
         "Slave.Layer-1" -> "Slave.Compiz"
         "Slave.Layer-2" -> "Slave.Compiz"
         "Slave.Layer-3" -> "Slave.Compiz"
         "Slave.Scaler-0" -> "Slave.Compiz"
         "Slave.Scaler-1" -> "Slave.Compiz"
      }

      subgraph cluster_pipeline_master {
          style=dashed
          label="Master Pipeline_A"
          node [shape=box]
          {
              node [bgcolor=grey style=dashed]
              "Scaler-0";
              "Scaler-1";
              "Scaler-0/1"
          }

         node [bgcolor=grey style=filled]
         "Layer-0" -> "Scaler-0"
         "Layer-1" -> "Scaler-0"
         "Layer-2" -> "Scaler-1"
         "Layer-3" -> "Scaler-1"

         "Slave.Compiz" -> "Compiz"
         "Layer-0" -> "Compiz"
         "Layer-1" -> "Compiz"
         "Layer-2" -> "Compiz"
         "Layer-3" -> "Compiz"
         "Scaler-0" -> "Compiz"
         "Scaler-1" -> "Compiz"

         "Compiz" -> "Scaler-0/1" -> "Wb_layer"
         "Compiz" -> "Improc" -> "Timing Controller"
      }

      "Wb_layer" -> "Memory"
      "Timing Controller" -> "Monitor"
   }

### 鐢ㄤ簬杈撳叆鍜岃緭鍑虹殑瀛愭祦姘寸嚎


涓€鏉″畬鏁寸殑鏄剧ず娴佹按绾垮彲浠ユ牴鎹緭鍏?杈撳嚭鐢ㄩ€旇交鏉惧垎涓轰笁涓瓙娴佹按绾裤€?
#### Layer(input) 娴佹按绾?

   :alt: Layer 鏁版嵁 digraph
   :caption: Layer锛堣緭鍏ワ級鏁版嵁娴?
   digraph layer_data_flow {
      rankdir=LR;
      node [shape=box]

      {
         node [bgcolor=grey style=dashed]
           "Scaler-n";
      }

      "Layer-n" -> "Scaler-n" -> "Compiz"
   }

   :alt: Layer Split digraph
   :caption: Layer Split 娴佹按绾?
   digraph layer_data_flow {
      rankdir=LR;
      node [shape=box]

      "Layer-0/1" -> "Scaler-0" -> "Merger"
      "Layer-2/3" -> "Scaler-1" -> "Merger"
      "Merger" -> "Compiz"
   }

#### Writeback(output) 娴佹按绾?
   :alt: 鍥炲啓 digraph
   :caption: Writeback锛堣緭鍑猴級鏁版嵁娴?
   digraph writeback_data_flow {
      rankdir=LR;
      node [shape=box]

      {
         node [bgcolor=grey style=dashed]
           "Scaler-n";
      }

      "Compiz" -> "Scaler-n" -> "Wb_layer"
   }

   :alt: 鎷嗗垎鍥炲啓 digraph
   :caption: Writeback锛堣緭鍑猴級鎷嗗垎鏁版嵁娴?
   digraph writeback_data_flow {
      rankdir=LR;
      node [shape=box]

      "Compiz" -> "Splitter"
      "Splitter" -> "Scaler-0" -> "Merger"
      "Splitter" -> "Scaler-1" -> "Merger"
      "Merger" -> "Wb_layer"
   }

#### 鏄剧ず杈撳嚭娴佹按绾?
   :alt: 鏄剧ず digraph
   :caption: 鏄剧ず杈撳嚭鏁版嵁娴?
   digraph single_ppl {
      rankdir=LR;
      node [shape=box]

      "Compiz" -> "Improc" -> "Timing Controller"
   }

鍦ㄤ笅闈㈢殑灏忚妭涓紝鎴戜滑灏嗙湅鍒拌繖涓変釜瀛愭祦姘寸嚎鍒嗗埆鐢?KMS-plane/wb_conn/crtc 澶勭悊銆?
## Komeda 璧勬簮鎶借薄


### struct komeda_pipeline/component


涓轰簡鍏呭垎鍒╃敤骞舵槗浜庤闂?閰嶇疆纭欢锛岄┍鍔ㄤ晶涔熶娇鐢ㄧ被浼肩殑鏋舵瀯锛歅ipeline/Component
鏉ユ弿杩扮‖浠剁壒鎬у拰鑳藉姏锛屼竴涓壒瀹氱殑缁勪欢鍖呭惈涓ら儴鍒嗭細

- 鏁版嵁娴佹帶鍒躲€?- 鐗瑰畾缁勪欢鐨勮兘鍔涗笌鐗规€с€?
鍥犳椹卞姩瀹氫箟浜嗕竴涓€氱敤澶撮儴缁撴瀯浣?komeda_component 鏉ユ弿杩版暟鎹祦鎺у埗锛屾墍鏈?鐗瑰畾缁勪欢閮芥槸姝ゅ熀纭€缁撴瀯鐨勫瓙绫汇€?
   :internal:

## 璧勬簮鍙戠幇涓庡垵濮嬪寲


Pipeline 鍜?component 鐢ㄤ簬鎻忚堪濡備綍澶勭悊鍍忕礌鏁版嵁銆傛垜浠粛鐒堕渶瑕佷竴涓?@struct
komeda_dev 鏉ユ弿杩拌澶囩殑鏁翠綋瑙嗗浘锛屼互鍙婅澶囩殑鎺у埗鑳藉姏銆?
鎴戜滑鏈?&komeda_dev銆?komeda_pipeline銆?komeda_component銆傜幇鍦ㄧ敤娴佹按绾垮～鍏?璁惧銆傜敱浜?komeda 涓嶄粎鐢ㄤ簬 D71锛屼篃闈㈠悜涔嬪悗鐨勪骇鍝侊紝鎴戜滑褰撶劧鏈€濂藉湪涓嶅悓浜у搧闂?灏藉彲鑳藉鍦板叡浜€備负姝わ紝灏?komeda 璁惧鍒嗕负涓ゅ眰锛欳ORE 鍜?CHIP銆?
- CORE锛氱敤浜庨€氱敤鐗规€т笌鑳藉姏鐨勫鐞嗐€?- CHIP锛氱敤浜庡瘎瀛樺櫒缂栫▼鍜岀‖浠剁壒瀹氱壒鎬э紙闄愬埗锛夌殑澶勭悊銆?
CORE 鍙互閫氳繃涓変釜 chip 鍑芥暟缁撴瀯璁块棶 CHIP锛?
- struct komeda_dev_funcs
- struct komeda_pipeline_funcs
- struct komeda_component_funcs

   :internal:

## 鏍煎紡澶勭悊


   :internal:
   :internal:

## 灏?komeda_dev 鎸傛帴鍒?DRM-KMS


Komeda 閫氳繃 pipeline/component 鎶借薄璧勬簮锛屼絾 DRM-KMS 浣跨敤 crtc/plane/connector銆?涓€涓?KMS 瀵硅薄涓嶈兘浠呬唬琛ㄥ崟涓粍浠讹紝鍥犱负鍗曚釜 KMS 瀵硅薄鐨勮姹備笉鑳界畝鍗曞湴鐢卞崟涓?缁勪欢婊¤冻锛岄€氬父閭ｉ渶瑕佸涓粍浠舵潵婊¤冻瑕佹眰銆備緥濡傝缃?mode銆乬amma銆乧tm 閮芥槸閽堝
KMS 鐨?CRTC 瀵硅薄锛屼絾 komeda 闇€瑕?compiz銆乮mproc 鍜?timing_ctrlr 鍗忓悓宸ヤ綔鏉?婊¤冻杩欎簺瑕佹眰銆傝€屼竴涓?KMS-Plane 鍙兘闇€瑕佸涓?komeda 璧勬簮锛歭ayer/scaler/compiz銆?
鍥犳锛屼竴涓?KMS 瀵硅薄浠ｈ〃 komeda 璧勬簮鐨勪竴涓瓙娴佹按绾裤€?
- Plane锛歚Layer(input) pipeline`_
- Wb_connector锛歚Writeback(output) pipeline`_
- Crtc锛歚Display output pipeline`_

鍥犳锛屽浜?komeda锛屾垜浠皢 KMS crtc/plane/connector 瑙嗕负 pipeline 鍜?component
鐨勪娇鐢ㄨ€咃紝骞朵笖鍦ㄤ换鎰忔椂鍒讳竴涓?pipeline/component 鍙兘琚竴涓娇鐢ㄨ€呬娇鐢ㄣ€傝€?pipeline/component 灏嗚瑙嗕负 DRM-KMS 鐨勭鏈夊璞★紱鍏剁姸鎬佷篃鐢?drm_atomic_state
绠＄悊銆?
### 濡備綍灏?plane 鏄犲皠鍒?Layer(input) 娴佹按绾?

Komeda 鏈夊涓?Layer 杈撳叆娴佹按绾匡紝鍙傝锛?- `Single pipeline data flow`_
- `Dual pipeline with Slave enabled`_

鏈€绠€鍗曠殑鏂规硶鏄妸涓€涓?plane 缁戝畾鍒颁竴涓浐瀹氱殑 Layer 娴佹按绾匡紝浣嗚€冭檻鍒?komeda 鐨?鑳藉姏锛?
- Layer Split锛屽弬瑙?`Layer(input) pipeline`_

    Layer_Split 鏄竴涓浉褰撳鏉傜殑鐗规€э紝瀹冨皢涓€骞呭ぇ鍥惧儚鍒囧垎涓轰袱閮ㄥ垎锛岀敱涓ゅ眰鍜?    涓や釜 scaler 鍒嗗埆澶勭悊銆備絾瀹冧細鍦ㄥ垏鍒嗗悗鍦ㄥ浘鍍忎腑闂村紩鍏ヨ竟缂橀棶棰樻垨鏁堟灉銆備负
    閬垮厤姝ょ被闂锛岄渶瑕佸鍒囧垎杩涜澶嶆潅璁＄畻锛屽苟瀵?layer 鍜?scaler 鍋氫竴浜涚壒娈?    閰嶇疆銆傛垜浠渶濂藉皢姝ょ被鐨勭‖浠剁浉鍏冲鏉傛€у鐢ㄦ埛鎬侀殣钘忋€?
- Slave 娴佹按绾匡紝鍙傝 `Dual pipeline with Slave enabled`_

    鐢变簬 compiz 缁勪欢涓嶈緭鍑?alpha 鍊硷紝slave 娴佹按绾垮彧鑳界敤浜庡簳灞傦紙bottom锛夊浘灞傜殑
    鍚堟垚銆俴omeda 椹卞姩甯屾湜鍚戠敤鎴烽殣钘忔闄愬埗銆傚仛娉曟槸鏍规嵁 plane_state->zpos 閫夋嫨
    涓€涓悎閫傜殑 Layer銆?
鍥犳瀵逛簬 komeda锛孠MS-plane 涓嶄唬琛ㄤ竴涓浐瀹氱殑 komeda layer 娴佹按绾匡紝鑰屾槸浠ｈ〃
澶氫釜鍏锋湁鐩稿悓鑳藉姏鐨?Layer銆侹omeda 浼氶€夋嫨涓€涓垨澶氫釜 Layer 鏉ユ弧瓒充竴涓?KMS-plane
鐨勮姹傘€?
### 灏?component/pipeline 璁句负 drm_private_obj


灏?`drm_private_obj` 娣诲姞鍒?`komeda_component`銆乣komeda_pipeline`


    struct komeda_component {
        struct drm_private_obj obj;
        ...
    }

    struct komeda_pipeline {
        struct drm_private_obj obj;
        ...
    }

### 閫氳繃 drm_atomic_state 璺熻釜 component_state/pipeline_state


灏?`drm_private_state` 鍜?user 娣诲姞鍒?`komeda_component_state`銆?`komeda_pipeline_state`


    struct komeda_component_state {
        struct drm_private_state obj;
        void *binding_user;
        ...
    }

    struct komeda_pipeline_state {
        struct drm_private_state obj;
        struct drm_crtc *crtc;
        ...
    }

### komeda 缁勪欢鏍￠獙


Komeda 鏈夊绉嶇被鍨嬬殑缁勪欢锛屼絾鏍￠獙杩囩▼绫讳技锛岄€氬父鍖呮嫭浠ヤ笅姝ラ锛?

    int komeda_xxxx_validate(struct komeda_component_xxx xxx_comp,
                struct komeda_component_output *input_dflow,
                struct drm_plane/crtc/connector *user,
                struct drm_plane/crtc/connector_state, *user_state)
    {
         setup 1: 妫€鏌ユ槸鍚﹂渶瑕佽缁勪欢锛屼緥濡?scaler 鍙栧喅浜?user_state 鏄彲閫夌殑锛?                  濡傛灉涓嶉渶瑕侊紝鐩存帴杩斿洖锛岃皟鐢ㄨ€呬細灏嗘暟鎹祦閫佸叆涓嬩竴闃舵銆?         Setup 2: 鐢ㄧ粍浠剁壒鎬у拰鑳藉姏涓?user_state 杩涜姣斿锛岀湅鏄惁鑳芥弧瓒宠姹傦紱
                  濡傛灉涓嶆弧瓒筹紝杩斿洖澶辫触銆?         Setup 3: 浠?drm_atomic_state 鑾峰彇 component_state锛屽苟灏濊瘯灏?user 璁剧疆
                  鍒扮粍浠讹紱濡傛灉缁勪欢宸茬粡琚垎閰嶇粰鍙︿竴涓?user锛岃繑鍥炲け璐ャ€?         Setup 3: 閰嶇疆 component_state锛屼緥濡傝缃叾杈撳叆缁勪欢锛?                  灏?user_state 杞崲涓虹粍浠剁壒瀹氱殑鐘舵€併€?         Setup 4: 璋冩暣 input_dflow 骞朵负涓嬩竴闃舵鍋氬噯澶囥€?    }

### komeda_kms 鎶借薄


   :internal:

### komde_kms 鍑芥暟


   :internal:
   :internal:

## 灏?komeda 鏋勫缓涓?Linux 妯″潡椹卞姩


鐜板湪鎴戜滑鏈変袱涓眰绾ц澶囷細

- komeda_dev锛氭弿杩扮湡瀹炵殑鏄剧ず纭欢銆?- komeda_kms_dev锛氬皢 komeda_dev 鎸傛帴鎴栬繛鎺ュ埌 DRM-KMS銆?
鎵€鏈?komeda 鎿嶄綔閮界敱 komeda_dev 鎴?komeda_kms_dev 鎻愪緵鎴栨墽琛岋紝妯″潡椹卞姩鍙槸
涓€涓畝鍗曠殑灏佽锛岀敤浜庡皢 Linux 鍛戒护锛坧robe/remove/pm锛変紶鍏?komeda_dev 鎴?komeda_kms_dev銆?