{
  # This comment and spacing must survive.
  services.nginx.enable    = true;

  packages.${system} = pkgs.hello;
  message = "package: ${pkgs.hello}";
  nested = { answer = 42; };
  script = ''
    services.fake.enable = true;
  '';
}
