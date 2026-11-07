// Not ported yet: the Java @Inject(HEAD) on WoodlandMansionPiece.makeLocation (WoodlandMansionPieceMixin.convertStringToId).
// Why it's off: No Horizon function mapped for WoodlandMansionPiece.makeLocation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for WoodlandMansionPiece.makeLocation: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Identifier.parse -> (not mapped yet)
//   WoodlandMansionPiece.makeLocation -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/mansion/WoodlandMansionPieceMixin.java:12:
// | @Inject(method = "makeLocation", at = @At("HEAD"), cancellable = true)
// | private static void convertStringToId(String string, CallbackInfoReturnable<Identifier> cir) {
// |    if (string.contains(":")) {
// |       cir.setReturnValue(Identifier.parse(string));
// |    }
// | }
