## CDROM ioctl 调用摘要


- Edward A. Falk <efalk@google.com>

November, 2004

本文档试图描述 CDROM 层支持的 ioctl(2) 调用。这些调用大体上（截至 Linux 2.6）
实现于 drivers/cdrom/cdrom.c 和 drivers/block/scsi_ioctl.c。

ioctl 值列在 <linux/cdrom.h> 中。截至本文撰写时，它们如下所示：

	========================  ===============================================
	CDROMPAUSE		  暂停音频操作
	CDROMRESUME		  恢复已暂停的音频操作
	CDROMPLAYMSF		  播放音频 MSF (struct cdrom_msf)
	CDROMPLAYTRKIND		  播放音频音轨/索引 (struct cdrom_ti)
	CDROMREADTOCHDR		  读取 TOC 头 (struct cdrom_tochdr)
	CDROMREADTOCENTRY	  读取 TOC 条目 (struct cdrom_tocentry)
	CDROMSTOP		  停止 cdrom 驱动器
	CDROMSTART		  启动 cdrom 驱动器
	CDROMEJECT		  弹出 cdrom 介质
	CDROMVOLCTRL		  控制输出音量（struct cdrom_volctrl）
	CDROMSUBCHNL		  读取子通道数据 (struct cdrom_subchnl)
	CDROMREADMODE2		  读取 CDROM 模式 2 数据（2336 字节）
				  (struct cdrom_read)
	CDROMREADMODE1		  读取 CDROM 模式 1 数据（2048 字节）
				  (struct cdrom_read)
	CDROMREADAUDIO		  (struct cdrom_read_audio)
	CDROMEJECT_SW		  启用(1)/禁用(0) 自动弹出
	CDROMMULTISESSION	  Obtain the start-of-last-session
				  address of multi session disks
				  (struct cdrom_multisession)
	CDROM_GET_MCN		  获取“通用产品代码”（Universal Product Code）
				  如果可用 (struct cdrom_mcn)
	CDROM_GET_UPC		  Deprecated, use CDROM_GET_MCN instead.
	CDROMRESET		  硬复位驱动器
	CDROMVOLREAD		  获取驱动器的音量设置
				  (struct cdrom_volctrl)
	CDROMREADRAW		  以原始模式读取数据（2352 字节）
				  (struct cdrom_read)
	CDROMREADCOOKED		  以 cooked（熟）模式读取数据
	CDROMSEEK		  定位到 msf 地址
	CDROMPLAYBLK		  仅 scsi-cd, (struct cdrom_blk)
	CDROMREADALL		  读取全部 2646 字节
	CDROMGETSPINDOWN	  return 4-bit spindown value
	CDROMSETSPINDOWN	  set 4-bit spindown value
	CDROMCLOSETRAY		  CDROMEJECT 的对应操作
	CDROM_SET_OPTIONS	  设置行为选项
	CDROM_CLEAR_OPTIONS	  清除行为选项
	CDROM_SELECT_SPEED	  设置 CD-ROM 速度
	CDROM_SELECT_DISC	  选择光盘（用于自动换盘机）
	CDROM_MEDIA_CHANGED	  检查介质是否已更改
	CDROM_TIMED_MEDIA_CHANGE  Check if media changed
				  since given time
				  (struct cdrom_timed_media_change_info)
	CDROM_DRIVE_STATUS	  获取托盘位置等
	CDROM_DISC_STATUS	  获取光盘类型等
	CDROM_CHANGER_NSLOTS	  获取插槽数量
	CDROM_LOCKDOOR		  锁定或解锁托盘门
	CDROM_DEBUG		  打开/关闭调试消息
	CDROM_GET_CAPABILITY	  获取能力
	CDROMAUDIOBUFSIZ	  设置音频缓冲区大小
	DVD_READ_STRUCT		  读取结构
	DVD_WRITE_STRUCT	  写入结构
	DVD_AUTH		  身份验证
	CDROM_SEND_PACKET	  向驱动器发送数据包
	CDROM_NEXT_WRITABLE	  获取下一个可写块
	CDROM_LAST_WRITTEN	  获取光盘上最后写入的块
	========================  ===============================================


