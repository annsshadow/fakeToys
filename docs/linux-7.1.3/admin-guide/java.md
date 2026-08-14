### Java(tm) Binary 鍐呮牳 鏀寔 鐢ㄤ簬 Linux v1.03

鏈枃浠嬬粛濡備綍鍦?Linux 鍐呮牳涓€氳繃 binfmt_misc 鏈哄埗鐩存帴鎵ц Java 浜岃繘鍒剁▼搴忎笌鍙墽琛?JAR 鏂囦欢锛岃鏄庢墍闇€鐨勫唴鏍搁厤缃€乥infmt_misc 娉ㄥ唽椤逛笌 Java 杩愯鏃剁幆澧冨噯澶囷紝闈㈠悜甯屾湜浠ュ唴鏍搁€忔槑鏂瑰紡杩愯 Java 搴旂敤鐨勭敤鎴枫€?


Linux beats them 鍏ㄩ儴! 鍚屾椂 鍏ㄩ儴 鍏朵粬 OS's 鏄?TALKING 鍏充簬 direct
鏀寔 鐨?Java Binaries 鍦?the OS, Linux 鏄?doing 瀹?

鎮ㄥ彲浠?execute Java applications 鍜?Java Applets just 绫讳技 浠讳綍
鍏朵粬 program 涔嬪悗 鎮?鍏锋湁 宸插畬鎴?the 浠ヤ笅:

1) 鎮?蹇呴』 绗竴 install the Java Developers Kit 鐢ㄤ簬 Linux.
   The Java 鍦?Linux HOWTO gives the details 鍦?getting 鍜?
   installing 姝? 姝?HOWTO 鍙?涓?found 鍦?

	ftp://sunsite.unc.edu/pub/Linux/docs/HOWTO/Java-HOWTO

   鎮?搴斿綋 涔?set up 涓€涓?reasonable CLASSPATH environment
   variable 鍒?浣跨敤 Java applications 璇?make 浣跨敤 鐨?浠讳綍
   nonstandard classes (涓?included 鍦?the 鐩稿悓 directory
   浣滀负 the 搴旂敤绋嬪簭 itself).

2) 鎮?鍏锋湁 鍒?compile BINFMT_MISC 浠讳竴涓?浣滀负 涓€涓?妯″潡 鎴?杩涘叆
   the 鍐呮牳 (`CONFIG_BINFMT_MISC`) 鍜?set 瀹?up properly.
   鑻?鎮?choose 鍒?compile 瀹?浣滀负 涓€涓?妯″潡, 鎮?灏?鍏锋湁
   鍒?insert 瀹?manually 涓?modprobe/insmod, 浣滀负 kmod
   cannot easily 涓?鍙楁敮鎸?涓?binfmt_misc.
   璇诲彇 the 鏂囦欢 'binfmt_misc.txt' 鍦?姝?directory 鍒?know
   鏇村 鍏充簬 the 閰嶇疆 杩涚▼.

3) Add the 浠ヤ笅 閰嶇疆 items 鍒?binfmt_misc
   (鎮?搴斿綋 really 鍏锋湁 璇诲彇 `binfmt_misc.txt` 鐜板湪):
