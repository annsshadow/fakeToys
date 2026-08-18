######## V4L2 API 鐨勫彉鏇?

鏈〉璁板綍 V4L2锛圴ideo4Linux2锛夌敤鎴风┖闂?API 鐨勬紨杩涗笌鍙樻洿鍘嗗彶锛屾寜鏃堕棿椤哄簭鏁寸悊浜嗚嚜 1998 骞?V4L2 鍙栦唬 V4L 浠ユ潵鍚勭増鏈湪鎺ュ彛銆乮octl 涓庢暟鎹牸寮忎笂鐨勫叧閿敼鍔紝渚涢┍鍔ㄥ紑鍙戣€呬笌搴旂敤绋嬪簭浣滆€呰拷婧?API 鐨勬紨鍙樸€?


Soon after the V4L API was added to the kernel it was criticised as too
inflexible. In August 1998 Bill Dirks proposed a number of improvements
and began to work on documentation, example drivers and applications.
With the help of other volunteers this eventually became the V4L2 API,
not just an extension but a replacement for the V4L API. However it took
another four years and two stable kernel releases until the new API was
finally accepted for inclusion into the kernel in its present form.

鍦?V4L API 鍔犲叆鍐呮牳鍚庝笉涔咃紝浜轰滑渚挎壒璇勫畠杩囦簬
缂轰箯鐏垫椿鎬с€?998 骞?8 鏈堬紝Bill Dirks 鎻愬嚭浜嗕竴绯诲垪鏀硅繘寤鸿锛?
骞跺紑濮嬬潃鎵嬬紪鍐欐枃妗ｃ€佺ず渚嬮┍鍔ㄧ▼搴忎互鍙婂簲鐢ㄧ▼搴忋€?
鍦ㄥ叾浠栧織鎰胯€呯殑甯姪涓嬶紝杩欎簺宸ヤ綔鏈€缁堟紨鍙樻垚浜?V4L2 API锛?
瀹冧笉浠呬粎鏄?V4L API 鐨勬墿灞曪紝鑰屾槸鍏舵浛浠ｅ搧銆傜劧鑰屽張杩囦簡
鍥涘勾浠ュ強涓ゆ绋冲畾鐨勫唴鏍稿彂甯冿紝杩欎釜鏂?API 鎵嶆渶缁堜互
鐩墠鐨勫舰寮忚鍐呮牳鎺ュ彈骞跺悎鍏ャ€?

## Early Versions

## 鏃╂湡鐗堟湰


1998-08-20: First version.

1998-08-20锛氶涓増鏈€?

1998-08-27: The `select()` function was introduced.

1998-08-27锛氬紩鍏ヤ簡 `select()` 鍑芥暟銆?

1998-09-10: New video standard interface.

1998-09-10锛氭柊鐨勮棰戞爣鍑嗘帴鍙ｃ€?

1998-09-18: The `VIDIOC_NONCAP` ioctl was replaced by the otherwise
meaningless `O_TRUNC` `open()` flag, and the
aliases `O_NONCAP` and `O_NOIO` were defined. Applications can set
this flag if they intend to access controls only, as opposed to capture
applications which need exclusive access. The `VIDEO_STD_XXX`
identifiers are now ordinals instead of flags, and the
`video_std_construct()` helper function takes id and
transmission arguments.

1998-09-18锛歚VIDIOC_NONCAP` ioctl 琚竴涓湰韬?
鏃犳剰涔夌殑 `O_TRUNC` `open()` 鏍囧織鎵€鍙栦唬锛屽悓鏃跺畾涔変簡
鍒悕 `O_NONCAP` 鍜?`O_NOIO`銆傚鏋滃簲鐢ㄧ▼搴忎粎鎵撶畻璁块棶
鎺у埗椤癸紙涓庨渶瑕佺嫭鍗犺闂殑閲囬泦搴旂敤绋嬪簭鐩稿锛夛紝鍒欏彲浠ヨ缃?
璇ユ爣蹇椼€俙VIDEO_STD_XXX`
鏍囪瘑绗︾幇鍦ㄦ槸搴忔暟鑰岄潪鏍囧織浣嶏紝鑰?
`video_std_construct()` 杈呭姪鍑芥暟鎺ュ彈 id 鍜?
浼犺緭锛坱ransmission锛夊弬鏁般€?

1998-09-28: Revamped video standard. Made video controls individually
enumerable.

1998-09-28锛氶噸濉戜簡瑙嗛鏍囧噯銆備娇瑙嗛鎺у埗椤瑰彲琚崟鐙?
鏋氫妇銆?

1998-10-02: The `id` field was removed from
struct `video_standard` and the color subcarrier fields were
renamed. The VIDIOC_QUERYSTD ioctl was
renamed to VIDIOC_ENUMSTD,
VIDIOC_G_INPUT <VIDIOC_G_INPUT> to
VIDIOC_ENUMINPUT. A first draft of the
Codec API was released.

1998-10-02锛氫粠 struct `video_standard` 涓Щ闄や簡 `id` 瀛楁锛?
棰滆壊鍓浇娉紙color subcarrier锛夊瓧娈佃閲嶅懡鍚嶃€俈IDIOC_QUERYSTD ioctl 琚?
閲嶅懡鍚嶄负 VIDIOC_ENUMSTD锛?
VIDIOC_G_INPUT <VIDIOC_G_INPUT> 琚噸鍛藉悕涓?
VIDIOC_ENUMINPUT銆侰odec API 鐨勯涓崏妗堝彂甯冦€?

1998-11-08: Many minor changes. Most symbols have been renamed. Some
material changes to struct v4l2_capability.

1998-11-08锛氬ぇ閲忕粏寰敼鍔ㄣ€傚ぇ澶氭暟绗﹀彿琚噸鍛藉悕銆俿truct v4l2_capability
鏈変竴浜涘疄璐ㄦ€ф敼鍔ㄣ€?

1998-11-12: The read/write direction of some ioctls was misdefined.

1998-11-12锛氭煇浜?ioctl 鐨勮/鍐欐柟鍚戝畾涔夋湁璇€?

1998-11-14: `V4L2_PIX_FMT_RGB24` changed to `V4L2_PIX_FMT_BGR24`,
and `V4L2_PIX_FMT_RGB32` changed to `V4L2_PIX_FMT_BGR32`. Audio
controls are now accessible with the
VIDIOC_G_CTRL <VIDIOC_G_CTRL> and
VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctls under names starting
with `V4L2_CID_AUDIO`. The `V4L2_MAJOR` define was removed from
`videodev.h` since it was only used once in the `videodev` kernel
module. The `YUV422` and `YUV411` planar image formats were added.

1998-11-14锛歚V4L2_PIX_FMT_RGB24` 鏀逛负 `V4L2_PIX_FMT_BGR24`锛?
`V4L2_PIX_FMT_RGB32` 鏀逛负 `V4L2_PIX_FMT_BGR32`銆傞煶棰?
鎺у埗椤圭幇鍦ㄥ彲閫氳繃
VIDIOC_G_CTRL <VIDIOC_G_CTRL> 鍜?
VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl 浠?
`V4L2_CID_AUDIO` 寮€澶寸殑鍚嶇О璁块棶銆俙V4L2_MAJOR` 瀹忓畾涔?
浠?`videodev.h` 涓Щ闄わ紝鍥犱负瀹冧粎鍦?`videodev` 鍐呮牳
妯″潡涓浣跨敤杩囦竴娆°€俙YUV422` 鍜?`YUV411` 骞抽潰鍥惧儚鏍煎紡琚姞鍏ャ€?

1998-11-28: A few ioctl symbols changed. Interfaces for codecs and video
output devices were added.

1998-11-28锛氬皯鏁?ioctl 绗﹀彿鍙戠敓鍙樺寲銆傛柊澧炰簡鐢ㄤ簬缂栬В鐮佸櫒锛坈odec锛夊拰瑙嗛
杈撳嚭璁惧鐨勬帴鍙ｃ€?

1999-01-14: A raw VBI capture interface was added.

1999-01-14锛氭柊澧炰簡鍘熷 VBI 閲囬泦鎺ュ彛銆?

1999-01-19: The `VIDIOC_NEXTBUF` ioctl was removed.

1999-01-19锛氱Щ闄や簡 `VIDIOC_NEXTBUF` ioctl銆?

## V4L2 Version 0.16 1999-01-31

## V4L2 0.16 鐗?1999-01-31


1999-01-27: There is now one QBUF ioctl, VIDIOC_QWBUF and VIDIOC_QRBUF
are gone. VIDIOC_QBUF takes a v4l2_buffer as a parameter. Added
digital zoom (cropping) controls.

1999-01-27锛氱幇鍦ㄥ彧鏈変竴涓?QBUF ioctl锛孷IDIOC_QWBUF 鍜?VIDIOC_QRBUF
宸插簾寮冦€俈IDIOC_QBUF 浠?v4l2_buffer 浣滀负鍙傛暟銆傛柊澧炰簡
鏁板瓧缂╂斁锛堣鍓級鎺у埗椤广€?

## V4L2 Version 0.18 1999-03-16

## V4L2 0.18 鐗?1999-03-16


Added a v4l to V4L2 ioctl compatibility layer to videodev.c. Driver
writers, this changes how you implement your ioctl handler. See the
Driver Writer's Guide. Added some more control id codes.

鍦?videodev.c 涓柊澧炰簡 v4l 鍒?V4L2 鐨?ioctl 鍏煎灞傘€傞┍鍔?
寮€鍙戣€呰娉ㄦ剰锛岃繖鏀瑰彉浜?ioctl 澶勭悊鍑芥暟鐨勫疄鐜版柟寮忋€傝鍙傞槄
銆婇┍鍔ㄥ紑鍙戣€呮寚鍗椼€嬨€傛柊澧炰簡鏇村鎺у埗 id 浠ｇ爜銆?

## V4L2 Version 0.19 1999-06-05

## V4L2 0.19 鐗?1999-06-05


1999-03-18: Fill in the category and catname fields of v4l2_queryctrl
objects before passing them to the driver. Required a minor change to
the VIDIOC_QUERYCTRL handlers in the sample drivers.

1999-03-18锛氬湪灏?v4l2_queryctrl 瀵硅薄浼犻€掔粰椹卞姩涔嬪墠锛岄渶瑕佸厛
濉ソ鍏?category 鍜?catname 瀛楁銆傝繖闇€瑕佸
绀轰緥椹卞姩涓殑 VIDIOC_QUERYCTRL 澶勭悊鍑芥暟鍋氬皬骞呮敼鍔ㄣ€?

1999-03-31: Better compatibility for v4l memory capture ioctls. Requires
changes to drivers to fully support new compatibility features, see
Driver Writer's Guide and v4l2cap.c. Added new control IDs:
V4L2_CID_HFLIP, _VFLIP. Changed V4L2_PIX_FMT_YUV422P to _YUV422P,
and _YUV411P to _YUV411P.

1999-03-31锛氭敼鍠勪簡瀵?v4l 鍐呭瓨閲囬泦 ioctl 鐨勫吋瀹规€с€傞渶瑕?
淇敼椹卞姩浠ュ畬鏁存敮鎸佹柊鐨勫吋瀹圭壒鎬э紝璇峰弬闃呫€婇┍鍔ㄥ紑鍙戣€呮寚鍗椼€嬪拰
v4l2cap.c銆傛柊澧炰簡鎺у埗 ID锛歏4L2_CID_HFLIP銆乢VFLIP銆傚皢
V4L2_PIX_FMT_YUV422P 鏀逛负 _YUV422P锛屽皢 _YUV411P 鏀逛负 _YUV411P銆?

1999-04-04: Added a few more control IDs.

1999-04-04锛氭柊澧炰簡鏇村鎺у埗 ID銆?

1999-04-07: Added the button control type.

1999-04-07锛氭柊澧炰簡鎸夐挳锛坆utton锛夋帶鍒剁被鍨嬨€?

1999-05-02: Fixed a typo in videodev.h, and added the
V4L2_CTRL_FLAG_GRAYED (later V4L2_CTRL_FLAG_GRABBED) flag.

1999-05-02锛氫慨姝ｄ簡 videodev.h 涓殑涓€涓嫾鍐欓敊璇紝骞舵柊澧炰簡
V4L2_CTRL_FLAG_GRAYED锛堝悗鏀瑰悕涓?V4L2_CTRL_FLAG_GRABBED锛夋爣蹇椼€?

1999-05-20: Definition of VIDIOC_G_CTRL was wrong causing a
malfunction of this ioctl.

1999-05-20锛歏IDIOC_G_CTRL 鐨勫畾涔夋湁璇紝瀵艰嚧璇?ioctl 宸ヤ綔寮傚父銆?

1999-06-05: Changed the value of V4L2_CID_WHITENESS.

1999-06-05锛氭洿鏀逛簡 V4L2_CID_WHITENESS 鐨勫€笺€?

## V4L2 Version 0.20 (1999-09-10)

## V4L2 0.20 鐗堬紙1999-09-10锛?


Version 0.20 introduced a number of changes which were *not backward
compatible* with 0.19 and earlier versions. Purpose of these changes was
to simplify the API, while making it more extensible and following
common Linux driver API conventions.

0.20 鐗堝紩鍏ヤ簡璁稿涓?0.19 鍙婃洿鏃╃増鏈?涓嶅悜鍚庡吋瀹?鐨勬敼鍔ㄣ€傝繖浜涙敼鍔ㄧ殑
鐩殑鏄畝鍖?API锛屽悓鏃朵娇鍏舵洿鍏峰彲鎵╁睍鎬э紝骞堕伒寰?
閫氱敤鐨?Linux 椹卞姩 API 鎯緥銆?

1. Some typos in `V4L2_FMT_FLAG` symbols were fixed. struct v4l2_clip
   was changed for compatibility with v4l. (1999-08-30)

1. 淇浜?`V4L2_FMT_FLAG` 绗﹀彿涓殑涓€浜涙嫾鍐欓敊璇€備负鍏煎 v4l锛?
   淇敼浜?struct v4l2_clip銆傦紙1999-08-30锛?

2. `V4L2_TUNER_SUB_LANG1` was added. (1999-09-05)

2. 鏂板浜?`V4L2_TUNER_SUB_LANG1`銆傦紙1999-09-05锛?

3. All ioctl() commands that used an integer argument now take a pointer
   to an integer. Where it makes sense, ioctls will return the actual
   new value in the integer pointed to by the argument, a common
   convention in the V4L2 API. The affected ioctls are: VIDIOC_PREVIEW,
   VIDIOC_STREAMON, VIDIOC_STREAMOFF, VIDIOC_S_FREQ,
   VIDIOC_S_INPUT, VIDIOC_S_OUTPUT, VIDIOC_S_EFFECT. For example

3. 鎵€鏈変娇鐢ㄦ暣鏁板弬鏁扮殑 ioctl() 鍛戒护鐜板湪閮芥敼涓烘帴鍙椾竴涓寚鍚戞暣鏁扮殑
   鎸囬拡銆傚湪鍚堢悊鐨勬儏鍐典笅锛宨octl 浼氶€氳繃鍙傛暟鎵€鎸囧悜鐨勬暣鏁拌繑鍥?
   瀹為檯鐨勬柊鍊硷紝杩欐槸 V4L2 API 涓殑閫氱敤鎯緥銆傚彈褰卞搷鐨?ioctl 鍖呮嫭锛?
   VIDIOC_PREVIEW銆乂IDIOC_STREAMON銆乂IDIOC_STREAMOFF銆乂IDIOC_S_FREQ銆?
   VIDIOC_S_INPUT銆乂IDIOC_S_OUTPUT銆乂IDIOC_S_EFFECT銆備緥濡?

   .. code-block:: c

       err = ioctl (fd, VIDIOC_XXX, V4L2_XXX);

   becomes

   鍙樹负

   .. code-block:: c

       int a = V4L2_XXX; err = ioctl(fd, VIDIOC_XXX, &a);

4. All the different get- and set-format commands were swept into one
   VIDIOC_G_FMT <VIDIOC_G_FMT> and
   VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl taking a union and a
   type field selecting the union member as parameter. Purpose is to
   simplify the API by eliminating several ioctls and to allow new and
   driver private data streams without adding new ioctls.

4. 鎵€鏈変笉鍚岀殑鑾峰彇/璁剧疆鏍煎紡鍛戒护琚悎骞朵负鍗曚竴鐨?
   VIDIOC_G_FMT <VIDIOC_G_FMT> 鍜?
   VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl锛屽叾鍙傛暟涓轰竴涓仈鍚堬紙union锛?
   浠ュ強涓€涓敤浜庨€夋嫨鑱斿悎鎴愬憳鐨?type 瀛楁銆傚叾鐩殑鏄€氳繃娑堥櫎鑻ュ共
   ioctl 鏉ョ畝鍖?API锛屽苟鍦ㄤ笉鏂板 ioctl 鐨勫墠鎻愪笅鏀寔鏂扮殑浠ュ強
   椹卞姩绉佹湁鐨勬暟鎹祦銆?

   This change obsoletes the following ioctls: `VIDIOC_S_INFMT`,
   `VIDIOC_G_INFMT`, `VIDIOC_S_OUTFMT`, `VIDIOC_G_OUTFMT`,
   `VIDIOC_S_VBIFMT` and `VIDIOC_G_VBIFMT`. The image format
   struct v4l2_format was renamed to struct v4l2_pix_format, while
   struct v4l2_format is now the enveloping structure
   for all format negotiations.

   杩欎竴鏀瑰姩浣夸互涓?ioctl 杩囨椂锛歚VIDIOC_S_INFMT`銆?
   `VIDIOC_G_INFMT`銆乣VIDIOC_S_OUTFMT`銆乣VIDIOC_G_OUTFMT`銆?
   `VIDIOC_S_VBIFMT` 鍜?`VIDIOC_G_VBIFMT`銆傚浘鍍忔牸寮?
   struct v4l2_format 琚噸鍛藉悕涓?struct v4l2_pix_format锛岃€?
   struct v4l2_format 鐜板湪鎴愪负鎵€鏈夋牸寮忓崗鍟嗙殑
   澶栧眰灏佽缁撴瀯銆?

