{
  description = "Logos zcash_wallet_cli: headless Zcash wallet sessions for logosctl.";

  inputs = {
    logos-module-builder.url = "github:logos-co/logos-module-builder";
    zcash_wallet_backend = {
      url = "github:logos-co/logos-zcash-wallet-backend";
      inputs.logos-module-builder.follows = "logos-module-builder";
    };
  };

  outputs = inputs@{ self, logos-module-builder, ... }:
    let
      nixpkgs = logos-module-builder.inputs.nixpkgs;
      systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems f;
    in
    {
      packages = forAllSystems (system:
        (logos-module-builder.lib.mkLogosModule {
          src = ./.;
          configFile = ./metadata.json;
          flakeInputs = inputs;
        }).packages.${system});
    };
}
