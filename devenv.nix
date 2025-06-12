{pkgs, ...}: {
  packages = [pkgs.openssl];

  languages.rust.enable = true;
}