5. Similar to the changes above, the `VIDIOC_G_PARM` and
   `VIDIOC_S_PARM` ioctls were merged with `VIDIOC_G_OUTPARM` and
   `VIDIOC_S_OUTPARM`. A `type` field in the new struct v4l2_streamparm
   selects the respective union member.

5. 涓庝笂闈㈢殑鏀瑰姩绫讳技锛宍VIDIOC_G_PARM` 鍜?
   `VIDIOC_S_PARM` ioctl 涓?`VIDIOC_G_OUTPARM` 鍜?
   `VIDIOC_S_OUTPARM` 鍚堝苟銆傛柊 struct v4l2_streamparm 涓殑
   `type` 瀛楁鐢ㄤ簬閫夋嫨鐩稿簲鐨勮仈鍚堟垚鍛樸€?

   This change obsoletes the `VIDIOC_G_OUTPARM` and
   `VIDIOC_S_OUTPARM` ioctls.

   杩欎竴鏀瑰姩浣?`VIDIOC_G_OUTPARM` 鍜?`VIDIOC_S_OUTPARM` ioctl 杩囨椂銆?

6. Control enumeration was simplified, and two new control flags were
   introduced and one dropped. The `catname` field was replaced by a
   `group` field.

6. 绠€鍖栦簡鎺у埗鏋氫妇锛屽紩鍏ヤ簡涓や釜鏂扮殑鎺у埗鏍囧織骞跺幓鎺変簡涓€涓€俙catname` 瀛楁
   琚?`group` 瀛楁鍙栦唬銆?

   Drivers can now flag unsupported and temporarily unavailable controls
   with `V4L2_CTRL_FLAG_DISABLED` and `V4L2_CTRL_FLAG_GRABBED`
   respectively. The `group` name indicates a possibly narrower
   classification than the `category`. In other words, there may be
   multiple groups within a category. Controls within a group would
   typically be drawn within a group box. Controls in different
   categories might have a greater separation, or may even appear in
   separate windows.

   椹卞姩鐜板湪鍙互浣跨敤 `V4L2_CTRL_FLAG_DISABLED` 鍜?
   `V4L2_CTRL_FLAG_GRABBED` 鍒嗗埆鏍囪涓嶅彈鏀寔鍜屼复鏃朵笉鍙敤鐨?
   鎺у埗椤广€俙group` 鍚嶇О琛ㄧず鍙兘姣?`category` 鏇寸粏鐨?
   鍒嗙被銆傛崲鍙ヨ瘽璇达紝涓€涓?category 涓彲鑳芥湁澶氫釜 group銆傚悓涓€ group 鍐呯殑
   鎺у埗椤归€氬父浼氳缁樺埗鍦ㄤ竴涓垎缁勬锛坓roup box锛変腑銆備笉鍚?category 涓殑
   鎺у埗椤瑰彲鑳介棿闅旀洿澶э紝鐢氳嚦鍙兘鍑虹幇鍦ㄧ嫭绔嬬殑绐楀彛涓€?

7. The struct v4l2_buffer `timestamp` was
   changed to a 64 bit integer, containing the sampling or output time
   of the frame in nanoseconds. Additionally timestamps will be in
   absolute system time, not starting from zero at the beginning of a
   stream. The data type name for timestamps is stamp_t, defined as a
   signed 64-bit integer. Output devices should not send a buffer out
   until the time in the timestamp field has arrived. I would like to
   follow SGI's lead, and adopt a multimedia timestamping system like
   their UST (Unadjusted System Time). See
   http://web.archive.org/web/\*/http://reality.sgi.com
   /cpirazzi_engr/lg/time/intro.html. UST uses timestamps that are
   64-bit signed integers (not struct timeval's) and given in nanosecond
   units. The UST clock starts at zero when the system is booted and
   runs continuously and uniformly. It takes a little over 292 years for
   UST to overflow. There is no way to set the UST clock. The regular
   Linux time-of-day clock can be changed periodically, which would
   cause errors if it were being used for timestamping a multimedia
   stream. A real UST style clock will require some support in the
   kernel that is not there yet. But in anticipation, I will change the
   timestamp field to a 64-bit integer, and I will change the
   v4l2_masterclock_gettime() function (used only by drivers) to
   return a 64-bit integer.

7. struct v4l2_buffer 鐨?`timestamp` 琚?
   鏀逛负 64 浣嶆暣鏁帮紝浠ョ撼绉掍负鍗曚綅淇濆瓨甯х殑閲囨牱鎴栬緭鍑烘椂闂淬€傛澶栵紝
   鏃堕棿鎴冲皢閲囩敤缁濆绯荤粺鏃堕棿锛岃€屼笉鏄粠娴佸紑濮嬫椂鐨勯浂绠楄捣銆傛椂闂存埑鐨?
   鏁版嵁绫诲瀷鍚嶄负 stamp_t锛屽畾涔変负鏈夌鍙?64 浣嶆暣鏁般€傝緭鍑鸿澶囧湪璇?
   timestamp 瀛楁鎵€琛ㄧず鐨勬椂闂村埌鏉ヤ箣鍓嶄笉搴斿彂鍑虹紦鍐插尯銆傛垜甯屾湜
   鏁堜豢 SGI 鐨勫仛娉曪紝閲囩敤绫讳技鍏?UST锛圲nadjusted System Time锛?
   鏈牎姝ｇ郴缁熸椂闂达級鐨勫濯掍綋鏃堕棿鎴崇郴缁熴€傚弬瑙?
   http://web.archive.org/web/\*/http://reality.sgi.com
   /cpirazzi_engr/lg/time/intro.html銆俇ST 浣跨敤 64 浣嶆湁绗﹀彿鏁存暟
   锛堣€岄潪 struct timeval锛変綔涓烘椂闂存埑锛屽崟浣嶄负绾崇銆俇ST 鏃堕挓鍦?
   绯荤粺鍚姩鏃朵粠闆跺紑濮嬶紝杩炵画涓斿潎鍖€鍦拌繍琛屻€俇ST 婧㈠嚭闇€瑕佺暐澶氫簬 292 骞淬€?
   UST 鏃堕挓鏃犳硶琚缃€傛櫘閫氱殑 Linux 鏃ユ椂閽燂紙time-of-day clock锛変細
   琚懆鏈熸€у湴鏇存敼锛岃嫢灏嗗叾鐢ㄤ簬澶氬獟浣撴祦鐨勬椂闂存埑鍒欎細瀵艰嚧閿欒銆傜湡姝ｇ殑
   UST 椋庢牸鏃堕挓闇€瑕佸唴鏍镐腑灏氫笉瀛樺湪鐨勬煇浜涙敮鎸併€備絾浣滀负棰勬湡锛屾垜浼氬皢
   timestamp 瀛楁鏀逛负 64 浣嶆暣鏁帮紝骞跺皢
   v4l2_masterclock_gettime() 鍑芥暟锛堜粎椹卞姩浣跨敤锛夋敼涓?
   杩斿洖涓€涓?64 浣嶆暣鏁般€?

8. A `sequence` field was added to struct v4l2_buffer. The `sequence`
   field counts captured frames, it is ignored by output devices. When a
   capture driver drops a frame, the sequence number of that frame is skipped.

8. 鍦?struct v4l2_buffer 涓柊澧炰簡 `sequence` 瀛楁銆俙sequence`
   瀛楁瀵归噰闆嗗埌鐨勫抚杩涜璁℃暟锛岃緭鍑鸿澶囦細蹇界暐瀹冦€傚綋閲囬泦椹卞姩
   涓㈠純鏌愪竴甯ф椂锛岃甯х殑搴忓彿浼氳璺宠繃銆?

## V4L2 Version 0.20 incremental changes

## V4L2 0.20 鐗堢殑澧為噺鏀瑰姩


1999-12-23: In struct v4l2_vbi_format the
`reserved1` field became `offset`. Previously drivers were required
to clear the `reserved1` field.

1999-12-23锛氬湪 struct v4l2_vbi_format 涓紝`reserved1` 瀛楁鍙樹负
`offset`銆傛鍓嶉┍鍔ㄩ渶瑕佹竻闄?`reserved1` 瀛楁銆?

2000-01-13: The `V4L2_FMT_FLAG_NOT_INTERLACED` flag was added.

2000-01-13锛氭柊澧炰簡 `V4L2_FMT_FLAG_NOT_INTERLACED` 鏍囧織銆?

2000-07-31: The `linux/poll.h` header is now included by
`videodev.h` for compatibility with the original `videodev.h` file.

2000-07-31锛氫负浜嗕笌鍘熷鐨?`videodev.h` 鏂囦欢鍏煎锛宍videodev.h`
鐜板湪鍖呭惈浜?`linux/poll.h` 澶存枃浠躲€?

2000-11-20: `V4L2_TYPE_VBI_OUTPUT` and `V4L2_PIX_FMT_Y41P` were
added.

2000-11-20锛氭柊澧炰簡 `V4L2_TYPE_VBI_OUTPUT` 鍜?`V4L2_PIX_FMT_Y41P`銆?

2000-11-25: `V4L2_TYPE_VBI_INPUT` was added.

2000-11-25锛氭柊澧炰簡 `V4L2_TYPE_VBI_INPUT`銆?

2000-12-04: A couple typos in symbol names were fixed.

2000-12-04锛氫慨姝ｄ簡绗﹀彿鍚嶄腑鐨勮嫢骞叉嫾鍐欓敊璇€?

2001-01-18: To avoid namespace conflicts the `fourcc` macro defined in
the `videodev.h` header file was renamed to `v4l2_fourcc`.

2001-01-18锛氫负閬垮厤鍛藉悕绌洪棿鍐茬獊锛宍videodev.h` 澶存枃浠朵腑瀹氫箟鐨?
`fourcc` 瀹忚閲嶅懡鍚嶄负 `v4l2_fourcc`銆?

2001-01-25: A possible driver-level compatibility problem between the
`videodev.h` file in Linux 2.4.0 and the `videodev.h` file included
in the `videodevX` patch was fixed. Users of an earlier version of
`videodevX` on Linux 2.4.0 should recompile their V4L and V4L2
drivers.

2001-01-25锛氫慨澶嶄簡 Linux 2.4.0 涓殑 `videodev.h` 鏂囦欢涓?
`videodevX` 琛ヤ竵涓墍鍖呭惈鐨?`videodev.h` 鏂囦欢涔嬮棿鍙兘瀛樺湪鐨?
椹卞姩绾у吋瀹规€ч棶棰樸€傚湪 Linux 2.4.0 涓婁娇鐢ㄨ緝鏃╃増鏈?
`videodevX` 鐨勭敤鎴峰簲閲嶆柊缂栬瘧鍏?V4L 鍜?V4L2
椹卞姩銆?

2001-01-26: A possible kernel-level incompatibility between the
`videodev.h` file in the `videodevX` patch and the `videodev.h`
file in Linux 2.2.x with devfs patches applied was fixed.

2001-01-26锛氫慨澶嶄簡 `videodevX` 琛ヤ竵涓殑 `videodev.h` 鏂囦欢
涓庢墦浜?devfs 琛ヤ竵鐨?Linux 2.2.x 涓殑 `videodev.h` 鏂囦欢涔嬮棿
鍙兘瀛樺湪鐨勫収鏍哥骇涓嶅吋瀹归棶棰樸€?

2001-03-02: Certain V4L ioctls which pass data in both direction
although they are defined with read-only parameter, did not work
correctly through the backward compatibility layer. [Solution?]

2001-03-02锛氭煇浜?V4L ioctl 浠ュ彧璇诲弬鏁板畾涔夛紝鍗翠細鍙屽悜浼犻€掓暟鎹紝
瀹冧滑閫氳繃鍚戝悗鍏煎灞傛椂鏃犳硶姝ｇ‘宸ヤ綔銆俒瑙ｅ喅鏂规锛焆

2001-04-13: Big endian 16-bit RGB formats were added.

2001-04-13锛氭柊澧炰簡澶х锛坆ig endian锛?6 浣?RGB 鏍煎紡銆?

2001-09-17: New YUV formats and the
VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> and
VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctls were added.
(The old `VIDIOC_G_FREQ` and `VIDIOC_S_FREQ` ioctls did not take
multiple tuners into account.)

2001-09-17锛氭柊澧炰簡鏂扮殑 YUV 鏍煎紡浠ュ強
VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> 鍜?
VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctls銆?
锛堟棫鐨?`VIDIOC_G_FREQ` 鍜?`VIDIOC_S_FREQ` ioctl 娌℃湁鑰冭檻
澶氫釜璋冭皭鍣ㄧ殑鎯呭喌銆傦級

2000-09-18: `V4L2_BUF_TYPE_VBI` was added. This may *break
compatibility* as the VIDIOC_G_FMT <VIDIOC_G_FMT> and
VIDIOC_S_FMT <VIDIOC_G_FMT> ioctls may fail now if the
struct `v4l2_fmt` `type` field does not contain
`V4L2_BUF_TYPE_VBI`. In the documentation of the struct v4l2_vbi_format`,
the `offset` field the ambiguous phrase "rising edge" was changed to
"leading edge".

2000-09-18锛氭柊澧炰簡 `V4L2_BUF_TYPE_VBI`銆傝繖鍙兘浼?鐮村潖
鍏煎鎬?锛屽洜涓哄鏋?struct `v4l2_fmt` 鐨?`type` 瀛楁涓嶅寘鍚?
`V4L2_BUF_TYPE_VBI`锛孷IDIOC_G_FMT <VIDIOC_G_FMT> 鍜?
VIDIOC_S_FMT <VIDIOC_G_FMT> ioctls 鐜板湪鍙兘浼氬け璐ャ€傚湪
struct v4l2_vbi_format 鐨勬枃妗ｄ腑锛宍offset` 瀛楁澶勫惈绯婄殑
鐭 "rising edge" 琚敼涓?"leading edge"銆?

## V4L2 Version 0.20 2000-11-23

## V4L2 0.20 鐗?2000-11-23


A number of changes were made to the raw VBI interface.

瀵瑰師濮?VBI 鎺ュ彛鍋氫簡鑻ュ共鏀瑰姩銆?

1. Figures clarifying the line numbering scheme were added to the V4L2
   API specification. The `start`\ [^0^] and `start`\ [^1^] fields no
   longer count line numbers beginning at zero. Rationale: a) The
   previous definition was unclear. b) The `start`\ [] values are
   ordinal numbers. c) There is no point in inventing a new line
   numbering scheme. We now use line number as defined by ITU-R, period.
   Compatibility: Add one to the start values. Applications depending on
   the previous semantics may not function correctly.

1. 鍦?V4L2 API 瑙勮寖涓柊澧炰簡鐢ㄤ簬闃愭槑琛岀紪鍙锋柟妗堢殑鍥剧ず銆俙start`\ [^0^]
   鍜?`start`\ [^1^] 瀛楁涓嶅啀浠庨浂寮€濮嬭鏁拌鍙枫€傜悊鐢憋細a) 涔嬪墠鐨勫畾涔?
   涓嶆竻銆俠) `start`\ [] 鐨勫€兼槸搴忔暟銆俢) 娌℃湁蹇呰鍙戞槑鏂扮殑琛?
   缂栧彿鏂规銆傜幇鍦ㄦ垜浠噰鐢?ITU-R 瀹氫箟鐨勮鍙凤紝浠呮鑰屽凡銆?
   鍏煎鎬э細闇€灏?start 鍊煎姞涓€銆備緷璧栧厛鍓嶈涔夌殑搴旂敤绋嬪簭鍙兘鏃犳硶
   姝ｅ父宸ヤ綔銆?

2. The restriction "count[^0^] > 0 and count[^1^] > 0" has been relaxed to
   "(count[^0^] + count[^1^]) > 0". Rationale: Drivers may allocate
   resources at scan line granularity and some data services are
   transmitted only on the first field. The comment that both `count`
   values will usually be equal is misleading and pointless and has been
   removed. This change **breaks compatibility** with earlier versions:
   Drivers may return `EINVAL`, applications may not function correctly.

2. 闄愬埗 "count[^0^] > 0 涓?count[^1^] > 0" 宸叉斁瀹借嚦
   "(count[^0^] + count[^1^]) > 0"銆傜悊鐢憋細椹卞姩鍙兘浠ユ壂鎻忚涓虹矑搴?
   鍒嗛厤璧勬簮锛岃€屾煇浜涙暟鎹湇鍔′粎鍦ㄧ涓€涓満锛坒ield锛変笂浼犺緭銆傚叧浜庝袱涓?
   `count` 鍊奸€氬父鐩哥瓑鐨勬敞閲婂叿鏈夎瀵兼€т笖鏃犳剰涔夛紝宸茶绉婚櫎銆傝繖涓€鏀瑰姩
   **鐮村潖浜嗕笌鏃╂湡鐗堟湰鐨勫吋瀹规€?*锛氶┍鍔ㄥ彲鑳借繑鍥?`EINVAL`锛?
   搴旂敤绋嬪簭鍙兘鏃犳硶姝ｅ父宸ヤ綔銆?

3. Drivers are again permitted to return negative (unknown) start values
   as proposed earlier. Why this feature was dropped is unclear. This
   change may **break compatibility** with applications depending on the
   start values being positive. The use of `EBUSY` and `EINVAL`
   error codes with the VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl was
   clarified. The `EBUSY` error code was finally documented, and the
   `reserved2` field which was previously mentioned only in the
   `videodev.h` header file.

3. 椹卞姩鍐嶆琚厑璁歌繑鍥炶礋鐨勶紙鏈煡鐨勶級start 鍊硷紝姝ｅ鏃╁厛鎵€寤鸿鐨勩€?
   涓嶆竻妤氬綋鍒濅负浣曞幓鎺変簡杩欎竴鐗规€с€傝繖涓€鏀瑰姩鍙兘**鐮村潖涓庝緷璧?
   姝ｇ殑 start 鍊肩殑搴旂敤绋嬪簭鐨勫吋瀹规€?*銆傛緞娓呬簡 `EBUSY` 鍜?`EINVAL`
   閿欒鐮佷笌 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鐨勪娇鐢ㄦ柟寮忋€俙EBUSY`
   閿欒鐮佺粓浜庡緱鍒颁簡鏂囨。璇存槑锛岃€屾鍓嶄粎鍦?`videodev.h` 澶存枃浠朵腑
   鎻愬強鐨?`reserved2` 瀛楁涔熷緱鍒颁簡璇存槑銆?