以下信息是通过阅读内核源代码确定的。随着时间的推移，可能会进行一些更正。

------------------------------------------------------------------------------

General（通用说明）：

	除非另有说明，所有 ioctl 调用在成功时返回 0，出错时返回 -1 并将
	errno 设置为适当的值。（某些 ioctl 返回非负的数据值。）

	除非另有说明，所有 ioctl 调用在尝试向用户地址空间复制数据或从中复制
	数据失败时返回 -1，并将 errno 设置为 EFAULT。

	各个驱动可能返回此处未列出的错误码。

	除非另有说明，所有数据结构和常量都定义在 <linux/cdrom.h> 中

------------------------------------------------------------------------------


CDROMPAUSE
	暂停音频操作


```

	  ioctl(fd, CDROMPAUSE, 0);


	inputs:
		none


	outputs:
		none


	error return:
	  - ENOSYS	cd drive not audio-capable.


```
CDROMRESUME
	恢复已暂停的音频操作


```

	  ioctl(fd, CDROMRESUME, 0);


	inputs:
		none


	outputs:
		none


	error return:
	  - ENOSYS	cd drive not audio-capable.


```
CDROMPLAYMSF
	播放音频 MSF

	(struct cdrom_msf)


```

	  struct cdrom_msf msf;

	  ioctl(fd, CDROMPLAYMSF, &msf);

	inputs:
		cdrom_msf structure, describing a segment of music to play


	outputs:
		none


	error return:
	  - ENOSYS	cd drive not audio-capable.

	notes:
		- MSF stands for minutes-seconds-frames
		- LBA stands for logical block address
		- Segment is described as start and end times, where each time
		  is described as minutes:seconds:frames.
		  A frame is 1/75 of a second.


```
CDROMPLAYTRKIND
	播放音频音轨/索引

	(struct cdrom_ti)


```

	  struct cdrom_ti ti;

	  ioctl(fd, CDROMPLAYTRKIND, &ti);

	inputs:
		cdrom_ti structure, describing a segment of music to play


	outputs:
		none


	error return:
	  - ENOSYS	cd drive not audio-capable.

	notes:
		- Segment is described as start and end times, where each time
		  is described as a track and an index.



```
CDROMREADTOCHDR
	读取 TOC 头

	(struct cdrom_tochdr)


```

	  cdrom_tochdr header;

	  ioctl(fd, CDROMREADTOCHDR, &header);

	inputs:
		cdrom_tochdr structure


	outputs:
		cdrom_tochdr structure


	error return:
	  - ENOSYS	cd drive not audio-capable.



```
CDROMREADTOCENTRY
	读取 TOC 条目

	(struct cdrom_tocentry)


```

	  struct cdrom_tocentry entry;

	  ioctl(fd, CDROMREADTOCENTRY, &entry);

	inputs:
		cdrom_tocentry structure


	outputs:
		cdrom_tocentry structure


	error return:
	  - ENOSYS	cd drive not audio-capable.
	  - EINVAL	entry.cdte_format not CDROM_MSF or CDROM_LBA
	  - EINVAL	requested track out of bounds
	  - EIO		I/O error reading TOC

	notes:
		- TOC stands for Table Of Contents
		- MSF stands for minutes-seconds-frames
		- LBA stands for logical block address



```
CDROMSTOP
	停止 cdrom 驱动器


```

	  ioctl(fd, CDROMSTOP, 0);


	inputs:
		none


	outputs:
		none


	error return:
	  - ENOSYS	cd drive not audio-capable.

	notes:
	  - Exact interpretation of this ioctl depends on the device,
	    but most seem to spin the drive down.


```
CDROMSTART
	启动 cdrom 驱动器


```

	  ioctl(fd, CDROMSTART, 0);


	inputs:
		none


	outputs:
		none


	error return:
	  - ENOSYS	cd drive not audio-capable.

	notes:
	  - Exact interpretation of this ioctl depends on the device,
	    but most seem to spin the drive up and/or close the tray.
	    Other devices ignore the ioctl completely.


```
CDROMEJECT
 - 弹出 cdrom 介质


