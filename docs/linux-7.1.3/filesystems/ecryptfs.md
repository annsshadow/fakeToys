
## eCryptfs：Linux 的堆叠式加密文件系统


eCryptfs 是自由软件。详情请参阅 COPYING 文件文档请参doc/ 子目录中的文件。构建与安装说明请参INSTALL 文件
:Maintainer: Phillip Hellewell
:Lead developer: Michael A. Halcrow <mhalcrow@us.ibm.com>
:Developers: Michael C. Thompson
             Kent Yoder
:Web Site: http://ecryptfs.sf.net

本软件当前仍在开发中。请务必对写eCryptfs 的任何数保留一份备份副本
eCryptfs 需要可SourceForge 站点下载的用户空间工具：

http://sourceforge.net/projects/ecryptfs/

用户空间需求包括：

- David Howells 的用户空间密钥环头文件与库（版本 1.0 或更高）  可从以下地址获取  http://people.redhat.com/~dhowells/keyutils/
- Libgcrypt



   eCryptfs 的测试版/实验性发布中，升eCryptfs 时，你应该先将文   复制到未加密的位置，然后再将文件复制回新eCryptfs 挂载点，
   以迁移这些文件

## 挂载级口

创建一个新目录，eCryptfs 将把加密文件写入其中（例/root/crypt）然后，创建挂载点目录

```

    mount -t ecryptfs /root/crypt /mnt/crypt

```
系统会提示你输入口令与盐值（盐值可以为空）
```

    echo "Hello, World" > /mnt/crypt/hello.txt

```
操作将完成。注/root/crypt 中出现了一个大小至少为 12288 字节
（取决于宿主机页大小）的新文件。这就是你刚刚写入内容的加密底层文件要完整地测试读取，你需要清空用户会话密钥环
keyctl clear @u

然后按照上面给出的说明卸/mnt/crypt 并重新挂载
```

    cat /mnt/crypt/hello.txt


```
## 注意事项


eCryptfs 0.1 版本只应挂载到（1）空目录，或）仅包含eCryptfs
创建的文件的目录中。如果你挂载一个包含非 eCryptfs 创建既有文件的目录，其行为是未定义的。除非纯粹出于调试或开发目的，
否则不要以更高的详细级别运行 eCryptfs，因为在那种情况机密值会被写入系统日志

Mike Halcrow
mhalcrow@us.ibm.com
