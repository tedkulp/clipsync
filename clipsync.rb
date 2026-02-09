cask "clipsync" do
  version "0.1.1"
  sha256 "" # Will be filled after release

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
