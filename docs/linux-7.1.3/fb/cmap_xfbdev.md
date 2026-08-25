## 理解 fbdev cmap


这些笔记解释X dix 层如何使fbdev cmap 结构
```

    struct fb_var_screeninfo {
	    .bits_per_pixel = 8,
	    .grayscale      = 1,
	    .red =          { 4, 3, 0 },
	    .green =        { 0, 0, 0 },
	    .blue =         { 0, 0, 0 },
    }
    struct fb_fix_screeninfo {
	    .visual =       FB_VISUAL_STATIC_PSEUDOCOLOR,
    }
    for (i = 0; i < 8; i++)
	info->cmap.red[i] = (((2*i)+1)*(0xFFFF))/16;
    memcpy(info->cmap.green, info->cmap.red, sizeof(u16)*8);
    memcpy(info->cmap.blue, info->cmap.red, sizeof(u16)*8);

```
```

    for (i=0; i < 8; i++) {
	char colorspec[64];
	memset(colorspec,0,64);
	sprintf(colorspec, "rgb:%x/%x/%x", i*36,i*36,i*36);
	if (!XParseColor(outputDisplay, testColormap, colorspec, &wantedColor))
		printf("Can't get color %s\n",colorspec);
	XAllocColor(outputDisplay, testColormap, &wantedColor);
	grays[i] = wantedColor;
    }

```
还存在像 gray1..x 这样的命名等价形式，前提是你有一rgb.txt
X 的某处调用链中，这会引发对处理颜色映射的 X 代码的调用。例如，Xfbdev 会命中以下内容：

```

  FindBestPixel(pentFirst, size, prgb, channel)

  dr = (long) pent->co.local.red - prgb->red;
  dg = (long) pent->co.local.green - prgb->green;
  db = (long) pent->co.local.blue - prgb->blue;
  sq = dr * dr;
  UnsignedToBigNum (sq, &sum);
  BigNumAdd (&sum, &temp, &sum);

```
co.local.red 是通过 FBIOGETCMAP 引入的条目，直接来自上面列出info->cmap.red。prgb 是应用程序想要匹配到rgb。上面的代码看起来像是在做最小二乘匹配函数。这就是为什cmap 条目不能被设置为一个颜色范围的左边界