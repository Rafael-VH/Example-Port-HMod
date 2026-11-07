// Not ported yet: the Java @WrapOperation(INVOKE) on Mob.addAdditionalSaveData (MixinMob_LeashNBTFix.etf$catchLeashCrash).
// Why it's off: No Horizon function mapped for Mob.addAdditionalSaveData (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for Mob.addAdditionalSaveData: run `match`.
// Note: The call it hooks (Mob.writeLeashData) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LeashData.delayedLeashInfo -> (not mapped yet)
//   LeashData.leashHolder -> (not mapped yet)
//   Mob.addAdditionalSaveData -> (not mapped yet)
//   Mob.writeLeashData -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinMob_LeashNBTFix.java:13:
// | @WrapOperation(
// |    method = "addAdditionalSaveData",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/entity/Mob;writeLeashData(Lnet/minecraft/world/level/storage/ValueOutput;Lnet/minecraft/world/entity/Leashable$LeashData;)V"
// |    )
// | )
// | private void etf$catchLeashCrash(Mob instance, ValueOutput valueOutput, LeashData leashData, Operation<Void> original) {
// |    try {
// |       original.call(new Object[]{instance, valueOutput, leashData});
// |    } catch (Exception e) {
// |       if (leashData.leashHolder != null || leashData.delayedLeashInfo != null) {
// |          throw e;
// |       }
// |    }
// | }
