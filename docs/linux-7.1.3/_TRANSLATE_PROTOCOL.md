# Linux 鍐呮牳鏂囨。涓枃鍖?鈥?瀛愪换鍔℃墽琛屽崗璁?
浣犳槸鏂囨。缈昏瘧瀛愪换鍔℃墽琛屽櫒銆傜洰鏍囷細鎶婃寚瀹氱殑鑻辨枃 Markdown 鏂囨。缈昏瘧涓虹畝浣撲腑鏂囷紝**鍘熷湴瑕嗙洊**鍘熸枃浠躲€傜粷涓嶆敼鍔?`translations/` 瀛愮洰褰曘€?
> 鏈换鍔℃槸瀵?`docs/绯荤粺鏂囨。/` 涓嬭嚜鍔ㄧ敓鎴愮殑 Markdown 鍋?*鏈湴鍘熷湴缈昏瘧**锛屼笉鏄唴鏍告簮鐮佽ˉ涓侊紱**涓嶈鎵ц git commit / checkpatch / SPDX 绛夊唴鏍告彁浜ゆ祦绋?*锛屼篃涓嶈鏀瑰姩 `translations/` 瀛愭爲銆?
## 0. 杈撳叆
- 浣犱細琚憡鐭ヤ綘鐨?**worker 缂栧彿 i**锛?-based锛夈€?- 璇诲彇 `D:/WORKSPACE/linux-7.1.3/docs/绯荤粺鏂囨。/.translate_workers.json`锛屽叾椤跺眰涓?`{"workers": [[璺緞,...], ...]}`锛涘彇 `workers[i]`锛堜竴涓粷瀵硅矾寰勫垪琛級浣滀负鏈换鍔¤澶勭悊鐨勬枃浠堕泦鍚堛€?
## 1. 鏈涓庝繚鐣欒鍒欙紙涓ユ牸閬靛畧锛?- 鍐呮牳鏍囧噯璇戞硶锛堜粎鐢ㄤ簬鑷劧璇█ prose锛夛細
  kernel=鍐呮牳, module=妯″潡, scheduler=璋冨害鍣? process=杩涚▼, thread=绾跨▼,
  spinlock=鑷棆閿? mutex=浜掓枼浣? semaphore=淇″彿閲? page=椤? interrupt=涓柇,
  exception=寮傚父, syscall=绯荤粺璋冪敤, filesystem=鏂囦欢绯荤粺, file system=鏂囦欢绯荤粺,
  device=璁惧, driver=椹卞姩, subsystem=瀛愮郴缁? buffer=缂撳啿鍖? cache=缂撳瓨,
  register=瀵勫瓨鍣? descriptor=鎻忚堪绗? context=涓婁笅鏂? atomic=鍘熷瓙, barrier=灞忛殰,
  scheduler=璋冨害鍣? priority=浼樺厛绾? thread=绾跨▼, queue=闃熷垪, stack=鏍? heap=鍫?
  kernel space=鍐呮牳绌洪棿, user space=鐢ㄦ埛绌洪棿, virtual=铏氭嫙, physical=鐗╃悊,
  mapping=鏄犲皠, allocation=鍒嗛厤, release=閲婃斁, lock=閿? unlock=瑙ｉ攣銆?- **蹇呴』鍘熸牱淇濈暀锛岀粷涓嶇炕璇戞垨鏀瑰啓**锛?  * 鍥存爮浠ｇ爜鍧楋紙```...```锛夊唴鐨勫叏閮ㄥ唴瀹癸紝鍚缉杩涖€佺鍙枫€佹敞閲娿€?  * 琛屽唴浠ｇ爜锛坄...`锛夈€?  * 閾炬帴鐩爣 / URL / 閭 / 鍥剧墖璺緞 / 閿氱偣銆?  * C 鏍囪瘑绗︺€佸嚱鏁板悕銆佸畯銆佺粨鏋勪綋鍚嶃€佸彉閲忓悕銆佸懡浠よ銆佸懡浠よ鍙傛暟銆佹枃浠惰矾寰勩€?  * 缂╁啓锛歊CU, CFS, DT, ACPI, PCI, PCIe, USB, API, ABI, CPU, MMU, IRQ, DMA,
    POSIX, Kconfig, Makefile, YAML, JSON, HTML, XML, UUID, ID, IO, MM, NUMA,
    SMP, TLS, TCP, UDP, IP, VLAN, NIC, SoC, RISC-V, ARM, x86 绛夈€?  * 鏁板瓧銆佸崟浣嶃€佺増鏈彿銆佹棩鏈熴€?- 閾炬帴鏂囧瓧鑻ヤ负涓枃鍙鍒欒瘧锛屼絾淇濈暀 `[鏂囧瓧](URL)` 缁撴瀯锛?  `[the scheduler](url)` 鈫?`[璋冨害鍣╙(url)`銆?
## 2. 閫愭枃浠舵祦绋?瀵规瘡涓枃浠?F锛?1. 璇诲彇 F 鍏ㄦ枃銆?2. **鏄惁闇€瑕佺炕璇?*锛堥伩鍏嶈浼ゅ凡璇?绾粨鏋勬枃浠讹級锛?   鍘绘帀鎵€鏈?``` 浠ｇ爜鍧椾笌琛屽唴浠ｇ爜锛屽啀鍘绘帀鏍囬琛屼笌閾炬帴琛屽悗锛屽鈥滃墿浣?prose鈥濆垽瀹氾細
   - 鑻ュ墿浣?prose 鍑犱箮涓虹┖锛屾垨鍓╀綑鑻辨枃璇?< 8 涓?鈫?**璺宠繃**锛堢粨鏋?绱㈠紩/鍗犱綅鏂囦欢锛夛紝浠嶅仛绗?6 姝ユ牎楠屻€?   - 鑻ヤ腑鏂囨瘮渚?鈮?10% **涓?* 鍓╀綑鑻辨枃璇?< 20 涓?鈫?瑙嗕负宸插畬鏁寸炕璇戯紝**璺宠繃**锛堜粛鍋氱 6 姝ユ牎楠岋級銆?   - 鍏朵綑鎯呭喌锛堝惈鈥滃崐璇戔€濇枃浠讹細涓枃姣斾緥 3%鈥?0%锛屾垨浠嶅惈杈冨鑻辨枃璇?杩炵画鑻辨枃娈佃惤锛夆啋 **蹇呴』缈昏瘧**锛岃ˉ榻愭湭瀹屾垚閮ㄥ垎銆?3. **鍐欐枃浠朵竴寰嬪師瀛愭柟寮?*锛氬厛鍐欏悓鍚?`.tmp`锛屽啀 `os.replace(tmp, F)`銆傜粷涓嶇洿鎺ヨ鐩栫洰鏍囷紝闃叉涓柇鎴柇銆?4. 缈昏瘧鏃跺彧瀵硅嚜鐒惰瑷€ prose 缈昏瘧锛屼弗鏍间繚鐣欑 1 鑺?鍘熸牱淇濈暀"鐨勫唴瀹癸紱淇濇寔 Markdown 缁撴瀯锛堟爣棰樺眰绾с€佸垪琛ㄣ€佽〃鏍煎垎闅旂 `|---|`銆侀摼鎺ヨ娉曘€佽剼娉級涓嶅彉銆?5. 鏂囦欢澶у皬鍐崇瓥锛?   - F 鈮?80KB锛氭暣浣撹銆佹暣浣撹瘧銆佸啓 `.tmp` 鍚?`os.replace`銆?   - F > 80KB锛氫娇鐢ㄧ 3 鑺?鍒嗗潡缈昏瘧"銆?6. **鍐欏悗鑷**锛氶噸璇?F锛岀粺璁?``` 鍥存爮鏄惁鎴愬锛堝伓鏁帮級锛涘"搴旇瘧"鏂囦欢妫€鏌?prose 涓枃姣斾緥 鈮?3%銆傝嫢寮傚父锛堝洿鏍忓鏁般€佷腑鏂囨瘮渚嬭繃浣庛€佹枃浠舵槑鏄炬埅鏂級锛岄噸鏂扮炕璇戝苟閲嶅啓璇ユ枃浠躲€?
## 3. 鍒嗗潡缈昏瘧锛堜粎鐢ㄤ簬 >80KB 鏂囦欢锛岄伩鍏嶅崟娆¤緭鍑鸿秴闀匡級
> **閲嶈锛堝凡鐭ョ己闄蜂慨姝ｏ級**锛?*绂佹鎸夋爣棰樿鍒囧垎**鈥斺€斾細瀵艰嚧鍧椾贡搴忋€侀噸澶嶃€佹紡璇戙€備竴寰嬫敼鐢ㄤ笅鏂?*鎸夎杈圭晫鍒囧垎**锛屽潡缂栧彿椤哄簭鍗虫枃妗ｉ『搴忋€?
a) 鐢ㄤ互涓?python锛圔ash 杩愯锛夋妸 F 鎸?*琛岃竟鐣?*鍒囧垎涓?鈮?40KB 鐨勫潡锛屽瓨鍏?`F.chunks/`锛堝懡鍚?`000.txt,001.txt...`锛夛紝椤哄簭涓庢枃妗ｄ竴鑷达細
```python
import os, sys
F=sys.argv[1]; outdir=F+".chunks"; os.makedirs(outdir,exist_ok=True)
lines=open(F,encoding='utf-8',errors='ignore').read().split('\n')
chunks=[]; buf=[]; depth=0
for ln in lines:
    if ln.lstrip().startswith('```'):
        depth = 1 - depth
    buf.append(ln)
    size = sum(len(x.encode('utf-8'))+1 for x in buf)
    # 浠呭湪"澶勪簬浠ｇ爜鍧椾箣澶?涓?閬囧埌绌鸿"鏃跺垏鍒嗭紝閬垮厤鍒囨柇浠ｇ爜鍧楁垨娈佃惤
    if size > 40000 and depth == 0 and ln.strip() == '':
        chunks.append('\n'.join(buf)); buf=[]
if buf:
    chunks.append('\n'.join(buf))
# 鍏滃簳锛氳嫢鍥犲崟涓秴澶т唬鐮佸潡瀵艰嚧鏌愬潡浠?>40KB锛屾寜琛屽己鍒跺垏鍒?# 锛堜唬鐮佸潡閫愯鍘熸牱淇濈暀锛岀炕璇戞椂浠嶄笉璇戜唬鐮侊紱鏈€缁堣嚜妫€浼氭牎楠屽洿鏍忓伓鏁帮級
final=[]
for c in chunks:
    if len(c.encode('utf-8')) <= 40000:
        final.append(c); continue
    for ln in c.split('\n'):
        if final and sum(len(x.encode('utf-8'))+1 for x in final[-1].split('\n')) > 40000:
            final.append(ln)
        else:
            final[-1] = (final[-1] + '\n' + ln) if final else ln
for idx,c in enumerate(final):
    open(os.path.join(outdir,'%03d.txt'%idx),'w',encoding='utf-8').write(c)
print("chunks:",len(final))
```
b) 渚濇鎸夌紪鍙峰姣忎釜鍧楁枃浠?`000.txt,001.txt...`锛歚Read` 瀹?鈫?**浠呯炕璇戣嚜鐒惰瑷€ prose**锛堝悓鏍蜂繚鐣欎唬鐮?鏍囪瘑绗?閾炬帴锛夆啋 **杩藉姞**鍐欏叆 `F.tmp`锛堜弗鏍兼寜缂栧彿椤哄簭杩藉啓锛屼笉鍙贡搴忋€佷笉鍙噸澶嶏級銆?c) 鍏ㄩ儴鍧楀畬鎴愬悗 `os.replace(F.tmp, F)`銆?d) 鑷鍚岀 2.6 鑺傦紱纭鏈€缁?``` 鍥存爮鍋舵暟銆乸rose 涓枃姣斾緥 鈮?3%銆?
## 4. 瀹屾垚鎶ュ憡锛堝繀椤伙級
瀹屾垚鍚庯紝**鍐欏叆** `D:/WORKSPACE/linux-7.1.3/docs/绯荤粺鏂囨。/.translate_results/w{i}.json`锛屽唴瀹癸細
```json
{"worker": i, "ok": <宸插畬鎴愭暟>, "skip": <璺宠繃鏁?, "fail": <澶辫触鏁?,
 "files": {"<鐩稿璺緞>": "DONE|SKIP|FAIL:<鍘熷洜>"}}
```
鍚屾椂鍦ㄤ綘鐨勫洖澶嶄腑鍙繑鍥炰竴鍙ユ€荤粨锛堝锛歚worker i 瀹屾垚锛欴ONE x, SKIP y, FAIL z`锛夛紝**涓嶈杩斿洖璇戞枃姝ｆ枃**銆?
## 5. 绾緥
- 涓嶆敼鍔?`translations/` 涓嬩换浣曟枃浠躲€?- 涓嶈噯閫犲唴瀹癸紱閬囧埌涓嶇‘瀹氱殑鏈锛屼繚鐣欒嫳鏂囧師鏂囷紙浠呭湪 prose 涓紝涓斿敖閲忕敤閫氳璇戞硶锛夈€?- 鑻ユ煇鏂囦欢璇诲彇/鍐欏叆澶辫触锛岃鍏?FAIL锛岀户缁笅涓€涓紝涓嶈涓柇鏁翠釜浠诲姟銆?- 涓嶆墽琛?git 鎻愪氦锛涙湰浠诲姟鏄湰鍦版枃妗ｅ師鍦扮炕璇戙€?