```

     ':Java:M::\xca\xfe\xba\xbe::/usr/local/bin/javawrapper:'

   support for executable Jar files::

     ':ExecutableJAR:E::jar::/usr/local/bin/jarwrapper:'

   support for Java Applets::

     ':Applet:E::html::/usr/bin/appletviewer:'

   or the following, if you want to be more selective::

     ':Applet:M::<!--applet::/usr/bin/appletviewer:'

   Of course you have to fix the path names. The path/file names given in this
   document match the Debian 2.1 system. (i.e. jdk installed in ``/usr``,
   custom wrappers from this document in ``/usr/local``)

   Note, that for the more selective applet support you have to modify
   existing html-files to contain ``<!--applet-->`` in the first line
   (``<`` has to be the first character!) to let this work!

   For the compiled Java programs you need a wrapper script like the
   following (this is because Java is broken in case of the filename
   handling), again fix the path names, both in the script and in the
   above given configuration string.

   You, too, need the little program after the script. Compile like::

	gcc -O2 -o javaclassname javaclassname.c

   and stick it to ``/usr/local/bin``.

   Both the javawrapper shellscript and the javaclassname program
   were supplied by Colin J. Watson <cjw44@cam.ac.uk>.

```
Javawrapper shell script:


  #!/bin/bash
  # /usr/鏈湴/bin/javawrapper - the wrapper 鐢ㄤ簬 binfmt_misc/java

  鑻?[ -z "$1" ]; 鐒跺悗
	exec 1>&2
	echo Usage: $0 class-file
	exit 1
  fi

  绫?$1
  FQCLASS=`/usr/local/bin/javaclassname $1`
  FQCLASSN=`echo $FQCLASS | sed -e 's/^.**\.\([^.]**\)$/\1/'`
  FQCLASSP=`echo $FQCLASS | sed -e 's-\.-/-g' -e 's-^[^/]**$--' -e 's-/[^/]**$--'`

  # 渚嬪:
  # 绫?Test.绫?
  # FQCLASS=foo.bar.Test
  # FQCLASSN=Test
  # FQCLASSP=foo/bar

  unset CLASSBASE

  declare -i LINKLEVEL=0

  鍚屾椂 :; 鎵ц
	鑻?[ "`basename $CLASS .class`" == "$FQCLASSN" ]; 鐒跺悗
		# 鍙傝 鑻?姝?directory works straight off
		cd -L `dirname $CLASS`
		CLASSDIR=$PWD
		cd $OLDPWD
		鑻?echo $CLASSDIR | grep -q "$FQCLASSP$"; 鐒跺悗
			CLASSBASE=`echo $CLASSDIR | sed -e "s.$FQCLASSP$.."`
			break;
		fi
		# Try dereferencing the directory name
		cd -P `dirname $CLASS`
		CLASSDIR=$PWD
		cd $OLDPWD
		鑻?echo $CLASSDIR | grep -q "$FQCLASSP$"; 鐒跺悗
			CLASSBASE=`echo $CLASSDIR | sed -e "s.$FQCLASSP$.."`
			break;
		fi
		# 鑻?鏃?鍏朵粬 鍙兘 filename exists
		鑻?[ ! -L $绫?]; 鐒跺悗
			exec 1>&2
			echo $0:
			echo "  $绫?搴斿綋 涓?鍦?涓€涓? \
			     "directory tree called $FQCLASSP"
			exit 1
		fi
	fi
	鑻?[ ! -L $绫?]; 鐒跺悗 break; fi
	# Go down one 鏇村 level 鐨?symbolic links
	let LINKLEVEL+=1
	鑻?[ $LINKLEVEL -gt 5 ]; 鐒跺悗
		exec 1>&2
		echo $0:
		echo "  Too 璁稿 symbolic links encountered"
		exit 1
	fi
	绫?`ls --color=no -l $CLASS | sed -e 's/^.** \([^ ]**\)$/\1/'`
  宸插畬鎴?

  鑻?[ -z "$CLASSBASE" ]; 鐒跺悗
	鑻?[ -z "$FQCLASSP" ]; 鐒跺悗
		GOODNAME=$FQCLASSN.绫?
	else
		GOODNAME=$FQCLASSP/$FQCLASSN.绫?
	fi
	exec 1>&2
	echo $0:
	echo "  $FQCLASS 搴斿綋 涓?鍦?涓€涓?鏂囦欢 called $GOODNAME"
	exit 1
  fi

  鑻?! echo $CLASSPATH | grep -q "^\(.**:\)**$CLASSBASE\(:.**\)**"; 鐒跺悗
	# 绫?鏄?涓?鍦?CLASSPATH, 鍥犳 prepend dir 鐨?绫?鍒?CLASSPATH
	鑻?[ -z "${CLASSPATH}" ] ; 鐒跺悗
		export CLASSPATH=$CLASSBASE
	else
		export CLASSPATH=$CLASSBASE:$CLASSPATH
	fi
  fi

  shift
  /usr/bin/java $FQCLASS "$@"

