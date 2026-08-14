


## NPCM 瑙嗛椹卞姩


璇ラ┍鍔ㄧ敤浜庢帶鍒?Nuvoton NPCM SoC 涓婂瓨鍦ㄧ殑瑙嗛鎹曡幏/宸垎锛圴CD锛夊紩鎿庝笌缂栫爜鍘嬬缉锛圗CE锛夊紩鎿庛€俈CD 鍙互浠庢暟瀛楄棰戣緭鍏ユ崟鑾蜂竴甯э紝骞跺湪鍐呭瓨涓瘮杈冧袱甯э紱ECE 鍙互灏嗗抚鏁版嵁鍘嬬缉涓?HEXTILE 鏍煎紡銆?
### 椹卞姩涓撶敤鎺у埗


#### V4L2_CID_NPCM_CAPTURE_MODE


VCD 寮曟搸鏀寔涓ょ妯″紡锛?
- COMPLETE 妯″紡锛?
  灏嗕笅涓€瀹屾暣甯ф崟鑾峰埌鍐呭瓨涓€?
- DIFF 妯″紡锛?
  灏嗚緭鍏ュ抚涓庡唴瀛樹腑瀛樺偍鐨勫抚杩涜姣旇緝锛屽苟鏇存柊鍐呭瓨涓殑宸垎甯с€?
搴旂敤绋嬪簭鍙互浣跨敤 `V4L2_CID_NPCM_CAPTURE_MODE` 鎺у埗锛岄€氳繃涓嶅悓鐨勬帶鍒跺€硷紙enum v4l2_npcm_capture_mode锛夎缃?VCD 妯″紡锛?
- `V4L2_NPCM_CAPTURE_MODE_COMPLETE`锛氬皢 VCD 璁剧疆涓?COMPLETE 妯″紡銆?- `V4L2_NPCM_CAPTURE_MODE_DIFF`锛氬皢 VCD 璁剧疆涓?DIFF 妯″紡銆?
#### V4L2_CID_NPCM_RECT_COUNT


濡傛灉浣跨敤 V4L2_PIX_FMT_HEXTILE 鏍煎紡锛孷CD 灏嗘崟鑾峰抚鏁版嵁锛岀劧鍚?ECE 灏嗘暟鎹帇缂╀负 HEXTILE 鐭╁舰锛屽苟鎸夌収杩滅▼甯х紦鍐插崗璁紙Remote Framebuffer Protocol锛変腑瀹氫箟鐨勫竷灞€瀛樺偍鍒?V4L2 瑙嗛缂撳啿鍖轰腑锛?```

           (RFC 6143, https://www.rfc-editor.org/rfc/rfc6143.html#section-7.6.1)

           +--------------+--------------+-------------------+
           | No. of bytes | Type [Value] | Description       |
           +--------------+--------------+-------------------+
           | 2            | U16          | x-position        |
           | 2            | U16          | y-position        |
           | 2            | U16          | width             |
           | 2            | U16          | height            |
           | 4            | S32          | encoding-type (5) |
           +--------------+--------------+-------------------+
           |             HEXTILE rectangle data              |
           +-------------------------------------------------+

```
搴旂敤绋嬪簭鍙互閫氳繃 VIDIOC_DQBUF 鑾峰彇瑙嗛缂撳啿鍖猴紝鐒跺悗璋冪敤 `V4L2_CID_NPCM_RECT_COUNT` 鎺у埗鏉ヨ幏鍙栬缂撳啿鍖轰腑 HEXTILE 鐭╁舰鐨勬暟閲忋€?
### 鍙傝€?

include/uapi/linux/npcm-video.h

**Copyright** |copy| 2022 Nuvoton Technologies
