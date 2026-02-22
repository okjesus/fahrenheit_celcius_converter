{
  description = "Rust-based project";
   
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";  
    };
    nixpkgs.url = "nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          fenix_pkg = fenix.packages.${system};
        in
          {
	    devShells.default = 
              pkgs.mkShell {
                 name = "rust-based project";
                 packages = with pkgs; [
                   fenix_pkg.stable.toolchain
                 ];
              };
          }
      );
}
