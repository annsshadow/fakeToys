
## 鏂囦欢锛歮edia/v4l/capture.c

鏈枃浠舵槸 V4L2 瑙嗛閲囬泦绀轰緥绋嬪簭 capture.c 鐨勬簮鐮侊紝婕旂ず濡備綍浣跨敤 V4L2 API 鎵撳紑璁惧銆侀厤缃牸寮忥紝骞堕€氳繃 read銆乵map銆乽serptr 绛夋柟寮忔崟鑾疯棰戝抚锛屼緵鐢ㄦ埛绌洪棿搴旂敤寮€鍙戝弬鑰冦€?




    /*
     - V4L2 瑙嗛閲囬泦绀轰緥
     *
     - 鏈▼搴忓彲鍦ㄦ棤浠讳綍闄愬埗鐨勬儏鍐典笅浣跨敤鍜屽垎鍙戙€?
     *
     - 鏈▼搴忛殢 V4L2 API 涓€鍚屾彁渚?
     - 鏇村淇℃伅璇峰弬闃?https://linuxtv.org/docs.php
     */

    #鍖呭惈 <stdio.h>
    #鍖呭惈 <stdlib.h>
    #鍖呭惈 <瀛楃涓?h>
    #鍖呭惈 <assert.h>

    #鍖呭惈 <getopt.h>             /** getopt_long() 鍑芥暟 **/

    #鍖呭惈 <fcntl.h>              /** 搴曞眰 I/O **/
    #鍖呭惈 <unistd.h>
    #鍖呭惈 <errno.h>
    #鍖呭惈 <sys/stat.h>
    #鍖呭惈 <sys/types.h>
    #鍖呭惈 <sys/time.h>
    #鍖呭惈 <sys/mman.h>
    #鍖呭惈 <sys/ioctl.h>

    #鍖呭惈 <linux/videodev2.h>

    #瀹氫箟 CLEAR(x) memset(&(x), 0, sizeof(x))

    enum io_鏂规硶 {
	    IO_鏂规硶_璇诲彇,
	    IO_鏂规硶_MMAP,
	    IO_鏂规硶_USERPTR,
    };

    缁撴瀯浣?缂撳啿鍖?{
	    void   *鍚姩;
	    澶у皬_t  闀垮害;
    };

    闈欐€?char            *dev_name;
    闈欐€?enum io_鏂规硶   io = IO_鏂规硶_MMAP;
    闈欐€?int              fd = -1;
    缁撴瀯浣?缂撳啿鍖?         *缂撳啿鍖?
    闈欐€?unsigned int     n_缂撳啿鍖?
    闈欐€?int              out_buf;
    闈欐€?int              force_鏍煎紡;
    闈欐€?int              甯count = 70;

    闈欐€?void errno_exit(const char *s)
    {
	    fprintf(stderr, "%s 閿欒 %d, %s\n", s, errno, strerror(errno));
	    exit(EXIT_FAILURE);
    }

    闈欐€?int xioctl(int fh, int 璇锋眰, void *arg)
    {
	    int r;

	    鎵ц {
		    r = ioctl(fh, 璇锋眰, arg);
	    } 鍚屾椂 (-1 == r && EINTR == errno);

	    return r;
    }

    闈欐€?void 杩涚▼_image(const void *p, int 澶у皬)
    {
	    鑻?(out_buf)
		    fwrite(p, 澶у皬, 1, stdout);

	    fflush(stderr);
	    fprintf(stderr, ".");
	    fflush(stdout);
    }

    闈欐€?int 璇诲彇_甯?void)
    {
	    缁撴瀯浣?v4l2_缂撳啿鍖?buf;
	    unsigned int i;

	    switch (io) {
	    case IO_鏂规硶_璇诲彇:
		    鑻?(-1 == 璇诲彇(fd, 缂撳啿鍖篬^0^].鍚姩, 缂撳啿鍖篬^0^].闀垮害)) {
			    switch (errno) {
			    case EAGAIN:
				    return 0;

			    case EIO:
				    /** 鍙拷鐣?EIO锛屽弬瑙佽鑼冦€?**/

				    /** 璐┛锛坒all through锛?**/

			    榛樿:
				    errno_exit("璇诲彇");
			    }
		    }

		    杩涚▼_image(缂撳啿鍖篬^0^].鍚姩, 缂撳啿鍖篬^0^].闀垮害);
		    break;

	    case IO_鏂规硶_MMAP:
		    CLEAR(buf);

		    buf.绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
		    buf.鍐呭瓨 = V4L2_鍐呭瓨_MMAP;

		    鑻?(-1 == xioctl(fd, VIDIOC_DQBUF, &buf)) {
			    switch (errno) {
			    case EAGAIN:
				    return 0;

			    case EIO:
				    /** 鍙拷鐣?EIO锛屽弬瑙佽鑼冦€?**/

				    /** 璐┛锛坒all through锛?**/

			    榛樿:
				    errno_exit("VIDIOC_DQBUF");
			    }
		    }

		    assert(buf.绱㈠紩 < n_缂撳啿鍖?;

		    杩涚▼_image(缂撳啿鍖篬buf.绱㈠紩].鍚姩, buf.bytesused);

		    鑻?(-1 == xioctl(fd, VIDIOC_QBUF, &buf))
			    errno_exit("VIDIOC_QBUF");
		    break;

	    case IO_鏂规硶_USERPTR:
		    CLEAR(buf);

		    buf.绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
		    buf.鍐呭瓨 = V4L2_鍐呭瓨_USERPTR;

		    鑻?(-1 == xioctl(fd, VIDIOC_DQBUF, &buf)) {
			    switch (errno) {
			    case EAGAIN:
				    return 0;

			    case EIO:
				    /** 鍙拷鐣?EIO锛屽弬瑙佽鑼冦€?**/

				    /** 璐┛锛坒all through锛?**/

			    榛樿:
				    errno_exit("VIDIOC_DQBUF");
			    }
		    }

		    鐢ㄤ簬 (i = 0; i < n_缂撳啿鍖? ++i)
			    鑻?(buf.m.userptr == (unsigned long)缂撳啿鍖篬i].鍚姩
				&& buf.闀垮害 == 缂撳啿鍖篬i].闀垮害)
				    break;

		    assert(i < n_缂撳啿鍖?;

		    杩涚▼_image((void *)buf.m.userptr, buf.bytesused);

		    鑻?(-1 == xioctl(fd, VIDIOC_QBUF, &buf))
			    errno_exit("VIDIOC_QBUF");
		    break;
	    }

	    return 1;
    }

    闈欐€?void mainloop(void)
    {
	    unsigned int count;

	    count = 甯count;

	    鍚屾椂 (count-- > 0) {
		    鐢ㄤ簬 (;;) {
			    fd_set fds;
			    缁撴瀯浣?timeval tv;
			    int r;

			    FD_ZERO(&fds);
			    FD_SET(fd, &fds);

			    /** 瓒呮椂銆?**/
			    tv.tv_sec = 2;
			    tv.tv_usec = 0;

			    r = select(fd + 1, &fds, NULL, NULL, &tv);

			    鑻?(-1 == r) {
				    鑻?(EINTR == errno)
					    continue;
				    errno_exit("select");
			    }

			    鑻?(0 == r) {
				    fprintf(stderr, "select 瓒呮椂\n");
				    exit(EXIT_FAILURE);
			    }

			    鑻?(璇诲彇_甯?))
				    break;
			    /** EAGAIN - 缁х画 select 寰幆銆?**/
		    }
	    }
    }

    闈欐€?void 鍋滄_capturing(void)
    {
	    enum v4l2_buf_绫诲瀷 绫诲瀷;

	    switch (io) {
	    case IO_鏂规硶_璇诲彇:
		    /** 鏃犻渶鎿嶄綔銆?**/
		    break;

	    case IO_鏂规硶_MMAP:
	    case IO_鏂规硶_USERPTR:
		    绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
		    鑻?(-1 == xioctl(fd, VIDIOC_STREAMOFF, &绫诲瀷))
			    errno_exit("VIDIOC_STREAMOFF");
		    break;
	    }
    }

    闈欐€?void 鍚姩_capturing(void)
    {
	    unsigned int i;
	    enum v4l2_buf_绫诲瀷 绫诲瀷;

	    switch (io) {
	    case IO_鏂规硶_璇诲彇:
		    /** 鏃犻渶鎿嶄綔銆?**/
		    break;

	    case IO_鏂规硶_MMAP:
		    鐢ㄤ簬 (i = 0; i < n_缂撳啿鍖? ++i) {
			    缁撴瀯浣?v4l2_缂撳啿鍖?buf;

			    CLEAR(buf);
			    buf.绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
			    buf.鍐呭瓨 = V4L2_鍐呭瓨_MMAP;
			    buf.绱㈠紩 = i;

			    鑻?(-1 == xioctl(fd, VIDIOC_QBUF, &buf))
				    errno_exit("VIDIOC_QBUF");
		    }
		    绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
		    鑻?(-1 == xioctl(fd, VIDIOC_STREAMON, &绫诲瀷))
			    errno_exit("VIDIOC_STREAMON");
		    break;

	    case IO_鏂规硶_USERPTR:
		    鐢ㄤ簬 (i = 0; i < n_缂撳啿鍖? ++i) {
			    缁撴瀯浣?v4l2_缂撳啿鍖?buf;

			    CLEAR(buf);
			    buf.绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
			    buf.鍐呭瓨 = V4L2_鍐呭瓨_USERPTR;
			    buf.绱㈠紩 = i;
			    buf.m.userptr = (unsigned long)缂撳啿鍖篬i].鍚姩;
			    buf.闀垮害 = 缂撳啿鍖篬i].闀垮害;

			    鑻?(-1 == xioctl(fd, VIDIOC_QBUF, &buf))
				    errno_exit("VIDIOC_QBUF");
		    }
		    绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
		    鑻?(-1 == xioctl(fd, VIDIOC_STREAMON, &绫诲瀷))
			    errno_exit("VIDIOC_STREAMON");
		    break;
	    }
    }

    闈欐€?void uninit_璁惧(void)
    {
	    unsigned int i;

	    switch (io) {
	    case IO_鏂规硶_璇诲彇:
		    free(缂撳啿鍖篬^0^].鍚姩);
		    break;

	    case IO_鏂规硶_MMAP:
		    鐢ㄤ簬 (i = 0; i < n_缂撳啿鍖? ++i)
			    鑻?(-1 == munmap(缂撳啿鍖篬i].鍚姩, 缂撳啿鍖篬i].闀垮害))
				    errno_exit("munmap");
		    break;

	    case IO_鏂规硶_USERPTR:
		    鐢ㄤ簬 (i = 0; i < n_缂撳啿鍖? ++i)
			    free(缂撳啿鍖篬i].鍚姩);
		    break;
	    }

	    free(缂撳啿鍖?;
    }

    闈欐€?void 鍒濆鍖朹璇诲彇(unsigned int 缂撳啿鍖篲澶у皬)
    {
	    缂撳啿鍖?= calloc(1, sizeof(*缂撳啿鍖?);

	    鑻?(!缂撳啿鍖? {
		    fprintf(stderr, "瓒呭嚭 鍐呭瓨\n");
		    exit(EXIT_FAILURE);
	    }

	    缂撳啿鍖篬^0^].闀垮害 = 缂撳啿鍖篲澶у皬;
	    缂撳啿鍖篬^0^].鍚姩 = malloc(缂撳啿鍖篲澶у皬);

	    鑻?(!缂撳啿鍖篬^0^].鍚姩) {
		    fprintf(stderr, "瓒呭嚭 鍐呭瓨\n");
		    exit(EXIT_FAILURE);
	    }
    }

    闈欐€?void 鍒濆鍖朹mmap(void)
    {
	    缁撴瀯浣?v4l2_requestbuffers req;

	    CLEAR(req);

	    req.count = 4;
	    req.绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
	    req.鍐呭瓨 = V4L2_鍐呭瓨_MMAP;

	    鑻?(-1 == xioctl(fd, VIDIOC_REQBUFS, &req)) {
		    鑻?(EINVAL == errno) {
			    fprintf(stderr, "%s 鎵ц 涓?鏀寔 "
				     "鍐呭瓨 鏄犲皠\n", dev_name);
			    exit(EXIT_FAILURE);
		    } else {
			    errno_exit("VIDIOC_REQBUFS");
		    }
	    }

	    鑻?(req.count < 2) {
		    fprintf(stderr, "Insufficient 缂撳啿鍖?鍐呭瓨 鍦?%s\n",
			     dev_name);
		    exit(EXIT_FAILURE);
	    }

	    缂撳啿鍖?= calloc(req.count, sizeof(*缂撳啿鍖?);

	    鑻?(!缂撳啿鍖? {
		    fprintf(stderr, "瓒呭嚭 鍐呭瓨\n");
		    exit(EXIT_FAILURE);
	    }

	    鐢ㄤ簬 (n_缂撳啿鍖?= 0; n_缂撳啿鍖?< req.count; ++n_缂撳啿鍖? {
		    缁撴瀯浣?v4l2_缂撳啿鍖?buf;

		    CLEAR(buf);

		    buf.绫诲瀷        = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
		    buf.鍐呭瓨      = V4L2_鍐呭瓨_MMAP;
		    buf.绱㈠紩       = n_缂撳啿鍖?

		    鑻?(-1 == xioctl(fd, VIDIOC_QUERYBUF, &buf))
			    errno_exit("VIDIOC_QUERYBUF");

		    缂撳啿鍖篬n_缂撳啿鍖篯.闀垮害 = buf.闀垮害;
		    缂撳啿鍖篬n_缂撳啿鍖篯.鍚姩 =
			    mmap(NULL /** 浠绘剰浣嶇疆寮€濮?**/,
				  buf.闀垮害,
				  PROT_璇诲彇 | PROT_鍐欏叆 /** 蹇呴渶 **/,
				  MAP_SHARED /** 鎺ㄨ崘 **/,
				  fd, buf.m.鍋忕Щ);

		    鑻?(MAP_FAILED == 缂撳啿鍖篬n_缂撳啿鍖篯.鍚姩)
			    errno_exit("mmap");
	    }
    }

    闈欐€?void 鍒濆鍖朹userp(unsigned int 缂撳啿鍖篲澶у皬)
    {
	    缁撴瀯浣?v4l2_requestbuffers req;

	    CLEAR(req);

	    req.count  = 4;
	    req.绫诲瀷   = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
	    req.鍐呭瓨 = V4L2_鍐呭瓨_USERPTR;

	    鑻?(-1 == xioctl(fd, VIDIOC_REQBUFS, &req)) {
		    鑻?(EINVAL == errno) {
			    fprintf(stderr, "%s 鎵ц 涓?鏀寔 "
				     "鐢ㄦ埛 鎸囬拡 i/o\n", dev_name);
			    exit(EXIT_FAILURE);
		    } else {
			    errno_exit("VIDIOC_REQBUFS");
		    }
	    }

	    缂撳啿鍖?= calloc(4, sizeof(*缂撳啿鍖?);

	    鑻?(!缂撳啿鍖? {
		    fprintf(stderr, "瓒呭嚭 鍐呭瓨\n");
		    exit(EXIT_FAILURE);
	    }

	    鐢ㄤ簬 (n_缂撳啿鍖?= 0; n_缂撳啿鍖?< 4; ++n_缂撳啿鍖? {
		    缂撳啿鍖篬n_缂撳啿鍖篯.闀垮害 = 缂撳啿鍖篲澶у皬;
		    缂撳啿鍖篬n_缂撳啿鍖篯.鍚姩 = malloc(缂撳啿鍖篲澶у皬);

		    鑻?(!缂撳啿鍖篬n_缂撳啿鍖篯.鍚姩) {
			    fprintf(stderr, "瓒呭嚭 鍐呭瓨\n");
			    exit(EXIT_FAILURE);
		    }
	    }
    }

    闈欐€?void 鍒濆鍖朹璁惧(void)
    {
	    缁撴瀯浣?v4l2_capability cap;
	    缁撴瀯浣?v4l2_cropcap cropcap;
	    缁撴瀯浣?v4l2_crop crop;
	    缁撴瀯浣?v4l2_鏍煎紡 fmt;
	    unsigned int min;

	    鑻?(-1 == xioctl(fd, VIDIOC_QUERYCAP, &cap)) {
		    鑻?(EINVAL == errno) {
			    fprintf(stderr, "%s 鏄?鏃?V4L2 璁惧\n",
				     dev_name);
			    exit(EXIT_FAILURE);
		    } else {
			    errno_exit("VIDIOC_QUERYCAP");
		    }
	    }

	    鑻?(!(cap.capabilities & V4L2_CAP_瑙嗛_CAPTURE)) {
		    fprintf(stderr, "%s 鏄?鏃?瑙嗛 capture 璁惧\n",
			     dev_name);
		    exit(EXIT_FAILURE);
	    }

	    switch (io) {
	    case IO_鏂规硶_璇诲彇:
		    鑻?(!(cap.capabilities & V4L2_CAP_READWRITE)) {
			    fprintf(stderr, "%s 鎵ц 涓?鏀寔 璇诲彇 i/o\n",
				     dev_name);
			    exit(EXIT_FAILURE);
		    }
		    break;

	    case IO_鏂规硶_MMAP:
	    case IO_鏂规硶_USERPTR:
		    鑻?(!(cap.capabilities & V4L2_CAP_STREAMING)) {
			    fprintf(stderr, "%s 鎵ц 涓?鏀寔 streaming i/o\n",
				     dev_name);
			    exit(EXIT_FAILURE);
		    }
		    break;
	    }


	    /** 鍦ㄦ閫夋嫨瑙嗛杈撳叆銆佽棰戞爣鍑嗕笌璋冭皭銆?**/


	    CLEAR(cropcap);

	    cropcap.绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;

	    鑻?(0 == xioctl(fd, VIDIOC_CROPCAP, &cropcap)) {
		    crop.绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
		    crop.c = cropcap.defrect; /** 閲嶇疆涓洪粯璁ゅ€?**/

		    鑻?(-1 == xioctl(fd, VIDIOC_S_CROP, &crop)) {
			    switch (errno) {
			    case EINVAL:
				    /** 涓嶆敮鎸佽鍓€?**/
				    break;
			    榛樿:
				    /** 宸插拷鐣ラ敊璇€?**/
				    break;
			    }
		    }
	    } else {
		    /** 宸插拷鐣ラ敊璇€?**/
	    }


	    CLEAR(fmt);

	    fmt.绫诲瀷 = V4L2_BUF_绫诲瀷_瑙嗛_CAPTURE;
	    鑻?(force_鏍煎紡) {
		    fmt.fmt.pix.width       = 640;
		    fmt.fmt.pix.height      = 480;
		    fmt.fmt.pix.pixelformat = V4L2_PIX_FMT_YUYV;
		    fmt.fmt.pix.瀛楁       = V4L2_瀛楁_INTERLACED;

		    鑻?(-1 == xioctl(fd, VIDIOC_S_FMT, &fmt))
			    errno_exit("VIDIOC_S_FMT");

		    /** 娉ㄦ剰 VIDIOC_S_FMT 鍙兘浼氭敼鍙樺搴﹀拰楂樺害銆?**/
	    } else {
		    /** 淇濈暀鐢?v4l2-ctl 绛夎缃殑鍘熷閰嶇疆 **/
		    鑻?(-1 == xioctl(fd, VIDIOC_G_FMT, &fmt))
			    errno_exit("VIDIOC_G_FMT");
	    }

	    /** 閽堝鏈夌己闄烽┍鍔ㄧ殑闃插尽鎬ф鏌ャ€?**/
	    min = fmt.fmt.pix.width * 2;
	    鑻?(fmt.fmt.pix.bytesperline < min)
		    fmt.fmt.pix.bytesperline = min;
	    min = fmt.fmt.pix.bytesperline * fmt.fmt.pix.height;
	    鑻?(fmt.fmt.pix.sizeimage < min)
		    fmt.fmt.pix.sizeimage = min;

	    switch (io) {
	    case IO_鏂规硶_璇诲彇:
		    鍒濆鍖朹璇诲彇(fmt.fmt.pix.sizeimage);
		    break;

	    case IO_鏂规硶_MMAP:
		    鍒濆鍖朹mmap();
		    break;

	    case IO_鏂规硶_USERPTR:
		    鍒濆鍖朹userp(fmt.fmt.pix.sizeimage);
		    break;
	    }
    }

    闈欐€?void 鍏抽棴_璁惧(void)
    {
	    鑻?(-1 == 鍏抽棴(fd))
		    errno_exit("鍏抽棴");

	    fd = -1;
    }

    闈欐€?void 鎵撳紑_璁惧(void)
    {
	    缁撴瀯浣?stat st;

	    鑻?(-1 == stat(dev_name, &st)) {
		    fprintf(stderr, "Cannot identify '%s': %d, %s\n",
			     dev_name, errno, strerror(errno));
		    exit(EXIT_FAILURE);
	    }

	    鑻?(!S_ISCHR(st.st_妯″紡)) {
		    fprintf(stderr, "%s 鏄?鏃?璁惧\n", dev_name);
		    exit(EXIT_FAILURE);
	    }

	    fd = 鎵撳紑(dev_name, O_RDWR /** 蹇呴渶 **/ | O_NONBLOCK, 0);

	    鑻?(-1 == fd) {
		    fprintf(stderr, "Cannot 鎵撳紑 '%s': %d, %s\n",
			     dev_name, errno, strerror(errno));
		    exit(EXIT_FAILURE);
	    }
    }

    闈欐€?void usage(鏂囦欢 *fp, int argc, char **argv)
    {
	    fprintf(fp,
		     "鐢ㄦ硶锛?s [閫夐」]\n\n"
		     "鐗堟湰 1.3\n"
		     "閫夐」锛歕n"
		     "-d | --璁惧 name   瑙嗛璁惧鍚嶇О [%s]\n"
		     "-h | --help          鎵撳嵃姝ゅ府鍔╀俊鎭痋n"
		     "-m | --mmap          浣跨敤鍐呭瓨鏄犲皠缂撳啿鍖?[榛樿]\n"
		     "-r | --璇诲彇          浣跨敤 璇诲彇() 璋冪敤\n"
		     "-u | --userp         浣跨敤搴旂敤鍒嗛厤鐨勭紦鍐插尯\n"
		     "-o | --杈撳嚭        灏嗘祦杈撳嚭鍒?stdout\n"
		     "-f | --鏍煎紡        寮哄埗鏍煎紡涓?640x480 YUYV\n"
		     "-c | --count         鎶撳彇甯ф暟 [%i]\n"
		     "",
		     argv[^0^], dev_name, 甯count);
    }

    闈欐€?const char short_閫夐」[] = "d:hmruofc:";

    闈欐€?const 缁撴瀯浣?閫夐」
    long_閫夐」[] = {
	    { "璁惧", 蹇呴渶_鍙傛暟, NULL, 'd' },
	    { "help",   鏃燺鍙傛暟,       NULL, 'h' },
	    { "mmap",   鏃燺鍙傛暟,       NULL, 'm' },
	    { "璇诲彇",   鏃燺鍙傛暟,       NULL, 'r' },
	    { "userp",  鏃燺鍙傛暟,       NULL, 'u' },
	    { "杈撳嚭", 鏃燺鍙傛暟,       NULL, 'o' },
	    { "鏍煎紡", 鏃燺鍙傛暟,       NULL, 'f' },
	    { "count",  蹇呴渶_鍙傛暟, NULL, 'c' },
	    { 0, 0, 0, 0 }
    };

    int 涓昏(int argc, char **argv)
    {
	    dev_name = "/dev/瑙嗛0";

	    鐢ㄤ簬 (;;) {
		    int idx;
		    int c;

		    c = getopt_long(argc, argv,
				    short_閫夐」, long_閫夐」, &idx);

		    鑻?(-1 == c)
			    break;

		    switch (c) {
		    case 0: /** getopt_long() 鏍囧織 **/
			    break;

		    case 'd':
			    dev_name = optarg;
			    break;

		    case 'h':
			    usage(stdout, argc, argv);
			    exit(EXIT_SUCCESS);

		    case 'm':
			    io = IO_鏂规硶_MMAP;
			    break;

		    case 'r':
			    io = IO_鏂规硶_璇诲彇;
			    break;

		    case 'u':
			    io = IO_鏂规硶_USERPTR;
			    break;

		    case 'o':
			    out_buf++;
			    break;

		    case 'f':
			    force_鏍煎紡++;
			    break;

		    case 'c':
			    errno = 0;
			    甯count = strtol(optarg, NULL, 0);
			    鑻?(errno)
				    errno_exit(optarg);
			    break;

		    榛樿:
			    usage(stderr, argc, argv);
			    exit(EXIT_FAILURE);
		    }
	    }

	    鎵撳紑_璁惧();
	    鍒濆鍖朹璁惧();
	    鍚姩_capturing();
	    mainloop();
	    鍋滄_capturing();
	    uninit_璁惧();
	    鍏抽棴_璁惧();
	    fprintf(stderr, "\n");
	    return 0;
    }

