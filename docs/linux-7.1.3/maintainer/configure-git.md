## 配置 Git


本章描述维护者级别的 git 配置。

拉取请求中使用的带标签分支（参见 Documentation/maintainer/pull-requests.rst）应使用开发者的公共 GPG 密钥签名。可以通过向 `git tag` 传递 `-u <key-id>` 来创建签名标签。不过，由于你**通常**会对该项目使用同一个密钥，你可以在配置中设置它，并使用 `-s`
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
你可能还希望告诉 `gpg` 使用哪个 `tty`（添加到你的 shell
```

	export GPG_TTY=$(tty)

```
