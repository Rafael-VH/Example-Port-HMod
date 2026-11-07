// Not ported yet: the Java @Redirect(INVOKE) on ServerLevel.<init>* (ServerLevelMixin.be_hasEnderDragonFight).
// Why it's off: No Horizon function mapped for ServerLevel.<init>* (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: A redirect is a WRAP_OPERATION that doesn't call `original`.
// Note: Wildcard or unknown selector `<init>*`.
// Note: The call it hooks (DimensionType.hasEnderDragonFight) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.is -> (not mapped yet)
//   Holder.unwrapKey -> (not mapped yet)
//   Identifier.getNamespace -> (not mapped yet)
//   ResourceKey.identifier -> (not mapped yet)
//   ServerLevel.setBlockAndUpdate -> (not mapped yet)
//   ServerLevel.tickPrecipitation -> (not mapped yet)
//   Level.<init> -> (not mapped yet)
//   Blocks.ICE -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   BuiltinDimensionTypes.END -> (not mapped yet)
//   BuiltinDimensionTypes.NETHER -> (not mapped yet)
//   BuiltinDimensionTypes.OVERWORLD -> (not mapped yet)
//   BuiltinDimensionTypes.OVERWORLD_CAVES -> (not mapped yet)
//   DimensionType.hasEnderDragonFight -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/ServerLevelMixin.java:52:
// | @Redirect(method = "<init>*", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/dimension/DimensionType;hasEnderDragonFight()Z"))
// | private boolean be_hasEnderDragonFight(DimensionType dimensionType) {
// |    return !GeneratorOptions.hasDragonFights() && this.dimensionTypeRegistration().is(BuiltinDimensionTypes.END)
// |       ? false
// |       : dimensionType.hasEnderDragonFight();
// | }
