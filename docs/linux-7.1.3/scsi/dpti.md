
## Adaptec dpti 椹卞姩


Redistribution and use in source form, with or without modification, are
permitted provided that redistributions of source code must retain the
above copyright notice, this list of conditions and the following disclaimer.

This software is provided `as is` by Adaptec and
any express or implied warranties, including, but not limited to, the
implied warranties of merchantability and fitness for a particular purpose,
are disclaimed. In no event shall Adaptec be
liable for any direct, indirect, incidental, special, exemplary or
consequential damages (including, but not limited to, procurement of
substitute goods or services; loss of use, data, or profits; or business
interruptions) however caused and on any theory of liability, whether in
contract, strict liability, or tort (including negligence or otherwise)
arising in any way out of the use of this driver software, even if advised
of the possibility of such damage.

鏈┍鍔ㄦ敮鎸?Adaptec I2O RAID 涓?DPT SmartRAID V I2O 鏉垮崱銆?
## 鑷磋阿


鍘熷 Linux 椹卞姩鐢?Karen White 鍦?Dell Computer 鏈熼棿绉绘鍒?Linux銆傚畠绉绘鑷?Bob Pasteur
锛圖PT锛夌殑鍘熷闈?Linux 椹卞姩銆侻ark Salyzyn 涓?Bob Pasteur 瀵瑰師濮嬮┍鍔ㄦ彁渚涗簡鍜ㄨ銆?
2.0 鐗堟湰鐨勯┍鍔ㄧ敱 Deanna Bonds 涓?Mark Salyzyn 瀹屾垚銆?
## 鍘嗗彶


璇ラ┍鍔ㄦ渶鍒濊绉绘鍒?linux 2.0.34 鐗堟湰銆?
==== ==========================================================================
V2.0 椹卞姩閲嶅啓銆傚熀浜?i2o 瀛愮郴缁熼噸鏂版灦鏋勩€傝繖鏄涓€涓畬鍏?GPL 鐨勭増鏈紝鍥犱负涓婁竴涓増鏈娇鐢ㄧ殑
     i2osig 澶存枃浠朵笉鏄?GPL 鐨勩€傚紑鍙戣€呮祴璇曠増鏈€?V2.1 鍐呴儴娴嬭瘯
V2.2 棣栦釜鍙戝竷鐗堟湰

V2.3 鍙樻洿锛?
     - 澧炲姞浜?Raptor 鏀寔
     - 淇浜嗗湪璐熻浇鏋侀噸銆佺鐞嗗伐鍏疯繍琛岋紙浠?kmalloc 鏍囧織涓Щ闄?GFP_DMA锛夋椂瀵艰嚧绯荤粺鎸傝捣鐨勭己闄?
V2.4 棣栦釜鍑嗗濂芥彁浜ゅ苟宓屽叆鍐呮牳鐨勭増鏈?
     鍙樻洿锛?
     - 瀹炵幇浜?Alan Cox 鐨勫缓璁?     - 涓?sg 灞傚鍔犱簡 resid 鐨勮绠?     - 鏇村ソ鐨勯敊璇鐞?     - 澧炲姞浜嗕笅婧㈡潯浠舵鏌?     - 澧炲姞浜?DATAPROTECT 妫€鏌?     - 鏇存敼浜嗛敊璇繑鍥炵爜
     - 淇浜嗘€荤嚎澶嶄綅渚嬬▼涓殑鎸囬拡缂洪櫡
     - 鍚敤浜嗘潵鑷?ioctl 鐨?hba 澶嶄綅锛堝厑璁?FW 鍒峰啓鍚庨噸鍚苟浣跨敤鏂?FW锛岃€屾棤闇€閲嶅惎绯荤粺锛?     - 鏇存敼浜?proc 杈撳嚭
==== ==========================================================================

## 寰呭姙


- 鍦?64 浣嶆灦鏋勪笂缂栬瘧鏃跺鍔?64 浣嶅垎鏁?鑱氶泦锛圫catter Gather锛夋敮鎸?- 澧炲姞绋€鐤?LUN 鎵弿
- 澧炲姞鍦?scsi-core 鍙戝嚭 test unit ready 鎴?inquiry 鍛戒护鏃讹紝妫€鏌ユ浘琚绾匡紙鍦?FW 灞傞潰锛夌殑璁惧
  鐜板凡鍦ㄧ嚎鐨勪唬鐮?- 澧炲姞 proc 璇绘帴鍙?- busrescan 鍛戒护
- rescan 鍛戒护
- 澧炲姞鍚?scsi-core 閫氱煡鏂拌澶囩殑 rescan 渚嬬▼浠ｇ爜
- 澧炲姞 C-PCI锛堢儹鎻掓嫈鐩稿叧锛夋敮鎸?- 澧炲姞 ioctl 閫忎紶閿欒鎭㈠

## 璇存槑


DPT 鍗′細浼樺寲鍛戒护澶勭悊鐨勯『搴忋€傚洜姝わ紝涓€鏉″懡浠ゅ湪鍙戦€佸埌鏉垮崱鍚庢渶澶氬彲鑳介渶瑕?6 鍒嗛挓鎵嶈兘瀹屾垚銆?
鏂囦欢 dpti_ioctl.h銆乨ptsig.h銆乷sd_defs.h銆乷sd_util.h銆乻ys_info.h 鏄?Adaptec 绠＄悊渚嬬▼鐨勬帴鍙?鏂囦欢鐨勪竴閮ㄥ垎銆傚畠浠畾涔変簡 ioctl 涓娇鐢ㄧ殑缁撴瀯浣撱€傚畠浠鍐欐垚鍙Щ妞嶇殑銆傚畠浠毦浠ラ槄璇伙紝浣嗘垜闇€瑕?鈥滃師鏍封€濅娇鐢ㄥ畠浠紝鍚﹀垯鎴戝彲鑳戒細婕忔帀鎺ュ彛鐨勫彉鏇淬€?