with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "env";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
#    pkg-config cairo atk gio-sharp gtk3-x11 gdk-pixbuf pango
    pkg-config cairo atk gio-sharp gtk4 gdk-pixbuf pango
  ];
}
