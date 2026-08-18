
## 浣跨敤 Sphinx 鐢熸垚鍐呮牳鏂囨。

Linux 鍐呮牳浣跨敤 `Sphinx`_ 浠?`Documentation` 涓嬬殑 `reStructuredText`_ 鏂囦欢鐢熸垚
缇庤鐨勬枃妗ｃ€傝浠?HTML 鎴?PDF 鏍煎紡鏋勫缓鏂囨。锛屽彲浣跨敤 `make htmldocs` 鎴?`make pdfdocs`銆?鐢熸垚鐨勬枃妗ｈ鏀剧疆鍦?`Documentation/output` 涓€?
reStructuredText 鏂囦欢鍙兘鍖呭惈鐢ㄤ簬浠庢簮鏂囦欢鍖呭惈缁撴瀯鍖栨枃妗ｆ敞閲婏紙鍗?kernel-doc 娉ㄩ噴锛夌殑鎸囦护銆?閫氬父杩欎簺鐢ㄤ簬鎻忚堪浠ｇ爜鐨勫嚱鏁般€佺被鍨嬩笌璁捐銆俴ernel-doc 娉ㄩ噴鏈変竴浜涚壒娈婄殑缁撴瀯涓庢牸寮忥紝浣嗛櫎姝や箣澶?瀹冧滑涔熻瑙嗕负 reStructuredText銆?
鏈€鍚庯紝鍦?`Documentation` 鍛ㄥ洿鏁ｅ竷鐫€鏁颁互鍗冭鐨勭函鏂囨湰鏂囨。鏂囦欢銆傚叾涓竴浜涘緢鍙兘闅忕潃鏃堕棿鐨勬帹绉?琚浆鎹负 reStructuredText锛屼絾鍏朵腑鐨勫ぇ閮ㄥ垎灏嗕繚鎸佺函鏂囨湰褰㈠紡銆?

## 瀹夎 Sphinx

Documentation/ 鏂囦欢褰撳墠浣跨敤鐨?ReST 鏍囪鏃ㄥ湪浣跨敤 `Sphinx` 3.4.3 鎴栨洿楂樼増鏈瀯寤恒€?
鏈変竴涓剼鏈敤浜庢鏌?Sphinx 鐨勯渶姹傘€傛洿澶氱粏鑺傝鍙傞槄 sphinx-pre-install銆?
澶у鏁板彂琛岀増閮介檮甯?Sphinx锛屼絾鍏跺伐鍏烽摼杈冧负鑴嗗急锛屽崌绾у畠鎴栨満鍣ㄤ笂鐨勬煇浜涘叾瀹?Python 鍖?瀵艰嚧鏂囨。鏋勫缓澶辫触鐨勬儏鍐靛苟涓嶅皯瑙併€?
閬垮厤杩欎竴鐐圭殑涓€绉嶆柟娉曟槸浣跨敤涓庡彂琛岀増鎵€闄勫甫鐗堟湰涓嶅悓鐨勭増鏈€備负姝わ紝寤鸿鍦ㄨ櫄鎷熺幆澧冧腑瀹夎 Sphinx锛?浣跨敤 `virtualenv-3` 鎴?`virtualenv`锛屽叿浣撳彇鍐充簬浣犵殑鍙戣鐗堝浣曟墦鍖?Python 3銆?
鎬讳箣锛屽鏋滀綘鎯冲畨瑁呮渶鏂扮増鏈殑 Sphinx锛屽彲浠ワ細

```
       $ virtualenv sphinx_latest
       $ . sphinx_latest/bin/activate
       (sphinx_latest) $ pip install -r Documentation/sphinx/requirements.txt

```
杩愯 `. sphinx_latest/bin/activate` 鍚庯紝鎻愮ず绗︿細鏀瑰彉锛屼互鎸囩ず浣犳鍦ㄤ娇鐢ㄦ柊鐨勭幆澧冦€?濡傛灉浣犳墦寮€涓€涓柊鐨?shell锛岄渶瑕佸湪鏋勫缓鏂囨。涔嬪墠閲嶆柊杩愯璇ュ懡浠や互鍐嶆杩涘叆铏氭嫙鐜銆?
### 鍥惧儚杈撳嚭

