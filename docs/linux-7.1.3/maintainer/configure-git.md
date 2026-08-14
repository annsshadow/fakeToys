## 閰嶇疆 Git


鏈珷鎻忚堪缁存姢鑰呯骇鍒殑 git 閰嶇疆銆?

鎷夊彇璇锋眰涓娇鐢ㄧ殑甯︽爣绛惧垎鏀紙鍙傝 Documentation/maintainer/pull-requests.rst锛夊簲浣跨敤寮€鍙戣€呯殑鍏叡 GPG 瀵嗛挜绛惧悕銆傚彲浠ラ€氳繃鍚?`git tag` 浼犻€?`-u <key-id>` 鏉ュ垱寤虹鍚嶆爣绛俱€備笉杩囷紝鐢变簬浣?*閫氬父**浼氬璇ラ」鐩娇鐢ㄥ悓涓€涓瘑閽ワ紝浣犲彲浠ュ湪閰嶇疆涓缃畠锛屽苟浣跨敤 `-s`
```

	git config user.signingkey "keyname"

```
```

	[user]
		name = Jane Developer
		email = jd@domain.org
		signingkey = jd@domain.org

```
```

	[gpg]
		program = /path/to/gpg2

```
浣犲彲鑳借繕甯屾湜鍛婅瘔 `gpg` 浣跨敤鍝釜 `tty`锛堟坊鍔犲埌浣犵殑 shell
```

	export GPG_TTY=$(tty)

```
