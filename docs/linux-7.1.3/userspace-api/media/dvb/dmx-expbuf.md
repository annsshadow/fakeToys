


######## ioctl DMX_EXPBUF


## 鍚嶇О


DMX_EXPBUF - 灏嗕竴涓紦鍐插尯瀵煎嚭涓?DMABUF 鏂囦欢鎻忚堪绗︺€?

## 姒傝


`int ioctl(int fd, DMX_EXPBUF, struct dmx_exportbuffer *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `dmx_exportbuffer` 鐨勬寚閽堛€?
## 鎻忚堪


璇?ioctl 鏄唴瀛樻槧灏?I/O 鏂规硶鐨勬墿灞曘€?瀹冨彲鐢ㄤ簬鍦ㄩ€氳繃 DMX_REQBUFS ioctl 鍒嗛厤缂撳啿鍖轰箣鍚庣殑浠绘剰鏃跺埢锛屽皢涓€涓紦鍐插尯瀵煎嚭涓?DMABUF 鏂囦欢銆?
瑕佸鍑轰竴涓紦鍐插尯锛屽簲鐢ㄧ▼搴忛渶瑕佸～鍏?struct `dmx_exportbuffer`銆?搴旂敤绋嬪簭蹇呴』璁剧疆 `index` 瀛楁銆傛湁鏁堢殑绱㈠紩缂栧彿鑼冨洿浠庨浂鍒颁娇鐢?DMX_REQBUFS 鍒嗛厤鐨勭紦鍐插尯鏁伴噺锛坰truct `dmx_requestbuffers` 鐨?`count`锛夊噺涓€銆?鍙互鍦?`flags` 瀛楁涓缃澶栫殑鏍囧織銆傛湁鍏宠缁嗕俊鎭紝璇峰弬鑰?open() 鐨勬墜鍐岄〉銆傜洰鍓嶄粎鏀寔 O_CLOEXEC銆丱_RDONLY銆丱_WRONLY 鍜?O_RDWR銆?鎵€鏈夊叾浠栧瓧娈靛繀椤昏缃负闆躲€傚湪澶氬钩闈紙multi-planar锛堿PI 鐨勬儏鍐典笅锛屾瘡涓钩闈㈤兘閫氳繃澶氭 DMX_EXPBUF 璋冪敤鍒嗗埆瀵煎嚭銆?
璋冪敤 DMX_EXPBUF 鍚庯紝鑻ユ垚鍔燂紝`fd` 瀛楁灏嗚椹卞姩璁剧疆銆傝繖鏄竴涓?DMABUF 鏂囦欢鎻忚堪绗︺€傚簲鐢ㄧ▼搴忓彲浠ュ皢鍏朵紶閫掔粰鍏朵粬鏀寔 DMABUF 鐨勮澶囥€傚缓璁湪涓嶄娇鐢ㄨ DMABUF 鏂囦欢鏃跺皢鍏跺叧闂紝浠ヤ究鍥炴敹鐩稿叧鐨勫唴瀛樸€?
## 绀轰緥


    int buffer_export(int v4lfd, enum dmx_buf_type bt, int index, int *dmafd)
    {
	struct dmx_exportbuffer expbuf;

	memset(&expbuf, 0, sizeof(expbuf));
	expbuf.type = bt;
	expbuf.index = index;
	if (ioctl(v4lfd, DMX_EXPBUF, &expbuf) == -1) {
	    perror("DMX_EXPBUF");
	    return -1;
	}

	*dmafd = expbuf.fd;

	return 0;
    }

## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤鐨勯敊璇爜鍦?Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    闃熷垪涓嶅浜?MMAP 妯″紡锛屾垨涓嶆敮鎸?DMABUF 瀵煎嚭锛屾垨 `flags`銆乣index` 瀛楁鏃犳晥銆?