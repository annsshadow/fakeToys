## NFS 瀹㈡埛绔。

## NFS 瀹㈡埛绔。

NFS 版本 2 协议最早记录在 RFC1094989 3 月）中。自那以后又发布了两个主要的 NFS 版本NFSv3 记录RFC1813995 6 月），NFSv4 记录RFC3530003 4 月）
Linux NFS 客户端目前支持上述所有已发布的版本，并且正在着手增加对 NFSv4 协议次要版本 1 的支持
本文档的目的是提供一些可由系统管理员配置NFS 客户端特殊特性的信息

## nfs4_unique_id 参数


NFSv4 要求客户端用一个唯一字符串向服务器标识自己。一个客户端与一个服务器之间共享的文件打开
和锁状态都关联于这个身份。为了支持健壮的 NFSv4 状态恢复和透明的状态迁移，这个身份字符串在
客户端重启期间不能改变
在没有任何其它干预的情况下，Linux 客户端使用一个包含本地系统节点名的字符串。然而，系统管理常常不确保节点名是完全限定（fully qualified）的，并且在客户端系统的整个生命周期内不改变。节点名
可能有其它管理上的要求，需要特定的行为，而这种行为作nfs_client_id4 字符串的一部分并不能很地工作
nfs.nfs4_unique_id 引导参数指定了一个唯一字符串，可在 NFS 客户端向服务器标识自己时与系统节点名
一起使用。因此，如果系统的节点名不唯一，它nfs.nfs4_unique_id 有助于防止与其它客户端冲突
nfs.nfs4_unique_id 字符串通常是一UUID，尽管它可以包含任何被认为在所NFS 客户端之间唯一的内容nfs4_unique_id 字符串应在安装客户端系统时选择，就像系统的根文件系统在安装时标签上获得一个新UUID 一样
该字符串应在客户端的整个生命周期内保持固定。如果小心地确保客户端干净关闭且所有未完成NFSv4
状态都已过期，则可以安全地更改它，以防NFSv4 状态丢失
这个字符串可以存储在 NFS 客户端的 grub.conf 中，也可以通过 PXE 等网络引导设施提供。它也可以作nfs.ko 模块参数指定
这个唯一标识字符串对于所有在容器中运行的 NFS 客户端都是相同的，除非它被写/sys/fs/nfs/net/nfs_client/identifier 的值覆盖，该值将是写入它的进程所在网络命名空间的本地值

## DNS 解析

NFSv4 允许一个服务器通过特殊"fs_locations" 属性把 NFS 客户端引向已迁移到另一台服务器上的数据请参`RFC3530 Section 6: Filesystem Migration and Replication`_ `Implementation Guide for Referrals in NFSv4`_

fs_locations 信息可以采用 ip 地址加路径，DNS 主机名加路径的形式。后者要NFS 客户端做一DNS 查找以挂载新卷，因此需要通过一upcall 让用户态来提供此服务
假设用户已将 'rpc_pipefs' 文件系统挂载在通常/var/lib/nfs/rpc_pipefs，upcall 由以下步骤组成：

   (1) 进程检dns_resolve 缓存，看它是否包含一个有效条目。如果有，就返回该条目并退出
   (2) 如果不存在有效条目，则运行辅助脚'/sbin/nfs_cache_getent'
       （可以使'nfs.cache_getent' 内核引导参数更改），带两个参数：
       - 缓存名，"dns_resolve"
       - 要解析的主机
   (3) 查找到相应的 ip 地址后，辅助脚本以如下（文本）格式把结果写入 rpc_pipefs 伪文       '/var/lib/nfs/rpc_pipefs/cache/dns_resolve/channel'
		"<ip address> <hostname> <ttl>\n"

       其中 <ip address> 采用通常IPv423.456.78.90）或 IPv6
       （ffee:ddcc:bbaa:9988:7766:5544:3322:1100、ffee::1100 等）格式       <hostname> 与辅助脚本的第二个参数相同，<ttl> 是该缓存条目的“生存时间”（以秒为单位）
```

            If <ip address> is invalid, say the string "0", then a negative
            entry is created, which will cause the kernel to treat the hostname
            as having no valid DNS translation.



```
## 一个基本的示例 /sbin/nfs_cache_getent


    #!/bin/bash
    #
    ttl=600
    #
    cut=/usr/bin/cut
    getent=/usr/bin/getent
    rpc_pipefs=/var/lib/nfs/rpc_pipefs
    #
    die()
    {
        echo "Usage: $0 cache_name entry_name"
        exit 1
    }

    [ $# -lt 2 ] && die
    cachename="$1"
    cache_path=${rpc_pipefs}/cache/${cachename}/channel

    case "${cachename}" in
        dns_resolve)
            name="$2"
            result="$(${getent} hosts ${name} | ${cut} -f1 -d\ )"
            [ -z "${result}" ] && result="0"
            ;;
        *)
            die
            ;;
    esac
    echo "${result} ${name} ${ttl}" >${cache_path}
