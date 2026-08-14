
## 铏氭嫙鏃犵姸鎬佽В鐮佸櫒椹卞姩锛坴isl锛?

鐢ㄤ簬鏃犵姸鎬?uAPI 寮€鍙戠殑铏氭嫙鏃犵姸鎬佽В鐮佸櫒璁惧銆?
璇ュ伐鍏风殑鐩爣鏄负浣跨敤 V4L2 鏃犵姸鎬?API 瑙ｇ爜濯掍綋鐨勭敤鎴风┖闂村簲鐢ㄧ▼搴忕殑寮€鍙戜笌娴嬭瘯
鎻愪緵甯姪銆傚嵆浣挎病鏈夊彲鐢ㄧ‖浠讹紝鎴栬€呯紪瑙ｇ爜鍣ㄧ殑鍐呮牳 uAPI 灏氭湭鍚堝叆涓荤嚎锛岀敤鎴风┖闂?瀹炵幇涔熷彲浠ヤ娇鐢?visl 鏉ヨ繍琛岃В鐮佸惊鐜€傝繖鏈夊姪浜庡湪鏃╂湡闃舵鍙戠幇缂洪櫡銆?
璇ラ┍鍔ㄨ繕鍙互璺熻釜鎻愪氦缁欏畠鐨?V4L2 鎺т欢鐨勫唴瀹广€傚畠杩樺彲浠ラ€氳繃 debugfs 鎺ュ彛杞偍 vb2
缂撳啿鍖虹殑鍐呭銆傝繖鍦ㄨ澶氭柟闈㈢被浼间簬鍏朵粬娴佽缂栬В鐮?API 鎵€鎻愪緵鐨勮窡韪熀纭€璁炬柦锛屽苟涓?鍙互閫氳繃浠ュ彟涓€涓紙鍙敤鐨勶級瀹炵幇浣滀负鍙傝€冩潵甯姪寮€鍙戠敤鎴风┖闂村簲鐢ㄧ▼搴忋€?

        visl 瀹為檯涓婂苟涓嶄細瀵硅棰戝抚杩涜浠讳綍瑙ｇ爜銆傚畠鏀圭敤 V4L2 娴嬭瘯鍥炬鐢熸垚鍣紝
        鍚戞崟鑾风紦鍐插尯鍐欏叆鍚勭璋冭瘯淇℃伅銆?
### 妯″潡鍙傛暟


- visl_debug锛氭縺娲昏皟璇曚俊鎭紝閫氳繃 dprintk 鎵撳嵃鍚勭璋冭瘯娑堟伅銆傚悓鏃舵帶鍒舵槸鍚︽樉绀?  閫愬抚璋冭瘯淇℃伅銆傞粯璁ゅ叧闂€傛敞鎰忥紝鍚敤姝ゅ姛鑳戒細閫氳繃涓插彛瀵艰嚧鎬ц兘涓嬮檷銆?
- visl_transtime_ms锛氭ā鎷熺殑澶勭悊鏃堕棿锛堟绉掞級銆傞檷浣庤В鐮侀€熷害鏈夊姪浜庤皟璇曘€?
- visl_dprintk_frame_start銆乿isl_dprintk_frame_nframes锛氭寚瀹氬惎鐢?dprintk 鐨?  甯ц寖鍥淬€傝繖浠呮寜甯ф帶鍒?dprintk 璺熻釜銆傛敞鎰忥紝閫氳繃涓插彛鎵撳嵃澶ч噺鏁版嵁鍙兘杈冩參銆?
- keep_bitstream_buffers锛氭帶鍒惰В鐮佷細璇濈粨鏉熷悗鏄惁淇濈暀鐮佹祦锛堝嵆 OUTPUT锛夌紦鍐插尯銆?  榛樿涓?false 浠ュ噺灏戞潅涔便€傚綋浣跨敤 GDB 瀹炴椂璋冭瘯瀹㈡埛绔▼搴忔椂锛?  keep_bitstream_buffers == false 琛ㄧ幇鑹ソ銆?
- bitstream_trace_frame_start銆乥itstream_trace_nframes锛氫笌 visl_dprintk_frame_start銆?  visl_dprintk_nframes 绫讳技锛屼絾鏀逛负鎺у埗閫氳繃 debugfs 杞偍缂撳啿鍖烘暟鎹€?
- tpg_verbose锛氬湪姣忎釜杈撳嚭甯т笂鍐欏叆棰濆淇℃伅浠ヤ究浜庤皟璇?API銆傚綋璁句负 true 鏃讹紝缁欏畾
  杈撳叆鐨勮緭鍑哄抚涓嶅啀绋冲畾锛屽洜涓轰細鍚戝叾涓坊鍔犳寚閽堟垨闃熷垪鐘舵€佺瓑淇℃伅銆?
