cask "clipsync" do
  version "0.2.0"
  sha256 "4a4dc102f6c515d1d37e9b4a368c873640c6577097e32d9cf1e865533f562389"

  url "https://github.com/tedkulp/clipsync/releases/download/v#{version}/ClipSync_#{version}_universal.dmg"
  name "ClipSync"
  desc "Cross-platform clipboard synchronization tool"
  homepage "https://github.com/tedkulp/clipsync"

  app "ClipSync.app"

  # Also install the server binary
  binary "#{appdir}/ClipSync.app/Contents/MacOS/clipsync-server"

  zap trash: [
    "~/Library/Application Support/com.clipsync.desktop",
    "~/Library/Preferences/com.clipsync.desktop.plist",
    "~/Library/Caches/com.clipsync.desktop",
  ]
end
