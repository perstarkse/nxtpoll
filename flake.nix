{
  description = "rust backend development environment";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        packages = [ pkgs.bashInteractive pkgs.cargo-lambda pkgs.rustc pkgs.rustup pkgs.awscli2 pkgs.aws-sam-cli pkgs.nodePackages.live-server pkgs.nodePackages.tailwindcss];
        SAM_CLI_BETA_RUST_CARGO_LAMBDA = 1;
        # AWS_ACCESS_KEY_ID = "op://development/private-aws-access-key/credential";
        # AWS_SECRET_ACCESS_KEY =  "op://development/private-aws-secret-access-key/credential";
        AWS_DEFAULT_REGION = "eu-north-1";
        IS_OFFLINE = true;
      };
    });
}
