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
            pkgs.pandoc
          ];

          postInstall = ''
            pandoc --standalone --to man doc/tri.md -o tri.1
            installManPage tri.1
          '';

          # cargoSha256 = "sha256-3M25zF3TiPAdnmf1rxb+xUdBFEGm/LGVg3Xc1lQn5Pk=";
          cargoSha256 = "sha256-D0HdbOLePCgbaO3kfPPg8NLRvi1XYfOla6/Clnb01xU=";
          meta = with pkgs.lib; {
            homepage = "https://github.com/WhiteBlackGoose/tree-imagemagick-editor";
            description = "Graphic editor with immutable and reproducible changes/transformations, based on imagemagick and inspired by git and nix";
            platforms = platforms.all;
            maintainers = with maintainers; [ WhiteBlackGoose ];
            license = licenses.gpl3Plus;
            mainProgram = "tree-editor";
          };
        };
    });
}
