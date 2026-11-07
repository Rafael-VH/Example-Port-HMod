// Not ported yet: the Java @ModifyReturnValue(RETURN) on WoodlandMansionPiece.makeSettings (WoodlandMansionPieceMixin.addShipwreckProcessor).
// Why it's off: No Horizon function mapped for WoodlandMansionPiece.makeSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for WoodlandMansionPiece.makeSettings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WoodlandMansionPiece.makeSettings -> (not mapped yet)
//   StructurePlaceSettings.addProcessor -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/WoodlandMansionPieceMixin.java:13:
// | @ModifyReturnValue(method = "makeSettings", at = @At("RETURN"))
// | private static StructurePlaceSettings addShipwreckProcessor(StructurePlaceSettings settings) {
// |    return Lithostitched.breaksSeedParity() ? settings.addProcessor(UnboundReferenceProcessor.of("woodland_mansion")) : settings;
// | }
