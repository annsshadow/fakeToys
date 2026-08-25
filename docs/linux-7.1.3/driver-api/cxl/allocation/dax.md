## DAX 设备


作为 DAX 设备暴露CXL 容量可通过 mmap 直接访问。用户可能希望使用此接口
机制编写自己的用户空CXL 分配器，或管理跨多个主机的共享或持久内存区域
如果容量跨主机共享或持久化，则必须采用适当的刷新机制，除非该区域支Snoop Back-Invalidate
注意，映射（大小和基址）必须与 DAX 设备的基址对齐方式对齐，通常2MB—但也可能配置得更大
```

  #include <stdio.h>
  #include <stdlib.h>
  #include <stdint.h>
  #include <sys/mman.h>
  #include <fcntl.h>
  #include <unistd.h>

  #define DEVICE_PATH "/dev/dax0.0" // Replace DAX device path
  #define DEVICE_SIZE (4ULL * 1024 * 1024 * 1024) // 4GB

  int main() {
      int fd;
      void* mapped_addr;

      /* Open the DAX device */
      fd = open(DEVICE_PATH, O_RDWR);
      if (fd < 0) {
          perror("open");
          return -1;
      }

      /* Map the device into memory */
      mapped_addr = mmap(NULL, DEVICE_SIZE, PROT_READ | PROT_WRITE,
                         MAP_SHARED, fd, 0);
      if (mapped_addr == MAP_FAILED) {
          perror("mmap");
          close(fd);
          return -1;
      }

      printf("Mapped address: %p\n", mapped_addr);

      /* You can now access the device through the mapped address */
      uint64_t* ptr = (uint64_t*)mapped_addr;
      *ptr = 0x1234567890abcdef; // Write a value to the device
      printf("Value at address %p: 0x%016llx\n", ptr, *ptr);

      /* Clean up */
      munmap(mapped_addr, DEVICE_SIZE);
      close(fd);
      return 0;
  }

```
