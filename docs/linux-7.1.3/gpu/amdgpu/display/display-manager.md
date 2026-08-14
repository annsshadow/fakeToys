AMDgpu 鏄剧ず绠＄悊鍣?


    :depth: 3

   :doc: overview

   :internal:

鐢熷懡鍛ㄦ湡


   :doc: DM Lifecycle

   :functions: dm_hw_init dm_hw_fini

涓柇


   :doc: overview

   :internal:

   :functions: register_hpd_handlers dm_crtc_high_irq dm_pflip_high_irq

鍘熷瓙鍖栧疄鐜?


   :doc: atomic

   :functions: amdgpu_dm_atomic_check amdgpu_dm_atomic_commit_tail

棰滆壊绠＄悊灞炴€?


   :doc: overview

   :internal:


DCN 鍚勪唬涔嬮棿鐨?DC 棰滆壊鑳藉姏


DRM/KMS 妗嗘灦瀹氫箟浜嗕笁涓?CRTC 棰滆壊鏍℃灞炴€э細degamma锛堝幓浼介┈锛夈€侀鑹插彉鎹㈢煩闃碉紙CTM锛夊拰 gamma锛堜冀椹級锛屼互鍙婄敤浜?degamma 鍜?gamma LUT 澶у皬鐨勪袱涓睘鎬с€侫MD DC 鍦ㄦ贩鍚堝墠缂栫▼閮ㄥ垎棰滆壊鏍℃鐗规€э紝浣?DRM/KMS 娌℃湁閫愬钩闈㈢殑棰滆壊鏍℃灞炴€с€?

涓€鑸€岃█锛孌RM CRTC 棰滆壊灞炴€ф寜濡備笅鏂瑰紡缂栫▼鍒?DC锛氭贩鍚堝悗鐨?CRTC gamma锛屼互鍙婃贩鍚堝墠鐨?CRTC degamma銆傚敖绠?CTM 鍦ㄦ贩鍚堝悗缂栫▼锛屼絾瀹冭鏄犲皠鍒?DPP 纭欢鍧楋紙娣峰悎鍓嶏級銆傜‖浠朵腑鍙敤鐨勫叾浠栭鑹茶兘鍔涚洰鍓嶅皻鏈敱 DRM 鎺ュ彛鏆撮湶锛屽洜鑰岃鏃佽矾銆?

   :doc: color-management-caps

   :internal:

棰滆壊娴佹按绾垮湪 DCN 纭欢鍚勪唬涔嬮棿鍙戠敓浜嗛噸澶у彉鍖栥€傛贩鍚堝墠鍜屾贩鍚堝悗鍙墽琛岀殑鎿嶄綔鍙栧喅浜庣‖浠惰兘鍔涳紝濡備笅鎵€绀?DCN 2.0 鍜?DCN 3.0 绯诲垪鐨勬灦鏋勭ず鎰忋€?

**DCN 2.0 绯诲垪棰滆壊鑳藉姏鍙婃槧灏?*


**DCN 3.0 绯诲垪棰滆壊鑳藉姏鍙婃槧灏?*


娣峰悎妯″紡灞炴€?


鍍忕礌娣峰悎妯″紡鏄?`drm_plane` 鐨勪竴涓?DRM 骞抽潰鍚堟垚灞炴€э紝鐢ㄤ簬鎻忚堪鍓嶆櫙骞抽潰锛坒g锛夌殑鍍忕礌濡備綍涓庤儗鏅钩闈紙bg锛夊悎鎴愩€傛澶勪粙缁?DRM 娣峰悎妯″紡鐨勪富瑕佹蹇碉紝浠ュ府鍔╃悊瑙ｈ灞炴€у浣曟槧灏勫埌 AMD DC 鎺ュ彛銆傛湁鍏虫 DRM 灞炴€у強 alpha 娣峰悎鏂圭▼鐨勬洿澶氬唴瀹癸紝璇峰弬闃?:ref:`DRM Plane Composition Properties <plane_composition_properties>`銆?

鍩烘湰涓婏紝娣峰悎妯″紡涓哄钩闈㈠悎鎴愯瀹?alpha 娣峰悎鏂圭▼锛岃鏂圭▼閫傜敤浜?alpha 閫氶亾褰卞搷鍍忕礌棰滆壊鍊肩姸鎬侊紙浠庤€屽奖鍝嶆渶缁堝儚绱犻鑹诧級鐨勬ā寮忋€備緥濡傦紝鑰冭檻 alpha 娣峰悎鏂圭▼鐨勪互涓嬪厓绱狅細

- **fg.rgb**锛氬墠鏅儚绱犵殑鍚勪釜 RGB 鍒嗛噺鍊笺€?
- **fg.alpha**锛氬墠鏅儚绱犵殑 alpha 鍒嗛噺鍊笺€?
- **bg.rgb**锛氳儗鏅殑鍚勪釜 RGB 鍒嗛噺鍊笺€?
- **plane_alpha**锛氱敱 **plane "alpha" property** 璁惧畾鐨勫钩闈?alpha 鍊硷紝璇﹁ DRM 骞抽潰鍚堟垚灞炴€?<plane_composition_properties>銆?

```

   out.rgb = alpha * fg.rgb + (1 - alpha) * bg.rgb

```
骞抽潰涓瘡涓儚绱犵殑 alpha 閫氶亾鍊艰蹇界暐锛屼粎骞抽潰 alpha 褰卞搷鏈€缁堢殑鍍忕礌棰滆壊鍊笺€?

DRM 瀹氫箟浜嗕笁绉嶆贩鍚堟ā寮忔潵瑙勫畾骞抽潰鍚堟垚涓殑娣峰悎鍏紡锛?

