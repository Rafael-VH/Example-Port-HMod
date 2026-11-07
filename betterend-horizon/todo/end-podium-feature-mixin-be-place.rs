// Not ported yet: the Java @Inject(HEAD) on EndPodiumFeature.place (EndPodiumFeatureMixin.be_place).
// Why it's off: No Horizon function mapped for EndPodiumFeature.place (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
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
// Decompiled Java, org/betterx/betterend/mixin/common/EndPodiumFeatureMixin.java:34:
// | @Inject(method = "place", at = @At("HEAD"), cancellable = true)
// | private void be_place(WorldGenLevel world, ChunkGenerator generator, RandomSource random, BlockPos origin, CallbackInfoReturnable<Boolean> info) {
// |    if (!GeneratorOptions.hasPortal()) {
// |       info.setReturnValue(false);
// |       info.cancel();
// |    } else if (GeneratorOptions.replacePortal()) {
// |       BlockPos blockPos = this.be_updatePortalPos(world);
// |       String path = this.active ? "portal/end_portal_active" : "portal/end_portal_inactive";
// |       StructureTemplate structure = StructureNBT.readTemplate(BetterEnd.C.mk(path));
// |       Vec3i size = structure.getSize();
// |       blockPos = blockPos.offset(-(size.getX() >> 1), -3, -(size.getZ() >> 1));
// |       structure.placeInWorld(world, blockPos, blockPos, new StructurePlaceSettings(), random, 2);
// |       info.setReturnValue(true);
// |       info.cancel();
// |    }
// | }
