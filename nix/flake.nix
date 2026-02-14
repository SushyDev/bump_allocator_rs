{
	description = "A minimal C development environment";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		sushy-lib = {
			url = "github:sushydev/nix-lib/main";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, nixpkgs, sushy-lib }: {
		devShells = sushy-lib.forPlatforms sushy-lib.platforms.default (platform:
			let
				pkgs = import nixpkgs { system = platform; };
			in
			{
				default = pkgs.mkShell {
					packages = with pkgs; [
						gcc       # The GNU Compiler Collection
						gnumake   # Make build system
						gdb       # GNU Debugger
						valgrind  # Memory debugging (optional, remove if on macOS arm64)
						clang
						glibc
						libclang
						unzip
						cargo
						rustup
					];
				};
			}
		);
	};
}
