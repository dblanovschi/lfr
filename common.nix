# action: 'dev' or 'build'
# 'build' will set up a minimal rust toolchain for
# building (used in `nix-build`) while 'dev' will
# setup one suitable for development (`nix-shell`).
# If on vscode, use nix env selector and point it to
# shell.nix.
# buildInputs: any extra build inputs
# maybe some packages nice to have in the shell but
# not necesary for building, for example ripgrep,
# debugger and the like.
{ action
, buildInputs ? (pkgs: [ ])
, extraToolchainComponents ? [ ]
}:

{ utils ? import ./utils.nix
, pkgs ? import <nixpkgs> {
    overlays = [
      (utils.importRepo { user = "oxalica"; repo = "rust-overlay"; branch = "master"; })
    ];
  }
}:

let
  thorConfig = import ./thor-config.nix;
  thor =
    # import ../thor/default.nix
    utils.importRepo { user = "dblanovschi"; repo = "thor"; }
      { inherit pkgs; config = thorConfig; };

  buildInputs' = buildInputs pkgs;
in
with thor.rust.toolchainCommons;
thor.rust.mkRustDerivation {
  inherit action;

  pname = "lfr";
  version = "0.1.0";

  cargoLock = ./Cargo.lock;

  src = builtins.path {
    path = ./.;
    filter = path: type:
      # type != "directory" ||
      ! builtins.any (t: t == builtins.baseNameOf path) [
        "target"
        "result"
        ".vscode"
        ".git"
        ".gitignore"
      ];
  };

  toolchain = "nightly-musl";

  inherit extraToolchainComponents;

  nativeBuildInputs = [ ];

  buildInputs = buildInputs';

  cargoAliases = {
    xtask = "run -p xtask --";
  };

  enableIncremental = true;

  shellAliases = {
    cr = "cargo r";
    crr = "cargo r --release";
    cb = "cargo b";
    cbr = "cargo b --release";
    cf = "cargo fmt -- --emit=files";
  };

  phases = {
    configurePhase = ''
      cargo xtask gen-syntax
    '';

    buildPhase = ''
      cargo build --release
    '';

    checkPhase = ''
      cargo test --release
    '';

    installPhase = ''
      mkdir -p $out/bin
      cp target/x86_64-unknown-linux-musl/release/lfr $out/bin/
    '';
  };
}
