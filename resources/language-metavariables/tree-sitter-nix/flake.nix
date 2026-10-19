{
  description = "tree-sitter-nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    nix-github-actions.url = "github:nix-community/nix-github-actions";
    nix-github-actions.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, nix-github-actions }: (
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;

      in
      {
        checks =
          let
            # shellPackages = (pkgs.callPackage ./shell.nix { }).packages;

            # If the generated code differs from the checked in we need
            # to check in the newly generated sources.
            mkCheck = name: check: pkgs.runCommand name
              {
                inherit (self.devShells.${system}.default) nativeBuildInputs;
              } ''
              cp -rv ${self} src
              chmod +w -R src
              cd src

              ${check}

              touch $out
            '';

          in
          {
            build = self.packages.${system}.tree-sitter-nix;

            editorconfig = mkCheck "editorconfig" "editorconfig-checker";

            # If the generated code differs from the checked in we need
            # to check in the newly generated sources.
            generated-diff = mkCheck "generated-diff" ''
              HOME=. npm run generate
              diff -r src/ ${self}/src
            '';

            treefmt = mkCheck "treefmt" "treefmt --no-cache --fail-on-change";

            rust-bindings =
              let
                cargo' = lib.importTOML ./Cargo.toml;
              in
              pkgs.rustPlatform.buildRustPackage {
                pname = cargo'.package.name;
                inherit (cargo'.package) version;
                src = self;
                cargoLock = {
                  lockFile = ./Cargo.lock;
                };
              };

          } // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
            # Requires xcode
            node-bindings =
              let
                package' = lib.importJSON ./package.json;
              in
              pkgs.stdenv.mkDerivation {
                pname = package'.name;
                inherit (package') version;
                src = self;
                nativeBuildInputs = with pkgs; [
                  importNpmLock.hooks.npmConfigHook
                  nodejs
                  nodejs.passthru.python # for node-gyp
                  npmHooks.npmBuildHook
                  npmHooks.npmInstallHook
                  tree-sitter
                ];
                npmDeps = pkgs.importNpmLock {
                  npmRoot = ./.;
                };
                buildPhase = ''
                  runHook preBuild
                  ${pkgs.nodePackages.node-gyp}/bin/node-gyp configure
                  npm run build
                  runHook postBuild
                '';
                installPhase = "touch $out";
              };
          };

        packages.tree-sitter-nix = pkgs.callPackage ./default.nix { src = self; };
        packages.default = self.packages.${system}.tree-sitter-nix;
        devShells.default = pkgs.callPackage ./shell.nix { };

        formatter = pkgs.writeShellScriptBin "tree-sitter-nix-fmt" ''
          exec ${pkgs.treefmt}/bin/treefmt --config-file ${./treefmt.toml} "$@"
        '';

      })) // {

      githubActions = nix-github-actions.lib.mkGithubMatrix {
        # Inherit GHA actions matrix from a subset of platforms supported by hosted runners
        checks = {
          inherit (self.checks) x86_64-linux;

          # Don't run linters on darwin as it's just scheduling overhead
          x86_64-darwin = builtins.removeAttrs self.checks.x86_64-darwin [ "editorconfig" "generated-diff" "treefmt" ];
        };
      };

    }
  );
}
