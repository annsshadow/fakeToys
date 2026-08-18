######## 鍐呭瓨鍒板唴瀛樻湁鐘舵€佽棰戠紪鐮佸櫒鎺ュ彛


鏈夌姸鎬佽棰戠紪鐮佸櫒鎸夋樉绀洪『搴忔帴鏀跺師濮嬭棰戝抚锛屽苟灏嗗叾缂栫爜涓哄瓧鑺傛祦銆傚畠鐢熸垚瀹屾暣鐨勫瓧鑺傛祦鐗囨锛屽寘鎷墍鏈夊厓鏁版嵁銆佸ご閮ㄧ瓑銆傜敓鎴愮殑瀛楄妭娴佹棤闇€瀹㈡埛绔仛浠讳綍杩涗竴姝ョ殑鍚庡鐞嗐€?

鍦ㄩ┍鍔ㄤ腑鎵ц杞欢娴佸鐞嗐€佸ご閮ㄧ敓鎴愮瓑鎿嶄綔浠ユ敮鎸佹湰鎺ュ彛锛屾槸寮虹儓涓嶅缓璁殑銆傝嫢纭疄闇€瑕佹绫绘搷浣滐紝寮虹儓寤鸿浣跨敤鏃犵姸鎬佽棰戠紪鐮佸櫒鎺ュ彛锛堝紑鍙戜腑锛夈€?

## 鏈枃妗ｄ娇鐢ㄧ殑绾﹀畾涓庤鍙?


1. 闄ら潪鏈枃妗ｅ彟鏈夎鏄庯紝V4L2 API 鐨勪竴鑸鍒欏潎閫傜敤銆?

2. 璇嶈 "must"銆?may"銆?should" 绛夌殑鍚箟閬靛惊 `RFC
   2119 <https://tools.ietf.org/html/rfc2119>`_.

3. 鎵€鏈夋湭鏍囨敞涓衡€渙ptional鈥濈殑姝ラ閮芥槸蹇呴渶鐨勩€?

