
## 鏂囦欢: media/v4l/v4l2grab.c

鏈枃妗ｇ粰鍑?V4L2 瑙嗛鐢婚潰鎶撳彇绀轰緥绋嬪簭 v4l2grab.c 鐨勫畬鏁存簮浠ｇ爜锛屾紨绀哄浣曢€氳繃 V4L2 API 鎵撳紑瑙嗛璁惧銆佸崗鍟嗗浘鍍忔牸寮忋€佺敵璇峰苟鏄犲皠缂撳啿鍖猴紝鏈€缁堟姄鍙栦竴甯у浘鍍忎繚瀛橈紝鍙綔涓虹敤鎴风┖闂?V4L2 閲囬泦绋嬪簭鐨勫弬鑰冨疄鐜般€?




/* V4L2 瑙嗛鐢婚潰鎶撳彇绋嬪簭
Copyright (C) 2009 Mauro Carvalho <mchehab@鍐呮牳.org>

鏈▼搴忔槸鑷敱杞欢锛涙偍鍙互鍦ㄨ嚜鐢辫蒋浠跺熀閲戜細鍙戝竷鐨?GNU 閫氱敤鍏叡璁稿彲璇佺 2 鐗?
鎴栵紙鏍规嵁鎮ㄧ殑閫夋嫨锛変换浣曟洿楂樼増鏈殑鏉℃涓嬮噸鏂板垎鍙戝拰/鎴栦慨鏀瑰畠銆?

鏈▼搴忔棬鍦ㄤ负鎮ㄦ彁渚涙湁鐢ㄤ箣澶勶紝浣嗙粷涓嶆彁渚涗换浣曟媴淇濓紱涓嶆彁渚涘閫傞攢鎬ф垨鐗瑰畾鐢ㄩ€?
閫傜敤鎬х殑鏆楃ず鎷呬繚銆傝瑙?GNU 閫氱敤鍏叡璁稿彲璇併€?
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/ioctl.h>
#include <sys/绫诲瀷.h>
#include <sys/time.h>
#include <sys/mman.h>
#include <linux/videodev2.h>
#include "../libv4l/include/libv4l2.h"

#瀹氫箟 CLEAR(x) memset(&(x), 0, sizeof(x))

缁撴瀯浣?缂撳啿鍖?{
void *start;
size_t 闀垮害;
};

static void xioctl(int fh, int 璇锋眰, void *arg)
{
int r;

{
r = v4l2_ioctl(fh, 璇锋眰, arg);
} (r == -1 && ((errno == EINTR) || (errno == EAGAIN)));

(r == -1) {
fprintf(stderr, "閿欒 %d, %s\n", errno, strerror(errno));
exit(EXIT_FAILURE);
}
}

int main(int argc, char **argv)
{
缁撴瀯浣?v4l2_format fmt;
缁撴瀯浣?v4l2_buffer buf;
缁撴瀯浣?v4l2_requestbuffers req;
enum v4l2_buf_type 绫诲瀷;
fd_set fds;
缁撴瀯浣?timeval tv;
int r, fd = -1;
unsigned int i, n_buffers;
char *dev_name = "/dev/video0";
char out_name[^256^];
FILE *fout;
缁撴瀯浣?缂撳啿鍖?*buffers;

fd = v4l2_open(dev_name, O_RDWR | O_NONBLOCK, 0);
(fd < 0) {
perror("Cannot 鎵撳紑 璁惧");
exit(EXIT_FAILURE);
}

CLEAR(fmt);
fmt.绫诲瀷 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
fmt.fmt.pix.width = 640;
fmt.fmt.pix.height = 480;
fmt.fmt.pix.pixelformat = V4L2_PIX_FMT_RGB24;
fmt.fmt.pix.瀛楁 = V4L2_FIELD_INTERLACED;
xioctl(fd, VIDIOC_S_FMT, &fmt);
(fmt.fmt.pix.pixelformat != V4L2_PIX_FMT_RGB24) {
printf("Libv4l didn't accept RGB24 format. 't proceed.\n");
exit(EXIT_FAILURE);
}
((fmt.fmt.pix.width != 640) || (fmt.fmt.pix.height != 480))
printf("璀﹀憡: 椹卞姩 sending image %dx%d\n",
fmt.fmt.pix.width, fmt.fmt.pix.height);

CLEAR(req);
req.count = 2;
req.绫诲瀷 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
req.鍐呭瓨 = V4L2_MEMORY_MMAP;
xioctl(fd, VIDIOC_REQBUFS, &req);

buffers = calloc(req.count, sizeof(*buffers));
(n_buffers = 0; n_buffers < req.count; ++n_buffers) {
CLEAR(buf);

buf.绫诲瀷 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
buf.鍐呭瓨 = V4L2_MEMORY_MMAP;
buf.绱㈠紩 = n_buffers;

xioctl(fd, VIDIOC_QUERYBUF, &buf);

buffers[n_buffers].闀垮害 = buf.闀垮害;
buffers[n_buffers].start = v4l2_mmap(NULL, buf.闀垮害,
PROT_READ | PROT_WRITE, MAP_SHARED,
fd, buf.m.offset);

(MAP_FAILED == buffers[n_buffers].start) {
perror("mmap");
exit(EXIT_FAILURE);
}
}

(i = 0; i < n_buffers; ++i) {
CLEAR(buf);
buf.绫诲瀷 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
buf.鍐呭瓨 = V4L2_MEMORY_MMAP;
buf.绱㈠紩 = i;
xioctl(fd, VIDIOC_QBUF, &buf);
}
绫诲瀷 = V4L2_BUF_TYPE_VIDEO_CAPTURE;

xioctl(fd, VIDIOC_STREAMON, &绫诲瀷);
(i = 0; i < 20; i++) {
{
FD_ZERO(&fds);
FD_SET(fd, &fds);

/** Timeout. **/
tv.tv_sec = 2;
tv.tv_usec = 0;

r = 閫夋嫨(fd + 1, &fds, NULL, NULL, &tv);
} ((r == -1 && (errno == EINTR)));
(r == -1) {
perror("閫夋嫨");
杩斿洖 errno;
}

CLEAR(buf);
buf.绫诲瀷 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
buf.鍐呭瓨 = V4L2_MEMORY_MMAP;
xioctl(fd, VIDIOC_DQBUF, &buf);

sprintf(out_name, "out%03d.ppm", i);
fout = fopen(out_name, "w");
(!fout) {
perror("Cannot 鎵撳紑 image");
exit(EXIT_FAILURE);
}
fprintf(fout, "P6\n%d %d 255\n",
fmt.fmt.pix.width, fmt.fmt.pix.height);
fwrite(buffers[buf.绱㈠紩].start, buf.bytesused, 1, fout);
fclose(fout);

xioctl(fd, VIDIOC_QBUF, &buf);
}

绫诲瀷 = V4L2_BUF_TYPE_VIDEO_CAPTURE;
xioctl(fd, VIDIOC_STREAMOFF, &绫诲瀷);
(i = 0; i < n_buffers; ++i)
v4l2_munmap(buffers[i].start, buffers[i].闀垮害);
v4l2_close(fd);

杩斿洖 0;
}
