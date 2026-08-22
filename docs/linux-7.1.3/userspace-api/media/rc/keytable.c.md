## 文件：uapi/v4l/keytable.c

本文档展V4L 遥控器（RC）子系统中的 keytable.c 示例程序源码，演示如何检查与替换红外（IR）按键映射表，主要供编写或使用红外遥控工具的用户空间开发者参考



    /* keytable.c - 本程序用于检替换 IR 的按
       Copyright (C) 2006-2009 Mauro Carvalho Chehab <mchehab@kernel.org>

       本程序是自由软件；你可以在自由软件基金会发布GNU 通用公共许可
       2 版的条款下，重新发布或修改它

       本程序的分发希望是有用的，但没有任何担保；甚至没有针对特定用途的
       适销性或适用性的默示担保。更多细节请参见 GNU 通用公共许可证
     */

    #include <ctype.h>
    #include <errno.h>
    #include <fcntl.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
    #include <linux/input.h>
    #include <sys/ioctl.h>

    #include "parse.h"

    void prtcode (int *codes)
    {
	    struct parse_key *p;

	    for (p=keynames;p->name!=NULL;p++) {
		    if (p->value == (unsigned)codes[^1^]) {
			    printf("scancode 0x%04x = %s (0x%02x)\\n", codes[^0^], p->name, codes[^1^]);
			    return;
		    }
	    }

	    if (isprint (codes[^1^]))
		    printf("scancode %d = '%c' (0x%02x)\\n", codes[^0^], codes[^1^], codes[^1^]);
	    else
		    printf("scancode %d = 0x%02x\\n", codes[^0^], codes[^1^]);
    }

    int parse_code(char *string)
    {
	    struct parse_key *p;

	    for (p=keynames;p->name!=NULL;p++) {
		    if (!strcasecmp(p->name, string)) {
			    return p->value;
		    }
	    }
	    return -1;
    }

    int main (int argc, char *argv[])
    {
	    int fd;
	    unsigned int i, j;
	    int codes[^2^];

	    if (argc<2 || argc>4) {
		    printf ("用法s <设备> 获取表；或\\n"
			    "       %s <设备> <扫描 <键码>\\n"
			    "       %s <设备> <键码文件>n",**argv,**argv,*argv);
		    return -1;
	    }

	    if ((fd = open(argv[^1^], O_RDONLY)) < 0) {
		    perror("无法打开输入设备");
		    return(-1);
	    }

	    if (argc==4) {
		    int value;

		    value=parse_code(argv[^3^]);

		    if (value==-1) {
			    value = strtol(argv[^3^], NULL, 0);
			    if (errno)
				    perror("value");
		    }

		    codes [^0^] = (unsigned) strtol(argv[^2^], NULL, 0);
		    codes [^1^] = (unsigned) value;

		    if(ioctl(fd, EVIOCSKEYCODE, codes))
			    perror ("EVIOCSKEYCODE");

		    if(ioctl(fd, EVIOCGKEYCODE, codes)==0)
			    prtcode(codes);
		    return 0;
	    }

	    if (argc==3) {
		    FILE *fin;
		    int value;
		    char **scancode, **keycode, s[^2048^];

		    fin=fopen(argv[^2^],"r");
		    if (fin==NULL) {
			    perror ("打开键码文件");
			    return -1;
		    }

		    /** 清空旧表 **/
		    for (j = 0; j < 256; j++) {
			    for (i = 0; i < 256; i++) {
				    codes[^0^] = (j << 8) | i;
				    codes[^1^] = KEY_RESERVED;
				    ioctl(fd, EVIOCSKEYCODE, codes);
			    }
		    }

		    while (fgets(s,sizeof(s),fin)) {
			    scancode=strtok(s,"\\n\\t =:");
			    if (!scancode) {
				    perror ("解析输入文件扫描);
				    return -1;
			    }
			    if (!strcasecmp(scancode, "scancode")) {
				    scancode = strtok(NULL,"\\n\\t =:");
				    if (!scancode) {
					    perror ("解析输入文件扫描);
					    return -1;
				    }
			    }

			    keycode=strtok(NULL,"\\n\\t =:(");
			    if (!keycode) {
				    perror ("解析输入文件键码");
				    return -1;
			    }

			    // printf ("parsing %s=%s:", scancode, keycode);
			    value=parse_code(keycode);
			    // printf ("\\tvalue=%d\\n",value);

			    if (value==-1) {
				    value = strtol(keycode, NULL, 0);
				    if (errno)
					    perror("value");
			    }

			    codes [^0^] = (unsigned) strtol(scancode, NULL, 0);
			    codes [^1^] = (unsigned) value;

			    // printf("\\t%04x=%04x\\n",codes[^0^], codes[^1^]);
			    if(ioctl(fd, EVIOCSKEYCODE, codes)) {
				    fprintf(stderr, "正通过 ",codes[^0^], codes[^1^]);
				    perror ("EVIOCSKEYCODE");
			    }

			    if(ioctl(fd, EVIOCGKEYCODE, codes)==0)
				    prtcode(codes);
		    }
		    return 0;
	    }

	    /** 获取扫描码表 **/
	    for (j = 0; j < 256; j++) {
		    for (i = 0; i < 256; i++) {
			    codes[^0^] = (j << 8) | i;
			    if (!ioctl(fd, EVIOCGKEYCODE, codes) && codes[^1^] != KEY_RESERVED)
				    prtcode(codes);
		    }
	    }
	    return 0;
    }
