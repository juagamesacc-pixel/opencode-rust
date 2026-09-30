//! Rust port of `src/renderer/i18n/ka.ts` (opencode v1.18.30).
//!
//! Verbatim locale string table. Values and placeholders are preserved byte-for-byte.
//! Original file: `packages/desktop/src/renderer/i18n/ka.ts`

/// Mirrors `export const dict` as a verbatim flattened string table.
pub const ENTRIES: &[(&str, &str)] = &[
    ("desktop.menu.checkForUpdates", "შეამოწმეთ განახლებები..."),
    ("desktop.menu.installCli", "დააინსტალირეთ CLI..."),
    ("desktop.menu.reloadWebview", "გადატვირთვა Webview"),
    ("desktop.menu.restart", "გადატვირთვა"),
    ("desktop.dialog.chooseFolder", "აირჩიე საქაღალდე"),
    ("desktop.dialog.chooseFile", "აირჩიე ფაილი"),
    ("desktop.dialog.saveFile", "ფაილის შენახვა"),
    ("desktop.updater.checkFailed.title", "განახლების შემოწმება ვერ მოხერხდა"),
    ("desktop.updater.checkFailed.message", "განახლებების შემოწმება ვერ მოხერხდა"),
    ("desktop.updater.none.title", "განახლება არ არის ხელმისაწვდომი"),
    ("desktop.updater.none.message", "თქვენ უკვე იყენებთ OpenCode-ის უახლეს ვერსიას"),
    ("desktop.updater.downloadFailed.title", "განახლება ვერ მოხერხდა"),
    ("desktop.updater.downloadFailed.message", "განახლების ჩამოტვირთვა ვერ მოხერხდა"),
    ("desktop.updater.downloaded.title", "განახლება ჩამოიტვირთა"),
    ("desktop.updater.downloaded.prompt", "OpenCode-ის {{version}} ვერსია ჩამოტვირთულია, გსურთ მისი ინსტალაცია და ხელახლა გაშვება?"),
    ("desktop.updater.installFailed.title", "განახლება ვერ მოხერხდა"),
    ("desktop.updater.installFailed.message", "განახლების დაყენება ვერ მოხერხდა"),
    ("desktop.cli.installed.title", "CLI დაინსტალირებულია"),
    ("desktop.cli.installed.message", "CLI დაინსტალირებულია {{path}}-ზე\n\nგადატვირთეთ ტერმინალი „opencode“ ბრძანების გამოსაყენებლად."),
    ("desktop.cli.failed.title", "ინსტალაცია ვერ მოხერხდა"),
    ("desktop.cli.failed.message", "ვერ დაინსტალირდა CLI: {{error}}"),
    ("desktop.error.dev.rootNotFound", "ძირის ელემენტი ვერ მოიძებნა. დაგავიწყდათ მისი დამატება თქვენს index.html-ში? ან იქნებ id ატრიბუტი არასწორად არის დაწერილი?"),
];
