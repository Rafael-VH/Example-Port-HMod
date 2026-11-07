// Not ported yet: the Java @Inject(HEAD) on EndSpikeFeature.place (SpikeFeatureMixin.be_place).
// Why it's off: No Horizon function mapped for EndSpikeFeature.place (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EndSpikeFeature.place: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.<init> -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   MutableBlockPos.<init> -> (not mapped yet)
//   MutableBlockPos.set -> (not mapped yet)
//   MutableBlockPos.setX -> (not mapped yet)
//   MutableBlockPos.setY -> (not mapped yet)
//   MutableBlockPos.setZ -> (not mapped yet)
//   Vec3i.getX -> (not mapped yet)
//   Vec3i.getY -> (not mapped yet)
//   Vec3i.getZ -> (not mapped yet)
//   CompoundTag.contains -> (not mapped yet)
//   CompoundTag.getInt -> (not mapped yet)
//   CompoundTag.putInt -> (not mapped yet)
//   Mth.abs -> (not mapped yet)
//   RandomSource.nextFloat -> (not mapped yet)
//   RandomSource.nextInt -> (not mapped yet)
//   EntitySpawnReason.STRUCTURE -> (not mapped yet)
//   EntityType.create -> horizon_client::mobs::finalize::create (suggested, high)
//   EntityTypes.END_CRYSTAL -> (not mapped yet)
//   EndCrystal.setBeamTarget -> (not mapped yet)
//   EndCrystal.setPermanentlyInvulnerable -> (not mapped yet)
//   EndCrystal.snapTo -> (not mapped yet)
//   ServerLevelAccessor.addFreshEntity -> (not mapped yet)
//   ServerLevelAccessor.getBlockState -> (not mapped yet)
//   ServerLevelAccessor.getChunk -> (not mapped yet)
//   ServerLevelAccessor.getLevel -> (not mapped yet)
//   Block.defaultBlockState -> (not mapped yet)
//   Blocks.BEDROCK -> (not mapped yet)
//   Blocks.CRYING_OBSIDIAN -> (not mapped yet)
//   Blocks.IRON_BARS -> (not mapped yet)
//   Blocks.OBSIDIAN -> (not mapped yet)
//   IronBarsBlock.EAST -> (not mapped yet)
//   IronBarsBlock.NORTH -> (not mapped yet)
//   IronBarsBlock.SOUTH -> (not mapped yet)
//   IronBarsBlock.WEST -> (not mapped yet)
//   BlockState.canBeReplaced -> horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested, high)
//   BlockState.setValue -> (not mapped yet)
//   ChunkAccess.getHeight -> (not mapped yet)
//   Types.WORLD_SURFACE -> (not mapped yet)
//   EndSpikeFeature.crystalBeamTarget -> (not mapped yet)
//   ... 11 more: see the report's call list.
//
// Decompiled Java, org/betterx/betterend/mixin/common/SpikeFeatureMixin.java:36:
// | @Inject(method = "place", at = @At("HEAD"), cancellable = true)
// | private void be_place(WorldGenLevel level, ChunkGenerator generator, RandomSource random, BlockPos origin, CallbackInfoReturnable<Boolean> info) {
// |    if (!GeneratorOptions.hasPillars()) {
// |       info.setReturnValue(false);
// |    }
// | }