4. 闄ら潪鍙︽湁璇存槑锛宍VIDIOC_G_EXT_CTRLS` 涓?`VIDIOC_S_EXT_CTRLS` 鍙垎鍒笌 `VIDIOC_G_CTRL` 鍜?`VIDIOC_S_CTRL` 浜掓崲浣跨敤銆?

5. 闄ら潪鍙︽湁璇存槑锛屾牴鎹紪鐮佸櫒鑳藉姏骞堕伒寰?V4L2 閫氱敤鍑嗗垯锛屽崟骞抽潰 API锛堣 planar-apis锛夊強閫傜敤鐨勭粨鏋勪綋鍙笌澶氬钩闈?API 浜掓崲浣跨敤銆?

6. i = [a..b]锛氫粠 a 鍒?b锛堝惈绔偣锛夌殑鏁存暟搴忓垪锛屽嵆 i = [0..2] 琛ㄧず i = 0, 1, 2銆?

7. 缁欏畾涓€涓?`OUTPUT` 缂撳啿鍖?A锛屽垯 A' 琛ㄧず `CAPTURE` 闃熷垪涓婄殑涓€涓紦鍐插尯锛屽叾涓寘鍚敱澶勭悊缂撳啿鍖?A 鎵€浜х敓鐨勬暟鎹€?

## 鏈琛?


鍙傝 decoder-glossary銆?

## 鐘舵€佹満


   :alt: DOT digraph of encoder state machine
   :caption: Encoder State Machine

   digraph encoder_state_machine {
       node [shape = doublecircle, label="Encoding"] Encoding;

       node [shape = circle, label="Initialization"] Initialization;
       node [shape = circle, label="Stopped"] Stopped;
       node [shape = circle, label="Drain"] Drain;
       node [shape = circle, label="Reset"] Reset;

       node [shape = point]; qi
       qi -> Initialization [ label = "open()" ];

       Initialization -> Encoding [ label = "Both queues streaming" ];

       Encoding -> Drain [ label = "V4L2_ENC_CMD_STOP" ];
       Encoding -> Reset [ label = "VIDIOC_STREAMOFF(CAPTURE)" ];
       Encoding -> Stopped [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];
       Encoding -> Encoding;

       Drain -> Stopped [ label = "All CAPTURE\nbuffers dequeued\nor\nVIDIOC_STREAMOFF(OUTPUT)" ];
       Drain -> Reset [ label = "VIDIOC_STREAMOFF(CAPTURE)" ];

       Reset -> Encoding [ label = "VIDIOC_STREAMON(CAPTURE)" ];
       Reset -> Initialization [ label = "VIDIOC_REQBUFS(OUTPUT, 0)" ];

       Stopped -> Encoding [ label = "V4L2_ENC_CMD_START\nor\nVIDIOC_STREAMON(OUTPUT)" ];
       Stopped -> Reset [ label = "VIDIOC_STREAMOFF(CAPTURE)" ];
   }

## 鏌ヨ鑳藉姏


1. 涓烘灇涓剧紪鐮佸櫒鏀寔鐨勪竴缁勭紪鐮佹牸寮忥紝瀹㈡埛绔彲鍦?`CAPTURE` 涓婅皟鐢?`VIDIOC_ENUM_FMT`銆?

   - 灏嗚繑鍥炲畬鏁寸殑鍙楁敮鎸佹牸寮忛泦鍚堬紝鏃犺 `OUTPUT` 涓婅缃殑鏍煎紡涓轰綍銆?

2. 涓烘灇涓惧彈鏀寔鐨勪竴缁勫師濮嬫牸寮忥紝瀹㈡埛绔彲鍦?`OUTPUT` 涓婅皟鐢?`VIDIOC_ENUM_FMT`銆?

   - 浠呰繑鍥炲綋鍓嶅湪 `CAPTURE` 涓婂浜庢椿鍔ㄧ姸鎬佺殑鏍煎紡鎵€鏀寔鐨勯偅浜涙牸寮忋€?

   - 瑕佹灇涓炬煇涓粰瀹氱紪鐮佹牸寮忔墍鏀寔鐨勫師濮嬫牸寮忥紝瀹㈡埛绔繀椤诲厛鍦?`CAPTURE` 涓婅缃缂栫爜鏍煎紡锛屽啀鍦?`OUTPUT` 涓婃灇涓炬牸寮忋€?

3. 瀹㈡埛绔彲浣跨敤 `VIDIOC_ENUM_FRAMESIZES` 鏉ユ娴嬫煇缁欏畾鏍煎紡鎵€鏀寔鐨勫垎杈ㄧ巼锛屽皢鏈熸湜鐨勫儚绱犳牸寮忛€氳繃 `v4l2_frmsizeenum` `pixel_format` 浼犲叆銆?

   - `VIDIOC_ENUM_FRAMESIZES` 閽堝缂栫爜鍍忕礌鏍煎紡杩斿洖鐨勫€硷紝灏嗗寘鍚紪鐮佸櫒瀵硅缁欏畾缂栫爜鍍忕礌鏍煎紡鏀寔鐨勬墍鏈夊彲鑳界殑缂栫爜鍒嗚鲸鐜囥€?

   - `VIDIOC_ENUM_FRAMESIZES` 閽堝鍘熷鍍忕礌鏍煎紡杩斿洖鐨勫€硷紝灏嗗寘鍚紪鐮佸櫒瀵硅缁欏畾鍘熷鍍忕礌鏍煎紡銆佷互鍙婂綋鍓嶅湪 `CAPTURE` 涓婅缃殑缂栫爜鏍煎紡鎵€鏀寔鐨勬墍鏈夊彲鑳界殑甯х紦鍐插尯鍒嗚鲸鐜囥€?

4. 瀹㈡埛绔彲浣跨敤 `VIDIOC_ENUM_FRAMEINTERVALS` 鏉ユ娴嬫煇缁欏畾鏍煎紡涓庡垎杈ㄧ巼鎵€鏀寔鐨勫抚闂撮殧锛屽皢鏈熸湜鐨勫儚绱犳牸寮忛€氳繃 `v4l2_frmivalenum` `pixel_format` 浼犲叆锛屽垎杈ㄧ巼閫氳繃 `v4l2_frmivalenum` `width` 鍜?`v4l2_frmivalenum` `height` 浼犲叆銆?

   - `VIDIOC_ENUM_FRAMEINTERVALS` 閽堝缂栫爜鍍忕礌鏍煎紡涓庣紪鐮佸垎杈ㄧ巼杩斿洖鐨勫€硷紝灏嗗寘鍚紪鐮佸櫒瀵硅缁欏畾缂栫爜鍍忕礌鏍煎紡涓庡垎杈ㄧ巼鎵€鏀寔鐨勬墍鏈夊彲鑳界殑甯ч棿闅斻€?

   - `VIDIOC_ENUM_FRAMEINTERVALS` 閽堝鍘熷鍍忕礌鏍煎紡涓庡垎杈ㄧ巼杩斿洖鐨勫€硷紝灏嗗寘鍚紪鐮佸櫒瀵硅缁欏畾鍘熷鍍忕礌鏍煎紡涓庡垎杈ㄧ巼銆佷互鍙婂綋鍓嶅湪 `CAPTURE` 涓婅缃殑缂栫爜鏍煎紡銆佺紪鐮佸垎杈ㄧ巼鍜岀紪鐮佸抚闂撮殧鎵€鏀寔鐨勬墍鏈夊彲鑳界殑甯ч棿闅斻€?

   - 瀵?`VIDIOC_ENUM_FRAMEINTERVALS` 鐨勬敮鎸佹槸鍙€夌殑銆傝嫢鏈疄鐜帮紝鍒欓櫎缂栬В鐮佸櫒鏈韩鐨勯檺鍒跺锛屾病鏈夊叾浠栫壒娈婇檺鍒躲€?

5. 瀵逛簬褰撳墠鍦?`CAPTURE` 涓婅缃殑缂栫爜鏍煎紡锛岃嫢閫傜敤锛屽叾鎵€鏀寔鐨勬。娆★紙profile锛変笌绾у埆锛坙evel锛夊彲閫氳繃鍚勮嚜瀵瑰簲鐨勬帶浠剁粡 `VIDIOC_QUERYCTRL` 鏌ヨ銆?

6. 浠讳綍鍏朵粬缂栫爜鍣ㄨ兘鍔涘潎鍙€氳繃鏌ヨ鍏跺悇鑷搴旂殑鎺т欢鏉ュ彂鐜般€?

## 鍒濆鍖?


1. 閫氳繃 `VIDIOC_S_FMT` 鍦?`CAPTURE` 闃熷垪涓婅缃紪鐮佹牸寮忋€?

   - **蹇呴渶瀛楁锛?*

     `type`
         涓€涓€傜敤浜?`CAPTURE` 鐨?`V4L2_BUF_TYPE_*` 鏋氫妇銆?

     `pixelformat`
         瑕佺敓鎴愮殑缂栫爜鏍煎紡銆?

     `sizeimage`
         `CAPTURE` 缂撳啿鍖虹殑鏈熸湜澶у皬锛涚紪鐮佸櫒鍙兘浼氬鍏惰繘琛岃皟鏁翠互鍖归厤纭欢瑕佹眰銆?

     `width`, `height`
         蹇界暐锛堝彧璇伙級銆?

     other fields
         閬靛惊鏍囧噯璇箟銆?

   - **杩斿洖瀛楁锛?*

     `sizeimage`
         缁忚皟鏁寸殑 `CAPTURE` 缂撳啿鍖哄ぇ灏忋€?

     `width`, `height`
         鐢辩紪鐮佸櫒鏍规嵁褰撳墠鐘舵€侊紙渚嬪 `OUTPUT` 鏍煎紡銆侀€夋嫨鐭╁舰绛夛級閫夊畾鐨勭紪鐮佸昂瀵革紙鍙锛夈€?

```

      Changing the ``CAPTURE`` format may change the currently set ``OUTPUT``
      format. How the new ``OUTPUT`` format is determined is up to the encoder
      and the client must ensure it matches its needs afterwards.

