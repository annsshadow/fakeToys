
## TEE（可信执行环境）用户空间 API


include/uapi/linux/tee.h 定义了与 TEE 的通用接口

用户空间（客户端）通过打开 /dev/tee[0-9]* /dev/teepriv[0-9]* 来连接到驱动

- TEE_IOC_SHM_ALLOC 分配共享内存并返回一个文件描述符，用户空间可以对其进mmap。当用户空间不再需要该文件描述符时，应当将其关闭。当不再需要共享内存时，应当使munmap() 解除映射，以便重用内存

- TEE_IOC_VERSION 让用户空间了解该驱动处理的是哪个 TEE 及其能力

- TEE_IOC_OPEN_SESSION 打开与可信应用程序的新会话

- TEE_IOC_INVOKE 调用可信应用程序中的函数

- TEE_IOC_CANCEL 可以取消正在进行TEE_IOC_OPEN_SESSION TEE_IOC_INVOKE

- TEE_IOC_CLOSE_SESSION 关闭与可信应用程序的会话

客户端有两类，正常客户端与请求者（supplicant）。后者是 TEE 访问 Linux 中资源（例如文件系统访问）的辅助进程。正常客户端打开 /dev/tee[0-9]*，而请求者打开 /dev/teepriv[0-9]

客户端与 TEE 之间的大部分通信对驱动是不透明的。驱动的主要工作是接收来自客户端的请求，将其转发TEE，并把结果发回。在请求者的情况下，通信方向相反，TEE 向请求者发送请求，请求者随后将结果发回
