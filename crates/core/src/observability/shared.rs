//! Rust port of `packages/core/src/observability/shared.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use std::sync::OnceLock;

fn uuid_v4() -> String {
    // std-only CSPRNG via /dev/urandom fallback to time-seeded xorshift
    let mut buf = [0u8; 16];
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        use std::io::Read;
        if f.read_exact(&mut buf).is_ok() {
            buf[6] = (buf[6] & 0x0f) | 0x40;
            buf[8] = (buf[8] & 0x3f) | 0x80;
            return format!(
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15]
            );
        }
    }
    // fallback simple
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9e3779b97f4a7c15);
    for b in &mut buf {
        seed = seed
            .wrapping_mul(0x5851f42d4c957f2d)
            .wrapping_add(0x14057b7ef767814f);
        *b = (seed >> 33) as u8;
    }
    buf[6] = (buf[6] & 0x0f) | 0x40;
    buf[8] = (buf[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15]
    )
}

static RUN_ID: OnceLock<String> = OnceLock::new();

pub fn run_id() -> &'static str {
    RUN_ID.get_or_init(|| uuid_v4()[..8].to_string())
}

/// Source: `export const runID = crypto.randomUUID().slice(0, 8)`
pub const RUN_ID_LEN: usize = 8;