```
2. **鍙€夈€?* 閫氳繃 `VIDIOC_ENUM_FMT` 鏋氫妇鎵€閫夌紪鐮佹牸寮忔敮鎸佺殑 `OUTPUT` 鏍煎紡锛堟簮鐨勫師濮嬫牸寮忥級銆?

   - **蹇呴渶瀛楁锛?*

     `type`
         涓€涓€傜敤浜?`OUTPUT` 鐨?`V4L2_BUF_TYPE_*` 鏋氫妇銆?

     other fields
         閬靛惊鏍囧噯璇箟銆?

   - **杩斿洖瀛楁锛?*

     `pixelformat`
         褰撳墠鍦?`CAPTURE` 闃熷垪涓婃墍閫夌紪鐮佹牸寮忔墍鏀寔鐨勫師濮嬫牸寮忋€?

     other fields
         閬靛惊鏍囧噯璇箟銆?

3. 閫氳繃 `VIDIOC_S_FMT` 鍦?`OUTPUT` 闃熷垪涓婅缃師濮嬫簮鏍煎紡銆?

   - **蹇呴渶瀛楁锛?*

     `type`
         涓€涓€傜敤浜?`OUTPUT` 鐨?`V4L2_BUF_TYPE_*` 鏋氫妇銆?

     `pixelformat`
         婧愮殑鍘熷鏍煎紡銆?

     `width`, `height`
         婧愬垎杈ㄧ巼銆?

     other fields
         閬靛惊鏍囧噯璇箟銆?

   - **杩斿洖瀛楁锛?*

     `width`, `height`
         鍙兘浼氳璋冩暣锛屼互鍖归厤褰撳墠鎵€閫夋牸寮忥紙濡?`VIDIOC_ENUM_FRAMESIZES` 鎵€鎶ュ憡锛夎姹傜殑缂栫爜鍣ㄦ渶灏忓€笺€佹渶澶у€煎拰瀵归綈瑕佹眰銆?

     other fields
         閬靛惊鏍囧噯璇箟銆?

   - 璁剧疆 `OUTPUT` 鏍煎紡浼氭牴鎹柊鍒嗚鲸鐜囧皢閫夋嫨鐭╁舰閲嶇疆涓洪粯璁ゅ€硷紝濡備笅涓€姝ユ墍杩般€?

4. 閫氳繃 `VIDIOC_S_PARM` 鍦?`OUTPUT` 闃熷垪涓婅缃師濮嬪抚闂撮殧銆傝繖鍚屾椂浼氬皢 `CAPTURE` 闃熷垪涓婄殑缂栫爜甯ч棿闅旇涓虹浉鍚岀殑鍊笺€?

   - **蹇呴渶瀛楁锛?*

     `type`
	 涓€涓€傜敤浜?`OUTPUT` 鐨?`V4L2_BUF_TYPE_*` 鏋氫妇銆?

     `parm.output`
	 闄?`parm.output.timeperframe` 澶栨墍鏈夊瓧娈佃涓?0銆?

     `parm.output.timeperframe`
	 鏈熸湜鐨勫抚闂撮殧锛涚紪鐮佸櫒鍙兘浼氬鍏惰繘琛岃皟鏁翠互鍖归厤纭欢瑕佹眰銆?

   - **杩斿洖瀛楁锛?*

     `parm.output.timeperframe`
	 缁忚皟鏁寸殑甯ч棿闅斻€?

```

      Changing the ``OUTPUT`` frame interval *also* sets the framerate that
      the encoder uses to encode the video. So setting the frame interval
      to 1/24 (or 24 frames per second) will produce a coded video stream
      that can be played back at that speed. The frame interval for the
      ``OUTPUT`` queue is just a hint, the application may provide raw
      frames at a different rate. It can be used by the driver to help
      schedule multiple encoders running in parallel.

      In the next step the ``CAPTURE`` frame interval can optionally be
      changed to a different value. This is useful for off-line encoding
      were the coded frame interval can be different from the rate at
      which raw frames are supplied.

   .. important::

      ``timeperframe`` deals with *frames*, not fields. So for interlaced
      formats this is the time per two fields, since a frame consists of
      a top and a bottom field.

   .. note::

      It is due to historical reasons that changing the ``OUTPUT`` frame
      interval also changes the coded frame interval on the ``CAPTURE``
      queue. Ideally these would be independent settings, but that would
      break the existing API.

```
5. **鍙€?* 閫氳繃 `VIDIOC_S_PARM` 鍦?`CAPTURE` 闃熷垪涓婅缃紪鐮佸抚闂撮殧銆備粎褰撶紪鐮佸抚闂撮殧涓庡師濮嬪抚闂撮殧涓嶅悓鏃舵墠闇€瑕佹姝ラ锛岀绾跨紪鐮侀€氬父灏卞睘浜庤繖绉嶆儏鍐点€傝鐗规€х殑鏀寔鐢?V4L2_FMT_FLAG_ENC_CAP_FRAME_INTERVAL <fmtdesc-flags> 鏍煎紡鏍囧織鏉ユ爣绀恒€?

   - **蹇呴渶瀛楁锛?*

     `type`
	 涓€涓€傜敤浜?`CAPTURE` 鐨?`V4L2_BUF_TYPE_*` 鏋氫妇銆?

     `parm.capture`
	 闄?`parm.capture.timeperframe` 澶栨墍鏈夊瓧娈佃涓?0銆?

     `parm.capture.timeperframe`
	 鏈熸湜鐨勭紪鐮佸抚闂撮殧锛涚紪鐮佸櫒鍙兘浼氬鍏惰繘琛岃皟鏁翠互鍖归厤纭欢瑕佹眰銆?

   - **杩斿洖瀛楁锛?*

     `parm.capture.timeperframe`
	 缁忚皟鏁寸殑甯ч棿闅斻€?

```

      Changing the ``CAPTURE`` frame interval sets the framerate for the
      coded video. It does *not* set the rate at which buffers arrive on the
      ``CAPTURE`` queue, that depends on how fast the encoder is and how
      fast raw frames are queued on the ``OUTPUT`` queue.

   .. important::

      ``timeperframe`` deals with *frames*, not fields. So for interlaced
      formats this is the time per two fields, since a frame consists of
      a top and a bottom field.

   .. note::

      Not all drivers support this functionality, in that case just set
      the desired coded frame interval for the ``OUTPUT`` queue.

      However, drivers that can schedule multiple encoders based on the
      ``OUTPUT`` frame interval must support this optional feature.

