## bnxt devlink 鏀寔


鏈枃妗ｆ弿杩?`bnxt` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## 鍙傛暟


   - - 鍚嶇О
     - 妯″紡
   - - `enable_sriov`
     - Permanent
   - - `ignore_ari`
     - Permanent
   - - `msix_vec_per_pf_max`
     - Permanent
   - - `msix_vec_per_pf_min`
     - Permanent
   - - `enable_remote_dev_reset`
     - Runtime
   - - `enable_roce`
     - Permanent

`bnxt` 椹卞姩杩樺疄鐜颁簡浠ヤ笅椹卞姩涓撶敤鍙傛暟銆?
   :widths: 5 5 5 85

   - - 鍚嶇О
     - 绫诲瀷
     - 妯″紡
     - 鎻忚堪
   - - `gre_ver_check`
     - Boolean
     - Permanent
     - 灏嗗湪璁惧涓惎鐢ㄩ€氱敤璺敱灏佽锛圙RE锛夌増鏈鏌ャ€傝嫢绂佺敤锛岃澶囧皢瀵?       鍏ョ珯鏁版嵁鍖呰烦杩囩増鏈鏌ャ€?
## 淇℃伅鐗堟湰


`bnxt_en` 椹卞姩鎶ュ憡浠ヤ笅鐗堟湰

      :widths: 5 5 90

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   - - `board.id`
     - fixed
     - 鏍囪瘑鏉垮崱璁捐鐨勯儴浠跺彿
   - - `asic.id`
     - fixed
     - ASIC 璁捐鏍囪瘑绗?   - - `asic.rev`
     - fixed
     - ASIC 璁捐淇鐗堟湰
   - - `fw.psid`
     - stored, running
     - 鏉垮崱鐨勫浐浠跺弬鏁伴泦鐗堟湰
   - - `fw`
     - stored, running
     - 鏁翠綋鏉垮崱鍥轰欢鐗堟湰
   - - `fw.mgmt`
     - stored, running
     - NIC 纭欢璧勬簮绠＄悊鍥轰欢鐗堟湰
   - - `fw.mgmt.api`
     - running
     - 椹卞姩涓庡浐浠朵箣闂存敮鎸佺殑鏈€浣庡浐浠舵帴鍙ｈ鑼冪増鏈?   - - `fw.nsci`
     - stored, running
     - 閫氱敤骞冲彴绠＄悊鍥轰欢鐗堟湰
   - - `fw.roce`
     - stored, running
     - RoCE 绠＄悊鍥轰欢鐗堟湰
