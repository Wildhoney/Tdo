class Tdo < Formula
  desc "Terminal based todo app for managing today's tasks with gentle reminders"
  homepage "https://github.com/Wildhoney/Tdo"
  version "0.1.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Wildhoney/Tdo/releases/download/v0.1.1/tdo-aarch64-apple-darwin.tar.gz"
      sha256 "3e2c01374c8064c8d471350b8b49e53ebc076b07d3899d09b87baa448b020280"
    else
      url "https://github.com/Wildhoney/Tdo/releases/download/v0.1.1/tdo-x86_64-apple-darwin.tar.gz"
      sha256 "5ccb27eb6f84deee740b620f112318d158f0f05d21ff72514448a5c02e4e60a2"
    end
  end

  on_linux do
    url "https://github.com/Wildhoney/Tdo/releases/download/v0.1.1/tdo-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "2b55ed341b4b82eaa3399bb9e7fb698ddb4c8db6fc36ef2efb68413e4e6a215c"
  end

  def install
    bin.install "tdo"
  end

  test do
    assert_match "Tdo", shell_output("#{bin}/tdo --help")
  end
end
