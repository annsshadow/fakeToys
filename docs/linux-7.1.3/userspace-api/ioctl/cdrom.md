## CDROM ioctl 璋冪敤鎽樿


- Edward A. Falk <efalk@google.com>

November, 2004

鏈枃妗ｈ瘯鍥炬弿杩?CDROM 灞傛敮鎸佺殑 ioctl(2) 璋冪敤銆傝繖浜涜皟鐢ㄥぇ浣撲笂锛堟埅鑷?Linux 2.6锛?
瀹炵幇浜?drivers/cdrom/cdrom.c 鍜?drivers/block/scsi_ioctl.c銆?

ioctl 鍊煎垪鍦?<linux/cdrom.h> 涓€傛埅鑷虫湰鏂囨挵鍐欐椂锛屽畠浠涓嬫墍绀猴細

	========================  ===============================================
	CDROMPAUSE		  鏆傚仠闊抽鎿嶄綔
	CDROMRESUME		  鎭㈠宸叉殏鍋滅殑闊抽鎿嶄綔
	CDROMPLAYMSF		  鎾斁闊抽 MSF (struct cdrom_msf)
	CDROMPLAYTRKIND		  鎾斁闊抽闊宠建/绱㈠紩 (struct cdrom_ti)
	CDROMREADTOCHDR		  璇诲彇 TOC 澶?(struct cdrom_tochdr)
	CDROMREADTOCENTRY	  璇诲彇 TOC 鏉＄洰 (struct cdrom_tocentry)
	CDROMSTOP		  鍋滄 cdrom 椹卞姩鍣?
	CDROMSTART		  鍚姩 cdrom 椹卞姩鍣?
	CDROMEJECT		  寮瑰嚭 cdrom 浠嬭川
	CDROMVOLCTRL		  鎺у埗杈撳嚭闊抽噺锛坰truct cdrom_volctrl锛?
	CDROMSUBCHNL		  璇诲彇瀛愰€氶亾鏁版嵁 (struct cdrom_subchnl)
	CDROMREADMODE2		  璇诲彇 CDROM 妯″紡 2 鏁版嵁锛?336 瀛楄妭锛?
				  (struct cdrom_read)
	CDROMREADMODE1		  璇诲彇 CDROM 妯″紡 1 鏁版嵁锛?048 瀛楄妭锛?
				  (struct cdrom_read)
	CDROMREADAUDIO		  (struct cdrom_read_audio)
	CDROMEJECT_SW		  鍚敤(1)/绂佺敤(0) 鑷姩寮瑰嚭
	CDROMMULTISESSION	  Obtain the start-of-last-session
				  address of multi session disks
				  (struct cdrom_multisession)
	CDROM_GET_MCN		  鑾峰彇鈥滈€氱敤浜у搧浠ｇ爜鈥濓紙Universal Product Code锛?
				  濡傛灉鍙敤 (struct cdrom_mcn)
	CDROM_GET_UPC		  Deprecated, use CDROM_GET_MCN instead.
	CDROMRESET		  纭浣嶉┍鍔ㄥ櫒
	CDROMVOLREAD		  鑾峰彇椹卞姩鍣ㄧ殑闊抽噺璁剧疆
				  (struct cdrom_volctrl)
	CDROMREADRAW		  浠ュ師濮嬫ā寮忚鍙栨暟鎹紙2352 瀛楄妭锛?
				  (struct cdrom_read)
	CDROMREADCOOKED		  浠?cooked锛堢啛锛夋ā寮忚鍙栨暟鎹?
	CDROMSEEK		  瀹氫綅鍒?msf 鍦板潃
	CDROMPLAYBLK		  浠?scsi-cd, (struct cdrom_blk)
	CDROMREADALL		  璇诲彇鍏ㄩ儴 2646 瀛楄妭
	CDROMGETSPINDOWN	  return 4-bit spindown value
	CDROMSETSPINDOWN	  set 4-bit spindown value
	CDROMCLOSETRAY		  CDROMEJECT 鐨勫搴旀搷浣?
	CDROM_SET_OPTIONS	  璁剧疆琛屼负閫夐」
	CDROM_CLEAR_OPTIONS	  娓呴櫎琛屼负閫夐」
	CDROM_SELECT_SPEED	  璁剧疆 CD-ROM 閫熷害
	CDROM_SELECT_DISC	  閫夋嫨鍏夌洏锛堢敤浜庤嚜鍔ㄦ崲鐩樻満锛?
	CDROM_MEDIA_CHANGED	  妫€鏌ヤ粙璐ㄦ槸鍚﹀凡鏇存敼
	CDROM_TIMED_MEDIA_CHANGE  Check if media changed
				  since given time
				  (struct cdrom_timed_media_change_info)
	CDROM_DRIVE_STATUS	  鑾峰彇鎵樼洏浣嶇疆绛?
	CDROM_DISC_STATUS	  鑾峰彇鍏夌洏绫诲瀷绛?
	CDROM_CHANGER_NSLOTS	  鑾峰彇鎻掓Ы鏁伴噺
	CDROM_LOCKDOOR		  閿佸畾鎴栬В閿佹墭鐩橀棬
	CDROM_DEBUG		  鎵撳紑/鍏抽棴璋冭瘯娑堟伅
	CDROM_GET_CAPABILITY	  鑾峰彇鑳藉姏
	CDROMAUDIOBUFSIZ	  璁剧疆闊抽缂撳啿鍖哄ぇ灏?
	DVD_READ_STRUCT		  璇诲彇缁撴瀯
	DVD_WRITE_STRUCT	  鍐欏叆缁撴瀯
	DVD_AUTH		  韬唤楠岃瘉
	CDROM_SEND_PACKET	  鍚戦┍鍔ㄥ櫒鍙戦€佹暟鎹寘
	CDROM_NEXT_WRITABLE	  鑾峰彇涓嬩竴涓彲鍐欏潡
	CDROM_LAST_WRITTEN	  鑾峰彇鍏夌洏涓婃渶鍚庡啓鍏ョ殑鍧?
	========================  ===============================================


