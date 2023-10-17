{
  description = "Rust development environment for dns monitor tool";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
  };

  outputs = { self, nixpkgs, ...}: let 
    system = "x86_64-linux";
  in { 
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in pkgs.mkShell {
      packages = with pkgs; [
        rustc
        cargo
        rust-analyzer
        openssl
        pkg-config
      ];
      shellHook = ''
        cargo --version
      '';
    };
  };
}