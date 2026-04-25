#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TAG="${1:-dev}"
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64) PLATFORM="macos-x64" ;;
  arm64) PLATFORM="macos-arm64" ;;
  *) PLATFORM="macos-$ARCH" ;;
esac

DIST="dist"
APP_DIR="$DIST/md-bider.app"
ICONSET_DIR="$DIST/md-bider.iconset"
ZIP_PATH="$DIST/md-bider-$TAG-$PLATFORM.zip"

cargo build --release

rm -rf "$APP_DIR" "$ICONSET_DIR" "$ZIP_PATH"
mkdir -p "$APP_DIR/Contents/MacOS" "$APP_DIR/Contents/Resources" "$ICONSET_DIR"

cp target/release/md-bider "$APP_DIR/Contents/MacOS/md-bider"
chmod +x "$APP_DIR/Contents/MacOS/md-bider"

cat > "$APP_DIR/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>md-bider</string>
  <key>CFBundleDisplayName</key>
  <string>md-bider</string>
  <key>CFBundleIdentifier</key>
  <string>com.cloveric.md-bider</string>
  <key>CFBundleVersion</key>
  <string>$TAG</string>
  <key>CFBundleShortVersionString</key>
  <string>$TAG</string>
  <key>CFBundleExecutable</key>
  <string>md-bider</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>CFBundleIconFile</key>
  <string>md-bider.icns</string>
</dict>
</plist>
PLIST

for size in 16 32 128 256 512; do
  sips -z "$size" "$size" assets/app_icon.png --out "$ICONSET_DIR/icon_${size}x${size}.png" >/dev/null
  sips -z "$((size * 2))" "$((size * 2))" assets/app_icon.png --out "$ICONSET_DIR/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$ICONSET_DIR" -o "$APP_DIR/Contents/Resources/md-bider.icns"

codesign --force --deep --sign - "$APP_DIR"
ditto -c -k --sequesterRsrc --keepParent "$APP_DIR" "$ZIP_PATH"
echo "$ZIP_PATH"
