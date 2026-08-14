### Java(tm) Binary 内核 支持 用于 Linux v1.03

本文介绍如何在 Linux 内核中通过 binfmt_misc 机制直接执行 Java 二进制程序与可执行 JAR 文件，说明所需的内核配置、binfmt_misc 注册项与 Java 运行时环境准备，面向希望以内核透明方式运行 Java 应用的用户。


Linux beats them 全部! 同时 全部 其他 OS's 是 TALKING 关于 direct
支持 的 Java Binaries 在 the OS, Linux 是 doing 它!

您可以 execute Java applications 和 Java Applets just 类似 任何
其他 program 之后 您 具有 已完成 the 以下:

1) 您 必须 第一 install the Java Developers Kit 用于 Linux.
   The Java 在 Linux HOWTO gives the details 在 getting 和
   installing 此. 此 HOWTO 可 为 found 在:

	ftp://sunsite.unc.edu/pub/Linux/docs/HOWTO/Java-HOWTO

   您 应当 也 set up 一个 reasonable CLASSPATH environment
   variable 到 使用 Java applications 该 make 使用 的 任何
   nonstandard classes (不 included 在 the 相同 directory
   作为 the 应用程序 itself).

2) 您 具有 到 compile BINFMT_MISC 任一个 作为 一个 模块 或 进入
   the 内核 (`CONFIG_BINFMT_MISC`) 和 set 它 up properly.
   若 您 choose 到 compile 它 作为 一个 模块, 您 将 具有
   到 insert 它 manually 与 modprobe/insmod, 作为 kmod
   cannot easily 为 受支持 与 binfmt_misc.
   读取 the 文件 'binfmt_misc.txt' 在 此 directory 到 know
   更多 关于 the 配置 进程.

