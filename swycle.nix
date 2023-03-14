{lib, fetchFromGitHub, rustPlatform, clippy}:
rustPlatform.buildRustPackage rec {
  pname = "swycle";
  version = "0.1.0";
  src = lib.cleanSource ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  prePatch = ''
      cp -r ../cargo-vendor-dir .
  '';
  doCheck = true;
  nativeBuildInputs = [ clippy ];
  postCheck = ''
    cargo-clippy
  '';

  meta = with lib; {
    description = "Change sway workspaces on a cyclic 2D plane";
    homepage = "https://github.com/sents/swycle";
    license = licenses.agpl3;
    mainProgram = "swycle";
    maintainers = [ {
      email = "finn@krein.moe";
      github = "sents";
      githubId = 26575793;
      name = "Finn Krein";
    } ];
  };
}
