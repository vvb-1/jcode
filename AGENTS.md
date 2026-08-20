# Jcode Repository Guidelines

## Scope and Default Target

- Treat the repository root containing this file as the source checkout for self-development.
- Default to the TUI/CLI product: package `jcode`, binary `jcode`, and self-dev target `tui`.
- Only switch to `jcode-desktop2`, desktop builds, or desktop UI debugging when the user explicitly asks about the desktop app.
- Resolve the checkout through `JCODE_REPO_DIR` when self-dev starts outside this repository.

## Development Workflow

- Inspect the relevant code and current git state before editing. Use a todo plan for non-trivial work and keep it current.
- Fix the root cause and complete necessary follow-through instead of stopping at diagnosis.
- Keep changes narrowly scoped. Do not mix opportunistic refactors into an unrelated fix.
- Preserve existing local changes. Never discard, overwrite, or revert work that you did not create.
- Stay on your own branch. Do not cherry-pick, merge, or copy code from another agent's or contributor's branch unless the source belongs to a repository maintainer and the user explicitly requests integration.
- Commit coherent changes as work progresses. Stage only files or hunks belonging to the current task.

## Self-Dev Build and Reload

- Prefer coordinated builds through the `selfdev` tool.
- Use `selfdev build` for compilation and `selfdev build-reload` when runtime verification needs the new binary immediately.
- Use target `tui` by default. Use `desktop2` or `all` only when the changed scope requires it.
- Cancel a queued or running build with `selfdev cancel-build` when it is no longer needed.
- Avoid release and `release-lto` builds unless release-specific behavior must be tested.
- If coordinated builds are unavailable, use:

  ```bash
  scripts/dev_cargo.sh build --profile selfdev -p jcode --bin jcode
  ```

  Fall back to `cargo build --profile selfdev -p jcode --bin jcode` only when the wrapper is unavailable, then use `selfdev reload`.
- After a successful reload, continue the task automatically. Do not wait for the user merely because the process reloaded.

## Verification

- Run the narrowest relevant tests first, then broaden verification when the change crosses crate or runtime boundaries.
- A successful `cargo build` does not prove runtime behavior. `jcode run` and interactive sessions use the long-lived daemon at `~/.jcode/builds/shared-server/jcode`.
- For isolated runtime checks that must not disturb the shared daemon or caller session, use a dedicated socket:

  ```bash
  cargo build --profile selfdev
  ./target/selfdev/jcode run --no-update --socket /run/user/1000/jcode-mytest.sock '<prompt>'
  ```

- For TUI changes, verify behavior through `debug_socket` testers and inspect rendered frames where practical.
- For configuration or process changes, validate the effective runtime state, not only the file contents.
- Before declaring completion, confirm relevant tests/builds, review the diff, remove temporary diagnostics, and check `git status`.

## Runtime Debugging Gotchas

- `crate::logging::info` writes to a log file, not stderr. Use `eprintln!` only for temporary diagnostics and remove it before committing.
- Resolve binary symlinks before inspecting binaries. For example, run `readlink -f ~/.jcode/builds/shared-server/jcode` before `strings` or similar tools.
- When testing a new build, confirm that the daemon or isolated socket is actually running that build. Otherwise the check may silently exercise old code.

## Install Layout

- `~/.local/bin/jcode` is the launcher found through `PATH`.
- `~/.jcode/builds/current/jcode` is the active local/source-build channel.
- `~/.jcode/builds/stable/jcode` is the stable release channel.
- `~/.jcode/builds/shared-server/jcode` is the binary used by the long-lived daemon.
- `~/.jcode/builds/versions/<version>/jcode` contains immutable versioned binaries.
- `~/.jcode/builds/canary/jcode` remains available for canary flows but is not the primary self-dev channel.
- Keep `~/.local/bin` before `~/.cargo/bin` in `PATH`.
- On Windows, use `%LOCALAPPDATA%\\jcode\\bin\\jcode.exe` for the launcher, `%LOCALAPPDATA%\\jcode\\builds\\stable\\jcode.exe` for stable, and `%LOCALAPPDATA%\\jcode\\builds\\versions\\<version>\\jcode.exe` for immutable installs.
