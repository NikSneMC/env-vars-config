{
  pkgs ?
    import <nixpkgs> {
      overlays = [
        (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
      ];
    },
}: let
  packages = with pkgs; [
    (rust-bin.stable.latest.default.override {
      extensions = ["rust-src"];
    })
  ];

  libraries = with pkgs; [
    pkg-config
    openssl
  ];
in
  with pkgs;
    mkShell {
      name = "env-vars";
      buildInputs = packages ++ libraries;

      DIRENV_LOG_FORMAT = "";
      LD_LIBRARY_PATH = "${lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH";
    }
