# o2web Development Environment Setup

## Prerequisites

| Requirement | Version | Notes |
|-------------|---------|-------|
| Node.js | 10.x (inferred from gulp 4 and dependency era) | No `.nvmrc` found; use a Node 10-compatible version |
| npm | Bundled with Node.js | |
| Gulp CLI | 2.x | Install globally: `npm install -g gulp-cli` |

## Clone and Install

```bash
cd oa/o2web
npm install
```

## Build

```bash
npm run build
```

This runs `gulp`, which reads `gulpapps.js` to determine which components to compile and produces the bundled output in `dest/`.

## Component Structure

Each `x_component_*` directory under `source/` follows this layout:

```
source/x_component_<Name>/
  Main.js              # Component entry point (MWF class definition)
  lp/                   # Language packs (zh-cn/, en/, etc.)
  $Main/                # Compiled or generated assets
  applications.json     # Component application registry (if present)
```

## Key Build Concepts

- `gulpapps.js` defines which components are included in the build and their source paths.
- `gulpfile.js` orchestrates concatenation, minification, sourcemaps, and optional FTP/SFTP upload.
- `gulpconfig.js` (optional) defines upload targets for deployment.

## Relationship to o2server

`o2web` builds into `dest/`, which is then served by `o2server` on port 20030 (or deployed into `o2server/servers/webServer/`). During development you can point a reverse proxy or directly open `dest/` assets against a running `o2server`.

## Common Issues

- **Gulp command not found**: Ensure `gulp-cli` is installed globally, or use `npx gulp`.
- **Build fails on Windows**: Some gulp plugins assume Unix-style paths; use WSL or the Linux build environment if you encounter path issues.

## Next Steps

After building `o2web`, deploy the `dest/` output alongside a running `o2server` (see `deployment/windows.md` or `deployment/linux.md`).
