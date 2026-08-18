######## 鍐呭瓨鍒板唴瀛樻湁鐘舵€佽棰戣В鐮佸櫒鎺ュ彛


鏈夌姸鎬侊紙stateful锛夎棰戣В鐮佸櫒鎺ユ敹瀹屾暣鐨勬暟鎹祦鏁版嵁鍧楋紙渚嬪 Annex-B
H.264/HEVC 娴併€佸師濮?VP8/9 娴侊級锛屽苟灏嗗叾瑙ｇ爜涓烘寜鏄剧ず椤哄簭鎺掑垪鐨勫師濮嬭棰戝抚銆傝В鐮佸櫒
鍦ㄥ鐞嗚繖浜涚紦鍐插尯鏃讹紝涓嶅簲闇€瑕佹潵鑷鎴风鐨勪换浣曢澶栦俊鎭€?

寮虹儓涓嶅缓璁湪椹卞姩涓璇ユ暟鎹祦杩涜杞欢瑙ｆ瀽銆佸鐞嗙瓑鎿嶄綔浠ユ敮鎸佹湰鎺ュ彛銆傚鏋滅‘瀹為渶瑕?
姝ょ被鎿嶄綔锛屽己鐑堝缓璁娇鐢ㄦ棤鐘舵€侊紙Stateless锛夎棰戣В鐮佸櫒鎺ュ彛锛堝紑鍙戜腑锛夈€?

## 鏈枃妗ｄ娇鐢ㄧ殑绾﹀畾涓庤鍙?

1. 鑻ユ湰鏂囨。鏈彟鏈夎鏄庯紝鍒欓€氱敤鐨?V4L2 API 瑙勫垯閫傜敤銆?

2. 璇嶈 "must"銆?may"銆?should" 绛夌殑鍚箟浠?`RFC
   2119 <https://tools.ietf.org/html/rfc2119>`_ 涓哄噯銆?

3. 鎵€鏈夋湭鏍囨敞 "optional" 鐨勬楠ら兘鏄繀闇€鐨勩€?