鍐呮牳鏂囨。鏋勫缓绯荤粺鍖呭惈涓€涓鐞?GraphViz 涓?SVG 涓ょ鏍煎紡鍥惧儚鐨勬墿灞曪紙鍙傝 sphinx_kfigure锛夈€?
瑕佷娇鍏跺伐浣滐紝浣犻渶瑕佸畨瑁?GraphViz 鍜?ImageMagick 涓や釜鍖呫€傚鏋滄湭瀹夎杩欎簺鍖咃紝鏋勫缓绯荤粺浠嶄細
鏋勫缓鏂囨。锛屼絾涓嶄細鍦ㄨ緭鍑轰腑鍖呭惈浠讳綍鍥惧儚銆?
### PDF 涓?LaTeX 鏋勫缓

姝ょ被鏋勫缓褰撳墠浠呮敮鎸?Sphinx 2.4 鍙婃洿楂樼増鏈€?
瀵逛簬 PDF 鍜?LaTeX 杈撳嚭锛屼綘杩橀渶瑕?`XeLaTeX` 3.14159265 鐗堟湰銆?
鏍规嵁鍙戣鐗堢殑涓嶅悓锛屼綘鍙兘杩橀渶瑕佸畨瑁呬竴绯诲垪 `texlive` 鍖咃紝浠ユ彁渚?`XeLaTeX` 宸ヤ綔鎵€闇€鐨?鏈€灏忓姛鑳介泦銆?
### HTML 涓殑鏁板琛ㄨ揪寮?
涓€浜?ReST 椤甸潰鍖呭惈鏁板琛ㄨ揪寮忋€傜敱浜?Sphinx 鐨勫伐浣滄柟寮忥紝杩欎簺琛ㄨ揪寮忎娇鐢?LaTeX 璁版硶涔﹀啓銆?Sphinx 鏈変袱绉嶉€夐」鏉ュ湪 html 杈撳嚭涓覆鏌撴暟瀛﹁〃杈惧紡銆備竴绉嶆槸鍚嶄负 `imgmath`_ 鐨勬墿灞曪紝瀹冨皢
鏁板琛ㄨ揪寮忚浆鎹负鍥惧儚骞跺祵鍏ュ埌 html 椤甸潰涓€傚彟涓€绉嶆槸鍚嶄负 `mathjax`_ 鐨勬墿灞曪紝瀹冨皢鏁板娓叉煋
濮旀墭缁欐敮鎸?JavaScript 鐨?Web 娴忚鍣ㄣ€傚墠鑰呮槸 6.1 涔嬪墠鍐呮牳鏂囨。鐨勫敮涓€閫夐」锛屽畠闇€瑕佺浉褰撳鐨?texlive 鍖咃紝鍏朵腑鍖呮嫭 amsfonts 鍜?amsmath 绛夈€?
鑷唴鏍?6.1 鐗堟湰璧凤紝鍖呭惈鏁板琛ㄨ揪寮忕殑 html 椤甸潰鍙互鍦ㄤ笉瀹夎浠讳綍 texlive 鍖呯殑鎯呭喌涓嬫瀯寤恒€?鏇村淇℃伅璇峰弬闃?`Choice of Math Renderer`_銆?

