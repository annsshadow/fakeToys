## uinput 妯″潡

鏈枃妗ｄ粙缁?uinput 鍐呮牳妯″潡锛屽畠鍏佽浠庣敤鎴风┖闂村垱寤哄苟妯℃嫙铏氭嫙杈撳叆璁惧锛堥€氳繃鍐欏叆 /dev/uinput 鍙戦€佷簨浠讹級锛屽苟缁欏嚭鎺ュ彛銆乴ibevdev 灏佽涓庣ず渚嬩唬鐮侊紝渚涢渶瑕佸湪鐢ㄦ埛鎬佹瀯閫犺緭鍏ヨ澶囩殑寮€鍙戣€呭弬鑰冦€?



## 绠€浠?


uinput 鏄竴涓唴鏍告ā鍧楋紝浣垮緱浠庣敤鎴风┖闂存ā鎷熻緭鍏ヨ澶囨垚涓哄彲鑳姐€傞€氳繃鍐欏叆 /dev/uinput锛堟垨
/dev/input/uinput锛夎澶囷紝涓€涓繘绋嬪彲浠ュ垱寤轰竴涓叿鏈夌壒瀹氳兘鍔涚殑铏氭嫙杈撳叆璁惧銆備竴鏃﹁铏氭嫙
璁惧琚垱寤猴紝杩涚▼灏卞彲浠ラ€氳繃瀹冨彂閫佷簨浠讹紝杩欎簺浜嬩欢灏嗚浼犻€掔粰鐢ㄦ埛绌洪棿鍜屽唴鏍稿唴娑堣垂鑰呫€?

## 鎺ュ彛


```

  linux/uinput.h

```
uinput 澶存枃浠跺畾涔変簡鐢ㄤ簬鍒涘缓銆佽缃拰閿€姣佽櫄鎷熻澶囩殑 ioctl銆?

## libevdev


libevdev 鏄?evdev 璁惧鐨勪竴涓皝瑁呭簱锛屾彁渚涗簡鍒涘缓 uinput 璁惧鍜屽彂閫佷簨浠剁殑鎺ュ彛銆俵ibevdev
姣旂洿鎺ヨ闂?uinput 鏇翠笉瀹规槗鍑洪敊锛屾柊杞欢搴旇€冭檻浣跨敤瀹冦€?

鏈夊叧 libevdev 鐨勭ず渚嬪拰鏇村淇℃伅锛?
https://www.freedesktop.org/software/libevdev/doc/latest/

## 绀轰緥


### 閿洏浜嬩欢


绗竴涓ず渚嬪睍绀哄浣曞垱寤轰竴涓柊铏氭嫙璁惧锛屼互鍙婂浣曞彂閫佷竴涓寜閿簨浠躲€備负浜嗙畝娲侊紝鎵€鏈夐粯璁ょ殑
澶存枃浠跺寘鍚拰閿欒澶勭悊绋嬪簭閮借绉婚櫎銆?


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

### 榧犳爣绉诲姩


姝ょず渚嬪睍绀哄浣曞垱寤轰竴涓〃鐜板緱鍍忕墿鐞嗛紶鏍囩殑铏氭嫙璁惧銆?


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


### uinput 鏃ф帴鍙?


鍦?uinput 鐗堟湰 5 涔嬪墠锛屾病鏈変笓鐢ㄧ殑 ioctl 鏉ヨ缃櫄鎷熻澶囥€傛敮鎸佹棫鐗堟湰 uinput 鎺ュ彛鐨?
绋嬪簭闇€瑕佸～鍏呬竴涓?uinput_user_dev 缁撴瀯浣撳苟灏嗗叾鍐欏叆 uinput 鏂囦欢鎻忚堪绗︽潵閰嶇疆鏂扮殑 uinput
璁惧銆傛柊浠ｇ爜涓嶅簲浣跨敤鏃ф帴鍙ｏ紝鑰屽簲閫氳繃 ioctl 璋冪敤涓?uinput 浜や簰锛屾垨鑰呬娇鐢?libevdev銆?


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
