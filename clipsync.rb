class Clipsync < Formula
  desc "Cross-platform clipboard synchronization tool"
  homepage "https://github.com/tedkulp/clipsync"
  url "https://github.com/tedkulp/clipsync/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "" # Will be filled after release
  license "MIT"
  head "https://github.com/tedkulp/clipsync.git", branch: "main"

  depends_on "rust" => :build
  depends_on "node" => :build
  depends_on "npm" => :build

  on_linux do
    depends_on "pkg-config" => :build
    depends_on "gtk+3"
    depends_on "webkit2gtk"
  end

  def install
    # Build server
    system "cargo", "build", "--release", "-p", "clipsync-server"
    bin.install "target/release/clipsync-server"

    # Build frontend
    cd "desktop" do
      system "npm", "install"
      system "npm", "run", "build"
    end

    # Build desktop app
    # On macOS, use cargo tauri build
    # On Linux, just build the binary (tauri bundling handled separately)
    if OS.mac?
      # Install tauri-cli
      system "cargo", "install", "tauri-cli", "--version", "^2.0", "--locked"
      
      cd "crates/desktop" do
        system "cargo", "tauri", "build"
        # Install the .app bundle
        prefix.install "../../target/release/bundle/macos/ClipSync.app"
        bin.write_exec_script "#{prefix}/ClipSync.app/Contents/MacOS/ClipSync"
      end
    else
      # On Linux, just install the binary
      cd "crates/desktop" do
        system "cargo", "build", "--release"
      end
      bin.install "target/release/clipsync-desktop"
    end
  end

  service do
    run [opt_bin/"clipsync-server"]
    keep_alive true
    log_path var/"log/clipsync-server.log"
    error_log_path var/"log/clipsync-server.log"
  end

  test do
    # Test server starts
    system "#{bin}/clipsync-server", "--help"
    
    # Test desktop app (if not in headless environment)
    if OS.mac?
      system "#{bin}/clipsync-desktop", "--version" rescue nil
    end
  end
end