```

	  ioctl(fd, CDROMEJECT, 0);


	inputs:
		none


	outputs:
		none


	error returns:
	  - ENOSYS	cd drive not capable of ejecting
	  - EBUSY	other processes are accessing drive, or door is locked

	notes:
		- See CDROM_LOCKDOOR, below.




```
CDROMCLOSETRAY
	CDROMEJECT 的对应操作


```

	  ioctl(fd, CDROMCLOSETRAY, 0);


	inputs:
		none


	outputs:
		none


	error returns:
	  - ENOSYS	cd drive not capable of closing the tray
	  - EBUSY	other processes are accessing drive, or door is locked

	notes:
		- See CDROM_LOCKDOOR, below.




```
CDROMVOLCTRL
	控制输出音量（struct cdrom_volctrl）


```

	  struct cdrom_volctrl volume;

	  ioctl(fd, CDROMVOLCTRL, &volume);

	inputs:
		cdrom_volctrl structure containing volumes for up to 4
		channels.

	outputs:
		none


	error return:
	  - ENOSYS	cd drive not audio-capable.



```
CDROMVOLREAD
	获取驱动器的音量设置

	(struct cdrom_volctrl)


```

	  struct cdrom_volctrl volume;

	  ioctl(fd, CDROMVOLREAD, &volume);

	inputs:
		none


	outputs:
		The current volume settings.


	error return:
	  - ENOSYS	cd drive not audio-capable.



```
CDROMSUBCHNL
	读取子通道数据

	(struct cdrom_subchnl)


```

	  struct cdrom_subchnl q;

	  ioctl(fd, CDROMSUBCHNL, &q);

	inputs:
		cdrom_subchnl structure


	outputs:
		cdrom_subchnl structure


	error return:
	  - ENOSYS	cd drive not audio-capable.
	  - EINVAL	format not CDROM_MSF or CDROM_LBA

	notes:
		- Format is converted to CDROM_MSF or CDROM_LBA
		  as per user request on return



```
CDROMREADRAW
	以原始模式读取数据（2352 字节）

	(struct cdrom_read)

```

	  union {

	    struct cdrom_msf msf;		/* input */
	    char buffer[CD_FRAMESIZE_RAW];	/* return */
	  } arg;
	  ioctl(fd, CDROMREADRAW, &arg);

	inputs:
		cdrom_msf structure indicating an address to read.

		Only the start values are significant.

	outputs:
		Data written to address provided by user.


	error return:
	  - EINVAL	address less than 0, or msf less than 0:2:0
	  - ENOMEM	out of memory

	notes:
		- As of 2.6.8.1, comments in <linux/cdrom.h> indicate that this
		  ioctl accepts a cdrom_read structure, but actual source code
		  reads a cdrom_msf structure and writes a buffer of data to
		  the same address.

		- MSF values are converted to LBA values via this formula::

		    lba = (((m * CD_SECS) + s) * CD_FRAMES + f) - CD_MSF_OFFSET;




```
CDROMREADMODE1
	读取 CDROM 模式 1 数据（2048 字节）

	(struct cdrom_read)

	notes:
		与 CDROMREADRAW 相同，只是块大小为
		CD_FRAMESIZE（2048）字节



CDROMREADMODE2
	读取 CDROM 模式 2 数据（2336 字节）

	(struct cdrom_read)

	notes:
		与 CDROMREADRAW 相同，只是块大小为
		CD_FRAMESIZE_RAW0（2336）字节



CDROMREADAUDIO
	(struct cdrom_read_audio)

```

	  struct cdrom_read_audio ra;

	  ioctl(fd, CDROMREADAUDIO, &ra);

	inputs:
		cdrom_read_audio structure containing read start
		point and length

	outputs:
		audio data, returned to buffer indicated by ra


	error return:
	  - EINVAL	format not CDROM_MSF or CDROM_LBA
	  - EINVAL	nframes not in range [1 75]
	  - ENXIO	drive has no queue (probably means invalid fd)
	  - ENOMEM	out of memory


```
CDROMEJECT_SW
	启用(1)/禁用(0) 自动弹出


```

	  int val;

	  ioctl(fd, CDROMEJECT_SW, val);

	inputs:
		Flag specifying auto-eject flag.


	outputs:
		none


	error return:
	  - ENOSYS	Drive is not capable of ejecting.
	  - EBUSY	Door is locked




```
CDROMMULTISESSION
	获取多会话光盘最后一个会话的起始地址

	(struct cdrom_multisession)