4. New buffer types `V4L2_TYPE_VBI_INPUT` and `V4L2_TYPE_VBI_OUTPUT`
   were added. The former is an alias for the old `V4L2_TYPE_VBI`, the
   latter was missing in the `videodev.h` file.

4. 鏂板浜嗘柊鐨勭紦鍐插尯绫诲瀷 `V4L2_TYPE_VBI_INPUT` 鍜?
   `V4L2_TYPE_VBI_OUTPUT`銆傚墠鑰呮槸鏃х殑 `V4L2_TYPE_VBI` 鐨勫埆鍚嶏紝
   鍚庤€呭垯鏄?`videodev.h` 鏂囦欢涓己澶辩殑銆?

## V4L2 Version 0.20 2002-07-25

## V4L2 0.20 鐗?2002-07-25


Added sliced VBI interface proposal.

鏂板浜嗗垎鐗囷紙sliced锛塚BI 鎺ュ彛鎻愭銆?

## V4L2 in Linux 2.5.46, 2002-10

## Linux 2.5.46 涓殑 V4L2锛?002-10


Around October-November 2002, prior to an announced feature freeze of
Linux 2.5, the API was revised, drawing from experience with V4L2 0.20.
This unnamed version was finally merged into Linux 2.5.46.

鍦?2002 骞?10 鏈堣嚦 11 鏈堝墠鍚庯紝鍦?Linux 2.5 瀹ｅ竷鐗规€у喕缁撲箣鍓嶏紝
璇?API 鍊熼壌 V4L2 0.20 鐨勭粡楠岃繘琛屼簡淇銆傝繖涓湭鍛藉悕鐨勭増鏈渶缁?
琚悎骞惰繘浜?Linux 2.5.46銆?

1. As specified in related, drivers must make related device
    functions available under all minor device numbers.

1. 濡傜浉鍏崇珷鑺傛墍瑙勫畾锛岄┍鍔ㄥ繀椤诲湪鎵€鏈夋璁惧鍙蜂笅鎻愪緵鐩稿叧鐨勮澶?
   鍔熻兘銆?

2. The `open()` function requires access mode
    `O_RDWR` regardless of the device type. All V4L2 drivers
    exchanging data with applications must support the `O_NONBLOCK`
    flag. The `O_NOIO` flag, a V4L2 symbol which aliased the
    meaningless `O_TRUNC` to indicate accesses without data exchange
    (panel applications) was dropped. Drivers must stay in "panel mode"
    until the application attempts to initiate a data exchange, see
    open.

2. 鏃犺璁惧绫诲瀷濡備綍锛宍open()` 鍑芥暟閮借姹備娇鐢?
    `O_RDWR` 璁块棶妯″紡銆傛墍鏈変笌搴旂敤绋嬪簭浜ゆ崲鏁版嵁鐨?V4L2 椹卞姩蹇呴』
    鏀寔 `O_NONBLOCK` 鏍囧織銆傛浘缁忎綔涓?V4L2 绗﹀彿銆佸皢鏃犳剰涔夌殑
    `O_TRUNC` 鍒悕涓鸿〃绀烘棤鏁版嵁浜ゆ崲璁块棶锛堥潰鏉垮簲鐢ㄧ▼搴忥級鐨?
    `O_NOIO` 鏍囧織琚幓鎺変簡銆傞┍鍔ㄥ繀椤讳繚鎸佸湪"闈㈡澘妯″紡"
    鐩村埌搴旂敤绋嬪簭灏濊瘯鍙戣捣鏁版嵁浜ゆ崲锛岃瑙?open銆?

3. The struct v4l2_capability changed
    dramatically. Note that also the size of the structure changed,
    which is encoded in the ioctl request code, thus older V4L2 devices
    will respond with an `EINVAL` error code to the new
    VIDIOC_QUERYCAP ioctl.

3. struct v4l2_capability 鍙戠敓浜?
    宸ㄥぇ鍙樺寲銆傛敞鎰忚缁撴瀯鐨勫ぇ灏忎篃鏀瑰彉浜嗭紝鑰岃繖琚紪鐮佽繘 ioctl 璇锋眰
    鐮佷腑锛屽洜姝よ緝鏃х殑 V4L2 璁惧浼氫互 `EINVAL` 閿欒鐮佹潵鍝嶅簲鏂扮殑
    VIDIOC_QUERYCAP ioctl銆?

    There are new fields to identify the driver, a new RDS device
    function `V4L2_CAP_RDS_CAPTURE`, the `V4L2_CAP_AUDIO` flag
    indicates if the device has any audio connectors, another I/O
    capability V4L2_CAP_ASYNCIO can be flagged. In response to these
    changes the `type` field became a bit set and was merged into the
    `flags` field. `V4L2_FLAG_TUNER` was renamed to
    `V4L2_CAP_TUNER`, `V4L2_CAP_VIDEO_OVERLAY` replaced
    `V4L2_FLAG_PREVIEW` and `V4L2_CAP_VBI_CAPTURE` and
    `V4L2_CAP_VBI_OUTPUT` replaced `V4L2_FLAG_DATA_SERVICE`.
    `V4L2_FLAG_READ` and `V4L2_FLAG_WRITE` were merged into
    `V4L2_CAP_READWRITE`.

    鏂板浜嗙敤浜庢爣璇嗛┍鍔ㄧ▼搴忕殑瀛楁銆佹柊鐨?RDS 璁惧鍔熻兘
    `V4L2_CAP_RDS_CAPTURE`锛沗V4L2_CAP_AUDIO` 鏍囧織
    鎸囩ず璁惧鏄惁鍏锋湁闊抽杩炴帴鍣紱鍙︿竴涓?I/O 鑳藉姏 V4L2_CAP_ASYNCIO 涔?
    鍙互琚爣璁般€備綔涓哄杩欎簺鏀瑰姩鐨勫搷搴旓紝`type` 瀛楁鍙樻垚浜嗕竴涓綅闆嗗悎锛?
    骞惰鍚堝苟杩?`flags` 瀛楁銆俙V4L2_FLAG_TUNER` 琚噸鍛藉悕涓?
    `V4L2_CAP_TUNER`锛宍V4L2_CAP_VIDEO_OVERLAY` 鍙栦唬浜?
    `V4L2_FLAG_PREVIEW`锛宍V4L2_CAP_VBI_CAPTURE` 鍜?
    `V4L2_CAP_VBI_OUTPUT` 鍙栦唬浜?`V4L2_FLAG_DATA_SERVICE`銆?
    `V4L2_FLAG_READ` 鍜?`V4L2_FLAG_WRITE` 琚悎骞朵负
    `V4L2_CAP_READWRITE`銆?

    The redundant fields `inputs`, `outputs` and `audios` were
    removed. These properties can be determined as described in
    video and audio.

    鍐椾綑鐨?`inputs`銆乣outputs` 鍜?`audios` 瀛楁琚?
    绉婚櫎銆傝繖浜涘睘鎬у彲鎸?video 鍜?audio 绔犺妭鎵€杩扮殑鏂瑰紡纭畾銆?

    The somewhat volatile and therefore barely useful fields
    `maxwidth`, `maxheight`, `minwidth`, `minheight`,
    `maxframerate` were removed. This information is available as
    described in format and standard.

    閭ｄ簺涓嶅お绋冲畾銆佸洜鑰屽嚑涔庢棤鐢ㄧ殑瀛楁
    `maxwidth`銆乣maxheight`銆乣minwidth`銆乣minheight`銆?
    `maxframerate` 琚Щ闄ゃ€傝繖浜涗俊鎭彲鎸?format 鍜?standard 绔犺妭
    鎵€杩扮殑鏂瑰紡鑾峰彇銆?

    `V4L2_FLAG_SELECT` was removed. We believe the select() function
    is important enough to require support of it in all V4L2 drivers
    exchanging data with applications. The redundant
    `V4L2_FLAG_MONOCHROME` flag was removed, this information is
    available as described in format.

    `V4L2_FLAG_SELECT` 琚Щ闄ゃ€傛垜浠涓?select() 鍑芥暟
    闈炲父閲嶈锛岃姹傛墍鏈変笌搴旂敤绋嬪簭浜ゆ崲鏁版嵁鐨?V4L2 椹卞姩閮芥敮鎸佸畠銆?
    鍐椾綑鐨?`V4L2_FLAG_MONOCHROME` 鏍囧織琚Щ闄わ紝璇ヤ俊鎭彲鎸?
    format 绔犺妭鎵€杩扮殑鏂瑰紡鑾峰彇銆?

4. In struct v4l2_input the `assoc_audio`
    field and the `capability` field and its only flag
    `V4L2_INPUT_CAP_AUDIO` was replaced by the new `audioset` field.
    Instead of linking one video input to one audio input this field
    reports all audio inputs this video input combines with.

4. 鍦?struct v4l2_input 涓紝`assoc_audio`
    瀛楁浠ュ強 `capability` 瀛楁鍙婂叾鍞竴鐨勬爣蹇?
    `V4L2_INPUT_CAP_AUDIO` 琚柊鐨?`audioset` 瀛楁鍙栦唬銆?
    璇ュ瓧娈典笉鍐嶅皢涓€涓棰戣緭鍏ュ叧鑱斿埌鍗曚釜闊抽杈撳叆锛岃€屾槸鎶ュ憡
    璇ヨ棰戣緭鍏ユ墍缁勫悎鐨勬墍鏈夐煶棰戣緭鍏ャ€?

    New fields are `tuner` (reversing the former link from tuners to
    video inputs), `std` and `status`.

    鏂板浜?`tuner`锛堝弽杞簡鍘熷厛浠庤皟璋愬櫒鍒拌棰戣緭鍏ョ殑鍏宠仈锛夈€?
    `std` 鍜?`status` 瀛楁銆?

    Accordingly struct v4l2_output lost its
    `capability` and `assoc_audio` fields. `audioset`,
    `modulator` and `std` where added instead.

    鐩稿簲鍦帮紝struct v4l2_output 澶卞幓浜嗗叾
    `capability` 鍜?`assoc_audio` 瀛楁銆傚彇鑰屼唬涔嬫柊澧炰簡
    `audioset`銆乣modulator` 鍜?`std`銆?

5. The struct v4l2_audio field `audio` was
    renamed to `index`, for consistency with other structures. A new
    capability flag `V4L2_AUDCAP_STEREO` was added to indicated if the
    audio input in question supports stereo sound.
    `V4L2_AUDCAP_EFFECTS` and the corresponding `V4L2_AUDMODE` flags
    where removed. This can be easily implemented using controls.
    (However the same applies to AVL which is still there.)

5. 涓轰笌鍏朵粬缁撴瀯淇濇寔涓€鑷达紝struct v4l2_audio 鐨?`audio` 瀛楁琚?
    閲嶅懡鍚嶄负 `index`銆傛柊澧炰簡涓€涓兘鍔涙爣蹇?`V4L2_AUDCAP_STEREO`
    浠ユ寚绀虹浉鍏抽煶棰戣緭鍏ユ槸鍚︽敮鎸佺珛浣撳０銆俙V4L2_AUDCAP_EFFECTS` 鍙?
    鐩稿簲鐨?`V4L2_AUDMODE` 鏍囧織琚Щ闄ゃ€傝繖鍙交鏄撳湴浣跨敤鎺у埗椤?
    瀹炵幇銆傦紙涓嶈繃 AVL 鐨勬儏鍐电被浼硷紝浣嗗畠浠嶇劧瀛樺湪銆傦級

    Again for consistency the struct v4l2_audioout field `audio` was renamed
    to `index`.

    鍚屾牱涓轰簡淇濇寔涓€鑷达紝struct v4l2_audioout 鐨?`audio` 瀛楁琚噸鍛藉悕涓?
    `index`銆?

6. The struct v4l2_tuner `input` field was
    replaced by an `index` field, permitting devices with multiple
    tuners. The link between video inputs and tuners is now reversed,
    inputs point to their tuner. The `std` substructure became a
    simple set (more about this below) and moved into struct v4l2_input.
    A `type` field was added.

6. struct v4l2_tuner 鐨?`input` 瀛楁琚?
    涓€涓?`index` 瀛楁鍙栦唬锛屼粠鑰屾敮鎸佸叿鏈夊涓皟璋愬櫒鐨勮澶囥€傝棰戣緭鍏?
    涓庤皟璋愬櫒涔嬮棿鐨勫叧鑱旂幇鍦ㄨ鍙嶈浆锛岃緭鍏ユ寚鍚戝叾璋冭皭鍣ㄣ€俙std` 瀛愮粨鏋?
    鍙樹负涓€涓畝鍗曠殑闆嗗悎锛堣瑙佷笅鏂囷級骞剁Щ鍏?struct v4l2_input銆?
    鏂板浜嗕竴涓?`type` 瀛楁銆?

    Accordingly in struct v4l2_modulator the
    `output` was replaced by an `index` field.

    鐩稿簲鍦帮紝鍦?struct v4l2_modulator 涓紝`output` 琚竴涓?
    `index` 瀛楁鍙栦唬銆?

    In struct v4l2_frequency the `port`
    field was replaced by a `tuner` field containing the respective
    tuner or modulator index number. A tuner `type` field was added
    and the `reserved` field became larger for future extensions
    (satellite tuners in particular).

    鍦?struct v4l2_frequency 涓紝`port`
    瀛楁琚竴涓寘鍚浉搴旇皟璋愬櫒鎴栬皟鍒跺櫒绱㈠紩鍙风殑 `tuner` 瀛楁鍙栦唬銆?
    鏂板浜嗚皟璋愬櫒 `type` 瀛楁锛屽苟涓?`reserved` 瀛楁琚墿澶т互
    渚垮皢鏉ユ墿灞曪紙灏ゅ叾鏄崼鏄熻皟璋愬櫒锛夈€?

7. The idea of completely transparent video standards was dropped.
    Experience showed that applications must be able to work with video
    standards beyond presenting the user a menu. Instead of enumerating
    supported standards with an ioctl applications can now refer to
    standards by v4l2_std_id <v4l2-std-id> and symbols
    defined in the `videodev2.h` header file. For details see
    standard. The VIDIOC_G_STD <VIDIOC_G_STD> and
    VIDIOC_S_STD <VIDIOC_G_STD> now take a pointer to this
    type as argument. VIDIOC_QUERYSTD was
    added to autodetect the received standard, if the hardware has this
    capability. In struct v4l2_standard an
    `index` field was added for
    VIDIOC_ENUMSTD. A
    v4l2_std_id <v4l2-std-id> field named `id` was added as
    machine readable identifier, also replacing the `transmission`
    field. The misleading `framerate` field was renamed to
    `frameperiod`. The now obsolete `colorstandard` information,
    originally needed to distguish between variations of standards, were
    removed.

7. 瀹屽叏閫忔槑鐨勮棰戞爣鍑嗚繖涓€璁炬兂琚斁寮冧簡銆傜粡楠岃〃鏄庯紝搴旂敤绋嬪簭蹇呴』
    鑳藉瓒呰秺"鍚戠敤鎴峰睍绀鸿彍鍗?鏉ヤ笌瑙嗛鏍囧噯鎵撲氦閬撱€傜幇鍦紝搴旂敤绋嬪簭涓嶅啀
    鐢?ioctl 鏋氫妇鎵€鏀寔鐨勬爣鍑嗭紝鑰屾槸鍙互閫氳繃 `videodev2.h` 澶存枃浠朵腑
    瀹氫箟鐨?v4l2_std_id <v4l2-std-id> 鍜岀鍙锋潵寮曠敤鏍囧噯銆傝鎯?
    瑙?standard銆俈IDIOC_G_STD <VIDIOC_G_STD> 鍜?
    VIDIOC_S_STD <VIDIOC_G_STD> 鐜板湪浠ユ寚鍚戣绫诲瀷鐨勬寚閽堜綔涓哄弬鏁般€?
    鏂板浜?VIDIOC_QUERYSTD锛岀敤浜庡湪纭欢鏀寔鐨勬儏鍐典笅
    鑷姩妫€娴嬫墍鎺ユ敹鐨勬爣鍑嗐€傚湪 struct v4l2_standard 涓柊澧炰簡
    `index` 瀛楁鐢ㄤ簬
    VIDIOC_ENUMSTD銆傛柊澧炰簡涓€涓悕涓?`id` 鐨?
    v4l2_std_id <v4l2-std-id> 瀛楁浣滀负鏈哄櫒鍙鏍囪瘑绗︼紝鍚屾椂
    鍙栦唬浜?`transmission` 瀛楁銆傚叿鏈夎瀵兼€х殑 `framerate` 瀛楁
    琚噸鍛藉悕涓?`frameperiod`銆傜幇宸茶繃鏃剁殑 `colorstandard` 淇℃伅
    锛堟渶鍒濈敤浜庡尯鍒嗘爣鍑嗙殑涓嶅悓鍙樹綋锛夎绉婚櫎銆?

    Struct `v4l2_enumstd` ceased to be.
    VIDIOC_ENUMSTD now takes a pointer to a
    struct v4l2_standard directly. The
    information which standards are supported by a particular video
    input or output moved into struct v4l2_input
    and struct v4l2_output fields named `std`,
    respectively.

    struct `v4l2_enumstd` 涓嶅瀛樺湪銆俈IDIOC_ENUMSTD 鐜板湪鐩存帴
    浠ユ寚鍚?struct v4l2_standard 鐨勬寚閽堜綔涓哄弬鏁般€傛煇鐗瑰畾瑙嗛
    杈撳叆鎴栬緭鍑烘墍鏀寔鐨勬爣鍑嗚繖涓€淇℃伅鍒嗗埆绉诲叆浜嗗悕涓?`std` 鐨?
    struct v4l2_input 鍜?struct v4l2_output 瀛楁銆?

8. The struct v4l2_queryctrl <v4l2-queryctrl> fields
    `category` and `group` did not catch on and/or were not
    implemented as expected and therefore removed.

