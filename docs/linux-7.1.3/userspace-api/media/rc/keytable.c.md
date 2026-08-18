## 鏂囦欢锛歶api/v4l/keytable.c

鏈枃妗ｅ睍绀?V4L 閬ユ帶鍣紙RC锛夊瓙绯荤粺涓殑 keytable.c 绀轰緥绋嬪簭婧愮爜锛屾紨绀哄浣曟鏌ヤ笌鏇挎崲绾㈠锛圛R锛夋寜閿槧灏勮〃锛屼富瑕佷緵缂栧啓鎴栦娇鐢ㄧ孩澶栭仴鎺у伐鍏风殑鐢ㄦ埛绌洪棿寮€鍙戣€呭弬鑰冦€?



    /* keytable.c - 鏈▼搴忕敤浜庢鏌?鏇挎崲 IR 鐨勬寜閿?
       Copyright (C) 2006-2009 Mauro Carvalho Chehab <mchehab@kernel.org>

       鏈▼搴忔槸鑷敱杞欢锛涗綘鍙互鍦ㄨ嚜鐢辫蒋浠跺熀閲戜細鍙戝竷鐨?GNU 閫氱敤鍏叡璁稿彲璇?
       绗?2 鐗堢殑鏉℃涓嬶紝閲嶆柊鍙戝竷鍜?鎴栦慨鏀瑰畠銆?

       鏈▼搴忕殑鍒嗗彂甯屾湜鏄湁鐢ㄧ殑锛屼絾娌℃湁浠讳綍鎷呬繚锛涚敋鑷虫病鏈夐拡瀵圭壒瀹氱敤閫旂殑
       閫傞攢鎬ф垨閫傜敤鎬х殑榛樼ず鎷呬繚銆傛洿澶氱粏鑺傝鍙傝 GNU 閫氱敤鍏叡璁稿彲璇併€?
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
		    printf ("鐢ㄦ硶锛?s <璁惧> 鑾峰彇琛紱鎴朶\n"
			    "       %s <璁惧> <鎵弿鐮? <閿爜>\\n"
			    "       %s <璁惧> <閿爜鏂囦欢>n",**argv,**argv,*argv);
		    return -1;
	    }

	    if ((fd = open(argv[^1^], O_RDONLY)) < 0) {
		    perror("鏃犳硶鎵撳紑杈撳叆璁惧");
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
			    perror ("鎵撳紑閿爜鏂囦欢");
			    return -1;
		    }

		    /** 娓呯┖鏃ц〃 **/
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
				    perror ("瑙ｆ瀽杈撳叆鏂囦欢鎵弿鐮?);
				    return -1;
			    }
			    if (!strcasecmp(scancode, "scancode")) {
				    scancode = strtok(NULL,"\\n\\t =:");
				    if (!scancode) {
					    perror ("瑙ｆ瀽杈撳叆鏂囦欢鎵弿鐮?);
					    return -1;
				    }
			    }

			    keycode=strtok(NULL,"\\n\\t =:(");
			    if (!keycode) {
				    perror ("瑙ｆ瀽杈撳叆鏂囦欢閿爜");
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
				    fprintf(stderr, "姝ｉ€氳繃 ",codes[^0^], codes[^1^]);
				    perror ("EVIOCSKEYCODE");
			    }

			    if(ioctl(fd, EVIOCGKEYCODE, codes)==0)
				    prtcode(codes);
		    }
		    return 0;
	    }

	    /** 鑾峰彇鎵弿鐮佽〃 **/
	    for (j = 0; j < 256; j++) {
		    for (i = 0; i < 256; i++) {
			    codes[^0^] = (j << 8) | i;
			    if (!ioctl(fd, EVIOCGKEYCODE, codes) && codes[^1^] != KEY_RESERVED)
				    prtcode(codes);
		    }
	    }
	    return 0;
    }