### 妫€鏌?Sphinx 渚濊禆椤?
鏈変竴涓剼鏈細鑷姩妫€鏌?Sphinx 渚濊禆椤广€傚鏋滃畠鑳借瘑鍒綘鐨勫彂琛岀増锛屽畠杩樹細缁欏嚭
瀹夎鎻愮ず锛?
```
	$ ./tools/docs/sphinx-pre-install
	Checking if the needed tools for Fedora release 26 (Twenty Six) are available
	Warning: better to also install "texlive-luatex85".
	You should run:

		sudo dnf install -y texlive-luatex85
		/usr/bin/virtualenv sphinx_2.4.4
		. sphinx_2.4.4/bin/activate
		pip install -r Documentation/sphinx/requirements.txt

	Can't build as 1 mandatory dependency is missing at ./tools/docs/sphinx-pre-install line 468.

```
榛樿鎯呭喌涓嬶紝瀹冧細妫€鏌?html 鍜?PDF 鐨勫叏閮ㄩ渶姹傦紝鍖呮嫭鍥惧儚銆佹暟瀛﹁〃杈惧紡鍜?LaTeX 鏋勫缓鐨勯渶姹傦紝
骞跺亣瀹氬皢浣跨敤 Python 铏氭嫙鐜銆傜敤浜?html 鏋勫缓鐨勯渶姹傝鍋囧畾涓哄己鍒剁殑锛涘叾瀹冨垯涓哄彲閫夌殑銆?
瀹冩敮鎸佷袱涓彲閫夊弬鏁帮細

`--no-pdf`
	绂佺敤瀵?PDF 鐨勬鏌ワ紱

`--no-virtualenv`
	浣跨敤鎿嶄綔绯荤粺鎵撳寘鐨?Sphinx锛岃€岄潪 Python 铏氭嫙鐜銆?
### 瀹夎 Sphinx 鏈€灏忕増鏈?
鍦ㄦ洿鏀?Sphinx 鏋勫缓绯荤粺鏃讹紝纭繚鏈€灏忕増鏈粛鍙楁敮鎸佸緢閲嶈銆傚浠婏紝鍦ㄧ幇浠ｅ彂琛岀増涓婅繖鏍峰仛姝ｅ彉寰?鎰堝彂鍥伴毦锛屽洜涓烘棤娉曞湪 Python 3.13 鍙婁互涓婄増鏈腑瀹夎銆?
鍙互浣跨敤 Documentation/process/changes.rst 涓畾涔夌殑鏈€浣庡彈鏀寔 Python 鐗堟湰杩涜娴嬭瘯锛?鏂规硶涓哄垱寤猴細

```
	/usr/bin/python3.9 -m venv sphinx_min
	. sphinx_min/bin/activate
	pip install -r Documentation/sphinx/min_requirements.txt

```
鍙互浣跨敤浠ヤ笅鏂瑰紡鍋氭洿鍏ㄩ潰鐨勬祴璇曪細

	tools/docs/test_doc_build.py

璇ヨ剼鏈负姣忎釜鍙楁敮鎸佺殑鐗堟湰鍒涘缓涓€涓?Python venv锛屽苟鍙€夊湴涓轰竴绯诲垪 Sphinx 鐗堟湰鏋勫缓鏂囨。銆?

## 鏋勫缓 Sphinx 鏂囨。

