


######## ioctl VIDIOC_EXPBUF


## 鍚嶇О


VIDIOC_EXPBUF - 灏嗕竴涓紦鍐插尯瀵煎嚭涓?DMABUF 鏂囦欢鎻忚堪绗︺€?
## 姒傝


`int ioctl(int fd, VIDIOC_EXPBUF, struct v4l2_exportbuffer *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_exportbuffer` 鐨勬寚閽堛€?
## 鎻忚堪


璇?ioctl 鏄唴瀛樻槧灏?<mmap> I/O 鏂规硶鐨勬墿灞曪紝鍥犳浠呭 `V4L2_MEMORY_MMAP`
缂撳啿鍖哄彲鐢ㄣ€傚畠鍙互鍦ㄤ娇鐢?VIDIOC_REQBUFS ioctl 鍒嗛厤瀹岀紦鍐插尯涔嬪悗鐨勪换浣曟椂鍒伙紝
灏嗙紦鍐插尯瀵煎嚭涓轰竴涓?DMABUF 鏂囦欢銆?
瑕佸鍑虹紦鍐插尯锛屽簲鐢ㄧ▼搴忛渶瑕佸～鍐?struct `v4l2_exportbuffer`銆俙type` 瀛楁搴旇涓?涓庝箣鍓嶄娇鐢?struct `v4l2_requestbuffers` 鐨?`type` 鐩稿悓鐨勭紦鍐插尯绫诲瀷銆傚簲鐢ㄧ▼搴?杩樺繀椤昏缃?`index` 瀛楁銆傛湁鏁堢殑绱㈠紩鍙疯寖鍥翠粠闆跺埌鐢?VIDIOC_REQBUFS锛坰truct
`v4l2_requestbuffers` 鐨?`count`锛夊垎閰嶇殑缂撳啿鍖烘暟閲忓噺涓€銆傚浜庡骞抽潰锛坢ulti-planar锛?API锛屽簲鐢ㄧ▼搴忓皢 `plane` 瀛楁璁句负瑕佸鍑虹殑骞抽潰绱㈠紩銆傛湁鏁堝钩闈㈣寖鍥翠粠闆跺埌褰撳墠娲诲姩
鏍煎紡鏀寔鐨勬渶澶ф湁鏁堝钩闈㈡暟銆傚浜庡崟骞抽潰锛坰ingle-planar锛堿PI锛屽簲鐢ㄧ▼搴忓繀椤诲皢
`plane` 璁句负闆躲€傚彲浠ュ湪 `flags` 瀛楁涓缃澶栫殑鏍囧織锛屽叿浣撶粏鑺傚弬瑙?open() 鐨?鎵嬪唽銆傜洰鍓嶄粎鏀寔 O_CLOEXEC銆丱_RDONLY銆丱_WRONLY 鍜?O_RDWR銆傛墍鏈夊叾瀹冨瓧娈靛繀椤?璁句负闆躲€傚浜庡骞抽潰 API锛屾瘡涓钩闈㈤兘浣跨敤澶氭 VIDIOC_EXPBUF 璋冪敤鏉ュ垎鍒鍑恒€?
璋冪敤 VIDIOC_EXPBUF 鍚庯紝`fd` 瀛楁浼氳椹卞姩璁剧疆銆傝繖鏄竴涓?DMABUF 鏂囦欢鎻忚堪绗︺€?搴旂敤绋嬪簭鍙皢鍏朵紶閫掔粰鍏跺畠鏀寔 DMABUF 鐨勮澶囥€傚叧浜庡皢 DMABUF 鏂囦欢瀵煎叆 V4L2 鑺傜偣鐨?缁嗚妭锛岃鍙傝€?DMABUF importing <dmabuf>銆傚缓璁湪涓嶅啀浣跨敤鏌愪釜 DMABUF 鏂囦欢鏃跺叧闂畠锛?浠ヤ究鍥炴敹鐩稿叧鐨勫唴瀛樸€?
## 绀轰緥



    int buffer_export(int v4lfd, enum v4l2_buf_type bt, int index, int *dmafd)
    {
	struct v4l2_exportbuffer expbuf;

	memset(&expbuf, 0, sizeof(expbuf));
	expbuf.type = bt;
	expbuf.index = index;
	if (ioctl(v4lfd, VIDIOC_EXPBUF, &expbuf) == -1) {
	    perror("VIDIOC_EXPBUF");
	    return -1;
	}

	*dmafd = expbuf.fd;

	return 0;
    }


    int buffer_export_mp(int v4lfd, enum v4l2_buf_type bt, int index,
	int dmafd[], int n_planes)
    {
	int i;

	for (i = 0; i < n_planes; ++i) {
	    struct v4l2_exportbuffer expbuf;

	    memset(&expbuf, 0, sizeof(expbuf));
	    expbuf.type = bt;
	    expbuf.index = index;
	    expbuf.plane = i;
	    if (ioctl(v4lfd, VIDIOC_EXPBUF, &expbuf) == -1) {
		perror("VIDIOC_EXPBUF");
		while (i)
		    close(dmafd[--i]);
		return -1;
	    }
	    dmafd[i] = expbuf.fd;
	}

	return 0;
    }



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 缂撳啿鍖虹殑绫诲瀷锛屼笌 struct `v4l2_format` 鐨?`type` 鎴?struct
	`v4l2_requestbuffers` 鐨?`type` 鐩稿悓锛岀敱搴旂敤绋嬪簭璁剧疆銆傚弬瑙?`v4l2_buf_type`
    - - __u32
      - `index`
      - 缂撳啿鍖虹殑缂栧彿锛岀敱搴旂敤绋嬪簭璁剧疆銆傝瀛楁浠呯敤浜庡唴瀛樻槧灏?<mmap> I/O锛?	鑼冨洿鍙粠闆跺埌鐢?VIDIOC_REQBUFS 鍜?鎴?VIDIOC_CREATE_BUFS ioctl 鍒嗛厤鐨?	缂撳啿鍖烘暟閲忋€?    - - __u32
      - `plane`
      - 浣跨敤澶氬钩闈?API 鏃惰瀵煎嚭鐨勫钩闈㈢储寮曘€傚惁鍒欒鍊煎繀椤昏涓洪浂銆?    - - __u32
      - `flags`
      - 鏂板垱寤烘枃浠剁殑鏍囧織锛岀洰鍓嶄粎鏀寔 `O_CLOEXEC`銆乣O_RDONLY`銆乣O_WRONLY`
	鍜?`O_RDWR`锛屾洿澶氱粏鑺傝鍙傝€?open() 鐨勬墜鍐屻€?    - - __s32
      - `fd`
      - 涓庣紦鍐插尯鍏宠仈鐨?DMABUF 鏂囦欢鎻忚堪绗︺€傜敱椹卞姩璁剧疆銆?    - - __u32
      - `reserved[^11^]`
      - 淇濈暀瀛楁锛屼緵灏嗘潵浣跨敤銆傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍璁句负闆躲€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    闃熷垪涓嶅浜?MMAP 妯″紡锛屾垨鏄笉鏀寔 DMABUF 瀵煎嚭锛屾垨鑰?`flags`銆乣type`銆?    `index` 鎴?`plane` 瀛楁鏃犳晥銆?