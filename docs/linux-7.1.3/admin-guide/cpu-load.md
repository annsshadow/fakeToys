## CPU 璐熻浇


Linux 閫氳繃 `/proc/stat` 鍜?`/proc/uptime` 瀵煎嚭鍚勭淇℃伅锛岀敤鎴锋€佸伐鍏凤紙濡?top(1)锛夊埄鐢ㄨ繖浜涗俊鎭潵璁＄畻

```
    $ iostat
    Linux 2.6.18.3-exp (linmac)     02/20/2007

    avg-cpu:  %user   %nice %system %iowait  %steal   %idle
              10.01    0.00    2.92    5.44    0.00   81.63

    ...

```

姝ゅ绯荤粺璁や负锛屽湪榛樿閲囨牱鍛ㄦ湡鍐咃紝绯荤粺鏈?10.01% 鐨勬椂闂村湪鐢ㄦ埛绌洪棿鎵ц宸ヤ綔锛?.92% 鍦ㄥ唴鏍镐腑锛屾暣浣撶┖闂叉椂闂翠负 81.63%銆?
鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝`/proc/stat` 鍙嶆槧鐨勬儏鍐典笌鐜板疄鐩稿綋鎺ヨ繎锛屼絾鐢变簬鍐呮牳閲囬泦姝ゆ暟鎹殑鏃舵満涓庢柟寮忔墍闄愶紝鏈夋椂瀹冨畬鍏ㄤ笉鍙俊銆?
閭ｄ箞杩欎簺淇℃伅鏄浣曢噰闆嗙殑鍛紵姣忓綋瀹氭椂鍣ㄤ腑鏂Е鍙戞椂锛屽唴鏍镐細鏌ョ湅褰撳墠姝ｅ湪杩愯鐨勪换鍔＄被鍨嬶紝骞惰涓庤浠诲姟绫诲瀷/鐘舵€佸搴旂殑璁℃暟鍣ㄥ姞涓€銆傞棶棰樺湪浜庯紝鍦ㄤ袱娆″畾鏃跺櫒涓柇涔嬮棿锛岀郴缁熷彲鑳藉湪澶氱鐘舵€佷箣闂村垏鎹簡澶氭锛屼絾璁℃暟鍣ㄥ彧閽堝鏈€鍚庝竴绉嶇姸鎬佽繘琛屼簡绱姞銆?

### 绀轰緥


璁炬兂涓€涓郴缁燂紝鍏朵腑鏈変竴涓换鍔″懆鏈熸€у湴娑堣€?CPU 鍛ㄦ湡

```

     time line between two timer interrupts
    |--------------------------------------|
     ^                                    ^
     |_ something begins working          |
                                          |_ something goes to sleep
                                         (only to be awaken quite soon)

```

鍦ㄤ笂杩版儏褰笅锛屾牴鎹?`/proc/stat` 鐨勫垽鏂紝绯荤粺璐熻浇涓?0%锛堝洜涓哄畾鏃跺櫒涓柇鎬绘槸鍙戠敓鍦ㄧ郴缁熸墽琛?idle 澶勭悊绋嬪簭鏃讹級锛屼絾瀹為檯涓婅礋杞芥洿鎺ヨ繎 99%銆?
浜轰滑鍙互璁炬兂鏇村姝ょ被鍐呮牳琛屼负瀵艰嚧鍋忓樊鐨勬儏褰?
```


	/* gcc -o hog smallhog.c */
	#include <time.h>
	#include <limits.h>
	#include <signal.h>
	#include <sys/time.h>
	#define HIST 10

	static volatile sig_atomic_t stop;

	static void sighandler(int signr)
	{
		(void) signr;
		stop = 1;
	}

	static unsigned long hog (unsigned long niters)
	{
		stop = 0;
		while (!stop && --niters);
		return niters;
	}

	int main (void)
	{
		int i;
		struct itimerval it = {
			.it_interval = { .tv_sec = 0, .tv_usec = 1 },
			.it_value    = { .tv_sec = 0, .tv_usec = 1 } };
		sigset_t set;
		unsigned long v[HIST];
		double tmp = 0.0;
		unsigned long n;
		signal(SIGALRM, &sighandler);
		setitimer(ITIMER_REAL, &it, NULL);

		hog (ULONG_MAX);
		for (i = 0; i < HIST; ++i) v[i] = ULONG_MAX - hog(ULONG_MAX);
		for (i = 0; i < HIST; ++i) tmp += v[i];
		tmp /= HIST;
		n = tmp - (tmp / 3.0);

		sigemptyset(&set);
		sigaddset(&set, SIGALRM);

		for (;;) {
			hog(n);
			sigwait(&set, &i);
		}
		return 0;
	}


```

### 鍙傝€?
- https://lore.kernel.org/r/loom.20070212T063225-663@post.gmane.org
- Documentation/filesystems/proc.rst (1.8)


### 鑷磋阿


Con Kolivas, Pavel Machek
