## 导出内核头文件以供用户空间使用


"make headers_install" 命令以内核头文件的形式导出，使其适合供用户空间程序使用。

导出的 Linux 内核头文件描述了面向试图使用内核服务的用户空间程序的 API。这些内核
头文件被系统的 C 库（例如 glibc 或 uClibc）用来定义可用的系统调用，以及与这些
系统调用一起使用的常量和结构体。C 库的头文件包含来自 "linux" 子目录的内核头文件。
系统的 libc 头文件通常安装在默认位置 /usr/include，内核头文件则位于其下的子目录
中（最值得注意的是 /usr/include/linux 和 /usr/include/asm）。

内核头文件向后兼容，但不向前兼容。这意味着针对使用较旧内核头文件的 C 库构建的程序
应当可以在较新的内核上运行（尽管可能无法使用新特性），但针对较新内核头文件构建的
程序可能无法在较旧的内核上工作。

"make headers_install" 命令可以在内核源代码树的顶层目录中运行（或使用标准的树外
构建）。它需要两个参数：

```
make headers_install ARCH=i386 INSTALL_HDR_PATH=/usr
```

ARCH 指明要为哪种架构生成头文件，默认值为当前架构。所导出内核头文件的 linux/asm
目录是平台特定的，要查看受支持架构的完整列表，可使用：

```
ls -d include/asm-* | sed 's/.*-//'
```

INSTALL_HDR_PATH 指明头文件的安装位置。默认值为 "./usr"。

会在 INSTALL_HDR_PATH 内部自动创建一个 'include' 目录，头文件被安装在
'INSTALL_HDR_PATH/include' 中。

内核头文件导出基础设施由 David Woodhouse <dwmw2@infradead.org> 维护。
