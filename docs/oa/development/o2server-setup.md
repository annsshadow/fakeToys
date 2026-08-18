# o2server Development Environment Setup

## Prerequisites

| Requirement | Version | Notes |
|-------------|---------|-------|
| JDK | 11 (Java 11) | The startup scripts reference `jvm/windows_java11` and `jvm/linux_java11` |
| Maven | 3.6+ | Used to compile the 57 Maven modules |
| Git | Any recent version | For source checkout |

## Clone and Build

```bash
git clone <repository-url>
cd oa/o2server
mvn clean package -DskipTests
```

The parent POM at `oa/o2server/pom.xml` declares 57 modules. A full compile produces `o2server/console.jar`, which is the application entry point.

## Local Startup

### Windows

```cmd
cd oa/o2server
start_windows.bat
```

The script:
- Detects an `local/update/` directory and applies any pending update before starting.
- Creates required directories (`commons`, `config`, `configSample`, `local`, `localSample`, `jvm`, `servers`, `store`) if missing.
- Launches `jvm/windows_java11/bin/java` with:
  - Heap: `-Xms4g -Xmx4g`
  - Timezone: `-Duser.timezone=GMT+08`
  - Module path: `commons/module_java11`
  - Main jar: `console.jar` with `-javaagent:console.jar`

### Linux

```bash
cd oa/o2server
bash start_linux.sh
```

The Linux script sets `MALLOC_ARENA_MAX=1` and uses `setsid` to detach the process. Memory and JVM flags match the Windows script.

### Debug Mode

Use `start_windows_debug.bat` or `start_linux_debug.sh` for debug-oriented JVM flags.

## Ports

| Port | Purpose |
|------|---------|
| 80 | HTTP frontend proxy (if configured) |
| 20020 | o2server internal service port |
| 20030 | o2web frontend asset port |

## Directory Layout After First Run

```
o2server/
  console.jar          # Application entry point
  index.html           # SPA entry
  commons/             # Shared libraries and Java 11 modules
  config/              # Runtime configuration (created on first start)
  configSample/        # Reference configuration files
  local/               # Local overrides and data (created on first start)
  localSample/         # Sample local files
  jvm/                 # Bundled JRE per platform
  servers/             # Deployed server instances
  store/               # File storage root
  start_*.bat / *.sh   # Platform-specific startup scripts
```

## Common Issues

- **OutOfMemoryError**: Increase `-Xmx` in the startup script. The default is 4 GB.
- **Port conflict**: Check that ports 80, 20020, and 20030 are free before starting.
- **Time zone mismatch**: Ensure the server time zone matches `-Duser.timezone=GMT+08` or adjust accordingly.

## Next Steps

After `o2server` is running, build and deploy `o2web` (see `o2web-setup.md`).
