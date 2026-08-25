
## Tracefs ring-buffer 内存映射


:Author: Vincent Donnefort <vdonnefort@google.com>

## 概述


Tracefs ring-buffer 的内存映射提供了一种高效的数据流传输方法，因为无需进行内存拷贝。映射了
ring-buffer 的应用程序随即成为该 ring-buffer 的一个消费者，方式类似trace_pipe
## 内存映射设置


该映射通过 mmap() trace_pipe_raw 接口来工作
映射的第一个系统页包含 ring-buffer 的统计信息与描述，被称为 meta-page（元页）。元页最重要字段之一reader。它包含可被映射者安全读取的子缓冲区（sub-buffer）ID（参ring-buffer-design.rst）
元页之后是所有子缓冲区，ID 升序排列。因此可以轻松地知道 reader 在映射中的起始位置：


        reader_id = meta->reader->id;
        reader_offset = meta->meta_page_size + reader_id * meta->subbuf_size;

当应用程序处理完当前 reader 后，它可以使trace_pipe_raw ioctl() TRACE_MMAP_IOCTL_GET_READER
获取一个新reader。该 ioctl 同时会更新元页字段
## 限制


当某Tracefs ring-buffer 上存在映射时，无法对其调整大小（无论是增大整ring-buffer 还是
每个 subbuf）。也无法使用 snapshot（快照），并且会导致 splice 拷贝 ring buffer 数据，而非
使用来自 ring buffer 的无拷贝交换
允许并发读者（无论是另一个映射该 ring-buffer 的应用程序，还是内核通过 trace_pipe），但不推荐它们将竞争该 ring-buffer，输出的结果是不可预测的，就trace_pipe 上的并发读者一样
## 示例



        #include <fcntl.h>
        #include <stdio.h>
        #include <stdlib.h>
        #include <unistd.h>

        #include <linux/trace_mmap.h>

        #include <sys/mman.h>
        #include <sys/ioctl.h>

        #define TRACE_PIPE_RAW "/sys/kernel/tracing/per_cpu/cpu0/trace_pipe_raw"

        int main(void)
        {
                int page_size = getpagesize(), fd, reader_id;
                unsigned long meta_len, data_len;
                struct trace_buffer_meta *meta;
                void **map, **reader, *data;

                fd = open(TRACE_PIPE_RAW, O_RDONLY | O_NONBLOCK);
                if (fd < 0)
                        exit(EXIT_FAILURE);

                map = mmap(NULL, page_size, PROT_READ, MAP_SHARED, fd, 0);
                if (map == MAP_FAILED)
                        exit(EXIT_FAILURE);

                meta = (struct trace_buffer_meta *)map;
                meta_len = meta->meta_page_size;

                printf("entries:        %llu\n", meta->entries);
                printf("overrun:        %llu\n", meta->overrun);
                printf("read:           %llu\n", meta->read);
                printf("nr_subbufs:     %u\n", meta->nr_subbufs);

                data_len = meta->subbuf_size * meta->nr_subbufs;
                data = mmap(NULL, data_len, PROT_READ, MAP_SHARED, fd, meta_len);
                if (data == MAP_FAILED)
                        exit(EXIT_FAILURE);

                if (ioctl(fd, TRACE_MMAP_IOCTL_GET_READER) < 0)
                        exit(EXIT_FAILURE);

                reader_id = meta->reader.id;
                reader = data + meta->subbuf_size * reader_id;

                printf("Current reader address: %p\n", reader);

                munmap(data, data_len);
                munmap(meta, meta_len);
                close (fd);

                return 0;
        }
