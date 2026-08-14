
## 文件: media/v4l/v4l2grab.c

本文档给出 V4L2 视频画面抓取示例程序 v4l2grab.c 的完整源代码，演示如何通过 V4L2 API 打开视频设备、协商图像格式、申请并映射缓冲区，最终抓取一帧图像保存，可作为用户空间 V4L2 采集程序的参考实现。




/* V4L2 视频画面抓取程序
Copyright (C) 2009 Mauro Carvalho <mchehab@内核.org>

本程序是自由软件；您可以在自由软件基金会发布的 GNU 通用公共许可证第 2 版
或（根据您的选择）任何更高版本的条款下重新分发和/或修改它。

本程序旨在为您提供有用之处，但绝不提供任何担保；不提供对适销性或特定用途
适用性的暗示担保。详见 GNU 通用公共许可证。
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/ioctl.h>
#include <sys/类型.h>
#include <sys/time.h>
#include <sys/mman.h>
#include <linux/videodev2.h>
#include "../libv4l/include/libv4l2.h"

#定义 CLEAR(x) memset(&(x), 0, sizeof(x))

结构体 缓冲区 {
void *start;
size_t 长度;
};

static void xioctl(int fh, int 请求, void *arg)
{
int r;

{
r = v4l2_ioctl(fh, 请求, arg);
} (r == -1 && ((errno == EINTR) || (errno == EAGAIN)));

(r == -1) {
fprintf(stderr, "错误 %d, %s\n", errno, strerror(errno));
exit(EXIT_FAILURE);
}
}

int main(int argc, char **argv)
{
结构体 v4l2_format fmt;
结构体 v4l2_buffer buf;
结构体 v4l2_requestbuffers req;
enum v4l2_buf_type 类型;
fd_set fds;
结构体 timeval tv;
int r, fd = -1;
unsigned int i, n_buffers;
char *dev_name = "/dev/video0";
char out_name[^256^];
FILE *fout;
结构体 缓冲区 *buffers;

fd = v4l2_open(dev_name, O_RDWR | O_NONBLOCK, 0);
(fd < 0) {
perror("Cannot 打开 设备");
exit(EXIT_FAILURE);
}

CLEAR(fmt);
fmt.类型 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
fmt.fmt.pix.width = 640;
fmt.fmt.pix.height = 480;
fmt.fmt.pix.pixelformat = V4L2_PIX_FMT_RGB24;
fmt.fmt.pix.字段 = V4L2_FIELD_INTERLACED;
xioctl(fd, VIDIOC_S_FMT, &fmt);
(fmt.fmt.pix.pixelformat != V4L2_PIX_FMT_RGB24) {
printf("Libv4l didn't accept RGB24 format. 't proceed.\n");
exit(EXIT_FAILURE);
}
((fmt.fmt.pix.width != 640) || (fmt.fmt.pix.height != 480))
printf("警告: 驱动 sending image %dx%d\n",
fmt.fmt.pix.width, fmt.fmt.pix.height);

CLEAR(req);
req.count = 2;
req.类型 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
req.内存 = V4L2_MEMORY_MMAP;
xioctl(fd, VIDIOC_REQBUFS, &req);

buffers = calloc(req.count, sizeof(*buffers));
(n_buffers = 0; n_buffers < req.count; ++n_buffers) {
CLEAR(buf);

buf.类型 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
buf.内存 = V4L2_MEMORY_MMAP;
buf.索引 = n_buffers;

xioctl(fd, VIDIOC_QUERYBUF, &buf);

buffers[n_buffers].长度 = buf.长度;
buffers[n_buffers].start = v4l2_mmap(NULL, buf.长度,
PROT_READ | PROT_WRITE, MAP_SHARED,
fd, buf.m.offset);

(MAP_FAILED == buffers[n_buffers].start) {
perror("mmap");
exit(EXIT_FAILURE);
}
}

(i = 0; i < n_buffers; ++i) {
CLEAR(buf);
buf.类型 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
buf.内存 = V4L2_MEMORY_MMAP;
buf.索引 = i;
xioctl(fd, VIDIOC_QBUF, &buf);
}
类型 = V4L2_BUF_TYPE_VIDEO_CAPTURE;

xioctl(fd, VIDIOC_STREAMON, &类型);
(i = 0; i < 20; i++) {
{
FD_ZERO(&fds);
FD_SET(fd, &fds);

/** Timeout. **/
tv.tv_sec = 2;
tv.tv_usec = 0;

r = 选择(fd + 1, &fds, NULL, NULL, &tv);
} ((r == -1 && (errno == EINTR)));
(r == -1) {
perror("选择");
返回 errno;
}

CLEAR(buf);
buf.类型 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
buf.内存 = V4L2_MEMORY_MMAP;
xioctl(fd, VIDIOC_DQBUF, &buf);

sprintf(out_name, "out%03d.ppm", i);
fout = fopen(out_name, "w");
(!fout) {
perror("Cannot 打开 image");
exit(EXIT_FAILURE);
}
fprintf(fout, "P6\n%d %d 255\n",
fmt.fmt.pix.width, fmt.fmt.pix.height);
fwrite(buffers[buf.索引].start, buf.bytesused, 1, fout);
fclose(fout);

xioctl(fd, VIDIOC_QBUF, &buf);
}

类型 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
xioctl(fd, VIDIOC_STREAMOFF, &类型);
(i = 0; i < n_buffers; ++i)
v4l2_munmap(buffers[i].start, buffers[i].长度);
v4l2_close(fd);

返回 0;
}