4. 闄ら潪鍙︽湁璇存槑锛宍VIDIOC_G_EXT_CTRLS` 鍜?`VIDIOC_S_EXT_CTRLS` 鍙笌
   `VIDIOC_G_CTRL` 鍜?`VIDIOC_S_CTRL` 浜掓崲浣跨敤銆?

5. 鍗曞钩闈紙single-planar锛堿PI锛堣 planar-apis锛夊強閫傜敤鐨勭粨鏋勪綋锛屼笌澶氬钩闈?
   锛坢ulti-planar锛堿PI 鍙湪婊¤冻瑙ｇ爜鍣ㄨ兘鍔涘苟閬靛惊閫氱敤 V4L2 鎸囧崡鐨勫墠鎻愪笅浜掓崲浣跨敤锛?
   闄ら潪鍙︽湁璇存槑銆?

6. i = [a..b]锛氫粠 a 鍒?b锛堝惈绔偣锛夌殑鏁存暟搴忓垪锛屽嵆 i = [0..2] 琛ㄧず i = 0, 1, 2銆?

7. 缁欏畾涓€涓?`OUTPUT` 缂撳啿鍖?A锛屽垯 A' 琛ㄧず `CAPTURE` 闃熷垪涓婂寘鍚敱澶勭悊缂撳啿鍖?A
   鎵€寰楁暟鎹殑缂撳啿鍖恒€?

## 鏈琛?

CAPTURE
   鐩爣缂撳啿鍖洪槦鍒楋紱瀵逛簬瑙ｇ爜鍣紝鏄寘鍚凡瑙ｇ爜甯х殑缂撳啿鍖洪槦鍒楋紱瀵逛簬缂栫爜鍣紝鏄寘鍚?
   宸茬紪鐮佹暟鎹祦鐨勭紦鍐插尯闃熷垪锛涘搴?`V4L2_BUF_TYPE_VIDEO_CAPTURE` 鎴?
   `V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`锛涙暟鎹敱纭欢鎹曡幏鍒?`CAPTURE` 缂撳啿鍖轰腑銆?

client
   涓庡疄鐜颁簡鏈帴鍙ｇ殑瑙ｇ爜鍣ㄦ垨缂栫爜鍣ㄩ€氫俊鐨勫簲鐢ㄧ▼搴忋€?

coded format
   宸茬紪鐮?鍘嬬缉鐨勮棰戞暟鎹祦鏍煎紡锛堜緥濡?H.264銆乂P8 绛夛級锛涘彟瑙侊細raw format銆?

coded height
   缁欏畾缂栫爜鍒嗚鲸鐜囦笅鐨勯珮搴︺€?

coded resolution
   浠ュ儚绱犺鐨勬祦鍒嗚鲸鐜囷紝瀵归綈鍒扮紪瑙ｇ爜鍣ㄥ拰纭欢鐨勮姹傦紱閫氬父涓哄彲瑙佸垎杈ㄧ巼鍚戜笂鍙栨暣鍒?
   瀹屾暣鐨勫畯鍧楋紱鍙﹁锛歷isible resolution銆?

coded width
   缁欏畾缂栫爜鍒嗚鲸鐜囦笅鐨勫搴︺€?

coding tree unit
   HEVC 缂栬В鐮佸櫒鐨勫鐞嗗崟鍏冿紙瀵瑰簲浜?H.264銆乂P8銆乂P9 涓殑瀹忓潡鍗曞厓锛夛紝鍙娇鐢ㄦ渶澶?
   64脳64 鍍忕礌鐨勫潡缁撴瀯銆傛搮闀垮皢鍥惧儚缁嗗垎涓哄彲鍙樺ぇ灏忕殑缁撴瀯銆?

decode order
   甯ц瑙ｇ爜鐨勯『搴忥紱濡傛灉缂栫爜鏍煎紡鍖呭惈甯ч噸鎺掑簭鐗规€э紝鍒欏彲鑳戒笌鏄剧ず椤哄簭涓嶅悓锛涘浜庤В鐮佸櫒锛?
   `OUTPUT` 缂撳啿鍖哄繀椤荤敱瀹㈡埛绔寜瑙ｇ爜椤哄簭鍏ラ槦锛涘浜庣紪鐮佸櫒 `CAPTURE` 缂撳啿鍖哄繀椤?
   鐢辩紪鐮佸櫒鎸夎В鐮侀『搴忚繑鍥炪€?

destination
   瑙ｇ爜杩囩▼浜х敓鐨勬暟鎹紱瑙?`CAPTURE`銆?

display order
   甯у繀椤昏鏄剧ず鐨勯『搴忥紱瀵逛簬缂栫爜鍣紝`OUTPUT` 缂撳啿鍖哄繀椤荤敱瀹㈡埛绔寜鏄剧ず椤哄簭鍏ラ槦锛?
   瀵逛簬瑙ｇ爜鍣紝`CAPTURE` 缂撳啿鍖哄繀椤荤敱瑙ｇ爜鍣ㄦ寜鏄剧ず椤哄簭杩斿洖銆?

DPB
   Decoded Picture Buffer锛堝凡瑙ｇ爜鍥惧儚缂撳啿鍖猴級锛汬.264/HEVC 涓殑涓€涓湳璇紝鎸囩敤浜庡瓨鍌?
   宸茶В鐮佸師濮嬪抚銆佷緵鍚庣画瑙ｇ爜姝ラ鍙傝€冪殑缂撳啿鍖恒€?

EOS
   end of stream锛堟祦缁撴潫锛夈€?

IDR
   Instantaneous Decoder Refresh锛堝嵆鏃惰В鐮佸埛鏂帮級锛汬.264/HEVC 缂栫爜娴佷腑鐨勪竴绉嶅叧閿抚
   绫诲瀷锛屽畠浼氭竻闄よ緝鏃╁弬鑰冨抚锛圖PB锛夌殑鍒楄〃銆?

keyframe
   涓嶅紩鐢ㄨ緝鏃╁凡瑙ｇ爜甯х殑缂栫爜甯э紝鍗冲彲浠ョ嫭绔嬪畬鏁村湴瑙ｇ爜銆?

macroblock
   鍩轰簬绾挎€у潡鍙樻崲鐨勫浘鍍忓拰瑙嗛鍘嬬缉鏍煎紡涓殑澶勭悊鍗曞厓锛堜緥濡?H.264銆乂P8銆乂P9锛夛紱涓庡叿浣?
   缂栬В鐮佸櫒鐩稿叧锛屼絾澶у鏁版祦琛岀紪瑙ｇ爜鍣ㄧ殑灏哄涓?16x16 閲囨牱锛堝儚绱狅級銆侶EVC 缂栬В鐮佸櫒
   浣跨敤涓€绉嶆洿鐏垫椿鐨勫鐞嗗崟鍏冿紝绉颁负 coding tree unit锛圕TU锛夈€?

OUTPUT
   婧愮紦鍐插尯闃熷垪锛涘浜庤В鐮佸櫒锛屾槸鍖呭惈宸茬紪鐮佹暟鎹祦鐨勭紦鍐插尯闃熷垪锛涘浜庣紪鐮佸櫒锛屾槸鍖呭惈
   鍘熷甯х殑缂撳啿鍖洪槦鍒楋紱瀵瑰簲 `V4L2_BUF_TYPE_VIDEO_OUTPUT` 鎴?
   `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`锛涚‖浠朵粠 `OUTPUT` 缂撳啿鍖鸿幏鍙栨暟鎹€?

PPS
   Picture Parameter Set锛堝浘鍍忓弬鏁伴泦锛夛紱H.264/HEVC 鏁版嵁娴佷腑鐨勪竴绉嶅厓鏁版嵁瀹炰綋銆?

raw format
   鍖呭惈鍘熷鍍忕礌鏁版嵁鐨勬湭鍘嬬缉鏍煎紡锛堜緥濡?YUV銆丷GB 鏍煎紡锛夈€?

resume point
   鏁版嵁娴佷腑鍙互寮€濮?缁х画瑙ｇ爜銆佷笖涓嶅瓨鍦ㄤ换浣曞厛鍓嶇姸鎬?鏁版嵁鐨勭偣锛屼緥濡傦細鍏抽敭甯?
   锛圴P8/VP9锛夋垨 SPS/PPS/IDR 搴忓垪锛圚.264/HEVC锛夛紱寮€濮嬭В鐮佷竴鏉℃柊娴侊紝鎴栧湪 seek
   涔嬪悗鎭㈠瑙ｇ爜锛岄兘闇€瑕佷竴涓仮澶嶇偣锛坮esume point锛夈€?

source
   棣堥€佺粰瑙ｇ爜鍣ㄦ垨缂栫爜鍣ㄧ殑鏁版嵁锛涜 `OUTPUT`銆?

source height
   缁欏畾婧愬垎杈ㄧ巼涓嬬殑鍍忕礌楂樺害锛涗粎涓庣紪鐮佸櫒鐩稿叧銆?

source resolution
   棣堥€佺粰缂栫爜鍣ㄧ殑婧愬抚鐨勫儚绱犲垎杈ㄧ巼锛屽苟鍙楅檺浜庤繘涓€姝ヨ鍓埌鍙鍒嗚鲸鐜囩殑杈圭晫锛涗粎涓?
   缂栫爜鍣ㄧ浉鍏炽€?

source width
   缁欏畾婧愬垎杈ㄧ巼涓嬬殑鍍忕礌瀹藉害锛涗粎涓庣紪鐮佸櫒鐩稿叧銆?

SPS
   Sequence Parameter Set锛堝簭鍒楀弬鏁伴泦锛夛紱H.264/HEVC 鏁版嵁娴佷腑鐨勪竴绉嶅厓鏁版嵁瀹炰綋銆?

stream metadata
   鍖呭惈鍦ㄥ凡缂栫爜鏁版嵁娴佷腑鐨勯檮鍔狅紙闈炶瑙夛級淇℃伅锛涗緥濡傦細缂栫爜鍒嗚鲸鐜囥€佸彲瑙佸垎杈ㄧ巼銆?
   缂栬В鐮佸櫒妗ｆ锛坧rofile锛夈€?

visible height
   缁欏畾鍙鍒嗚鲸鐜囦笅鐨勯珮搴︼紱鍗虫樉绀洪珮搴︺€?

visible resolution
   鍙鍥惧儚鐨勬祦鍒嗚鲸鐜囷紙鍍忕礌锛夛紝鐢ㄤ簬鏄剧ず鐩殑锛涘繀椤诲皬浜庢垨绛変簬缂栫爜鍒嗚鲸鐜囷紱鍗?
   鏄剧ず鍒嗚鲸鐜囥€?

visible width
   缁欏畾鍙鍒嗚鲸鐜囦笅鐨勫搴︼紱鍗虫樉绀哄搴︺€?

## 鐘舵€佹満

   :alt: DOT digraph of decoder state machine
   :caption: Decoder State Machine

   digraph decoder_state_machine {
       node [shape = doublecircle, label="Decoding"] Decoding;

       node [shape = circle, label="Initialization"] Initialization;
       node [shape = circle, label="Capture\nsetup"] CaptureSetup;
       node [shape = circle, label="Dynamic\nResolution\nChange"] ResChange;
       node [shape = circle, label="Stopped"] Stopped;
       node [shape = circle, label="Drain"] Drain;
       node [shape = circle, label="Seek"] Seek;
       node [shape = circle, label="End of Stream"] EoS;

       node [shape = point]; qi
       qi -> Initialization [ label = "open()" ];

       Initialization -> CaptureSetup [ label = "CAPTURE\nformat\nestablished" ];

       CaptureSetup -> Stopped [ label = "CAPTURE\nbuffers\nready" ];

       Decoding -> ResChange [ label = "Stream\nresolution\nchange" ];
       Decoding -> Drain [ label = "V4L2_DEC_CMD_STOP" ];
       Decoding -> EoS [ label = "EoS mark\nin the stream" ];
       Decoding -> Seek [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];
       Decoding -> Stopped [ label = "VIDIOC_STREAMOFF(CAPTURE)" ];
       Decoding -> Decoding;

       ResChange -> CaptureSetup [ label = "CAPTURE\nformat\nestablished" ];
       ResChange -> Seek [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];

       EoS -> Drain [ label = "Implicit\ndrain" ];

       Drain -> Stopped [ label = "All CAPTURE\nbuffers dequeued\nor\nVIDIOC_STREAMOFF(CAPTURE)" ];
       Drain -> Seek [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];

       Seek -> Decoding [ label = "VIDIOC_STREAMON(OUTPUT)" ];
       Seek -> Initialization [ label = "VIDIOC_REQBUFS(OUTPUT, 0)" ];

       Stopped -> Decoding [ label = "V4L2_DEC_CMD_START\nor\nVIDIOC_STREAMON(CAPTURE)" ];
       Stopped -> Seek [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];
   }

## 鏌ヨ鑳藉姏

1. 瑕佹灇涓捐В鐮佸櫒鏀寔鐨勭紪鐮佹牸寮忛泦鍚堬紝瀹㈡埛绔彲浠ュ湪 `OUTPUT` 涓婅皟鐢?
   `VIDIOC_ENUM_FMT`銆?

   - 鏃犺 `CAPTURE` 涓婅缃殑鏄粈涔堟牸寮忥紝閮戒細杩斿洖鍙楁敮鎸佹牸寮忕殑鍏ㄩ儴闆嗗悎銆?
   - 妫€鏌?`v4l2_fmtdesc` 鐨?flags 瀛楁锛屼互浜嗚В瑙ｇ爜鍣ㄧ浉瀵逛簬姣忕缂栫爜鏍煎紡鐨?
     鑳藉姏銆傚挨鍏舵槸瑙ｇ爜鍣ㄦ槸鍚﹀叿鏈夊畬澶囩殑鏁版嵁娴佽В鏋愬櫒锛屼互鍙婃槸鍚︽敮鎸佸姩鎬佸垎杈ㄧ巼
     鍙樺寲銆?

2. 瑕佹灇涓惧彈鏀寔鐨勫師濮嬶紙raw锛夋牸寮忛泦鍚堬紝瀹㈡埛绔彲浠ュ湪 `CAPTURE` 涓婅皟鐢?
   `VIDIOC_ENUM_FMT`銆?

   - 鍙細杩斿洖褰撳墠鍦?`OUTPUT` 涓婂浜庢椿鍔ㄧ姸鎬佺殑鏍煎紡鎵€鏀寔鐨勬牸寮忋€?

   - 涓轰簡鏋氫妇鏌愮粰瀹氱紪鐮佹牸寮忔墍鏀寔鐨勫師濮嬫牸寮忥紝瀹㈡埛绔繀椤诲厛鍦?`OUTPUT` 涓?
     璁剧疆璇ョ紪鐮佹牸寮忥紝鐒跺悗鍐嶅湪 `CAPTURE` 涓婃灇涓炬牸寮忋€?

3. 瀹㈡埛绔彲浠ヤ娇鐢?`VIDIOC_ENUM_FRAMESIZES` 鏉ユ娴嬬粰瀹氭牸寮忔敮鎸佺殑
   鍒嗚鲸鐜囷紝鏂规硶鏄妸鏈熸湜鐨勫儚绱犳牸寮忎紶鍏?`v4l2_frmsizeenum` 鐨?`pixel_format`
   瀛楁銆?

   - `VIDIOC_ENUM_FRAMESIZES` 閽堝缂栫爜鍍忕礌鏍煎紡杩斿洖鐨勫€硷紝灏嗗寘鍚В鐮佸櫒
     閽堝缁欏畾缂栫爜鍍忕礌鏍煎紡鏀寔鐨勬墍鏈夊彲鑳界紪鐮佸垎杈ㄧ巼銆?

   - `VIDIOC_ENUM_FRAMESIZES` 閽堝鍘熷鍍忕礌鏍煎紡杩斿洖鐨勫€硷紝灏嗗寘鍚В鐮佸櫒
     閽堝缁欏畾鍘熷鍍忕礌鏍煎紡浠ュ強褰撳墠鍦?`OUTPUT` 涓婅缃殑缂栫爜鏍煎紡鏀寔鐨勬墍鏈?
     鍙兘甯х紦鍐插尯鍒嗚鲸鐜囥€?

4. 瀵逛簬褰撳墠鍦?`OUTPUT` 涓婅缃殑缂栫爜鏍煎紡锛屽鏋滈€傜敤锛屽叾鏀寔鐨勬。娆★紙profile锛?
   鍜岀骇鍒紙level锛夊彲浠ラ€氳繃鍚勮嚜鐨勬帶浠讹紝缁忕敱 `VIDIOC_QUERYCTRL` 鏌ヨ銆?

## 鍒濆鍖?

1. 閫氳繃 `VIDIOC_S_FMT` 鍦?`OUTPUT` 涓婅缃紪鐮佹牸寮忋€?

   - **蹇呭～瀛楁锛?*

     `type`
         `OUTPUT` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

     `pixelformat`
         涓€绉嶇紪鐮佸儚绱犳牸寮忋€?

     `width`銆乣height`
         鏁版嵁娴佺殑缂栫爜鍒嗚鲸鐜囷紱浠呭綋鏃犳硶浠庢暟鎹祦涓拡瀵圭粰瀹氱紪鐮佹牸寮忚В鏋愬嚭璇ュ€兼椂
         鎵嶉渶瑕佽缃紱鍚﹀垯瑙ｇ爜鍣ㄤ細灏嗚鍒嗚鲸鐜囩敤浣滃崰浣嶅垎杈ㄧ巼锛屼竴鏃﹁兘浠庢暟鎹祦涓В鏋愬嚭
         瀹為檯缂栫爜鍒嗚鲸鐜囷紝璇ュ€煎氨鍙兘浼氭敼鍙樸€?

     `sizeimage`
         `OUTPUT` 缂撳啿鍖虹殑鏈熸湜澶у皬锛涜В鐮佸櫒鍙鍏惰繘琛岃皟鏁翠互鍖归厤纭欢瑕佹眰銆?

     other fields
         閬靛惊鏍囧噯璇箟銆?

   - **杩斿洖瀛楁锛?*

     `sizeimage`
         璋冩暣鍚庣殑 `OUTPUT` 缂撳啿鍖哄ぇ灏忋€?

   - `CAPTURE` 鏍煎紡浼氭牴鎹?`VIDIOC_S_FMT` 杩斿洖鐨勫搴﹀拰楂樺害锛岀珛鍗虫洿鏂颁负
     鍚堥€傜殑甯х紦鍐插尯鍒嗚鲸鐜囥€備絾鏄紝瀵逛簬鍖呭惈娴佸垎杈ㄧ巼淇℃伅鐨勭紪鐮佹牸寮忥紝鍦ㄨВ鐮佸櫒瀹屾垚
     浠庢暟鎹祦涓В鏋愯淇℃伅鍚庯紝鏃犺鍏舵槸鍚︿笌瀹㈡埛绔缃殑鍊煎尮閰嶏紝瀹冮兘浼氱敤鏂板€兼洿鏂?
     `CAPTURE` 鏍煎紡骞跺彂鍑烘簮鍙樺寲锛坰ource change锛変簨浠躲€?

