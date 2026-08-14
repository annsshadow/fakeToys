
## 涓嶅彲鎵ц mfd 鐨勫紩鍏?


:Author:
    Daniel Verkamp <dverkamp@chromium.org>
    Jeff Xu <jeffxu@chromium.org>

:Contributor:
	Aleksa Sarai <cyphar@cyphar.com>

鑷?Linux 寮曞叆 memfd 鐗规€т互鏉ワ紝memfd 濮嬬粓甯︽湁鎵ц浣嶏紝鑰?memfd_create() 绯荤粺璋冪敤涓嶅厑璁镐互涓嶅悓鏂瑰紡璁剧疆瀹冦€?

鐒惰€岋紝鍦ㄤ竴涓粯璁ゅ畨鍏紙secure-by-default锛夌殑绯荤粺锛堝 ChromeOS锛屽叾涓墍鏈夊彲鎵ц鏂囦欢閮藉簲鏉ヨ嚜鍙楅獙璇佸惎鍔ㄤ繚鎶ょ殑鏍规枃浠剁郴缁燂級涓紝memfd 鐨勮繖绉嶅彲鎵ц鐗规€т负 NoExec 缁曡繃鎵撳紑浜嗗ぇ闂紝骞朵績鎴愪簡鈥滄贩娣嗕唬鐞嗘敾鍑伙紙confused deputy attack锛夆€濄€備緥濡傦紝鍦?VRP 缂洪櫡 [^1^] 涓細cros_vm 杩涚▼鍒涘缓浜嗕竴涓?memfd 鏉ヤ笌澶栭儴杩涚▼鍏变韩鍐呭锛屼絾璇?memfd 琚鍐欏苟鐢ㄤ簬鎵ц浠绘剰浠ｇ爜涓庢彁鏉冦€俒^2^] 鍒楀嚭浜嗘洿澶氭绫?VRP銆?

鍙︿竴鏂归潰锛屽彲鎵ц鐨?memfd 鏈夊叾鍚堟硶鐢ㄩ€旓細runc 浣跨敤 memfd 鐨?seal 涓庡彲鎵ц鐗规€ф潵澶嶅埗浜岃繘鍒剁殑鍐呭鐒跺悗鎵ц瀹冧滑銆傚浜庤繖鏍风殑绯荤粺锛屾垜浠渶瑕佷竴绉嶆柟妗堟潵鍖哄垎 runc 瀵瑰彲鎵ц memfd 鐨勪娇鐢ㄤ笌鏀诲嚮鑰呯殑浣跨敤 [^3^]銆?

涓轰簡瑙ｅ喅涓婅堪闂锛?
 - 璁?memfd_create() 鍦ㄥ垱寤烘椂璁剧疆 X 浣嶃€?
 - 褰撹缃?NX 鏃讹紝璁?memfd 琚?seal 浠ョ姝慨鏀?X 浣嶃€?
 - 鏂板涓€涓?pid namespace sysctl锛歷m.memfd_noexec锛屼互甯姪搴旂敤绋嬪簭杩佺Щ骞跺己鍒朵娇鐢ㄤ笉鍙墽琛?MFD銆?

## 鐢ㄦ埛 API


`int memfd_create(const char *name, unsigned int flags)`

`MFD_NOEXEC_SEAL`
	褰?`flags` 涓缃簡 MFD_NOEXEC_SEAL 浣嶆椂锛宮emfd 浠?NX 鍒涘缓銆侳_SEAL_EXEC 琚缃紝涓?memfd 涔嬪悗涓嶈兘琚慨鏀逛负娣诲姞 X銆傚悓鏃堕殣鍚?MFD_ALLOW_SEALING銆?
	杩欐槸搴旂敤绋嬪簭浣跨敤 memfd 鏈€甯歌鐨勬儏鍐点€?

`MFD_EXEC`
	褰?`flags` 涓缃簡 MFD_EXEC 浣嶆椂锛宮emfd 浠?X 鍒涘缓銆?

娉ㄦ剰锛?
	`MFD_NOEXEC_SEAL` 闅愬惈 `MFD_ALLOW_SEALING`銆傝嫢搴旂敤绋嬪簭涓嶅笇鏈?seal锛屽畠鍙互鍦ㄥ垱寤哄悗娣诲姞 F_SEAL_SEAL銆?

## Sysctl锛?


`pid namespaced sysctl vm.memfd_noexec`

鏂扮殑 pid namespaced sysctl vm.memfd_noexec 鏈?3 涓€硷細

 - 0: MEMFD_NOEXEC_SCOPE_EXEC
	涓嶅甫 MFD_EXEC 涔熶笉甯?MFD_NOEXEC_SEAL 鐨?memfd_create() 琛ㄧ幇寰楀鍚岃缃簡 MFD_EXEC銆?

 - 1: MEMFD_NOEXEC_SCOPE_NOEXEC_SEAL
	涓嶅甫 MFD_EXEC 涔熶笉甯?MFD_NOEXEC_SEAL 鐨?memfd_create() 琛ㄧ幇寰楀鍚岃缃簡 MFD_NOEXEC_SEAL銆?

 - 2: MEMFD_NOEXEC_SCOPE_NOEXEC_ENFORCED
	涓嶅甫 MFD_NOEXEC_SEAL 鐨?memfd_create() 灏嗚鎷掔粷銆?

璇?sysctl 鍏佽瀵逛笉璁剧疆鎵ц浣嶇殑鏃ц蒋浠惰繘琛屾洿绮剧粏鐨?memfd_create 鎺у埗锛涗緥濡傦紝涓€涓?vm.memfd_noexec=1 鐨勫鍣ㄦ剰鍛崇潃鏃ц蒋浠堕粯璁ゅ皢鍒涘缓涓嶅彲鎵ц鐨?memfd锛岃€屾柊杞欢鍙互閫氳繃璁剧疆 MFD_EXEC 鍒涘缓鍙墽琛岀殑 memfd銆?

vm.memfd_noexec 鐨勫€煎湪鍒涘缓鏃朵紶閫掔粰瀛愬懡鍚嶇┖闂淬€傛澶栵紝璇ヨ缃槸鍒嗗眰鐨勶紝鍗冲湪 memfd_create 鏈熼棿锛屾垜浠皢浠庡綋鍓?ns 鎼滅储鍒版牴 ns锛屽苟浣跨敤鏈€涓ユ牸鐨勮缃€?

[^1^] https://crbug.com/1305267

[^2^] https://bugs.chromium.org/p/chromium/issues/list?q=type%3Dbug-security%20memfd%20escalation&can=1

[^3^] https://lwn.net/Articles/781013/
