## cx2341x 驱动


### 非压缩文件格式


cx23416 可以产生（而 cx23415 也可以读取）原始 YUV 输出。YUV 帧的格式为 16x16 线性
平铺的 NV12（V4L2_PIX_FMT_NV12_16L16）。

该格式为 YUV 4:2:0，每个像素使用 1 个 Y 字节，每四个像素使用 1 个 U 与 1 个 V 字节。

数据被编码为两个宏块平面，第一个包含 Y 值，第二个包含 UV 宏块。

Y 平面从左到右、从上到下划分为 16x16 像素的块。每个块依次按行传输。

因此前 16 个字节是左上块的第一行，接下来的 16 个字节是左上块的第二行，依此类推。传输完
该块后，传输其右侧块的第一行，依此类推。

UV 平面从左到右、从上到下划分为 16x8 个 UV 值的块。每个块依次按行传输。

因此前 16 个字节是左上块的第一行，包含 8 对 UV 值（共 16 字节）。接下来的 16 个字节是
左上块的第二行（8 对 UV），依此类推。传输完该块后，传输其右侧块的第一行，依此类推。

下面的代码作为示例，展示了如何将 V4L2_PIX_FMT_NV12_16L16 转换为独立的 Y、U 与 V 平面。
该代码假定帧大小为 720x576（PAL）像素。

帧的宽度始终为 720 像素，与实际指定的宽度无关。

如果高度不是 32 行的整数倍，则捕获的视频会在末尾丢失宏块而无法使用。因此高度必须是
32 的整数倍。

#### 原始格式的 C 示例



	#include <stdio.h>
	#include <stdlib.h>
	#include <string.h>

	static unsigned char frame[576**720**3/2];
	static unsigned char framey[576*720];
	static unsigned char frameu[576*720 / 4];
	static unsigned char framev[576*720 / 4];

	static void de_macro_y(unsigned char** dst, unsigned char **src, int dstride, int w, int h)
	{
	unsigned int y, x, i;

	// descramble Y plane
	// dstride = 720 = w
	// The Y plane is divided into blocks of 16x16 pixels
	// Each block in transmitted in turn, line-by-line.
	for (y = 0; y < h; y += 16) {
		for (x = 0; x < w; x += 16) {
		for (i = 0; i < 16; i++) {
			memcpy(dst + x + (y + i) * dstride, src, 16);
			src += 16;
		}
		}
	}
	}

	static void de_macro_uv(unsigned char **dstu, unsigned char **dstv, unsigned char *src, int dstride, int w, int h)
	{
	unsigned int y, x, i;

	// descramble U/V plane
	// dstride = 720 / 2 = w
	// The U/V values are interlaced (UVUV...).
	// Again, the UV plane is divided into blocks of 16x16 UV values.
	// Each block in transmitted in turn, line-by-line.
	for (y = 0; y < h; y += 16) {
		for (x = 0; x < w; x += 8) {
		for (i = 0; i < 16; i++) {
			int idx = x + (y + i) * dstride;

			dstu[idx+0] = src[^0^];  dstv[idx+0] = src[^1^];
			dstu[idx+1] = src[^2^];  dstv[idx+1] = src[^3^];
			dstu[idx+2] = src[^4^];  dstv[idx+2] = src[^5^];
			dstu[idx+3] = src[^6^];  dstv[idx+3] = src[^7^];
			dstu[idx+4] = src[^8^];  dstv[idx+4] = src[^9^];
			dstu[idx+5] = src[^10^]; dstv[idx+5] = src[^11^];
			dstu[idx+6] = src[^12^]; dstv[idx+6] = src[^13^];
			dstu[idx+7] = src[^14^]; dstv[idx+7] = src[^15^];
			src += 16;
		}
		}
	}
	}

	/*************************************************************************/
	int main(int argc, char **argv)
	{
	FILE *fin;
	int i;

	if (argc == 1) fin = stdin;
	else fin = fopen(argv[^1^], "r");

	if (fin == NULL) {
		fprintf(stderr, "cannot open input\n");
		exit(-1);
	}
	while (fread(frame, sizeof(frame), 1, fin) == 1) {
		de_macro_y(framey, frame, 720, 720, 576);
		de_macro_uv(frameu, framev, frame + 720 * 576, 720 / 2, 720 / 2, 576 / 2);
		fwrite(framey, sizeof(framey), 1, stdout);
		fwrite(framev, sizeof(framev), 1, stdout);
		fwrite(frameu, sizeof(frameu), 1, stdout);
	}
	fclose(fin);
	return 0;
	}


### 内嵌 V4L2_MPEG_STREAM_VBI_FMT_IVTV VBI 数据的格式


作者：Hans Verkuil <hverkuil@kernel.org>


本节描述内嵌于 MPEG-2 节目流中的 VBI 数据的 V4L2_MPEG_STREAM_VBI_FMT_IVTV 格式。该格式
部分由 ivtv 驱动（面向 Conexant cx23415/6 芯片的驱动）的某些硬件限制决定，特别是 VBI
数据的最大尺寸。超出该尺寸的部分会在通过 cx23415 回放 MPEG 流时被截断。

该格式的优点是非常紧凑，并且所有行的 VBI 数据都可以在不超过最大允许尺寸的情况下存储。

VBI 数据的流 ID 为 0xBD。内嵌数据的最大尺寸为 4 + 43 ** 36，即 4 字节的头部，以及
2 ** 18 条 VBI 行，每行有 1 字节头部与 42 字节载荷。超出此限制的部分会被 cx23415/6
固件截断。除了 VBI 行的数据外，我们还需要 36 位用于确定捕获了哪些行的位掩码，以及
4 字节用于一个 magic cookie，表明该数据包包含 V4L2_MPEG_STREAM_VBI_FMT_IVTV VBI 数据。
如果所有行都被使用，则不再有空间存放位掩码。为解决此问题，引入了两个不同的 magic 数：

'itv0'：在此 magic 数之后跟随两个 unsigned long。第一个 unsigned long 的位 0-17 表示
第一个场中捕获了哪些行。第一个 unsigned long 的位 18-31 与第二个 unsigned long 的位 0-3
用于第二个场。

'ITV0'：此 magic 数假定捕获了所有 VBI 行，即它隐式地表示位掩码为 0xffffffff 与 0xf。

在这些 magic cookie（以及在 'itv0' 情况下的 8 字节位掩码）之后，捕获的 VBI 行开始：

对于每一行，第一个字节的最低 4 位包含数据类型。可能的值如下表所示。载荷位于随后的
42 字节中。

以下是可能的数据类型：


	#define IVTV_SLICED_TYPE_TELETEXT       0x1     // Teletext (uses lines 6-22 for PAL)
	#define IVTV_SLICED_TYPE_CC             0x4     // Closed Captions (line 21 NTSC)
	#define IVTV_SLICED_TYPE_WSS            0x5     // Wide Screen Signal (line 23 PAL)
	#define IVTV_SLICED_TYPE_VPS            0x7     // Video Programming System (PAL) (line 16)
