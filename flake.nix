{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    libraries = with pkgs; [
      # https://github.com/gfx-rs/wgpu/blame/trunk/shell.nix
      alsa-lib
      fontconfig
      freetype
      shaderc
      directx-shader-compiler

      libGL
      vulkan-headers
      vulkan-loader
      vulkan-tools
      vulkan-tools-lunarg
      vulkan-extension-layer
      vulkan-validation-layers # don't need them *strictly* but immensely helpful

      # necessary for developing (all of) wgpu itself
      cargo-nextest
      cargo-fuzz

      # nice for developing wgpu itself
      typos

      # WINIT_UNIX_BACKEND=wayland
      wayland

      # WINIT_UNIX_BACKEND=x11
      libxkbcommon
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
    ];
    overrides = builtins.fromTOML (builtins.readFile ./rust-toolchain.toml);
  in
    with pkgs; rec {
      devShells.${system}.default = mkShell rec {
        buildInputs = with pkgs; [
          pkg-config
          cmake
          mold # could use any linker, needed for rustix (but mold is fast)

          # Binaries necessary for this project
          openssl

          rustup
          clippy
        ];

        shellHook = ''
          export RUSTC_VERSION="${overrides.toolchain.channel}"
          export PATH="$PATH:''${CARGO_HOME:-~/.cargo}/bin"
          export PATH="$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/"
          export LD_LIBRARY_PATH="${lib.makeLibraryPath libraries}"

          rustup default $RUSTC_VERSION
          rustup component add rust-src rust-analyzer
        '';
      };
    };
}
