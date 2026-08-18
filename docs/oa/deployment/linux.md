# Linux Deployment

## Prerequisites

- Linux server (CentOS, Ubuntu, Debian, or compatible)
- glibc-based distribution
- Java 11 is bundled; no system JDK required.

## Distribution Layout

After extracting the release tarball:

```
o2server/
  console.jar
  index.html
  start_linux.sh
  start_linux_debug.sh
  start_linux_min.sh
  stop_linux.sh
  restart_linux.sh
  jvm/
    linux_java11/
      bin/java
  commons/
  config/
  configSample/
  local/
  servers/
  store/
```

## Required Ports

| Port | Direction | Purpose |
|------|-----------|---------|
| 80 | Inbound | HTTP (optional reverse proxy) |
| 20020 | Inbound | o2server service port |
| 20030 | Inbound | o2web frontend assets |

Use `firewall-cmd` or `iptables` to open these ports.

```bash
# firewalld example
firewall-cmd --permanent --add-port=20020/tcp
firewall-cmd --permanent --add-port=20030/tcp
firewall-cmd --reload
```

## Permissions

```bash
chmod +x start_linux.sh
chmod +x stop_linux.sh
chmod +x restart_linux.sh
```

## Startup

```bash
cd /opt/o2server   # or your install path
bash start_linux.sh
```

The script:
- Creates required directories if missing.
- Detects `local/update/o2server/` and applies any pending update.
- Launches `jvm/linux_java11/bin/java` with `setsid` to detach from the terminal.

To stop:

```bash
bash stop_linux.sh
```

To restart:

```bash
bash restart_linux.sh
```

## Logs

Logs are under `servers/` and `local/`. Use `tail` to monitor:

```bash
tail -f servers/o2server/server.log
tail -f local/logs/*.log
```

## Configuration

Copy reference files from `configSample/` to `config/` and edit as needed. At minimum, configure database connections and storage backends. See `reference/configuration.md`.

## Systemd Service (Optional)

Create `/etc/systemd/system/o2server.service`:

```ini
[Unit]
Description=O2OA Server
After=network.target

[Service]
Type=forking
WorkingDirectory=/opt/o2server
ExecStart=/opt/o2server/start_linux.sh
ExecStop=/opt/o2server/stop_linux.sh
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Then:

```bash
systemctl daemon-reload
systemctl enable o2server
systemctl start o2server
```

## Update / Hot-Deploy

Place the update package under `local/update/o2server/` and restart. The startup script applies the update and exits; restart again to run the updated version.
