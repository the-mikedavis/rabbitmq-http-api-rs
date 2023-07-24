{ pkgs }:

pkgs.mkShell {
  buildInputs = with pkgs; [openssl];
  nativeBuildInputs = with pkgs; [
    (rust-bin.stable.latest.default.override {
      extensions = ["rustfmt" "rust-src" "rust-analyzer" "clippy"];
    })
    pkg-config
  ];
  RUST_BACKTRACE = "1";
}
