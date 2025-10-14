{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = {
      nixpkgs.follows = "nixpkgs";
    };

    comet = {
      url = "github:iced-rs/comet";
      flake = false;
    };

    hot = {
      url = "github:hecrj/cargo-hot";
      flake = false;
    };
  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem =
        {
          lib,
          pkgs,
          ...
        }:
        let
          comet = pkgs.rustPlatform.buildRustPackage {
            pname = "iced_comet";
            version = "0.14.0-dev"; # Update this to match your package version
            src = inputs.comet;
            cargoLock = {
              lockFile = "${inputs.comet}/Cargo.lock";
              outputHashes = {
                "cryoglyph-0.1.0" = "sha256-X7S9jq8wU6g1DDNEzOtP3lKWugDnpopPDBK49iWvD4o=";
                "dpi-0.1.1" = "sha256-hlVhlQ8MmIbNFNr6BM4edKdZbe+ixnPpKm819zauFLQ=";
                "iced-0.14.0-dev" = "sha256-Thoqj1WY+48VqRkPEJVOkHs3g/pt6MEzg1J7pdogKzA=";
                "iced_palace-0.14.0-dev" = "sha256-OcFPDxp8GBUvQaX0SU1I/b2enIIqKgaLdGvmHHrKrp4=";

              };
            };
            nativeBuildInputs = with pkgs; [
              pkg-config
            ];
            buildInputs = with pkgs; [
              openssl
            ];
          };

          hot = pkgs.rustPlatform.buildRustPackage {
            pname = "cargo-hot";
            version = "0.1.0"; # Update this to match your package version
            src = inputs.hot;
            cargoLock = {
              lockFile = "${inputs.hot}/Cargo.lock";
              outputHashes = { };
            };
            nativeBuildInputs = with pkgs; [
              pkg-config
            ];
            buildInputs = with pkgs; [
              openssl
            ];
          };
        in
        {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.
          devenv.shells.default = {
            # https://devenv.sh/reference/options/
            dotenv.disableHint = true;

            languages.cplusplus.enable = true;
            languages.java.enable = true;
            languages.rust.enable = true;
            languages.rust.channel = "stable";
            packages = with pkgs; [
              act
              comet
              hot
              just
              # openssl
              # pkg-config
              tokio-console

              qt6.full
              protobuf
            ];

            env = {
              LD_LIBRARY_PATH = lib.makeLibraryPath (
                with pkgs;
                [
                  libGL
                  libxkbcommon
                  vulkan-loader
                  wayland
                  xorg.libXcursor
                  xorg.libXrandr
                  xorg.libXi
                  xorg.libX11
                ]
              );
              ICED_BACKEND = "wgpu"; # wgpu or skia
              RUST_LOG = "info";
            };
          };
        };
    };
}
