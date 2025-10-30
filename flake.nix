{
  description = "Outsider API development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
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
            rustToolchain
            cargo
            rustc
            rust-analyzer
            
            # Required for OpenSSL and crypto libraries
            openssl
            pkg-config
            
            # Additional development tools
            git
            curl
            
            # Optional but useful
            cargo-watch
            cargo-edit
          ];

          env = {
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          };

          shellHook = ''
            echo "🦀 Rust development environment loaded!"
            echo "OpenSSL: ${pkgs.openssl.dev}"
            rustc --version
            cargo --version
          '';
        };
      });
}