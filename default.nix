{
  pkgs ? import <nixpkgs> {
    # config = {
    #   allowUnfree = true;
    # };
  },
}:

pkgs.mkShell {
  ANDROID_SDK_ROOT = "/home/tau2c/Android/Sdk";
  packages = [
    pkgs.typst
    # pkgs.cargo-tmp
    pkgs.rustup
    pkgs.espup
    pkgs.esp-generate
    pkgs.espflash
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.expat
    pkgs.fontconfig
    pkgs.freetype
    pkgs.freetype.dev
    pkgs.libGL
    pkgs.pkg-config
    pkgs.libx11
    pkgs.libxcursor
    pkgs.libxi
    pkgs.libxrandr
    pkgs.wayland
    pkgs.libxkbcommon
  ];

  shellHook = ''
    export PATH="$HOME/.cargo/bin:$PATH"
    source $HOME/export-esp.sh
  '';
}
