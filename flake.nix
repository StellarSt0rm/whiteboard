{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    devShells."${system}".default = pkgs.mkShell {
      packages = with pkgs; [
        gdk-pixbuf
        libadwaita
        pkg-config
        graphene
        pango
        glib
        gtk4
      ];
    };
  };
}