8. struct v4l2_queryctrl <v4l2-queryctrl> 鐨?
    `category` 鍜?`group` 瀛楁鏈骞挎硾閲囩敤鍜?鎴栨湭濡傞鏈?
    瀹炵幇锛屽洜姝よ绉婚櫎銆?

9. The VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl was added to
    negotiate data formats as with
    VIDIOC_S_FMT <VIDIOC_G_FMT>, but without the overhead of
    programming the hardware and regardless of I/O in progress.

9. 鏂板浜?VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl锛岀敤浜?
    鍍?VIDIOC_S_FMT <VIDIOC_G_FMT> 閭ｆ牱鍗忓晢鏁版嵁鏍煎紡锛屼絾涓?
    浜х敓缂栫▼纭欢鐨勫紑閿€锛屼笖涓嶅彈杩涜涓殑 I/O 褰卞搷銆?

    In struct v4l2_format the `fmt` union was
    extended to contain struct v4l2_window. All
    image format negotiations are now possible with `VIDIOC_G_FMT`,
    `VIDIOC_S_FMT` and `VIDIOC_TRY_FMT`; ioctl. The `VIDIOC_G_WIN`
    and `VIDIOC_S_WIN` ioctls to prepare for a video overlay were
    removed. The `type` field changed to type enum v4l2_buf_type and
    the buffer type names changed as follows.

    鍦?struct v4l2_format 涓紝`fmt` 鑱斿悎琚?
    鎵╁睍涓哄寘鍚?struct v4l2_window銆傜幇鍦ㄦ墍鏈夊浘鍍忔牸寮忓崗鍟嗛兘鍙互閫氳繃
    `VIDIOC_G_FMT`銆乣VIDIOC_S_FMT` 鍜?`VIDIOC_TRY_FMT` ioctl
    瀹屾垚銆俙VIDIOC_G_WIN` 鍜?`VIDIOC_S_WIN` 杩欎袱涓敤浜庡噯澶囪棰?
    鍙犲姞锛坥verlay锛夌殑 ioctl 琚Щ闄ゃ€俙type` 瀛楁鏀逛负 enum v4l2_buf_type
    绫诲瀷锛岀紦鍐插尯绫诲瀷鍚嶇О鏀瑰姩濡備笅銆?


```
	:header-rows:  1
	:stub-columns: 0

	* - Old defines
	  - enum v4l2_buf_type
	* - ``V4L2_BUF_TYPE_CAPTURE``
	  - ``V4L2_BUF_TYPE_VIDEO_CAPTURE``
	* - ``V4L2_BUF_TYPE_CODECIN``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_CODECOUT``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_EFFECTSIN``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_EFFECTSIN2``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_EFFECTSOUT``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_VIDEOOUT``
	  - ``V4L2_BUF_TYPE_VIDEO_OUTPUT``
	* - ``-``
	  - ``V4L2_BUF_TYPE_VIDEO_OVERLAY``
	* - ``-``
	  - ``V4L2_BUF_TYPE_VBI_CAPTURE``
	* - ``-``
	  - ``V4L2_BUF_TYPE_VBI_OUTPUT``
	* - ``-``
	  - ``V4L2_BUF_TYPE_SLICED_VBI_CAPTURE``
	* - ``-``
	  - ``V4L2_BUF_TYPE_SLICED_VBI_OUTPUT``
	* - ``V4L2_BUF_TYPE_PRIVATE_BASE``
	  - ``V4L2_BUF_TYPE_PRIVATE`` (but this is deprecated)

```

10. In struct v4l2_fmtdesc a enum v4l2_buf_type field named `type` was
    added as in struct v4l2_format. The `VIDIOC_ENUM_FBUFFMT` ioctl is no
    longer needed and was removed. These calls can be replaced by
    VIDIOC_ENUM_FMT with type `V4L2_BUF_TYPE_VIDEO_OVERLAY`.

10. 鍦?struct v4l2_fmtdesc 涓紝鏂板浜嗕竴涓悕涓?`type` 鐨?
    enum v4l2_buf_type 瀛楁锛屼笌 struct v4l2_format 涓浉鍚屻€?
    `VIDIOC_ENUM_FBUFFMT` ioctl 涓嶅啀闇€瑕侊紝宸茶绉婚櫎銆傝繖浜涜皟鐢ㄥ彲琚?
    浣跨敤 `V4L2_BUF_TYPE_VIDEO_OVERLAY` 绫诲瀷鐨?VIDIOC_ENUM_FMT 鍙栦唬銆?

11. In struct v4l2_pix_format the `depth`
    field was removed, assuming applications which recognize the format
    by its four-character-code already know the color depth, and others
    do not care about it. The same rationale lead to the removal of the
    `V4L2_FMT_FLAG_COMPRESSED` flag. The
    `V4L2_FMT_FLAG_SWCONVECOMPRESSED` flag was removed because drivers
    are not supposed to convert images in kernel space. A user library
    of conversion functions should be provided instead. The
    `V4L2_FMT_FLAG_BYTESPERLINE` flag was redundant. Applications can
    set the `bytesperline` field to zero to get a reasonable default.
    Since the remaining flags were replaced as well, the `flags` field
    itself was removed.

11. 鍦?struct v4l2_pix_format 涓紝`depth`
    瀛楁琚Щ闄わ紝鍥犱负鍋囧畾閭ｄ簺閫氳繃鍥涘瓧绗︾爜锛坒our-character-code锛夎瘑鍒?
    鏍煎紡鐨勫簲鐢ㄧ▼搴忓凡缁忕煡閬撻鑹叉繁搴︼紝鑰屽叾浠栧簲鐢ㄧ▼搴忓苟涓嶅叧蹇冨畠銆傚悓鏍风殑
    鐞嗙敱瀵艰嚧浜?`V4L2_FMT_FLAG_COMPRESSED` 鏍囧織鐨勭Щ闄ゃ€?
    `V4L2_FMT_FLAG_SWCONVECOMPRESSED` 鏍囧織琚Щ闄わ紝鍥犱负椹卞姩涓嶅簲鍦?
    鍐呮牳绌洪棿杞崲鍥惧儚銆傚簲鏀逛负鎻愪緵涓€涓敤鎴锋€佺殑杞崲鍑芥暟搴撱€?
    `V4L2_FMT_FLAG_BYTESPERLINE` 鏍囧織鏄啑浣欑殑銆傚簲鐢ㄧ▼搴忓彲浠ュ皢
    `bytesperline` 瀛楁璁句负闆朵互鑾峰緱鍚堢悊鐨勯粯璁ゅ€笺€傜敱浜庡叾浣欐爣蹇椾篃
    琚浛鎹簡锛屽洜姝?`flags` 瀛楁鏈韩涔熻绉婚櫎銆?

    The interlace flags were replaced by a enum v4l2_field value in a
    newly added `field` field.

    闅旇锛坕nterlace锛夋爣蹇楄鏂板鍔犵殑 `field` 瀛楁涓殑
    enum v4l2_field 鍊兼墍鍙栦唬銆?


```
	:header-rows:  1
	:stub-columns: 0

	* - Old flag
	  - enum v4l2_field
	* - ``V4L2_FMT_FLAG_NOT_INTERLACED``
	  - ?
	* - ``V4L2_FMT_FLAG_INTERLACED`` = ``V4L2_FMT_FLAG_COMBINED``
	  - ``V4L2_FIELD_INTERLACED``
	* - ``V4L2_FMT_FLAG_TOPFIELD`` = ``V4L2_FMT_FLAG_ODDFIELD``
	  - ``V4L2_FIELD_TOP``
	* - ``V4L2_FMT_FLAG_BOTFIELD`` = ``V4L2_FMT_FLAG_EVENFIELD``
	  - ``V4L2_FIELD_BOTTOM``
	* - ``-``
	  - ``V4L2_FIELD_SEQ_TB``
	* - ``-``
	  - ``V4L2_FIELD_SEQ_BT``
	* - ``-``
	  - ``V4L2_FIELD_ALTERNATE``

    The color space flags were replaced by a enum v4l2_colorspace value in
    a newly added ``colorspace`` field, where one of
    ``V4L2_COLORSPACE_SMPTE170M``, ``V4L2_COLORSPACE_BT878``,
    ``V4L2_COLORSPACE_470_SYSTEM_M`` or
    ``V4L2_COLORSPACE_470_SYSTEM_BG`` replaces ``V4L2_FMT_CS_601YUV``.

    棰滆壊绌洪棿锛坈olor space锛夋爣蹇楄鏂板鍔犵殑 ``colorspace`` 瀛楁涓殑
    enum v4l2_colorspace 鍊兼墍鍙栦唬锛屽叾涓?
    ``V4L2_COLORSPACE_SMPTE170M``銆乣`V4L2_COLORSPACE_BT878``銆?
    ``V4L2_COLORSPACE_470_SYSTEM_M`` 鎴?
    ``V4L2_COLORSPACE_470_SYSTEM_BG`` 涔嬩竴鍙栦唬浜?
    ``V4L2_FMT_CS_601YUV``銆?

```

12. In struct v4l2_requestbuffers the
    `type` field was properly defined as enum v4l2_buf_type. Buffer types
    changed as mentioned above. A new `memory` field of type
    enum v4l2_memory was added to distinguish between
    I/O methods using buffers allocated by the driver or the
    application. See io for details.

12. 鍦?struct v4l2_requestbuffers 涓紝`type` 瀛楁琚纭畾涔変负
    enum v4l2_buf_type銆傜紦鍐插尯绫诲瀷濡傚墠鎵€杩板彂鐢熶簡鍙樺寲銆傛柊澧炰簡涓€涓?
    enum v4l2_memory 绫诲瀷鐨?`memory` 瀛楁锛岀敤浜庡尯鍒?
    浣跨敤椹卞姩鍒嗛厤杩樻槸搴旂敤绋嬪簭鍒嗛厤鐨勭紦鍐插尯鐨?I/O 鏂规硶銆傝瑙?io銆?

13. In struct v4l2_buffer the `type` field was
    properly defined as enum v4l2_buf_type.
    Buffer types changed as mentioned above. A `field` field of type
    enum v4l2_field was added to indicate if a
    buffer contains a top or bottom field. The old field flags were
    removed. Since no unadjusted system time clock was added to the
    kernel as planned, the `timestamp` field changed back from type
    stamp_t, an unsigned 64 bit integer expressing the sample time in
    nanoseconds, to struct timeval. With the addition
    of a second memory mapping method the `offset` field moved into
    union `m`, and a new `memory` field of type enum v4l2_memory
    was added to distinguish between
    I/O methods. See io for details.

13. 鍦?struct v4l2_buffer 涓紝`type` 瀛楁琚纭畾涔変负
    enum v4l2_buf_type銆傜紦鍐插尯绫诲瀷濡傚墠鎵€杩板彂鐢熶簡鍙樺寲銆傛柊澧炰簡涓€涓?
    enum v4l2_field 绫诲瀷鐨?`field` 瀛楁锛岀敤浜庢寚绀虹紦鍐插尯鍖呭惈鐨勬槸
    椤跺満锛坱op锛夎繕鏄簳鍦猴紙bottom锛夈€傛棫鐨勫満鏍囧織琚Щ闄ゃ€傜敱浜庡唴鏍镐腑
    骞舵湭鎸夊師璁″垝鍔犲叆鏈牎姝ｇ郴缁熸椂闂存椂閽燂紝`timestamp` 瀛楁浠?
    琛ㄧず閲囨牱鏃堕棿锛堢撼绉掞級鐨勬棤绗﹀彿 64 浣嶆暣鏁?stamp_t 绫诲瀷鏀瑰洖浜?
    struct timeval銆傞殢鐫€绗簩绉嶅唴瀛樻槧灏勬柟娉曠殑鍔犲叆锛宍offset`
    瀛楁绉诲叆浜嗚仈鍚?`m`锛屽苟鏂板浜嗕竴涓?enum v4l2_memory 绫诲瀷鐨?
    `memory` 瀛楁鐢ㄤ簬鍖哄垎涓嶅悓鐨?I/O 鏂规硶銆傝瑙?io銆?

    The `V4L2_BUF_REQ_CONTIG` flag was used by the V4L compatibility
    layer, after changes to this code it was no longer needed. The
    `V4L2_BUF_ATTR_DEVICEMEM` flag would indicate if the buffer was
    indeed allocated in device memory rather than DMA-able system
    memory. It was barely useful and so was removed.

    `V4L2_BUF_REQ_CONTIG` 鏍囧織鏇剧敱 V4L 鍏煎灞備娇鐢紝鍦ㄥ璇ヤ唬鐮?
    杩涜鏀瑰姩鍚庝笉鍐嶉渶瑕併€傝€?`V4L2_BUF_ATTR_DEVICEMEM` 鏍囧織鐢ㄤ簬鎸囩ず
    缂撳啿鍖烘槸鍚︾‘瀹炲垎閰嶅湪璁惧鍐呭瓨鑰岄潪鍙?DMA 鐨勭郴缁熷唴瀛樹腑銆傚畠鍑犱箮
    娌℃湁鐢ㄥ锛屽洜姝よ绉婚櫎銆?

14. In struct v4l2_framebuffer the
    `base[^3^]` array anticipating double- and triple-buffering in
    off-screen video memory, however without defining a synchronization
    mechanism, was replaced by a single pointer. The
    `V4L2_FBUF_CAP_SCALEUP` and `V4L2_FBUF_CAP_SCALEDOWN` flags were
    removed. Applications can determine this capability more accurately
    using the new cropping and scaling interface. The
    `V4L2_FBUF_CAP_CLIPPING` flag was replaced by
    `V4L2_FBUF_CAP_LIST_CLIPPING` and
    `V4L2_FBUF_CAP_BITMAP_CLIPPING`.

14. 鍦?struct v4l2_framebuffer 涓紝閭ｄ釜棰勬湡鐢ㄤ簬绂诲睆瑙嗛鍐呭瓨涓?
    鍙岀紦鍐插拰涓夌紦鍐层€佸嵈鏈畾涔夊悓姝ユ満鍒剁殑 `base[^3^]` 鏁扮粍琚崟涓?
    鎸囬拡鍙栦唬銆俙V4L2_FBUF_CAP_SCALEUP` 鍜?`V4L2_FBUF_CAP_SCALEDOWN`
    鏍囧織琚Щ闄ゃ€傚簲鐢ㄧ▼搴忓彲閫氳繃鏂扮殑瑁佸壀鍜岀缉鏀炬帴鍙ｆ洿鍑嗙‘鍦扮‘瀹氳繖涓€
    鑳藉姏銆俙V4L2_FBUF_CAP_CLIPPING` 鏍囧織琚?
    `V4L2_FBUF_CAP_LIST_CLIPPING` 鍜?
    `V4L2_FBUF_CAP_BITMAP_CLIPPING` 鍙栦唬銆?

15. In struct v4l2_clip the `x`, `y`,
    `width` and `height` field moved into a `c` substructure of
    type struct v4l2_rect. The `x` and `y`
    fields were renamed to `left` and `top`, i. e. offsets to a
    context dependent origin.

15. 鍦?struct v4l2_clip 涓紝`x`銆乣y`銆?
    `width` 鍜?`height` 瀛楁绉诲叆浜?struct v4l2_rect 绫诲瀷鐨?
    `c` 瀛愮粨鏋勩€傚叾涓?`x` 鍜?`y`
    瀛楁琚噸鍛藉悕涓?`left` 鍜?`top`锛屽嵆鐩稿浜?
    涓婁笅鏂囩浉鍏冲師鐐圭殑鍋忕Щ閲忋€?

16. In struct v4l2_window the `x`, `y`,
    `width` and `height` field moved into a `w` substructure as
    above. A `field` field of type enum v4l2_field was added to
    distinguish between field and frame (interlaced) overlay.

16. 鍦?struct v4l2_window 涓紝`x`銆乣y`銆?
    `width` 鍜?`height` 瀛楁濡備笂鎵€杩扮Щ鍏ヤ簡 `w` 瀛愮粨鏋勩€傛柊澧炰簡
    涓€涓?enum v4l2_field 绫诲瀷鐨?`field` 瀛楁锛岀敤浜庡尯鍒?
    鍦猴紙field锛夊拰甯э紙frame锛屽嵆闅旇锛夊彔鍔犮€?

17. The digital zoom interface, including struct `v4l2_zoomcap`,
    struct `v4l2_zoom`, `V4L2_ZOOM_NONCAP` and
    `V4L2_ZOOM_WHILESTREAMING` was replaced by a new cropping and
    scaling interface. The previously unused
    struct v4l2_cropcap and struct v4l2_crop
    where redefined for this purpose. See crop for details.

17. 鏁板瓧缂╂斁鎺ュ彛锛堝寘鎷?struct `v4l2_zoomcap`銆乻truct `v4l2_zoom`銆?
    `V4L2_ZOOM_NONCAP` 鍜?`V4L2_ZOOM_WHILESTREAMING`锛夎鏂扮殑
    瑁佸壀鍜岀缉鏀炬帴鍙ｅ彇浠ｃ€傛鍓嶆湭浣跨敤鐨?struct v4l2_cropcap 鍜?
    struct v4l2_crop 涓烘琚噸鏂板畾涔夈€傝瑙?crop銆?

18. In struct v4l2_vbi_format the
    `SAMPLE_FORMAT` field now contains a four-character-code as used
    to identify video image formats and `V4L2_PIX_FMT_GREY` replaces
    the `V4L2_VBI_SF_UBYTE` define. The `reserved` field was
    extended.

18. 鍦?struct v4l2_vbi_format 涓紝`SAMPLE_FORMAT` 瀛楁鐜板湪鍖呭惈
    涓€涓敤浜庢爣璇嗚棰戝浘鍍忔牸寮忕殑鍥涘瓧绗︾爜锛屼笖 `V4L2_PIX_FMT_GREY`
    鍙栦唬浜?`V4L2_VBI_SF_UBYTE` 瀹氫箟銆俙reserved` 瀛楁琚墿灞曘€?

19. In struct v4l2_captureparm the type of
    the `timeperframe` field changed from unsigned long to
    struct v4l2_fract. This allows the accurate
    expression of multiples of the NTSC-M frame rate 30000 / 1001. A new
    field `readbuffers` was added to control the driver behaviour in
    read I/O mode.