浠ヤ笅淇℃伅鏄€氳繃闃呰鍐呮牳婧愪唬鐮佺‘瀹氱殑銆傞殢鐫€鏃堕棿鐨勬帹绉伙紝鍙兘浼氳繘琛屼竴浜涙洿姝ｃ€?

------------------------------------------------------------------------------

General锛堥€氱敤璇存槑锛夛細

	闄ら潪鍙︽湁璇存槑锛屾墍鏈?ioctl 璋冪敤鍦ㄦ垚鍔熸椂杩斿洖 0锛屽嚭閿欐椂杩斿洖 -1 骞跺皢
	errno 璁剧疆涓洪€傚綋鐨勫€笺€傦紙鏌愪簺 ioctl 杩斿洖闈炶礋鐨勬暟鎹€笺€傦級

	闄ら潪鍙︽湁璇存槑锛屾墍鏈?ioctl 璋冪敤鍦ㄥ皾璇曞悜鐢ㄦ埛鍦板潃绌洪棿澶嶅埗鏁版嵁鎴栦粠涓鍒?
	鏁版嵁澶辫触鏃惰繑鍥?-1锛屽苟灏?errno 璁剧疆涓?EFAULT銆?

	鍚勪釜椹卞姩鍙兘杩斿洖姝ゅ鏈垪鍑虹殑閿欒鐮併€?

	闄ら潪鍙︽湁璇存槑锛屾墍鏈夋暟鎹粨鏋勫拰甯搁噺閮藉畾涔夊湪 <linux/cdrom.h> 涓?

------------------------------------------------------------------------------


CDROMPAUSE
	鏆傚仠闊抽鎿嶄綔


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
	鎭㈠宸叉殏鍋滅殑闊抽鎿嶄綔


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
	鎾斁闊抽 MSF

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
	鎾斁闊抽闊宠建/绱㈠紩

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
	璇诲彇 TOC 澶?

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
	璇诲彇 TOC 鏉＄洰

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
	鍋滄 cdrom 椹卞姩鍣?


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
	鍚姩 cdrom 椹卞姩鍣?


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
 - 寮瑰嚭 cdrom 浠嬭川


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
	CDROMEJECT 鐨勫搴旀搷浣?


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
	鎺у埗杈撳嚭闊抽噺锛坰truct cdrom_volctrl锛?


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
	鑾峰彇椹卞姩鍣ㄧ殑闊抽噺璁剧疆

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
	璇诲彇瀛愰€氶亾鏁版嵁

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
	浠ュ師濮嬫ā寮忚鍙栨暟鎹紙2352 瀛楄妭锛?

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
	璇诲彇 CDROM 妯″紡 1 鏁版嵁锛?048 瀛楄妭锛?

	(struct cdrom_read)

	notes:
		涓?CDROMREADRAW 鐩稿悓锛屽彧鏄潡澶у皬涓?
		CD_FRAMESIZE锛?048锛夊瓧鑺?



