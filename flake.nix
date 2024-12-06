{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      pre-commit-hooks,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        nativeBuildInputs =
          with pkgs;
          [
            rust-bin.stable.latest.default
            rust-analyzer
          ]
          ++ (
            if system == "aarch64-darwin" || system == "x86_64-darwin" then
              [
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.SystemConfiguration
              ]
            else
              [ ]
          );

        buildInputs = with pkgs; [ openssl ];

        src = pkgs.lib.cleanSource ./.;

        cargoToml = (pkgs.lib.importTOML ./Cargo.toml).package;
        pname = cargoToml.name;
        version = cargoToml.version;

        default = pkgs.rustPlatform.buildRustPackage {
          inherit
            pname
            version
            src
            nativeBuildInputs
            buildInputs
            ;
          cargoLock.lockFile = ./Cargo.lock;
        };

        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            nixfmt-rfc-style.enable = true;
          };
        };
      in
      with pkgs;
      {
        formatter = nixfmt-rfc-style;

        checks = {
          inherit pre-commit-check;
        };

        devShells.default = mkShell {
          inherit nativeBuildInputs buildInputs;
          inherit (pre-commit-check) shellHook;
        };

        packages = {
          inherit default;

          container = dockerTools.buildImage {
            name = "${pname}";
            tag = "${version}";

            config = {
              Entrypoint = [ "${default}/bin/${pname}" ];
            };
          };
        };
      }
    );
}
