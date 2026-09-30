// source: src/permission/arity.ts — exports: prefix, BashArity
// (ARITY table generated per header comment; longest-prefix-wins verbatim).

/// source: prefix() — longest matching prefix wins; fallback first token
/// (empty → []). Verbatim.
pub fn prefix(tokens: &[String]) -> Vec<String> {
    for len in (1..=tokens.len()).rev() {
        let prefix = tokens[..len].join(" ");
        if let Some(arity) = arity(&prefix) {
            return tokens[..arity.min(tokens.len())].to_vec();
        }
    }
    if tokens.is_empty() {
        return vec![];
    }
    tokens[..1].to_vec()
}

/// source: ARITY — verbatim table (alphabetical per generator prompt).
pub fn arity(prefix: &str) -> Option<usize> {
    Some(match prefix {
        "cat" | "cd" | "chmod" | "chown" | "cp" | "echo" | "env" | "export" | "grep" | "kill"
        | "killall" | "ln" | "ls" | "mkdir" | "mv" | "ps" | "pwd" | "rm" | "rmdir" | "sleep"
        | "source" | "tail" | "touch" | "unset" | "which" => 1,
        "aws" | "az" | "doctl" | "gcloud" | "gh" | "sfdx" => 3,
        "bazel" | "brew" | "bun" | "cargo" | "cdk" | "cf" | "cmake" | "composer" | "consul"
        | "crictl" | "deno" | "docker" | "eksctl" | "firebase" | "flyctl" | "go" | "gradle"
        | "helm" | "heroku" | "hugo" | "ip" | "kind" | "kubectl" | "kustomize" | "make" | "mc"
        | "minikube" | "mongosh" | "mysql" | "mvn" | "ng" | "npm" | "nvm" | "nx" | "openssl"
        | "pip" | "pipenv" | "pnpm" | "poetry" | "podman" | "psql" | "pulumi" | "pyenv"
        | "python" | "rake" | "rbenv" | "redis-cli" | "rustup" | "serverless" | "skaffold"
        | "sls" | "sst" | "swift" | "systemctl" | "terraform" | "tmux" | "turbo" | "ufw"
        | "vault" | "vercel" | "volta" | "wp" | "yarn" => 2,
        "bun run"
        | "bun x"
        | "cargo add"
        | "cargo run"
        | "consul kv"
        | "deno task"
        | "docker builder"
        | "docker compose"
        | "docker container"
        | "docker image"
        | "docker network"
        | "docker volume"
        | "eksctl create"
        | "git config"
        | "git remote"
        | "git stash"
        | "ip addr"
        | "ip link"
        | "ip netns"
        | "ip route"
        | "kind create"
        | "kubectl kustomize"
        | "kubectl rollout"
        | "mc admin"
        | "npm exec"
        | "npm init"
        | "npm run"
        | "npm view"
        | "openssl req"
        | "openssl x509"
        | "pnpm dlx"
        | "pnpm exec"
        | "pnpm run"
        | "podman container"
        | "podman image"
        | "pulumi stack"
        | "terraform workspace"
        | "vault auth"
        | "vault kv"
        | "yarn dlx"
        | "yarn run" => 3,
        _ => return None,
    })
}
