let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { 
    overlays = [ rust_overlay];
  };
  rustVersion = "latest";
  # rustVersion = "1.62.0";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
    ];
    targets = [
      "x86_64-pc-windows-gnu"
    ];
  };
in
(pkgs.buildFHSEnv {
  name = "money_counter_dev";
  
  targetPkgs = pkgs: [
    rust
  ] ++ (with pkgs; [
    zlib
    gcc
    pkgsCross.mingwW64.stdenv.cc
    wayland
    xorg.libX11
    libxkbcommon
    pkg-config
  ]) ++ (with pkgs.xorg; [
    libxcb
    libXcursor
    libXrandr
    libXi
  ]);

  profile = with pkgs;
  ''
    export RUST_BACKTRACE=1
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L native=${pkgsCross.mingwW64.windows.pthreads}/lib"
    export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${lib.makeLibraryPath ([libGL libGLU])}
    # export PKG_CONFIG_PATH="${gtk3.dev}/lib/pkgconfig:${glib.dev}/lib/pkgconfig:${cairo.dev}/lib/pkgconfig:${pango.dev}/lib/pkgconfig:${harfbuzz.dev}/lib/pkgconfig:${gdk-pixbuf.dev}/lib/pkgconfig:${atk.dev}/lib/pkgconfig"
  '';
  

  runScript = "${pkgs.writeShellScriptBin "dev_env" ''
    tmux new-session -d -t money_counter_dev

    tmux split-window -h -t money_counter_dev
    tmux resize-pane -t money_counter_dev:0.1 -x 20%

    tmux send-keys -t money_counter_dev:0 'bash' C-m

    tmux send-keys -t money_counter_dev:0.0 'hx' C-m

    tmux attach-session -t money_counter_dev

    while tmux has-session -t money_counter_dev; do sleep 1; done
    exit
  ''}/bin/dev_env";
}).env