### 璇ラ┍鍔ㄧ殑榛樿鐢ㄤ緥鏄粈涔堬紵


璇ラ┍鍔ㄥ彲鐢ㄤ簬姣旇緝涓嶅悓鐨勭敤鎴风┖闂村疄鐜般€傝繖鍋囪鍏堣繍琛屼竴涓彲鐢ㄧ殑瀹㈡埛绔鎺?visl锛?鐒跺悗鍒╃敤 ftrace 鍜?OUTPUT 缂撳啿鍖烘暟鎹潵璋冭瘯涓€涓鍦ㄥ紑鍙戜腑鐨勫疄鐜般€?
灏界瀹為檯涓婂苟鏈繘琛岃棰戣В鐮侊紝浣嗚緭鍑哄抚浠嶅彲浣滀负缁欏畾杈撳叆鐨勫弬鑰冿紝闄ら潪 tpg_verbose
琚涓?true銆?
鏍规嵁 tpg_verbose 鍙傛暟鐨勫€硷紝鍏充簬鍙傝€冨抚銆佸叾鏃堕棿鎴炽€丱UTPUT 涓?CAPTURE 闃熷垪鐨勭姸鎬佺瓑
鏇村淇℃伅锛屽彲浠ョ洿鎺ヤ粠 CAPTURE 缂撳啿鍖轰腑璇诲彇銆?
### 鏀寔鐨勭紪瑙ｇ爜鍣?

- FWHT
- MPEG2
- VP8
- VP9
- H.264
- HEVC
- AV1

### visl 璺熻釜浜嬩欢


璺熻釜浜嬩欢鏄寜缂栬В鐮佸櫒瀹氫箟鐨勶紝渚嬪锛?

        $ ls /sys/kernel/tracing/events/ | grep visl
        visl_av1_controls
        visl_fwht_controls
        visl_h264_controls
        visl_hevc_controls
        visl_mpeg2_controls
        visl_vp8_controls
        visl_vp9_controls

渚嬪锛岃杞偍 HEVC SPS 鏁版嵁锛?

        $ echo 1 >  /sys/kernel/tracing/events/visl_hevc_controls/v4l2_ctrl_hevc_sps/enable

SPS 鏁版嵁灏嗚杞偍鍒拌窡韪紦鍐插尯锛屽嵆锛?

        $ cat /sys/kernel/tracing/trace
        video_parameter_set_id 0
        seq_parameter_set_id 0
        pic_width_in_luma_samples 1920
        pic_height_in_luma_samples 1080
        bit_depth_luma_minus8 0
        bit_depth_chroma_minus8 0
        log2_max_pic_order_cnt_lsb_minus4 4
        sps_max_dec_pic_buffering_minus1 6
        sps_max_num_reorder_pics 2
        sps_max_latency_increase_plus1 0
        log2_min_luma_coding_block_size_minus3 0
        log2_diff_max_min_luma_coding_block_size 3
        log2_min_luma_transform_block_size_minus2 0
        log2_diff_max_min_luma_transform_block_size 3
        max_transform_hierarchy_depth_inter 2
        max_transform_hierarchy_depth_intra 2
        pcm_sample_bit_depth_luma_minus1 0
        pcm_sample_bit_depth_chroma_minus1 0
        log2_min_pcm_luma_coding_block_size_minus3 0
        log2_diff_max_min_pcm_luma_coding_block_size 0
        num_short_term_ref_pic_sets 0
        num_long_term_ref_pics_sps 0
        chroma_format_idc 1
        sps_max_sub_layers_minus1 0
        flags AMP_ENABLED|SAMPLE_ADAPTIVE_OFFSET|TEMPORAL_MVP_ENABLED|STRONG_INTRA_SMOOTHING_ENABLED


