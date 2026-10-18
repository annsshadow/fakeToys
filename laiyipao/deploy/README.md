# 来一炮 · 测试环境部署手册（laiyipao）

> 参考 `businessMatchingPlatformSvr` 的 `.bat`/`.ps1` 模式，把「后端 + 运营后台(admin) + 小程序 H5(miniapp)」三端部署到测试服务器。
> 本目录下的 `.local.keys`、`bin/`、`remote-config.env` 已被 `.gitignore` 拦截，**不含任何密钥**，可放心提交本目录下的 `.conf`/`.service`/`README.md`。

## 0. 占位符说明（开源仓库脱敏）

本文件里的 <YOUR_TEST_DOMAIN> 是占位符。真实测试域名（本团队实际值）保存在**不入库**的 laiyipao/deploy/.local.keys 的 DOMAIN 行。
部署前：DOMAIN=，再把文件里（及构建命令里）的占位符替换为该值。
仓库里**不含**任何真实域名 / IP / 口令。

## 1. 三端部署地址与账号

| 端 | 访问地址 | 账号 / 密码 |
|---|---|---|
| 运营后台 (admin) | `http://<YOUR_TEST_DOMAIN>/laiyipao-admin/` | `admin` / 见本地 `.local.keys` 的 `ADMIN_PASS` 行（不入库） |
| 小程序 H5 (miniapp) | `http://<YOUR_TEST_DOMAIN>/laiyipao-h5/` | 游客进入，无需账号 |
| 后端 API | `https://<YOUR_TEST_DOMAIN>/laiyipao-api/` | — |

- 后端 systemd 服务名：`laiyipao.service`（监听 `0.0.0.0:9020`）
- nginx location 片段：`/etc/nginx/conf.d/laiyipao/10-test.conf`（挂进 `ccy.conf` 的测试域名 server 块（`server_name` 含 <YOUR_TEST_DOMAIN>））
- 远端产物目录：`/usr/local/laiyipao/{bin,admin,h5,etc,logs}`
- 数据库：PostgreSQL `laiyipao`（专用账号 `laiyipao_app`，口令在远端 `config.env` 与本地 `.local.keys`，均不入库）

## 2. 密钥与本地覆盖（不入库）

`deploy/.gitignore` 拦截：

| 文件 | 内容 |
|---|---|
| `.local.keys` | `JWT_SECRET` / `ADMIN_PASS` / `DB_DSN`（含数据库口令） |
| `bin/` | 交叉编译的二进制 |
| `remote-config.env` | 上传到服务器的 `config.env` 副本（含数据库口令） |

远端 `/usr/local/laiyipao/etc/config.env`（`chmod 600`）含 `DATABASE_URL`、`JWT_SECRET`、`BOOTSTRAP_ADMIN_PASS`，只存在于服务器，不进仓库。

## 3. 复现部署的步骤（密钥从本地 `.local.keys` 读取，不在本文件里）

```powershell
# 1) 编译后端（Linux amd64，CGO 关闭）
cd laiyipao/server
$env:CGO_ENABLED="0"; $env:GOOS="linux"; $env:GOARCH="amd64"
go build -ldflags "-s -w" -o ..\deploy\bin\lyp-server-amd64 .\cmd\api
go build -ldflags "-s -w" -o ..\deploy\bin\lyp-migrate .\cmd\migrate
go build -ldflags "-s -w" -o ..\deploy\bin\lyp-seed    .\cmd\seed

# 2) 编译前端（烘焙 API 地址）
$env:VITE_API_BASE="https://<YOUR_TEST_DOMAIN>/laiyipao-api/api/v1"
cd ..\admin   ; npm run build:only
cd ..\miniapp ; npm run build:h5

# 3) 上传 + 安装（scp 序列见 git 历史）
#    - scp 二进制 → /usr/local/laiyipao/bin/
#    - scp admin/dist → /usr/local/laiyipao/admin
#    - scp miniapp/dist/build/h5 → /usr/local/laiyipao/h5
#    - scp deploy/systemd/laiyipao.service → /etc/systemd/system/
#    - scp deploy/nginx/laiyipao-test.conf → /etc/nginx/conf.d/laiyipao/10-test.conf
#    - ccy.conf 里加 include（有备份 ccy.conf.bak.20261016laiyipao）
# 4) systemctl restart laiyipao && nginx -t && systemctl reload nginx
```

## 4. 回滚

```bash
# 后端
systemctl stop laiyipao
mv /usr/local/laiyipao/bin/lyp-server-amd64{,.old} && systemctl start laiyipao
# 前端
mv /usr/local/laiyipao/admin{,.old}   # 或 h5
# nginx
mv /etc/nginx/conf.d/laiyipao/10-test.conf{,.bak} && systemctl reload nginx
# 数据库
psql -d laiyipao -f <备份>   # 见 docs/laiyipao/OPERATIONS.md §6
```

## 5. 安全备注

- `APP_ENV=prod`：远端 `config.env` 已显式覆盖 `JWT_SECRET` 与 `BOOTSTRAP_ADMIN_PASS`，`config.go` 的 prod 守卫不再 panic。
- `JWT_SECRET`（64 字节随机）与 `ADMIN_PASS` 存放在本地 `laiyipao/deploy/.local.keys`，已被 `.gitignore` 拦截，**不进仓库**。
- 数据库口令（`laiyipao_app` 用户）只存在于远端 `config.env`（600 权限）与本地 `.local.keys` 的 `DB_DSN` 行，**不进仓库**。
- 查看具体口令：服务器上 `cat /usr/local/laiyipao/etc/config.env`（600），或本地读 `laiyipao/deploy/.local.keys`。
