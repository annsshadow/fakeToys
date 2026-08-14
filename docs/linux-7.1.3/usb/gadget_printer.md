## Linux USB 鎵撳嵃鏈?Gadget 椹卞姩


06/04/2007

Copyright (C) 2007 Craig W. Nadler <craig@nadler.us>



## 姒傝堪


濡傛灉浣犳鍦ㄤ娇鐢?Linux 浣滀负宓屽叆寮忔搷浣滅郴缁熺紪鍐欐墦鍗版満鍥轰欢锛屽彲浠ヤ娇鐢ㄦ湰椹卞姩銆傛湰椹卞姩涓庡湪浣犵殑 Linux 涓绘満绯荤粺涓婁娇鐢ㄦ墦鍗版満鏃犲叧銆?

浣犻渶瑕佷竴涓?USB 璁惧鎺у埗鍣紝浠ュ強鑳芥帴鍙楀熀浜?Linux USB Gadget API 鐨?gadget / 鈥滆澶囩被鈥濋┍鍔ㄧ殑 Linux 椹卞姩銆傚湪鍔犺浇 USB 璁惧鎺у埗鍣ㄩ┍鍔ㄤ箣鍚庯紝鍐嶅姞杞芥墦鍗版満 gadget 椹卞姩銆傝繖灏嗗湪浣犵殑 USB 璁惧绔彛鎵€杩炴帴鐨?USB 涓绘満涓婂憟鐜颁竴涓墦鍗版満鎺ュ彛銆?

鏈┍鍔ㄩ潰鍚戣繍琛屽湪鐢ㄦ埛妯″紡鐨勬墦鍗版満鍥轰欢鑰岃璁°€傜敤鎴锋ā寮忕殑鎵撳嵃鏈哄浐浠堕€氳繃璁惧鏂囦欢涓庡唴鏍告ā寮忕殑鎵撳嵃鏈?gadget 椹卞姩璇诲啓鏁版嵁銆傚綋 USB 涓绘満鍙戦€佽澶囪姹備互鑾峰彇鎵撳嵃鏈虹姸鎬佹椂锛屾墦鍗版満浼氳繑鍥炰竴涓墦鍗版満鐘舵€佸瓧鑺傘€傜敤鎴风┖闂村浐浠跺彲浠ヤ娇鐢ㄨ澶囨枃浠?/dev/g_printer 璇诲彇鎴栧啓鍏ヨ鐘舵€佸瓧鑺傘€傞樆濉炰笌闈為樆濉炵殑璇?鍐欒皟鐢ㄥ潎鍙楁敮鎸併€?




## 濡備綍浣跨敤鏈┍鍔?


瑕佸姞杞?USB 璁惧鎺у埗鍣ㄩ┍鍔ㄥ拰鎵撳嵃鏈?gadget 椹卞姩銆?
```

	modprobe net2280
	modprobe g_printer


```
浠ヤ笅鍛戒护琛屽弬鏁板彲鍦ㄥ姞杞芥墦鍗版満 gadget 鏃朵娇鐢?
锛堜緥濡傦細modprobe g_printer idVendor=0x0525 idProduct=0xa4a8 锛夛細

idVendor
	杩欐槸璁惧鎻忚堪绗︿腑浣跨敤鐨勫巶鍟?ID銆傞粯璁ゅ€间负
	Netchip 鍘傚晢 ID 0x0525銆傚湪鍙戝竷浜у搧涔嬪墠锛屼綘蹇呴』灏嗗叾鏀逛负
	浣犺嚜宸辩殑鍘傚晢 ID銆傚鏋滀綘璁″垝鍙戝竷浜у搧浣嗗皻鏈嫢鏈夊巶鍟?ID锛岃璁块棶
	www.usb.org 浜嗚В濡備綍鑾峰彇鐨勬柟娉曘€?