鐢熸垚鏂囨。鐨勯€氬父鏂瑰紡鏄繍琛?`make htmldocs` 鎴?`make pdfdocs`銆傝繕鏈夊叾瀹冨彲鐢ㄦ牸寮忥細
璇峰弬闃?`make help` 鐨勬枃妗ｉ儴鍒嗐€傜敓鎴愮殑鏂囨。琚斁缃湪 `Documentation/output` 涓嬬壒瀹氫簬鏍煎紡鐨?瀛愮洰褰曚腑銆?
瑕佺敓鎴愭枃妗ｏ紝鏄剧劧蹇呴』瀹夎 Sphinx锛坄sphinx-build`锛夈€傚浜?PDF 杈撳嚭锛屼綘杩橀渶瑕佹潵鑷?ImageMagick
鐨?`XeLaTeX` 鍜?`convert(1)` (https://www.imagemagick.org)銆俓 [#ink]_ 杩欎簺閮借兘骞挎硾鑾峰彇锛?骞剁敱鍙戣鐗堟墦鍖呫€?
瑕佸悜 Sphinx 浼犻€掗澶栭€夐」锛屽彲浠ヤ娇鐢?`SPHINXOPTS` make 鍙橀噺銆備緥濡傦紝浣跨敤
`make SPHINXOPTS=-v htmldocs` 鍙幏寰楁洿璇︾粏鐨勮緭鍑恒€?
涔熷彲浠ラ€氳繃浣跨敤 `DOCS_CSS` make 鍙橀噺浼犲叆棰濆鐨?DOCS_CSS 瑕嗙洊鏂囦欢锛屼互鑷畾涔?html 甯冨眬銆?
榛樿鎯呭喌涓嬶紝鏋勫缓 HTML 鏂囨。浣跨敤 "Alabaster" 涓婚锛涜涓婚闅?Sphinx 涓€鍚屾彁渚涳紝鏃犻渶鍗曠嫭瀹夎銆?Sphinx 涓婚鍙互閫氳繃浣跨敤 `DOCS_THEME` make 鍙橀噺鏉ヨ鐩栥€?

   鏈変簺浜哄彲鑳芥洿鍠滄瀵?html 杈撳嚭浣跨敤 RTD 涓婚銆傛牴鎹?Sphinx 鐗堟湰鐨勪笉鍚岋紝瀹冨簲浣跨敤
   `pip install sphinx_rtd_theme` 鍗曠嫭瀹夎銆?
杩樻湁鍙︿竴涓?make 鍙橀噺 `SPHINXDIRS`锛屽湪娴嬭瘯鏋勫缓鏂囨。瀛愰泦鏃跺緢鏈夌敤銆備緥濡傦紝浣犲彲浠ラ€氳繃杩愯
`make SPHINXDIRS=doc-guide htmldocs` 鏉ユ瀯寤?`Documentation/doc-guide` 涓嬬殑鏂囨。銆?`make help` 鐨勬枃妗ｉ儴鍒嗕細鏄剧ず浣犲彲浠ユ寚瀹氱殑瀛愮洰褰曞垪琛ㄣ€?
瑕佺Щ闄ょ敓鎴愮殑鏂囨。锛岃繍琛?`make cleandocs`銆?
	 鍚屾牱涔熻兘鏀瑰杽宓屽叆 PDF 鏂囨。鐨勫浘鍍忚川閲忥紝灏ゅ叾鏄浜庡唴鏍?5.18 鍙婃洿楂樼増鏈€?
### 鏁板娓叉煋鍣ㄧ殑閫夋嫨

鑷唴鏍?6.1 鐗堟湰璧凤紝mathjax 浣滀负 html 杈撳嚭鏁板娓叉煋鍣ㄧ殑鍥為€€鏂规宸ヤ綔銆俓 [#sph1_8]_

鏁板娓叉煋鍣ㄦ牴鎹彲鐢ㄥ懡浠ら€夋嫨锛屽涓嬫墍绀猴細


    ============= ================= ============
    Math renderer Required commands Image format
    ============= ================= ============
    imgmath       latex, dvipng     PNG (raster)
    mathjax
    ============= ================= ============


鍙互閫氳繃浠ヤ笅鏂瑰紡璁剧疆鐜鍙橀噺 `SPHINX_IMGMATH` 鏉ヨ鐩栬閫夋嫨锛?

    ====================== ========
    Setting                Renderer
    ====================== ========
    `SPHINX_IMGMATH=yes` imgmath
    `SPHINX_IMGMATH=no`  mathjax
    ====================== ========



## 缂栧啓鏂囨。

娣诲姞鏂版枃妗ｅ彲浠ュ緢绠€鍗曪細

1. 鍦?`Documentation` 涓嬬殑鏌愬娣诲姞涓€涓柊 `.rst` 鏂囦欢銆?2. 浠?`Documentation/index.rst` 涓殑 Sphinx 涓?`TOC tree`_ 寮曠敤瀹冦€?

杩欏浜庣畝鍗曟枃妗ｏ紙灏卞儚浣犵幇鍦ㄦ鍦ㄩ槄璇荤殑杩欎唤锛夐€氬父宸茬粡瓒冲锛屼絾瀵逛簬杈冨ぇ鐨勬枃妗ｏ紝寤鸿鍒涘缓涓€涓?瀛愮洰褰曪紙鎴栦娇鐢ㄥ凡鏈夌殑瀛愮洰褰曪級銆備緥濡傦紝鍥惧舰瀛愮郴缁熸枃妗ｄ綅浜?`Documentation/gpu`锛屾媶鍒嗕负鑻ュ共
`.rst` 鏂囦欢锛屽苟鎷ユ湁鑷韩鍗曠嫭鐨?`index.rst`锛堝甫鏈夎嚜宸辩殑 `toctree`锛夛紝鐢变富绱㈠紩寮曠敤銆?
鍏充簬浣犲彲浠ョ敤 Sphinx 鍜?reStructuredText 鍋氫粈涔堬紝璇峰弬闃?`Sphinx`_ 鍜?`reStructuredText`_
鐨勬枃妗ｃ€傜壒鍒槸锛孲phinx `reStructuredText Primer`_ 鏄紑濮嬪涔?reStructuredText 鐨勫ソ鍘诲銆?涔熸湁涓€浜?`Sphinx specific markup constructs`_銆?

### 鍐呮牳鏂囨。鐨勭壒瀹氬噯鍒?
浠ヤ笅鏄拡瀵瑰唴鏍告枃妗ｇ殑涓€浜涚壒瀹氬噯鍒欙細

- 璇蜂笉瑕佽繃搴︿娇鐢?reStructuredText 鏍囪銆備繚鎸佺畝鍗曘€傚湪澶у鏁版儏鍐典笅锛屾枃妗ｅ簲涓虹函鏂囨湰锛?  鍙渶鍦ㄦ牸寮忎笂淇濇寔瓒冲鐨勪竴鑷存€э紝浠ヤ究鑳借浆鎹负鍏跺畠鏍煎紡銆?
- 鍦ㄥ皢鐜版湁鏂囨。杞崲涓?reStructuredText 鏃讹紝璇峰敖閲忎繚鎸佹牸寮忔敼鍔ㄦ渶灏忋€?
- 鍦ㄨ浆鎹㈡枃妗ｆ椂锛屼篃瑕佹洿鏂板唴瀹癸紝鑰屼笉浠呬粎鏄牸寮忋€?
- 璇烽伒寰互涓嬫爣棰樿楗扮鐨勯『搴忥細

```

       ==============
       Document title
       ==============

  2. ``=`` for chapters::

       Chapters
       ========

  3. ``-`` for sections::

       Section
       -------

  4. ``~`` for subsections::

       Subsection
       ~~~~~~~~~~

  Although RST doesn't mandate a specific order ("Rather than imposing a fixed
  number and order of section title adornment styles, the order enforced will be
  the order as encountered."), having the higher levels the same overall makes
  it easier to follow the documents.