javaclassname.c:


  /* javaclassname.c
   *
   - Extracts the 绫?name 鏉ヨ嚜 涓€涓?Java 绫?鏂囦欢; intended 鐢ㄤ簬 浣跨敤 鍦?涓€涓?Java
   - wrapper 鐨?the 绫诲瀷 鍙楁敮鎸?鐢?the binfmt_misc 閫夐」 鍦?the Linux 鍐呮牳.
   *
   - Copyright (C) 1999 Colin J. Watson <cjw44@cam.ac.uk>.
   *
   - 姝?program 鏄?free 杞欢; 鎮ㄥ彲浠?redistribute 瀹?鍜?鎴?modify
   - 瀹?鍦ㄢ€︿笅 the terms 鐨?the GNU 閫氱敤 鍏叡 License 浣滀负 published 鐢?
   - the Free 杞欢 Foundation; 浠讳竴涓?鐗堟湰 2 鐨?the License, 鎴?
   - (鍦?鎮ㄧ殑 閫夐」) 浠讳綍 绋嶅悗 鐗堟湰.
   *
   - 姝?program 鏄?distributed 鍦?the hope 璇?瀹?灏?涓?useful,
   - 浣?鏃?浠讳綍 WARRANTY; 鏃?even the implied warranty 鐨?
   - MERCHANTABILITY 鎴?FITNESS 鐢ㄤ簬 涓€涓?鐗瑰畾 PURPOSE.  鍙傝 the
   - GNU 閫氱敤 鍏叡 License 鐢ㄤ簬 鏇村 details.
   *
   - 鎮?搴斿綋 鍏锋湁 received 涓€涓?copy 鐨?the GNU 閫氱敤 鍏叡 License
   - along 涓?姝?program; 鑻?涓? 鍐欏叆 鍒?the Free 杞欢
   - Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
   */

  #鍖呭惈 <stdlib.h>
  #鍖呭惈 <stdio.h>
  #鍖呭惈 <stdarg.h>
  #鍖呭惈 <sys/types.h>

  /** 鏉ヨ嚜 Sun's Java VM Specification, 浣滀负 tag 鏉＄洰 鍦?the constant pool. **/

  #瀹氫箟 CP_UTF8 1
  #瀹氫箟 CP_INTEGER 3
  #瀹氫箟 CP_FLOAT 4
  #瀹氫箟 CP_LONG 5
  #瀹氫箟 CP_DOUBLE 6
  #瀹氫箟 CP_绫?7
  #瀹氫箟 CP_瀛楃涓?8
  #瀹氫箟 CP_FIELDREF 9
  #瀹氫箟 CP_METHODREF 10
  #瀹氫箟 CP_INTERFACEMETHODREF 11
  #瀹氫箟 CP_NAMEANDTYPE 12
  #瀹氫箟 CP_METHODHANDLE 15
  #瀹氫箟 CP_METHODTYPE 16
  #瀹氫箟 CP_INVOKEDYNAMIC 18

  /** 瀹氫箟 涓€浜?commonly 浣跨敤 閿欒 messages **/

  #瀹氫箟 seek_閿欒() 閿欒("%s: Cannot seek\n", program)
  #瀹氫箟 corrupt_閿欒() 閿欒("%s: 绫?鏂囦欢 corrupt\n", program)
  #瀹氫箟 eof_閿欒() 閿欒("%s: Unexpected end 鐨?鏂囦欢\n", program)
  #瀹氫箟 utf8_閿欒() 閿欒("%s: 浠?ASCII 1-255 鍙楁敮鎸乗n", program);

  char *program;

  long *pool;

  u_int8_t 璇诲彇_8(鏂囦欢 *classfile);
  u_int16_t 璇诲彇_16(鏂囦欢 *classfile);
  void skip_constant(鏂囦欢 **classfile, u_int16_t **cur);
  void 閿欒(const char *鏍煎紡, ...);
  int 涓昏(int argc, char **argv);

  /** Reads 鍦?涓€涓?unsigned 8-浣?integer. **/
  u_int8_t 璇诲彇_8(鏂囦欢 *classfile)
  {
	int b = fgetc(classfile);
	鑻?b == EOF)
		eof_閿欒();
	return (u_int8_t)b;
  }

  /** Reads 鍦?涓€涓?unsigned 16-浣?integer. **/
  u_int16_t 璇诲彇_16(鏂囦欢 *classfile)
  {
	int b1, b2;
	b1 = fgetc(classfile);
	鑻?b1 == EOF)
		eof_閿欒();
	b2 = fgetc(classfile);
	鑻?b2 == EOF)
		eof_閿欒();
	return (u_int16_t)((b1 << 8) | b2);
  }

  /** Reads 鍦?涓€涓?鍊?鏉ヨ嚜 the constant pool. **/
  void skip_constant(鏂囦欢 **classfile, u_int16_t **cur)
  {
	u_int16_t len;
	int seekerr = 1;
	pool[*cur] = ftell(classfile);
	switch(璇诲彇_8(classfile))
	{
	case CP_UTF8:
		len = 璇诲彇_16(classfile);
		seekerr = fseek(classfile, len, SEEK_CUR);
		break;
	case CP_绫?
	case CP_瀛楃涓?
	case CP_METHODTYPE:
		seekerr = fseek(classfile, 2, SEEK_CUR);
		break;
	case CP_METHODHANDLE:
		seekerr = fseek(classfile, 3, SEEK_CUR);
		break;
	case CP_INTEGER:
	case CP_FLOAT:
	case CP_FIELDREF:
	case CP_METHODREF:
	case CP_INTERFACEMETHODREF:
	case CP_NAMEANDTYPE:
	case CP_INVOKEDYNAMIC:
		seekerr = fseek(classfile, 4, SEEK_CUR);
		break;
	case CP_LONG:
	case CP_DOUBLE:
		seekerr = fseek(classfile, 8, SEEK_CUR);
		++(*cur);
		break;
	榛樿:
		corrupt_閿欒();
	}
	鑻?seekerr)
		seek_閿欒();
  }

  void 閿欒(const char *鏍煎紡, ...)
  {
	va_鍒楀嚭 ap;
	va_鍚姩(ap, 鏍煎紡);
	vfprintf(stderr, 鏍煎紡, ap);
	va_end(ap);
	exit(1);
  }

  int 涓昏(int argc, char **argv)
  {
	鏂囦欢 *classfile;
	u_int16_t cp_count, i, 姝绫? classinfo_ptr;
	u_int8_t 闀垮害;

	program = argv[^0^];

	鑻?!argv[^1^])
		閿欒("%s: Missing 杈撳叆 鏂囦欢\n", program);
	classfile = fopen(argv[^1^], "rb");
	鑻?!classfile)
		閿欒("%s: 閿欒 opening %s\n", program, argv[^1^]);

	鑻?fseek(classfile, 8, SEEK_SET))  /** skip magic 鍜?鐗堟湰 numbers **/
		seek_閿欒();
	cp_count = 璇诲彇_16(classfile);
	pool = calloc(cp_count, sizeof(long));
	鑻?!pool)
		閿欒("%s: 瓒呭嚭 鍐呭瓨 鐢ㄤ簬 constant pool\n", program);

	鐢ㄤ簬(i = 1; i < cp_count; ++i)
		skip_constant(classfile, &i);
	鑻?fseek(classfile, 2, SEEK_CUR))	/** skip access 鏍囧織 **/
		seek_閿欒();

	姝绫?= 璇诲彇_16(classfile);
	鑻?姝绫?< 1 || 姝绫?>= cp_count)
		corrupt_閿欒();
	鑻?!pool[姝绫籡 || pool[姝绫籡 == -1)
		corrupt_閿欒();
	鑻?fseek(classfile, pool[姝绫籡 + 1, SEEK_SET))
		seek_閿欒();

	classinfo_ptr = 璇诲彇_16(classfile);
	鑻?classinfo_ptr < 1 || classinfo_ptr >= cp_count)
		corrupt_閿欒();
	鑻?!pool[classinfo_ptr] || pool[classinfo_ptr] == -1)
		corrupt_閿欒();
	鑻?fseek(classfile, pool[classinfo_ptr] + 1, SEEK_SET))
		seek_閿欒();

	闀垮害 = 璇诲彇_16(classfile);
	鐢ㄤ簬(i = 0; i < 闀垮害; ++i)
	{
		u_int8_t x = 璇诲彇_8(classfile);
		鑻?(x & 0x80) || !x)
		{
			鑻?(x & 0xE0) == 0xC0)
			{
				u_int8_t y = 璇诲彇_8(classfile);
				鑻?(y & 0xC0) == 0x80)
				{
					int c = ((x & 0x1f) << 6) + (y & 0x3f);
					鑻?c) putchar(c);
					else utf8_閿欒();
				}
				else utf8_閿欒();
			}
			else utf8_閿欒();
		}
		else 鑻?x == '/') putchar('.');
		else putchar(x);
	}
	putchar('\n');
	free(pool);
	fclose(classfile);
	return 0;
  }