idProduct
	杩欐槸璁惧鎻忚堪绗︿腑浣跨敤鐨勪骇鍝?ID銆傞粯璁ゅ€间负
	0xa4a8锛屽鏋滀綘宸叉湁鍏朵粬 USB 浜у搧锛屽簲灏嗗叾鏀逛负鏈杩欎簺浜у搧
	浣跨敤鐨?ID銆備竴涓ソ涓绘剰鏄粠 0x0001 涔嬬被鐨勭紪鍙峰紑濮嬩负浣犵殑浜у搧缂栧彿銆?

bcdDevice
	杩欐槸浣犱骇鍝佺殑鐗堟湰鍙枫€傛渶濂藉皢浣犵殑鍥轰欢鐗堟湰鏀惧湪姝ゅ銆?

iManufacturer
	涓€涓寘鍚巶鍟嗗悕绉扮殑瀛楃涓层€?

iProduct
	涓€涓寘鍚骇鍝佸悕绉扮殑瀛楃涓层€?

iSerialNum
	涓€涓寘鍚簭鍒楀彿鐨勫瓧绗︿覆銆傚浜庝綘鐨勬瘡涓骇鍝佸崟鍏冿紝閮藉簲淇敼姝ゅ€笺€?

iPNPstring
	鏈墦鍗版満浣跨敤鐨?PNP ID 瀛楃涓层€備綘闇€瑕侀€氳繃鍛戒护琛屾垨
	纭紪鐮佹柟寮忚缃綘鐨勬墦鍗版満浜у搧鎵€浣跨敤鐨?PNP ID 瀛楃涓层€?

qlen
	姣忎釜绔偣浣跨敤鐨?8k 缂撳啿鍖烘暟閲忋€傞粯璁ゅ€间负 10锛屼綘搴旈拡瀵?
	浣犵殑浜у搧杩涜璋冩暣銆備綘鍙兘杩橀渶瑕侀拡瀵逛綘鐨勪骇鍝佽皟鏁存瘡涓紦鍐插尯鐨勫ぇ灏忋€?




## 浣跨敤绀轰緥浠ｇ爜


鏈ず渚嬩唬鐮佷笌 stdout 閫氫俊锛岃€岄潪涓庢墦鍗板紩鎿庨€氫俊銆?

瑕佺紪璇戜笅闈㈢殑娴嬭瘯浠ｇ爜锛?

1) 灏嗗叾淇濆瓨涓哄悕涓?prn_example.c 鐨勬枃浠?
```

	 gcc prn_example.c -o prn_example



```
```

	# prn_example -read_data


```
```

	# cat data_file | prn_example -write_data


```
```

	# prn_example -get_status

	Printer status is:
	     Printer is NOT Selected
	     Paper is Out
	     Printer OK


```
```

	# prn_example -selected


```
```

	# prn_example -not_selected


```
```

	# prn_example -paper_out


```
```

	# prn_example -paper_loaded


```
```

	# prn_example -no_error


```
```

	# prn_example -error




```
## 绀轰緥浠ｇ爜