```
6. **鍙€夈€?* 鑻ュ笇鏈涙祦鍏冩暟鎹殑鍙鍒嗚鲸鐜囦笉鍚屼簬瀹屾暣鐨?OUTPUT 鍒嗚鲸鐜囷紝鍙€氳繃 `VIDIOC_S_SELECTION` 鍦?`OUTPUT` 闃熷垪涓婅缃彲瑙佸垎杈ㄧ巼銆?

   - **蹇呴渶瀛楁锛?*

     `type`
         涓€涓€傜敤浜?`OUTPUT` 鐨?`V4L2_BUF_TYPE_*` 鏋氫妇銆?

     `target`
         璁句负 `V4L2_SEL_TGT_CROP`銆?

     `r.left`, `r.top`, `r.width`, `r.height`
         鍙鐭╁舰锛涘畠蹇呴』钀藉湪 `V4L2_SEL_TGT_CROP_BOUNDS` 鐭╁舰涔嬪唴锛屽苟鍙兘琚皟鏁翠互绗﹀悎缂栬В鐮佸櫒鍜岀‖浠剁害鏉熴€?

   - **杩斿洖瀛楁锛?*

     `r.left`, `r.top`, `r.width`, `r.height`
         缁忕紪鐮佸櫒璋冩暣鐨勫彲瑙佺煩褰€?

   - 鍦?`OUTPUT` 涓婃敮鎸佷互涓嬮€夋嫨鐩爣锛?

     `V4L2_SEL_TGT_CROP_BOUNDS`
         绛変簬瀹屾暣鐨勬簮甯э紝涓庢椿鍔ㄧ殑 `OUTPUT` 鏍煎紡涓€鑷淬€?

     `V4L2_SEL_TGT_CROP_DEFAULT`
         绛変簬 `V4L2_SEL_TGT_CROP_BOUNDS`銆?

     `V4L2_SEL_TGT_CROP`
         婧愮紦鍐插尯涓皢琚紪鐮佽繘 `CAPTURE` 娴佺殑鐭╁舰锛涢粯璁や负 `V4L2_SEL_TGT_CROP_DEFAULT`銆?

```

            A common use case for this selection target is encoding a source
            video with a resolution that is not a multiple of a macroblock,
            e.g.  the common 1920x1080 resolution may require the source
            buffers to be aligned to 1920x1088 for codecs with 16x16 macroblock
            size. To avoid encoding the padding, the client needs to explicitly
            configure this selection target to 1920x1080.

   .. warning::

      The encoder may adjust the crop/compose rectangles to the nearest
      supported ones to meet codec and hardware requirements. The client needs
      to check the adjusted rectangle returned by :c:func:`VIDIOC_S_SELECTION`.

```
7. 閫氳繃 `VIDIOC_REQBUFS` 涓?`OUTPUT` 鍜?`CAPTURE` 鍒嗛厤缂撳啿鍖恒€傚彲浠ヤ互浠绘剰椤哄簭鎵ц銆?

   - **蹇呴渶瀛楁锛?*

     `count`
         璇锋眰鍒嗛厤鐨勭紦鍐插尯鏁伴噺锛涘繀椤诲ぇ浜庨浂銆?

     `type`
         涓€涓€傜敤浜?`OUTPUT` 鎴?`CAPTURE` 鐨?`V4L2_BUF_TYPE_*` 鏋氫妇銆?

     other fields
         閬靛惊鏍囧噯璇箟銆?

   - **杩斿洖瀛楁锛?*

     `count`
         瀹為檯鍒嗛厤鐨勭紦鍐插尯鏁伴噺銆?

```

      The actual number of allocated buffers may differ from the ``count``
      given. The client must check the updated value of ``count`` after the
      call returns.

   .. note::

      To allocate more than the minimum number of OUTPUT buffers (for pipeline
      depth), the client may query the ``V4L2_CID_MIN_BUFFERS_FOR_OUTPUT``
      control to get the minimum number of buffers required, and pass the
      obtained value plus the number of additional buffers needed in the
      ``count`` field to :c:func:`VIDIOC_REQBUFS`.

   Alternatively, :c:func:`VIDIOC_CREATE_BUFS` can be used to have more
   control over buffer allocation.

   * **Required fields:**

     ``count``
         requested number of buffers to allocate; greater than zero.

     ``type``
         a ``V4L2_BUF_TYPE_*`` enum appropriate for ``OUTPUT``.

     other fields
         follow standard semantics.

   * **Returned fields:**

     ``count``
         adjusted to the number of allocated buffers.

