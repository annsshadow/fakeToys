### Mono(tm) 浜岃繘鍒跺唴鏍告敮鎸侊紙Linux锛?

瑕侀厤缃?Linux 浠ヨ嚜鍔ㄦ墽琛屽熀浜?Mono 鐨?.NET 浜岃繘鍒舵枃浠讹紙浠?.exe 鏂囦欢褰㈠紡锛夛紝鑰屾棤闇€
浣跨敤 mono CLR 鍖呰鍣紝浣犲彲浠ヤ娇鐢?BINFMT_MISC 鍐呮牳鏀寔銆?
瀹屾垚浠ヤ笅姝ラ鍚庯紝杩欏皢鍏佽浣犲儚鎵ц鍏朵粬浠讳綍绋嬪簭涓€鏍锋墽琛屽熀浜?Mono 鐨?.NET 浜岃繘鍒舵枃浠讹細

1) 浣犲繀椤婚鍏堝畨瑁?Mono CLR 鏀寔锛屽彲浠ラ€氳繃涓嬭浇浜岃繘鍒跺寘銆佹簮浠ｇ爜 tarball 鎴栦粠 Git
   瀹夎銆傝嫢骞插彂琛岀増鐨勪簩杩涘埗鍖呭彲鍦ㄤ互涓嬩綅缃壘鍒帮細

	https://www.mono-project.com/download/

   Mono 鐨勭紪璇戣鏄庡彲鍦ㄤ互涓嬩綅缃壘鍒帮細

	https://www.mono-project.com/docs/compiling-mono/linux/

   涓€鏃﹀畨瑁呬簡 Mono CLR 鏀寔锛屽彧闇€纭 `/usr/bin/mono`锛堝彲鑳戒綅浜庡埆澶勶紝渚嬪
   `/usr/local/bin/mono`锛夊彲浠ユ甯稿伐浣溿€?
2) 浣犲繀椤诲皢 BINFMT_MISC 缂栬瘧涓烘ā鍧楁垨缂栧叆鍐呮牳锛坄CONFIG_BINFMT_MISC`锛夊苟姝ｇ‘璁剧疆銆?   濡傛灉浣犻€夋嫨灏嗗叾缂栬瘧涓烘ā鍧楋紝鍒欏繀椤讳娇鐢?modprobe/insmod 鎵嬪姩鎻掑叆锛屽洜涓?kmod 鏃犳硶
   杞绘槗鍦扮敱 binfmt_misc 鏀寔銆傞槄璇绘湰鐩綍涓殑 `binfmt_misc.txt` 鏂囦欢浠ヤ簡瑙ｆ洿澶?   鍏充簬閰嶇疆杩囩▼鐨勪俊鎭€?
3) 灏嗕互涓嬫潯鐩坊鍔犲埌 `/etc/rc.local` 鎴栫被浼肩殑鍦ㄧ郴缁熷惎鍔ㄦ椂杩愯鐨勮剼鏈腑锛?
   .. code-block:: sh

    # Insert BINFMT_MISC module into the kernel
    if [ ! -e /proc/sys/fs/binfmt_misc/register ]; then
        /sbin/modprobe binfmt_misc
	# Some distributions, like Fedora Core, perform
	# the following command automatically when the
	# binfmt_misc module is loaded into the kernel
	# or during normal boot up (systemd-based systems).
	# Thus, it is possible that the following line
	# is not needed at all.
	mount -t binfmt_misc none /proc/sys/fs/binfmt_misc
    fi

    # Register support for .NET CLR binaries
    if [ -e /proc/sys/fs/binfmt_misc/register ]; then
	# Replace /usr/bin/mono with the correct pathname to
	# the Mono CLR runtime (usually /usr/local/bin/mono
	# when compiling from sources or CVS).
        echo ':CLR:M::MZ::/usr/bin/mono:' > /proc/sys/fs/binfmt_misc/register
    else
        echo "No binfmt_misc support"
        exit 1
    fi

4) 纭 `.exe` 浜岃繘鍒舵枃浠舵棤闇€鍖呰鑴氭湰鍗冲彲杩愯锛屽彧闇€鐩存帴鍚姩璇?`.exe` 鏂囦欢銆?```

	/usr/bin/xsd.exe

   .. note::

      If this fails with a permission denied error, check
      that the ``.exe`` file has execute permissions.

```