```

	  struct cdrom_multisession ms_info;

	  ioctl(fd, CDROMMULTISESSION, &ms_info);

	inputs:
		cdrom_multisession structure containing desired

	  format.

	outputs:
		cdrom_multisession structure is filled with last_session
		information.

	error return:
	  - EINVAL	format not CDROM_MSF or CDROM_LBA


```
CDROM_GET_MCN
	获取“通用产品代码”（Universal Product Code）
	如果可用

	(struct cdrom_mcn)


```

	  struct cdrom_mcn mcn;

	  ioctl(fd, CDROM_GET_MCN, &mcn);

	inputs:
		none


	outputs:
		Universal Product Code


	error return:
	  - ENOSYS	Drive is not capable of reading MCN data.

	notes:
		- Source code comments state::

		    The following function is implemented, although very few
		    audio discs give Universal Product Code information, which
		    should just be the Medium Catalog Number on the box.  Note,
		    that the way the code is written on the CD is /not/ uniform
		    across all discs!




```
CDROM_GET_UPC
	CDROM_GET_MCN（已弃用）


	未实现，截至 2.6.8.1



CDROMRESET
	硬复位驱动器


```

	  ioctl(fd, CDROMRESET, 0);


	inputs:
		none


	outputs:
		none


	error return:
	  - EACCES	Access denied:  requires CAP_SYS_ADMIN
	  - ENOSYS	Drive is not capable of resetting.




```
CDROMREADCOOKED
	以 cooked（熟）模式读取数据


```

	  u8 buffer[CD_FRAMESIZE]

	  ioctl(fd, CDROMREADCOOKED, buffer);

	inputs:
		none


	outputs:
		2048 bytes of data, "cooked" mode.


	notes:
		Not implemented on all drives.





```
CDROMREADALL
	读取全部 2646 字节


	与 CDROMREADCOOKED 相同，但读取 2646 字节。



CDROMSEEK
	定位到 msf 地址


```

	  struct cdrom_msf msf;

	  ioctl(fd, CDROMSEEK, &msf);

	inputs:
		MSF address to seek to.


	outputs:
		none




```
CDROMPLAYBLK
	仅 scsi-cd

	(struct cdrom_blk)


```

	  struct cdrom_blk blk;

	  ioctl(fd, CDROMPLAYBLK, &blk);

	inputs:
		Region to play


	outputs:
		none




```
CDROMGETSPINDOWN
	已过时，曾仅用于 ide-cd


```

	  char spindown;

	  ioctl(fd, CDROMGETSPINDOWN, &spindown);

	inputs:
		none


	outputs:
		The value of the current 4-bit spindown value.





```
CDROMSETSPINDOWN
	已过时，曾仅用于 ide-cd


```

	  char spindown

	  ioctl(fd, CDROMSETSPINDOWN, &spindown);

	inputs:
		4-bit value used to control spindown (TODO: more detail here)


	outputs:
		none






```
CDROM_SET_OPTIONS
	设置行为选项


```

	  int options;

	  ioctl(fd, CDROM_SET_OPTIONS, options);

	inputs:
		New values for drive options.  The logical 'or' of:

	    ==============      ==================================
	    CDO_AUTO_CLOSE	close tray on first open(2)
	    CDO_AUTO_EJECT	open tray on last release
	    CDO_USE_FFLAGS	use O_NONBLOCK information on open
	    CDO_LOCK		lock tray on open files
	    CDO_CHECK_TYPE	check type on open for data
	    ==============      ==================================

	outputs:
		Returns the resulting options settings in the
		ioctl return value.  Returns -1 on error.

	error return:
	  - ENOSYS	selected option(s) not supported by drive.




```
CDROM_CLEAR_OPTIONS
	清除行为选项


	与 CDROM_SET_OPTIONS 相同，只是所选选项被关闭。



CDROM_SELECT_SPEED
	设置 CD-ROM 速度


```

	  int speed;

	  ioctl(fd, CDROM_SELECT_SPEED, speed);

	inputs:
		New drive speed.


	outputs:
		none


	error return:
	  - ENOSYS	speed selection not supported by drive.



```
CDROM_SELECT_DISC
	选择光盘（用于自动换盘机）


