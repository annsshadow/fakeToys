## SA1100 涓插彛


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
浣犲繀椤诲湪鎵€浣跨敤鐨勬牴鏂囦欢绯荤粺鐨?/dev 涓垱寤鸿繖浜?inode
```

	mknod ttySA0 c 204 5
	mknod ttySA1 c 204 6
	mknod ttySA2 c 204 7
	mknod cusa0 c 205 5
	mknod cusa1 c 205 6
	mknod cusa2 c 205 7

```
闄や簡涓婇潰鍒涘缓鐩稿簲鐨勮澶囪妭鐐逛箣澶栵紝浣犺繕蹇呴』纭繚浣犵殑鐢ㄦ埛绌洪棿搴旂敤绋嬪簭浣跨敤浜嗘纭殑璁惧鍚嶃€備竴涓吀鍨嬬殑渚嬪瓙鏄?/etc/inittab 鏂囦欢鐨勫唴瀹癸紝鍏朵腑浣犲彲鑳藉湪涓€涓?ttyS0 涓婂惎鍔ㄤ簡涓€涓?getty 杩涚▼銆?
鍦ㄨ繖绉嶆儏鍐典笅锛?
- 灏?ttyS0 鍑虹幇涔嬪鏇挎崲涓?ttySA0锛宼tyS1 鏇挎崲涓?ttySA1锛屼緷姝ょ被鎺ㄣ€?
- 涓嶈蹇樿鍦?/etc/securetty 涓姞鍏?'ttySA0'銆?console' 鎴栫浉搴旂殑 tty 鍚嶇О锛屼互渚?root 涔熻兘鐧诲綍銆?