```
8. 閫氳繃 `VIDIOC_STREAMON` 鍦?`OUTPUT` 鍜?`CAPTURE` 涓や釜闃熷垪涓婂紑濮嬫暟鎹祦銆傚彲浠ヤ互浠绘剰椤哄簭鎵ц銆傚綋涓や釜闃熷垪閮藉紑濮嬫暟鎹祦鏃讹紝瀹為檯鐨勭紪鐮佽繃绋嬫墠寮€濮嬨€?


   鑻ュ鎴风鍦ㄧ紪鐮佽繃绋嬩腑鍋滄 `CAPTURE` 闃熷垪锛岄殢鍚庡張閲嶆柊鍚姩瀹冿紝缂栫爜鍣ㄥ皢寮€濮嬬敓鎴愪竴鏉′笌鍋滄鍓嶆墍鐢熸垚娴佺浉浜掔嫭绔嬬殑娴併€傚叿浣撶殑绾︽潫鍙栧喅浜庣紪鐮佹牸寮忥紝浣嗗彲鑳藉寘鎷互涓嬪悗鏋滐細

   - 閲嶅惎鍚庣敓鎴愮殑缂栫爜甯т笉寰楀紩鐢ㄥ仠姝㈠墠鐢熸垚鐨勪换浣曞抚锛屼緥濡?H.264/HEVC 涓笉鍏佽闀挎湡鍙傝€冿紝

   - 浠讳綍蹇呴』鍖呭惈鍦ㄧ嫭绔嬫祦涓殑澶撮儴閮藉繀椤婚噸鏂扮敓鎴愶紝渚嬪 H.264/HEVC 鐨?SPS 鍜?PPS銆?

## 缂栫爜


鍦?`Initialization` 搴忓垪鎴愬姛瀹屾垚鍚庤繘鍏ユ鐘舵€併€傚湪姝ょ姸鎬佷笅锛屽鎴风閫氳繃 `VIDIOC_QBUF` 鍜?`VIDIOC_DQBUF` 鍚戜袱涓槦鍒楀叆闃熷拰鍑洪槦缂撳啿鍖猴紝閬靛惊鏍囧噯璇箟銆?

缂栫爜鍚?`CAPTURE` 缂撳啿鍖虹殑鍐呭鍙栧喅浜庢椿鍔ㄧ殑缂栫爜鍍忕礌鏍煎紡锛屽苟鍙兘鍙楀悇鏍煎紡鏂囨。涓墍杩扮殑缂栬В鐮佸櫒鐗瑰畾鎵╁睍鎺т欢褰卞搷銆?

涓や釜闃熷垪鐙珛杩愯锛岄伒寰?V4L2 缂撳啿鍖洪槦鍒椾笌鍐呭瓨鍒板唴瀛樿澶囩殑鏍囧噯琛屼负銆傛澶栵紝鐢变簬鎵€閫夌紪鐮佹牸寮忕殑鐗规€э紙渚嬪甯ч噸鎺掑簭锛夛紝浠?`CAPTURE` 闃熷垪鍑洪槦鐨勭紪鐮佸抚椤哄簭锛屽彲鑳戒笌鍚?`OUTPUT` 闃熷垪鍏ラ槦鍘熷甯х殑椤哄簭涓嶅悓銆?

瀹㈡埛绔笉寰楀亣瀹?`CAPTURE` 涓?`OUTPUT` 缂撳啿鍖轰箣闂村瓨鍦ㄤ换浣曠洿鎺ュ叧绯伙紝涔熶笉寰楀亣瀹氱紦鍐插尯鍙樹负鍙嚭闃熺殑鍏蜂綋鏃舵満銆傚叿浣撹€岃█锛?

- 鍏ラ槦鍒?`OUTPUT` 鐨勭紦鍐插尯鍙兘鍦?`CAPTURE` 涓婁骇鐢熷浜庝竴涓紦鍐插尯锛堜緥濡傦紝鑻ヨ繑鍥炰竴涓紪鐮佸抚浣跨紪鐮佸櫒寰椾互杩斿洖涓€涓湪鏄剧ず椤哄簭涓綅浜庡叾涔嬪墠銆佷絾鍦ㄨВ鐮侀『搴忎腑浣嶄簬鍏朵箣鍚庣殑甯э紱褰撶劧涔熷彲鑳芥湁鍏朵粬鍘熷洜锛夛紝

- 鍏ラ槦鍒?`OUTPUT` 鐨勭紦鍐插尯鍙兘鍦ㄧ紪鐮佽繃绋嬬殑鏇存櫄闃舵銆佸拰/鎴栧湪澶勭悊浜嗘洿澶?`OUTPUT` 缂撳啿鍖轰箣鍚庯紝鎵嶅湪 `CAPTURE` 涓婁骇鐢熶竴涓紦鍐插尯锛屾垨鑰呬贡搴忚繑鍥烇紙渚嬪鍦ㄤ娇鐢ㄦ樉绀洪噸鎺掑簭鏃讹級锛?

- 鍗充娇娌℃湁棰濆缂撳啿鍖哄叆闃熷埌 `OUTPUT`锛宍CAPTURE` 闃熷垪涓婁篃鍙兘浼氭湁缂撳啿鍖哄彉涓哄彲鐢紙渚嬪鍦?drain 鎴?`EOS` 鏈熼棿锛夛紝杩欐槸鍥犱负杩囧幓鍏ラ槦鐨?`OUTPUT` 缂撳啿鍖猴紝鍏剁紪鐮佺粨鏋滅敱浜庣紪鐮佽繃绋嬬殑鐗规€ц€岃鍒版洿鏅氱殑鏃跺埢鎵嶅彲鐢紝

- 鍏ラ槦鍒?`OUTPUT` 鐨勭紦鍐插尯鍦ㄧ紪鐮佽繘鐩稿簲鐨?`CAPTURE` 缂撳啿鍖哄悗锛屽彲鑳戒笉浼氱珛鍗冲彉涓哄彲鍑洪槦锛屼緥濡傚綋缂栫爜鍣ㄩ渶瑕佸皢璇ュ抚鐢ㄤ綔缂栫爜鍚庣画甯х殑鍙傝€冩椂銆?


   涓轰娇缂栫爜鍚庣殑 `CAPTURE` 缂撳啿鍖鸿兘涓庡叾鏉ユ簮 `OUTPUT` 缂撳啿鍖虹浉鍖归厤锛屽鎴风鍙湪鍏ラ槦涓€涓?`OUTPUT` 缂撳啿鍖烘椂锛岃缃?`v4l2_buffer` 缁撴瀯浣撶殑 `timestamp` 瀛楁銆傜敱缂栫爜璇?`OUTPUT` 缂撳啿鍖烘墍浜х敓鐨?`CAPTURE` 缂撳啿鍖猴紝鍦ㄥ嚭闃熸椂鍏?`timestamp` 瀛楁浼氳璁句负鐩稿悓鐨勫€笺€?

   闄や竴涓?`OUTPUT` 缂撳啿鍖轰骇鐢熶竴涓?`CAPTURE` 缂撳啿鍖虹殑绠€鍗曟儏鍐靛锛岃繕瀹氫箟浜嗕互涓嬫儏鍐碉細

   - 涓€涓?`OUTPUT` 缂撳啿鍖虹敓鎴愬涓?`CAPTURE` 缂撳啿鍖猴細鐩稿悓鐨?`OUTPUT` 鏃堕棿鎴充細琚鍒跺埌澶氫釜 `CAPTURE` 缂撳啿鍖猴紝

   - 缂栫爜椤哄簭涓庡憟鐜伴『搴忎笉鍚岋紙鍗?`CAPTURE` 缂撳啿鍖虹浉瀵逛簬 `OUTPUT` 缂撳啿鍖烘槸涔卞簭鐨勶級锛歚CAPTURE` 鏃堕棿鎴充笉浼氫繚鎸?`OUTPUT` 鏃堕棿鎴崇殑椤哄簭銆?


   涓鸿瀹㈡埛绔尯鍒嗗抚绫诲瀷锛堝叧閿抚銆佷腑闂村抚锛涚‘鍒囩殑绫诲瀷鍒楄〃鍙栧喅浜庣紪鐮佹牸寮忥級锛宍CAPTURE` 缂撳啿鍖哄湪鍑洪槦鏃讹紝鍏?`v4l2_buffer` 缁撴瀯浣撲腑浼氳缃浉搴旂殑鏍囧織浣嶃€傜‘鍒囩殑鏍囧織鍒楄〃鍙婂叾鍚箟锛岃鍙傞槄 `v4l2_buffer` 浠ュ強鍚勭紪鐮佸儚绱犳牸寮忕殑鏂囨。銆?

