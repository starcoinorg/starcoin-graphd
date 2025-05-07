{ pkgs ? import <nixpkgs> {} }:

let
  rustup = pkgs.rustup;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.openssl
    pkgs.pkg-config
    pkgs.protobuf
    pkgs.llvmPackages.libcxxClang
    pkgs.rust-analyzer
    rustup
  ];

  # 设置 rustup 环境，确保使用 rust-toolchain.toml 的版本
  shellHook = ''
    if [ -f rust-toolchain.toml ]; then
      rust_version=$(grep 'channel' rust-toolchain.toml | cut -d '"' -f 2)
      rustup override set "$rust_version"
      rustup component add rustfmt --toolchain "$rust_version"
      rustup component add rust-analysis --toolchain "$rust_version"
    fi
    export TMPDIR="$HOME/.cache/tmp"
    mkdir -p "$TMPDIR"
  '';
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
