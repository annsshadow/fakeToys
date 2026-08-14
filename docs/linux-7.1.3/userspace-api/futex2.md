## futex2


:Author: Andr茅 Almeida <andrealmeid@collabora.com>

futex锛屽嵆 fast user mutex锛堝揩閫熺敤鎴锋€佷簰鏂ヤ綋锛夛紝鏄竴缁勭郴缁熻皟鐢紝鍏佽鐢ㄦ埛鎬?鍒涘缓楂樻€ц兘鐨勫悓姝ユ満鍒讹紝渚嬪鐢ㄦ埛鎬佷腑鐨勪簰鏂ヤ綋銆佷俊鍙烽噺鍜屾潯浠跺彉閲忋€侰 鏍囧噯搴擄紙濡?glibc锛夊皢鍏剁敤浣滃疄鐜版洿楂樺眰鎺ュ彛锛堝 pthreads锛夌殑鎵嬫銆?
futex2 鏄渶鍒濈殑 futex 绯荤粺璋冪敤鐨勫悗缁増鏈紝鏃ㄥ湪鍏嬫湇鍘熷鎺ュ彛鐨勫眬闄愭€с€?
## 鐢ㄦ埛 API


### ``futex_waitv()``


```
futex_waitv(struct futex_waitv *waiters, unsigned int nr_futexes,
            unsigned int flags, struct timespec *timeout, clockid_t clockid)

  struct futex_waitv {
        __u64 val;
        __u64 uaddr;
        __u32 flags;
        __u32 __reserved;
  };

```
鐢ㄦ埛鎬佽缃竴涓?struct futex_waitv 鏁扮粍锛堟渶澶?128 涓潯鐩級锛屼娇鐢?`uaddr` 琛ㄧず
瑕佺瓑寰呯殑鍦板潃锛宍val` 琛ㄧず鏈熸湜鍊硷紝`flags` 鎸囧畾 futex 鐨勭被鍨嬶紙濡傜鏈夛級鍜屽ぇ灏忋€?`__reserved` 蹇呴』涓?0锛屼絾鍙敤浜庢湭鏉ユ墿灞曘€傛暟缁勭涓€涓潯鐩殑鎸囬拡浣滀负 `waiters`
浼犲叆銆傝嫢 `waiters` 鎴栦换鎰?`uaddr` 鍦板潃鏃犳晥锛屽垯杩斿洖 `-EFAULT`銆?
濡傛灉鐢ㄦ埛鎬佷娇鐢?32 浣嶆寚閽堬紝搴旇繘琛屾樉寮忚浆鎹互纭繚楂樹綅琚竻闆躲€俙uintptr_t` 鍙阀濡?鍦板畬鎴愯繖涓€宸ヤ綔锛屼笖瀵?32/64 浣嶆寚閽堝潎閫傜敤銆?
`nr_futexes` 鎸囧畾鏁扮粍鐨勫ぇ灏忋€傝秴鍑?[1, 128] 鍖洪棿鐨勬暟鍊煎皢浣跨郴缁熻皟鐢ㄨ繑鍥?`-EINVAL`銆?
绯荤粺璋冪敤鐨?`flags` 鍙傛暟闇€瑕佷负 0锛屼絾鍙敤浜庢湭鏉ユ墿灞曘€?
瀵逛簬 `waiters` 鏁扮粍涓殑姣忎釜鏉＄洰锛屽皢 `uaddr` 澶勭殑褰撳墠鍊间笌 `val` 姣旇緝銆傝嫢涓嶅悓锛?绯荤粺璋冪敤灏嗘挙閿€杩勪粖涓烘鎵€鍋氱殑鍏ㄩ儴宸ヤ綔骞惰繑鍥?`-EAGAIN`銆傝嫢鎵€鏈夋祴璇曚笌鏍￠獙鍧?鎴愬姛锛岀郴缁熻皟鐢ㄥ皢绛夊緟鐩村埌鍙戠敓浠ヤ笅鎯呭喌涔嬩竴锛?
- 瓒呮椂鍒版湡锛岃繑鍥?`-ETIMEOUT`銆?- 鍚戠潯鐪犱换鍔″彂閫佷簡淇″彿锛岃繑鍥?`-ERESTARTSYS`銆?- 鍒楄〃涓殑鏌愪釜 futex 琚敜閱掞紝杩斿洖琚敜閱?futex 鐨勭储寮曘€?
濡備綍浣跨敤璇ユ帴鍙ｇ殑绀轰緥鍙湪 `tools/testing/selftests/futex/functional/futex_waitv.c` 涓壘鍒般€?
### 瓒呮椂锛圱imeout锛?

`struct timespec *timeout` 鍙傛暟鏄竴涓彲閫夊弬鏁帮紝鎸囧悜涓€涓粷瀵硅秴鏃躲€傞渶瑕佸湪
`clockid` 鍙傛暟涓寚瀹氭墍鐢ㄦ椂閽熺殑绫诲瀷銆傛敮鎸?`CLOCK_MONOTONIC` 鍜?`CLOCK_REALTIME`銆傝绯荤粺璋冪敤鍙帴鍙?64 浣?timespec 缁撴瀯浣撱€?
### futex 鐨勭被鍨?

futex 鍙互鏄鏈夌殑鎴栧叡浜殑銆傜鏈?futex 鐢ㄤ簬鍏变韩鍚屼竴鍐呭瓨绌洪棿銆佷笖 futex 鐨?铏氭嫙鍦板潃瀵规墍鏈夎繘绋嬮兘鐩稿悓鐨勮繘绋嬨€傝繖鍏佽鍐呮牳杩涜浼樺寲銆傝浣跨敤绉佹湁 futex锛岄渶鍦?futex 鏍囧織涓寚瀹?`FUTEX_PRIVATE_FLAG`銆傚浜庝笉鍏变韩鍚屼竴鍐呭瓨绌洪棿銆佸洜姝ゅ悓涓€
futex 鍙兘鍏锋湁涓嶅悓铏氭嫙鍦板潃鐨勮繘绋嬶紙渚嬪浣跨敤鏂囦欢鏀寔鐨勫叡浜唴瀛橈級锛屽垯闇€瑕佷笉鍚岀殑
鍐呴儴鏈哄埗鎵嶈兘琚纭叆闃熴€傝繖鏄粯璁よ涓猴紝涓斿绉佹湁鍜屽叡浜?futex 閮介€傜敤銆?
futex 鍙互鏈変笉鍚岀殑澶у皬锛?銆?6銆?2 鎴?64 浣嶃€傜洰鍓嶅敮涓€鍙楁敮鎸佺殑鏄?32 浣嶅ぇ灏忕殑
futex锛屼笖蹇呴』浣跨敤 `FUTEX_32` 鏍囧織鎸囧畾銆?