```

- 瀵逛簬鎻掑叆鍥哄畾瀹藉害鐨勬枃鏈潡锛堢敤浜庝唬鐮佺ず渚嬨€佺敤渚嬬ず渚嬬瓑锛夛紝瀵逛笉鐪熸鍙楃泭浜庤娉曢珮浜殑鍐呭
  锛堝挨鍏舵槸鐭墖娈碉級浣跨敤 `::`銆傚鍙楃泭浜庨珮浜殑杈冮暱浠ｇ爜鍧椾娇鐢?`.. code-block:: <language>`銆?  瀵逛簬宓屽叆鏂囨湰涓殑鐭唬鐮佺墖娈碉紝浣跨敤 \`\`銆?

### C 鍩?
**Sphinx C 鍩?*锛堝悕涓?c锛夐€傜敤浜?C API 鐨勬枃妗ｃ€備緥濡備竴涓嚱鏁板師鍨嬶細


    .. c:function:: int ioctl( int fd, int request )

kernel-doc 鐨?C 鍩熸湁涓€浜涢檮鍔犵壒鎬с€備緥濡傦紝浣犲彲浠ョ敤 `open` 鎴?`ioctl` 杩欐牱鐨勯€氱敤鍚嶇О
**閲嶅懡鍚?*涓€涓嚱鏁扮殑寮曠敤鍚嶏細


     .. c:function:: int ioctl( int fd, int request )
        :name: VIDIOC_LOG_STATUS

func-name锛堜緥濡?ioctl锛変繚鐣欏湪杈撳嚭涓紝浣?ref-name 浠?`ioctl` 鏇存敼涓?`VIDIOC_LOG_STATUS`銆?璇ュ嚱鏁扮殑绱㈠紩鏉＄洰涔熼殢涔嬫洿鏀逛负 `VIDIOC_LOG_STATUS`銆?
璇锋敞鎰忥紝鏃犻渶浣跨敤 `c:func:` 鏉ョ敓鎴愬埌鍑芥暟鏂囨。鐨勪氦鍙夊紩鐢ㄣ€傜敱浜庢煇浜?Sphinx 鎵╁睍鐨勯瓟娉曪紝
濡傛灉缁欏畾鍑芥暟鍚嶅瓨鍦ㄧ储寮曟潯鐩紝鏂囨。鏋勫缓绯荤粺浼氳嚜鍔ㄥ皢鍒?`function()` 鐨勫紩鐢ㄨ浆鎹负浜ゅ弶寮曠敤銆?濡傛灉浣犲湪鍐呮牳鏂囨。涓湅鍒?`c:func:` 鐨勪娇鐢紝璇烽殢鎰忓皢鍏剁Щ闄ゃ€?
### 琛ㄦ牸

reStructuredText 涓鸿〃鏍艰娉曟彁渚涗簡鑻ュ共閫夐」銆傚唴鏍歌〃鏍奸鏍煎€惧悜浜庝娇鐢?*绠€鍗曡〃鏍?*璇硶鎴?**缃戞牸琛ㄦ牸**璇硶銆傛洿澶氱粏鑺傝鍙傞槄 `reStructuredText user reference for table syntax`_銆?
   https://docutils.sourceforge.io/docs/user/rst/quickref.html#tables

#### 鍒楄〃琛ㄦ牸

list-table 鏍煎紡瀵逛簬涓嶆槗鐢ㄩ€氬父鐨?Sphinx ASCII 瀛楃鐢绘牸寮忔帓甯冪殑琛ㄦ牸寰堟湁鐢ㄣ€備笉杩囷紝瀵逛簬
绾枃鏈枃妗ｇ殑璇昏€呰€岃█锛岃繖浜涙牸寮忓嚑涔庢棤娉曠悊瑙ｏ紝鍦ㄦ病鏈夊厖鍒嗙悊鐢辩殑鎯呭喌涓嬪簲閬垮厤浣跨敤銆?
`flat-table` 鏄竴涓被浼间簬 `list-table` 鐨勪袱绾у垪琛紝甯︽湁涓€浜涢檮鍔犵壒鎬э細

- column-span锛氶€氳繃瑙掕壊 `cspan`锛屼竴涓崟鍏冩牸鍙墿灞曞埌棰濆鐨勫垪

- row-span锛氶€氳繃瑙掕壊 `rspan`锛屼竴涓崟鍏冩牸鍙墿灞曞埌棰濆鐨勮

- 鑷姩灏嗚〃鏍艰鏈€鍙充晶鐨勫崟鍏冩牸璺ㄨ繃璇ヨ〃鏍艰鍙充晶缂哄け鐨勫崟鍏冩牸銆傞€氳繃閫夐」 `:fill-cells:` 鍙皢姝?  琛屼负浠?*鑷姩璺ㄥ垪锛坅uto span锛?*鏇存敼涓?*鑷姩濉厖锛坅uto fill锛?*锛屽嵆鑷姩鎻掑叆锛堢┖锛夊崟鍏冩牸锛?  鑰岄潪璺ㄦ帴鏈€鍚庝竴涓崟鍏冩牸銆?
options锛?
- `:header-rows:`   [int] 琛ㄥご琛屾暟
- `:stub-columns:`  [int] 瀛樻牴鍒楁暟
- `:widths:`        [[int] [int] ... ] 鍒楀
- `:fill-cells:`    鑷姩鎻掑叆缂哄け鍗曞厓鏍硷紝鑰岄潪鑷姩璺ㄦ帴缂哄け鍗曞厓鏍?
roles锛?
- `:cspan:` [int] 棰濆鍒楁暟锛?*morecols**锛?- `:rspan:` [int] 棰濆琛屾暟锛?*morerows**锛?
涓嬮潰鐨勭ず渚嬪睍绀轰簡濡備綍浣跨敤姝ゆ爣璁般€傚垎绾у垪琛ㄧ殑绗竴绾ф槸 **table-row**銆傚湪 **table-row** 涓?鍙厑璁镐竴绉嶆爣璁帮紝鍗宠 **table-row** 涓崟鍏冩牸鐨勫垪琛ㄣ€備緥澶栨槸 **comments**锛?`..` 锛夊拰
**targets**锛堜緥濡傚 ``last row <last row>`` 鐨勫紩鐢?/ :ref:`last row <last row>`锛夈€?

   .. flat-table:: table title
      :widths: 2 1 1 3

      - - head col 1
        - head col 2
        - head col 3
        - head col 4

      - - row 1
        - field 1.1
        - field 1.2 with autospan

      - - row 2
        - field 2.1
        - `1` `1` field 2.2 - 3.3

      - .. _`last row`:

        - row 3

娓叉煋涓猴細

   .. flat-table:: table title
      :widths: 2 1 1 3

      - - head col 1
        - head col 2
        - head col 3
        - head col 4

      - - row 1
        - field 1.1
        - field 1.2 with autospan

      - - row 2
        - field 2.1
        - `1` `1` field 2.2 - 3.3

      - .. _`last row`:

        - row 3

### 浜ゅ弶寮曠敤

浠庝竴涓枃妗ｉ〉浜ゅ弶寮曠敤鍒板彟涓€涓枃妗ｉ〉锛屽彧闇€鍐欏嚭鏂囨。鏂囦欢鐨勮矾寰勫嵆鍙紝鏃犻渶鐗规畩璇硶銆?璺緞鍙互鏄粷瀵硅矾寰勬垨鐩稿璺緞銆傚浜庣粷瀵硅矾寰勶紝浠?"Documentation/" 寮€澶淬€備緥濡傦紝瑕佷氦鍙夊紩鐢?鍒版湰椤碉紝鏍规嵁褰撳墠鏂囨。鐨勭洰褰曪紙娉ㄦ剰锛?
```
    See Documentation/doc-guide/sphinx.rst. This always works.
    Take a look at sphinx.rst, which is at this same directory.
    Read ../sphinx.rst, which is one directory above.

