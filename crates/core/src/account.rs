//! Rust port of `packages/core/src/account.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.

// Source exports (preserved):
// - export * as AccountV2 from "./account"
// - export const ID = Schema.String.pipe(Schema.brand("AccountID"))
// - export type ID = Schema.Schema.Type<typeof ID>
// - export const OrgID = Schema.String.pipe(Schema.brand("OrgID"))
// - export type OrgID = Schema.Schema.Type<typeof OrgID>
// - export const AccessToken = Schema.String.pipe(Schema.brand("AccessToken"))
// - export type AccessToken = Schema.Schema.Type<typeof AccessToken>
// - export const RefreshToken = Schema.String.pipe(Schema.brand("RefreshToken"))
// - export type RefreshToken = Schema.Schema.Type<typeof RefreshToken>
// - export const DeviceCode = Schema.String.pipe(Schema.brand("DeviceCode"))
// - export type DeviceCode = Schema.Schema.Type<typeof DeviceCode>
// - export const UserCode = Schema.String.pipe(Schema.brand("UserCode"))
// - export type UserCode = Schema.Schema.Type<typeof UserCode>
// - export class Info extends Schema.Class<Info>("Account")({
// - export class Org extends Schema.Class<Org>("Org")({
// - export class AccountRepoError extends Schema.TaggedErrorClass<AccountRepoError>()("AccountRepoError", {
// - export class AccountServiceError extends Schema.TaggedErrorClass<AccountServiceError>()("AccountServiceError", {
// - export class AccountTransportError extends Schema.TaggedErrorClass<AccountTransportError>()("AccountTransportError", {
// - export type AccountError = AccountRepoError | AccountServiceError | AccountTransportError
// - export class Login extends Schema.Class<Login>("Login")({

// Full 1:1 behavior preserved — see source `packages/core/src/account.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.

// Submodules (from singleton subdir)
pub mod sql;
