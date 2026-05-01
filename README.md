# Myman

Myman is a local-first personal assistant desktop app. The first version focuses on quick notes, tasks, schedules, Markdown knowledge entries, file descriptions, tags, and unified local search.

## Tech stack

- Tauri desktop shell
- React + TypeScript frontend
- SQLite local database
- SQLite FTS5 full-text search

## Development commands

```powershell
npm install
npm run dev
npm run build
npm run lint
npm run tauri:dev
npm run tauri:build
npm run tauri:build:no-bundle
```

`npm run dev`, `npm run build`, and `npm run lint` validate the frontend. `npm run tauri:dev`, `npm run tauri:build`, and `npm run tauri:build:no-bundle` also require a working Rust toolchain with `cargo` and `rustc` available on `PATH`.

On Windows, Tauri's MSVC target also needs Visual Studio Build Tools with the C++ workload. If a normal shell cannot find `link.exe`, run the Tauri commands from the Visual Studio developer environment:

```powershell
cmd.exe /c '"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" -arch=x64 && npm run tauri:build:no-bundle'
```

Use `npm run tauri:build:no-bundle` to validate the desktop executable without producing MSI/NSIS bundles. Full bundling may download WiX on Windows.

This repository sets `bundle.useLocalToolsDir` in `src-tauri\tauri.conf.json`, so Tauri caches Windows bundling tools under the project target directory instead of a user-global cache.

If external downloads are unstable, run build commands with the local proxy on port `7890`:

```powershell
$env:HTTP_PROXY = 'http://127.0.0.1:7890'
$env:HTTPS_PROXY = 'http://127.0.0.1:7890'
$env:ALL_PROXY = 'http://127.0.0.1:7890'
npm run tauri:build
```

The proxy is especially useful for `npm`, Rust crates, rustup, and Tauri's Windows bundling tools such as WiX and NSIS. With the proxy enabled, the full Windows build produces:

- `src-tauri\target\release\bundle\msi\Myman_0.1.0_x64_en-US.msi`
- `src-tauri\target\release\bundle\nsis\Myman_0.1.0_x64-setup.exe`

## Current implementation

- React app shell for the main dashboard, quick capture form, entity filters, and global search input.
- Tauri command layer for creating, listing, searching, and summarizing entities.
- Initial SQLite migration for the unified entity model:
  - `entities`
  - `entity_properties`
  - `entity_contents`
  - `tags`
  - `entity_tags`
  - `entity_links`
  - `file_index`
  - `reminders`
  - `revisions`
  - `search_index_jobs`
  - `search_index` FTS5 table

## Data model direction

Notes, tasks, events, knowledge entries, and file records share one entity model. Type-specific fields live in properties or specialized tables, while tags, content, links, reminders, and search indexing are shared across entity types.

Files are metadata-only in the MVP: file name, path, description, tags, type, hash, size, and modified time. PDF, Office, OCR, and semantic indexing are planned as later indexing extensions.