```

  #!/bin/bash
  # /usr/local/java/bin/jarwrapper - the wrapper for binfmt_misc/jar

  java -jar $1


```
鐜板湪 simply `chmod +x` the `.class`, `.jar` 鍜?鎴?`.html` 鏂囦欢 鎮?
甯屾湜 鍒?execute.

鍒?add 涓€涓?Java program 鍒?鎮ㄧ殑 path best put 涓€涓?symbolic link 鍒?the 涓昏
.绫?鏂囦欢 杩涘叆 /usr/bin (鎴?another place 鎮?绫讳技) omitting the .绫?
extension. The directory containing the original .绫?鏂囦欢 灏?涓?
added 鍒?鎮ㄧ殑 CLASSPATH 鏈熼棿 execution.


鍒?test 鎮ㄧ殑 鏂?setup, enter 鍦?the 浠ヤ笅 绠€鍗?Java app, 鍜?name
瀹?"HelloWorld.java":


	绫?HelloWorld {
		鍏叡 闈欐€?void 涓昏(瀛楃涓?args[]) {
			绯荤粺.out.println("Hello World!");
		}
	}

```

	javac HelloWorld.java

```
```

	chmod 755 HelloWorld.class

```
```

	./HelloWorld.class


```
鍒?execute Java Jar 鏂囦欢, 绠€鍗?chmod the `*.jar` 鏂囦欢 鍒?鍖呭惈
```

       ./Application.jar


```
鍒?execute Java Applets, 绠€鍗?chmod the `*.html` 鏂囦欢 鍒?鍖呭惈
```

	./Applet.html


```
originally 鐢?Brian 涓€涓? Lantz, brian@lantz.com
heavily edited 鐢ㄤ簬 binfmt_misc 鐢?Richard G眉nther
鏂?scripts 鐢?Colin J. Watson <cjw44@cam.ac.uk>
added executable Jar 鏂囦欢 鏀寔 鐢?Kurt Huwig <kurt@iku-netz.de>
