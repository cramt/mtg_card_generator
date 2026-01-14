{
  description = "MTG Card Generator - CLI tool for generating Magic: The Gathering card images";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            
            # Build dependencies
            pkg-config
            openssl
            
            # Chromium for rendering
            chromium
            
            # Development tools
            cargo-watch
            cargo-edit
          ];

          shellHook = ''
            export CHROME_PATH="${pkgs.chromium}/bin/chromium"
            echo "MTG Card Generator development environment"
            echo "Chromium available at: $CHROME_PATH"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "mtg-gen";
          version = "0.1.0";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          meta = with pkgs.lib; {
            description = "CLI tool for generating Magic: The Gathering card images from YAML definitions";
            license = licenses.mit;
          };
        };
      }
    );
}
