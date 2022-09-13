# Maintainer: varsity <varsity@duck.com>
pkgname=hybrid-bar-git
pkgver=0.1.4
pkgrel=1
makedepends=('rust' 'cargo' 'git')
depends=('gtk3' 'gtk-layer-shell')
url="https://github.com/vars1ty/HybridBar"
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="A status bar focused on wl-roots Wayland compositors"
license=('MIT')

# Generated in accordance to https://wiki.archlinux.org/title/Rust_package_guidelines.
# Might require further modification depending on the package involved.
prepare() {
  git clone https://github.com/vars1ty/HybridBar .
  cargo fetch --target "$CARCH-unknown-linux-gnu"
}

build() {
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  pwd
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/hybrid-bar"
}