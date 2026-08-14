## 鐞嗚В fbdev 鐨?cmap


杩欎簺绗旇瑙ｉ噴浜?X 鐨?dix 灞傚浣曚娇鐢?fbdev 鐨?cmap 缁撴瀯銆?
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
杩樺瓨鍦ㄥ儚 gray1..x 杩欐牱鐨勫懡鍚嶇瓑浠峰舰寮忥紝鍓嶆彁鏄綘鏈変竴涓?rgb.txt銆?
鍦?X 鐨勬煇澶勮皟鐢ㄩ摼涓紝杩欎細寮曞彂瀵瑰鐞嗛鑹叉槧灏勭殑 X 浠ｇ爜鐨勮皟鐢ㄣ€備緥濡傦紝Xfbdev 浼氬懡涓互涓嬪唴瀹癸細

```

  FindBestPixel(pentFirst, size, prgb, channel)

  dr = (long) pent->co.local.red - prgb->red;
  dg = (long) pent->co.local.green - prgb->green;
  db = (long) pent->co.local.blue - prgb->blue;
  sq = dr * dr;
  UnsignedToBigNum (sq, &sum);
  BigNumAdd (&sum, &temp, &sum);

```
co.local.red 鏄€氳繃 FBIOGETCMAP 寮曞叆鐨勬潯鐩紝鐩存帴鏉ヨ嚜涓婇潰鍒楀嚭鐨?info->cmap.red銆俻rgb 鏄簲鐢ㄧ▼搴忔兂瑕佸尮閰嶅埌鐨?rgb銆備笂闈㈢殑浠ｇ爜鐪嬭捣鏉ュ儚鏄湪鍋氭渶灏忎簩涔樺尮閰嶅嚱鏁般€傝繖灏辨槸涓轰粈涔?cmap 鏉＄洰涓嶈兘琚缃负涓€涓鑹茶寖鍥寸殑宸﹁竟鐣屻€?