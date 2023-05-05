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
      let pkgs = nixpkgs.legacyPackages.${system}; in 
      {
        devShells.default =
          pkgs.mkShell {
            buildInputs = [
              pkgs.cargo
              pkgs.rustc
              pkgs.rust-analyzer
              pkgs.vscode-extensions.vadimcn.vscode-lldb
              pkgs.imagemagick
              (pkgs.writeScriptBin "tri" "./target/debug/tri $@")
            ];
            VSCODE_CODELLDB = "${pkgs.vscode-extensions.vadimcn.vscode-lldb}";
          };

        packages.default = (pkgs.makeRustPlatform {
          inherit (fenix.packages.${system}.minimal) cargo rustc;
        }).buildRustPackage {
          pname = "tri";
          version = "0.0.1";
          src = ./.;

          nativeBuildInputs = [
            pkgs.installShellFiles
          ];

          # TODO: add elvish and powershell
          postInstall = ''
            installManPage ./artifacts/tri.1
            installShellCompletion ./artifacts/_tri
            installShellCompletion ./artifacts/tri.bash
            installShellCompletion ./artifacts/tri.fish
          '';

          # cargoSha256 = "";
          cargoSha256 = "sha256-i/uT0q81Fhf8OK0oDI4w3zx5W1zS3shCwVvvK/bOxko=";
          meta = with pkgs.lib; {
            homepage = "https://github.com/WhiteBlackGoose/tree-imagemagick-editor";
            description = "Graphic editor with immutable and reproducible changes/transformations, based on imagemagick and inspired by git and nix";
            platforms = platforms.all;
            maintainers = with maintainers; [ WhiteBlackGoose ];
            license = licenses.gpl3Plus;
            mainProgram = "tri";
          };
        };
    });
}