鑻ュ彂鐢熺紪鐮侀敊璇紝灏嗕緷鎹紪鐮佸櫒鐨勮兘鍔涳紝浠ョ浉搴旂殑璇︾粏绋嬪害鎶ュ憡缁欏鎴风銆傚叿浣撹€岃█锛?

- 鍖呭惈澶辫触缂栫爜鎿嶄綔缁撴灉鐨?`CAPTURE` 缂撳啿鍖猴紙鑻ユ湁锛夊皢浠ヨ缃?`V4L2_BUF_FLAG_ERROR` 鏍囧織鐨勭姸鎬佽繑鍥烇紝

- 鑻ョ紪鐮佸櫒鑳藉绮剧‘鎶ュ憡瑙﹀彂閿欒鐨?`OUTPUT` 缂撳啿鍖猴紝鍒欐绫荤紦鍐插尯灏嗕互璁剧疆 `V4L2_BUF_FLAG_ERROR` 鏍囧織鐨勭姸鎬佽繑鍥炪€?


   鑻?`CAPTURE` 缂撳啿鍖鸿繃灏忥紝鍒欏畠浠呬互璁剧疆 `V4L2_BUF_FLAG_ERROR` 鏍囧織鐨勭姸鎬佽繑鍥炪€傝繕闇€瑕佸仛鏇村宸ヤ綔鏉ユ娴嬧€滅紦鍐插尯杩囧皬鈥濊繖涓€閿欒鍘熷洜锛屽苟鎻愪緵閲婃斁杩囧皬缂撳啿鍖虹殑鏀寔銆?

濡傛灉鍙戠敓涓嶅厑璁哥紪鐮佺户缁殑鑷村懡澶辫触锛屽璇ョ紪鐮佸櫒鏂囦欢鍙ユ焺鐨勪换浣曡繘涓€姝ユ搷浣滈兘灏嗚繑鍥?-EIO 閿欒鐮併€傚鎴风鍙互鍏抽棴璇ユ枃浠跺彞鏌勫苟鎵撳紑涓€涓柊鐨勶紝鎴栬€呴€氳繃鍋滄涓や釜闃熷垪鐨勬暟鎹祦銆侀噴鏀炬墍鏈夌紦鍐插尯骞堕噸鏂版墽琛屽垵濮嬪寲搴忓垪鏉ラ噸鏂板垵濮嬪寲璇ュ疄渚嬨€?

## 缂栫爜鍙傛暟鏇存敼


瀹㈡埛绔彲闅忔椂浣跨敤 `VIDIOC_S_CTRL` 鏉ユ洿鏀圭紪鐮佸櫒鍙傛暟銆傚弬鏁扮殑鍙敤鎬у洜缂栫爜鍣ㄨ€屽紓锛屽鎴风蹇呴』鏌ヨ缂栫爜鍣ㄤ互纭畾鍙敤鎺т欢闆嗗悎銆?

鑳藉惁鍦ㄧ紪鐮佽繃绋嬩腑鏇存敼姣忎釜鍙傛暟鍥犵紪鐮佸櫒鑰屽紓锛岄伒寰?V4L2 鎺т欢鎺ュ彛鐨勬爣鍑嗚涔夈€傚鎴风鍙互灏濊瘯鍦ㄧ紪鐮佽繃绋嬩腑璁剧疆鎺т欢锛岃嫢鎿嶄綔浠?-EBUSY 閿欒鐮佸け璐ワ紝鍒欓渶瑕佸仠姝?`CAPTURE` 闃熷垪鎵嶅厑璁告洿鏀归厤缃€備负姝わ紝瀹冨彲浠ラ伒寰?`Drain` 搴忓垪锛屼互閬垮厤涓㈠け宸插叆闃?宸茬紪鐮佺殑甯с€?

鍙傛暟鏇存柊鐨勬椂鏈哄洜缂栫爜鍣ㄨ€屽紓锛岄伒寰?V4L2 鎺т欢鎺ュ彛鐨勬爣鍑嗚涔夈€傝嫢瀹㈡埛绔渶瑕佸湪鐗瑰畾甯х簿纭簲鐢ㄥ弬鏁帮紝搴旇€冭檻浣跨敤 Request API锛坢edia-request-api锛夛紝鍓嶆彁鏄紪鐮佸櫒鏀寔銆?

## 鎺掔┖锛圖rain锛?


涓虹‘淇濇墍鏈夊凡鍏ラ槦鐨?`OUTPUT` 缂撳啿鍖洪兘宸茶澶勭悊锛屼笖鐩稿叧鐨?`CAPTURE` 缂撳啿鍖哄凡浜や粯缁欏鎴风锛屽鎴风蹇呴』閬靛惊涓嬭堪 drain 搴忓垪銆俤rain 搴忓垪缁撴潫鍚庯紝瀹㈡埛绔凡鏀跺埌鍦ㄨ搴忓垪鍚姩鍓嶅叆闃熺殑鎵€鏈?`OUTPUT` 缂撳啿鍖虹殑鍏ㄩ儴缂栫爜甯с€?

1. 閫氳繃鍙戝嚭 `VIDIOC_ENCODER_CMD` 寮€濮?drain 搴忓垪銆?

   - **蹇呴渶瀛楁锛?*

     `cmd`
         璁句负 `V4L2_ENC_CMD_STOP`銆?

     `flags`
         璁句负 0銆?

     `pts`
         璁句负 0銆?

```

      The sequence can be only initiated if both ``OUTPUT`` and ``CAPTURE``
      queues are streaming. For compatibility reasons, the call to
      :c:func:`VIDIOC_ENCODER_CMD` will not fail even if any of the queues is
      not streaming, but at the same time it will not initiate the `Drain`
      sequence and so the steps described below would not be applicable.

```
2. 鍦ㄥ彂鍑?`VIDIOC_ENCODER_CMD` 涔嬪墠瀹㈡埛绔叆闃熺殑浠讳綍 `OUTPUT` 缂撳啿鍖猴紝閮藉皢鐓у父澶勭悊鍜岀紪鐮併€傚鎴风蹇呴』缁х画鐙珛澶勭悊涓や釜闃熷垪锛岀被浼间簬姝ｅ父鐨勭紪鐮佹搷浣溿€傝繖鍖呮嫭锛?

   - 鍏ラ槦鍜屽嚭闃?`CAPTURE` 缂撳啿鍖猴紝鐩村埌鍑洪槦涓€涓甫鏈?`V4L2_BUF_FLAG_LAST` 鏍囧織鐨勭紦鍐插尯锛?

