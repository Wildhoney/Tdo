class Tdo < Formula
  desc "Terminal based todo app for managing today's tasks with gentle reminders"
  homepage "https://github.com/Wildhoney/Tdo"
  version "${VERSION}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Wildhoney/Tdo/releases/download/v${VERSION}/tdo-aarch64-apple-darwin.tar.gz"
      sha256 "${SHA256_ARM}"
    else
      url "https://github.com/Wildhoney/Tdo/releases/download/v${VERSION}/tdo-x86_64-apple-darwin.tar.gz"
      sha256 "${SHA256_X86}"
    end
  end

  on_linux do
    url "https://github.com/Wildhoney/Tdo/releases/download/v${VERSION}/tdo-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "${SHA256_LINUX}"
  end

  def install
    bin.install "tdo"
  end

  test do
    assert_match "Tdo", shell_output("#{bin}/tdo --help")
  end
end