19. 鍦?struct v4l2_captureparm 涓紝`timeperframe` 瀛楁鐨勭被鍨嬩粠
    unsigned long 鏀逛负 struct v4l2_fract銆傝繖鏍峰彲浠ョ簿纭湴琛ㄨ揪
    NTSC-M 甯х巼 30000 / 1001 鐨勫€嶆暟銆傛柊澧炰簡 `readbuffers` 瀛楁
    鐢ㄤ簬鎺у埗椹卞姩鍦?read I/O 妯″紡涓嬬殑琛屼负銆?

    Similar changes were made to struct v4l2_outputparm.

    struct v4l2_outputparm 涔熷仛浜嗙被浼肩殑鏀瑰姩銆?

20. The struct `v4l2_performance` and
    `VIDIOC_G_PERF` ioctl were dropped. Except when using the
    read/write I/O method <rw>, which is limited anyway, this
    information is already available to applications.

20. struct `v4l2_performance` 鍜?
    `VIDIOC_G_PERF` ioctl 琚簾寮冦€傞櫎浜嗗彈闄愮殑 read/write I/O 鏂规硶
    <rw> 澶栵紝杩欎簺淇℃伅搴旂敤绋嬪簭宸茬粡鍙互鑾峰彇銆?

21. The example transformation from RGB to YCbCr color space in the old
    V4L2 documentation was inaccurate, this has been corrected in
    pixfmt.

21. 鏃х増 V4L2 鏂囨。涓粠 RGB 鍒?YCbCr 棰滆壊绌洪棿鐨勭ず渚嬪彉鎹㈡槸涓嶅噯纭殑锛?
    宸插湪 pixfmt 涓簣浠ョ籂姝ｃ€?

## V4L2 2003-06-19

## V4L2 2003-06-19


1. A new capability flag `V4L2_CAP_RADIO` was added for radio devices.
   Prior to this change radio devices would identify solely by having
   exactly one tuner whose type field reads `V4L2_TUNER_RADIO`.

1. 涓烘棤绾跨數锛坮adio锛夎澶囨柊澧炰簡鑳藉姏鏍囧織 `V4L2_CAP_RADIO`銆傚湪姝ゆ敼鍔?
   涔嬪墠锛屾棤绾跨數璁惧浠呭嚟鍏舵嫢鏈夋伆濂戒竴涓?type 瀛楁涓?`V4L2_TUNER_RADIO`
   鐨勮皟璋愬櫒鏉ユ爣璇嗐€?

2. An optional driver access priority mechanism was added, see
   app-pri for details.

2. 鏂板浜嗕竴涓彲閫夌殑椹卞姩璁块棶浼樺厛绾ф満鍒讹紝璇﹁ app-pri銆?

3. The audio input and output interface was found to be incomplete.

3. 浜轰滑鍙戠幇闊抽杈撳叆鍜岃緭鍑烘帴鍙ｅ苟涓嶅畬鏁淬€?

   Previously the VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> ioctl would
   enumerate the available audio inputs. An ioctl to determine the
   current audio input, if more than one combines with the current video
   input, did not exist. So `VIDIOC_G_AUDIO` was renamed to
   `VIDIOC_G_AUDIO_OLD`, this ioctl was removed on Kernel 2.6.39. The
   VIDIOC_ENUMAUDIO ioctl was added to
   enumerate audio inputs, while
   VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> now reports the current
   audio input.

   姝ゅ墠 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> ioctl 浼氭灇涓惧彲鐢ㄧ殑闊抽
   杈撳叆銆傝€岀敤浜庣‘瀹氬綋鍓嶉煶棰戣緭鍏ョ殑 ioctl锛堝綋涓嶆涓€涓煶棰戣緭鍏ヤ笌褰撳墠
   瑙嗛杈撳叆缁勫悎鏃讹級骞朵笉瀛樺湪銆傚洜姝?`VIDIOC_G_AUDIO` 琚噸鍛藉悕涓?
   `VIDIOC_G_AUDIO_OLD`锛岃 ioctl 鍦ㄥ唴鏍?2.6.39 涓绉婚櫎銆傛柊澧炰簡
   VIDIOC_ENUMAUDIO ioctl 鐢ㄤ簬鏋氫妇闊抽杈撳叆锛?
   鑰?VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> 鐜板湪鎶ュ憡褰撳墠
   闊抽杈撳叆銆?

   The same changes were made to
   VIDIOC_G_AUDOUT <VIDIOC_G_AUDOUT> and
   VIDIOC_ENUMAUDOUT <VIDIOC_ENUMAUDOUT>.

   瀵?VIDIOC_G_AUDOUT <VIDIOC_G_AUDOUT> 鍜?
   VIDIOC_ENUMAUDOUT <VIDIOC_ENUMAUDOUT> 涔熷仛浜嗗悓鏍风殑鏀瑰姩銆?

   Until further the "videodev" module will automatically translate
   between the old and new ioctls, but drivers and applications must be
   updated to successfully compile again.

   鍦ㄦ杩囨浮鏈熼棿锛?videodev" 妯″潡浼氳嚜鍔ㄥ湪鏃?ioctl 鍜屾柊 ioctl 涔嬮棿
   杩涜杞崲锛屼絾椹卞姩鍜屽簲鐢ㄧ▼搴忓繀椤绘洿鏂版墠鑳芥垚鍔熼噸鏂扮紪璇戙€?

4. The VIDIOC_OVERLAY ioctl was incorrectly
   defined with write-read parameter. It was changed to write-only,
   while the write-read version was renamed to `VIDIOC_OVERLAY_OLD`.
   The old ioctl was removed on Kernel 2.6.39. Until further the
   "videodev" kernel module will automatically translate to the new
   version, so drivers must be recompiled, but not applications.

4. VIDIOC_OVERLAY ioctl 鏇捐閿欒鍦板畾涔変负璇?鍐欏弬鏁般€傚畠琚敼涓?
   鍙啓锛坵rite-only锛夛紝鑰岃-鍐欑増鏈閲嶅懡鍚嶄负 `VIDIOC_OVERLAY_OLD`銆?
   鏃х殑 ioctl 鍦ㄥ唴鏍?2.6.39 涓绉婚櫎銆傚湪姝よ繃娓℃湡闂达紝"videodev"
   鍐呮牳妯″潡浼氳嚜鍔ㄨ浆鎹负鏂扮増锛屽洜姝ら┍鍔ㄥ繀椤婚噸鏂扮紪璇戯紝浣嗗簲鐢ㄧ▼搴?
   鏃犻渶閲嶆柊缂栬瘧銆?

5. overlay incorrectly stated that clipping rectangles define
   regions where the video can be seen. Correct is that clipping
   rectangles define regions where **no** video shall be displayed and so
   the graphics surface can be seen.

5. overlay 鏂囨。閿欒鍦板０绉拌鍓煩褰㈠畾涔変簡瑙嗛鍙鐨勫尯鍩熴€傛纭殑璇存硶鏄紝
   瑁佸壀鐭╁舰瀹氫箟浜?*涓嶅簲**鏄剧ず瑙嗛鐨勫尯鍩燂紝浠庤€屽彲浠ョ湅鍒板浘褰㈣〃闈€?

6. The VIDIOC_S_PARM <VIDIOC_G_PARM> and
   VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctls were defined with
   write-only parameter, inconsistent with other ioctls modifying their
   argument. They were changed to write-read, while a `_OLD` suffix
   was added to the write-only versions. The old ioctls were removed on
   Kernel 2.6.39. Drivers and applications assuming a constant parameter
   need an update.

6. VIDIOC_S_PARM <VIDIOC_G_PARM> 鍜?
   VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctls 琚畾涔変负鍙啓鍙傛暟锛岃繖涓庡叾浠?
   淇敼鍏跺弬鏁扮殑 ioctl 涓嶄竴鑷淬€傚畠浠鏀逛负璇?鍐欙紝鑰屽彧鍐欑増鏈姞涓婁簡
   `_OLD` 鍚庣紑銆傛棫 ioctl 鍦ㄥ唴鏍?2.6.39 涓绉婚櫎銆傚亣瀹氬弬鏁颁负
   甯搁噺锛坈onstant锛夌殑椹卞姩鍜屽簲鐢ㄧ▼搴忛渶瑕佹洿鏂般€?

## V4L2 2003-11-05

## V4L2 2003-11-05


1. In pixfmt-rgb the following pixel formats were incorrectly
   transferred from Bill Dirks' V4L2 specification. Descriptions below
   refer to bytes in memory, in ascending address order.


```
       :header-rows:  1
       :stub-columns: 0

       * - Symbol
	 - In this document prior to revision 0.5
	 - Corrected
       * - ``V4L2_PIX_FMT_RGB24``
	 - B, G, R
	 - R, G, B
       * - ``V4L2_PIX_FMT_BGR24``
	 - R, G, B
	 - B, G, R
       * - ``V4L2_PIX_FMT_RGB32``
	 - B, G, R, X
	 - R, G, B, X
       * - ``V4L2_PIX_FMT_BGR32``
	 - R, G, B, X
	 - B, G, R, X

   The ``V4L2_PIX_FMT_BGR24`` example was always correct.

   In :ref:`v4l-image-properties` the mapping of the V4L
   ``VIDEO_PALETTE_RGB24`` and ``VIDEO_PALETTE_RGB32`` formats to V4L2
   pixel formats was accordingly corrected.

```

1. 鍦?pixfmt-rgb 涓紝浠ヤ笅鍍忕礌鏍煎紡閿欒鍦扮収鎼嚜 Bill Dirks 鐨?V4L2 瑙勮寖銆?
   涓嬮潰鐨勬弿杩版寚鐨勬槸鍐呭瓨涓殑瀛楄妭锛屾寜鍦板潃鍗囧簭鎺掑垪銆?


   ``V4L2_PIX_FMT_BGR24`` 鐨勭ず渚嬪缁堟槸姝ｇ‘鐨勩€?

   鍦?:ref:`v4l-image-properties` 涓紝V4L 鐨?
   ``VIDEO_PALETTE_RGB24`` 鍜?``VIDEO_PALETTE_RGB32`` 鏍煎紡鍒?V4L2
   鍍忕礌鏍煎紡鐨勬槧灏勪篃鍋氫簡鐩稿簲鐨勭籂姝ｃ€?

2. Unrelated to the fixes above, drivers may still interpret some V4L2
   RGB pixel formats differently. These issues have yet to be addressed,
   for details see pixfmt-rgb.

2. 涓庝笂杩颁慨姝ｆ棤鍏筹紝椹卞姩鍙兘浠嶇劧浠ヤ笉鍚屾柟寮忚В閲婃煇浜?V4L2 RGB 鍍忕礌鏍煎紡銆?
   杩欎簺闂灏氬緟瑙ｅ喅锛岃瑙?pixfmt-rgb銆?

## V4L2 in Linux 2.6.6, 2004-05-09

## Linux 2.6.6 涓殑 V4L2锛?004-05-09


1. The VIDIOC_CROPCAP ioctl was incorrectly
   defined with read-only parameter. It is now defined as write-read
   ioctl, while the read-only version was renamed to
   `VIDIOC_CROPCAP_OLD`. The old ioctl was removed on Kernel 2.6.39.

1. VIDIOC_CROPCAP ioctl 鏇捐閿欒鍦板畾涔変负鍙鍙傛暟銆傜幇鍦ㄥ畠琚畾涔変负
   璇?鍐?ioctl锛岃€屽彧璇荤増鏈閲嶅懡鍚嶄负 `VIDIOC_CROPCAP_OLD`銆傛棫鐨?
   ioctl 鍦ㄥ唴鏍?2.6.39 涓绉婚櫎銆?

## V4L2 in Linux 2.6.8

## Linux 2.6.8 涓殑 V4L2


1. A new field `input` (former `reserved[^0^]`) was added to the
   struct v4l2_buffer. Purpose of this
   field is to alternate between video inputs (e. g. cameras) in step
   with the video capturing process. This function must be enabled with
   the new `V4L2_BUF_FLAG_INPUT` flag. The `flags` field is no
   longer read-only.

1. 鍦?struct v4l2_buffer 涓柊澧炰簡涓€涓瓧娈?`input`锛堝師
   `reserved[^0^]`锛夈€傝瀛楁鐨勭洰鐨勬槸鍦ㄨ棰戦噰闆嗚繃绋嬩腑
   涓庨噰闆嗘祦绋嬪悓姝ュ湴鍒囨崲涓嶅悓鐨勮棰戣緭鍏ワ紙渚嬪鎽勫儚澶达級銆傝繖涓€鍔熻兘蹇呴』
   閫氳繃鏂扮殑 `V4L2_BUF_FLAG_INPUT` 鏍囧織鍚敤銆俙flags` 瀛楁涓嶅啀鏄?
   鍙鐨勩€?

## V4L2 spec erratum 2004-08-01

## V4L2 瑙勮寖鍕樿 2004-08-01


1. The return value of the func-open function was incorrectly
   documented.

1. func-open 鍑芥暟鐨勮繑鍥炲€兼枃妗ｆ湁璇€?

2. Audio output ioctls end in -AUDOUT, not -AUDIOOUT.

2. 闊抽杈撳嚭 ioctl 浠?-AUDOUT 缁撳熬锛岃€岄潪 -AUDIOOUT銆?

3. In the Current Audio Input example the `VIDIOC_G_AUDIO` ioctl took
   the wrong argument.

3. 鍦?褰撳墠闊抽杈撳叆"绀轰緥涓紝`VIDIOC_G_AUDIO` ioctl 浣跨敤浜嗛敊璇殑
   鍙傛暟銆?

4. The documentation of the VIDIOC_QBUF and
   VIDIOC_DQBUF <VIDIOC_QBUF> ioctls did not mention the
   struct v4l2_buffer `memory` field. It was
   also missing from examples. Also on the `VIDIOC_DQBUF` page the `EIO`
   error code was not documented.

4. VIDIOC_QBUF 鍜?VIDIOC_DQBUF <VIDIOC_QBUF> ioctls 鐨勬枃妗?
   娌℃湁鎻愬強 struct v4l2_buffer 鐨?`memory` 瀛楁銆傜ず渚嬩腑
   涔熺己澶变簡瀹冦€傛澶栵紝鍦?`VIDIOC_DQBUF` 椤甸潰涓?`EIO` 閿欒鐮佷篃
   鏈鏂囨。璇存槑銆?

## V4L2 in Linux 2.6.14

## Linux 2.6.14 涓殑 V4L2


1. A new sliced VBI interface was added. It is documented in
   sliced and replaces the interface first proposed in V4L2
   specification 0.8.

1. 鏂板浜嗕竴涓柊鐨勫垎鐗囷紙sliced锛塚BI 鎺ュ彛銆傚畠鍦?sliced 涓湁鏂囨。锛?
   鍙栦唬浜?V4L2 瑙勮寖 0.8 涓渶鍒濇彁鍑虹殑鎺ュ彛銆?

## V4L2 in Linux 2.6.15

## Linux 2.6.15 涓殑 V4L2


1. The VIDIOC_LOG_STATUS ioctl was added.

1. 鏂板浜?VIDIOC_LOG_STATUS ioctl銆?

2. New video standards `V4L2_STD_NTSC_443`, `V4L2_STD_SECAM_LC`,
   `V4L2_STD_SECAM_DK` (a set of SECAM D, K and K1), and
   `V4L2_STD_ATSC` (a set of `V4L2_STD_ATSC_8_VSB` and
   `V4L2_STD_ATSC_16_VSB`) were defined. Note the `V4L2_STD_525_60`
   set now includes `V4L2_STD_NTSC_443`. See also
   v4l2-std-id.

2. 瀹氫箟浜嗘柊鐨勮棰戞爣鍑?`V4L2_STD_NTSC_443`銆乣V4L2_STD_SECAM_LC`銆?
   `V4L2_STD_SECAM_DK`锛堜竴缁?SECAM D銆並 鍜?K1锛変互鍙?
   `V4L2_STD_ATSC`锛堜竴缁?`V4L2_STD_ATSC_8_VSB` 鍜?
   `V4L2_STD_ATSC_16_VSB`锛夈€傛敞鎰?`V4L2_STD_525_60` 闆嗗悎鐜板湪
   鍖呭惈浜?`V4L2_STD_NTSC_443`銆傚彟瑙?v4l2-std-id銆?

3. The `VIDIOC_G_COMP` and `VIDIOC_S_COMP` ioctl were renamed to
   `VIDIOC_G_MPEGCOMP` and `VIDIOC_S_MPEGCOMP` respectively. Their
   argument was replaced by a struct
   `v4l2_mpeg_compression` pointer. (The
   `VIDIOC_G_MPEGCOMP` and `VIDIOC_S_MPEGCOMP` ioctls where removed
   in Linux 2.6.25.)

3. `VIDIOC_G_COMP` 鍜?`VIDIOC_S_COMP` ioctl 鍒嗗埆琚噸鍛藉悕涓?
   `VIDIOC_G_MPEGCOMP` 鍜?`VIDIOC_S_MPEGCOMP`銆傚畠浠殑鍙傛暟琚浛鎹负
   struct `v4l2_mpeg_compression` 鎸囬拡銆傦紙`VIDIOC_G_MPEGCOMP` 鍜?
   `VIDIOC_S_MPEGCOMP` ioctls 鍦?Linux 2.6.25 涓绉婚櫎銆傦級

## V4L2 spec erratum 2005-11-27

## V4L2 瑙勮寖鍕樿 2005-11-27


The capture example in capture-example called the
VIDIOC_S_CROP <VIDIOC_G_CROP> ioctl without checking if
cropping is supported. In the video standard selection example in
standard the VIDIOC_S_STD <VIDIOC_G_STD> call used
the wrong argument type.

capture-example 涓殑閲囬泦绀轰緥璋冪敤浜?VIDIOC_S_CROP <VIDIOC_G_CROP>
ioctl锛屽嵈娌℃湁妫€鏌ユ槸鍚︽敮鎸佽鍓€傝€屽湪 standard 涓殑瑙嗛鏍囧噯閫夋嫨
绀轰緥閲岋紝VIDIOC_S_STD <VIDIOC_G_STD> 璋冪敤浣跨敤浜嗛敊璇殑鍙傛暟绫诲瀷銆?

