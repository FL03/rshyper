---
title: Quickstart for Rust Development Container
description: Quickstart guide for setting up a Rust development container using devcontainers.
---

For format details, see <https://aka.ms/devcontainer.json>. For config options, see the [README](https://github.com/devcontainers/templates/blob/main/src/rust/README.md) for the [devcontainer templates](https://github.com/devcontainers/templates/tree/main/src/rust) repository.

Use 'mounts' to make the cargo cache persistent in a Docker Volume.

```json
"mounts": [
 {
  "source": "devcontainer-cargo-cache-${devcontainerId}",
  "target": "/usr/local/cargo",
  "type": "volume"
 }
]
```

Features to add to the dev container. More info: <https://containers.dev/features>.

```json
"features": {},
```

Use 'forwardPorts' to make a list of ports inside the container available locally.

```json
"forwardPorts": {},
```

Use 'postCreateCommand' to run commands after the container is created.

```json
"postCreateCommand": "rustc --version",
```

Configure tool-specific properties.

```json
"customizations": {
  "vscode": {
    "extensions": [
      "rust-lang.rust-analyzer",
      "matklad.rust-analyzer"
    ]
  }
}
```

Uncomment to connect as root instead. More info: <https://aka.ms/dev-containers-non-root>.

```json
"remoteUser": "vscode"
```
