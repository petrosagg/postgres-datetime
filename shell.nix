{ pkgs ? import <nixpkgs> {} }:
with pkgs;

stdenv.mkDerivation {
  name = "c2rust";
  buildInputs = [
    pkgs.llvmPackages_14.clang
    pkgs.llvmPackages_14.llvm
    pkgs.cmake
    pkgs.openssl
    pkgs.pkgconfig
    pkgs.python3
    pkgs.rustup
    pkgs.zlib
  ];
  LIBCLANG_PATH="${llvmPackages_14.libclang.dev}";
}
