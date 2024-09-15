{
  description = "taskwarrior-web-server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlay];
      };
    in {
      devShell = pkgs.mkShell {
        buildInputs = [
          pkgs.rustc
          pkgs.cargo
          pkgs.rust-analyzer
        ];

        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };

      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "taskwarrior-web-server";
        version = "0.1.0";

        src = ./.;

        cargoSha256 = pkgs.lib.fakeSha256;

        buildInputs = [];

        meta = with pkgs.lib; {
          description = "taskwarrior-web-server";
          license = licenses.mit;
          maintainers = ["dltompki"];
        };
      };
    });
}
