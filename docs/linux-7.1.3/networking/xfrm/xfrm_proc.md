
## XFRM proc - /proc/net/xfrm_* 鏂囦欢


Masahide NAKAMURA <nakam@linux-ipv6.org>


### 杞崲缁熻


xfrm_proc 浠ｇ爜鏄竴缁勭粺璁℃暟鎹紝鏄剧ず浜嗚杞崲浠ｇ爜涓㈠純鐨勬暟鎹寘鏁伴噺鍙婂叾鍘熷洜銆?杩欎簺璁℃暟鍣ㄥ畾涔変负 Linux 绉佹湁 MIB 鐨勪竴閮ㄥ垎銆傝繖浜涜鏁板櫒鍙互鍦?/proc/net/xfrm_stat 涓煡鐪嬨€?

#### 鍏ョ珯閿欒


XfrmInError:
	鎵€鏈変笉鍖归厤鍏跺畠椤圭殑鎵€鏈夐敊璇?
XfrmInBufferError:
	娌℃湁鍓╀綑缂撳啿鍖?
XfrmInHdrError:
	澶撮儴閿欒

XfrmInNoStates:
	鏈壘鍒扮姸鎬?	鍗冲叆绔欑殑 SPI銆佸湴鍧€鎴?SA 涓婄殑 IPsec 鍗忚鏈夎

XfrmInStateProtoError:
	杞崲鍗忚鐩稿叧閿欒
	渚嬪 SA 瀵嗛挜鏈夎

XfrmInStateModeError:
	杞崲妯″紡鐩稿叧閿欒

XfrmInStateSeqError:
	搴忓垪鍙烽敊璇?	鍗冲簭鍒楀彿瓒呭嚭绐楀彛

XfrmInStateExpired:
	鐘舵€佸凡杩囨湡

XfrmInStateMismatch:
	鐘舵€佸瓨鍦ㄤ笉鍖归厤鐨勯€夐」
	渚嬪 UDP 灏佽绫诲瀷涓嶅尮閰?
XfrmInStateInvalid:
	鐘舵€佹棤鏁?
XfrmInTmplMismatch:
	鐘舵€佹病鏈夊尮閰嶇殑妯℃澘
	渚嬪鍏ョ珯 SA 姝ｇ‘浣?SP 瑙勫垯鏈夎

XfrmInNoPols:
	鐘舵€佹湭鎵惧埌绛栫暐
	渚嬪鍏ョ珯 SA 姝ｇ‘浣嗘湭鎵惧埌 SP

XfrmInPolBlock:
	绛栫暐涓㈠純

XfrmInPolError:
	绛栫暐閿欒

XfrmAcquireError:
	鐘舵€佸湪浣跨敤鍓嶅皻鏈瀹屽叏鑾峰彇

XfrmFwdHdrError:
	涓嶅厑璁稿鏁版嵁鍖呰繘琛岃浆鍙戣矾鐢?
XfrmInStateDirError:
        鐘舵€佹柟鍚戜笉鍖归厤锛堝湪鍏ョ珯璺緞涓婃煡鎵惧埌浜嗗嚭绔欑姸鎬侊紝鏈熸湜涓哄叆绔欐垨鏃犳柟鍚戯級

#### 鍑虹珯閿欒

XfrmOutError:
	鎵€鏈変笉鍖归厤鍏跺畠椤圭殑鎵€鏈夐敊璇?
XfrmOutBundleGenError:
	鎹嗙粦锛坆undle锛夌敓鎴愰敊璇?
XfrmOutBundleCheckError:
	鎹嗙粦妫€鏌ラ敊璇?
XfrmOutNoStates:
	鏈壘鍒扮姸鎬?
XfrmOutStateProtoError:
	杞崲鍗忚鐩稿叧閿欒

XfrmOutStateModeError:
	杞崲妯″紡鐩稿叧閿欒

XfrmOutStateSeqError:
	搴忓垪鍙烽敊璇?	鍗冲簭鍒楀彿婧㈠嚭

XfrmOutStateExpired:
	鐘舵€佸凡杩囨湡

XfrmOutPolBlock:
	绛栫暐涓㈠純

XfrmOutPolDead:
	绛栫暐宸插け鏁?
XfrmOutPolError:
	绛栫暐閿欒

XfrmOutStateInvalid:
	鐘舵€佹棤鏁堬紝鍙兘宸茶繃鏈?
XfrmOutStateDirError:
        鐘舵€佹柟鍚戜笉鍖归厤锛堝湪鍑虹珯璺緞涓婃煡鎵惧埌浜嗗叆绔欑姸鎬侊紝鏈熸湜涓哄嚭绔欐垨鏃犳柟鍚戯級
