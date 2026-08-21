## Video4Linux 像素格式 4CC 指南


本文档规定了使用 v4l2_fourcc() 定义Video4Linux 4CC 代码
首字符定义像素格式的性质、压缩方式与色彩空间。其余三个字符的解释取决于第一个字符

已有4CC 可能不遵循这些指南

### 原始 bayer


原始 bayer 格式使用以下首字符：

- B：原bayer，未压缩
- b：原bayer，DPCM 压缩
- a：A-law 压缩
- u：u-law 压缩

2 个字符：像素顺序

- B：BGGR
- G：GBRG
- g：GRBG
- R：RGGB

3 个字符：未压缩每像素位数 0--9, A--

4 个字符：压缩每像素位0--9, A--
