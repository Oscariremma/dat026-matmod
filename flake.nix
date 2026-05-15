{
	description = "Development and build flake for dat026-matmod";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs {
					inherit system;
				};

				nativeBuildInputs = with pkgs; [
					cargo
					clippy
					pkg-config
					rust-analyzer
					rustc
					rustfmt
				];

				buildInputs = with pkgs; [
					alsa-lib
					libxkbcommon
					libx11
					libxcursor
					libxi
					libxrandr
					udev
					vulkan-loader
					wayland
				];
			in {
				packages.default = pkgs.rustPlatform.buildRustPackage {
					pname = "balls";
					version = "0.1.0";
					src = ./.;

					cargoLock = {
						lockFile = ./Cargo.lock;
					};

					inherit nativeBuildInputs buildInputs;
				};

				devShells.default = pkgs.mkShell {
					inherit nativeBuildInputs buildInputs;

					LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
					RUST_BACKTRACE = "1";
				};

				apps.default = flake-utils.lib.mkApp {
					drv = self.packages.${system}.default;
				};
			});
}
