{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    toolchain = (pkgs.rust-bin.stable."1.96.0".default.override {
      targets = [ "aarch64-unknown-linux-gnu" ];
      extensions = [ "rust-src" "rustfmt" ];
    });
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        toolchain
        pkgs.cargo-zigbuild
        pkgs.zig
        pkgs.just
      ];
    };
  };
}