3) Add the 以下 配置 items 到 binfmt_misc
   (您 应当 really 具有 读取 `binfmt_misc.txt` 现在):
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
  # /usr/本地/bin/javawrapper - the wrapper 用于 binfmt_misc/java

  若 [ -z "$1" ]; 然后
	exec 1>&2
	echo Usage: $0 class-file
	exit 1
  fi

  类=$1
  FQCLASS=`/usr/local/bin/javaclassname $1`
  FQCLASSN=`echo $FQCLASS | sed -e 's/^.**\.\([^.]**\)$/\1/'`
  FQCLASSP=`echo $FQCLASS | sed -e 's-\.-/-g' -e 's-^[^/]**$--' -e 's-/[^/]**$--'`

  # 例如:
  # 类=Test.类
  # FQCLASS=foo.bar.Test
  # FQCLASSN=Test
  # FQCLASSP=foo/bar

  unset CLASSBASE

  declare -i LINKLEVEL=0

  同时 :; 执行
	若 [ "`basename $CLASS .class`" == "$FQCLASSN" ]; 然后
		# 参见 若 此 directory works straight off
		cd -L `dirname $CLASS`
		CLASSDIR=$PWD
		cd $OLDPWD
		若 echo $CLASSDIR | grep -q "$FQCLASSP$"; 然后
			CLASSBASE=`echo $CLASSDIR | sed -e "s.$FQCLASSP$.."`
			break;
		fi
		# Try dereferencing the directory name
		cd -P `dirname $CLASS`
		CLASSDIR=$PWD
		cd $OLDPWD
		若 echo $CLASSDIR | grep -q "$FQCLASSP$"; 然后
			CLASSBASE=`echo $CLASSDIR | sed -e "s.$FQCLASSP$.."`
			break;
		fi
		# 若 无 其他 可能 filename exists
		若 [ ! -L $类 ]; 然后
			exec 1>&2
			echo $0:
			echo "  $类 应当 为 在 一个" \
			     "directory tree called $FQCLASSP"
			exit 1
		fi
	fi
	若 [ ! -L $类 ]; 然后 break; fi
	# Go down one 更多 level 的 symbolic links
	let LINKLEVEL+=1
	若 [ $LINKLEVEL -gt 5 ]; 然后
		exec 1>&2
		echo $0:
		echo "  Too 许多 symbolic links encountered"
		exit 1
	fi
	类=`ls --color=no -l $CLASS | sed -e 's/^.** \([^ ]**\)$/\1/'`
  已完成

  若 [ -z "$CLASSBASE" ]; 然后
	若 [ -z "$FQCLASSP" ]; 然后
		GOODNAME=$FQCLASSN.类
	else
		GOODNAME=$FQCLASSP/$FQCLASSN.类
	fi
	exec 1>&2
	echo $0:
	echo "  $FQCLASS 应当 为 在 一个 文件 called $GOODNAME"
	exit 1
  fi

  若 ! echo $CLASSPATH | grep -q "^\(.**:\)**$CLASSBASE\(:.**\)**"; 然后
	# 类 是 不 在 CLASSPATH, 因此 prepend dir 的 类 到 CLASSPATH
	若 [ -z "${CLASSPATH}" ] ; 然后
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
   - Extracts the 类 name 来自 一个 Java 类 文件; intended 用于 使用 在 一个 Java
   - wrapper 的 the 类型 受支持 由 the binfmt_misc 选项 在 the Linux 内核.
   *
   - Copyright (C) 1999 Colin J. Watson <cjw44@cam.ac.uk>.
   *
   - 此 program 是 free 软件; 您可以 redistribute 它 和/或 modify
   - 它 在…下 the terms 的 the GNU 通用 公共 License 作为 published 由
   - the Free 软件 Foundation; 任一个 版本 2 的 the License, 或
   - (在 您的 选项) 任何 稍后 版本.
   *
   - 此 program 是 distributed 在 the hope 该 它 将 为 useful,
   - 但 无 任何 WARRANTY; 无 even the implied warranty 的
   - MERCHANTABILITY 或 FITNESS 用于 一个 特定 PURPOSE.  参见 the
   - GNU 通用 公共 License 用于 更多 details.
   *
   - 您 应当 具有 received 一个 copy 的 the GNU 通用 公共 License
   - along 与 此 program; 若 不, 写入 到 the Free 软件
   - Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
   */

  #包含 <stdlib.h>
  #包含 <stdio.h>
  #包含 <stdarg.h>
  #包含 <sys/types.h>

  /** 来自 Sun's Java VM Specification, 作为 tag 条目 在 the constant pool. **/

  #定义 CP_UTF8 1
  #定义 CP_INTEGER 3
  #定义 CP_FLOAT 4
  #定义 CP_LONG 5
  #定义 CP_DOUBLE 6
  #定义 CP_类 7
  #定义 CP_字符串 8
  #定义 CP_FIELDREF 9
  #定义 CP_METHODREF 10
  #定义 CP_INTERFACEMETHODREF 11
  #定义 CP_NAMEANDTYPE 12
  #定义 CP_METHODHANDLE 15
  #定义 CP_METHODTYPE 16
  #定义 CP_INVOKEDYNAMIC 18

  /** 定义 一些 commonly 使用 错误 messages **/

  #定义 seek_错误() 错误("%s: Cannot seek\n", program)
  #定义 corrupt_错误() 错误("%s: 类 文件 corrupt\n", program)
  #定义 eof_错误() 错误("%s: Unexpected end 的 文件\n", program)
  #定义 utf8_错误() 错误("%s: 仅 ASCII 1-255 受支持\n", program);

  char *program;

  long *pool;

  u_int8_t 读取_8(文件 *classfile);
  u_int16_t 读取_16(文件 *classfile);
  void skip_constant(文件 **classfile, u_int16_t **cur);
  void 错误(const char *格式, ...);
  int 主要(int argc, char **argv);

  /** Reads 在 一个 unsigned 8-位 integer. **/
  u_int8_t 读取_8(文件 *classfile)
  {
	int b = fgetc(classfile);
	若(b == EOF)
		eof_错误();
	return (u_int8_t)b;
  }

  /** Reads 在 一个 unsigned 16-位 integer. **/
  u_int16_t 读取_16(文件 *classfile)
  {
	int b1, b2;
	b1 = fgetc(classfile);
	若(b1 == EOF)
		eof_错误();
	b2 = fgetc(classfile);
	若(b2 == EOF)
		eof_错误();
	return (u_int16_t)((b1 << 8) | b2);
  }

  /** Reads 在 一个 值 来自 the constant pool. **/
  void skip_constant(文件 **classfile, u_int16_t **cur)
  {
	u_int16_t len;
	int seekerr = 1;
	pool[*cur] = ftell(classfile);
	switch(读取_8(classfile))
	{
	case CP_UTF8:
		len = 读取_16(classfile);
		seekerr = fseek(classfile, len, SEEK_CUR);
		break;
	case CP_类:
	case CP_字符串:
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
	默认:
		corrupt_错误();
	}
	若(seekerr)
		seek_错误();
  }

  void 错误(const char *格式, ...)
  {
	va_列出 ap;
	va_启动(ap, 格式);
	vfprintf(stderr, 格式, ap);
	va_end(ap);
	exit(1);
  }

  int 主要(int argc, char **argv)
  {
	文件 *classfile;
	u_int16_t cp_count, i, 此_类, classinfo_ptr;
	u_int8_t 长度;

	program = argv[^0^];

	若(!argv[^1^])
		错误("%s: Missing 输入 文件\n", program);
	classfile = fopen(argv[^1^], "rb");
	若(!classfile)
		错误("%s: 错误 opening %s\n", program, argv[^1^]);

	若(fseek(classfile, 8, SEEK_SET))  /** skip magic 和 版本 numbers **/
		seek_错误();
	cp_count = 读取_16(classfile);
	pool = calloc(cp_count, sizeof(long));
	若(!pool)
		错误("%s: 超出 内存 用于 constant pool\n", program);

	用于(i = 1; i < cp_count; ++i)
		skip_constant(classfile, &i);
	若(fseek(classfile, 2, SEEK_CUR))	/** skip access 标志 **/
		seek_错误();

	此_类 = 读取_16(classfile);
	若(此_类 < 1 || 此_类 >= cp_count)
		corrupt_错误();
	若(!pool[此_类] || pool[此_类] == -1)
		corrupt_错误();
	若(fseek(classfile, pool[此_类] + 1, SEEK_SET))
		seek_错误();

	classinfo_ptr = 读取_16(classfile);
	若(classinfo_ptr < 1 || classinfo_ptr >= cp_count)
		corrupt_错误();
	若(!pool[classinfo_ptr] || pool[classinfo_ptr] == -1)
		corrupt_错误();
	若(fseek(classfile, pool[classinfo_ptr] + 1, SEEK_SET))
		seek_错误();

	长度 = 读取_16(classfile);
	用于(i = 0; i < 长度; ++i)
	{
		u_int8_t x = 读取_8(classfile);
		若((x & 0x80) || !x)
		{
			若((x & 0xE0) == 0xC0)
			{
				u_int8_t y = 读取_8(classfile);
				若((y & 0xC0) == 0x80)
				{
					int c = ((x & 0x1f) << 6) + (y & 0x3f);
					若(c) putchar(c);
					else utf8_错误();
				}
				else utf8_错误();
			}
			else utf8_错误();
		}
		else 若(x == '/') putchar('.');
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
现在 simply `chmod +x` the `.class`, `.jar` 和/或 `.html` 文件 您
希望 到 execute.

到 add 一个 Java program 到 您的 path best put 一个 symbolic link 到 the 主要
.类 文件 进入 /usr/bin (或 another place 您 类似) omitting the .类
extension. The directory containing the original .类 文件 将 为
added 到 您的 CLASSPATH 期间 execution.


到 test 您的 新 setup, enter 在 the 以下 简单 Java app, 和 name
它 "HelloWorld.java":


	类 HelloWorld {
		公共 静态 void 主要(字符串 args[]) {
			系统.out.println("Hello World!");
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
到 execute Java Jar 文件, 简单 chmod the `*.jar` 文件 到 包含
```

       ./Application.jar


```
到 execute Java Applets, 简单 chmod the `*.html` 文件 到 包含
```

	./Applet.html


```
originally 由 Brian 一个. Lantz, brian@lantz.com
heavily edited 用于 binfmt_misc 由 Richard Günther
新 scripts 由 Colin J. Watson <cjw44@cam.ac.uk>
added executable Jar 文件 支持 由 Kurt Huwig <kurt@iku-netz.de>
