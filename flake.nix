{
  description = "nexum-chrome-sys - Rust bindings for Chrome Extension APIs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            rustToolchain
            cargo-watch
            wasm-pack

            # Node.js / pnpm (for chrome-types)
            nodejs_20
            pnpm

            # Python (required by chrome-types IDL parser)
            python3

            # Utilities
            git
            jq
          ];

          shellHook = ''
            echo "nexum-chrome-sys development environment"
            echo "  Rust: $(rustc --version)"
            echo "  Node: $(node --version)"
            echo "  pnpm: $(pnpm --version)"
            echo "  Python: $(python3 --version)"
          '';
        };
      }
    );
}