```

	  int disk;

	  ioctl(fd, CDROM_SELECT_DISC, disk);

	inputs:
		Disk to load into drive.


	outputs:
		none


	error return:
	  - EINVAL	Disk number beyond capacity of drive



```
CDROM_MEDIA_CHANGED
	检查介质是否已更改


```

	  int slot;

	  ioctl(fd, CDROM_MEDIA_CHANGED, slot);

	inputs:
		Slot number to be tested, always zero except for jukeboxes.

		May also be special values CDSL_NONE or CDSL_CURRENT

	outputs:
		Ioctl return value is 0 or 1 depending on whether the media

	  has been changed, or -1 on error.

	error returns:
	  - ENOSYS	Drive can't detect media change
	  - EINVAL	Slot number beyond capacity of drive
	  - ENOMEM	Out of memory



```
CDROM_DRIVE_STATUS
	获取托盘位置等


```

	  int slot;

	  ioctl(fd, CDROM_DRIVE_STATUS, slot);

	inputs:
		Slot number to be tested, always zero except for jukeboxes.

		May also be special values CDSL_NONE or CDSL_CURRENT

	outputs:
		Ioctl return value will be one of the following values

	  from <linux/cdrom.h>:

	    =================== ==========================
	    CDS_NO_INFO		Information not available.
	    CDS_NO_DISC
	    CDS_TRAY_OPEN
	    CDS_DRIVE_NOT_READY
	    CDS_DISC_OK
	    -1			error
	    =================== ==========================

	error returns:
	  - ENOSYS	Drive can't detect drive status
	  - EINVAL	Slot number beyond capacity of drive
	  - ENOMEM	Out of memory




```
CDROM_DISC_STATUS
	获取光盘类型等


```

	  ioctl(fd, CDROM_DISC_STATUS, 0);


	inputs:
		none


	outputs:
		Ioctl return value will be one of the following values

	  from <linux/cdrom.h>:

	    - CDS_NO_INFO
	    - CDS_AUDIO
	    - CDS_MIXED
	    - CDS_XA_2_2
	    - CDS_XA_2_1
	    - CDS_DATA_1

	error returns:
		none at present

	notes:
	    - Source code comments state::


		Ok, this is where problems start.  The current interface for
		the CDROM_DISC_STATUS ioctl is flawed.  It makes the false
		assumption that CDs are all CDS_DATA_1 or all CDS_AUDIO, etc.
		Unfortunately, while this is often the case, it is also
		very common for CDs to have some tracks with data, and some
		tracks with audio.	Just because I feel like it, I declare
		the following to be the best way to cope.  If the CD has
		ANY data tracks on it, it will be returned as a data CD.
		If it has any XA tracks, I will return it as that.	Now I
		could simplify this interface by combining these returns with
		the above, but this more clearly demonstrates the problem
		with the current interface.  Too bad this wasn't designed
		to use bitmasks...	       -Erik

		Well, now we have the option CDS_MIXED: a mixed-type CD.
		User level programmers might feel the ioctl is not very
		useful.
				---david




```
CDROM_CHANGER_NSLOTS
	获取插槽数量


```

	  ioctl(fd, CDROM_CHANGER_NSLOTS, 0);


	inputs:
		none


	outputs:
		The ioctl return value will be the number of slots in a
		CD changer.  Typically 1 for non-multi-disk devices.

	error returns:
		none



```
CDROM_LOCKDOOR
	锁定或解锁托盘门


```

	  int lock;

	  ioctl(fd, CDROM_LOCKDOOR, lock);

	inputs:
		Door lock flag, 1=lock, 0=unlock


	outputs:
		none


	error returns:
	  - EDRIVE_CANT_DO_THIS

				Door lock function not supported.
	  - EBUSY

				Attempt to unlock when multiple users
				have the drive open and not CAP_SYS_ADMIN

	notes:
		As of 2.6.8.1, the lock flag is a global lock, meaning that
		all CD drives will be locked or unlocked together.  This is
		probably a bug.

		The EDRIVE_CANT_DO_THIS value is defined in <linux/cdrom.h>
		and is currently (2.6.8.1) the same as EOPNOTSUPP



```
CDROM_DEBUG
	打开/关闭调试消息


