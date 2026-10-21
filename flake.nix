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

        version = "0.1.0";
        # Binary release settings for x86_64-linux (prebuilt binary from GitHub Releases)
        binVersion = "0.1.0"; # BIN_VERSION
        binHash = "sha256-p4RJVcJ2BEF2f7YWYPb3izfbjrheXtJAESudbSo686Y="; # BIN_HASH

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
          graphene
        ];

        runtimeDeps = with pkgs; [
          pipewire
          pulseaudio
        ];

        commonPostFixup = ''
          wrapProgram $out/bin/efedrin \
            --prefix PATH : ${pkgs.lib.makeBinPath [
              pkgs.ffmpeg
              pkgs.pipewire
              pkgs.wireplumber
              pkgs.which
              pkgs.xdg-utils
            ]}
        '';

        # Build from source via cargo
        efedrin-source = pkgs.rustPlatform.buildRustPackage {
          pname = "efedrin";
          inherit version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          postInstall = ''
            mkdir -p $out/share/applications $out/share/icons/hicolor/scalable/apps $out/share/icons/hicolor/scalable/actions
            if [ -f resources/efedrin.desktop ]; then
              cp resources/efedrin.desktop $out/share/applications/
            fi
            if [ -f resources/efedrin.svg ]; then
              cp resources/efedrin.svg $out/share/icons/hicolor/scalable/apps/
            fi
            if [ -f resources/tag-symbolic.svg ]; then
              cp resources/tag-symbolic.svg $out/share/icons/hicolor/scalable/actions/
              cp resources/tag-symbolic.svg $out/share/icons/hicolor/scalable/apps/
            fi
          '';

          postFixup = commonPostFixup;

          meta = with pkgs.lib; {
            description = "PipeWire Soundpad for Linux with dual-routing and virtual microphone support (built from source)";
            homepage = "https://github.com/IVKLD/Efedrin";
            license = licenses.mit;
            platforms = platforms.linux;
            mainProgram = "efedrin";
          };
        };

        # Prebuilt binary package with autoPatchelf (instant install without building)
        efedrin-bin = pkgs.stdenv.mkDerivation rec {
          pname = "efedrin-bin";
          version = binVersion;

          src = pkgs.fetchurl {
            url = "https://github.com/IVKLD/Efedrin/releases/download/v${version}/efedrin-x86_64-linux.tar.gz";
            hash = binHash;
          };

          sourceRoot = ".";

          nativeBuildInputs = with pkgs; [
            autoPatchelfHook
            wrapGAppsHook4
            makeWrapper
          ];

          buildInputs = with pkgs; [
            gtk4
            libadwaita
            glib
            pipewire
            alsa-lib
            graphene
          ];

          installPhase = ''
            mkdir -p $out/bin $out/share/applications $out/share/icons/hicolor/scalable/apps $out/share/icons/hicolor/scalable/actions
            install -m755 -D efedrin $out/bin/efedrin
            if [ -f efedrin.desktop ]; then
              install -m644 -D efedrin.desktop $out/share/applications/efedrin.desktop
            fi
            if [ -f efedrin.svg ]; then
              install -m644 -D efedrin.svg $out/share/icons/hicolor/scalable/apps/efedrin.svg
            fi
            if [ -f tag-symbolic.svg ]; then
              install -m644 -D tag-symbolic.svg $out/share/icons/hicolor/scalable/actions/tag-symbolic.svg
              install -m644 -D tag-symbolic.svg $out/share/icons/hicolor/scalable/apps/tag-symbolic.svg
            fi
          '';

          postFixup = commonPostFixup;

          meta = with pkgs.lib; {
            description = "PipeWire Soundpad for Linux with dual-routing and virtual microphone support (prebuilt binary)";
            homepage = "https://github.com/IVKLD/Efedrin";
            license = licenses.mit;
            platforms = [ "x86_64-linux" ];
            mainProgram = "efedrin";
          };
        };
      in
      {
        packages = {
          default = efedrin-bin;
          efedrin = efedrin-bin;
          efedrin-bin = efedrin-bin;
          efedrin-source = efedrin-source;
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
        efedrin-bin = self.packages.${prev.stdenv.hostPlatform.system}.efedrin-bin;
        efedrin-source = self.packages.${prev.stdenv.hostPlatform.system}.efedrin-source;
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

      homeManagerModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.programs.efedrin;
        in
        {
          options.programs.efedrin = {
            enable = lib.mkEnableOption "Efedrin PipeWire Soundpad";
            package = lib.mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "default" { };
          };

          config = lib.mkIf cfg.enable {
            home.packages = [ cfg.package ];
          };
        };
    };
}
