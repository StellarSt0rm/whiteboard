{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gdk-pixbuf
    libadwaita
    pkg-config
    graphene
    pango
    glib
    gtk4
  ];
}
