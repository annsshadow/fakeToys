

######## ioctl VIDIOC_QBUF, VIDIOC_DQBUF


## 鍚嶇О


VIDIOC_QBUF - VIDIOC_DQBUF - 涓庨┍鍔ㄤ氦鎹竴涓紦鍐插尯

## 姒傝


`int ioctl(int fd, VIDIOC_QBUF, struct v4l2_buffer *argp)`


`int ioctl(int fd, VIDIOC_DQBUF, struct v4l2_buffer *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_buffer` 鐨勬寚閽堛€?
## 璇存槑


搴旂敤绋嬪簭璋冪敤 `VIDIOC_QBUF` ioctl 灏嗙┖鐨勶紙capture 鎹曡幏锛夋垨宸插～鍏呯殑锛坥utput 杈撳嚭锛夌紦鍐插尯鏀惧叆椹卞姩鐨勫叆闃熼槦鍒椼€傚叾璇箟鍙栧喅浜庢墍閫夋嫨鐨?I/O 鏂规硶銆?
瑕佸叆闃熶竴涓紦鍐插尯锛屽簲鐢ㄧ▼搴忓皢 struct `v4l2_buffer` 鐨?`type` 瀛楁璁句负涔嬪墠涓?struct `v4l2_format` 鐨?`type` 浠ュ強 struct `v4l2_requestbuffers` 鐨?`type` 鎵€鐢ㄨ繃鐨勭浉鍚岀紦鍐插尯绫诲瀷銆傚簲鐢ㄧ▼搴忚繕蹇呴』璁剧疆 `index` 瀛楁銆傛湁鏁堢殑绱㈠紩鍙疯寖鍥翠粠 0 鍒扮敤 VIDIOC_REQBUFS锛坰truct `v4l2_requestbuffers` 鐨?`count`锛夊垎閰嶇殑缂撳啿鍖烘暟閲忓噺涓€銆俈IDIOC_QUERYBUF ioctl 杩斿洖鐨?struct `v4l2_buffer` 鐨勫唴瀹瑰悓鏍峰彲鐢ㄣ€傚綋缂撳啿鍖虹敤浜庤緭鍑猴紙`type` 涓?`V4L2_BUF_TYPE_VIDEO_OUTPUT`銆乣V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE` 鎴?`V4L2_BUF_TYPE_VBI_OUTPUT`锛夋椂锛屽簲鐢ㄧ▼搴忚繕蹇呴』鍒濆鍖?`bytesused`銆乣field` 鍜?`timestamp` 瀛楁锛岃瑙?buffer銆傚簲鐢ㄧ▼搴忚繕蹇呴』灏?`flags` 璁句负 0銆俙reserved2` 鍜?`reserved` 瀛楁蹇呴』璁句负 0銆傚綋浣跨敤澶氬钩闈?API <planar-apis> 鏃讹紝`m.planes` 瀛楁蹇呴』鍖呭惈涓€涓寚鍚戝凡濉厖鐨?struct `v4l2_plane` 鏁扮粍鐨勭敤鎴风┖闂存寚閽堬紝涓?`length` 瀛楁蹇呴』璁句负璇ユ暟缁勭殑鍏冪礌涓暟銆?
瑕佸叆闃熶竴涓唴瀛樻槧灏?<mmap> 缂撳啿鍖猴紝搴旂敤绋嬪簭灏?`memory` 瀛楁璁句负 `V4L2_MEMORY_MMAP`銆傚綋鐢ㄦ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_QBUF` 鏃讹紝椹卞姩浼氳缃?`V4L2_BUF_FLAG_MAPPED` 鍜?`V4L2_BUF_FLAG_QUEUED` 鏍囧織锛屽苟娓呴櫎 `flags` 瀛楁涓殑 `V4L2_BUF_FLAG_DONE` 鏍囧織锛屽惁鍒欒繑鍥?`EINVAL` 閿欒鐮併€?
瑕佸叆闃熶竴涓敤鎴锋寚閽?<userp> 缂撳啿鍖猴紝搴旂敤绋嬪簭灏?`memory` 瀛楁璁句负 `V4L2_MEMORY_USERPTR`锛屽皢 `m.userptr` 瀛楁璁句负缂撳啿鍖虹殑鍦板潃锛屽苟灏?`length` 璁句负鍏跺ぇ灏忋€傚綋浣跨敤澶氬钩闈?API 鏃讹紝蹇呴』鏀圭敤鎵€浼犲叆鐨?struct `v4l2_plane` 鏁扮粍鐨?`m.userptr` 鍜?`length` 鎴愬憳銆傚綋鐢ㄦ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_QBUF` 鏃讹紝椹卞姩浼氳缃?`V4L2_BUF_FLAG_QUEUED` 鏍囧織骞舵竻闄?`flags` 瀛楁涓殑 `V4L2_BUF_FLAG_MAPPED` 鍜?`V4L2_BUF_FLAG_DONE` 鏍囧織锛屽惁鍒欒繑鍥為敊璇爜銆傝 ioctl 浼氬皢缂撳啿鍖虹殑鐗╃悊鍐呭瓨椤甸攣瀹氾紝瀹冧滑涓嶈兘琚崲鍑哄埌纾佺洏銆傜紦鍐插尯浼氫竴鐩翠繚鎸侀攣瀹氾紝鐩村埌琚嚭闃熴€佽皟鐢?VIDIOC_STREAMOFF <VIDIOC_STREAMON> 鎴?VIDIOC_REQBUFS ioctl锛屾垨鑰呰澶囪鍏抽棴銆?
瑕佸叆闃熶竴涓?DMABUF <dmabuf> 缂撳啿鍖猴紝搴旂敤绋嬪簭灏?`memory` 瀛楁璁句负 `V4L2_MEMORY_DMABUF`锛屽苟灏?`m.fd` 瀛楁璁句负涓€涓笌 DMABUF 缂撳啿鍖虹浉鍏宠仈鐨勬枃浠舵弿杩扮銆傚綋浣跨敤澶氬钩闈?API 鏃讹紝蹇呴』鏀圭敤鎵€浼犲叆鐨?struct `v4l2_plane` 鏁扮粍鐨?`m.fd` 瀛楁銆傚綋鐢ㄦ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_QBUF` 鏃讹紝椹卞姩浼氳缃?`V4L2_BUF_FLAG_QUEUED` 鏍囧織骞舵竻闄?`flags` 瀛楁涓殑 `V4L2_BUF_FLAG_MAPPED` 鍜?`V4L2_BUF_FLAG_DONE` 鏍囧織锛屽惁鍒欒繑鍥為敊璇爜銆傝 ioctl 浼氶攣瀹氱紦鍐插尯銆傞攣瀹氱紦鍐插尯鎰忓懗鐫€灏嗗叾浜ょ粰椹卞姩杩涜纭欢璁块棶锛堥€氬父鏄?DMA锛夈€傚鏋滃簲鐢ㄧ▼搴忚闂紙璇?鍐欙級涓€涓凡閿佸畾鐨勭紦鍐插尯锛岀粨鏋滄槸鏈畾涔夌殑銆傜紦鍐插尯浼氫竴鐩翠繚鎸侀攣瀹氾紝鐩村埌琚嚭闃熴€佽皟鐢?VIDIOC_STREAMOFF <VIDIOC_STREAMON> 鎴?VIDIOC_REQBUFS ioctl锛屾垨鑰呰澶囪鍏抽棴銆?
`request_fd` 瀛楁鍙互涓?`VIDIOC_QBUF` ioctl 涓€璧蜂娇鐢紝浠ユ寚瀹氫竴涓姹?<media-request-api> 鐨勬枃浠舵弿杩扮锛堝鏋滀娇鐢ㄤ簡璇锋眰锛夈€傝缃畠琛ㄧず鍦ㄨ璇锋眰鏈韩琚叆闃熶箣鍓嶏紝缂撳啿鍖轰笉浼氳浼犻€掔粰椹卞姩銆傛澶栵紝椹卞姩浼氬簲鐢ㄤ笌璇ヨ姹傚叧鑱旂殑銆侀拡瀵规缂撳啿鍖虹殑浠讳綍璁剧疆銆傞櫎闈炶缃簡 `V4L2_BUF_FLAG_REQUEST_FD` 鏍囧織锛屽惁鍒欒瀛楁浼氳蹇界暐銆傚鏋滆澶囦笉鏀寔璇锋眰锛屽垯杩斿洖 `EBADR`銆傚鏋滄敮鎸佽姹備絾缁欏嚭浜嗘棤鏁堢殑璇锋眰鏂囦欢鎻忚堪绗︼紝鍒欒繑鍥?`EINVAL`銆?
   涓嶅厑璁稿皢璇锋眰鍏ラ槦涓庣洿鎺ュ叆闃熺紦鍐插尯娣风敤銆傚鏋滅涓€涓紦鍐插尯鏄洿鎺ュ叆闃熺殑锛岀劧鍚庡簲鐢ㄧ▼搴忓張灏濊瘯鍏ラ槦涓€涓姹傦紝鎴栬€呭弽涔嬶紝鍒欒繑鍥?`EBUSY`銆傚湪鍏抽棴鏂囦欢鎻忚堪绗︺€佽皟鐢?VIDIOC_STREAMOFF <VIDIOC_STREAMON> 鎴栬皟鐢?VIDIOC_REQBUFS 涔嬪悗锛屾椤规鏌ヤ細琚噸缃€?
   瀵逛簬鍐呭瓨鍒板唴瀛樿澶?<mem2mem>锛屼綘鍙兘涓鸿緭鍑虹紦鍐插尯鎸囧畾 `request_fd`锛屼笉鑳戒负鎹曡幏缂撳啿鍖烘寚瀹氥€傝嫢灏濊瘯涓烘崟鑾风紦鍐插尯鎸囧畾锛屼細瀵艰嚧 `EBADR` 閿欒銆?
