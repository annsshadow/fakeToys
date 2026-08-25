## 构建 libbpf


libelf zlib libbpf 的内部依赖，因此需要与之链接，并且必须安装在系统上以便应用程序正常工作。默认使pkg-config 查找 libelf，所调用的程序可通过 PKG_CONFIG 覆盖
如果在构建时不希望使pkg-config，可在调make 时设NO_PKG_CONFIG=1 将其禁用
要同时构建静态库 libbpf.a 与共享库 libbpf.so

    $ cd src
    $ make

要仅build/ 目录中构建静态库 libbpf.a，并连同 libbpf 头文件一起安装到暂存目录 root/

    $ cd src
    $ mkdir build root
    $ BUILD_STATIC_ONLY=y OBJDIR=build DESTDIR=root make install

要针对安装在 /build/root/ 下的自定libelf 依赖构建静态库 libbpf.a 与共享库 libbpf.so，并连同 libbpf 头文件一起安装到构建目录 /build/root/

    $ cd src
    $ PKG_CONFIG_PATH=/build/root/lib64/pkgconfig DESTDIR=/build/root make
