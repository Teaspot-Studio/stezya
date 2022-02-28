with import ./nix/pkgs.nix {};

stdenv.mkDerivation rec {
  name = "stezya-rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    rust-analyzer
    SDL2
    SDL2_image
    SDL2_mixer
  ];
}
