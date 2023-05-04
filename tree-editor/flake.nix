{
  description = "TRI editor project";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in {
      devShells.${system}.default =
        pkgs.mkShell {
          buildInputs = [
            pkgs.cargo
            pkgs.rustc
            pkgs.rust-analyzer
            pkgs.vscode-extensions.vadimcn.vscode-lldb
            pkgs.imagemagick
          ];
          VSCODE_CODELLDB = "${pkgs.vscode-extensions.vadimcn.vscode-lldb}";
        };

      defaultPackage = (pkgs.makeRustPlatform {
        inherit (fenix.packages.${system}.minimal) cargo rustc;
      }).buildRustPackage {
        pname = "tri";
        version = "0.1.0";
        src = ./.;
        cargoSha256 = nixpkgs.lib.fakeSha256;
      };
    });
}
