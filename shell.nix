import ./common.nix {
  action = "dev";
  buildInputs = pkgs: with pkgs; [ nixpkgs-fmt ];
}
