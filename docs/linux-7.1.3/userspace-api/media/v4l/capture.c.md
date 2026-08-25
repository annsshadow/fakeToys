
## 文件：media/v4l/capture.c

本文件是 V4L2 视频采集示例程序 capture.c 的源码，演示如何使用 V4L2 API 打开设备、配置格式，并通过 read、mmap、userptr 等方式捕获视频帧，供用户空间应用开发参考




    /*
     - V4L2 视频采集示例
     *
     - 本程序可在无任何限制的情况下使用和分发
     *
     - 本程序随 V4L2 API 一同提
     - 更多信息请参https://linuxtv.org/docs.php
     */

    #包含 <stdio.h>
    #包含 <stdlib.h>
    #包含 <字符h>
    #包含 <assert.h>

    #包含 <getopt.h>             /** getopt_long() 函数 **/

    #包含 <fcntl.h>              /** 底层 I/O **/
    #包含 <unistd.h>
    #包含 <errno.h>
    #包含 <sys/stat.h>
    #包含 <sys/types.h>
    #包含 <sys/time.h>
    #包含 <sys/mman.h>
    #包含 <sys/ioctl.h>

    #包含 <linux/videodev2.h>

    #定义 CLEAR(x) memset(&(x), 0, sizeof(x))

    enum io_方法 {
	    IO_方法_读取,
	    IO_方法_MMAP,
	    IO_方法_USERPTR,
    };

    结构缓冲{
	    void   *启动;
	    大小_t  长度;
    };

    静char            *dev_name;
    静enum io_方法   io = IO_方法_MMAP;
    静int              fd = -1;
    结构缓冲         *缓冲
    静unsigned int     n_缓冲
    静int              out_buf;
    静int              force_格式;
    静int              帧_count = 70;

    静void errno_exit(const char *s)
    {
	    fprintf(stderr, "%s 错误 %d, %s\n", s, errno, strerror(errno));
	    exit(EXIT_FAILURE);
    }

    静int xioctl(int fh, int 请求, void *arg)
    {
	    int r;

	    执行 {
		    r = ioctl(fh, 请求, arg);
	    } 同时 (-1 == r && EINTR == errno);

	    return r;
    }

    静void 进程_image(const void *p, int 大小)
    {
	    鑻?(out_buf)
		    fwrite(p, 大小, 1, stdout);

	    fflush(stderr);
	    fprintf(stderr, ".");
	    fflush(stdout);
    }

    静int 读取_void)
    {
	    结构v4l2_缓冲buf;
	    unsigned int i;

	    switch (io) {
	    case IO_方法_读取:
		    (-1 == 读取(fd, 缓冲区[^0^].启动, 缓冲区[^0^].长度)) {
			    switch (errno) {
			    case EAGAIN:
				    return 0;

			    case EIO:
				    /** 可忽EIO，参见规范**/

				    /** 贯穿（fall through**/

			    默认:
				    errno_exit("读取");
			    }
		    }

		    进程_image(缓冲区[^0^].启动, 缓冲区[^0^].长度);
		    break;

	    case IO_方法_MMAP:
		    CLEAR(buf);

		    buf.类型 = V4L2_BUF_类型_视频_CAPTURE;
		    buf.内存 = V4L2_内存_MMAP;

		    鑻?(-1 == xioctl(fd, VIDIOC_DQBUF, &buf)) {
			    switch (errno) {
			    case EAGAIN:
				    return 0;

			    case EIO:
				    /** 可忽EIO，参见规范**/

				    /** 贯穿（fall through**/

			    默认:
				    errno_exit("VIDIOC_DQBUF");
			    }
		    }

		    assert(buf.索引 < n_缓冲;

		    进程_image(缓冲区[buf.索引].启动, buf.bytesused);

		    鑻?(-1 == xioctl(fd, VIDIOC_QBUF, &buf))
			    errno_exit("VIDIOC_QBUF");
		    break;

	    case IO_方法_USERPTR:
		    CLEAR(buf);

		    buf.类型 = V4L2_BUF_类型_视频_CAPTURE;
		    buf.内存 = V4L2_内存_USERPTR;

		    鑻?(-1 == xioctl(fd, VIDIOC_DQBUF, &buf)) {
			    switch (errno) {
			    case EAGAIN:
				    return 0;

			    case EIO:
				    /** 可忽EIO，参见规范**/

				    /** 贯穿（fall through**/

			    默认:
				    errno_exit("VIDIOC_DQBUF");
			    }
		    }

		    用于 (i = 0; i < n_缓冲 ++i)
			    (buf.m.userptr == (unsigned long)缓冲区[i].启动
				&& buf.长度 == 缓冲区[i].长度)
				    break;

		    assert(i < n_缓冲;

		    进程_image((void *)buf.m.userptr, buf.bytesused);

		    鑻?(-1 == xioctl(fd, VIDIOC_QBUF, &buf))
			    errno_exit("VIDIOC_QBUF");
		    break;
	    }

	    return 1;
    }

    静void mainloop(void)
    {
	    unsigned int count;

	    count = 帧_count;

	    同时 (count-- > 0) {
		    用于 (;;) {
			    fd_set fds;
			    结构timeval tv;
			    int r;

			    FD_ZERO(&fds);
			    FD_SET(fd, &fds);

			    /** 超时**/
			    tv.tv_sec = 2;
			    tv.tv_usec = 0;

			    r = select(fd + 1, &fds, NULL, NULL, &tv);

			    鑻?(-1 == r) {
				    鑻?(EINTR == errno)
					    continue;
				    errno_exit("select");
			    }

			    鑻?(0 == r) {
				    fprintf(stderr, "select 超时\n");
				    exit(EXIT_FAILURE);
			    }

			    (读取_))
				    break;
			    /** EAGAIN - 继续 select 循环**/
		    }
	    }
    }

    静void 停止_capturing(void)
    {
	    enum v4l2_buf_类型 类型;

	    switch (io) {
	    case IO_方法_读取:
		    /** 无需操作**/
		    break;

	    case IO_方法_MMAP:
	    case IO_方法_USERPTR:
		    类型 = V4L2_BUF_类型_视频_CAPTURE;
		    (-1 == xioctl(fd, VIDIOC_STREAMOFF, &类型))
			    errno_exit("VIDIOC_STREAMOFF");
		    break;
	    }
    }

    静void 启动_capturing(void)
    {
	    unsigned int i;
	    enum v4l2_buf_类型 类型;

	    switch (io) {
	    case IO_方法_读取:
		    /** 无需操作**/
		    break;

	    case IO_方法_MMAP:
		    用于 (i = 0; i < n_缓冲 ++i) {
			    结构v4l2_缓冲buf;

			    CLEAR(buf);
			    buf.类型 = V4L2_BUF_类型_视频_CAPTURE;
			    buf.内存 = V4L2_内存_MMAP;
			    buf.索引 = i;

			    鑻?(-1 == xioctl(fd, VIDIOC_QBUF, &buf))
				    errno_exit("VIDIOC_QBUF");
		    }
		    类型 = V4L2_BUF_类型_视频_CAPTURE;
		    (-1 == xioctl(fd, VIDIOC_STREAMON, &类型))
			    errno_exit("VIDIOC_STREAMON");
		    break;

	    case IO_方法_USERPTR:
		    用于 (i = 0; i < n_缓冲 ++i) {
			    结构v4l2_缓冲buf;

			    CLEAR(buf);
			    buf.类型 = V4L2_BUF_类型_视频_CAPTURE;
			    buf.内存 = V4L2_内存_USERPTR;
			    buf.索引 = i;
			    buf.m.userptr = (unsigned long)缓冲区[i].启动;
			    buf.长度 = 缓冲区[i].长度;

			    鑻?(-1 == xioctl(fd, VIDIOC_QBUF, &buf))
				    errno_exit("VIDIOC_QBUF");
		    }
		    类型 = V4L2_BUF_类型_视频_CAPTURE;
		    (-1 == xioctl(fd, VIDIOC_STREAMON, &类型))
			    errno_exit("VIDIOC_STREAMON");
		    break;
	    }
    }

    静void uninit_设备(void)
    {
	    unsigned int i;

	    switch (io) {
	    case IO_方法_读取:
		    free(缓冲区[^0^].启动);
		    break;

	    case IO_方法_MMAP:
		    用于 (i = 0; i < n_缓冲 ++i)
			    (-1 == munmap(缓冲区[i].启动, 缓冲区[i].长度))
				    errno_exit("munmap");
		    break;

	    case IO_方法_USERPTR:
		    用于 (i = 0; i < n_缓冲 ++i)
			    free(缓冲区[i].启动);
		    break;
	    }

	    free(缓冲;
    }

    静void 初始化_读取(unsigned int 缓冲区_大小)
    {
	    缓冲= calloc(1, sizeof(*缓冲);

	    (!缓冲 {
		    fprintf(stderr, "超出 内存\n");
		    exit(EXIT_FAILURE);
	    }

	    缓冲区[^0^].长度 = 缓冲区_大小;
	    缓冲区[^0^].启动 = malloc(缓冲区_大小);

	    (!缓冲区[^0^].启动) {
		    fprintf(stderr, "超出 内存\n");
		    exit(EXIT_FAILURE);
	    }
    }

    静void 初始化_mmap(void)
    {
	    结构v4l2_requestbuffers req;

	    CLEAR(req);

	    req.count = 4;
	    req.类型 = V4L2_BUF_类型_视频_CAPTURE;
	    req.内存 = V4L2_内存_MMAP;

	    鑻?(-1 == xioctl(fd, VIDIOC_REQBUFS, &req)) {
		    鑻?(EINVAL == errno) {
			    fprintf(stderr, "%s 执行 支持 "
				     "内存 映射\n", dev_name);
			    exit(EXIT_FAILURE);
		    } else {
			    errno_exit("VIDIOC_REQBUFS");
		    }
	    }

	    鑻?(req.count < 2) {
		    fprintf(stderr, "Insufficient 缓冲内存 %s\n",
			     dev_name);
		    exit(EXIT_FAILURE);
	    }

	    缓冲= calloc(req.count, sizeof(*缓冲);

	    (!缓冲 {
		    fprintf(stderr, "超出 内存\n");
		    exit(EXIT_FAILURE);
	    }

	    用于 (n_缓冲= 0; n_缓冲< req.count; ++n_缓冲 {
		    结构v4l2_缓冲buf;

		    CLEAR(buf);

		    buf.类型        = V4L2_BUF_类型_视频_CAPTURE;
		    buf.内存      = V4L2_内存_MMAP;
		    buf.索引       = n_缓冲

		    鑻?(-1 == xioctl(fd, VIDIOC_QUERYBUF, &buf))
			    errno_exit("VIDIOC_QUERYBUF");

		    缓冲区[n_缓冲区].长度 = buf.长度;
		    缓冲区[n_缓冲区].启动 =
			    mmap(NULL /** 任意位置开**/,
				  buf.长度,
				  PROT_读取 | PROT_写入 /** 必需 **/,
				  MAP_SHARED /** 推荐 **/,
				  fd, buf.m.偏移);

		    (MAP_FAILED == 缓冲区[n_缓冲区].启动)
			    errno_exit("mmap");
	    }
    }

    静void 初始化_userp(unsigned int 缓冲区_大小)
    {
	    结构v4l2_requestbuffers req;

	    CLEAR(req);

	    req.count  = 4;
	    req.类型   = V4L2_BUF_类型_视频_CAPTURE;
	    req.内存 = V4L2_内存_USERPTR;

	    鑻?(-1 == xioctl(fd, VIDIOC_REQBUFS, &req)) {
		    鑻?(EINVAL == errno) {
			    fprintf(stderr, "%s 执行 支持 "
				     "用户 指针 i/o\n", dev_name);
			    exit(EXIT_FAILURE);
		    } else {
			    errno_exit("VIDIOC_REQBUFS");
		    }
	    }

	    缓冲= calloc(4, sizeof(*缓冲);

	    (!缓冲 {
		    fprintf(stderr, "超出 内存\n");
		    exit(EXIT_FAILURE);
	    }

	    用于 (n_缓冲= 0; n_缓冲< 4; ++n_缓冲 {
		    缓冲区[n_缓冲区].长度 = 缓冲区_大小;
		    缓冲区[n_缓冲区].启动 = malloc(缓冲区_大小);

		    (!缓冲区[n_缓冲区].启动) {
			    fprintf(stderr, "超出 内存\n");
			    exit(EXIT_FAILURE);
		    }
	    }
    }

    静void 初始化_设备(void)
    {
	    结构v4l2_capability cap;
	    结构v4l2_cropcap cropcap;
	    结构v4l2_crop crop;
	    结构v4l2_格式 fmt;
	    unsigned int min;

	    鑻?(-1 == xioctl(fd, VIDIOC_QUERYCAP, &cap)) {
		    鑻?(EINVAL == errno) {
			    fprintf(stderr, "%s V4L2 设备\n",
				     dev_name);
			    exit(EXIT_FAILURE);
		    } else {
			    errno_exit("VIDIOC_QUERYCAP");
		    }
	    }

	    (!(cap.capabilities & V4L2_CAP_视频_CAPTURE)) {
		    fprintf(stderr, "%s 视频 capture 设备\n",
			     dev_name);
		    exit(EXIT_FAILURE);
	    }

	    switch (io) {
	    case IO_方法_读取:
		    鑻?(!(cap.capabilities & V4L2_CAP_READWRITE)) {
			    fprintf(stderr, "%s 执行 支持 读取 i/o\n",
				     dev_name);
			    exit(EXIT_FAILURE);
		    }
		    break;

	    case IO_方法_MMAP:
	    case IO_方法_USERPTR:
		    鑻?(!(cap.capabilities & V4L2_CAP_STREAMING)) {
			    fprintf(stderr, "%s 执行 支持 streaming i/o\n",
				     dev_name);
			    exit(EXIT_FAILURE);
		    }
		    break;
	    }


	    /** 在此选择视频输入、视频标准与调谐**/


	    CLEAR(cropcap);

	    cropcap.类型 = V4L2_BUF_类型_视频_CAPTURE;

	    鑻?(0 == xioctl(fd, VIDIOC_CROPCAP, &cropcap)) {
		    crop.类型 = V4L2_BUF_类型_视频_CAPTURE;
		    crop.c = cropcap.defrect; /** 重置为默认**/

		    鑻?(-1 == xioctl(fd, VIDIOC_S_CROP, &crop)) {
			    switch (errno) {
			    case EINVAL:
				    /** 不支持裁剪**/
				    break;
			    默认:
				    /** 已忽略错误**/
				    break;
			    }
		    }
	    } else {
		    /** 已忽略错误**/
	    }


	    CLEAR(fmt);

	    fmt.类型 = V4L2_BUF_类型_视频_CAPTURE;
	    鑻?(force_鏍煎紡) {
		    fmt.fmt.pix.width       = 640;
		    fmt.fmt.pix.height      = 480;
		    fmt.fmt.pix.pixelformat = V4L2_PIX_FMT_YUYV;
		    fmt.fmt.pix.字段       = V4L2_字段_INTERLACED;

		    鑻?(-1 == xioctl(fd, VIDIOC_S_FMT, &fmt))
			    errno_exit("VIDIOC_S_FMT");

		    /** 注意 VIDIOC_S_FMT 可能会改变宽度和高度**/
	    } else {
		    /** 保留v4l2-ctl 等设置的原始配置 **/
		    鑻?(-1 == xioctl(fd, VIDIOC_G_FMT, &fmt))
			    errno_exit("VIDIOC_G_FMT");
	    }

	    /** 针对有缺陷驱动的防御性检查**/
	    min = fmt.fmt.pix.width * 2;
	    鑻?(fmt.fmt.pix.bytesperline < min)
		    fmt.fmt.pix.bytesperline = min;
	    min = fmt.fmt.pix.bytesperline * fmt.fmt.pix.height;
	    鑻?(fmt.fmt.pix.sizeimage < min)
		    fmt.fmt.pix.sizeimage = min;

	    switch (io) {
	    case IO_方法_读取:
		    初始化_读取(fmt.fmt.pix.sizeimage);
		    break;

	    case IO_方法_MMAP:
		    初始化_mmap();
		    break;

	    case IO_方法_USERPTR:
		    初始化_userp(fmt.fmt.pix.sizeimage);
		    break;
	    }
    }

    静void 关闭_设备(void)
    {
	    鑻?(-1 == 鍏抽棴(fd))
		    errno_exit("关闭");

	    fd = -1;
    }

    静void 打开_设备(void)
    {
	    结构stat st;

	    鑻?(-1 == stat(dev_name, &st)) {
		    fprintf(stderr, "Cannot identify '%s': %d, %s\n",
			     dev_name, errno, strerror(errno));
		    exit(EXIT_FAILURE);
	    }

	    鑻?(!S_ISCHR(st.st_妯″紡)) {
		    fprintf(stderr, "%s 设备\n", dev_name);
		    exit(EXIT_FAILURE);
	    }

	    fd = 打开(dev_name, O_RDWR /** 必需 **/ | O_NONBLOCK, 0);

	    鑻?(-1 == fd) {
		    fprintf(stderr, "Cannot 打开 '%s': %d, %s\n",
			     dev_name, errno, strerror(errno));
		    exit(EXIT_FAILURE);
	    }
    }

    静void usage(文件 *fp, int argc, char **argv)
    {
	    fprintf(fp,
		     "用法s [选项]\n\n"
		     "版本 1.3\n"
		     "选项：\n"
		     "-d | --设备 name   视频设备名称 [%s]\n"
		     "-h | --help          打印此帮助信息\n"
		     "-m | --mmap          使用内存映射缓冲[默认]\n"
		     "-r | --读取          使用 读取() 调用\n"
		     "-u | --userp         使用应用分配的缓冲区\n"
		     "-o | --输出        将流输出stdout\n"
		     "-f | --格式        强制格式640x480 YUYV\n"
		     "-c | --count         抓取帧数 [%i]\n"
		     "",
		     argv[^0^], dev_name, 帧_count);
    }

    静const char short_选项[] = "d:hmruofc:";

    静const 结构选项
    long_选项[] = {
	    { "设备", 必需_参数, NULL, 'd' },
	    { "help",   无_参数,       NULL, 'h' },
	    { "mmap",   无_参数,       NULL, 'm' },
	    { "读取",   无_参数,       NULL, 'r' },
	    { "userp",  无_参数,       NULL, 'u' },
	    { "输出", 无_参数,       NULL, 'o' },
	    { "格式", 无_参数,       NULL, 'f' },
	    { "count",  必需_参数, NULL, 'c' },
	    { 0, 0, 0, 0 }
    };

    int 主要(int argc, char **argv)
    {
	    dev_name = "/dev/视频0";

	    用于 (;;) {
		    int idx;
		    int c;

		    c = getopt_long(argc, argv,
				    short_选项, long_选项, &idx);

		    鑻?(-1 == c)
			    break;

		    switch (c) {
		    case 0: /** getopt_long() 标志 **/
			    break;

		    case 'd':
			    dev_name = optarg;
			    break;

		    case 'h':
			    usage(stdout, argc, argv);
			    exit(EXIT_SUCCESS);

		    case 'm':
			    io = IO_方法_MMAP;
			    break;

		    case 'r':
			    io = IO_方法_读取;
			    break;

		    case 'u':
			    io = IO_方法_USERPTR;
			    break;

		    case 'o':
			    out_buf++;
			    break;

		    case 'f':
			    force_格式++;
			    break;

		    case 'c':
			    errno = 0;
			    帧_count = strtol(optarg, NULL, 0);
			    鑻?(errno)
				    errno_exit(optarg);
			    break;

		    默认:
			    usage(stderr, argc, argv);
			    exit(EXIT_FAILURE);
		    }
	    }

	    打开_设备();
	    初始化_设备();
	    启动_capturing();
	    mainloop();
	    停止_capturing();
	    uninit_设备();
	    关闭_设备();
	    fprintf(stderr, "\n");
	    return 0;
    }