```

      Changing the ``OUTPUT`` format may change the currently set ``CAPTURE``
      format. How the new ``CAPTURE`` format is determined is up to the decoder
      and the client must ensure it matches its needs afterwards.

```

2. 閫氳繃 `VIDIOC_REQBUFS` 鍦?`OUTPUT` 涓婂垎閰嶆簮锛坆ytestream锛夌紦鍐插尯銆?

    - **蹇呭～瀛楁锛?*

      `count`
          璇锋眰鐨勭紦鍐插尯鍒嗛厤鏁伴噺锛涘繀椤诲ぇ浜庨浂銆?

      `type`
          `OUTPUT` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

      `memory`
          閬靛惊鏍囧噯璇箟銆?

    - **杩斿洖瀛楁锛?*

      `count`
          瀹為檯鍒嗛厤鐨勭紦鍐插尯鏁伴噺銆?

```

       The actual number of allocated buffers may differ from the ``count``
       given. The client must check the updated value of ``count`` after the
       call returns.

    Alternatively, :c:func:`VIDIOC_CREATE_BUFS` on the ``OUTPUT`` queue can be
    used to have more control over buffer allocation.

    * **Required fields:**

      ``count``
          requested number of buffers to allocate; greater than zero.

      ``type``
          a ``V4L2_BUF_TYPE_*`` enum appropriate for ``OUTPUT``.

      ``memory``
          follows standard semantics.

      ``format``
          follows standard semantics.

    * **Returned fields:**

      ``count``
          adjusted to the number of allocated buffers.

    .. warning::

       The actual number of allocated buffers may differ from the ``count``
       given. The client must check the updated value of ``count`` after the
       call returns.

```

3. 閫氳繃 `VIDIOC_STREAMON` 鍦?`OUTPUT` 闃熷垪涓婂惎鍔ㄦ暟鎹祦锛坰treaming锛夈€?

4. **姝ゆ楠や粎閫傜敤浜庡湪娴佷腑鍖呭惈鍒嗚鲸鐜囦俊鎭殑缂栫爜鏍煎紡銆?* 缁х画閫氳繃 `VIDIOC_QBUF`
   鍜?`VIDIOC_DQBUF` 鍦?`OUTPUT` 闃熷垪涓婂叆闃?鍑洪槦鏁版嵁娴佺紦鍐插尯銆傜紦鍐插尯灏嗚
   鎸夐『搴忓鐞嗗苟杩旇繕缁欏鎴风锛岀洿鍒版壘鍒伴厤缃?`CAPTURE` 闃熷垪鎵€闇€鐨勫厓鏁版嵁涓烘銆傝繖鐢?
   瑙ｇ爜鍣ㄥ彂閫?`changes` 璁句负 `V4L2_EVENT_SRC_CH_RESOLUTION` 鐨?
   `V4L2_EVENT_SOURCE_CHANGE` 浜嬩欢鏉ユ寚绀恒€?

    - 濡傛灉绗竴涓紦鍐插尯鍖呭惈鐨勬暟鎹笉瓒充互瑙﹀彂璇ヤ簨浠讹紝杩欎笉绠楅敊璇€傚彧瑕佽繕闇€瑕佹洿澶?
      鏁版嵁锛屽氨浼氱户缁鐞嗙紦鍐插尯銆?

    - 濡傛灉瑙﹀彂璇ヤ簨浠剁殑缂撳啿鍖轰腑鐨勬暟鎹槸瑙ｇ爜绗竴甯ф墍蹇呴渶鐨勶紝閭ｄ箞鍦ㄥ垵濮嬪寲搴忓垪
      瀹屾垚涓旇甯ц瑙ｇ爜涔嬪墠锛岃缂撳啿鍖轰笉浼氳杩旇繕缁欏鎴风銆?

    - 濡傛灉瀹㈡埛绔病鏈夎嚜琛岃缃暟鎹祦鐨勭紪鐮佸垎杈ㄧ巼锛岄偅涔堝湪 `CAPTURE` 闃熷垪涓婅皟鐢?
      `VIDIOC_G_FMT`銆乣VIDIOC_S_FMT`銆乣VIDIOC_TRY_FMT` 鎴?
      `VIDIOC_REQBUFS`锛屽湪鍙戝嚭 `changes` 璁句负
      `V4L2_EVENT_SRC_CH_RESOLUTION` 鐨?`V4L2_EVENT_SOURCE_CHANGE` 浜嬩欢
      涔嬪墠锛岄兘涓嶄細杩斿洖鏁版嵁娴佺殑鐪熷疄鍊笺€?

```

       Any client query issued after the decoder queues the event will return
       values applying to the just parsed stream, including queue formats,
       selection rectangles and controls.

    .. note::

       A client capable of acquiring stream parameters from the bytestream on
       its own may attempt to set the width and height of the ``OUTPUT`` format
       to non-zero values matching the coded size of the stream, skip this step
       and continue with the `Capture Setup` sequence. However, it must not
       rely on any driver queries regarding stream parameters, such as
       selection rectangles and controls, since the decoder has not parsed them
       from the stream yet. If the values configured by the client do not match
       those parsed by the decoder, a `Dynamic Resolution Change` will be
       triggered to reconfigure them.

    .. note::

       No decoded frames are produced during this phase.

```

5. 缁х画 `Capture Setup` 搴忓垪銆?

## 鎹曡幏璁剧疆锛圕apture Setup锛?

1. 鍦?`CAPTURE` 闃熷垪涓婅皟鐢?`VIDIOC_G_FMT`锛屼互鑾峰彇浠庢暟鎹祦涓В鏋?瑙ｇ爜鍑虹殑
    鐩爣缂撳啿鍖虹殑鏍煎紡銆?

    - **蹇呭～瀛楁锛?*

      `type`
          `CAPTURE` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

    - **杩斿洖瀛楁锛?*

      `width`銆乣height`
          宸茶В鐮佸抚鐨勫抚缂撳啿鍖哄垎杈ㄧ巼銆?

      `pixelformat`
          宸茶В鐮佸抚鐨勫儚绱犳牸寮忋€?

      `num_planes`锛堜粎閫傜敤浜?_MPLANE `type`锛?
          pixelformat 鐨勫钩闈㈡暟閲忋€?

      `sizeimage`銆乣bytesperline`
          閬靛惊鏍囧噯璇箟锛涗笌甯х紦鍐插尯鏍煎紡鍖归厤銆?

```

       The value of ``pixelformat`` may be any pixel format supported by the
       decoder for the current stream. The decoder should choose a
       preferred/optimal format for the default configuration. For example, a
       YUV format may be preferred over an RGB format if an additional
       conversion step would be required for the latter.

```