```


  #include <stdio.h>
  #include <stdlib.h>
  #include <fcntl.h>
  #include <linux/poll.h>
  #include <sys/ioctl.h>
  #include <linux/usb/g_printer.h>

  #define PRINTER_FILE			"/dev/g_printer"
  #define BUF_SIZE			512


  /*
   * 'usage()' - Show program usage.
   */

  static void
  usage(const char *option)		/* I - Option string or NULL */
  {
	if (option) {
		fprintf(stderr,"prn_example: Unknown option \"%s\"!\n",
				option);
	}

	fputs("\n", stderr);
	fputs("Usage: prn_example -[options]\n", stderr);
	fputs("Options:\n", stderr);
	fputs("\n", stderr);
	fputs("-get_status    Get the current printer status.\n", stderr);
	fputs("-selected      Set the selected status to selected.\n", stderr);
	fputs("-not_selected  Set the selected status to NOT selected.\n",
			stderr);
	fputs("-error         Set the error status to error.\n", stderr);
	fputs("-no_error      Set the error status to NO error.\n", stderr);
	fputs("-paper_out     Set the paper status to paper out.\n", stderr);
	fputs("-paper_loaded  Set the paper status to paper loaded.\n",
			stderr);
	fputs("-read_data     Read printer data from driver.\n", stderr);
	fputs("-write_data    Write printer sata to driver.\n", stderr);
	fputs("-NB_read_data  (Non-Blocking) Read printer data from driver.\n",
			stderr);
	fputs("\n\n", stderr);

	exit(1);
  }


  static int
  read_printer_data()
  {
	struct pollfd	fd[1];

	/* Open device file for printer gadget. */
	fd[0].fd = open(PRINTER_FILE, O_RDWR);
	if (fd[0].fd < 0) {
		printf("Error %d opening %s\n", fd[0].fd, PRINTER_FILE);
		close(fd[0].fd);
		return(-1);
	}

	fd[0].events = POLLIN | POLLRDNORM;

	while (1) {
		static char buf[BUF_SIZE];
		int bytes_read;
		int retval;

		/* Wait for up to 1 second for data. */
		retval = poll(fd, 1, 1000);

		if (retval && (fd[0].revents & POLLRDNORM)) {

			/* Read data from printer gadget driver. */
			bytes_read = read(fd[0].fd, buf, BUF_SIZE);

			if (bytes_read < 0) {
				printf("Error %d reading from %s\n",
						fd[0].fd, PRINTER_FILE);
				close(fd[0].fd);
				return(-1);
			} else if (bytes_read > 0) {
				/* Write data to standard OUTPUT (stdout). */
				fwrite(buf, 1, bytes_read, stdout);
				fflush(stdout);
			}

		}

	}

	/* Close the device file. */
	close(fd[0].fd);

	return 0;
  }


  static int
  write_printer_data()
  {
	struct pollfd	fd[1];

	/* Open device file for printer gadget. */
	fd[0].fd = open (PRINTER_FILE, O_RDWR);
	if (fd[0].fd < 0) {
		printf("Error %d opening %s\n", fd[0].fd, PRINTER_FILE);
		close(fd[0].fd);
		return(-1);
	}

	fd[0].events = POLLOUT | POLLWRNORM;

	while (1) {
		int retval;
		static char buf[BUF_SIZE];
		/* Read data from standard INPUT (stdin). */
		int bytes_read = fread(buf, 1, BUF_SIZE, stdin);

		if (!bytes_read) {
			break;
		}

		while (bytes_read) {

			/* Wait for up to 1 second to sent data. */
			retval = poll(fd, 1, 1000);

			/* Write data to printer gadget driver. */
			if (retval && (fd[0].revents & POLLWRNORM)) {
				retval = write(fd[0].fd, buf, bytes_read);
				if (retval < 0) {
					printf("Error %d writing to %s\n",
							fd[0].fd,
							PRINTER_FILE);
					close(fd[0].fd);
					return(-1);
				} else {
					bytes_read -= retval;
				}

			}

		}

	}

	/* Wait until the data has been sent. */
	fsync(fd[0].fd);

	/* Close the device file. */
	close(fd[0].fd);

	return 0;
  }


  static int
  read_NB_printer_data()
  {
	int		fd;
	static char	buf[BUF_SIZE];
	int		bytes_read;

	/* Open device file for printer gadget. */
	fd = open(PRINTER_FILE, O_RDWR|O_NONBLOCK);
	if (fd < 0) {
		printf("Error %d opening %s\n", fd, PRINTER_FILE);
		close(fd);
		return(-1);
	}

	while (1) {
		/* Read data from printer gadget driver. */
		bytes_read = read(fd, buf, BUF_SIZE);
		if (bytes_read <= 0) {
			break;
		}

		/* Write data to standard OUTPUT (stdout). */
		fwrite(buf, 1, bytes_read, stdout);
		fflush(stdout);
	}

	/* Close the device file. */
	close(fd);

	return 0;
  }


  static int
  get_printer_status()
  {
	int	retval;
	int	fd;

	/* Open device file for printer gadget. */
	fd = open(PRINTER_FILE, O_RDWR);
	if (fd < 0) {
		printf("Error %d opening %s\n", fd, PRINTER_FILE);
		close(fd);
		return(-1);
	}

	/* Make the IOCTL call. */
	retval = ioctl(fd, GADGET_GET_PRINTER_STATUS);
	if (retval < 0) {
		fprintf(stderr, "ERROR: Failed to set printer status\n");
		return(-1);
	}

	/* Close the device file. */
	close(fd);

	return(retval);
  }


  static int
  set_printer_status(unsigned char buf, int clear_printer_status_bit)
  {
	int	retval;
	int	fd;

	retval = get_printer_status();
	if (retval < 0) {
		fprintf(stderr, "ERROR: Failed to get printer status\n");
		return(-1);
	}

	/* Open device file for printer gadget. */
	fd = open(PRINTER_FILE, O_RDWR);

	if (fd < 0) {
		printf("Error %d opening %s\n", fd, PRINTER_FILE);
		close(fd);
		return(-1);
	}

	if (clear_printer_status_bit) {
		retval &= ~buf;
	} else {
		retval |= buf;
	}

	/* Make the IOCTL call. */
	if (ioctl(fd, GADGET_SET_PRINTER_STATUS, (unsigned char)retval)) {
		fprintf(stderr, "ERROR: Failed to set printer status\n");
		return(-1);
	}

	/* Close the device file. */
	close(fd);

	return 0;
  }


  static int
  display_printer_status()
  {
	char	printer_status;

	printer_status = get_printer_status();
	if (printer_status < 0) {
		fprintf(stderr, "ERROR: Failed to get printer status\n");
		return(-1);
	}

	printf("Printer status is:\n");
	if (printer_status & PRINTER_SELECTED) {
		printf("     Printer is Selected\n");
	} else {
		printf("     Printer is NOT Selected\n");
	}
	if (printer_status & PRINTER_PAPER_EMPTY) {
		printf("     Paper is Out\n");
	} else {
		printf("     Paper is Loaded\n");
	}
	if (printer_status & PRINTER_NOT_ERROR) {
		printf("     Printer OK\n");
	} else {
		printf("     Printer ERROR\n");
	}

	return(0);
  }


  int
  main(int  argc, char *argv[])
  {
	int	i;		/* Looping var */
	int	retval = 0;

	/* No Args */
	if (argc == 1) {
		usage(0);
		exit(0);
	}

	for (i = 1; i < argc && !retval; i ++) {

		if (argv[i][0] != '-') {
			continue;
		}

		if (!strcmp(argv[i], "-get_status")) {
			if (display_printer_status()) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-paper_loaded")) {
			if (set_printer_status(PRINTER_PAPER_EMPTY, 1)) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-paper_out")) {
			if (set_printer_status(PRINTER_PAPER_EMPTY, 0)) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-selected")) {
			if (set_printer_status(PRINTER_SELECTED, 0)) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-not_selected")) {
			if (set_printer_status(PRINTER_SELECTED, 1)) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-error")) {
			if (set_printer_status(PRINTER_NOT_ERROR, 1)) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-no_error")) {
			if (set_printer_status(PRINTER_NOT_ERROR, 0)) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-read_data")) {
			if (read_printer_data()) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-write_data")) {
			if (write_printer_data()) {
				retval = 1;
			}

		} else if (!strcmp(argv[i], "-NB_read_data")) {
			if (read_NB_printer_data()) {
				retval = 1;
			}

		} else {
			usage(argv[i]);
			retval = 1;
		}
	}

	exit(retval);
  }

```