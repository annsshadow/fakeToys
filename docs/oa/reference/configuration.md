# Configuration Reference

The reference configuration files live under `oa/o2server/configSample/`. A `manifest.json` in the same directory indexes 32 files by name. This reference documents the files present in the current repository snapshot.

## Files Present in This Snapshot

| File | Purpose |
|------|---------|
| `manifest.json` | Index of all config-sample files |
| `web.json` | Web-layer global parameters |
| `externalStorageSources.json` | File storage backends (protocol, host, port, credentials, weight) |
| `messageSendRule.js` | Message dispatch rules (JavaScript) |

## Files Referenced by Manifest but Not Present

The `manifest.json` lists the following files that are not included in this repository snapshot. They are part of the full platform distribution and should be treated as additional configuration surfaces when deploying a complete server:

- `general.json` — Platform system settings
- `processPlatform.json` — Process engine settings
- `portal.json` — Portal configuration
- `person.json` — Person/organization settings
- `query.json` — Query engine settings
- `components.json` — Component registry
- `collect.json` — Collect (survey) settings
- `appStyle.json` — Application style settings
- `workTime.json` — Work time / attendance rules
- `meeting.json` — Open meeting settings
- `messages.json` — Message channel settings
- `andFx.json` — AndFx integration
- `dingding.json` — DingTalk integration
- `zhengwuDingding.json` — Government DingTalk integration
- `qiyeweixin.json` — Enterprise WeChat integration
- `weLink.json` — Huawei WeLink integration
- `mPweixin.json` — MP WeChat integration
- `exmail.json` — Exmail integration
- `jpushConfig.json` — JPush integration
- `externalDataSources.json` — Default database connection pool
- `externalDataSources_mysql.json` — MySQL connection pool
- `externalDataSources_oracle.json` — Oracle connection pool
- `externalDataSources_sqlserver.json` — SQL Server connection pool
- `externalDataSources_postgresql.json` — PostgreSQL connection pool
- `externalDataSources_db2.json` — DB2 connection pool
- `externalDataSources_dm.json` — DM (DaMeng) connection pool
- `externalDataSources_informix.json` — Informix connection pool
- `externalDataSources_kingbase.json` — KingBase connection pool
- `dumpRestoreData.json` — Data dump/restore settings

## Configuration Conventions

- JSON config files are loaded at server startup and hot-reloaded where supported.
- Place overrides in `config/`; do not edit files under `configSample/` directly.
- Use the per-database `externalDataSources_*.json` files to register non-default database vendors.

## Key Fields in Present Files

### `web.json`

Currently a placeholder in this snapshot. In production, it carries web-layer globals such as theme, domain, and cross-origin settings.

### `externalStorageSources.json`

Top-level keys are domain names (`file`, `processPlatform`, `mind`, etc.). Each domain maps to an array of storage endpoints:

```json
{
  "file": [
    {
      "protocol": "webdav",
      "host": "127.0.0.1",
      "port": 8080,
      "username": "admin",
      "password": "admin",
      "enable": true,
      "weight": 100,
      "deepPath": false
    }
  ]
}
```

Fields:
- `protocol` — Storage protocol (`webdav`, `ftp`, `sftp`, `local`, etc.)
- `host` / `port` — Server address
- `username` / `password` — Credentials when the protocol requires authentication
- `enable` — Whether this endpoint is active
- `weight` — Load-balancing weight among enabled endpoints
- `deepPath` — Whether to use deep-path directory sharding

### `messageSendRule.js`

JavaScript evaluated by the message dispatch engine. Used to implement conditional routing and filtering rules for outgoing messages.
