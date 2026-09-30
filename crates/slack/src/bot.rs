//! Rust port of `packages/slack/src/index.ts` (opencode v1.18.30).
//!
//! Source 145 lines. Exports: Slack bolt app wired to opencode SDK.
//!
//! 1:1 notes:
//! - `new App({ token: SLACK_BOT_TOKEN, signingSecret: SLACK_SIGNING_SECRET, socketMode:true, appToken: SLACK_APP_TOKEN })` verbatim env keys.
//! - Bot config logs `🔧 Bot configuration:` + 3 `present:!!env` + `🚀 Starting opencode server...` + `createOpencode({port:0})` + `✅ Opencode server ready`.
//! - `sessions = Map<string,{client,server,sessionId,channel,thread}>` key `${channel}-${thread}`.
//! - `opencode.client.event.subscribe()` stream `for await (event)` → `message.part.updated` → `part.type==="tool"` → find `session.sessionId===part.sessionID` → `handleToolUpdate`.
//! - `handleToolUpdate(part, channel, thread)` if `part.state.status!=="completed"` return; message `*${part.tool}* - ${part.state.title}` → `chat.postMessage({channel, thread_ts:thread, text})` with `.catch(()=>{})`.
//! - `app.use` middleware logs `📡 Raw Slack event:` + `JSON(context)` + `next()`.
//! - `app.message` handler: logs `📨 Received message event:` + JSON, skip if `subtype` or no `text` → `⏭️ Skipping message - no text or has subtype`; else `✅ Processing message:`; `channel=message.channel`, `thread=thread_ts||ts`, `sessionKey`; if no session → `🆕 Creating new opencode session...` → `client.session.create({body:{title:"Slack thread {thread}"}})` error → `❌ Failed to create session:` + `say("Sorry, I had trouble creating a session. Please try again.")`; else `✅ Created opencode session:` + id, store, `client.session.share({path:{id}})` → `🔗 Session shared:` + url → `postMessage({channel, thread_ts:thread, text:url})`; then `📝 Sending to opencode:` → `client.session.prompt({path:{id:sessionId}, body:{parts:[{type:"text", text}]}})` error → `❌ Failed to send message:` + `say("Sorry, I had trouble processing your message. Please try again.")`; else `📤 Opencode response:` + JSON, build `responseText = info?.content || parts.filter(p.type==="text").map(p.text).join("\n") || "I received your message but didn't have a response."` → `💬 Sending response:` → `say({text:responseText, thread_ts:thread})`.
//! - `app.command("/test", ...)` → ack + `🧪 Test command received:` + JSON → `say("🤖 Bot is working! I can hear you loud and clear.")`.
//! - `await app.start()` → `⚡️ Slack bot is running!`.
//!
//! PROVISIONAL: `@slack/bolt` and `@opencode-ai/sdk` are host runtimes — descriptor only.

// ---------------------------------------------------------------------------
// Env keys verbatim
// ---------------------------------------------------------------------------

pub const ENV_BOT_TOKEN: &str = "SLACK_BOT_TOKEN";
pub const ENV_SIGNING_SECRET: &str = "SLACK_SIGNING_SECRET";
pub const ENV_APP_TOKEN: &str = "SLACK_APP_TOKEN";

// ---------------------------------------------------------------------------
// Log strings verbatim
// ---------------------------------------------------------------------------

pub const LOG_CONFIG_HEADER: &str = "🔧 Bot configuration:";
pub const LOG_BOT_TOKEN_PRESENT: &str = "- Bot token present:";
pub const LOG_SIGNING_SECRET_PRESENT: &str = "- Signing secret present:";
pub const LOG_APP_TOKEN_PRESENT: &str = "- App token present:";
pub const LOG_STARTING: &str = "🚀 Starting opencode server...";
pub const LOG_READY: &str = "✅ Opencode server ready";
pub const LOG_RAW_EVENT: &str = "📡 Raw Slack event:";
pub const LOG_RECEIVED: &str = "📨 Received message event:";
pub const LOG_SKIPPING: &str = "⏭️ Skipping message - no text or has subtype";
pub const LOG_PROCESSING: &str = "✅ Processing message:";
pub const LOG_NEW_SESSION: &str = "🆕 Creating new opencode session...";
pub const LOG_CREATED: &str = "✅ Created opencode session:";
pub const LOG_SHARED: &str = "🔗 Session shared:";
pub const LOG_SENDING_TO_OPENCODE: &str = "📝 Sending to opencode:";
pub const LOG_RESPONSE: &str = "📤 Opencode response:";
pub const LOG_FAILED_CREATE: &str = "❌ Failed to create session:";
pub const LOG_FAILED_SEND: &str = "❌ Failed to send message:";
pub const LOG_SENDING_RESPONSE: &str = "💬 Sending response:";
pub const LOG_TEST_COMMAND: &str = "🧪 Test command received:";
pub const LOG_BOT_RUNNING: &str = "⚡️ Slack bot is running!";

