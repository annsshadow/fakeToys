## DNS 解析器模


 - 概述
 - 编译
 - 设置
 - 用法
 - 机制
 - 调试


## 概述


DNS 解析器模块为内核服务提供了一种通过请求 key 类型dns_resolver 
密钥来进DNS 查询的方法。这些查询通过 /sbin/request-key 向上调用
（upcall）到用户空间

这些例程必须由用户空间工dns.upcall、cifs.upcall request-key 支持
它仍在开发中，尚未提供完整的功能集。它支持的功能包括：

 - 实现dns_resolver key_type 以联系用户空间

它尚不支持以AFS 功能

 - AFSDB 资源记录DNS 查询支持

此代码从 CIFS 文件系统中提取


## 编译


```

	CONFIG_DNS_RESOLVER	- tristate "DNS Resolver support"


```
## 设置


要设置此功能，必须修/etc/request-key.conf 文件，以/sbin/request-key
能够适当地引导向上调用。例如，为了处理基本dname IPv4/IPv6 地址
解析，应添加如下一行：
```

	#OP	TYPE		DESC	CO-INFO	PROGRAM ARG1 ARG2 ARG3 ...
	#======	============	=======	=======	==========================
	create	dns_resolver  	*	*	/usr/sbin/cifs.upcall %k

```
为了引导对查询类'foo' 的查询，应添加如下一行：
```

	create	dns_resolver  	foo:*	*	/usr/sbin/dns.foo %k


```
## 用法


```

	#include <linux/dns_resolver.h>

```
```

	int dns_query(const char *type, const char *name, size_t namelen,
		     const char *options, char **_result, time_t *_expiry);

```
这是基本的访问函数。它查找缓存DNS 查询，如果没找到，则向上调用用户空间
发起一个新DNS 查询，该查询随后可能被缓存。密钥描述被构造为如下字符串：
```

		[<type>:]<name>

```
其中 <type> 可选地指定要调用的特定向上调用程序，并因此指定查询的类型，
<name> 指定要查找的字符串。默认查询类型是直接的主机名IP 地址集合查找

name 参数不要求是 NUL 结尾的字符串，其长度应由 namelen 参数给出

options 参数可以NULL，也可以是一组适合该查询类型的选项

返回值是一个适合该查询类型的字符串。例如，对于默认查询类型，它只是一
逗号分隔IPv4 IPv6 地址列表。调用者必须释放该结果

成功时返回结果字符串的长度，否则返回负的错误码。如DNS 查找失败，将
返回 -EKEYREJECTED

如果 _expiry NULL，则结果的到期时间（TTL）也会被返回

内维持有一个内部密钥环（keyring），在其中缓存已查找的密钥。任何具
CAP_SYS_ADMIN 能力的进程都可以通过对该密钥ID 使用 KEYCTL_KEYRING_CLEAR
来清除它


## 从用户空间读DNS 密钥


dns_resolver 类型的密钥可以使keyctl_read() "keyctl read/print/pipe"
从用户空间读取


## 机制


dns_resolver 模块注册了一个名"dns_resolver" 的密钥类型。此类型的密
用于在用户空间之间传输和缓存 DNS 查找结果

当调dns_query() 时，它调request_key() 在本地密钥环中搜索缓存的 DNS
结果。如果没找到，它会向上调用用户空间以获取新结果

向用户空间的向上调用是通过 request_key() 向上调用向量进行的，并由
/etc/request-key.conf 中的配置行引导，这些配置行告/sbin/request-key
运行什么程序来实例化（instantiate）该密钥

向上调用处理程序程序负责查询 DNS，将结果处理为适合传递给
keyctl_instantiate_key() 例程的形式。然后它将数据传递给
dns_resolver_instantiate()，后者剥离并处理数据中任何包含的选项，然后将
字符串的剩余部分作为载荷（payload）附加到密钥上

向上调用处理程序程序应将密钥的到期时间设为它从中提取结果的所有记录中
最低的 TTL。这意味着当该密钥持有的数据到期时，密钥将被丢弃并重新创建

dns_query() 返回附加到密钥的值的副本，或者如果指示了错误则返回该错误

有关 request-key 函数的更多信息，请参Documentation/security/keys/request-key.rst


## 调试


可以通过1 写入以下内容来动态开启调试消息：
```

	/sys/module/dns_resolver/parameters/debug
```
