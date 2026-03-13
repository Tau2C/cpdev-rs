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

  # LD_LIBRARY_PATH=pkgs.lib.makeLibraryPath buildInputs;

  shellHook = ''
    # export PATH="$HOME/.cargo/bin:$ANDROID_SDK_ROOT/platform-tools:$PATH"
    . $HOME/export-esp.sh
  '';
}