2. **鍙€夈€?* 閫氳繃 `VIDIOC_G_SELECTION` 鑾峰彇鍙鍒嗚鲸鐜囥€?

    - **蹇呭～瀛楁锛?*

      `type`
          `CAPTURE` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

      `target`
          璁句负 `V4L2_SEL_TGT_COMPOSE`銆?

    - **杩斿洖瀛楁锛?*

      `r.left`銆乣r.top`銆乣r.width`銆乣r.height`
          鍙鐭╁舰锛涘畠蹇呴』钀藉湪 `CAPTURE` 涓?`VIDIOC_G_FMT` 杩斿洖鐨勫抚缂撳啿鍖?
          鍒嗚鲸鐜囦箣鍐呫€?

    - 鍦?`CAPTURE` 涓婃敮鎸佷互涓嬮€夋嫨鐩爣锛?

      `V4L2_SEL_TGT_CROP_BOUNDS`
          瀵瑰簲浜庢暟鎹祦鐨勭紪鐮佸垎杈ㄧ巼銆?

      `V4L2_SEL_TGT_CROP_DEFAULT`
          瑕嗙洊 `CAPTURE` 缂撳啿鍖轰腑鍖呭惈鏈夋剰涔夊浘鍍忔暟鎹紙鍙鍖哄煙锛夐儴鍒嗙殑鐭╁舰锛?
          鍏跺搴﹀拰楂樺害绛変簬鏁版嵁娴佺殑鍙鍒嗚鲸鐜囥€?

      `V4L2_SEL_TGT_CROP`
          缂栫爜鍒嗚鲸鐜囧唴灏嗚杈撳嚭鍒?`CAPTURE` 鐨勭煩褰紱榛樿绛変簬
          `V4L2_SEL_TGT_CROP_DEFAULT`锛涘湪涓嶅叿澶囬澶?compose/缂╂斁鑳藉姏鐨勭‖浠朵笂
          涓哄彧璇汇€?

      `V4L2_SEL_TGT_COMPOSE_BOUNDS`
          `CAPTURE` 缂撳啿鍖轰腑瑁佸壀鍚庡抚鍙鍚堟垚鍒扮殑鏈€澶х煩褰紱鑻ョ‖浠朵笉鏀寔
          compose/缂╂斁锛屽垯绛変簬 `V4L2_SEL_TGT_CROP`銆?

      `V4L2_SEL_TGT_COMPOSE_DEFAULT`
          绛変簬 `V4L2_SEL_TGT_CROP`銆?

      `V4L2_SEL_TGT_COMPOSE`
          `CAPTURE` 缂撳啿鍖轰腑鍐欏叆瑁佸壀鍚庡抚鐨勭煩褰紱榛樿绛変簬
          `V4L2_SEL_TGT_COMPOSE_DEFAULT`锛涘湪涓嶅叿澶囬澶?compose/缂╂斁鑳藉姏鐨勭‖浠朵笂
          涓哄彧璇汇€?

      `V4L2_SEL_TGT_COMPOSE_PADDED`
          `CAPTURE` 缂撳啿鍖轰腑琚‖浠惰鐩栫殑鐭╁舰锛涜嫢纭欢涓嶅啓鍏ュ～鍏呭儚绱狅紝鍒欑瓑浜?
          `V4L2_SEL_TGT_COMPOSE`銆?

```

       The values are guaranteed to be meaningful only after the decoder
       successfully parses the stream metadata. The client must not rely on the
       query before that happens.

```

3. **鍙€夈€?* 閫氳繃 `CAPTURE` 闃熷垪涓婄殑 `VIDIOC_ENUM_FMT` 鏋氫妇
    `CAPTURE` 鏍煎紡銆備竴鏃︽祦淇℃伅琚В鏋愬苟宸茬煡锛屽鎴风鍙互浣跨敤姝?ioctl 鏉ュ彂鐜?
    缁欏畾娴佹敮鎸佸摢浜涘師濮嬫牸寮忥紝骞堕€氳繃 `VIDIOC_S_FMT` 閫夋嫨鍏朵腑涔嬩竴銆?

```

       The decoder will return only formats supported for the currently
       established coded format, as per the ``OUTPUT`` format and/or stream
       metadata parsed in this initialization sequence, even if more formats
       may be supported by the decoder in general. In other words, the set
       returned will be a subset of the initial query mentioned in the
       `Querying Capabilities` section.

       For example, a decoder may support YUV and RGB formats for resolutions
       1920x1088 and lower, but only YUV for higher resolutions (due to
       hardware limitations). After parsing a resolution of 1920x1088 or lower,
       :c:func:`VIDIOC_ENUM_FMT` may return a set of YUV and RGB pixel formats,
       but after parsing resolution higher than 1920x1088, the decoder will not
       return RGB, unsupported for this resolution.

       However, subsequent resolution change event triggered after
       discovering a resolution change within the same stream may switch
       the stream into a lower resolution and :c:func:`VIDIOC_ENUM_FMT`
       would return RGB formats again in that case.

```

4. **鍙€夈€?* 閫氳繃 `CAPTURE` 闃熷垪涓婄殑 `VIDIOC_S_FMT` 璁剧疆
    `CAPTURE` 鏍煎紡銆傚鎴风鍙互閫夋嫨涓嶅悓浜庤В鐮佸櫒鍦?`VIDIOC_G_FMT` 涓?
    閫夋嫨/寤鸿鐨勬牸寮忋€?

    - **蹇呭～瀛楁锛?*

      `type`
          `CAPTURE` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

      `pixelformat`
          涓€绉嶅師濮嬪儚绱犳牸寮忋€?

      `width`銆乣height`
          宸茶В鐮佹祦鐨勫抚缂撳啿鍖哄垎杈ㄧ巼锛涢€氬父涓?`VIDIOC_G_FMT` 杩斿洖鐨勫€肩浉鍚岋紝
          浣嗗鏋滅‖浠舵敮鎸佸悎鎴愶紙composition锛夊拰/鎴栫缉鏀撅紝鍒欏彲鑳戒笉鍚屻€?

   - 濡傚墠涓€鑺傛墍杩帮紝璁剧疆 `CAPTURE` 鏍煎紡浼氭牴鎹柊鍒嗚鲸鐜囧皢 compose 閫夋嫨鐭╁舰
     閲嶇疆涓哄畠浠殑榛樿鍊笺€?

5. **鍙€夈€?* 濡傛灉鏈熸湜涓旇В鐮佸櫒鍏峰 compose 鍜?鎴栫缉鏀捐兘鍔涳紝閫氳繃 `CAPTURE`
   闃熷垪涓婄殑 `VIDIOC_S_SELECTION` 璁剧疆 compose 鐭╁舰銆?

   - **蹇呭～瀛楁锛?*

     `type`
         `CAPTURE` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

     `target`
         璁句负 `V4L2_SEL_TGT_COMPOSE`銆?

     `r.left`銆乣r.top`銆乣r.width`銆乣r.height`
         `CAPTURE` 缂撳啿鍖轰腑鍐欏叆瑁佸壀鍚庡抚鐨勭煩褰紱榛樿绛変簬
         `V4L2_SEL_TGT_COMPOSE_DEFAULT`锛涘湪涓嶅叿澶囬澶?compose/缂╂斁鑳藉姏鐨?
         纭欢涓婁负鍙銆?

   - **杩斿洖瀛楁锛?*

     `r.left`銆乣r.top`銆乣r.width`銆乣r.height`
         鍙鐭╁舰锛涘畠蹇呴』钀藉湪 `CAPTURE` 涓?`VIDIOC_G_FMT` 杩斿洖鐨勫抚缂撳啿鍖?
         鍒嗚鲸鐜囦箣鍐呫€?

```

      The decoder may adjust the compose rectangle to the nearest
      supported one to meet codec and hardware requirements. The client needs
      to check the adjusted rectangle returned by :c:func:`VIDIOC_S_SELECTION`.

```

6. 濡傛灉婊¤冻浠ヤ笅鎵€鏈夋潯浠讹紝瀹㈡埛绔彲浠ョ珛鍗虫仮澶嶈В鐮侊細

    - 鏂版牸寮忥紙鍦ㄥ墠闈㈡楠や腑纭畾锛夌殑 `sizeimage` 灏忎簬鎴栫瓑浜庡綋鍓嶅凡鍒嗛厤缂撳啿鍖虹殑
      澶у皬锛?

    - 褰撳墠宸插垎閰嶇殑缂撳啿鍖烘暟閲忓ぇ浜庢垨绛変簬鍓嶉潰姝ラ涓幏鍙栫殑鏈€灏忕紦鍐插尯鏁伴噺銆備负婊¤冻
      姝よ姹傦紝瀹㈡埛绔彲浠ヤ娇鐢?`VIDIOC_CREATE_BUFS` 鏉ユ柊澧炵紦鍐插尯銆?

    鍦ㄨ繖绉嶆儏鍐典笅锛屽墿浣欐楠や笉閫傜敤锛屽鎴风鍙互閫氳繃涓嬪垪鎿嶄綔涔嬩竴鎭㈠瑙ｇ爜锛?

    - 濡傛灉 `CAPTURE` 闃熷垪姝ｅ湪娴佸紡浼犺緭锛屽垯浣跨敤 `V4L2_DEC_CMD_START` 鍛戒护
      璋冪敤 `VIDIOC_DECODER_CMD`锛?

    - 濡傛灉 `CAPTURE` 闃熷垪鏈祦寮忎紶杈擄紝鍒欏湪 `CAPTURE` 闃熷垪涓婅皟鐢?
      `VIDIOC_STREAMON`銆?

    浣嗘槸锛屽鏋滃鎴风鍑轰簬闄嶄綆鍐呭瓨鍗犵敤鎴栧叾浠栦换浣曞師鍥犳墦绠楁洿鏀圭紦鍐插尯闆嗗悎锛屽垯鍙互
    閫氳繃鎵ц浠ヤ笅姝ラ瀹炵幇銆?

7. **濡傛灉** `CAPTURE` **闃熷垪姝ｅ湪娴佸紡浼犺緭锛?* 缁х画鍦?`CAPTURE` 闃熷垪涓?
    鍏ラ槦鍜屽嚭闃熺紦鍐插尯锛岀洿鍒板嚭涓€涓甫鏈?`V4L2_BUF_FLAG_LAST` 鏍囧織鐨勭紦鍐插尯銆?

