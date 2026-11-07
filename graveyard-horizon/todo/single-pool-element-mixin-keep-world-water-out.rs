// Not ported yet: the Java @Inject(RETURN) on SinglePoolElement.getSettings (SinglePoolElementMixin.graveyard$keepWorldWaterOut).
// Why it's off: No Horizon function mapped for SinglePoolElement.getSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for SinglePoolElement.getSettings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.isBound -> (not mapped yet)
//   Holder.value -> (not mapped yet)
//   SinglePoolElement.getSettings -> (not mapped yet)
//   SinglePoolElement.getShuffledJigsawBlocks -> (not mapped yet)
//   LiquidSettings.IGNORE_WATERLOGGING -> (not mapped yet)
//   StructurePlaceSettings.setLiquidSettings -> (not mapped yet)
//   StructureProcessorList.list -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/SinglePoolElementMixin.java:50:
// | @Inject(method = "getSettings", at = @At("RETURN"))
// | private void graveyard$keepWorldWaterOut(
// |    Rotation rotation, BoundingBox box, LiquidSettings liquidSettings, boolean keepJigsaws, CallbackInfoReturnable<StructurePlaceSettings> callback
// | ) {
// |    if (this.processors.isBound()) {
// |       for (StructureProcessor processor : ((StructureProcessorList)this.processors.value()).list()) {
// |          if (processor instanceof RemoveWaterloggedProcessor || processor instanceof RemoveWaterloggedCryptProcessor) {
// |             ((StructurePlaceSettings)callback.getReturnValue()).setLiquidSettings(LiquidSettings.IGNORE_WATERLOGGING);
// |             return;
// |          }
// |       }
// |    }
// | }
