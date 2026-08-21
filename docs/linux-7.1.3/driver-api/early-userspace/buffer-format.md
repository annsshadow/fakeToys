## initramfs 缓冲区格

Al Viro，H. Peter Anvin

随着内核 2.5.x，旧initial ramdisk"（初RAM 盘）协议被补充了一"initial ramfs"（初RAM 文件系统）协议。initramfs 内容使用initrd 相同内存缓冲区协议传递，但内容不同。initramfs 缓冲区包含一个被展开ramfs 文件
系统的归档；本文档详initramfs 缓冲区格式
initramfs 缓冲区格式基"newc" "crc" CPIO 格式，并可以cpio(1) 工具创建cpio 归档可以使用 gzip(1) 或任何通过 CONFIG_DECOMPRESS_* 提供的其他算法压缩因此，一个有效的 initramfs 缓冲区版本就是一个单独的 .cpio.gz 文件
initramfs 缓冲区的完整格式由以下定```

	*	表示"0 次或多次出现"
	(|)	表示可选项
	+	表示连接
	GZIP()	表示对操作数进行 gzip 压缩
	BZIP2()	表示对操作数进行 bzip2 压缩
	LZMA()	表示对操作数进行 lzma 压缩
	XZ()	表示对操作数进行 xz 压缩
	LZO()	表示对操作数进行 lzo 压缩
	LZ4()	表示对操作数进行 lz4 压缩
	ZSTD()	表示对操作数进行 zstd 压缩
	ALGN(n)	表示用空字节填充n 字节边界

	initramfs := ("\0" | cpio_archive | cpio_compressed_archive)*

	cpio_compressed_archive := (GZIP(cpio_archive) | BZIP2(cpio_archive)
		| LZMA(cpio_archive) | XZ(cpio_archive) | LZO(cpio_archive)
		| LZ4(cpio_archive) | ZSTD(cpio_archive))

	cpio_archive := cpio_file* + (<nothing> | cpio_trailer)

	cpio_file := ALGN(4) + cpio_header + filename + "\0" + ALGN(4) + data

	cpio_trailer := ALGN(4) + cpio_header + "TRAILER!!!\0" + ALGN(4)


```
通俗地说，initramfs 缓冲区包含一组已压缩或未压缩cpio 归档newc" "crc" 格式）；成员之间可以添加任意数量的零字节（用于填充）
cpio "TRAILER!!!" 条目（cpio 归档结束标记）是可选的，但不会被忽略；参见
下文"硬链接的处理"
cpio_header 的结构如下（所有字段都包含十六进制 ASCII 数字，左侧用 '0' 补全字段的整个宽度，例如整数 4780 ASCII 字符"000012ac" 表示）：

============= ================== ==============================================
字段      字段大小	 含义
============= ================== ==============================================
c_magic	      6 字节		 字符"070701" "070702"
c_ino	      8 字节		 文件 inode c_mode	      8 字节		 文件模式与权c_uid	      8 字节		 文件 uid
c_gid	      8 字节		 文件 gid
c_nlink	      8 字节		 链接c_mtime	      8 字节		 修改时间
c_filesize    8 字节		 数据字段大小
c_maj	      8 字节		 文件设备号的主设备部c_min	      8 字节		 文件设备号的从设备部c_rmaj	      8 字节		 设备节点引用的主设备部分
c_rmin	      8 字节		 设备节点引用的从设备部分
c_namesize    8 字节		 文件名长度，含末尾的 \0
c_chksum      8 字节		 数据字段的校验和（若 c_magic 070702）；
				 否则为零
============= ================== ==============================================

c_mode 字段Linux stat(2) 返回st_mode 内容一致，编码了文件类型和文件
权限
除非设置CONFIG_INITRAMFS_PRESERVE_MTIME=y，否c_mtime 被忽略
对于任何非普通文件或符号链接的文件，c_filesize 应为零
只要该值不超过 PATH_MAX，c_namesize 可以计入多个末尾 '\0'。这对于确保后续的文数据段对齐（例如对齐到文件系统块边界）很有用
c_chksum 字段包含数据字段中所有字节的简32 位无符号求和。cpio(1) 将此称为
"crc"，这显然是不正确的（循环冗余校验是一种不同的、且显著更强的完整性检查）然而这正是所使用的算法
如果文件名为 "TRAILER!!!"，这实际上是一个归档结束标记；归档结束标记c_filesize 必须为零

## 硬链接的处理


当看到一c_nlink > 1 的非目录时，会在元组缓冲区中查找 (c_maj,c_min,c_ino)
元组。如果未找到，则将其存入元组缓冲区，并按常规创建该条目；如果找到，则创建
一个硬链接，而不是文件的第二份副本。包含第二份文件内容不是必需的（但允许）如果不包含文件内容，c_filesize 字段应设为零，表示后面没有数据段。如果存数据，则文件的先前实例会被覆盖；这使得携带数据的文件实例可以出现在序列中的任位置（据报告 GNU cpio 仅将数据附加到文件的最后一个实例）
对于符号链接，c_filesize 不能为零
当看到一"TRAILER!!!" 归档结束标记时，元组缓冲区被重置。这使得独立生成的归可以被连接在一起
因此，要合并来自不同源的文件数据（而无需重新生成 (c_maj,c_min,c_ino) 字段），
可以使用以下任一技术：

a) "TRAILER!!!" 归档结束标记分隔不同的文件数据源；或

b) 确保所有非目录条目c_nlink == 1