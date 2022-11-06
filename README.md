# rust-gtk
A template for GTK4 projects in Rust.

## Dependencies
- gtk4

## Building

### Nixos
Create default.nix
```Shell
with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "env";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    pkg-config cairo atk gio-sharp gtk4 gdk-pixbuf pango
  ];
}
```
Then build with cargo
```Shell
nix-shell . --command "cargo build"
```
