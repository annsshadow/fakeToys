# Windows Deployment

## Prerequisites

- Windows Server 2012 or later (or Windows 10/11 for testing)
- Java 11 runtime is **not** required separately; the distribution bundles `jvm/windows_java11/`.

## Distribution Layout

After unzipping the release package:

```
o2server/
  console.jar
  index.html
  start_windows.bat
  start_windows_debug.bat
  stop_windows.bat
  restart_windows.bat
  jvm/
    windows_java11/
      bin/java.exe
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
| 3306 (example) | Outbound | Database (if using MySQL) |

Open these ports in Windows Firewall if the default policy blocks them.

## Startup

```cmd
start_windows.bat
```

To stop:

```cmd
stop_windows.bat
```

To restart:

```cmd
restart_windows.bat
```

## Logs

Logs are written under `servers/` and `local/`. Check `servers/o2server/server.log` and `local/logs/` for runtime output.

## Configuration

Copy reference files from `configSample/` to `config/` and edit as needed. At minimum, configure:

- `web.json` — web layer global parameters
- `externalDataSources*.json` — database connection pools
- `externalStorageSources.json` — file storage backends

See `reference/configuration.md` for per-file details.

## Update / Hot-Deploy

Place an update package into `local/update/o2server/` before starting. The startup script detects this directory and applies the update automatically, then exits so you can restart.

## Running as a Service

Use `start_windows.bat` with the Windows Task Scheduler or NSSM to run `o2server` as a background service. The script is designed to run detached; no console window is required after launch.