## V4L2 spec erratum 2006-01-10

## V4L2 瑙勮寖鍕樿 2006-01-10


1. The `V4L2_IN_ST_COLOR_KILL` flag in struct v4l2_input not only
   indicates if the color killer is enabled, but also if it is active.
   (The color killer disables color decoding when it detects no color
   in the video signal to improve the image quality.)

1. struct v4l2_input 涓殑 `V4L2_IN_ST_COLOR_KILL` 鏍囧織涓嶄粎鎸囩ず
   娑堣壊鍣紙color killer锛夋槸鍚﹀惎鐢紝杩樻寚绀哄叾鏄惁澶勪簬娲诲姩鐘舵€併€?
   锛堝綋娑堣壊鍣ㄦ娴嬪埌瑙嗛淇″彿涓病鏈夐鑹叉椂锛屽畠浼氱鐢ㄩ鑹茶В鐮佷互鏀瑰杽
   鍥惧儚璐ㄩ噺銆傦級

2. VIDIOC_S_PARM <VIDIOC_G_PARM> is a write-read ioctl, not
   write-only as stated on its reference page. The ioctl changed in 2003
   as noted above.

2. VIDIOC_S_PARM <VIDIOC_G_PARM> 鏄竴涓-鍐?ioctl锛岃€屼笉鏄叾
   鍙傝€冮〉涓婃墍璇寸殑鍙啓銆傝 ioctl 鍦?2003 骞村凡濡傚墠鎵€杩板彂鐢熶簡鏀瑰彉銆?

## V4L2 spec erratum 2006-02-03

## V4L2 瑙勮寖鍕樿 2006-02-03


1. In struct v4l2_captureparm and struct v4l2_outputparm the `timeperframe`
   field gives the time in seconds, not microseconds.

1. 鍦?struct v4l2_captureparm 鍜?struct v4l2_outputparm 涓紝
   `timeperframe` 瀛楁缁欏嚭鐨勬椂闂村崟浣嶆槸绉掞紝鑰岄潪寰銆?

## V4L2 spec erratum 2006-02-04

## V4L2 瑙勮寖鍕樿 2006-02-04


1. The `clips` field in struct v4l2_window
   must point to an array of struct v4l2_clip, not
   a linked list, because drivers ignore the
   struct v4l2_clip. `next` pointer.

1. struct v4l2_window 涓殑 `clips` 瀛楁蹇呴』鎸囧悜
   struct v4l2_clip 鏁扮粍锛岃€屼笉鏄摼琛紝鍥犱负椹卞姩浼氬拷鐣?
   struct v4l2_clip 鐨?`next` 鎸囬拡銆?

## V4L2 in Linux 2.6.17

## Linux 2.6.17 涓殑 V4L2


1. New video standard macros were added: `V4L2_STD_NTSC_M_KR` (NTSC M
   South Korea), and the sets `V4L2_STD_MN`, `V4L2_STD_B`,
   `V4L2_STD_GH` and `V4L2_STD_DK`. The `V4L2_STD_NTSC` and
   `V4L2_STD_SECAM` sets now include `V4L2_STD_NTSC_M_KR` and
   `V4L2_STD_SECAM_LC` respectively.

1. 鏂板浜嗚棰戞爣鍑嗗畯锛歚V4L2_STD_NTSC_M_KR`锛圢TSC M 闊╁浗鐗堬級锛?
   浠ュ強闆嗗悎 `V4L2_STD_MN`銆乣V4L2_STD_B`銆乣V4L2_STD_GH` 鍜?
   `V4L2_STD_DK`銆俙V4L2_STD_NTSC` 鍜?`V4L2_STD_SECAM` 闆嗗悎鐜板湪
   鍒嗗埆鍖呭惈浜?`V4L2_STD_NTSC_M_KR` 鍜?`V4L2_STD_SECAM_LC`銆?

2. A new `V4L2_TUNER_MODE_LANG1_LANG2` was defined to record both
   languages of a bilingual program. The use of
   `V4L2_TUNER_MODE_STEREO` for this purpose is deprecated now. See
   the VIDIOC_G_TUNER <VIDIOC_G_TUNER> section for details.

2. 瀹氫箟浜嗕竴涓柊鐨?`V4L2_TUNER_MODE_LANG1_LANG2`锛岀敤浜庤褰曞弻璇妭鐩?
   鐨勪袱绉嶈瑷€銆傜幇鍦ㄤ笉鎺ㄨ崘涓烘鐩殑浣跨敤 `V4L2_TUNER_MODE_STEREO`銆?
   璇﹁ VIDIOC_G_TUNER <VIDIOC_G_TUNER> 绔犺妭銆?

## V4L2 spec erratum 2006-09-23 (Draft 0.15)

## V4L2 瑙勮寖鍕樿 2006-09-23锛堣崏妗?0.15锛?


1. In various places `V4L2_BUF_TYPE_SLICED_VBI_CAPTURE` and
   `V4L2_BUF_TYPE_SLICED_VBI_OUTPUT` of the sliced VBI interface were
   not mentioned along with other buffer types.

1. 鍦ㄨ澶氬湴鏂癸紝鍒嗙墖 VBI 鎺ュ彛鐨?`V4L2_BUF_TYPE_SLICED_VBI_CAPTURE` 鍜?
   `V4L2_BUF_TYPE_SLICED_VBI_OUTPUT` 娌℃湁鍜屽叾浠栫紦鍐插尯绫诲瀷涓€璧疯
   鎻愬強銆?

2. In VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> it was clarified that the
   struct v4l2_audio `mode` field is a flags field.

2. 鍦?VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> 涓緞娓呬簡 struct v4l2_audio 鐨?
   `mode` 瀛楁鏄竴涓爣蹇椾綅瀛楁銆?

3. VIDIOC_QUERYCAP did not mention the sliced VBI and radio
   capability flags.

3. VIDIOC_QUERYCAP 娌℃湁鎻愬強鍒嗙墖 VBI 鍜屾棤绾跨數鐨勮兘鍔涙爣蹇椼€?

4. In VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> it was clarified that
   applications must initialize the tuner `type` field of
   struct v4l2_frequency before calling
   VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY>.

4. 鍦?VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> 涓緞娓呬簡搴旂敤绋嬪簭
   蹇呴』鍦ㄨ皟鐢?VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> 涔嬪墠鍒濆鍖?
   struct v4l2_frequency 鐨勮皟璋愬櫒 `type` 瀛楁銆?

5. The `reserved` array in struct v4l2_requestbuffers has 2 elements,
   not 32.

5. struct v4l2_requestbuffers 涓殑 `reserved` 鏁扮粍鏈?2 涓厓绱狅紝
   鑰岄潪 32 涓€?

6. In output and raw-vbi the device file names
   `/dev/vout` which never caught on were replaced by `/dev/video`.

6. 鍦?output 鍜?raw-vbi 涓紝浠庢湭娴佽鐨勮澶囨枃浠跺悕
   `/dev/vout` 琚?`/dev/video` 鍙栦唬銆?

7. With Linux 2.6.15 the possible range for VBI device minor numbers was
   extended from 224-239 to 224-255. Accordingly device file names
   `/dev/vbi0` to `/dev/vbi31` are possible now.

7. 浠?Linux 2.6.15 璧凤紝VBI 璁惧娆¤澶囧彿鐨勫彲鑳借寖鍥翠粠 224-239 鎵╁睍鍒?
   224-255銆傜浉搴斿湴锛岀幇鍦ㄥ彲浠ヤ娇鐢?`/dev/vbi0` 鍒?`/dev/vbi31` 杩欐牱鐨?
   璁惧鏂囦欢鍚嶃€?

## V4L2 in Linux 2.6.18

## Linux 2.6.18 涓殑 V4L2


1. New ioctls VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>,
   VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> and
   VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> were added, a
   flag to skip unsupported controls with
   VIDIOC_QUERYCTRL, new control types
   `V4L2_CTRL_TYPE_INTEGER64` and `V4L2_CTRL_TYPE_CTRL_CLASS`
   (enum v4l2_ctrl_type), and new control flags
   `V4L2_CTRL_FLAG_READ_ONLY`, `V4L2_CTRL_FLAG_UPDATE`,
   `V4L2_CTRL_FLAG_INACTIVE` and `V4L2_CTRL_FLAG_SLIDER`
   (control-flags). See extended-controls for details.

1. 鏂板浜?VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>銆?
   VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鍜?
   VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctls锛屼竴涓厤鍚?
   VIDIOC_QUERYCTRL 璺宠繃涓嶅彈鏀寔鎺у埗椤圭殑鏍囧織锛屾柊鐨勬帶鍒剁被鍨?
   `V4L2_CTRL_TYPE_INTEGER64` 鍜?`V4L2_CTRL_TYPE_CTRL_CLASS`
   锛坋num v4l2_ctrl_type锛夛紝浠ュ強鏂扮殑鎺у埗鏍囧織
   `V4L2_CTRL_FLAG_READ_ONLY`銆乣V4L2_CTRL_FLAG_UPDATE`銆?
   `V4L2_CTRL_FLAG_INACTIVE` 鍜?`V4L2_CTRL_FLAG_SLIDER`
   锛坈ontrol-flags锛夈€傝瑙?extended-controls銆?

## V4L2 in Linux 2.6.19

## Linux 2.6.19 涓殑 V4L2


1. In struct v4l2_sliced_vbi_cap a
   buffer type field was added replacing a reserved field. Note on
   architectures where the size of enum types differs from int types the
   size of the structure changed. The
   VIDIOC_G_SLICED_VBI_CAP <VIDIOC_G_SLICED_VBI_CAP> ioctl
   was redefined from being read-only to write-read. Applications must
   initialize the type field and clear the reserved fields now. These
   changes may **break the compatibility** with older drivers and
   applications.

1. 鍦?struct v4l2_sliced_vbi_cap 涓紝鏂板浜嗕竴涓紦鍐插尯绫诲瀷瀛楁浠?
   鍙栦唬涓€涓繚鐣欏瓧娈点€傛敞鎰忓湪鏋氫妇绫诲瀷澶у皬涓?int 绫诲瀷涓嶅悓鐨勬灦鏋勪笂锛?
   璇ョ粨鏋勭殑澶у皬浼氬彂鐢熸敼鍙樸€俈IDIOC_G_SLICED_VBI_CAP
   <VIDIOC_G_SLICED_VBI_CAP> ioctl 浠庡彧璇昏閲嶆柊瀹氫箟涓鸿-鍐欍€?
   搴旂敤绋嬪簭鐜板湪蹇呴』鍒濆鍖?type 瀛楁骞舵竻闄や繚鐣欏瓧娈点€傝繖浜涘彉鍖栧彲鑳?
   **鐮村潖涓庤緝鏃ч┍鍔ㄥ拰搴旂敤绋嬪簭鐨勫吋瀹规€?*銆?

2. The ioctls VIDIOC_ENUM_FRAMESIZES
   and
   VIDIOC_ENUM_FRAMEINTERVALS
   were added.

2. 鏂板浜?VIDIOC_ENUM_FRAMESIZES 鍜?
   VIDIOC_ENUM_FRAMEINTERVALS ioctls銆?

3. A new pixel format `V4L2_PIX_FMT_RGB444` (pixfmt-rgb) was
   added.

3. 鏂板浜嗘柊鐨勫儚绱犳牸寮?`V4L2_PIX_FMT_RGB444`锛坧ixfmt-rgb锛夈€?

## V4L2 spec erratum 2006-10-12 (Draft 0.17)

## V4L2 瑙勮寖鍕樿 2006-10-12锛堣崏妗?0.17锛?


1. `V4L2_PIX_FMT_HM12` (reserved-formats) is a YUV 4:2:0, not
   4:2:2 format.

1. `V4L2_PIX_FMT_HM12`锛坮eserved-formats锛夋槸 YUV 4:2:0锛岃€岄潪
   4:2:2 鏍煎紡銆?

## V4L2 in Linux 2.6.21

## Linux 2.6.21 涓殑 V4L2


1. The `videodev2.h` header file is now dual licensed under GNU
   General Public License version two or later, and under a 3-clause
   BSD-style license.

1. `videodev2.h` 澶存枃浠剁幇鍦ㄩ噰鐢ㄥ弻閲嶈鍙細GNU 閫氱敤鍏叡璁稿彲璇侊紙鐗堟湰浜?
   鎴栨洿楂橈級浠ュ強 3 鏉℃鐨?BSD 椋庢牸璁稿彲璇併€?

## V4L2 in Linux 2.6.22

## Linux 2.6.22 涓殑 V4L2


1. Two new field orders `V4L2_FIELD_INTERLACED_TB` and
   `V4L2_FIELD_INTERLACED_BT` were added. See enum v4l2_field for
   details.

1. 鏂板浜嗕袱涓柊鐨勫満搴?`V4L2_FIELD_INTERLACED_TB` 鍜?
   `V4L2_FIELD_INTERLACED_BT`銆傝瑙?enum v4l2_field銆?

2. Three new clipping/blending methods with a global or straight or
   inverted local alpha value were added to the video overlay interface.
   See the description of the VIDIOC_G_FBUF <VIDIOC_G_FBUF>
   and VIDIOC_S_FBUF <VIDIOC_G_FBUF> ioctls for details.

2. 瑙嗛鍙犲姞锛坥verlay锛夋帴鍙ｆ柊澧炰簡涓夌瑁佸壀/娣峰悎锛坈lipping/blending锛?
   鏂规硶锛屽垎鍒甫鏈夊叏灞€銆佹垨姝ｅ悜銆佹垨鍙栧弽鐨勫眬閮?alpha 鍊笺€傝瑙?
   VIDIOC_G_FBUF <VIDIOC_G_FBUF> 鍜?VIDIOC_S_FBUF
   <VIDIOC_G_FBUF> ioctls 鐨勬弿杩般€?

   A new `global_alpha` field was added to struct v4l2_window,
   extending the structure. This may **break compatibility** with
   applications using a struct v4l2_window directly. However the
   VIDIOC_G/S/TRY_FMT <VIDIOC_G_FMT> ioctls, which take a
   pointer to a struct v4l2_format parent structure
   with padding bytes at the end, are not affected.

   鍦?struct v4l2_window 涓柊澧炰簡 `global_alpha` 瀛楁锛屾墿灞曚簡璇?
   缁撴瀯銆傝繖鍙兘**鐮村潖涓庣洿鎺ヤ娇鐢?struct v4l2_window 鐨勫簲鐢ㄧ▼搴忕殑
   鍏煎鎬?*銆備笉杩囷紝VIDIOC_G/S/TRY_FMT <VIDIOC_G_FMT> ioctls 鎺ュ彈
   涓€涓寚鍚戞湯灏惧甫濉厖瀛楄妭鐨?struct v4l2_format 鐖剁粨鏋勭殑鎸囬拡锛?
   涓嶄細鍙楀埌褰卞搷銆?

3. The format of the `chromakey` field in struct v4l2_window changed from
   "host order RGB32" to a pixel value in the same format as the framebuffer.
   This may **break compatibility** with existing applications. Drivers
   supporting the "host order RGB32" format are not known.

3. struct v4l2_window 涓?`chromakey` 瀛楁鐨勬牸寮忎粠
   "host order RGB32" 鏀逛负涓庡抚缂撳啿锛坒ramebuffer锛夌浉鍚岀殑鍍忕礌鍊兼牸寮忋€?
   杩欏彲鑳?*鐮村潖涓庣幇鏈夊簲鐢ㄧ▼搴忕殑鍏煎鎬?*銆傜洰鍓嶄笉鐭ラ亾鏈夐┍鍔ㄦ敮鎸?
   "host order RGB32" 鏍煎紡銆?

## V4L2 in Linux 2.6.24

## Linux 2.6.24 涓殑 V4L2


1. The pixel formats `V4L2_PIX_FMT_PAL8`, `V4L2_PIX_FMT_YUV444`,
   `V4L2_PIX_FMT_YUV555`, `V4L2_PIX_FMT_YUV565` and
   `V4L2_PIX_FMT_YUV32` were added.

1. 鏂板浜嗗儚绱犳牸寮?`V4L2_PIX_FMT_PAL8`銆乣V4L2_PIX_FMT_YUV444`銆?
   `V4L2_PIX_FMT_YUV555`銆乣V4L2_PIX_FMT_YUV565` 鍜?
   `V4L2_PIX_FMT_YUV32`銆?

## V4L2 in Linux 2.6.25

## Linux 2.6.25 涓殑 V4L2


1. The pixel formats V4L2_PIX_FMT_Y16 <V4L2-PIX-FMT-Y16> and
   V4L2_PIX_FMT_SBGGR16 <V4L2-PIX-FMT-SBGGR16> were added.

1. 鏂板浜嗗儚绱犳牸寮?V4L2_PIX_FMT_Y16 <V4L2-PIX-FMT-Y16> 鍜?
   V4L2_PIX_FMT_SBGGR16 <V4L2-PIX-FMT-SBGGR16>銆?

2. New controls <control> `V4L2_CID_POWER_LINE_FREQUENCY`,
   `V4L2_CID_HUE_AUTO`, `V4L2_CID_WHITE_BALANCE_TEMPERATURE`,
   `V4L2_CID_SHARPNESS` and `V4L2_CID_BACKLIGHT_COMPENSATION` were
   added. The controls `V4L2_CID_BLACK_LEVEL`, `V4L2_CID_WHITENESS`,
   `V4L2_CID_HCENTER` and `V4L2_CID_VCENTER` were deprecated.

2. 鏂板浜嗘帶鍒堕」 <control> `V4L2_CID_POWER_LINE_FREQUENCY`銆?
   `V4L2_CID_HUE_AUTO`銆乣V4L2_CID_WHITE_BALANCE_TEMPERATURE`銆?
   `V4L2_CID_SHARPNESS` 鍜?`V4L2_CID_BACKLIGHT_COMPENSATION`銆?
   鎺у埗椤?`V4L2_CID_BLACK_LEVEL`銆乣V4L2_CID_WHITENESS`銆?
   `V4L2_CID_HCENTER` 鍜?`V4L2_CID_VCENTER` 琚純鐢ㄣ€?

3. A Camera controls class <camera-controls> was added, with
   the new controls `V4L2_CID_EXPOSURE_AUTO`,
   `V4L2_CID_EXPOSURE_ABSOLUTE`, `V4L2_CID_EXPOSURE_AUTO_PRIORITY`,
   `V4L2_CID_PAN_RELATIVE`, `V4L2_CID_TILT_RELATIVE`,
   `V4L2_CID_PAN_RESET`, `V4L2_CID_TILT_RESET`,
   `V4L2_CID_PAN_ABSOLUTE`, `V4L2_CID_TILT_ABSOLUTE`,
   `V4L2_CID_FOCUS_ABSOLUTE`, `V4L2_CID_FOCUS_RELATIVE` and
   `V4L2_CID_FOCUS_AUTO`.

3. 鏂板浜嗙浉鏈猴紙Camera锛夋帶鍒剁被 <camera-controls>锛屽寘鍚柊鐨勬帶鍒堕」
   `V4L2_CID_EXPOSURE_AUTO`銆乣V4L2_CID_EXPOSURE_ABSOLUTE`銆?
   `V4L2_CID_EXPOSURE_AUTO_PRIORITY`銆乣V4L2_CID_PAN_RELATIVE`銆?
   `V4L2_CID_TILT_RELATIVE`銆乣V4L2_CID_PAN_RESET`銆?
   `V4L2_CID_TILT_RESET`銆乣V4L2_CID_PAN_ABSOLUTE`銆?
   `V4L2_CID_TILT_ABSOLUTE`銆乣V4L2_CID_FOCUS_ABSOLUTE`銆?
   `V4L2_CID_FOCUS_RELATIVE` 鍜?`V4L2_CID_FOCUS_AUTO`銆?