```
濡傛灉浣犲笇鏈涢摼鎺ュ叿鏈変笉鍚屼簬鏂囨。璺緞鐨勬覆鏌撴枃鏈紝鍙互锛?
```
    See :doc:`my custom link text for document sphinx <sphinx>`.

```
瀵逛簬澶у鏁扮敤渚嬶紝鍓嶈€呮洿鍙楅潚鐫愶紝鍥犱负瀹冩洿骞插噣锛屾洿閫傚悎闃呰婧愭枃浠剁殑浜恒€傚鏋滀綘閬囧埌娌℃湁甯︽潵浠讳綍
浠峰€肩殑 `:doc:` 鐢ㄦ硶锛岃闅忔剰灏嗗叾杞崲涓轰粎鏂囨。璺緞銆?
鍏充簬浜ゅ弶寮曠敤鍒?kernel-doc 鍑芥暟鎴栫被鍨嬬殑淇℃伅锛岃鍙傞槄 Documentation/doc-guide/kernel-doc.rst銆?
#### 寮曠敤鎻愪氦

瀵?git 鎻愪氦鐨勫紩鐢ㄤ細鑷姩鍙樹负瓒呴摼鎺ワ紝鍙瀹冧滑鏄細

```
    commit 72bf4f1767f0
    commit 72bf4f1767f0 ("net: do not leave an empty skb in write queue")

