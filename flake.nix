{
  description = "Rust development environment for LeetCode practice";

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
            cargo-watch
            cargo-edit
            just
          ];

          shellHook = ''
            echo "🦀 Rust LeetCode workspace loaded!"
            echo "Rust version: $(rustc --version)"
            echo ""
            echo "Available commands:"
            echo "  just new <name>    Create new problem"
            echo "  just test <name>   Test specific problem"
            echo "  just test-all      Test all problems"
            echo "  just run <name>    Run specific problem"
            echo "  just watch <name>  Watch and test"
            echo "  just list          List all problems"
          '';
        };
      });
}
