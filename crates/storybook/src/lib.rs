// source: packages/storybook — Storybook config + app mocks @ v1.18.30
//! Config/assets copied VERBATIM to assets/ (28 files, mirroring the source
//! tree). Manifest + parity tests.

pub const ASSET_COUNT: usize = 28;

/// source: key config files present verbatim
pub const KEY_FILES: &[&str] = &[
    ".storybook/main.ts",
    ".storybook/manager.ts",
    ".storybook/preview.tsx",
    ".storybook/theme-tool.ts",
    ".storybook/playground-css-plugin.ts",
    "package.json",
    "tsconfig.json",
    "sst-env.d.ts",
    "debug-storybook.log",
    ".gitignore",
];

/// source: mock modules under .storybook/mocks (app), verbatim paths
pub const MOCK_FILES: &[&str] = &[
    ".storybook/mocks/app/components/dialog-select-model-unpaid.tsx",
    ".storybook/mocks/app/components/dialog-select-model.tsx",
    ".storybook/mocks/app/context/command.ts",
    ".storybook/mocks/app/context/comments.ts",
    ".storybook/mocks/app/context/file.ts",
    ".storybook/mocks/app/context/global-sync.ts",
    ".storybook/mocks/app/context/language.ts",
    ".storybook/mocks/app/context/layout.ts",
    ".storybook/mocks/app/context/local.ts",
    ".storybook/mocks/app/context/permission.ts",
    ".storybook/mocks/app/context/platform.ts",
    ".storybook/mocks/app/context/prompt.ts",
    ".storybook/mocks/app/context/sdk.ts",
    ".storybook/mocks/app/context/server-sdk.ts",
    ".storybook/mocks/app/context/server-sync.ts",
    ".storybook/mocks/app/context/sync.ts",
    ".storybook/mocks/app/hooks/use-providers.ts",
    ".storybook/mocks/solid-router.tsx",
];