```

	  int debug;

	  ioctl(fd, CDROM_DEBUG, debug);

	inputs:
		Cdrom debug flag, 0=disable, 1=enable


	outputs:
		The ioctl return value will be the new debug flag.


	error return:
	  - EACCES	Access denied:  requires CAP_SYS_ADMIN



```
CDROM_GET_CAPABILITY
	获取能力


```

	  ioctl(fd, CDROM_GET_CAPABILITY, 0);


	inputs:
		none


	outputs:
		The ioctl return value is the current device capability
		flags.  See CDC_CLOSE_TRAY, CDC_OPEN_TRAY, etc.



```
CDROMAUDIOBUFSIZ
	设置音频缓冲区大小


```

	  int arg;

	  ioctl(fd, CDROMAUDIOBUFSIZ, val);

	inputs:
		New audio buffer size


	outputs:
		The ioctl return value is the new audio buffer size, or -1
		on error.

	error return:
	  - ENOSYS	Not supported by this driver.

	notes:
		Not supported by all drivers.




```
DVD_READ_STRUCT			读取结构

```

	  dvd_struct s;

	  ioctl(fd, DVD_READ_STRUCT, &s);

	inputs:
		dvd_struct structure, containing:

	    =================== ==========================================
	    type		specifies the information desired, one of
				DVD_STRUCT_PHYSICAL, DVD_STRUCT_COPYRIGHT,
				DVD_STRUCT_DISCKEY, DVD_STRUCT_BCA,
				DVD_STRUCT_MANUFACT
	    physical.layer_num	desired layer, indexed from 0
	    copyright.layer_num	desired layer, indexed from 0
	    disckey.agid
	    =================== ==========================================

	outputs:
		dvd_struct structure, containing:

	    =================== ================================
	    physical		for type == DVD_STRUCT_PHYSICAL
	    copyright		for type == DVD_STRUCT_COPYRIGHT
	    disckey.value	for type == DVD_STRUCT_DISCKEY
	    bca.{len,value}	for type == DVD_STRUCT_BCA
	    manufact.{len,valu}	for type == DVD_STRUCT_MANUFACT
	    =================== ================================

	error returns:
	  - EINVAL	physical.layer_num exceeds number of layers
	  - EIO		Received invalid response from drive



```
DVD_WRITE_STRUCT		写入结构

	未实现，截至 2.6.8.1



DVD_AUTH			身份验证

```

	  dvd_authinfo ai;

	  ioctl(fd, DVD_AUTH, &ai);

	inputs:
		dvd_authinfo structure.  See <linux/cdrom.h>


	outputs:
		dvd_authinfo structure.


	error return:
	  - ENOTTY	ai.type not recognized.



```
CDROM_SEND_PACKET
	向驱动器发送数据包


```

	  struct cdrom_generic_command cgc;

	  ioctl(fd, CDROM_SEND_PACKET, &cgc);

	inputs:
		cdrom_generic_command structure containing the packet to send.


	outputs:
		none

	  cdrom_generic_command structure containing results.

	error return:
	  - EIO

			command failed.
	  - EPERM

			Operation not permitted, either because a
			write command was attempted on a drive which
			is opened read-only, or because the command
			requires CAP_SYS_RAWIO
	  - EINVAL

			cgc.data_direction not set



```
CDROM_NEXT_WRITABLE
	获取下一个可写块


```

	  long next;

	  ioctl(fd, CDROM_NEXT_WRITABLE, &next);

	inputs:
		none


	outputs:
		The next writable block.


	notes:
		If the device does not support this ioctl directly, the

	  ioctl will return CDROM_LAST_WRITTEN + 7.



```
CDROM_LAST_WRITTEN
	获取光盘上最后写入的块


```

	  long last;

	  ioctl(fd, CDROM_LAST_WRITTEN, &last);

	inputs:
		none


	outputs:
		The last block written on disc


	notes:
		If the device does not support this ioctl directly, the
		result is derived from the disc's table of contents.  If the
		table of contents can't be read, this ioctl returns an
		error.

```