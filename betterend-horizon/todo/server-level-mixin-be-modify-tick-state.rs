// Not ported yet: the Java @ModifyArg(INVOKE) on ServerLevel.tickPrecipitation (ServerLevelMixin.be_modifyTickState).
// Why it's off: No Horizon function mapped for ServerLevel.tickPrecipitation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for ServerLevel.tickPrecipitation: run `match`.
// Note: The call it hooks (ServerLevel.setBlockAndUpdate) isn't mapped to a Rust function yet.
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
// Decompiled Java, org/betterx/betterend/mixin/common/ServerLevelMixin.java:76:
// | @ModifyArg(
// |    method = "tickPrecipitation",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/server/level/ServerLevel;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z"
// |    )
// | )
// | private BlockState be_modifyTickState(BlockPos pos, BlockState state) {
// |    if (state.is(Blocks.ICE)) {
// |       Identifier biome = ((ResourceKey)this.getBiome(pos).unwrapKey().orElseThrow()).identifier();
// |       if (biome.getNamespace().equals(BetterEnd.MOD_ID)) {
// |          state = ((EmeraldIceBlock)EndDecorBlocks.EMERALD_ICE.block()).defaultBlockState();
// |       }
// |    }
// |
// |    return state;
// | }
