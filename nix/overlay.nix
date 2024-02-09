{
  crane,
  cranix,
  fenix,
}: final: prev: let
  simplemoji = prev.callPackage ./. {inherit crane cranix fenix;};
in {
  simplemoji = simplemoji.packages.default;
}
