class Tdo < Formula
  desc "Terminal based todo app for managing today's tasks with gentle reminders"
  homepage "https://github.com/Wildhoney/Tdo"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Wildhoney/Tdo/releases/download/v0.1.0/tdo-aarch64-apple-darwin.tar.gz"
      sha256 "22bc4fcc51547965e69812b2e19ebcd85a2a3a598eb42fd9c422e2797afd1dbe"
    else
      url "https://github.com/Wildhoney/Tdo/releases/download/v0.1.0/tdo-x86_64-apple-darwin.tar.gz"
      sha256 "eaddb6738c6a381bab3acdadc780facd6fca381fd0e9e0eb0469effa637dc99a"
    end
  end

  on_linux do
    url "https://github.com/Wildhoney/Tdo/releases/download/v0.1.0/tdo-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "3f4cb057499aa79a068499f0790bf78f274dd021d1ae7b0ca8582cedd9b54969"
  end

  def install
    bin.install "tdo"
  end

  test do
    assert_match "Tdo", shell_output("#{bin}/tdo --help")
  end
end
