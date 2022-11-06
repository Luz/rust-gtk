nixos:
	nix-shell . --command "make -f Makefile"
clean:
	nix-shell . --command "make -f Makefile clean"
