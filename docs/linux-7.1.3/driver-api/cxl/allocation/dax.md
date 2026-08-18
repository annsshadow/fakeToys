## DAX 璁惧


浣滀负 DAX 璁惧鏆撮湶鐨?CXL 瀹归噺鍙€氳繃 mmap 鐩存帴璁块棶銆傜敤鎴峰彲鑳藉笇鏈涗娇鐢ㄦ鎺ュ彛
鏈哄埗缂栧啓鑷繁鐨勭敤鎴风┖闂?CXL 鍒嗛厤鍣紝鎴栫鐞嗚法澶氫釜涓绘満鐨勫叡浜垨鎸佷箙鍐呭瓨鍖哄煙銆?
濡傛灉瀹归噺璺ㄤ富鏈哄叡浜垨鎸佷箙鍖栵紝鍒欏繀椤婚噰鐢ㄩ€傚綋鐨勫埛鏂版満鍒讹紝闄ら潪璇ュ尯鍩熸敮鎸?Snoop Back-Invalidate銆?
娉ㄦ剰锛屾槧灏勶紙澶у皬鍜屽熀鍧€锛夊繀椤讳笌 DAX 璁惧鐨勫熀鍧€瀵归綈鏂瑰紡瀵归綈锛岄€氬父涓?2MB鈥斺€?浣嗕篃鍙兘閰嶇疆寰楁洿澶с€?
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
