{ edge, edges }:

edge {
  src = ./.;
  edges =  with edges; [ ];
  schema = with edges; ''
    @0x9deaa106a95c1af8;

    struct FsFileError {
        notFound @0 :Text;
    }
  '';
}
