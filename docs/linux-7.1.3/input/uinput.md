## uinput 模块

本文档介绍 uinput 内核模块，它允许从用户空间创建并模拟虚拟输入设备（通过写入 /dev/uinput 发送事件），并给出接口、libevdev 封装与示例代码，供需要在用户态构造输入设备的开发者参考。



## 简介


uinput 是一个内核模块，使得从用户空间模拟输入设备成为可能。通过写入 /dev/uinput（或
/dev/input/uinput）设备，一个进程可以创建一个具有特定能力的虚拟输入设备。一旦该虚拟
设备被创建，进程就可以通过它发送事件，这些事件将被传递给用户空间和内核内消费者。

## 接口


```

  linux/uinput.h

```
uinput 头文件定义了用于创建、设置和销毁虚拟设备的 ioctl。

## libevdev


libevdev 是 evdev 设备的一个封装库，提供了创建 uinput 设备和发送事件的接口。libevdev
比直接访问 uinput 更不容易出错，新软件应考虑使用它。

有关 libevdev 的示例和更多信息：
https://www.freedesktop.org/software/libevdev/doc/latest/

## 示例


### 键盘事件


第一个示例展示如何创建一个新虚拟设备，以及如何发送一个按键事件。为了简洁，所有默认的
头文件包含和错误处理程序都被移除。


   #include <linux/uinput.h>

   void emit(int fd, int type, int code, int val)
   {
      struct input_event ie;

      ie.type = type;
      ie.code = code;
      ie.value = val;
      /** timestamp values below are ignored **/
      ie.time.tv_sec = 0;
      ie.time.tv_usec = 0;

      write(fd, &ie, sizeof(ie));
   }

   int main(void)
   {
      struct uinput_setup usetup;

      int fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);


      /*
       - The ioctls below will enable the device that is about to be
       - created, to pass key events, in this case the space key.
       */
      ioctl(fd, UI_SET_EVBIT, EV_KEY);
      ioctl(fd, UI_SET_KEYBIT, KEY_SPACE);

      memset(&usetup, 0, sizeof(usetup));
      usetup.id.bustype = BUS_USB;
      usetup.id.vendor = 0x1234; /** sample vendor **/
      usetup.id.product = 0x5678; /** sample product **/
      strcpy(usetup.name, "Example device");

      ioctl(fd, UI_DEV_SETUP, &usetup);
      ioctl(fd, UI_DEV_CREATE);

      /*
       - On UI_DEV_CREATE the kernel will create the device node for this
       - device. We are inserting a pause here so that userspace has time
       - to detect, initialize the new device, and can start listening to
       - the event, otherwise it will not notice the event we are about
       - to send. This pause is only needed in our example code!
       */
      sleep(1);

      /** Key press, report the event, send key release, and report again **/
      emit(fd, EV_KEY, KEY_SPACE, 1);
      emit(fd, EV_SYN, SYN_REPORT, 0);
      emit(fd, EV_KEY, KEY_SPACE, 0);
      emit(fd, EV_SYN, SYN_REPORT, 0);

      /*
       - Give userspace some time to read the events before we destroy the
       - device with UI_DEV_DESTROY.
       */
      sleep(1);

      ioctl(fd, UI_DEV_DESTROY);
      close(fd);

      return 0;
   }

### 鼠标移动


此示例展示如何创建一个表现得像物理鼠标的虚拟设备。


   #include <linux/uinput.h>

   /** emit function is identical to of the first example **/

   int main(void)
   {
      struct uinput_setup usetup;
      int i = 50;

      int fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);

      /** enable mouse button left and relative events **/
      ioctl(fd, UI_SET_EVBIT, EV_KEY);
      ioctl(fd, UI_SET_KEYBIT, BTN_LEFT);

      ioctl(fd, UI_SET_EVBIT, EV_REL);
      ioctl(fd, UI_SET_RELBIT, REL_X);
      ioctl(fd, UI_SET_RELBIT, REL_Y);

      memset(&usetup, 0, sizeof(usetup));
      usetup.id.bustype = BUS_USB;
      usetup.id.vendor = 0x1234; /** sample vendor **/
      usetup.id.product = 0x5678; /** sample product **/
      strcpy(usetup.name, "Example device");

      ioctl(fd, UI_DEV_SETUP, &usetup);
      ioctl(fd, UI_DEV_CREATE);

      /*
       - On UI_DEV_CREATE the kernel will create the device node for this
       - device. We are inserting a pause here so that userspace has time
       - to detect, initialize the new device, and can start listening to
       - the event, otherwise it will not notice the event we are about
       - to send. This pause is only needed in our example code!
       */
      sleep(1);

      /** Move the mouse diagonally, 5 units per axis **/
      while (i--) {
         emit(fd, EV_REL, REL_X, 5);
         emit(fd, EV_REL, REL_Y, 5);
         emit(fd, EV_SYN, SYN_REPORT, 0);
         usleep(15000);
      }

      /*
       - Give userspace some time to read the events before we destroy the
       - device with UI_DEV_DESTROY.
       */
      sleep(1);

      ioctl(fd, UI_DEV_DESTROY);
      close(fd);

      return 0;
   }


### uinput 旧接口


在 uinput 版本 5 之前，没有专用的 ioctl 来设置虚拟设备。支持旧版本 uinput 接口的
程序需要填充一个 uinput_user_dev 结构体并将其写入 uinput 文件描述符来配置新的 uinput
设备。新代码不应使用旧接口，而应通过 ioctl 调用与 uinput 交互，或者使用 libevdev。


   #include <linux/uinput.h>

   /** emit function is identical to of the first example **/

   int main(void)
   {
      struct uinput_user_dev uud;
      int version, rc, fd;

      fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
      rc = ioctl(fd, UI_GET_VERSION, &version);

      if (rc == 0 && version >= 5) {
         /** use UI_DEV_SETUP **/
         return 0;
      }

      /*
       - The ioctls below will enable the device that is about to be
       - created, to pass key events, in this case the space key.
       */
      ioctl(fd, UI_SET_EVBIT, EV_KEY);
      ioctl(fd, UI_SET_KEYBIT, KEY_SPACE);

      memset(&uud, 0, sizeof(uud));
      snprintf(uud.name, UINPUT_MAX_NAME_SIZE, "uinput old interface");
      write(fd, &uud, sizeof(uud));

      ioctl(fd, UI_DEV_CREATE);

      /*
       - On UI_DEV_CREATE the kernel will create the device node for this
       - device. We are inserting a pause here so that userspace has time
       - to detect, initialize the new device, and can start listening to
       - the event, otherwise it will not notice the event we are about
       - to send. This pause is only needed in our example code!
       */
      sleep(1);

      /** Key press, report the event, send key release, and report again **/
      emit(fd, EV_KEY, KEY_SPACE, 1);
      emit(fd, EV_SYN, SYN_REPORT, 0);
      emit(fd, EV_KEY, KEY_SPACE, 0);
      emit(fd, EV_SYN, SYN_REPORT, 0);

      /*
       - Give userspace some time to read the events before we destroy the
       - device with UI_DEV_DESTROY.
       */
      sleep(1);

      ioctl(fd, UI_DEV_DESTROY);

      close(fd);
      return 0;
   }