4. The `VIDIOC_G_MPEGCOMP` and `VIDIOC_S_MPEGCOMP` ioctls, which
   were superseded by the extended controls <extended-controls>
   interface in Linux 2.6.18, where finally removed from the
   `videodev2.h` header file.

4. 鍦?Linux 2.6.18 涓凡琚墿灞曟帶鍒?<extended-controls> 鎺ュ彛鍙栦唬鐨?
   `VIDIOC_G_MPEGCOMP` 鍜?`VIDIOC_S_MPEGCOMP` ioctls锛屾渶缁堜粠
   `videodev2.h` 澶存枃浠朵腑绉婚櫎銆?

## V4L2 in Linux 2.6.26

## Linux 2.6.26 涓殑 V4L2


1. The pixel formats `V4L2_PIX_FMT_Y16` and `V4L2_PIX_FMT_SBGGR16`
   were added.

1. 鏂板浜嗗儚绱犳牸寮?`V4L2_PIX_FMT_Y16` 鍜?`V4L2_PIX_FMT_SBGGR16`銆?

2. Added user controls `V4L2_CID_CHROMA_AGC` and
   `V4L2_CID_COLOR_KILLER`.

2. 鏂板浜嗙敤鎴锋帶鍒堕」 `V4L2_CID_CHROMA_AGC` 鍜?`V4L2_CID_COLOR_KILLER`銆?

## V4L2 in Linux 2.6.27

## Linux 2.6.27 涓殑 V4L2


1. The VIDIOC_S_HW_FREQ_SEEK ioctl
   and the `V4L2_CAP_HW_FREQ_SEEK` capability were added.

1. 鏂板浜?VIDIOC_S_HW_FREQ_SEEK ioctl 鍜?
   `V4L2_CAP_HW_FREQ_SEEK` 鑳藉姏銆?

2. The pixel formats `V4L2_PIX_FMT_YVYU`, `V4L2_PIX_FMT_PCA501`,
   `V4L2_PIX_FMT_PCA505`, `V4L2_PIX_FMT_PCA508`,
   `V4L2_PIX_FMT_PCA561`, `V4L2_PIX_FMT_SGBRG8`,
   `V4L2_PIX_FMT_PAC207` and `V4L2_PIX_FMT_PJPG` were added.

2. 鏂板浜嗗儚绱犳牸寮?`V4L2_PIX_FMT_YVYU`銆乣V4L2_PIX_FMT_PCA501`銆?
   `V4L2_PIX_FMT_PCA505`銆乣V4L2_PIX_FMT_PCA508`銆?
   `V4L2_PIX_FMT_PCA561`銆乣V4L2_PIX_FMT_SGBRG8`銆?
   `V4L2_PIX_FMT_PAC207` 鍜?`V4L2_PIX_FMT_PJPG`銆?

## V4L2 in Linux 2.6.28

## Linux 2.6.28 涓殑 V4L2


1. Added `V4L2_MPEG_AUDIO_ENCODING_AAC` and
   `V4L2_MPEG_AUDIO_ENCODING_AC3` MPEG audio encodings.

1. 鏂板浜?`V4L2_MPEG_AUDIO_ENCODING_AAC` 鍜?
   `V4L2_MPEG_AUDIO_ENCODING_AC3` MPEG 闊抽缂栫爜銆?

2. Added `V4L2_MPEG_VIDEO_ENCODING_MPEG_4_AVC` MPEG video encoding.

2. 鏂板浜?`V4L2_MPEG_VIDEO_ENCODING_MPEG_4_AVC` MPEG 瑙嗛缂栫爜銆?

3. The pixel formats `V4L2_PIX_FMT_SGRBG10` and
   `V4L2_PIX_FMT_SGRBG10DPCM8` were added.

3. 鏂板浜嗗儚绱犳牸寮?`V4L2_PIX_FMT_SGRBG10` 鍜?`V4L2_PIX_FMT_SGRBG10DPCM8`銆?

## V4L2 in Linux 2.6.29

## Linux 2.6.29 涓殑 V4L2


1. The `VIDIOC_G_CHIP_IDENT` ioctl was renamed to
   `VIDIOC_G_CHIP_IDENT_OLD` and `VIDIOC_DBG_G_CHIP_IDENT` was
   introduced in its place. The old struct `v4l2_chip_ident` was renamed to
   struct `v4l2_chip_ident_old`.

1. `VIDIOC_G_CHIP_IDENT` ioctl 琚噸鍛藉悕涓?
   `VIDIOC_G_CHIP_IDENT_OLD`锛屽苟寮曞叆 `VIDIOC_DBG_G_CHIP_IDENT`
   鍙栬€屼唬涔嬨€傛棫鐨?struct `v4l2_chip_ident` 琚噸鍛藉悕涓?
   struct `v4l2_chip_ident_old`銆?

2. The pixel formats `V4L2_PIX_FMT_VYUY`, `V4L2_PIX_FMT_NV16` and
   `V4L2_PIX_FMT_NV61` were added.

2. 鏂板浜嗗儚绱犳牸寮?`V4L2_PIX_FMT_VYUY`銆乣V4L2_PIX_FMT_NV16` 鍜?
   `V4L2_PIX_FMT_NV61`銆?

3. Added camera controls `V4L2_CID_ZOOM_ABSOLUTE`,
   `V4L2_CID_ZOOM_RELATIVE`, `V4L2_CID_ZOOM_CONTINUOUS` and
   `V4L2_CID_PRIVACY`.

3. 鏂板浜嗙浉鏈烘帶鍒堕」 `V4L2_CID_ZOOM_ABSOLUTE`銆?
   `V4L2_CID_ZOOM_RELATIVE`銆乣V4L2_CID_ZOOM_CONTINUOUS` 鍜?
   `V4L2_CID_PRIVACY`銆?

## V4L2 in Linux 2.6.30

## Linux 2.6.30 涓殑 V4L2


1. New control flag `V4L2_CTRL_FLAG_WRITE_ONLY` was added.

1. 鏂板浜嗘帶鍒舵爣蹇?`V4L2_CTRL_FLAG_WRITE_ONLY`銆?

2. New control `V4L2_CID_COLORFX` was added.

2. 鏂板浜嗘帶鍒堕」 `V4L2_CID_COLORFX`銆?

## V4L2 in Linux 2.6.32

## Linux 2.6.32 涓殑 V4L2


1. In order to be easier to compare a V4L2 API and a kernel version, now
   V4L2 API is numbered using the Linux Kernel version numeration.

1. 涓轰簡渚夸簬灏?V4L2 API 涓庡唴鏍哥増鏈繘琛屾瘮杈冿紝鐜板湪 V4L2 API 閲囩敤
   Linux 鍐呮牳鐗堟湰鍙疯繘琛岀紪鍙枫€?

2. Finalized the RDS capture API. See rds for more information.

2. 瀹屽杽浜?RDS 閲囬泦 API銆傛洿澶氫俊鎭 rds銆?

3. Added new capabilities for modulators and RDS encoders.

3. 涓鸿皟鍒跺櫒锛坢odulator锛夊拰 RDS 缂栫爜鍣ㄦ柊澧炰簡鑳藉姏銆?

4. Add description for libv4l API.

4. 澧炲姞浜?libv4l API 鐨勮鏄庛€?

5. Added support for string controls via new type
   `V4L2_CTRL_TYPE_STRING`.

5. 閫氳繃鏂扮被鍨?`V4L2_CTRL_TYPE_STRING` 澧炲姞浜嗗瀛楃涓叉帶鍒堕」鐨勬敮鎸併€?

6. Added `V4L2_CID_BAND_STOP_FILTER` documentation.

6. 澧炲姞浜?`V4L2_CID_BAND_STOP_FILTER` 鏂囨。銆?

7. Added FM Modulator (FM TX) Extended Control Class:
   `V4L2_CTRL_CLASS_FM_TX` and their Control IDs.

7. 鏂板浜?FM 璋冨埗鍣紙FM TX锛夋墿灞曟帶鍒剁被锛歚V4L2_CTRL_CLASS_FM_TX` 鍙?
   鍏舵帶鍒?ID銆?

8. Added FM Receiver (FM RX) Extended Control Class:
   `V4L2_CTRL_CLASS_FM_RX` and their Control IDs.

8. 鏂板浜?FM 鎺ユ敹鍣紙FM RX锛夋墿灞曟帶鍒剁被锛歚V4L2_CTRL_CLASS_FM_RX` 鍙?
   鍏舵帶鍒?ID銆?

9. Added Remote Controller chapter, describing the default Remote
   Controller mapping for media devices.

9. 鏂板浜?閬ユ帶鍣?锛圧emote Controller锛夌珷鑺傦紝鎻忚堪濯掍綋璁惧鐨勯粯璁?
   閬ユ帶鍣ㄦ槧灏勩€?

## V4L2 in Linux 2.6.33

## Linux 2.6.33 涓殑 V4L2


1. Added support for Digital Video timings in order to support HDTV
   receivers and transmitters.

1. 鏂板浜嗗鏁板瓧瑙嗛锛圖igital Video锛夋椂搴忕殑鏀寔锛屼互渚挎敮鎸?HDTV
   鎺ユ敹鍣ㄥ拰鍙戦€佸櫒銆?

## V4L2 in Linux 2.6.34

## Linux 2.6.34 涓殑 V4L2


1. Added `V4L2_CID_IRIS_ABSOLUTE` and `V4L2_CID_IRIS_RELATIVE`
   controls to the Camera controls class <camera-controls>.

1. 鍚戠浉鏈烘帶鍒剁被 <camera-controls> 涓柊澧炰簡
   `V4L2_CID_IRIS_ABSOLUTE` 鍜?`V4L2_CID_IRIS_RELATIVE` 鎺у埗椤广€?

## V4L2 in Linux 2.6.37

## Linux 2.6.37 涓殑 V4L2


1. Remove the vtx (videotext/teletext) API. This API was no longer used
   and no hardware exists to verify the API. Nor were any userspace
   applications found that used it. It was originally scheduled for
   removal in 2.6.35.

1. 绉婚櫎浜?vtx锛坴ideotext/teletext锛堿PI銆傝 API 宸蹭笉鍐嶈浣跨敤锛屼篃娌℃湁
   鍙敤浜庨獙璇佽 API 鐨勭‖浠躲€備篃鏈彂鐜颁换浣曚娇鐢ㄥ畠鐨勭敤鎴锋€佸簲鐢ㄧ▼搴忋€?
   瀹冨師鏈鍒掑湪 2.6.35 涓Щ闄ゃ€?

## V4L2 in Linux 2.6.39

## Linux 2.6.39 涓殑 V4L2


1. The old VIDIOC_*_OLD symbols and V4L1 support were removed.

1. 绉婚櫎浜嗘棫鐨?VIDIOC_*_OLD 绗﹀彿浠ュ強 V4L1 鏀寔銆?

2. Multi-planar API added. Does not affect the compatibility of current
   drivers and applications. See multi-planar API <planar-apis>
   for details.

2. 鏂板浜嗗骞抽潰锛坢ulti-planar锛堿PI銆備笉褰卞搷褰撳墠椹卞姩鍜屽簲鐢ㄧ▼搴忕殑
   鍏煎鎬с€傝瑙佸骞抽潰 API <planar-apis>銆?

## V4L2 in Linux 3.1

## Linux 3.1 涓殑 V4L2


1. VIDIOC_QUERYCAP now returns a per-subsystem version instead of a
   per-driver one.

1. VIDIOC_QUERYCAP 鐜板湪杩斿洖姣忎釜瀛愮郴缁燂紙per-subsystem锛夌殑鐗堟湰锛岃€岄潪
   姣忎釜椹卞姩锛坧er-driver锛夌殑鐗堟湰銆?

   Standardize an error code for invalid ioctl.

   涓烘棤鏁堢殑 ioctl 缁熶竴浜嗛敊璇爜銆?

   Added V4L2_CTRL_TYPE_BITMASK.

   鏂板浜?V4L2_CTRL_TYPE_BITMASK銆?

## V4L2 in Linux 3.2

## Linux 3.2 涓殑 V4L2


1. V4L2_CTRL_FLAG_VOLATILE was added to signal volatile controls to
   userspace.

1. 鏂板浜?V4L2_CTRL_FLAG_VOLATILE锛岀敤浜庡悜鐢ㄦ埛鎬佹爣绀烘槗鍙橈紙volatile锛?
   鐨勬帶鍒堕」銆?

2. Add selection API for extended control over cropping and composing.
   Does not affect the compatibility of current drivers and
   applications. See selection API <selection-api> for details.

2. 鏂板浜嗛€夋嫨锛坰election锛堿PI锛岀敤浜庢墿灞曞瑁佸壀锛坈ropping锛夊拰鍚堟垚
   锛坈omposing锛夌殑鎺у埗銆備笉褰卞搷褰撳墠椹卞姩鍜屽簲鐢ㄧ▼搴忕殑鍏煎鎬с€傝瑙?
   閫夋嫨 API <selection-api>銆?

## V4L2 in Linux 3.3

## Linux 3.3 涓殑 V4L2


1. Added `V4L2_CID_ALPHA_COMPONENT` control to the
   User controls class <control>.

1. 鍚戠敤鎴锋帶鍒剁被 <control> 涓柊澧炰簡 `V4L2_CID_ALPHA_COMPONENT` 鎺у埗椤广€?

2. Added the device_caps field to struct v4l2_capabilities and added
   the new V4L2_CAP_DEVICE_CAPS capability.

2. 鍦?struct v4l2_capabilities 涓柊澧炰簡 device_caps 瀛楁锛屽苟鏂板浜?
   鏂扮殑 V4L2_CAP_DEVICE_CAPS 鑳藉姏銆?

## V4L2 in Linux 3.4

## Linux 3.4 涓殑 V4L2


1. Added JPEG compression control class <jpeg-controls>.

1. 鏂板浜?JPEG 鍘嬬缉鎺у埗绫?<jpeg-controls>銆?

2. Extended the DV Timings API:
   VIDIOC_ENUM_DV_TIMINGS,
   VIDIOC_QUERY_DV_TIMINGS and
   VIDIOC_DV_TIMINGS_CAP.

2. 鎵╁睍浜?DV Timings API锛氭柊澧炰簡 VIDIOC_ENUM_DV_TIMINGS銆?
   VIDIOC_QUERY_DV_TIMINGS 鍜?VIDIOC_DV_TIMINGS_CAP銆?

## V4L2 in Linux 3.5

## Linux 3.5 涓殑 V4L2


1. Added integer menus, the new type will be
   V4L2_CTRL_TYPE_INTEGER_MENU.

1. 鏂板浜嗘暣鏁拌彍鍗曪紝鏂扮被鍨嬩负 V4L2_CTRL_TYPE_INTEGER_MENU銆?

2. Added selection API for V4L2 subdev interface:
   VIDIOC_SUBDEV_G_SELECTION and
   VIDIOC_SUBDEV_S_SELECTION <VIDIOC_SUBDEV_G_SELECTION>.

2. 涓?V4L2 瀛愯澶囷紙subdev锛夋帴鍙ｆ柊澧炰簡閫夋嫨 API锛?
   VIDIOC_SUBDEV_G_SELECTION 鍜?
   VIDIOC_SUBDEV_S_SELECTION <VIDIOC_SUBDEV_G_SELECTION>銆?

3. Added `V4L2_COLORFX_ANTIQUE`, `V4L2_COLORFX_ART_FREEZE`,
   `V4L2_COLORFX_AQUA`, `V4L2_COLORFX_SILHOUETTE`,
   `V4L2_COLORFX_SOLARIZATION`, `V4L2_COLORFX_VIVID` and
   `V4L2_COLORFX_ARBITRARY_CBCR` menu items to the
   `V4L2_CID_COLORFX` control.

3. 鍚?`V4L2_CID_COLORFX` 鎺у埗椤规柊澧炰簡鑿滃崟椤?
   `V4L2_COLORFX_ANTIQUE`銆乣V4L2_COLORFX_ART_FREEZE`銆?
   `V4L2_COLORFX_AQUA`銆乣V4L2_COLORFX_SILHOUETTE`銆?
   `V4L2_COLORFX_SOLARIZATION`銆乣V4L2_COLORFX_VIVID` 鍜?
   `V4L2_COLORFX_ARBITRARY_CBCR`銆?

4. Added `V4L2_CID_COLORFX_CBCR` control.

4. 鏂板浜?`V4L2_CID_COLORFX_CBCR` 鎺у埗椤广€?

