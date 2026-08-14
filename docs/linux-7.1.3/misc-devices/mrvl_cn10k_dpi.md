## Marvell CN10K DMA 鏁版嵁鍖呮帴鍙ｏ紙DPI锛夐┍鍔?

## 姒傝堪


DPI 鏄?Marvell CN10K 鑺墖涓殑 DMA 鏁版嵁鍖呮帴鍙ｇ‖浠舵ā鍧椼€侱PI 纭欢鍖呭惈涓€涓墿鐞嗗姛鑳?锛圥F锛夈€佸叾铏氭嫙鍔熻兘銆侀偖绠遍€昏緫锛屼互鍙婁竴缁?DMA 寮曟搸鍜?DMA 鍛戒护闃熷垪銆?
DPI PF 鍔熻兘鏄竴涓鐞嗗姛鑳斤紝瀹冨鐞嗘潵鑷叾 VF 鍔熻兘鐨勯偖绠辫姹傦紝骞跺悜鍏?VF 鍔熻兘鍒嗛厤
DMA 寮曟搸璧勬簮銆?
mrvl_cn10k_dpi.ko misc 椹卞姩鍦?DPI PF 璁惧涓婂姞杞斤紝骞跺鐞?VF 璁惧鎻愪氦鐨勯偖绠卞懡浠わ紝
鐩稿簲鍦板垵濮嬪寲 DMA 寮曟搸鍜?VF 璁惧鐨?DMA 鍛戒护闃熷垪銆傛澶栵紝椹卞姩鍒涘缓 /dev/mrvl-cn10k-dpi
鑺傜偣锛岀敤浜庤缃?DMA 寮曟搸鍜?PEM锛圥CIe 鎺ュ彛锛夌鍙ｅ睘鎬э紝濡?fifo 闀垮害銆乵olr銆乵ps 鍜?mrrs銆?
DPI PF 椹卞姩鍙槸涓€涓敤浜庤缃叾 VF 璁惧闃熷垪骞跺垎閰嶇‖浠惰祫婧愮殑绠＄悊椹卞姩锛屽畠涓嶈兘鍙戣捣
浠讳綍 DMA 鎿嶄綔銆傚彧鏈?VF 璁惧琚垎閰嶄簡 DMA 鑳藉姏銆?
## 椹卞姩浣嶇疆


drivers/misc/mrvl_cn10k_dpi.c

## 椹卞姩 IOCTL


`DPI_MPS_MRRS_CFG`
ioctl锛岀敤浜庤缃?DMA 寮曟搸鎵€杩炴帴鐨?pem 绔彛鐨勬渶澶ф湁鏁堣礋杞藉ぇ灏忓拰鏈€澶ц璇锋眰澶у皬
鍙傛暟銆?
`DPI_ENGINE_CFG`
ioctl锛岀敤浜庤缃?DMA 寮曟搸鐨?fifo 澶у皬鍜屾渶澶ф湭瀹屾垚鍔犺浇璇锋眰闃堝€笺€?
## 鐢ㄦ埛绌洪棿浠ｇ爜绀轰緥


DPI VF 璁惧閫氳繃 vfio-pci 椹卞姩浠庣敤鎴风┖闂村簲鐢ㄧ▼搴忔帰娴嬪拰璁块棶銆備笅闈㈡槸涓€涓ず渚?dpi
dma 搴旂敤绋嬪簭锛屾紨绀哄簲鐢ㄧ▼搴忓浣曚娇鐢ㄦ潵鑷?DPI PF 鍐呮牳椹卞姩鐨勯偖绠卞拰 ioctl 鏈嶅姟銆?
https://github.com/MarvellEmbeddedProcessors/dpi-sample-app
