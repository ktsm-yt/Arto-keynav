# Install Arto Keynav

Arto Keynav is an unofficial fork of [Arto](https://github.com/arto-app/Arto).
The beta download targets Apple Silicon macOS only; Intel, Windows, and Linux downloads are not provided.

1. Download the `aarch64.dmg` and `SHA256SUMS` from [this fork's releases](https://github.com/ktsm-yt/Arto-keynav/releases).
2. In the download directory, run `shasum -a 256 -c SHA256SUMS`.
3. Open the DMG and drag **Arto Keynav.app** to **Applications**.

The beta is ad-hoc signed, not Developer ID signed or notarized. If Gatekeeper blocks it, review the source of the download and allow this app through **System Settings → Privacy & Security**. Do not disable Gatekeeper system-wide.

This fork has its own bundle identifier, settings, history, cache, IPC socket, and Keychain service. It does not replace `Arto.app` or import its settings. macOS settings live in `~/Library/Application Support/arto-keynav/`.

The beta does not embed a Quick Look extension, so an existing upstream Markdown preview provider stays in charge.

For terminal use, invoke the executable inside this app:

```sh
"/Applications/Arto Keynav.app/Contents/MacOS/arto" README.md
```

To build and verify locally, see the [README](../README.md#ソースからビルドする).
