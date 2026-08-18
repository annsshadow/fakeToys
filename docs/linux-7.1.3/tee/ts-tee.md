
## TS-TEE锛圱rusted Services 椤圭洰锛?

璇ラ┍鍔ㄦ彁渚涘鐢?Trusted Services 瀹炵幇鐨勫畨鍏ㄦ湇鍔＄殑璁块棶銆?
Trusted Services [^1^] 鏄竴涓?TrustedFirmware.org 椤圭洰锛屾彁渚涗簡涓€濂楃敤浜庡湪 FF-A [^2^] S-EL0
瀹夊叏鍒嗗尯锛圫ecure Partition锛変腑寮€鍙戝拰閮ㄧ讲璁惧鍙俊鏍癸紙Root of Trust锛夋湇鍔＄殑妗嗘灦銆傝椤圭洰鎵樼浜?闈㈠悜 Arm A-profile 璁惧鐨?Arm Platform Security Architecture [^3^] 鐨勫弬鑰冨疄鐜般€?
FF-A 瀹夊叏鍒嗗尯锛圫P锛夊彲閫氳繃 FF-A 椹卞姩 [^4^] 璁块棶锛岃椹卞姩涓烘湰鏂囨。鎵€杩伴┍鍔ㄦ彁渚涗簡搴曞眰閫氫俊銆傚湪姝?涔嬩笂浣跨敤鐨勬槸 Trusted Services RPC 鍗忚 [^5^]銆傝浠庣敤鎴风┖闂翠娇鐢ㄨ椹卞姩锛屽湪 [^6^] 澶勬彁渚涗簡涓€涓?鍙傝€冨疄鐜帮紝瀹冩槸鍚嶄负 libts [^7^] 鐨?Trusted Services 瀹㈡埛绔簱鐨勪竴閮ㄥ垎銆?
鎵€鏈?Trusted Services锛圱S锛塖P 鎷ユ湁鐩稿悓鐨?FF-A UUID锛涘畠鏍囪瘑鐨勬槸 TS RPC 鍗忚銆備竴涓?TS SP 鍙互
鎵胯浇涓€涓垨澶氫釜鏈嶅姟锛堜緥濡?PSA Crypto銆丳SA ITS 绛夛級銆備竴涓湇鍔＄敱鍏舵湇鍔?UUID 鏍囪瘑锛涘悓涓€绫诲瀷鐨勬湇鍔?涓嶈兘鍦ㄥ悓涓€ SP 涓嚭鐜颁袱娆°€傚湪 SP 鍚姩鏈熼棿锛孲P 涓殑姣忎釜鏈嶅姟浼氳鍒嗛厤涓€涓€滄帴鍙?ID鈥濓紙interface ID锛夈€?杩欏彧鏄竴涓畝鐭殑 ID锛岀敤浜庣畝鍖栨秷鎭鍧€銆?
閫氱敤 TEE 鐨勮璁℃槸涓€娆℃€т笌鍙俊鎿嶄綔绯荤粺锛圱rusted OS锛夊叡浜唴瀛橈紝鐒跺悗璇ュ唴瀛樺彲琚鐢紝鐢ㄦ潵涓庤繍琛屽湪
鍙俊鎿嶄綔绯荤粺涓婄殑澶氫釜搴旂敤绋嬪簭閫氫俊銆傜劧鑰岋紝鍦?FF-A 鐨勬儏鍐典笅锛屽唴瀛樺叡浜槸鍦ㄧ鐐癸紙endpoint锛夊眰闈㈠伐浣?鐨勶紝鍗冲唴瀛樻槸涓庣壒瀹氱殑 SP 鍏变韩鐨勩€傜敤鎴风┖闂村繀椤昏兘澶熸牴鎹鐐?ID 鍒嗗埆涓庢瘡涓?SP 鍏变韩鍐呭瓨锛涘洜姝わ紝涓?姣忎竴涓鍙戠幇鐨?TS SP 娉ㄥ唽涓€涓嫭绔嬬殑 TEE 璁惧銆傛墦寮€涓€涓?SP 瀵瑰簲浜庢墦寮€璇?TEE 璁惧骞跺垱寤轰竴涓?TEE
涓婁笅鏂囥€備竴涓?TS SP 鎵胯浇涓€涓垨澶氫釜鏈嶅姟銆傛墦寮€涓€涓湇鍔″搴斾簬鍦ㄧ粰瀹氱殑 tee_context 涓墦寮€涓€涓細璇濄€?
```

   User space                  Kernel space                   Secure world
   ~~~~~~~~~~                  ~~~~~~~~~~~~                   ~~~~~~~~~~~~
   +--------+                                               +-------------+
   | Client |                                               | Trusted     |
   +--------+                                               | Services SP |
      /\                                                    +-------------+
      ||                                                          /\
      ||                                                          ||
      ||                                                          ||
      \/                                                          \/
   +-------+                +----------+--------+           +-------------+
   | libts |                |  TEE     | TS-TEE |           |  FF-A SPMC  |
   |       |                |  subsys  | driver |           |   + SPMD    |
   +-------+----------------+----+-----+--------+-----------+-------------+
   |      Generic TEE API        |     |  FF-A  |     TS RPC protocol     |
   |      IOCTL (TEE_IOC_*)      |     | driver |        over FF-A        |
   +-----------------------------+     +--------+-------------------------+

```
## 鍙傝€?

[^1^] https://www.trustedfirmware.org/projects/trusted-services/

[^2^] https://developer.arm.com/documentation/den0077/

[^3^] https://www.arm.com/architecture/security-features/platform-security

[^4^] drivers/firmware/arm_ffa/

[^5^] https://trusted-services.readthedocs.io/en/v1.0.0/developer/service-access-protocols.html#abi

[^6^] https://git.trustedfirmware.org/TS/trusted-services.git/tree/components/rpc/ts_rpc/caller/linux/ts_rpc_caller_linux.c?h=v1.0.0

[^7^] https://git.trustedfirmware.org/TS/trusted-services.git/tree/deployments/libts/arm-linux/CMakeLists.txt?h=v1.0.0