```

        The last buffer may be empty (with :c:type:`v4l2_buffer`
        ``bytesused`` = 0) and in that case it must be ignored by the client,
        as it does not contain an encoded frame.

     .. note::

        Any attempt to dequeue more ``CAPTURE`` buffers beyond the buffer
        marked with ``V4L2_BUF_FLAG_LAST`` will result in a -EPIPE error from
        :c:func:`VIDIOC_DQBUF`.

   * dequeuing processed ``OUTPUT`` buffers, until all the buffers queued
     before the ``V4L2_ENC_CMD_STOP`` command are dequeued,

   * dequeuing the ``V4L2_EVENT_EOS`` event, if the client subscribes to it.

   .. note::

      For backwards compatibility, the encoder will signal a ``V4L2_EVENT_EOS``
      event when the last frame has been encoded and all frames are ready to be
      dequeued. It is deprecated behavior and the client must not rely on it.
      The ``V4L2_BUF_FLAG_LAST`` buffer flag should be used instead.

```
3. 涓€鏃﹀湪 `V4L2_ENC_CMD_STOP` 璋冪敤涔嬪墠鍏ラ槦鐨勬墍鏈?`OUTPUT` 缂撳啿鍖洪兘宸插嚭闃燂紝涓旀渶鍚庝竴涓?`CAPTURE` 缂撳啿鍖轰篃宸插嚭闃燂紝缂栫爜鍣ㄥ嵆鍋滄锛涙鍚庡畠灏嗘帴鍙椼€佷絾涓嶅啀澶勭悊浠讳綍鏂板叆闃熺殑 `OUTPUT` 缂撳啿鍖猴紝鐩村埌瀹㈡埛绔彂鍑轰互涓嬩换涓€鎿嶄綔锛?

   - `V4L2_ENC_CMD_START` 鈥斺€?缂栫爜鍣ㄤ笉浼氳閲嶇疆锛屽皢甯︾潃 drain 涔嬪墠鐨勬墍鏈夌姸鎬佹仮澶嶆甯告搷浣滐紝

   - 鍦?`CAPTURE` 闃熷垪涓婄殑涓€瀵?`VIDIOC_STREAMOFF` 鍜?`VIDIOC_STREAMON` 鈥斺€?缂栫爜鍣ㄥ皢琚噸缃紙瑙?`Reset` 搴忓垪锛夛紝鐒跺悗鎭㈠缂栫爜锛?

   - 鍦?`OUTPUT` 闃熷垪涓婄殑涓€瀵?`VIDIOC_STREAMOFF` 鍜?`VIDIOC_STREAMON` 鈥斺€?缂栫爜鍣ㄥ皢鎭㈠姝ｅ父鎿嶄綔锛屼絾鍦?`V4L2_ENC_CMD_STOP` 涓?`VIDIOC_STREAMOFF` 涔嬮棿鍏ラ槦鍒?`OUTPUT` 闃熷垪鐨勪换浣曟簮甯ч兘灏嗚涓㈠純銆?


   涓€鏃﹀惎鍔ㄤ簡 drain 搴忓垪锛屽鎴风灏遍渶瑕佹寜涓婅堪姝ラ灏嗗叾鎺ㄨ繘鑷冲畬鎴愶紝闄ら潪瀹冮€氳繃鍦ㄤ换浣?`OUTPUT` 鎴?`CAPTURE` 闃熷垪涓婂彂鍑?`VIDIOC_STREAMOFF` 鏉ヤ腑姝㈣杩囩▼銆傚湪 drain 搴忓垪杩涜鏈熼棿锛屽鎴风涓嶅緱鍐嶆鍙戝嚭 `V4L2_ENC_CMD_START` 鎴?`V4L2_ENC_CMD_STOP`锛屽惁鍒欏皾璇曟椂灏嗗け璐ュ苟杩斿洖 -EBUSY 閿欒鐮併€?

   浣滀负鍙傝€冿紝涓嬮潰鎻忚堪浜嗗悇绉嶈竟鐣屾儏鍐电殑澶勭悊锛?

   - 鑻ュ湪鍙戝嚭 `V4L2_ENC_CMD_STOP` 鍛戒护鏃?`OUTPUT` 闃熷垪涓病鏈夌紦鍐插尯锛屽垯 drain 搴忓垪绔嬪嵆瀹屾垚锛岀紪鐮佸櫒杩斿洖涓€涓甫鏈?`V4L2_BUF_FLAG_LAST` 鏍囧織鐨勭┖ `CAPTURE` 缂撳啿鍖恒€?

   - 鑻ュ湪 drain 搴忓垪瀹屾垚鏃?`CAPTURE` 闃熷垪涓病鏈夌紦鍐插尯锛屽垯涓嬫瀹㈡埛绔叆闃熶竴涓?`CAPTURE` 缂撳啿鍖烘椂锛屽畠浼氱珛鍗充綔涓轰竴涓甫鏈?`V4L2_BUF_FLAG_LAST` 鏍囧織鐨勭┖缂撳啿鍖鸿繑鍥炪€?

   - 鑻ュ湪 drain 搴忓垪杩涜鏈熼棿鍦?`CAPTURE` 闃熷垪涓婅皟鐢?`VIDIOC_STREAMOFF`锛屽垯 drain 搴忓垪琚彇娑堬紝鎵€鏈?`CAPTURE` 缂撳啿鍖鸿闅愬紡杩斿洖缁欏鎴风銆?

   - 鑻ュ湪 drain 搴忓垪杩涜鏈熼棿鍦?`OUTPUT` 闃熷垪涓婅皟鐢?`VIDIOC_STREAMOFF`锛屽垯 drain 搴忓垪绔嬪嵆瀹屾垚锛屼笅涓€涓?`CAPTURE` 缂撳啿鍖哄皢浣滀负甯︽湁 `V4L2_BUF_FLAG_LAST` 鏍囧織鐨勭┖缂撳啿鍖鸿繑鍥炪€?

   灏界涓嶆槸寮哄埗瑕佹眰锛屼絾鍙互浣跨敤 `VIDIOC_TRY_ENCODER_CMD` 鏌ヨ缂栫爜鍣ㄥ懡浠ょ殑鍙敤鎬с€?

