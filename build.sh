carnix generate-nix --standalone --src .
nix-build Cargo.nix -A nux
nix-env -i ./result