```

## 鍥捐〃涓庡浘鍍?
濡傛灉浣犳兂娣诲姞鍥惧儚锛屽簲浣跨敤 `kernel-figure` 鍜?`kernel-image` 鎸囦护銆備緥濡傦紝瑕佹彃鍏ヤ竴涓?鍙缉鏀剧殑鍥撅細

```
    .. kernel-figure::  svg_image.svg
       :alt:    simple SVG image

       SVG image example

```

   :alt:    simple SVG image

   SVG image example

鍐呮牳鍥撅紙鍜屽浘鍍忥級鎸囦护鏀寔 **DOT** 鏍煎紡鐨勬枃浠讹紝鍙傝

- DOT: http://graphviz.org/pdf/dotguide.pdf
- Graphviz: http://www.graphviz.org/content/dot-language

```
  .. kernel-figure::  hello.dot
     :alt:    hello world

     DOT's hello world example

```

   :alt:    hello world

   DOT's hello world example

宓屽叆鐨?**render** 鏍囪锛堟垨璇█锛夛紝濡?Graphviz 鐨?**DOT**锛岀敱锛?
```
  .. kernel-render:: DOT
     :alt: foobar digraph
     :caption: Embedded **DOT** (Graphviz) code

     digraph foo {
      "bar" -> "baz";
     }

