{
  description = "TRI editor project";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, fenix, ... }:
      let 
        systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ]; 
      in {
        devShells = nixpkgs.lib.genAttrs systems (system: 
        let 
          pkgs = nixpkgs.legacyPackages.${system}; in
        {
          default =
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
        });

        packages = nixpkgs.lib.genAttrs systems (system: 
        let 
          pkgs = nixpkgs.legacyPackages.${system}; in
        {
          default = (nixpkgs.legacyPackages.${system}.pkgs.makeRustPlatform {
              inherit (fenix.packages.${system}.minimal) cargo rustc;
            }).buildRustPackage {
            pname = "tri";
            version = "0.0.1";
            src = ./.;

            nativeBuildInputs = [
              pkgs.installShellFiles
              pkgs.makeWrapper
            ];

            buildInputs = [
              pkgs.imagemagick
            ];

            # TODO: add elvish and powershell
            postInstall = ''
              installManPage ./artifacts/tri.1
              installShellCompletion ./artifacts/_tri
              installShellCompletion ./artifacts/tri.bash
              installShellCompletion ./artifacts/tri.fish
              wrapProgram $out/bin/tri --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.imagemagick ]}
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
    };
}
