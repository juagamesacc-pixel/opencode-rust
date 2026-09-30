# Docs mapping for package.json / vite.config.ts / index.html

This file records the entry points and alias mapping for packages/app, per Lane D spec (docs-mapping only, no .rs).

## package.json (v1.18.30)
- name: @opencode-ai/app
- version: 1.18.30
- type: module
- exports:
  - "." -> ./src/index.ts (entry)
  - "./desktop-menu" -> ./src/desktop-menu.ts
  - "./i18n/desktop-native" -> ./src/i18n/desktop-native.ts
  - "./updater" -> ./src/updater.ts
  - "./wsl/types" -> ./src/wsl/types.ts
  - "./vite" -> ./vite.js
  - "./index.css" -> ./src/index.css
- scripts: typecheck -> tsgo -b, dev/start -> vite, build -> vite build, test -> bun test, etc.
- dependencies: solid-js, @solidjs/router, @tanstack/solid-query, ghostty-web, shiki, @pierre/trees, etc. (full list in source package.json)

## vite.config.ts
- defineConfig with plugins: [desktopPlugin (from ./vite), sentryVitePlugin (conditional on SENTRY_AUTH_TOKEN)]
- server: host 0.0.0.0, allowedHosts true, port 3000
- build: target esnext, sourcemap true

## index.html
- lang en, title OpenCode, viewport includes interactive-widget=resizes-content
- favicon: /favicon-96x96-v3.png, /favicon-v3.svg, /favicon-v3.ico, apple-touch-icon
- manifest: /site.webmanifest
- theme-color #fafafa
- scripts: /oc-theme-preload.js (id oc-theme-preload-script), module /src/entry.tsx
- root div id="root" class="flex flex-col h-dvh ..."
- body class antialiased overscroll-none text-12-regular overflow-hidden bg-v2-background-bg-deep

## Alias mapping
- @/ -> crate root / packages/app/src (vite resolve alias "@/")
- In Rust crate, @/ maps to crate root `crate::` (mirrored via `crate::` paths)

No Rust code generated for these files per plan — docs mapping only.
