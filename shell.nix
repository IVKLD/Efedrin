{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
    rust-analyzer
    wrapGAppsHook4
  ];

  buildInputs = with pkgs; [
    gtk4
    libadwaita
    glib
    alsa-lib
    pipewire
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.gtk4 pkgs.libadwaita pkgs.glib pkgs.pipewire pkgs.alsa-lib ]}:$LD_LIBRARY_PATH"
  '';
}
