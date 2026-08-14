## SA1100 串口


```

  > Date: Sun, 24 Sep 2000 21:40:27 -0700
  > From: H. Peter Anvin <hpa@transmeta.com>
  > To: Nicolas Pitre <nico@CAM.ORG>
  > Cc: Device List Maintainer <device@lanana.org>
  > Subject: Re: device
  >
  > Okay.  Note that device numbers 204 and 205 are used for "low density
  > serial devices", so you will have a range of minors on those majors (the
  > tty device layer handles this just fine, so you don't have to worry about
  > doing anything special.)
  >
  > So your assignments are:
  >
  > 204 char        Low-density serial ports
  >                   5 = /dev/ttySA0               SA1100 builtin serial port 0
  >                   6 = /dev/ttySA1               SA1100 builtin serial port 1
  >                   7 = /dev/ttySA2               SA1100 builtin serial port 2
  >
  > 205 char        Low-density serial ports (alternate device)
  >                   5 = /dev/cusa0                Callout device for ttySA0
  >                   6 = /dev/cusa1                Callout device for ttySA1
  >                   7 = /dev/cusa2                Callout device for ttySA2
  >

```
你必须在所使用的根文件系统的 /dev 中创建这些 inode
```

	mknod ttySA0 c 204 5
	mknod ttySA1 c 204 6
	mknod ttySA2 c 204 7
	mknod cusa0 c 205 5
	mknod cusa1 c 205 6
	mknod cusa2 c 205 7

```
除了上面创建相应的设备节点之外，你还必须确保你的用户空间应用程序使用了正确的设备名。一个典型的例子是 /etc/inittab 文件的内容，其中你可能在一个 ttyS0 上启动了一个 getty 进程。

在这种情况下：

- 将 ttyS0 出现之处替换为 ttySA0，ttyS1 替换为 ttySA1，依此类推。

- 不要忘记在 /etc/securetty 中加入 'ttySA0'、'console' 或相应的 tty 名称，以便 root 也能登录。
