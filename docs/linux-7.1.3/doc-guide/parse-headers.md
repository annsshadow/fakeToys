## 包含 uAPI 头文件


有时，包含头文件和 C 示例代码是很有用的，以便描述用户空间 API，并在代码与
文档之间生成交叉引用。为用户空间 API 文件添加交叉引用还有一个额外的好处：
如果某个符号在文档中找不到，Sphinx 会生成警告。这有助于使 uAPI 文档与
内核改动保持同步。parse_headers.py <parse_headers> 提供了一种生成此类
交叉引用的方法。它必须在构建文档时通过 Makefile 调用。关于如何在内核树中
使用它，请参阅 `Documentation/userspace-api/media/Makefile` 中的示例。


##### tools/docs/parse_headers.py


######## 名称


parse_headers.py - 解析一个 C 文件，以识别函数、结构体、枚举和宏定义，并
创建到 Sphinx 文档的交叉引用。

######## 用法


parse-headers.py [-h] [-d] [-t] `FILE_IN` `FILE_OUT` `FILE_RULES`

######## 简介


将输入的 C 头文件或源文件 `FILE_IN` 转换为一个 ReStructured Text，通过
..parsed-literal 块包含，并为描述该 API 的文档文件创建交叉引用。它接受一个
可选的 `FILE_RULES` 文件，用于描述哪些元素将被忽略，或指向非默认引用
类型/名称。

输出写入 `FILE_OUT`。

它能够识别 `define`、`struct`、`typedef`、`enum` 以及枚举 `symbol`，并为它们
全部创建交叉引用。

它还能够区分用于指定 Linux 特定宏（用以定义 `ioctl`）的 `#define`。

```

    ignore ioctl VIDIOC_ENUM_FMT
    replace ioctl VIDIOC_DQBUF vidioc_qbuf
    replace define V4L2_EVENT_MD_FL_HAVE_FRAME_SEQ :c:type:`v4l2_event_motion_det`

```
######## 位置参数


  `FILE_IN`
      输入的 C 文件

  `FILE_OUT`
      输出的 RST 文件

  `FILE_RULES`
      例外文件（可选）

######## 选项


  `-h`, `--help`
      显示帮助信息并退出
  `-d`, `--debug`
      提高调试级别。可以多次使用
  `-t`, `--toc`
      不在字面块中输出，而是在 RST 文件中输出一个目录表（TOC）

######## 描述


从 `FILE_IN` 创建内核头文件的增强版本，为其每个 C 数据结构类型添加交叉链接，
并使用 reStructuredText 标记进行格式化，可以是原样，也可以是一个目录表。

它接受一个可选的 `FILE_RULES`，用于描述哪些元素将被忽略或指向非默认引用，
并可选择性地定义要使用的 C 命名空间。

其目标是允许拥有更全面的文档，其中 uAPI 头文件将为代码创建交叉引用链接。

输出写入 `FILE_OUT`。

`FILE_RULES` 可能包含三种类型的语句：**ignore**、**replace** 和 **namespace**。

默认情况下，它会为所有符号和宏定义创建规则，但也允许解析一个例外文件。此类
文件包含一组使用以下语法的规则：

1. 忽略规则：

    ignore **type** **symbol**

将符号从引用生成中移除。

2. 替换规则：

    replace **type** **old_symbol** **new_reference**

    将 **old_symbol** 替换为 **new_reference**。
    **new_reference** 可以是：

    - 一个简单的符号名；
    - 一个完整的 Sphinx 引用。

3. 命名空间规则

    namespace **namespace**

    设置交叉引用生成期间要使用的 C **namespace**。可被替换规则覆盖。

在忽略和替换规则中，**type** 可以是：

    - ioctl：
        用于形如 `_IO*` 的宏定义，例如 ioctl 定义

    - define：
        用于其它宏定义

    - symbol：
        用于枚举中定义的符号；

    - typedef：
        用于 typedef；

    - enum：
        用于非匿名枚举的名称；

    - struct：
        用于结构体。

######## 示例


```
    ignore define _VIDEODEV2_H

```
```
    enum foo { BAR1, BAR2, PRIVATE };

  It won't generate cross-references for ``PRIVATE``::

    ignore symbol PRIVATE

  对于同一个结构体，与其为每个符号创建一个交叉引用，不如让它们全部指向
  ``enum foo`` C 类型::

    replace symbol BAR1 :c:type:`foo`
    replace symbol BAR2 :c:type:`foo`

```
```
    namespace MC

```
######## 缺陷


Report bugs to Mauro Carvalho Chehab <mchehab@kernel.org>

######## 版权


Copyright (c) 2016, 2025 by Mauro Carvalho Chehab <mchehab+huawei@kernel.org>.

License GPLv2: GNU GPL version 2 <https://gnu.org/licenses/gpl.html>.

This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
