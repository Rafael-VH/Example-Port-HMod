// Not ported yet: the Java @WrapOperation(INVOKE) on EndPortalBlock.getPortalDestination (EndPortalBlockMixin.be_changeSpawnInEnd).
// Why it's off: No Horizon function mapped for EndPortalBlock.getPortalDestination (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for EndPortalBlock.getPortalDestination: run `match`.
// Note: The call it hooks (Vec3.atBottomCenterOf) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerLevel.END_SPAWN_POINT -> (not mapped yet)
//   EndPortalBlock.getPortalDestination -> (not mapped yet)
//   Vec3.atBottomCenterOf -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/EndPortalBlockMixin.java:16:
// | @WrapOperation(
// |    method = "getPortalDestination",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/phys/Vec3;atBottomCenterOf(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/phys/Vec3;")
// | )
// | Vec3 be_changeSpawnInEnd(Vec3i pos, Operation<Vec3> original) {
// |    if (GeneratorOptions.changeSpawn() && pos == ServerLevel.END_SPAWN_POINT) {
// |       BlockPos spawn = GeneratorOptions.getSpawn();
// |       return (Vec3)original.call(new Object[]{spawn});
// |    } else {
// |       return (Vec3)original.call(new Object[]{pos});
// |    }
// | }
