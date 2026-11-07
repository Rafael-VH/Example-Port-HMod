// Not ported yet: the Java @ModifyReturnValue(RETURN) on OceanRuinPiece.makeSettings (OceanRuinPieceMixin.addShipwreckProcessor).
// Why it's off: No Horizon function mapped for OceanRuinPiece.makeSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for OceanRuinPiece.makeSettings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   OceanRuinPiece.makeSettings -> (not mapped yet)
//   Type.getName -> (not mapped yet)
//   StructurePlaceSettings.addProcessor -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/OceanRuinPieceMixin.java:15:
// | @ModifyReturnValue(method = "makeSettings", at = @At("RETURN"))
// | private static StructurePlaceSettings addShipwreckProcessor(StructurePlaceSettings settings, Rotation rotation, float f, Type type) {
// |    return Lithostitched.breaksSeedParity() ? settings.addProcessor(UnboundReferenceProcessor.of("ocean_ruin_" + type.getName())) : settings;
// | }
