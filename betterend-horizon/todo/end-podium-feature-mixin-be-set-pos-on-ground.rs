// Not ported yet: the Java @ModifyVariable(HEAD) on EndPodiumFeature.place (EndPodiumFeatureMixin.be_setPosOnGround).
// Why it's off: No Horizon function mapped for EndPodiumFeature.place (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for EndPodiumFeature.place: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.<init> -> (not mapped yet)
//   BlockPos.CODEC -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   BlockPos.offset -> (not mapped yet)
//   Vec3i.getX -> (not mapped yet)
//   Vec3i.getZ -> (not mapped yet)
//   CompoundTag.read -> (not mapped yet)
//   CompoundTag.store -> (not mapped yet)
//   WorldGenLevel.getHeight -> (not mapped yet)
//   Types.WORLD_SURFACE -> (not mapped yet)
//   EndPodiumFeature.place -> (not mapped yet)
//   StructurePlaceSettings.<init> -> (not mapped yet)
//   StructureTemplate.getSize -> (not mapped yet)
//   StructureTemplate.placeInWorld -> horizon_world::worldgen::structure::processor::place_in_world (suggested, high)
//
// Decompiled Java, org/betterx/betterend/mixin/common/EndPodiumFeatureMixin.java:51:
// | @ModifyVariable(method = "place", ordinal = 0, at = @At("HEAD"), argsOnly = true)
// | private BlockPos be_setPosOnGround(BlockPos origin, WorldGenLevel world, ChunkGenerator generator, RandomSource random) {
// |    return this.be_updatePortalPos(world);
// | }