搴旂敤绋嬪簭璋冪敤 `VIDIOC_DQBUF` ioctl 浠庨┍鍔ㄧ殑鍑洪槦闃熷垪涓彇鍑轰竴涓凡濉厖锛坈apture 鎹曡幏锛夋垨宸叉樉绀猴紙output 杈撳嚭锛夌殑缂撳啿鍖恒€傚畠浠彧闇€鎸変笂杩版柟寮忚缃?struct `v4l2_buffer` 鐨?`type`銆乣memory` 鍜?`reserved` 瀛楁锛屽綋鐢ㄦ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_DQBUF` 鏃讹紝椹卞姩浼氬～鍏呮墍鏈夊墿浣欏瓧娈碉紝鍚﹀垯杩斿洖閿欒鐮併€傞┍鍔ㄤ篃鍙兘鍦?`flags` 瀛楁涓缃?`V4L2_BUF_FLAG_ERROR`銆傚畠琛ㄧず闈炶嚧鍛斤紙鍙仮澶嶏級鐨勬祦閿欒銆傚湪杩欑鎯呭喌涓嬶紝搴旂敤绋嬪簭鍙互鐓у父缁х画锛屼絾搴斿綋娉ㄦ剰鍑洪槦缂撳啿鍖轰腑鐨勬暟鎹彲鑳藉凡琚牬鍧忋€備娇鐢ㄥ骞抽潰 API 鏃讹紝涔熷繀椤讳紶鍏?planes 鏁扮粍銆?
濡傛灉搴旂敤绋嬪簭灏?`memory` 瀛楁璁句负 `V4L2_MEMORY_DMABUF` 浠ュ嚭闃熶竴涓?DMABUF <dmabuf> 缂撳啿鍖猴紝椹卞姩浼氬皢 `m.fd` 瀛楁濉厖涓轰竴涓湪鏁板€间笂涓庣紦鍐插尯鍏ラ槦鏃舵彁渚涚粰 `VIDIOC_QBUF` 鐨勬枃浠舵弿杩扮鐩稿悓鐨勬枃浠舵弿杩扮銆傚嚭闃熸椂涓嶄細鍒涘缓鏂扮殑鏂囦欢鎻忚堪绗︼紝璇ュ€间粎渚涘簲鐢ㄧ▼搴忔柟渚夸娇鐢ㄣ€備娇鐢ㄥ骞抽潰 API 鏃讹紝鏀逛负濉厖鎵€浼犲叆鐨?struct `v4l2_plane` 鏁扮粍鐨?`m.fd` 瀛楁銆?
榛樿鎯呭喌涓嬶紝褰撳嚭闃熼槦鍒椾腑娌℃湁缂撳啿鍖烘椂 `VIDIOC_DQBUF` 浼氶樆濉炪€傚綋 `open()` 鍑芥暟琚粰浜?`O_NONBLOCK` 鏍囧織鏃讹紝鑻ユ病鏈夊彲鐢ㄧ紦鍐插尯锛宍VIDIOC_DQBUF` 浼氱珛鍗宠繑鍥?`EAGAIN` 閿欒鐮併€?
struct `v4l2_buffer` 缁撴瀯鐨勫畾涔夎 buffer銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 涓€绔犱腑鎻忚堪銆?
EAGAIN
    宸蹭娇鐢?`O_NONBLOCK` 閫夋嫨浜嗛潪闃诲 I/O锛岃€屽嚭闃熼槦鍒椾腑娌℃湁缂撳啿鍖恒€?
EINVAL
    涓嶆敮鎸佺紦鍐插尯 `type`锛屾垨 `index` 瓒婄晫锛屾垨灏氭湭鍒嗛厤浠讳綍缂撳啿鍖猴紝鎴?`userptr` 鎴?`length` 鏃犳晥锛屾垨璁剧疆浜?`V4L2_BUF_FLAG_REQUEST_FD` 鏍囧織浣嗙粰瀹氱殑 `request_fd` 鏃犳晥锛屾垨 `m.fd` 鏄棤鏁堢殑 DMABUF 鏂囦欢鎻忚堪绗︺€?
EIO
    `VIDIOC_DQBUF` 鍥犲唴閮ㄩ敊璇€屽け璐ャ€備篃鍙兘琛ㄧず淇″彿涓㈠け绛変复鏃舵€ч棶棰樸€?
```
       The driver might dequeue an (empty) buffer despite returning
       an error, or even stop capturing. Reusing such buffer may be unsafe
       though and its details (e.g. ``index``) may not be returned either.
       It is recommended that drivers indicate recoverable errors by setting
       the ``V4L2_BUF_FLAG_ERROR`` and returning 0 instead. In that case the
       application should be able to safely reuse the buffer and continue
       streaming.

