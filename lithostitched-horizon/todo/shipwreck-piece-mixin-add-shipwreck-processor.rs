// Not ported yet: the Java @ModifyReturnValue(RETURN) on ShipwreckPiece.makeSettings (ShipwreckPieceMixin.addShipwreckProcessor).
// Why it's off: No Horizon function mapped for ShipwreckPiece.makeSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for ShipwreckPiece.makeSettings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ShipwreckPiece.makeSettings -> (not mapped yet)
//   StructurePlaceSettings.addProcessor -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/ShipwreckPieceMixin.java:13:
// | @ModifyReturnValue(method = "makeSettings", at = @At("RETURN"))
// | private static StructurePlaceSettings addShipwreckProcessor(StructurePlaceSettings settings) {
// |    return Lithostitched.breaksSeedParity() ? settings.addProcessor(UnboundReferenceProcessor.of("shipwreck")) : settings;
// | }
