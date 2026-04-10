{
  description = "Neote development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            clang
            lld
          ];

          buildInputs = with pkgs; [
            # Rust toolchain
            rustc
            cargo
            rustfmt
            clippy

            # System libraries
            # Wayland
            wayland
            libxkbcommon
            wayland-protocols
            # X11 fallback
            libX11
            libXcursor
            libXi
            libXrandr
            # Graphics
            libglvnd
            vulkan-loader
            # Fonts
            fontconfig
            freetype
            expat
            # Other
            openssl
            # For file dialogs on Wayland
            xdg-desktop-portal
            xdg-desktop-portal-gtk
            xdg-desktop-portal-wlr
            # For rfd - GTK backend
            glib
            gtk3
            pango
            atk
            cairo
            gdk-pixbuf
            # DBus for portals
            dbus
          ];

          # Environment variables
          env = {
            # Don't force any backend - let winit choose the appropriate one
            # This allows both X11 and Wayland to work
            # Ensure linker can find libraries
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
              # Wayland dependencies
              wayland
              libxkbcommon
              wayland-protocols
              # X11 fallback dependencies
              libX11
              libXcursor
              libXi
              libXrandr
              # Graphics
              libglvnd
              vulkan-loader
              # Fonts
              fontconfig
              freetype
              expat
              # GTK for file dialogs
              glib
              gtk3
              pango
              atk
              cairo
              gdk-pixbuf
              # DBus
              dbus
              # Other
              openssl
            ];
            # For Wayland file dialogs
            XDG_CURRENT_DESKTOP = "sway";  # or "gnome", "kde", etc.
            GTK_USE_PORTAL = "1";
          };

          shellHook = ''
            echo "Neote development environment"
            echo "Run: cargo run --bin desktop"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "neote";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
          ];

          buildInputs = with pkgs; [
            libxkbcommon
            fontconfig
            freetype
            expat
            libglvnd
            libX11
            libXcursor
            libXi
            libXrandr
            vulkan-loader
            wayland
            openssl
          ];

          # Don't force any backend - let winit choose the appropriate one
          # This allows both X11 and Wayland to work
        };
      }
    );
}