```
EPIPE
    `VIDIOC_DQBUF` 鍦ㄧ┖鎹曡幏闃熷垪涓婇拡瀵?mem2mem 缂栬В鐮佸櫒杩斿洖姝ら敊璇紝鏉′欢鏄甫鏈?`V4L2_BUF_FLAG_LAST` 鐨勭紦鍐插尯宸茶鍑洪槦涓旈璁′笉浼氭湁鏂扮紦鍐插尯鍙敤銆?
EBADR
    璁剧疆浜?`V4L2_BUF_FLAG_REQUEST_FD` 鏍囧織浣嗚澶囦笉鏀寔璇ョ粰瀹氱紦鍐插尯绫诲瀷鐨勮姹傦紝鎴栬€呮湭璁剧疆 `V4L2_BUF_FLAG_REQUEST_FD` 鏍囧織浣嗚澶囪姹傝缂撳啿鍖烘槸鏌愪釜璇锋眰鐨勪竴閮ㄥ垎銆?
EBUSY
    绗竴涓紦鍐插尯鏄€氳繃璇锋眰鍏ラ槦鐨勶紝浣嗗簲鐢ㄧ▼搴忕幇鍦ㄥ皾璇曠洿鎺ュ叆闃熷畠锛屾垨鑰呭弽涔嬶紙涓嶅厑璁告贩鐢ㄨ繖涓ょ API锛夈€?