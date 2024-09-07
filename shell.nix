{ pkgs ? import <nixpkgs> {} }:
  let
    overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
    
    libPath = with pkgs; lib.makeLibraryPath [
      libxkbcommon
      libGL
    ];
in
  pkgs.mkShell rec {
    nativeBuildInputs = with pkgs; [
      clang
      cmake
      llvmPackages_19.bintools
      rustup
      vulkan-validation-layers
      kdePackages.extra-cmake-modules
      pkg-config
      wayland-scanner
      kdePackages.wayland-protocols
    ];

    buildInputs = with pkgs; [
      kdePackages.wayland
      libffi
    ];

    RUSTC_VERSION = overrides.toolchain.channel;

    LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_19.libclang.lib ];

    shellHook = ''
      export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
      export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
      '';

    RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') []);

    LD_LIBRARY_PATH = libPath;

    BINDGEN_EXTRA_CLANG_ARGS = (builtins.map (a: ''-I"${a}/include"'') [
      pkgs.glibc.dev
      pkgs.vulkan-headers
    ])
    ++ [
      ''-I"${pkgs.llvmPackages_19.libclang.lib}/lib/clang/${pkgs.llvmPackages_19.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
    ];
  }