8. **濡傛灉** `CAPTURE` **闃熷垪姝ｅ湪娴佸紡浼犺緭锛?* 鍦?`CAPTURE` 闃熷垪涓婅皟鐢?
    `VIDIOC_STREAMOFF` 浠ュ仠姝㈡祦寮忎紶杈撱€?

```

       The ``OUTPUT`` queue must remain streaming. Calling
       :c:func:`VIDIOC_STREAMOFF` on it would abort the sequence and trigger a
       seek.

```

9. **濡傛灉** `CAPTURE` **闃熷垪宸插垎閰嶇紦鍐插尯锛?* 浣跨敤 `VIDIOC_REQBUFS`
    閲婃斁 `CAPTURE` 缂撳啿鍖恒€?

    - **蹇呭～瀛楁锛?*

      `count`
          璁句负 0銆?

      `type`
          `CAPTURE` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

      `memory`
          閬靛惊鏍囧噯璇箟銆?

10. 閫氳繃 `CAPTURE` 闃熷垪涓婄殑 `VIDIOC_REQBUFS` 鍒嗛厤 `CAPTURE` 缂撳啿鍖恒€?

    - **蹇呭～瀛楁锛?*

      `count`
          璇锋眰鐨勭紦鍐插尯鍒嗛厤鏁伴噺锛涘繀椤诲ぇ浜庨浂銆?

      `type`
          `CAPTURE` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

      `memory`
          閬靛惊鏍囧噯璇箟銆?

    - **杩斿洖瀛楁锛?*

      `count`
          瀹為檯鍒嗛厤鐨勭紦鍐插尯鏁伴噺銆?

```

       The actual number of allocated buffers may differ from the ``count``
       given. The client must check the updated value of ``count`` after the
       call returns.

    .. note::

       To allocate more than the minimum number of buffers (for pipeline
       depth), the client may query the ``V4L2_CID_MIN_BUFFERS_FOR_CAPTURE``
       control to get the minimum number of buffers required, and pass the
       obtained value plus the number of additional buffers needed in the
       ``count`` field to :c:func:`VIDIOC_REQBUFS`.

    Alternatively, :c:func:`VIDIOC_CREATE_BUFS` on the ``CAPTURE`` queue can be
    used to have more control over buffer allocation. For example, by
    allocating buffers larger than the current ``CAPTURE`` format, future
    resolution changes can be accommodated.

    * **Required fields:**

      ``count``
          requested number of buffers to allocate; greater than zero.

      ``type``
          a ``V4L2_BUF_TYPE_*`` enum appropriate for ``CAPTURE``.

      ``memory``
          follows standard semantics.

      ``format``
          a format representing the maximum framebuffer resolution to be
          accommodated by newly allocated buffers.

    * **Returned fields:**

      ``count``
          adjusted to the number of allocated buffers.

    .. warning::

        The actual number of allocated buffers may differ from the ``count``
        given. The client must check the updated value of ``count`` after the
        call returns.

    .. note::

       To allocate buffers for a format different than parsed from the stream
       metadata, the client must proceed as follows, before the metadata
       parsing is initiated:

       * set width and height of the ``OUTPUT`` format to desired coded resolution to
         let the decoder configure the ``CAPTURE`` format appropriately,

       * query the ``CAPTURE`` format using :c:func:`VIDIOC_G_FMT` and save it
         until this step.

       The format obtained in the query may be then used with
       :c:func:`VIDIOC_CREATE_BUFS` in this step to allocate the buffers.

```

11. 鍦?`CAPTURE` 闃熷垪涓婅皟鐢?`VIDIOC_STREAMON` 浠ュ紑濮嬭В鐮佸抚銆?

## 瑙ｇ爜

`Capture Setup` 搴忓垪鎴愬姛瀹屾垚鍚庡嵆杩涘叆姝ょ姸鎬併€傚湪姝ょ姸鎬佷笅锛屽鎴风閫氳繃
`VIDIOC_QBUF` 鍜?`VIDIOC_DQBUF` 鎸夌収鏍囧噯璇箟鍚戜袱涓槦鍒楀叆闃熷拰鍑洪槦缂撳啿鍖恒€?

婧?`OUTPUT` 缂撳啿鍖虹殑鍐呭鍙栧喅浜庡綋鍓嶆椿鍔ㄧ殑缂栫爜鍍忕礌鏍煎紡锛屽苟鍙兘鍙楃紪瑙ｇ爜鍣?
鐗瑰畾鐨勬墿灞曟帶浠跺奖鍝嶏紝濡傛瘡绉嶆牸寮忕殑鏂囨。鎵€杩般€?

涓や釜闃熷垪鐙珛杩愯锛岄伒寰?V4L2 缂撳啿鍖洪槦鍒楀拰鍐呭瓨鍒板唴瀛橈紙memory-to-memory锛夎澶囩殑
鏍囧噯琛屼负銆傛澶栵紝鐢变簬鎵€閫夌紪鐮佹牸寮忕殑鐗规€э紙渚嬪甯ч噸鎺掑簭锛夛紝浠?`CAPTURE` 闃熷垪
鍑洪槦鐨勫凡瑙ｇ爜甯х殑椤哄簭锛屽彲鑳戒笌鍚?`OUTPUT` 闃熷垪鍏ラ槦缂栫爜甯х殑椤哄簭涓嶅悓銆?

瀹㈡埛绔笉寰楀亣瀹?`CAPTURE` 涓?`OUTPUT` 缂撳啿鍖轰箣闂达紝浠ュ強缂撳啿鍖哄彲琚嚭闃熺殑
浠讳綍鐗瑰畾鏃跺簭涔嬮棿瀛樺湪浠讳綍鐩存帴鍏崇郴銆傚叿浣撹€岃█锛?

- 鍏ラ槦鍒?`OUTPUT` 鐨勭紦鍐插尯鍙兘涓嶅湪 `CAPTURE` 涓婁骇鐢熶换浣曠紦鍐插尯锛堜緥濡傦紝濡傛灉
  瀹冧笉鍖呭惈宸茬紪鐮佹暟鎹紝鎴栬€呭叾涓粎瀛樺湪鍏冩暟鎹娉曠粨鏋勶級锛?

- 鍏ラ槦鍒?`OUTPUT` 鐨勭紦鍐插尯鍙兘鍦?`CAPTURE` 涓婁骇鐢熷浜庝竴涓紦鍐插尯锛堝鏋滃凡缂栫爜
  鏁版嵁鍖呭惈澶氫釜甯э紝鎴栬€呰繑鍥炰竴涓凡瑙ｇ爜甯т娇寰楄В鐮佸櫒鑳藉杩斿洖涓€涓湪瑙ｇ爜椤哄簭涓婁綅浜庡叾
  涔嬪墠銆佷絾鍦ㄦ樉绀洪『搴忎笂浣嶄簬鍏朵箣鍚庣殑甯э級锛?

- 鍏ラ槦鍒?`OUTPUT` 鐨勭紦鍐插尯鍙兘鍦ㄨВ鐮佽繃绋嬬殑鏇存櫄闃舵銆佸拰/鎴栧湪澶勭悊浜嗘洿澶?
  `OUTPUT` 缂撳啿鍖轰箣鍚庯紝鎵嶅湪 `CAPTURE` 涓婁骇鐢熺紦鍐插尯锛屾垨鑰呬贡搴忚繑鍥烇紙渚嬪锛?
  濡傛灉浣跨敤浜嗘樉绀洪噸鎺掑簭锛夛紝

