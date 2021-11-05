import ./common.nix {
  action = "build";
  extraToolchainComponents = [ "rustfmt" ];
}
