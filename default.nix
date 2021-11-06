import ./common.nix {
  action = "build";
  extraToolchainComponents = [ "rustfmt" ];
  toolchainTargets = (targets: {
    targets = [ targets.x86_64-linux-musl ];
    defaultTarget = targets.x86_64-linux-musl;
  });
}