- 鍗充娇娌℃湁鍚?`OUTPUT` 棰濆鍏ラ槦缂撳啿鍖猴紝`CAPTURE` 闃熷垪涓婁篃鍙兘浼氬嚭鐜板彲鐢?
  缂撳啿鍖猴紙渚嬪鍦?drain 鎴?`EOS` 鏈熼棿锛夛紝杩欐槸鍥犱负杩囧幓鍏ラ槦鍒?`OUTPUT` 鐨?
  鏌愪簺缂撳啿鍖猴紝鍏惰В鐮佺粨鏋滅敱浜庤В鐮佽繃绋嬬殑鐗规€ц绛夊埌鏇存櫄鐨勬椂鍒绘墠鍙敤銆?

   涓轰簡鑳藉灏嗗凡瑙ｇ爜鐨?`CAPTURE` 缂撳啿鍖轰笌浜х敓瀹冧滑鐨?`OUTPUT` 缂撳啿鍖哄搴旇捣鏉ワ紝
   瀹㈡埛绔彲浠ュ湪鍏ラ槦 `OUTPUT` 缂撳啿鍖烘椂璁剧疆 `v4l2_buffer` 缁撴瀯浣撶殑 `timestamp`
   瀛楁銆傜敱瑙ｇ爜璇?`OUTPUT` 缂撳啿鍖烘墍浜х敓鐨?`CAPTURE` 缂撳啿鍖猴紝鍦ㄥ嚭闃熸椂鍏?
   `timestamp` 瀛楁灏嗚璁句负鐩稿悓鐨勫€笺€?

   闄や簡涓€涓?`OUTPUT` 缂撳啿鍖轰骇鐢熶竴涓?`CAPTURE` 缂撳啿鍖鸿繖绉嶇畝鍗曟儏鍐靛锛岃繕瀹氫箟浜?
   浠ヤ笅鎯呭喌锛?

   - 涓€涓?`OUTPUT` 缂撳啿鍖轰骇鐢熷涓?`CAPTURE` 缂撳啿鍖猴細鍚屼竴涓?`OUTPUT` 鏃堕棿鎴?
     灏嗚澶嶅埗鍒板涓?`CAPTURE` 缂撳啿鍖恒€?

   - 澶氫釜 `OUTPUT` 缂撳啿鍖轰骇鐢熶竴涓?`CAPTURE` 缂撳啿鍖猴細灏嗗鍒舵渶鍏堝叆闃熺殑
     `OUTPUT` 缂撳啿鍖虹殑鏃堕棿鎴炽€?

   - 瑙ｇ爜椤哄簭涓庢樉绀洪『搴忎笉鍚岋紙鍗?`CAPTURE` 缂撳啿鍖虹浉瀵逛簬 `OUTPUT` 缂撳啿鍖烘槸
     涔卞簭鐨勶級锛歚CAPTURE` 鏃堕棿鎴冲皢涓嶄細淇濈暀 `OUTPUT` 鏃堕棿鎴崇殑椤哄簭銆?


   琚祦鐢ㄤ綔鍙傝€冨抚鐨?`CAPTURE` 缂撳啿鍖猴紝鍏跺簳灞傚唴瀛樺湪鍑洪槦鍚庝粛鍙兘琚‖浠惰鍙栥€?
   鍥犳锛屽鎴风搴旈伩鍏嶅湪 `CAPTURE` 闃熷垪娴佸紡浼犺緭鏈熼棿鍐欏叆杩欏潡鍐呭瓨銆傚惁鍒欏彲鑳藉鑷?
   宸茶В鐮佸抚鎹熷潖銆?

   绫讳技鍦帮紝褰撲娇鐢ㄧ殑鍐呭瓨绫诲瀷涓嶆槸 `V4L2_MEMORY_MMAP` 鏃讹紝瀹㈡埛绔簲纭繚鍦?
   `CAPTURE` 闃熷垪娴佸紡浼犺緭鏈熼棿锛屾瘡涓?`CAPTURE` 缂撳啿鍖哄缁堜娇鐢ㄧ浉鍚岀殑搴曞眰鍐呭瓨
   鍏ラ槦銆傚師鍥犳槸 V4L2 缂撳啿鍖虹储寮曞彲琚┍鍔ㄧ敤鏉ヨ瘑鍒抚銆傚洜姝わ紝濡傛灉鍙傝€冨抚鐨勫簳灞傚唴瀛?
   浠ヤ笉鍚岀殑缂撳啿鍖?ID 鎻愪氦锛岄┍鍔ㄥ彲鑳戒細璇瘑鍒畠锛屽苟鍦ㄥ叾浠嶈浣跨敤鏃跺皢鏂板抚瑙ｇ爜鍒板叾涓紝
   浠庤€屽鑷村悗缁抚鎹熷潖銆?

鍦ㄨВ鐮佽繃绋嬩腑锛岃В鐮佸櫒鍙兘浼氬惎鍔ㄤ笅鍒楃壒娈婂簭鍒椾箣涓€銆傝繖浜涘簭鍒椾細瀵艰嚧瑙ｇ爜鍣ㄨ繑鍥炴墍鏈?
鍦ㄥ簭鍒楀紑濮嬩箣鍓嶅鐞嗙殑 `OUTPUT` 缂撳啿鍖烘墍浜х敓鐨?`CAPTURE` 缂撳啿鍖恒€傛渶鍚庝竴涓?
缂撳啿鍖哄皢甯︽湁 `V4L2_BUF_FLAG_LAST` 鏍囧織銆備负浜嗙‘瀹氶渶瑕侀伒寰摢涓簭鍒楋紝瀹㈡埛绔繀椤?
妫€鏌ユ槸鍚﹀瓨鍦ㄥ緟澶勭悊浜嬩欢锛屽苟涓旓細

- 濡傛灉寰呭鐞嗙殑鏄?`changes` 璁句负 `V4L2_EVENT_SRC_CH_RESOLUTION` 鐨?
  `V4L2_EVENT_SOURCE_CHANGE` 浜嬩欢锛屽垯闇€瑕侀伒寰?`Dynamic Resolution
  Change` 搴忓垪锛?

- 濡傛灉寰呭鐞嗙殑鏄?`V4L2_EVENT_EOS` 浜嬩欢锛屽垯闇€瑕侀伒寰?`End of Stream` 搴忓垪銆?

鏌愪簺搴忓垪鍙互鐩镐簰浜ら敊锛岄渶瑕佹寜鍙戠敓鏃剁殑鎯呭舰澶勭悊銆傛瘡涓簭鍒楃殑纭垏鎿嶄綔鍦ㄧ浉搴旂珷鑺備腑
鏈夋枃妗ｈ鏄庛€?

濡傛灉鍙戠敓瑙ｇ爜閿欒锛屽皢鏍规嵁瑙ｇ爜鍣ㄧ殑鑳藉姏锛屼互涓嶅悓鐨勮缁嗙▼搴︽姤鍛婄粰瀹㈡埛绔€傚叿浣撹€岃█锛?

- 鍖呭惈澶辫触瑙ｇ爜鎿嶄綔缁撴灉鐨?CAPTURE 缂撳啿鍖哄皢琚繑鍥烇紝骞跺甫鏈?V4L2_BUF_FLAG_ERROR 鏍囧織锛?

- 濡傛灉瑙ｇ爜鍣ㄨ兘澶熺簿纭姤鍛婅Е鍙戦敊璇殑 OUTPUT 缂撳啿鍖猴紝鍒欒缂撳啿鍖哄皢琚繑鍥烇紝骞跺甫鏈?
  V4L2_BUF_FLAG_ERROR 鏍囧織銆?

濡傛灉鍙戠敓涓嶅厑璁哥户缁В鐮佺殑鑷村懡澶辫触锛屽垯瀵硅瑙ｇ爜鍣ㄦ枃浠跺彞鏌勭殑浠讳綍杩涗竴姝ユ搷浣滈兘浼氳繑鍥?
-EIO 閿欒鐮併€傚鎴风鍙互鍏抽棴璇ユ枃浠跺彞鏌勫苟鎵撳紑涓€涓柊鐨勶紝鎴栬€呴€氳繃鍦ㄤ袱涓槦鍒椾笂鍋滄
娴佸紡浼犺緭銆侀噴鏀炬墍鏈夌紦鍐插尯骞跺啀娆℃墽琛?`Initialization` 搴忓垪鏉ラ噸鏂板垵濮嬪寲瀹炰緥銆?

## 瀹氫綅锛圫eek锛?

Seek 鐢?`OUTPUT` 闃熷垪鎺у埗锛屽洜涓哄畠鏄凡缂栫爜鏁版嵁鐨勬潵婧愩€俿eek 涓嶉渶瑕佸 `CAPTURE`
闃熷垪鎵ц浠讳綍鐗瑰畾鎿嶄綔锛屼絾瀹冨彲鑳戒細鍙楀埌瑙ｇ爜鍣ㄦ甯告搷浣滅殑褰卞搷銆?

1. 閫氳繃 `VIDIOC_STREAMOFF` 鍋滄 `OUTPUT` 闃熷垪浠ュ紑濮?seek 搴忓垪銆?

   - **蹇呭～瀛楁锛?*

     `type`
         `OUTPUT` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

   - 瑙ｇ爜鍣ㄥ皢涓㈠純鎵€鏈夊緟澶勭悊鐨?`OUTPUT` 缂撳啿鍖猴紝瀹冧滑蹇呴』琚涓哄凡杩旇繕缁欏鎴风
     锛堥伒寰爣鍑嗚涔夛級銆?

2. 閫氳繃 `VIDIOC_STREAMON` 閲嶅惎 `OUTPUT` 闃熷垪銆?

   - **蹇呭～瀛楁锛?*

     `type`
         `OUTPUT` 閫傜敤鐨?`V4L2_BUF_TYPE_*` 鏋氫妇鍊笺€?

   - 璋冪敤杩斿洖鍚庯紝瑙ｇ爜鍣ㄥ皢寮€濮嬫帴鍙楁柊鐨勬簮鏁版嵁娴佺紦鍐插尯銆?

3. 寮€濮嬪皢鍖呭惈 seek 涔嬪悗缂栫爜鏁版嵁鐨勭紦鍐插尯鍏ラ槦鍒?`OUTPUT` 闃熷垪锛岀洿鍒版壘鍒板悎閫傜殑
   鎭㈠鐐癸紙resume point锛夈€?

```

      There is no requirement to begin queuing coded data starting exactly
      from a resume point (e.g. SPS or a keyframe). Any queued ``OUTPUT``
      buffers will be processed and returned to the client until a suitable
      resume point is found.  While looking for a resume point, the decoder
      should not produce any decoded frames into ``CAPTURE`` buffers.

      Some hardware is known to mishandle seeks to a non-resume point. Such an
      operation may result in an unspecified number of corrupted decoded frames
      being made available on the ``CAPTURE`` queue. Drivers must ensure that
      no fatal decoding errors or crashes occur, and implement any necessary
      handling and workarounds for hardware issues related to seek operations.

   .. warning::

      In case of the H.264/HEVC codec, the client must take care not to seek
      over a change of SPS/PPS. Even though the target frame could be a
      keyframe, the stale SPS/PPS inside decoder state would lead to undefined
      results when decoding. Although the decoder must handle that case without
      a crash or a fatal decode error, the client must not expect a sensible
      decode output.

      If the hardware can detect such corrupted decoded frames, then
      corresponding buffers will be returned to the client with the
      V4L2_BUF_FLAG_ERROR set. See the `Decoding` section for further
      description of decode error reporting.

