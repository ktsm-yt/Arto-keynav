#!/usr/bin/env bash
# Reject a macOS app bundle that would not launch outside the build environment.
#
# Release bundles must link only against libraries macOS itself ships. A bundle
# built inside the Nix devShell instead records absolute /nix/store install
# names (libiconv, pulled in by the `libc` crate's `#[link(name = "iconv")]`),
# and dyld kills the app at launch on every machine without that store path.
#
# Three independent checks:
#
#   1. No load command in any Mach-O of the bundle may mention /nix/store.
#      `otool -l` (not -L) is used so LC_RPATH and LC_LOAD_WEAK_DYLIB entries
#      are covered too, not just LC_LOAD_DYLIB.
#   2. The executable actually starts. Only dyld can confirm that every
#      recorded dependency resolves and satisfies its compatibility version;
#      `--version` exercises that and exits immediately.
#   3. The executable must embed its frontend rather than read the source tree.
set -euo pipefail

app="${1:?usage: platform/macos/verify-bundle.sh <path to .app> <executable name>}"
executable="${2:?usage: platform/macos/verify-bundle.sh <path to .app> <executable name>}"

# Verify the seal after all metadata and resources have been embedded.
codesign --verify --deep --strict "$app"
test -s "$app/Contents/Resources/LICENSE"
test -s "$app/Contents/Resources/THIRD-PARTY-NOTICES.txt"

# This diagnostic exists only in the source-dependent debug frontend loader.
# shellcheck disable=SC2016
if grep -aFq 'Frontend bundle is missing; run `just frontend::assets`' "$app/Contents/MacOS/$executable"; then
  echo "Error: debug executable reads frontend files from the source tree; rebuild with --release" >&2
  exit 1
fi

status=0
while IFS= read -r -d '' bin; do
  file -b "$bin" | grep -q '^Mach-O' || continue

  # Captured before grepping: piping into grep under `pipefail` would let an
  # otool failure masquerade as a clean result.
  if ! load_commands="$(otool -l "$bin")"; then
    echo "Error: cannot read load commands of ${bin#"$app"/}" >&2
    status=1
    continue
  fi

  if refs="$(grep -F /nix/store <<<"$load_commands")"; then
    echo "Error: ${bin#"$app"/} depends on the build environment:" >&2
    echo "$refs" >&2
    status=1
  fi
done < <(find "$app" -type f -print0)

if [[ "$status" -ne 0 ]]; then
  exit "$status"
fi

"$app/Contents/MacOS/$executable" --version
echo "Bundle verified: valid signature, LICENSE, embedded frontend, no /nix/store references, executable launches"