5. Added camera controls `V4L2_CID_AUTO_EXPOSURE_BIAS`,
   `V4L2_CID_AUTO_N_PRESET_WHITE_BALANCE`,
   `V4L2_CID_IMAGE_STABILIZATION`, `V4L2_CID_ISO_SENSITIVITY`,
   `V4L2_CID_ISO_SENSITIVITY_AUTO`, `V4L2_CID_EXPOSURE_METERING`,
   `V4L2_CID_SCENE_MODE`, `V4L2_CID_3A_LOCK`,
   `V4L2_CID_AUTO_FOCUS_START`, `V4L2_CID_AUTO_FOCUS_STOP`,
   `V4L2_CID_AUTO_FOCUS_STATUS` and `V4L2_CID_AUTO_FOCUS_RANGE`.

5. 鏂板浜嗙浉鏈烘帶鍒堕」 `V4L2_CID_AUTO_EXPOSURE_BIAS`銆?
   `V4L2_CID_AUTO_N_PRESET_WHITE_BALANCE`銆乣V4L2_CID_IMAGE_STABILIZATION`銆?
   `V4L2_CID_ISO_SENSITIVITY`銆乣V4L2_CID_ISO_SENSITIVITY_AUTO`銆?
   `V4L2_CID_EXPOSURE_METERING`銆乣V4L2_CID_SCENE_MODE`銆?
   `V4L2_CID_3A_LOCK`銆乣V4L2_CID_AUTO_FOCUS_START`銆?
   `V4L2_CID_AUTO_FOCUS_STOP`銆乣V4L2_CID_AUTO_FOCUS_STATUS` 鍜?
   `V4L2_CID_AUTO_FOCUS_RANGE`銆?

## V4L2 in Linux 3.6

## Linux 3.6 涓殑 V4L2


1. Replaced `input` in struct v4l2_buffer by
   `reserved2` and removed `V4L2_BUF_FLAG_INPUT`.

1. 灏?struct v4l2_buffer 涓殑 `input` 鏇挎崲涓?`reserved2`锛屽苟
   绉婚櫎浜?`V4L2_BUF_FLAG_INPUT`銆?

2. Added V4L2_CAP_VIDEO_M2M and V4L2_CAP_VIDEO_M2M_MPLANE
   capabilities.

2. 鏂板浜?V4L2_CAP_VIDEO_M2M 鍜?V4L2_CAP_VIDEO_M2M_MPLANE 鑳藉姏銆?

3. Added support for frequency band enumerations:
   VIDIOC_ENUM_FREQ_BANDS.

3. 鏂板浜嗗棰戝甫锛坒requency band锛夋灇涓剧殑鏀寔锛歏IDIOC_ENUM_FREQ_BANDS銆?

## V4L2 in Linux 3.9

## Linux 3.9 涓殑 V4L2


1. Added timestamp types to `flags` field in
   struct v4l2_buffer. See buffer-flags.

1. 鍦?struct v4l2_buffer 鐨?`flags` 瀛楁涓柊澧炰簡鏃堕棿鎴崇被鍨嬨€?
   璇﹁ buffer-flags銆?

2. Added `V4L2_EVENT_CTRL_CH_RANGE` control event changes flag. See
   ctrl-changes-flags.

2. 鏂板浜?`V4L2_EVENT_CTRL_CH_RANGE` 鎺у埗浜嬩欢鍙樻洿鏍囧織銆傝瑙?
   ctrl-changes-flags銆?

## V4L2 in Linux 3.10

## Linux 3.10 涓殑 V4L2


1. Removed obsolete and unused DV_PRESET ioctls VIDIOC_G_DV_PRESET,
   VIDIOC_S_DV_PRESET, VIDIOC_QUERY_DV_PRESET and
   VIDIOC_ENUM_DV_PRESET. Remove the related v4l2_input/output
   capability flags V4L2_IN_CAP_PRESETS and V4L2_OUT_CAP_PRESETS.

1. 绉婚櫎浜嗚繃鏃朵笖鏈娇鐢ㄧ殑 DV_PRESET ioctls锛歏IDIOC_G_DV_PRESET銆?
   VIDIOC_S_DV_PRESET銆乂IDIOC_QUERY_DV_PRESET 鍜?
   VIDIOC_ENUM_DV_PRESET銆傜Щ闄や簡鐩稿叧鐨?v4l2_input/output 鑳藉姏鏍囧織
   V4L2_IN_CAP_PRESETS 鍜?V4L2_OUT_CAP_PRESETS銆?

2. Added new debugging ioctl
   VIDIOC_DBG_G_CHIP_INFO.

2. 鏂板浜嗚皟璇?ioctl VIDIOC_DBG_G_CHIP_INFO銆?

## V4L2 in Linux 3.11

## Linux 3.11 涓殑 V4L2


1. Remove obsolete `VIDIOC_DBG_G_CHIP_IDENT` ioctl.

1. 绉婚櫎宸茶繃鏃剁殑 `VIDIOC_DBG_G_CHIP_IDENT` ioctl銆?

## V4L2 in Linux 3.14

## Linux 3.14 涓殑 V4L2


1. In struct v4l2_rect, the type of `width` and
   `height` fields changed from _s32 to _u32.

1. 鍦?struct v4l2_rect 涓紝`width` 鍜?`height` 瀛楁鐨勭被鍨嬩粠
   _s32 鏀逛负 _u32銆?

## V4L2 in Linux 3.15

## Linux 3.15 涓殑 V4L2


1. Added Software Defined Radio (SDR) Interface.

1. 鏂板浜嗚蒋浠跺畾涔夋棤绾跨數锛圫DR锛夋帴鍙ｃ€?

## V4L2 in Linux 3.16

## Linux 3.16 涓殑 V4L2


1. Added event V4L2_EVENT_SOURCE_CHANGE.

1. 鏂板浜嗕簨浠?V4L2_EVENT_SOURCE_CHANGE銆?

## V4L2 in Linux 3.17

## Linux 3.17 涓殑 V4L2


1. Extended struct v4l2_pix_format. Added
   format flags.

1. 鎵╁睍浜?struct v4l2_pix_format銆傛柊澧炰簡鏍煎紡鏍囧織銆?

2. Added compound control types and
   VIDIOC_QUERY_EXT_CTRL <VIDIOC_QUERYCTRL>.

2. 鏂板浜嗗鍚堬紙compound锛夋帶鍒剁被鍨嬩互鍙?
   VIDIOC_QUERY_EXT_CTRL <VIDIOC_QUERYCTRL>銆?

## V4L2 in Linux 3.18

## Linux 3.18 涓殑 V4L2


1. Added `V4L2_CID_PAN_SPEED` and `V4L2_CID_TILT_SPEED` camera
   controls.

1. 鏂板浜嗙浉鏈烘帶鍒堕」 `V4L2_CID_PAN_SPEED` 鍜?`V4L2_CID_TILT_SPEED`銆?

## V4L2 in Linux 3.19

## Linux 3.19 涓殑 V4L2


1. Rewrote Colorspace chapter, added new enum v4l2_ycbcr_encoding
   and enum v4l2_quantization fields to struct v4l2_pix_format,
   struct v4l2_pix_format_mplane and struct v4l2_mbus_framefmt.

1. 閲嶅啓浜?Colorspace 绔犺妭锛屽悜 struct v4l2_pix_format銆?
   struct v4l2_pix_format_mplane 鍜?struct v4l2_mbus_framefmt 涓?
   鏂板浜?enum v4l2_ycbcr_encoding 鍜?enum v4l2_quantization 瀛楁銆?

## V4L2 in Linux 4.4

## Linux 4.4 涓殑 V4L2


1. Renamed `V4L2_TUNER_ADC` to `V4L2_TUNER_SDR`. The use of
   `V4L2_TUNER_ADC` is deprecated now.

2. Added `V4L2_CID_RF_TUNER_RF_GAIN` RF Tuner control.

2. 鏂板浜?`V4L2_CID_RF_TUNER_RF_GAIN` RF 璋冭皭鍣ㄦ帶鍒堕」銆?

3. Added transmitter support for Software Defined Radio (SDR) Interface.

3. 鏂板浜嗗杞欢瀹氫箟鏃犵嚎鐢碉紙SDR锛夋帴鍙ｇ殑鍙戝皠鍣紙transmitter锛夋敮鎸併€?


## Relation of V4L2 to other Linux multimedia APIs

## V4L2 涓庡叾浠?Linux 澶氬獟浣?API 鐨勫叧绯?


### X Video Extension

### X Video 鎵╁睍


The X Video Extension (abbreviated XVideo or just Xv) is an extension of
the X Window system, implemented for example by the XFree86 project. Its
scope is similar to V4L2, an API to video capture and output devices for
X clients. Xv allows applications to display live video in a window,
send window contents to a TV output, and capture or output still images
in XPixmaps [#f1]_. With their implementation XFree86 makes the extension
available across many operating systems and architectures.

X Video 鎵╁睍锛堢畝绉?XVideo 鎴?Xv锛夋槸 X Window 绯荤粺鐨勪竴涓墿灞曪紝鐢?
XFree86 椤圭洰绛夊疄鐜般€傚畠鐨勪綔鐢ㄨ寖鍥翠笌 V4L2 绫讳技锛岄兘鏄潰鍚?X 瀹㈡埛绔殑
瑙嗛閲囬泦鍜岃緭鍑鸿澶?API銆俋v 鍏佽搴旂敤绋嬪簭鍦ㄧ獥鍙ｄ腑鏄剧ず瀹炴椂瑙嗛銆佸皢
绐楀彛鍐呭鍙戦€佸埌鐢佃杈撳嚭锛屽苟鍦?XPixmaps [#f1]_ 涓噰闆嗘垨杈撳嚭闈欐€佸浘鍍忋€?
閫氳繃 XFree86 鐨勫疄鐜帮紝璇ユ墿灞曞湪璁稿鎿嶄綔绯荤粺鍜屾灦鏋勪笂鍧囧彲浣跨敤銆?

Because the driver is embedded into the X server Xv has a number of
advantages over the V4L2 video overlay interface <overlay>. The
driver can easily determine the overlay target, i. e. visible graphics
memory or off-screen buffers for a destructive overlay. It can program
the RAMDAC for a non-destructive overlay, scaling or color-keying, or
the clipping functions of the video capture hardware, always in sync
with drawing operations or windows moving or changing their stacking
order.

鐢变簬椹卞姩宓屽叆鍦?X 鏈嶅姟鍣ㄤ腑锛孹v 鐩稿浜?V4L2 瑙嗛鍙犲姞锛坥verlay锛夋帴鍙?
<overlay> 鍏锋湁鑻ュ共浼樺娍銆傞┍鍔ㄥ彲浠ヨ交鏉惧湴纭畾鍙犲姞鐩爣锛屽嵆鍙鐨?
鍥惧舰鏄惧瓨鎴栫敤浜庣牬鍧忔€у彔鍔狅紙destructive overlay锛夌殑绂诲睆缂撳啿鍖恒€傚畠
鍙互缂栫▼ RAMDAC 浠ュ疄鐜伴潪鐮村潖鎬у彔鍔犮€佺缉鏀炬垨鑹查敭锛坈olor-keying锛夛紝
鎴栬€呭埄鐢ㄨ棰戦噰闆嗙‖浠剁殑瑁佸壀鍔熻兘锛屽苟涓斿缁堜笌缁樺浘鎿嶄綔鎴栫獥鍙ｇЩ鍔ㄣ€?
鏀瑰彉鍫嗗彔椤哄簭淇濇寔鍚屾銆?

To combine the advantages of Xv and V4L a special Xv driver exists in
XFree86 and XOrg, just programming any overlay capable Video4Linux
device it finds. To enable it `/etc/X11/XF86Config` must contain these
lines:

涓轰簡缁撳悎 Xv 涓?V4L 鐨勪紭鍔匡紝XFree86 鍜?XOrg 涓瓨鍦ㄤ竴涓壒娈婄殑 Xv 椹卞姩锛?
瀹冧細瀵瑰畠鍙戠幇鐨勪换浣曟敮鎸佸彔鍔犵殑 Video4Linux 璁惧杩涜缂栫▼銆傝鍚敤瀹冿紝
`/etc/X11/XF86Config` 蹇呴』鍖呭惈浠ヤ笅琛岋細

```
    Section "Module"
	Load "v4l"
    EndSection
```

As of XFree86 4.2 this driver still supports only V4L ioctls, however it
should work just fine with all V4L2 devices through the V4L2
backward-compatibility layer. Since V4L2 permits multiple opens it is
possible (if supported by the V4L2 driver) to capture video while an X
client requested video overlay. Restrictions of simultaneous capturing
and overlay are discussed in overlay apply.

鎴嚦 XFree86 4.2锛岃椹卞姩浠嶇劧鍙敮鎸?V4L ioctls锛屼笉杩囬€氳繃 V4L2 鐨?
鍚戝悗鍏煎灞傦紝瀹冨簲璇ヨ兘涓庢墍鏈?V4L2 璁惧姝ｅ父宸ヤ綔銆傜敱浜?V4L2 鍏佽澶氭
鎵撳紑锛屽洜姝わ紙鍦?V4L2 椹卞姩鏀寔鐨勬儏鍐典笅锛夊彲浠ュ湪 X 瀹㈡埛绔姹傝棰戝彔鍔犵殑
鍚屾椂閲囬泦瑙嗛銆傚悓鏃堕噰闆嗗拰鍙犲姞鐨勯檺鍒跺湪 overlay apply 涓璁恒€?

Only marginally related to V4L2, XFree86 extended Xv to support hardware
YUV to RGB conversion and scaling for faster video playback, and added
an interface to MPEG-2 decoding hardware. This API is useful to display
images captured with V4L2 devices.

涓?V4L2 浠呮湁灏戣鍏宠仈锛孹Free86 鎵╁睍浜?Xv 浠ユ敮鎸佺‖浠?YUV 鍒?RGB 杞崲鍜?
缂╂斁锛屼粠鑰屽疄鐜版洿蹇殑瑙嗛鎾斁锛屽苟澧炲姞浜嗗 MPEG-2 瑙ｇ爜纭欢鐨勬帴鍙ｃ€?
璇?API 鍙敤浜庢樉绀虹敤 V4L2 璁惧閲囬泦鐨勫浘鍍忋€?

### Digital Video

### 鏁板瓧瑙嗛


V4L2 does not support digital terrestrial, cable or satellite broadcast.
A separate project aiming at digital receivers exists. You can find its
homepage at `https://linuxtv.org <https://linuxtv.org>`__. The Linux
DVB API has no connection to the V4L2 API except that drivers for hybrid
hardware may support both.

V4L2 涓嶆敮鎸佹暟瀛楀湴闈€佹湁绾挎垨鍗槦骞挎挱銆傚瓨鍦ㄤ竴涓潰鍚戞暟瀛楁帴鏀跺櫒鐨勭嫭绔?
椤圭洰銆備綘鍙互鍦?`https://linuxtv.org <https://linuxtv.org>`__ 鎵惧埌瀹冪殑
涓婚〉銆侺inux DVB API 涓?V4L2 API 娌℃湁鍏崇郴锛屽彧鏄贩鍚堬紙hybrid锛夌‖浠剁殑
椹卞姩鍙兘鍚屾椂鏀寔涓よ€呫€?

### Audio Interfaces

### 闊抽鎺ュ彛


[to do - OSS/ALSA]

[寰呭姙 - OSS/ALSA]


## Experimental API Elements

## 瀹為獙鎬?API 鍏冪礌


The following V4L2 API elements are currently experimental and may
change in the future.

浠ヤ笅 V4L2 API 鍏冪礌鐩墠鏄疄楠屾€х殑锛屽皢鏉ュ彲鑳戒細鍙戠敓鍙樺寲銆?

- VIDIOC_DBG_G_REGISTER and
   VIDIOC_DBG_S_REGISTER <VIDIOC_DBG_G_REGISTER> ioctls.

- VIDIOC_DBG_G_REGISTER 鍜?
   VIDIOC_DBG_S_REGISTER <VIDIOC_DBG_G_REGISTER> ioctls銆?

- VIDIOC_DBG_G_CHIP_INFO ioctl.

- VIDIOC_DBG_G_CHIP_INFO ioctl銆?


## Obsolete API Elements

## 宸插簾寮冪殑 API 鍏冪礌


The following V4L2 API elements were superseded by new interfaces and
should not be implemented in new drivers.

浠ヤ笅 V4L2 API 鍏冪礌宸茶鏂版帴鍙ｅ彇浠ｏ紝涓嶅簲鍦ㄦ柊椹卞姩涓疄鐜般€?

- `VIDIOC_G_MPEGCOMP` and `VIDIOC_S_MPEGCOMP` ioctls. Use Extended
   Controls, extended-controls.

- `VIDIOC_G_MPEGCOMP` 鍜?`VIDIOC_S_MPEGCOMP` ioctls銆傝浣跨敤鎵╁睍
   鎺у埗锛圗xtended Controls锛夛紝extended-controls銆?

- VIDIOC_G_DV_PRESET, VIDIOC_S_DV_PRESET,
   VIDIOC_ENUM_DV_PRESETS and VIDIOC_QUERY_DV_PRESET ioctls. Use
   the DV Timings API (dv-timings).

- VIDIOC_G_DV_PRESET銆乂IDIOC_S_DV_PRESET銆?
   VIDIOC_ENUM_DV_PRESETS 鍜?VIDIOC_QUERY_DV_PRESET ioctls銆傝浣跨敤
   DV Timings API锛坉v-timings锛夈€?

- `VIDIOC_SUBDEV_G_CROP` and `VIDIOC_SUBDEV_S_CROP` ioctls. Use
   `VIDIOC_SUBDEV_G_SELECTION` and `VIDIOC_SUBDEV_S_SELECTION`,
   VIDIOC_SUBDEV_G_SELECTION.

- `VIDIOC_SUBDEV_G_CROP` 鍜?`VIDIOC_SUBDEV_S_CROP` ioctls銆傝浣跨敤
   `VIDIOC_SUBDEV_G_SELECTION` 鍜?`VIDIOC_SUBDEV_S_SELECTION`锛?
   VIDIOC_SUBDEV_G_SELECTION銆?

   This is not implemented in XFree86.

   杩欏湪 XFree86 涓湭瀹炵幇銆?