```

4. 鎵惧埌鎭㈠鐐瑰悗锛岃В鐮佸櫒灏嗗紑濮嬭繑鍥炲寘鍚凡瑙ｇ爜甯х殑 `CAPTURE` 缂撳啿鍖恒€?


   seek 鍙兘瀵艰嚧 `Dynamic Resolution Change` 搴忓垪琚惎鍔紝鍘熷洜鏄?seek 鐩爣鐨?
   瑙ｇ爜鍙傛暟涓?seek 涔嬪墠宸茶В鐮佺殑娴侀儴鍒嗕笉鍚屻€傚繀椤绘寜瑙ｇ爜鍣ㄧ殑姝ｅ父鎿嶄綔鏉ュ鐞嗚搴忓垪銆?


   鏈瀹?`CAPTURE` 闃熷垪浣曟椂寮€濮嬩骇鐢熷寘鍚?seek 涔嬪悗鍏ラ槦鐨?`OUTPUT` 缂撳啿鍖?
   瑙ｇ爜鏁版嵁鐨勭紦鍐插尯锛屽洜涓哄畠涓?`OUTPUT` 闃熷垪鐙珛杩愯銆?

   瑙ｇ爜鍣ㄥ彲鑳戒細杩斿洖鑻ュ共鍓╀綑鐨?`CAPTURE` 缂撳啿鍖猴紝鍏朵腑鍖呭惈鐢卞湪 seek 搴忓垪鎵ц鍓?
   鍏ラ槦鐨?`OUTPUT` 缂撳啿鍖轰骇鐢熺殑宸茶В鐮佸抚銆?

   `VIDIOC_STREAMOFF` 鎿嶄綔浼氫涪寮冩墍鏈夊墿浣欑殑宸插叆闃?`OUTPUT` 缂撳啿鍖猴紝杩欐剰鍛崇潃
   骞堕潪鎵€鏈夊湪 seek 搴忓垪涔嬪墠鍏ラ槦鐨?`OUTPUT` 缂撳啿鍖洪兘涓€瀹氭湁瀵瑰簲鐨?`CAPTURE`
   缂撳啿鍖轰骇鐢熴€備緥濡傦紝缁欏畾 `OUTPUT` 闃熷垪涓婄殑鎿嶄綔搴忓垪锛?

     QBUF(A), QBUF(B), STREAMOFF(), STREAMON(), QBUF(G), QBUF(H),

   鍦?`CAPTURE` 闃熷垪涓婂嚭鐜颁互涓嬩换浣曠粨鏋滈兘鏄厑璁哥殑锛?

     {A', B', G', H'}, {A', G', H'}, {G', H'}.

   瑕佺‘瀹?seek 涔嬪悗鍖呭惈绗竴涓凡瑙ｇ爜甯х殑 CAPTURE 缂撳啿鍖猴紝瀹㈡埛绔彲浠ヨ瀵熸椂闂存埑浠?
   鍖归厤 CAPTURE 鍜?OUTPUT 缂撳啿鍖猴紝鎴栬€呬娇鐢?V4L2_DEC_CMD_STOP 鍜?
   V4L2_DEC_CMD_START 鏉ユ帓绌猴紙drain锛夎В鐮佸櫒銆?


   涓轰簡瀹炵幇鍗虫椂 seek锛屽鎴风涔熷彲浠ュ湪 `CAPTURE` 闃熷垪涓婇噸鍚祦寮忎紶杈擄紝浠ヤ涪寮冨凡
   瑙ｇ爜浣嗗皻鏈嚭闃熺殑缂撳啿鍖恒€?

## 鍔ㄦ€佸垎杈ㄧ巼鍙樺寲锛圖ynamic Resolution Change锛?

鍦ㄦ祦涓寘鍚垎杈ㄧ巼鍏冩暟鎹殑鏁版嵁娴侊紝鍙兘鍦ㄨВ鐮佽繃绋嬩腑闇€瑕佸垏鎹㈠埌涓嶅悓鐨勫垎杈ㄧ巼銆?


   骞堕潪鎵€鏈夎В鐮佸櫒閮借兘妫€娴嬪垎杈ㄧ巼鍙樺寲銆傞偅浜涜兘澶熸娴嬬殑瑙ｇ爜鍣ㄤ細鍦ㄨ皟鐢?
   `VIDIOC_ENUM_FMT` 鏃讹紝涓虹紪鐮佹牸寮忚缃?`V4L2_FMT_FLAG_DYN_RESOLUTION`
   鏍囧織銆?

褰撹В鐮佸櫒妫€娴嬪埌鏌愪竴缂栫爜甯х殑浠ヤ笅涓€涓垨澶氫釜鍙傛暟锛屼笌涔嬪墠宸茬‘绔嬶紙骞跺弽鏄犲湪鐩稿簲鏌ヨ涓級
鐨勫弬鏁颁笉鍚屾椂锛屽簭鍒楀嵆寮€濮嬶細

- 缂栫爜鍒嗚鲸鐜囷紙`OUTPUT` 鐨勫搴﹀拰楂樺害锛夛紝

- 鍙鍒嗚鲸鐜囷紙閫夋嫨鐭╁舰锛夛紝

- 瑙ｇ爜鎵€闇€鐨勬渶灏忕紦鍐插尯鏁伴噺锛?

- 鏁版嵁娴佺殑浣嶆繁锛坆it-depth锛夊凡鏀瑰彉锛?

- 鏁版嵁娴佺殑鑹插僵绌洪棿锛坈olorspace锛夊凡鏀瑰彉锛屼絾涓嶉渶瑕侀噸鏂板垎閰嶇紦鍐插尯銆?

涓€鏃﹀彂鐢熶笂杩版儏鍐碉紝瑙ｇ爜鍣ㄥ繀椤绘寜濡備笅鏂瑰紡缁х画锛?

1. 鍦ㄦ祦涓亣鍒板垎杈ㄧ巼鍙樺寲鍚庯紝瑙ｇ爜鍣ㄥ彂閫?`changes` 璁句负
    `V4L2_EVENT_SRC_CH_RESOLUTION` 鐨?`V4L2_EVENT_SOURCE_CHANGE` 浜嬩欢銆?

```

       Any client query issued after the decoder queues the event will return
       values applying to the stream after the resolution change, including
       queue formats, selection rectangles and controls.

```

2. 鐒跺悗锛岃В鐮佸櫒灏嗗鐞嗗苟瑙ｇ爜鍒嗚鲸鐜囧彉鍖栫偣涔嬪墠鐨勬墍鏈夊墿浣欑紦鍐插尯銆?

    - 鍙樺寲涔嬪墠鐨勬渶鍚庝竴涓紦鍐插尯蹇呴』甯︽湁 `V4L2_BUF_FLAG_LAST` 鏍囧織锛岀被浼间簬
      涓婇潰鐨?`Drain` 搴忓垪銆?

```

       The last buffer may be empty (with :c:type:`v4l2_buffer` ``bytesused``
       = 0) and in that case it must be ignored by the client, as it does not
       contain a decoded frame.

    .. note::

       Any attempt to dequeue more ``CAPTURE`` buffers beyond the buffer marked
       with ``V4L2_BUF_FLAG_LAST`` will result in a -EPIPE error from
       :c:func:`VIDIOC_DQBUF`.

```

瀹㈡埛绔繀椤绘寜鐓т笅杩版柟寮忕户缁搴忓垪锛屼互缁х画瑙ｇ爜杩囩▼銆?

1. 鍑洪槦婧愬彉鍖栦簨浠躲€?

```

       A source change triggers an implicit decoder drain, similar to the
       explicit `Drain` sequence. The decoder is stopped after it completes.
       The decoding process must be resumed with either a pair of calls to
       :c:func:`VIDIOC_STREAMOFF` and :c:func:`VIDIOC_STREAMON` on the
       ``CAPTURE`` queue, or a call to :c:func:`VIDIOC_DECODER_CMD` with the
       ``V4L2_DEC_CMD_START`` command.

```

2. 缁х画 `Capture Setup` 搴忓垪銆?


   鍦ㄥ垎杈ㄧ巼鍙樺寲搴忓垪鏈熼棿锛宍OUTPUT` 闃熷垪蹇呴』淇濇寔娴佸紡浼犺緭銆傚湪 `OUTPUT` 闃熷垪涓?
   璋冪敤 `VIDIOC_STREAMOFF` 浼氫腑姝㈣搴忓垪骞跺惎鍔ㄤ竴娆?seek銆?

   鍘熷垯涓婏紝`OUTPUT` 闃熷垪涓?`CAPTURE` 闃熷垪鐙珛杩愯锛屽湪鏁翠釜鍒嗚鲸鐜囧彉鍖栧簭鍒楁湡闂?
   涔熸槸濡傛銆?

   涓轰簡鑾峰緱鏈€浣虫€ц兘鍜岀畝渚挎€э紝瀹㈡埛绔嵆浣垮湪澶勭悊姝ゅ簭鍒楁椂锛屼篃搴旂户缁悜 `OUTPUT`
   闃熷垪鍏ラ槦/鍑洪槦缂撳啿鍖恒€?

## 鎺掔┖锛圖rain锛?

涓轰簡纭繚鎵€鏈夊凡鍏ラ槦鐨?`OUTPUT` 缂撳啿鍖洪兘宸茶澶勭悊锛屼笖鐩稿叧鐨?`CAPTURE` 缂撳啿鍖?
閮藉凡浜ょ粰瀹㈡埛绔紝瀹㈡埛绔繀椤婚伒寰笅杩?drain 搴忓垪銆俤rain 搴忓垪缁撴潫鍚庯紝瀹㈡埛绔凡鏀跺埌
鍦ㄥ簭鍒楀惎鍔ㄤ箣鍓嶅叆闃熺殑鎵€鏈?`OUTPUT` 缂撳啿鍖哄搴旂殑鎵€鏈夊凡瑙ｇ爜甯с€?

