{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
    nix-github-actions = {
      url = "github:nix-community/nix-github-actions";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    advisory-db,
    nix-github-actions,
    ...
  }:
    flake-utils.lib.eachSystem [flake-utils.lib.system.x86_64-linux flake-utils.lib.system.aarch64-darwin]
    (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
        ];
      };

      rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;

      src = let
        mdFilter = path: _type: builtins.match ".*md$" path != null;
        graphqlFilter = path: _type: builtins.match ".*graphql$" path != null;
        miscOrCargo = path: type:
          (mdFilter path type)
          || (graphqlFilter path type)
          || (craneLib.filterCargoSources path type);
      in
        pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = miscOrCargo;
        };

      commonArgs = {
        pname = "runpod-workspace";
        inherit src;

        buildInputs = with pkgs;
          [
            pkg-config
            openssl
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];
      };

      nativeArgs = commonArgs // {pname = "runpod";};

      cargoArtifacts = craneLib.buildDepsOnly nativeArgs;

      runpod = craneLib.buildPackage (
        commonArgs
        // {
          inherit cargoArtifacts;
          pname = "runpod";
          cargoBuildCommand = "cargo build --release --package runpod";
        }
      );
      runpod-cli = craneLib.buildPackage (
        commonArgs
        // {
          inherit cargoArtifacts;
          pname = "runpod-cli";
          cargoBuildCommand = "cargo build --release --bin runpod-cli";
        }
      );
    in {
      checks = {
        workspace-clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          }
        );

        workspace-doc = craneLib.cargoDoc (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );

        workspace-fmt = craneLib.cargoFmt {
          inherit src;
        };

        workspace-audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };
      };

      packages = {
        inherit runpod runpod-cli;

        default = runpod-cli;

        githubCheckActions = nix-github-actions.lib.mkGithubMatrix {
          attrPrefix = "githubCheckActions.checks";
          checks = nixpkgs.lib.getAttrs ["x86_64-linux"] self.checks;
        };
      };

      apps = {
        inherit runpod-cli;
      };

      devShells.default = craneLib.devShell {
        inputsFrom = [
          runpod
          runpod-cli
        ];

        packages = with pkgs; [
          cargo-audit
          cargo-watch
        ];

        checks = self.checks.${system};

        RUST_LOG = "info";
      };
    });
}
