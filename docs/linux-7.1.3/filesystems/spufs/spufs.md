
## spufs

## 名称（Name）

       spufs - SPU 文件系统

## 描述（Description）

       SPU 文件系统用于实现了 Cell 宽带引擎架构（Cell Broadband Engine
       Architecture）的 PowerPC 机器上，以便访问协同处理器单元（Synergistic
       Processor Units，简称 SPU）。

       该文件系统提供了类似于 POSIX 共享内存或消息队列的名字空间。对文件系统拥有写权限的
       用户可以使用 spu_create(2) 在 spufs 根目录下建立 SPU 上下文。

       每个 SPU 上下文都由一个包含一组预定义文件的目录来表示。这些文件可用于操纵逻辑 SPU
       的状态。用户可以修改这些文件的权限，但实际不能增删文件。

## 挂载选项（Mount Options）

       uid=<uid>
              设置拥有挂载点的用户，默认为 0（root）。

       gid=<gid>
              设置拥有挂载点的组，默认为 0（root）。

## 文件（Files）

       spufs 中的文件大体上遵循常规系统调用（如 read(2) 或 write(2)）的标准行为，但通常只
       支持常规文件系统所支持操作的一个子集。此列表详述了所支持的操作，以及与各自 man 页中
       行为的差异。

       所有支持 read(2) 操作的文件也都支持 readv(2)，所有支持 write(2) 操作的文件也都支持
       writev(2)。所有文件都支持 access(2) 与 stat(2) 系列操作，但只有 struct stat 中的
       st_mode、st_nlink、st_uid 和 st_gid 字段包含可靠信息。

       所有文件都支持 chmod(2)/fchmod(2) 与 chown(2)/fchown(2) 操作，但无法授予与可能操作
       相矛盾的权限，例如在 wbox 文件上的读权限。

       当前的一组文件如下：

   /mem
       SPU 本地存储内存的内容。可以像常规共享内存文件一样访问，并在 SPU 的地址空间中同时
       包含代码与数据。在已打开的 mem 文件上可能进行的操作有：

       read(2), pread(2), write(2), pwrite(2), lseek(2)
              这些操作都按文档所述工作，例外是 seek(2)、write(2) 和 pwrite(2) 不支持超出
              文件末尾。文件大小即 SPU 本地存储的大小，通常为 256 千字节。

       mmap(2)
              把 mem 映射到进程地址空间中，即可在进程地址空间内访问 SPU 本地存储。只允许
              MAP_SHARED 映射。

   /mbox
       第一个 SPU 到 CPU 的通信邮箱。该文件是只读的，并且可以以 32 位为单位读取。该文件只能
       以非阻塞模式使用，即便是 poll() 也不会在其上阻塞。在已打开的 mbox 文件上可能进行的
       操作有：

       read(2)
              如果请求的 count 小于 4，read 返回 -1 并把 errno 设为 EINVAL。如果邮箱中没有
              可用数据，返回值设为 -1 且 errno 变成 EAGAIN。当成功读取出数据时，会把四个字节
              放入数据缓冲区，并返回值 4。

   /ibox
       第二个 SPU 到 CPU 的通信邮箱。该文件与第一个邮箱文件类似，但可以以阻塞 I/O 模式读取，
       并且可以使用 poll 系列系统调用来等待它。在已打开的 ibox 文件上可能进行的操作有：

       read(2)
              如果请求的 count 小于 4，read 返回 -1 并把 errno 设为 EINVAL。如果邮箱中没有
              可用数据，且文件描述符是以 O_NONBLOCK 打开的，返回值设为 -1 且 errno 变成
              EAGAIN。

              如果邮箱中没有可用数据，且文件描述符不是以 O_NONBLOCK 打开的，调用将阻塞，直到
              SPU 写入其中断邮箱通道。当成功读取出数据时，会把四个字节放入数据缓冲区，并返回
              值 4。

       poll(2)
              只要有数据可读，对 ibox 文件的 poll 就会返回 (POLLIN | POLLRDNORM)。

   /wbox
       CPU 到 SPU 的通信邮箱。它是只写的，可以以 32 位为单位写入。如果邮箱已满，write() 将
       阻塞，并可以使用 poll 等待其再次变为空。在已打开的 wbox 文件上可能进行的操作有：
       write(2) 如果请求的 count 小于 4，write 返回 -1 并把 errno 设为 EINVAL。如果邮箱中
       没有可用空间，且文件描述符是以 O_NONBLOCK 打开的，返回值设为 -1 且 errno 变成 EAGAIN。

       如果邮箱中没有可用空间，且文件描述符不是以 O_NONBLOCK 打开的，调用将阻塞，直到 SPU
       从其 PPE 邮箱通道读取。当成功写入数据时，会把四个字节放入数据缓冲区，并返回值 4。

       poll(2)
              只要有可写空间，对 ibox 文件的 poll 就会返回 (POLLOUT | POLLWRNORM)。

   /mbox_stat, /ibox_stat, /wbox_stat
       只读文件，包含当前队列的长度，即从 mbox 或 ibox 可读出多少字，或向 wbox 写入多少字
       而不会阻塞。这些文件只能以 4 字节为单位读取，并返回一个 big-endian 的二进制整数。在
       已打开的 `*box_stat` 文件上可能进行的操作有：

       read(2)
              如果请求的 count 小于 4，read 返回 -1 并把 errno 设为 EINVAL。否则，会向数据
              缓冲区放入一个四字节的值，包含在不阻塞或产生 EAGAIN 的前提下，可从（对于
              mbox_stat 和 ibox_stat）或向（对于 wbox_stat）相应邮箱读/写的元素数量。

   /npc, /decr, /decr_status, /spu_tag_mask, /event_mask, /srr0
       SPU 的内部寄存器。其表示形式为 ASCII 字符串，包含下一条要执行指令的数值。这些文件可
       用于读/写模式以进行调试，但程序的正常运行不应依赖它们，因为除 npc 外访问其中任何一个
       都需要保存 SPU 上下文，因此效率很低。

       这些文件的内容为：

       =================== ===================================
       npc                 Next Program Counter
       decr                SPU Decrementer
       decr_status         Decrementer Status
       spu_tag_mask        MFC tag mask for SPU DMA
       event_mask          Event mask for SPU interrupts
       srr0                Interrupt Return address register
       =================== ===================================

       npc、decr、decr_status、spu_tag_mask、event_mask 或 srr0 这些已打开文件上可能进行的
       操作有：

       read(2)
              当提供给 read 调用的 count 短于指针值加上换行符所需的长度时，对同一文件描述符
              的后续读取会补全该字符串，而不管运行中的 SPU 任务是否修改了该寄存器。当读取完
              一个完整字符串后，所有后续的读操作都会返回零字节，需要打开一个新的文件描述符
              才能再次读取该值。

       write(2)
               对文件的写操作会把寄存器设置为字符串中给定的值。字符串从开头解析到第一个非
               数字字符或缓冲区末尾。对同一文件描述符的后续写操作会覆盖先前的设置。

   /fpcr
       该文件以四字节长的文件形式提供对浮点状态与控制寄存器（Floating Point Status and
       Control Register）的访问。fpcr 文件上的操作有：

       read(2)
              如果请求的 count 小于 4，read 返回 -1 并把 errno 设为 EINVAL。否则，会向数据
              缓冲区放入一个四字节的值，包含 fpcr 寄存器的当前值。

       write(2)
              如果请求的 count 小于 4，write 返回 -1 并把 errno 设为 EINVAL。否则，会从数据
              缓冲区复制一个四字节的值，以更新 fpcr 寄存器的值。

   /signal1, /signal2
       SPU 的两个信号通知通道。这些是作用在 32 位字上的读-写文件。向其中一个文件写入会在 SPU
       上触发一个中断。写入 signal 文件的值可以通过通道读从 SPU 读取，或通过该文件从宿主机
       用户空间读取。该值被 SPU 读取后会被重置为零。在已打开的 signal1 或 signal2 文件上可能
       进行的操作有：

       read(2)
              如果请求的 count 小于 4，read 返回 -1 并把 errno 设为 EINVAL。否则，会向数据
              缓冲区放入一个四字节的值，包含指定信号通知寄存器的当前值。

       write(2)
              如果请求的 count 小于 4，write 返回 -1 并把 errno 设为 EINVAL。否则，会从数据
              缓冲区复制一个四字节的值，以更新指定信号通知寄存器的值。该信号通知寄存器会被
              输入数据替换，或更新为旧值与输入数据的按位或（OR），取决于 signal1_type 或
              signal2_type 文件的内容。

   /signal1_type, /signal2_type
       这两个文件改变 signal1 和 signal2 通知文件的行为。它们包含一个被读作 "1" 或 "0" 的
       数字 ASCII 字符串。在模式 0（覆盖）下，硬件用写入其中的数据替换信号通道的内容；在
       模式 1（逻辑或）下，硬件累积随后写入其中的各个比特。在已打开的 signal1_type 或
       signal2_type 文件上可能进行的操作有：

       read(2)
              当提供给 read 调用的 count 短于数字加上换行符所需的长度时，对同一文件描述符的
              后续读取会补全该字符串。当读取完一个完整字符串后，所有后续的读操作都会返回零
              字节，需要打开一个新的文件描述符才能再次读取该值。

       write(2)
               对文件的写操作会把寄存器设置为字符串中给定的值。字符串从开头解析到第一个非
               数字字符或缓冲区末尾。对同一文件描述符的后续写操作会覆盖先前的设置。

## 示例（Examples）

       /etc/fstab 条目
              none      /spu      spufs     gid=spu   0    0

## 作者（Authors）

       Arnd  Bergmann  <arndb@de.ibm.com>,  Mark  Nutter <mnutter@us.ibm.com>,
       Ulrich Weigand <Ulrich.Weigand@de.ibm.com>

## 参见（See Also）

       capabilities(7), close(2), spu_create(2), spu_run(2), spufs(7)
