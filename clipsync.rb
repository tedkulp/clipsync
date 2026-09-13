cask "clipsync" do
  version "0.2.0"
  sha256 "4a4dc102f6c515d1d37e9b4a368c873640c6577097e32d9cf1e865533f562389"

  url "https://github.com/tedkulp/clipsync/releases/download/v#{version}/ClipSync_#{version}_universal.dmg"
  name "ClipSync"
  desc "Cross-platform clipboard synchronization tool"
  homepage "https://github.com/tedkulp/clipsync"

  app "ClipSync.app"

  # The app is only ad-hoc signed; clear quarantine so Gatekeeper allows it to open.
  postflight do
    system_command "/usr/bin/xattr", args: ["-dr", "com.apple.quarantine", "#{appdir}/ClipSync.app"]
  end

  zap trash: [
    "~/Library/Application Support/com.clipsync.desktop",
    "~/Library/Preferences/com.clipsync.desktop.plist",
    "~/Library/Caches/com.clipsync.desktop",
  ]
end
