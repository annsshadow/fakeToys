## Linux USB 打印机 Gadget 驱动


06/04/2007

Copyright (C) 2007 Craig W. Nadler <craig@nadler.us>



## 概述


如果你正在使用 Linux 作为嵌入式操作系统编写打印机固件，可以使用本驱动。本驱动与在你的 Linux 主机系统上使用打印机无关。

你需要一个 USB 设备控制器，以及能接受基于 Linux USB Gadget API 的 gadget / “设备类”驱动的 Linux 驱动。在加载 USB 设备控制器驱动之后，再加载打印机 gadget 驱动。这将在你的 USB 设备端口所连接的 USB 主机上呈现一个打印机接口。

本驱动面向运行在用户模式的打印机固件而设计。用户模式的打印机固件通过设备文件与内核模式的打印机 gadget 驱动读写数据。当 USB 主机发送设备请求以获取打印机状态时，打印机会返回一个打印机状态字节。用户空间固件可以使用设备文件 /dev/g_printer 读取或写入该状态字节。阻塞与非阻塞的读/写调用均受支持。




## 如何使用本驱动


要加载 USB 设备控制器驱动和打印机 gadget 驱动。
```

	modprobe net2280
	modprobe g_printer


```
以下命令行参数可在加载打印机 gadget 时使用
（例如：modprobe g_printer idVendor=0x0525 idProduct=0xa4a8 ）：

idVendor
	这是设备描述符中使用的厂商 ID。默认值为
	Netchip 厂商 ID 0x0525。在发布产品之前，你必须将其改为
	你自己的厂商 ID。如果你计划发布产品但尚未拥有厂商 ID，请访问
	www.usb.org 了解如何获取的方法。

idProduct
	这是设备描述符中使用的产品 ID。默认值为
	0xa4a8，如果你已有其他 USB 产品，应将其改为未被这些产品
	使用的 ID。一个好主意是从 0x0001 之类的编号开始为你的产品编号。

bcdDevice
	这是你产品的版本号。最好将你的固件版本放在此处。

iManufacturer
	一个包含厂商名称的字符串。

iProduct
	一个包含产品名称的字符串。

iSerialNum
	一个包含序列号的字符串。对于你的每个产品单元，都应修改此值。

iPNPstring
	本打印机使用的 PNP ID 字符串。你需要通过命令行或
	硬编码方式设置你的打印机产品所使用的 PNP ID 字符串。

qlen
	每个端点使用的 8k 缓冲区数量。默认值为 10，你应针对
	你的产品进行调整。你可能还需要针对你的产品调整每个缓冲区的大小。




## 使用示例代码


本示例代码与 stdout 通信，而非与打印引擎通信。

要编译下面的测试代码：

1) 将其保存为名为 prn_example.c 的文件
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
## 示例代码


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