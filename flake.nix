{
  description = "Rust VS Code Dev Environment";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };
  outputs = { self, nixpkgs, flake-utils, fenix, ...}:
    flake-utils.lib.eachDefaultSystem (
      system: 
      let
        pkgs = import nixpkgs {inherit system; config.allowUnfree = true; };
        rust-toolchain = fenix.packages.${system}.complete;
      in
      {
        devShells.default = with pkgs; mkShell rec {
          buildInputs = [
            (rust-toolchain.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            pkg-config
            cargo-udeps
            git
            udev udev.dev alsa-lib lutris
            vulkan-tools vulkan-headers vulkan-loader vulkan-validation-layers
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
            libxkbcommon wayland # To use the wayland feature
            rustc.llvmPackages.clang
            rustc.llvmPackages.bintools
            (wrapBintoolsWith { bintools = mold; })
          ];
          LIBCLANG_PATH = lib.makeLibraryPath [ rustc.llvmPackages.libclang.lib ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          RUST_SRC_PATH = "${rust-toolchain.rust-src}/lib/rustlib/src/rust/library";
          PATH = "${rust-toolchain.cargo}/bin";
          RUSTFLAGS = "-C link-arg=-fuse-ld=mold -C linker=clang -Zshare-generics=y";
        };
      }
    );
}