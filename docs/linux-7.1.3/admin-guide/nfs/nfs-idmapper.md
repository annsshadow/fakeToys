## NFS ID 映射器


Id 映射器被 NFS 用于将用户和组 id 转换为名称，并将用户和组名称转换为 id。
该转换的一部分涉及向用户态发起 upcall 以请求信息。NFS 可通过两种方式获取
这些信息：调用 /sbin/request-key，或调用 rpc.idmap 守护进程。

NFS 会先尝试调用 /sbin/request-key。若成功，结果将使用通用的 request-key
缓存进行缓存。只有当 /etc/request-key.conf 未为 id_resolver 密钥类型配置时
该调用才会失败，若想使用 request-key 方法，请参见下文“配置”一节。

若对 /sbin/request-key 的调用失败（即 /etc/request-key.conf 未使用
id_resolver 密钥类型配置），则 id 映射器将向遗留的 rpc.idmap 守护进程请求
id 映射。该结果将存储在自定义的 NFS idmap 缓存中。

## 配置


需要修改文件 /etc/request-key.conf，以便 /sbin/request-key 能够引导该
upcall。应添加以下行：

`#OP	TYPE	DESCRIPTION	CALLOUT INFO	PROGRAM ARG1 ARG2 ARG3 ...`
`#======	=======	===============	===============	===============================`
`create	id_resolver	**	**		/usr/sbin/nfs.idmap %k %d 600`


这将把所有 id_resolver 请求导向程序 /usr/sbin/nfs.idmap。最后一个参数 600
定义了密钥将在未来多少秒后过期。该参数对 /usr/sbin/nfs.idmap 是可选的。
未指定超时时，nfs.idmap 默认使用 600 秒。

```
  uid:  查找给定用户的 UID
  gid:  查找给定组的 GID
 user:  查找给定 UID 的用户名
group:  查找给定 GID 的组名

```
你可以单独处理其中任意一种，而不必使用通用的 upcall 程序。若想使用自己的程序
进行 uid 查找，可以编辑 request-key.conf，使其类似如下：

`#OP	TYPE	DESCRIPTION	CALLOUT INFO	PROGRAM ARG1 ARG2 ARG3 ...`
`#======	=======	===============	===============	===============================`
`create	id_resolver	uid:**	**		/some/other/program %k %d 600`
`create	id_resolver	**	**		/usr/sbin/nfs.idmap %k %d 600`


注意新行被添加在通用程序所在行之上。request-key 会找到第一个匹配的行及相应
的程序。在此例中，/some/other/program 将处理所有 uid 查找，而
/usr/sbin/nfs.idmap 将处理 gid、user 和 group 查找。

有关 request-key 函数的更多信息，请参见 Documentation/security/keys/request-key.rst。

## nfs.idmap


nfs.idmap 设计为由 request-key 调用，不应“手动”运行。该程序接受两个参数，
一个序列化的密钥和一个密钥描述。序列化密钥首先被转换为 key_serial_t，然后
作为参数传递给 keyctl_instantiate（二者都是 keyutils.h 的一部分）。

实际的查找由 nfsidmap.h 中的函数执行。nfs.idmap 通过查看描述字符串的第一部分
来确定要调用的正确函数。例如，uid 查找描述将形如 “uid:user@domain”。

若密钥被实例化，nfs.idmap 返回 0，否则返回非 0。