## 閲嶇疆


瀹㈡埛绔彲鑳藉笇鏈涜姹傜紪鐮佸櫒閲嶆柊鍒濆鍖栫紪鐮侊紝浣垮緱鍚庣画鐨勬祦鏁版嵁鐙珛浜庝箣鍓嶇敓鎴愮殑娴佹暟鎹€傛牴鎹紪鐮佹牸寮忕殑涓嶅悓锛岃繖鍙兘鎰忓懗鐫€锛?

- 閲嶅惎鍚庣敓鎴愮殑缂栫爜甯т笉寰楀紩鐢ㄥ仠姝㈠墠鐢熸垚鐨勪换浣曞抚锛屼緥濡?H.264/HEVC 涓笉鍏佽闀挎湡鍙傝€冿紝

- 浠讳綍蹇呴』鍖呭惈鍦ㄧ嫭绔嬫祦涓殑澶撮儴閮藉繀椤婚噸鏂扮敓鎴愶紝渚嬪 H.264/HEVC 鐨?SPS 鍜?PPS銆?

杩欏彲浠ラ€氳繃鎵ц閲嶇疆搴忓垪鏉ュ疄鐜般€?

1. 鎵ц `Drain` 搴忓垪锛屼互纭繚鎵€鏈夊湪閫旂紪鐮侀兘宸插畬鎴愪笖鐩稿簲缂撳啿鍖洪兘宸插嚭闃熴€?

2. 閫氳繃 `VIDIOC_STREAMOFF` 鍋滄 `CAPTURE` 闃熷垪涓婄殑鏁版嵁娴併€傝繖灏嗘妸鎵€鏈夊綋鍓嶅凡鍏ラ槦鐨?`CAPTURE` 缂撳啿鍖鸿繑鍥炵粰瀹㈡埛绔紝涓斾笉鍚湁鏁堝抚鏁版嵁銆?

3. 閫氳繃 `VIDIOC_STREAMON` 鍦?`CAPTURE` 闃熷垪涓婂惎鍔ㄦ暟鎹祦锛屽苟缁х画杩涜甯歌缂栫爜搴忓垪銆備粠姝ゅ埢璧凤紝鐢熸垚鍒?`CAPTURE` 缂撳啿鍖轰腑鐨勭紪鐮佸抚灏嗗寘鍚竴鏉＄嫭绔嬫祦锛屾棤闇€閲嶇疆搴忓垪涔嬪墠缂栫爜鐨勫抚鍗冲彲瑙ｇ爜锛涜鐙珛娴佸浜庡湪鍙戝嚭 `Drain` 搴忓垪鐨?`V4L2_ENC_CMD_STOP` 涔嬪悗鍏ラ槦鐨勭涓€涓?`OUTPUT` 缂撳啿鍖恒€?

璇ュ簭鍒椾篃鍙敤浜庝负閭ｄ簺鏃犳硶鍦ㄨ繍琛屼腑鏇存敼鍙傛暟鐨勭紪鐮佸櫒鏇存敼缂栫爜鍙傛暟銆?

## 鎻愪氦鐐?


璁剧疆鏍煎紡鍜屽垎閰嶇紦鍐插尯浼氳Е鍙戠紪鐮佸櫒琛屼负鐨勬敼鍙樸€?

1. 鍦?`CAPTURE` 闃熷垪涓婅缃牸寮忥紝鍙兘浼氭敼鍙?`OUTPUT` 闃熷垪涓婃敮鎸?閫氬憡鐨勬牸寮忛泦鍚堛€傜壒鍒湴锛岃繖涔熸剰鍛崇潃 `OUTPUT` 鏍煎紡鍙兘浼氳閲嶇疆锛屽鎴风涓嶅緱渚濊禆涔嬪墠璁剧疆鐨勬牸寮忚淇濈暀銆?

2. 鍦?`OUTPUT` 闃熷垪涓婃灇涓炬牸寮忥紝鎬绘槸鍙繑鍥炲綋鍓?`CAPTURE` 鏍煎紡鎵€鏀寔鐨勬牸寮忋€?

3. 鍦?`OUTPUT` 闃熷垪涓婅缃牸寮忥紝涓嶄細鏀瑰彉 `CAPTURE` 闃熷垪涓婂彲鐢ㄦ牸寮忓垪琛ㄣ€傝嫢灏濊瘯璁剧疆褰撳墠鎵€閫?`CAPTURE` 鏍煎紡涓嶆敮鎸佺殑 `OUTPUT` 鏍煎紡锛岀紪鐮佸櫒浼氬皢鎵€璇锋眰鐨?`OUTPUT` 鏍煎紡璋冩暣涓哄彈鏀寔鐨勬煇涓牸寮忋€?

4. 鍦?`CAPTURE` 闃熷垪涓婃灇涓炬牸寮忥紝鎬绘槸杩斿洖鍙楁敮鎸佺紪鐮佹牸寮忕殑瀹屾暣闆嗗悎锛屼笌褰撳墠 `OUTPUT` 鏍煎紡鏃犲叧銆?

5. 褰撶紦鍐插尯宸插湪 `OUTPUT` 鎴?`CAPTURE` 浠讳竴闃熷垪涓婂垎閰嶆椂锛屽鎴风涓嶅緱鏇存敼 `CAPTURE` 闃熷垪涓婄殑鏍煎紡銆傚浜庝换浣曟绫绘牸寮忔洿鏀瑰皾璇曪紝椹卞姩灏嗚繑鍥?-EBUSY 閿欒鐮併€?

鎬荤粨鑰岃█锛岃缃牸寮忎笌鍒嗛厤缂撳啿鍖哄繀椤诲缁堜粠 `CAPTURE` 闃熷垪寮€濮嬶紝鑰?`CAPTURE` 闃熷垪鏄富鎺ф柟锛屽畠鍐冲畾浜?`OUTPUT` 闃熷垪鎵€鏀寔鐨勬牸寮忛泦鍚堛€?