```
鍏舵覆鏌撴柟寮忓彇鍐充簬鎵€瀹夎鐨勫伐鍏枫€傚鏋滃畨瑁呬簡 Graphviz锛屼綘浼氱湅鍒扮煝閲忓浘鍍忋€傚惁鍒欙紝鍘熷鏍囪浼氫綔涓?**literal-block**锛坔ello_dot_render锛夋彃鍏ャ€?

   :alt: foobar digraph
   :caption: Embedded **DOT** (Graphviz) code

   digraph foo {
      "bar" -> "baz";
   }

**render** 鎸囦护鎷ユ湁 **figure** 鎸囦护宸茬煡鐨勬墍鏈夐€夐」锛屽鍔犻€夐」 `caption`銆傚鏋?`caption` 鏈夊€硷紝
鍒欐彃鍏ヤ竴涓?**figure** 鑺傜偣锛涘惁鍒欐彃鍏ヤ竴涓?**image** 鑺傜偣銆傚鏋滀綘鎯宠寮曠敤瀹冿紙hello_svg_render锛夛紝
涔熼兘闇€瑕佷竴涓?`caption`銆?
```
  .. kernel-render:: SVG
     :caption: Embedded **SVG** markup
     :alt: so-nw-arrow

     <?xml version="1.0" encoding="UTF-8"?>
     <svg xmlns="http://www.w3.org/2000/svg" version="1.1" ...>
        ...
     </svg>

```

   :caption: Embedded **SVG** markup
   :alt: so-nw-arrow

   <?xml version="1.0" encoding="UTF-8"?>
   <svg xmlns="http://www.w3.org/2000/svg"
     version="1.1" baseProfile="full" width="70px" height="40px" viewBox="0 0 700 400">
   <line x1="180" y1="370" x2="500" y2="50" stroke="black" stroke-width="15px"/>
   <polygon points="585 0 525 25 585 50" transform="rotate(135 525 25)"/>
   </svg>