// ---------------------------------------------------------------------------
// Opencode create params verbatim
// ---------------------------------------------------------------------------

pub const OPENCODE_PORT: u16 = 0;

// ---------------------------------------------------------------------------
// Session map / titles
// ---------------------------------------------------------------------------

/// Session key template verbatim: `"${channel}-${thread}"`.
pub fn session_key(channel: &str, thread: &str) -> String {
    format!("{}-{}", channel, thread)
}

/// Session title verbatim: `"Slack thread {thread}"`.
pub fn session_title(thread: &str) -> String {
    format!("Slack thread {}", thread)
}

// ---------------------------------------------------------------------------
// Tool update handling
// ---------------------------------------------------------------------------

/// Tool update status verbatim: `"completed"` is the only firing trigger.
pub const TOOL_STATUS_COMPLETED: &str = "completed";

/// Tool message template verbatim: `"*${part.tool}* - ${part.state.title}"`.
pub fn tool_message(tool: &str, title: &str) -> String {
    format!("*{}* - {}", tool, title)
}

// ---------------------------------------------------------------------------
// Message handler constants verbatim
// ---------------------------------------------------------------------------

pub const ERR_CREATE_SESSION: &str = "Sorry, I had trouble creating a session. Please try again.";
pub const ERR_SEND_MESSAGE: &str =
    "Sorry, I had trouble processing your message. Please try again.";
pub const FALLBACK_RESPONSE: &str = "I received your message but didn't have a response.";
pub const TEST_COMMAND: &str = "/test";
pub const TEST_COMMAND_RESPONSE: &str = "🤖 Bot is working! I can hear you loud and clear.";

// ---------------------------------------------------------------------------
// Event stream constants verbatim
// ---------------------------------------------------------------------------

pub const EVENT_TYPE_PART_UPDATED: &str = "message.part.updated";
pub const PART_TYPE_TOOL: &str = "tool";

// ---------------------------------------------------------------------------
// SDK share/session descriptors
// ---------------------------------------------------------------------------

/// Port of `responseText` derivation: `info?.content || parts.filter(text).join("\n") || fallback`.
pub fn response_text(info_content: Option<&str>, parts: &[(&str, &str)]) -> String {
    if let Some(c) = info_content {
        if !c.is_empty() {
            return c.to_string();
        }
    }
    let texts: Vec<&str> = parts
        .iter()
        .filter(|(typ, _)| *typ == "text")
        .map(|(_, text)| *text)
        .collect();
    if !texts.is_empty() {
        return texts.join("\n");
    }
    FALLBACK_RESPONSE.to_string()
}

// ---------------------------------------------------------------------------
// App descriptor
// ---------------------------------------------------------------------------

/// Mirrors Slack `App` construction `new App({token, signingSecret, socketMode:true, appToken})`.
#[derive(Clone, Debug, PartialEq)]
pub struct SlackConfig {
    pub socket_mode: bool,
}

impl SlackConfig {
    pub const SOCKET_MODE: bool = true;
}

/// Mirrors `SlackBot` surface (handler registration order verbatim).
#[derive(Clone, Debug, PartialEq)]
pub struct SlackBot;

impl SlackBot {
    pub const HANDLER_ORDER: &'static [&'static str] =
        &["use(middleware)", "message", "command(/test)", "start"];
}

// ---------------------------------------------------------------------------
// PROVISIONAL stubs
// ---------------------------------------------------------------------------

/// PROVISIONAL: `@slack/bolt` `App`/`App.client.chat.postMessage`/`say`/`ack`.
pub mod slack_provisional {
    pub const PACKAGE: &str = "@slack/bolt";
    pub const APP_CLASS: &str = "App";
}

/// PROVISIONAL: `@opencode-ai/sdk` `createOpencode`/`Opencode` client (pending `crates/client` + `crates/sdk`).
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk";
    pub const CREATE_FN: &str = "createOpencode";
}