**None**锛氬拷鐣ュ儚绱?alpha 鐨勬贩鍚堝叕寮忋€?

**Pre-multiplied**锛氬亣瀹氬钩闈腑鐨勫儚绱犻鑹插€煎湪杩涜瀛樺偍鍓嶅凡缁忚繃鑷韩 alpha 閫氶亾棰勪箻鐨勬贩鍚堝叕寮忋€?

**Coverage**锛氬亣瀹氬儚绱犻鑹插€兼湭涓?alpha 閫氶亾鍊奸涔樼殑娣峰悎鍏紡銆?

棰勪箻鏄粯璁ょ殑鍍忕礌娣峰悎妯″紡锛岃繖鎰忓懗鐫€褰撴湭鍒涘缓鎴栧畾涔夋贩鍚堟ā寮忓睘鎬ф椂锛孌RM 璁や负骞抽潰鐨勫儚绱犲叿鏈夐涔樼殑棰滆壊鍊笺€傚湪 IGT GPU 宸ュ叿涓紝kms_plane_alpha_blend 娴嬭瘯鎻愪緵浜嗕竴缁勫瓙娴嬭瘯锛岀敤浜庨獙璇佸钩闈?alpha 鍜屾贩鍚堟ā寮忓睘鎬с€?

鐒跺悗锛孌RM 娣峰悎妯″紡鍙婂叾鍏冪礌鐢?AMDGPU 鏄剧ず绠＄悊鍣紙DM锛夋槧灏勶紝浠ョ紪绋嬪绠￠亾/骞抽潰缁勫悎锛圡PC锛夌殑娣峰悎閰嶇疆锛屽涓嬫墍绀猴細

   :identifiers: mpcc_blnd_cfg

鍥犳锛孧PC 鏍戜笂鍗曚釜 MPCC 瀹炰緥鐨勬贩鍚堥厤缃敱 `mpcc_blnd_cfg` 瀹氫箟锛屽叾涓?
`pre_multiplied_alpha` 鏄敤浜庤瀹?`MPCC_ALPHA_MULTIPLIED_MODE` 鐨?alpha 棰勪箻妯″紡鏍囧織銆傚畠鎺у埗 alpha 鏄惁琚箻锛坱rue/false锛夛紝浠呭湪 DRM 棰勪箻娣峰悎妯″紡涓嬩负 true銆?
`mpcc_alpha_blend_mode` 瀹氫箟浜嗗叧浜庡儚绱?alpha 鍜屽钩闈?alpha 鍊肩殑 alpha 娣峰悎妯″紡銆傚畠涓?
`MPCC_ALPHA_BLND_MODE` 璁惧畾涓夌妯″紡涔嬩竴锛屽涓嬫墍杩般€?

   :identifiers: mpcc_alpha_blend_mode

鐒跺悗 DM 灏?`enum mpcc_alpha_blend_mode` 鐨勫厓绱犳槧灏勫埌 DRM 娣峰悎鍏紡涓殑鍏冪礌锛屽涓嬫墍绀猴細

- **MPC 鍍忕礌 alpha** 瀵瑰簲 **DRM fg.alpha**锛屽嵆鏉ヨ嚜骞抽潰鍍忕礌鐨?alpha 鍒嗛噺鍊笺€?
- **MPC 鍏ㄥ眬 alpha** 鍦ㄥ簲蹇界暐鍍忕礌 alpha 鏃跺搴?**DRM plane_alpha**锛屽洜姝ゅ儚绱犲€兼湭棰勪箻銆?
- **MPC 鍏ㄥ眬澧炵泭** 鍦?*DRM fg.alpha** 涓?**DRM plane_alpha* 閮藉弬涓庢贩鍚堟柟绋嬫椂锛屽亣瀹氫负 **MPC 鍏ㄥ眬 alpha** 鍊笺€?

绠€鑰岃█涔嬶紝閫氳繃閫夋嫨 `MPCC_ALPHA_BLEND_MODE_GLOBAL_ALPHA` 浼氬拷鐣?**fg.alpha**銆傚彟涓€鏂归潰锛岄€氳繃閫夋嫨 `MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA_COMBINED_GLOBAL_GAIN` 鍙娇锛坧lane_alpha * fg.alpha锛夊垎閲忓彲鐢ㄣ€傝€?`MPCC_ALPHA_MULTIPLIED_MODE` 瀹氫箟浜嗗儚绱犻鑹插€兼槸鍚﹁ alpha 棰勪箻銆?

娣峰悎閰嶇疆娴佺▼


alpha 娣峰悎鏂圭▼閫氳繃浠ヤ笅璺緞浠?DRM 閰嶇疆鍒?DC 鎺ュ彛锛?

1. 鏇存柊 `drm_plane_state <drm_plane_state>` 鏃讹紝DM 璋冪敤
   `amdgpu_dm_plane_fill_blending_from_plane_state()`锛屽皢
   `drm_plane_state <drm_plane_state>` 灞炴€ф槧灏勫埌
   `dc_plane_info <dc_plane_info>` 缁撴瀯浣擄紝浜ょ敱
   鎿嶄綔绯荤粺鏃犲叧缁勪欢锛圖C锛夊鐞嗐€?

2. 鍦?DC 鎺ュ彛涓婏紝`struct mpcc_blnd_cfg <mpcc_blnd_cfg>` 缂栫▼
   MPCC 娣峰悎閰嶇疆锛屽苟鑰冭檻鏉ヨ嚜 DPP 鐨?:c:type:`dc_plane_info
   <dc_plane_info>` 杈撳叆銆?
