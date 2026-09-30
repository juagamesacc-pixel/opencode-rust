// source: packages/tui/src/util/renderer.ts (7 lines, v1.18.30)
// 1:1 port — terminal title reset, then destroy unless already gone.

#![allow(dead_code)]

/// Minimal renderer surface read by `destroyRenderer`.
pub trait RendererHandle {
    fn is_destroyed(&self) -> bool;
    fn set_terminal_title(&mut self, title: &str);
    fn destroy(&mut self);
}

/// Mirrors `destroyRenderer`.
pub fn destroy_renderer(renderer: &mut dyn RendererHandle) {
    renderer.set_terminal_title("");
    if renderer.is_destroyed() {
        return;
    }
    renderer.destroy();
}