1. 閫氳繃鍙戝嚭 `VIDIOC_DECODER_CMD` 寮€濮?drain銆?

   - **蹇呭～瀛楁锛?*

     `cmd`
         璁句负 `V4L2_DEC_CMD_STOP`銆?

     `flags`
         璁句负 0銆?

     `pts`
         璁句负 0銆?

```

      The sequence can be only initiated if both ``OUTPUT`` and ``CAPTURE``
      queues are streaming. For compatibility reasons, the call to
      :c:func:`VIDIOC_DECODER_CMD` will not fail even if any of the queues is
      not streaming, but at the same time it will not initiate the `Drain`
      sequence and so the steps described below would not be applicable.

```

2. 瀹㈡埛绔湪鍙戝嚭 `VIDIOC_DECODER_CMD` 涔嬪墠鍏ラ槦鐨勪换浣?`OUTPUT` 缂撳啿鍖猴紝灏?
   鍍忔甯告儏鍐典竴鏍疯澶勭悊鍜岃В鐮併€傚鎴风蹇呴』缁х画鐙珛澶勭悊涓や釜闃熷垪锛岀被浼间簬姝ｅ父鐨?
   瑙ｇ爜鎿嶄綔銆傝繖鍖呮嫭锛?

   - 鍦ㄥ鐞嗚繖浜涚紦鍐插尯鎵€瑙﹀彂鐨勬墍鏈夋搷浣滐紙渚嬪 `Dynamic Resolution Change`
     搴忓垪锛変箣鍚庯紝鍐嶇户缁?drain 搴忓垪锛?

   - 鍏ラ槦鍜屽嚭闃?`CAPTURE` 缂撳啿鍖猴紝鐩村埌鍑轰竴涓甫鏈?`V4L2_BUF_FLAG_LAST`
     鏍囧織鐨勭紦鍐插尯锛?

```

        The last buffer may be empty (with :c:type:`v4l2_buffer`
        ``bytesused`` = 0) and in that case it must be ignored by the client,
        as it does not contain a decoded frame.

     .. note::

        Any attempt to dequeue more ``CAPTURE`` buffers beyond the buffer
        marked with ``V4L2_BUF_FLAG_LAST`` will result in a -EPIPE error from
        :c:func:`VIDIOC_DQBUF`.

   * dequeuing processed ``OUTPUT`` buffers, until all the buffers queued
     before the ``V4L2_DEC_CMD_STOP`` command are dequeued,

   * dequeuing the ``V4L2_EVENT_EOS`` event, if the client subscribed to it.

   .. note::

      For backwards compatibility, the decoder will signal a ``V4L2_EVENT_EOS``
      event when the last frame has been decoded and all frames are ready to be
      dequeued. It is a deprecated behavior and the client must not rely on it.
      The ``V4L2_BUF_FLAG_LAST`` buffer flag should be used instead.

```

3. 涓€鏃﹀湪 `V4L2_DEC_CMD_STOP` 璋冪敤涔嬪墠鍏ラ槦鐨勬墍鏈?`OUTPUT` 缂撳啿鍖洪兘宸插嚭闃燂紝
   涓旀渶鍚庝竴涓?`CAPTURE` 缂撳啿鍖轰篃宸插嚭闃燂紝瑙ｇ爜鍣ㄥ嵆鍋滄锛屽畠灏嗘帴鍙椾絾涓嶄細澶勭悊浠讳綍
   鏂板叆闃熺殑 `OUTPUT` 缂撳啿鍖猴紝鐩村埌瀹㈡埛绔彂鍑轰互涓嬩换涓€鎿嶄綔锛?

   - `V4L2_DEC_CMD_START` - 瑙ｇ爜鍣ㄤ笉浼氳閲嶇疆锛屽苟灏嗗甫鐫€ drain 涔嬪墠鐨勬墍鏈夌姸鎬?
     姝ｅ父鎭㈠鎿嶄綔锛?

   - 鍦?`CAPTURE` 闃熷垪涓婄殑涓€瀵?`VIDIOC_STREAMOFF` 鍜?`VIDIOC_STREAMON` -
     瑙ｇ爜鍣ㄥ皢姝ｅ父鎭㈠鎿嶄綔锛屼絾闃熷垪涓换浣曚粛瀛樺湪鐨?`CAPTURE` 缂撳啿鍖哄皢琚繑鍥炵粰
     瀹㈡埛绔紝

   - 鍦?`OUTPUT` 闃熷垪涓婄殑涓€瀵?`VIDIOC_STREAMOFF` 鍜?`VIDIOC_STREAMON` - 浠讳綍
     寰呭鐞嗙殑婧愮紦鍐插尯灏嗚杩斿洖缁欏鎴风锛屽苟涓斾細瑙﹀彂 `Seek` 搴忓垪銆?


   涓€鏃?drain 搴忓垪鍚姩锛屽鎴风灏遍渶瑕佹寜涓婅堪姝ラ灏嗗叾椹卞姩鑷冲畬鎴愶紝闄ら潪瀹冮€氳繃鍦?
   `OUTPUT` 鎴?`CAPTURE` 闃熷垪涓婂彂鍑?`VIDIOC_STREAMOFF` 涓璇ヨ繃绋嬨€傚湪 drain
   搴忓垪杩涜鏈熼棿锛屽鎴风涓嶅厑璁稿啀娆″彂鍑?`V4L2_DEC_CMD_START` 鎴?
   `V4L2_DEC_CMD_STOP`锛屽鏋滃皾璇曪紝瀹冧滑灏嗕互 -EBUSY 閿欒鐮佸け璐ャ€?

   铏界劧骞堕潪寮哄埗锛屼絾瑙ｇ爜鍣ㄥ懡浠ょ殑鍙敤鎬у彲浠ラ€氳繃 `VIDIOC_TRY_DECODER_CMD` 鏌ヨ銆?

## 娴佺粨鏉燂紙End of Stream锛?

濡傛灉瑙ｇ爜鍣ㄥ湪娴佷腑閬囧埌娴佺粨鏉燂紙end of stream锛夋爣璁帮紝瑙ｇ爜鍣ㄥ皢鍚姩 `Drain` 搴忓垪锛?
瀹㈡埛绔繀椤绘寜涓婅堪鏂瑰紡澶勭悊璇ュ簭鍒楋紝浣嗚烦杩囧垵濮嬬殑 `VIDIOC_DECODER_CMD`銆?

## 鎻愪氦鐐癸紙Commit Points锛?

璁剧疆鏍煎紡鍜屽垎閰嶇紦鍐插尯浼氳Е鍙戣В鐮佸櫒琛屼负鐨勫彉鍖栥€?

1. 鍦?`OUTPUT` 闃熷垪涓婅缃牸寮忥紝鍙兘浼氭敼鍙?`CAPTURE` 闃熷垪涓婂彈鏀寔/閫氬憡鐨?
   鏍煎紡闆嗗悎銆傜壒鍒湴锛岃繖涔熸剰鍛崇潃 `CAPTURE` 鏍煎紡鍙兘浼氳閲嶇疆锛屽鎴风涓嶅緱渚濊禆
   鍏堝墠璁剧疆鐨勬牸寮忚淇濈暀銆?

2. 鍦?`CAPTURE` 闃熷垪涓婃灇涓炬牸寮忥紝鎬绘槸鍙繑鍥炲綋鍓?`OUTPUT` 鏍煎紡鎵€鏀寔鐨勬牸寮忋€?

3. 鍦?`CAPTURE` 闃熷垪涓婅缃牸寮忥紝涓嶄細鏀瑰彉 `OUTPUT` 闃熷垪涓婂彲鐢ㄦ牸寮忓垪琛ㄣ€傚皾璇?
   璁剧疆涓€涓褰撳墠鎵€閫?`OUTPUT` 鏍煎紡涓嶆敮鎸佺殑 `CAPTURE` 鏍煎紡锛屼細瀵艰嚧瑙ｇ爜鍣ㄥ皢
   鎵€璇锋眰鐨?`CAPTURE` 鏍煎紡璋冩暣涓哄彈鏀寔鐨勬牸寮忋€?

4. 鍦?`OUTPUT` 闃熷垪涓婃灇涓炬牸寮忥紝鎬绘槸杩斿洖鍙楁敮鎸佺紪鐮佹牸寮忕殑瀹屾暣闆嗗悎锛屼笌褰撳墠
   `CAPTURE` 鏍煎紡鏃犲叧銆?

5. 鍙 `OUTPUT` 鎴?`CAPTURE` 闃熷垪涓婂垎閰嶄簡缂撳啿鍖猴紝瀹㈡埛绔氨涓嶅緱鏇存敼 `OUTPUT`
   闃熷垪涓婄殑鏍煎紡銆傚浜庝换浣曟绫绘牸寮忔洿鏀瑰皾璇曪紝椹卞姩閮戒細杩斿洖 -EBUSY 閿欒鐮併€?

鎬昏€岃█涔嬶紝璁剧疆鏍煎紡鍜屽垎閰嶅繀椤诲缁堜粠 `OUTPUT` 闃熷垪寮€濮嬶紝骞朵笖 `OUTPUT` 闃熷垪鏄?
鎺岀 `CAPTURE` 闃熷垪鍙楁敮鎸佹牸寮忛泦鍚堢殑涓绘帶鏂广€?
