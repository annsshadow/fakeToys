## 设备表


PCMCIA 设备与驱动的匹配使用以下一个或多个准则完成：

- 厂商 ID
- 卡 ID
- 产品 ID 字符串 _及_ 这些字符串的哈希
- 功能 ID
- 设备功能（实际与伪）

你应使用 include/pcmcia/device_id.h 中的辅助宏来生成将设备匹配到驱动的
struct pcmcia_device_id[] 条目。

若想匹配产品 ID 字符串，你还需要将字符串的 crc32 哈希传给宏，例如若想
匹配产品 ID 字符串 1，你需要使用

PCMCIA_DEVICE_PROD_ID1("some_string", 0x(hash_of_some_string)),

如果哈希不正确，内核会在模块初始化时于 "dmesg" 中通知你，并告知你
正确的哈希。

你可以通过 cat 该 PCMCIA 设备 sysfs 目录下的 "modalias" 文件来确定产品
ID 字符串的哈希。它会生成如下形式的字符串：
pcmcia:m0149cC1ABf06pfn00fn00pa725B842DpbF1EFEE84pc0877B627pd00000000

"pa" 之后的十六进制值是产品 ID 字符串 1 的哈希，"pb" 之后的是字符串 2 的
哈希，依此类推。

或者，你可以使用 crc32hash（见 tools/pcmcia/crc32hash.c）来确定 crc32
哈希。只需将你想计算的字符串作为参数传给该程序，例如：
$ tools/pcmcia/crc32hash "Dual Speed"
