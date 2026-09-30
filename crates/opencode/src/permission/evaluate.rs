// source: src/permission/evaluate.ts — `export { evaluate } from "."`
// (implementation canonical home; parent barrel re-exports it).
use super::Rule;

/// source: ask-fallback rule — { action: "ask", permission, pattern: "*" }. Verbatim.
pub fn ask_fallback(permission: &str) -> Rule {
    Rule {
        permission: permission.to_string(),
        pattern: "*".to_string(),
        action: "ask".to_string(),
    }
}

/// source: evaluate() — rulesets.flat().findLast(both wildcard match) ??
/// ask-fallback. Verbatim (uses crate wildcard matcher).
pub fn evaluate(permission: &str, pattern: &str, rulesets: &[Vec<Rule>]) -> Rule {
    for rule in rulesets.iter().flatten().rev() {
        if crate::util::wildcard::match_(permission, &rule.permission)
            && crate::util::wildcard::match_(pattern, &rule.pattern)
        {
            return rule.clone();
        }
    }
    ask_fallback(permission)
}
