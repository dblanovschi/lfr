import ./common.nix {
  action = "dev";
  buildInputs = pkgs: with pkgs; [ nixpkgs-fmt ];
  toolchainTargets = (targets: {
    targets = [ targets.x86_64-linux-gnu targets.x86_64-linux-musl ];
    defaultTarget = targets.x86_64-linux-musl;
  });
}
