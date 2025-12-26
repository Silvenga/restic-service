# restic-service

A simple service for Windows that hosts and executes restic backup/forget runs.

- Supports multiple jobs, with only one job running at a time.
- Gracefully stops running jobs when the service is stopped (e.g. on shutdown).
- Automatic detection of fixed drives (opt-in).

## Installation

Install the service using the MSI downloaded from the releases page.

The service doesn't ship restic, so you need to install it separately. Make sure restic is available in the system PATH
and note that restic will run as the service user (usually `LocalSystem`). WinGet is recommended:

```powershell
winget install --exact --id restic.restic --scope Machine
```

Other installation methods are also supported, see
the [restic documentation](https://restic.readthedocs.io/en/latest/020_installation.html#windows).

## Configuration

Configure the service by editing `C:\Program Files\Restic Service\service_config.toml` (use elevation). Changes will automatically be
picked up by the service (running jobs will be gracefully stopped).

An example configuration file is in [`./docs/service_config.toml`](./docs/service_config.toml).

## Building

Assumes rust (>= 1.91) and node (LTS) are installed.

```pwsh
# Installer:
winget install WiXToolset.WiXCLI
wix extension add -g WixToolset.UI.wixet

# Building
cargo install --force cargo-make

# UI
corepack enable

# Build the service/UI/installer
cargo make release
```