CDROMREADMODE2
	璇诲彇 CDROM 妯″紡 2 鏁版嵁锛?336 瀛楄妭锛?

	(struct cdrom_read)

	notes:
		涓?CDROMREADRAW 鐩稿悓锛屽彧鏄潡澶у皬涓?
		CD_FRAMESIZE_RAW0锛?336锛夊瓧鑺?



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
	鍚敤(1)/绂佺敤(0) 鑷姩寮瑰嚭


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
	鑾峰彇澶氫細璇濆厜鐩樻渶鍚庝竴涓細璇濈殑璧峰鍦板潃

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
	鑾峰彇鈥滈€氱敤浜у搧浠ｇ爜鈥濓紙Universal Product Code锛?
	濡傛灉鍙敤

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
	CDROM_GET_MCN锛堝凡寮冪敤锛?


	鏈疄鐜帮紝鎴嚦 2.6.8.1



CDROMRESET
	纭浣嶉┍鍔ㄥ櫒


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
	浠?cooked锛堢啛锛夋ā寮忚鍙栨暟鎹?


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
	璇诲彇鍏ㄩ儴 2646 瀛楄妭


	涓?CDROMREADCOOKED 鐩稿悓锛屼絾璇诲彇 2646 瀛楄妭銆?



CDROMSEEK
	瀹氫綅鍒?msf 鍦板潃


```

	  struct cdrom_msf msf;

	  ioctl(fd, CDROMSEEK, &msf);

	inputs:
		MSF address to seek to.


	outputs:
		none




```
CDROMPLAYBLK
	浠?scsi-cd

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
	宸茶繃鏃讹紝鏇句粎鐢ㄤ簬 ide-cd


```

	  char spindown;

	  ioctl(fd, CDROMGETSPINDOWN, &spindown);

	inputs:
		none


	outputs:
		The value of the current 4-bit spindown value.





```
CDROMSETSPINDOWN
	宸茶繃鏃讹紝鏇句粎鐢ㄤ簬 ide-cd


```

	  char spindown

	  ioctl(fd, CDROMSETSPINDOWN, &spindown);

	inputs:
		4-bit value used to control spindown (TODO: more detail here)


	outputs:
		none






```
CDROM_SET_OPTIONS
	璁剧疆琛屼负閫夐」


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
	娓呴櫎琛屼负閫夐」


	涓?CDROM_SET_OPTIONS 鐩稿悓锛屽彧鏄墍閫夐€夐」琚叧闂€?



CDROM_SELECT_SPEED
	璁剧疆 CD-ROM 閫熷害


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
	閫夋嫨鍏夌洏锛堢敤浜庤嚜鍔ㄦ崲鐩樻満锛?


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
	妫€鏌ヤ粙璐ㄦ槸鍚﹀凡鏇存敼


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
	鑾峰彇鎵樼洏浣嶇疆绛?


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
	鑾峰彇鍏夌洏绫诲瀷绛?


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
	鑾峰彇鎻掓Ы鏁伴噺


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
	閿佸畾鎴栬В閿佹墭鐩橀棬


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
	鎵撳紑/鍏抽棴璋冭瘯娑堟伅


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
	鑾峰彇鑳藉姏


```

	  ioctl(fd, CDROM_GET_CAPABILITY, 0);


	inputs:
		none


	outputs:
		The ioctl return value is the current device capability
		flags.  See CDC_CLOSE_TRAY, CDC_OPEN_TRAY, etc.



```
CDROMAUDIOBUFSIZ
	璁剧疆闊抽缂撳啿鍖哄ぇ灏?


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
DVD_READ_STRUCT			璇诲彇缁撴瀯

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
DVD_WRITE_STRUCT		鍐欏叆缁撴瀯

	鏈疄鐜帮紝鎴嚦 2.6.8.1



DVD_AUTH			韬唤楠岃瘉

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
	鍚戦┍鍔ㄥ櫒鍙戦€佹暟鎹寘


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
	鑾峰彇涓嬩竴涓彲鍐欏潡


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
	鑾峰彇鍏夌洏涓婃渶鍚庡啓鍏ョ殑鍧?


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