### 閫氳繃 debugfs 杞偍 OUTPUT 缂撳啿鍖烘暟鎹?

濡傛灉鍚敤浜?**VISL_DEBUGFS** Kconfig锛寁isl 浼氭牴鎹?bitstream_trace_frame_start 鍜?bitstream_trace_nframes 鐨勫€硷紝灏?OUTPUT 缂撳啿鍖烘暟鎹～鍏?**/sys/kernel/debug/visl/bitstream**銆傝繖鍙互鍙戠幇閿欒锛屽洜涓哄瓨鍦ㄧ己闄风殑瀹㈡埛绔彲鑳?鏃犳硶姝ｇ‘濉厖缂撳啿鍖恒€?
涓烘瘡涓凡澶勭悊鐨?OUTPUT 缂撳啿鍖哄垱寤轰竴涓崟鐙殑鏂囦欢銆傚叾鍚嶇О鍖呭惈涓€涓〃绀虹紦鍐插尯搴忓彿
鐨勬暣鏁帮紝鍗筹細


	snprintf(name, 32, "bitstream%d", run->src->sequence);

杞偍杩欎簺鍊煎彧闇€浠庢枃浠惰鍙栧嵆鍙紝鍗筹細

瀵逛簬 sequence == 0 鐨勭紦鍐插尯锛?

        $ xxd /sys/kernel/debug/visl/bitstream/bitstream0
        00000000: 2601 af04 d088 bc25 a173 0e41 a4f2 3274  &......%.s.A..2t
        00000010: c668 cb28 e775 b4ac f53a ba60 f8fd 3aa1  .h.(.u...:.`..:.
        00000020: 46b4 bcfc 506c e227 2372 e5f5 d7ea 579f  F...Pl.'#r....W.
        00000030: 6371 5eb5 0eb8 23b5 ca6a 5de5 983a 19e4  cq^...#..j]..:..
        00000040: e8c3 4320 b4ba a226 cbc1 4138 3a12 32d6  ..C ...&..A8:.2.
        00000050: fef3 247b 3523 4e90 9682 ac8e eb0c a389  ..${5#N.........
        00000060: ddd0 6cfc 0187 0e20 7aae b15b 1812 3d33  ..l.... z..[..=3
        00000070: e1c5 f425 a83a 00b7 4f18 8127 3c4c aefb  ...%.:..O..'<L..

瀵逛簬 sequence == 1 鐨勭紦鍐插尯锛?

        $ xxd /sys/kernel/debug/visl/bitstream/bitstream1
        00000000: 0201 d021 49e1 0c40 aa11 1449 14a6 01dc  ...!I..@...I....
        00000010: 7023 889a c8cd 2cd0 13b4 dab0 e8ca 21fe  p#....,.......!.
        00000020: c4c8 ab4c 486e 4e2f b0df 96cc c74e 8dde  ...LHnN/.....N..
        00000030: 8ce7 ee36 d880 4095 4d64 30a0 ff4f 0c5e  ...6..@.Md0..O.^
        00000040: f16b a6a1 d806 ca2a 0ece a673 7bea 1f37  .k.....*...s{..7
        00000050: 370f 5bb9 1dc4 ba21 6434 bc53 0173 cba0  7.[....!d4.S.s..
        00000060: dfe6 bc99 01ea b6e0 346b 92b5 c8de 9f5d  ........4k.....]
        00000070: e7cc 3484 1769 fef2 a693 a945 2c8b 31da  ..4..i.....E,.1.

渚濇绫绘帹銆?
榛樿鎯呭喌涓嬶紝杩欎簺鏂囦欢浼氬湪 STREAMOFF 鏈熼棿琚垹闄ゃ€傝繖鏄负浜嗗噺灏戞潅涔便€?