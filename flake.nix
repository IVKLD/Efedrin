{
  description = "Efedrin - Production-Ready PipeWire Soundpad for Linux";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        nativeBuildInputs = with pkgs; [
          pkg-config
          cargo
          rustc
          rustfmt
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

        runtimeDeps = with pkgs; [
          pipewire
          pulseaudio
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "efedrin";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          postInstall = ''
            mkdir -p $out/share/applications $out/share/icons/hicolor/scalable/apps
            if [ -f resources/efedrin.desktop ]; then
              cp resources/efedrin.desktop $out/share/applications/
            fi
            if [ -f resources/efedrin.svg ]; then
              cp resources/efedrin.svg $out/share/icons/hicolor/scalable/apps/
            fi
          '';

          postFixup = ''
            wrapProgram $out/bin/efedrin \
              --prefix PATH : ${pkgs.lib.makeBinPath [
                pkgs.ffmpeg
                pkgs.pipewire
                pkgs.wireplumber
                pkgs.which
                pkgs.xdg-utils
              ]}
          '';

          meta = with pkgs.lib; {
            description = "PipeWire Soundpad for Linux with dual-routing and virtual microphone support";
            homepage = "https://github.com/IVKLD/Efedrin";
            license = licenses.mit;
            platforms = platforms.linux;
            mainProgram = "efedrin";
          };
        };

        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath (buildInputs ++ runtimeDeps)}:$LD_LIBRARY_PATH"
          '';
        };
      }
    ) // {
      overlays.default = final: prev: {
        efedrin = self.packages.${prev.stdenv.hostPlatform.system}.default;
      };

      nixosModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.programs.efedrin;
        in
        {
          options.programs.efedrin = {
            enable = lib.mkEnableOption "Efedrin PipeWire Soundpad";
            package = lib.mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "default" { };
          };

          config = lib.mkIf cfg.enable {
            environment.systemPackages = [ cfg.package ];
            security.polkit.enable = lib.mkDefault true;
            services.pipewire.enable = lib.mkDefault true;
          };
        };
    };
}
