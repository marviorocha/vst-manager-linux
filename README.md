# Yabridge VST Manager

Yabridge VST Manager is a Linux desktop application for checking Yabridge requirements, registering Windows plugin folders, and syncing those plugins with a Linux DAW.

**Current version:** 0.1.0  
**Languages:** English (default), Spanish, and Portuguese (Brazil)  
**Platform:** Linux desktop

## What the app manages

The app manages plugin folders registered with `yabridgectl`. It does not download or install VST plugin installers. Install a Windows plugin into a Wine prefix first, then register the folder containing its plugin files in the app.

## First launch

The setup assistant has three steps:

1. **Requirements:** checks the Linux distribution, Wine, and `yabridgectl`. If a supported system is missing a dependency, the app offers a guided installation.
2. **Locations:** add or remove a plugin folder, or refresh the list of folders already registered with Yabridge.
3. **Sync:** run `yabridgectl status` to inspect the current setup, or `yabridgectl sync` to apply changes and update the bridged plugins.

The **Settings** button opens language preferences and the About section. The language choice is saved on this computer.

## System requirements and installation

Yabridge requires a recent compatible Wine Staging installation and `yabridgectl`. The app checks for these tools when it opens; it does not install anything without the user's confirmation.

Automatic installation is currently supported on:

- **Fedora 43/44, x86_64, non-Atomic:** enables the community `patrickl/wine-tkg` COPR recommended by Yabridge and installs the Wine/Yabridge packages for that repository.
- **Arch Linux and Manjaro, x86_64:** installs `wine-staging`, `yabridge`, and `yabridgectl` from the package repositories already configured on the system.

Installation requires `pkexec` and a working graphical Polkit authentication agent. Fedora setup enables a third-party COPR and can install, update, or replace system packages. Review the package manager's transaction and authorize it only if you agree with the changes.

Automatic installation is disabled on Fedora Atomic, unsupported Fedora versions, other distributions, and unsupported architectures. Follow the [official Yabridge installation instructions](https://github.com/robbert-vdh/yabridge#usage) for those systems. Debian-based distributions should use the WineHQ instructions for a compatible Wine Staging version.

## Registering plugin folders

1. Install the Windows plugin with Wine into the prefix you use for plugins.
2. Open **Locations** in the setup assistant.
3. Select **Choose folder** and choose the directory containing the plugin files, or enter its full path.
4. Select **Add location**. **Refresh list** runs `yabridgectl list` to show the registered folders.
5. Go to **Sync** and run `yabridgectl sync` so Yabridge updates the bridged plugins.

Use **Remove link** to unregister the selected path. This only removes the path from Yabridge's configuration; it does not delete the folder or plugin files. Run Sync after adding or removing folders.

## Troubleshooting

**The requirements page says the system is unavailable.** Open the desktop Tauri application. The browser preview does not have access to the Rust commands that inspect the system.

**The password prompt does not respond.** The app relies on the desktop Polkit authentication agent. You can check whether Polkit prompts work in the graphical session by running `pkexec /usr/bin/true` in a terminal. If that prompt also hangs, fix the system's Polkit agent before retrying the app.

**The install button is unavailable.** Automatic installation is limited to the distributions and versions listed above. Use the official installation instructions for other systems.

**A plugin does not appear in the DAW.** Confirm that the folder containing the plugin was registered, run Sync, then check that the DAW scans the appropriate Linux VST directories. Use the app's Status command to inspect Yabridge's report.

## Publishing a GitHub release

The workflow in `.github/workflows/release.yml` builds and publishes a Linux release whenever a version tag matching `v*` is pushed. It uses GitHub's built-in `GITHUB_TOKEN`; no personal access token or signing secret is required for this workflow.

Before publishing, set the same version in `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml`. For example, to prepare version `0.1.1`:

1. Run `npm version 0.1.1 --no-git-tag-version` to update `package.json` and `package-lock.json`.
2. Set `"version": "0.1.1"` in `src-tauri/tauri.conf.json` and `version = "0.1.1"` in `src-tauri/Cargo.toml`.
3. Run `cargo check --manifest-path src-tauri/Cargo.toml` to update and verify `Cargo.lock`.
4. Commit and push the version changes.
5. Create and push the tag:

   ```sh
   git tag v0.1.1
   git push origin v0.1.1
   ```

The workflow checks that the tag and all three project versions match, builds on Ubuntu 24.04, and publishes the Tauri Linux bundle artifacts in a non-draft GitHub Release. If versions do not match, the workflow stops before building. Check the repository's **Actions** tab for build progress and **Releases** for the published downloads.

## Running from source

From the project directory:

```sh
npm install
npm run tauri dev
```

Build the frontend with `npm run build`. Rust backend tests can be run with:

```sh
cargo test --manifest-path src-tauri/Cargo.toml --bin vst_gui
```
