
## Tracefs ring-buffer 鍐呭瓨鏄犲皠


:Author: Vincent Donnefort <vdonnefort@google.com>

## 姒傝堪


Tracefs ring-buffer 鐨勫唴瀛樻槧灏勬彁渚涗簡涓€绉嶉珮鏁堢殑鏁版嵁娴佷紶杈撴柟娉曪紝鍥犱负鏃犻渶杩涜鍐呭瓨鎷疯礉銆傛槧灏勪簡
ring-buffer 鐨勫簲鐢ㄧ▼搴忛殢鍗虫垚涓鸿 ring-buffer 鐨勪竴涓秷璐硅€咃紝鏂瑰紡绫讳技浜?trace_pipe銆?
## 鍐呭瓨鏄犲皠璁剧疆


璇ユ槧灏勯€氳繃 mmap() trace_pipe_raw 鎺ュ彛鏉ュ伐浣溿€?
鏄犲皠鐨勭涓€涓郴缁熼〉鍖呭惈 ring-buffer 鐨勭粺璁′俊鎭笌鎻忚堪锛岃绉颁负 meta-page锛堝厓椤碉級銆傚厓椤垫渶閲嶈鐨?瀛楁涔嬩竴鏄?reader銆傚畠鍖呭惈鍙鏄犲皠鑰呭畨鍏ㄨ鍙栫殑瀛愮紦鍐插尯锛坰ub-buffer锛塈D锛堝弬瑙?ring-buffer-design.rst锛夈€?
鍏冮〉涔嬪悗鏄墍鏈夊瓙缂撳啿鍖猴紝鎸?ID 鍗囧簭鎺掑垪銆傚洜姝ゅ彲浠ヨ交鏉惧湴鐭ラ亾 reader 鍦ㄦ槧灏勪腑鐨勮捣濮嬩綅缃細


        reader_id = meta->reader->id;
        reader_offset = meta->meta_page_size + reader_id * meta->subbuf_size;

褰撳簲鐢ㄧ▼搴忓鐞嗗畬褰撳墠 reader 鍚庯紝瀹冨彲浠ヤ娇鐢?trace_pipe_raw 鐨?ioctl() TRACE_MMAP_IOCTL_GET_READER
鑾峰彇涓€涓柊鐨?reader銆傝 ioctl 鍚屾椂浼氭洿鏂板厓椤靛瓧娈点€?
## 闄愬埗


褰撴煇涓?Tracefs ring-buffer 涓婂瓨鍦ㄦ槧灏勬椂锛屾棤娉曞鍏惰皟鏁村ぇ灏忥紙鏃犺鏄澶ф暣涓?ring-buffer 杩樻槸
姣忎釜 subbuf锛夈€備篃鏃犳硶浣跨敤 snapshot锛堝揩鐓э級锛屽苟涓斾細瀵艰嚧 splice 鎷疯礉 ring buffer 鏁版嵁锛岃€岄潪
浣跨敤鏉ヨ嚜 ring buffer 鐨勬棤鎷疯礉浜ゆ崲銆?
鍏佽骞跺彂璇昏€咃紙鏃犺鏄彟涓€涓槧灏勮 ring-buffer 鐨勫簲鐢ㄧ▼搴忥紝杩樻槸鍐呮牳閫氳繃 trace_pipe锛夛紝浣嗕笉鎺ㄨ崘銆?瀹冧滑灏嗙珵浜夎 ring-buffer锛岃緭鍑虹殑缁撴灉鏄笉鍙娴嬬殑锛屽氨鍍?trace_pipe 涓婄殑骞跺彂璇昏€呬竴鏍枫€?
## 绀轰緥



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
