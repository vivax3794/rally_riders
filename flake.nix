{
  description = "Dev shell for bevy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
            nativeBuildInputs = [
               pkg-config
            ];

            buildInputs = [
                ( rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                }) )
                rust-bin.stable.latest.default
                rust-analyzer
                mold
                just

                udev alsa-lib vulkan-loader
                xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
                libxkbcommon wayland # To use the wayland feature
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
    }
    );
}

