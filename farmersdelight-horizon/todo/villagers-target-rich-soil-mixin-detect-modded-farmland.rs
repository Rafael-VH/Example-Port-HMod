// Not ported yet: the Java @WrapOperation(INVOKE) on SecondaryPoiSensor.doTick (VillagersTargetRichSoilMixin.detectModdedFarmland).
// Why it's off: No Horizon function mapped for SecondaryPoiSensor.doTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for SecondaryPoiSensor.doTick: run `match`.
// Note: The call it hooks (ImmutableSet.contains) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ImmutableSet.contains -> (not mapped yet)
//   Holder.is -> (not mapped yet)
//   SecondaryPoiSensor.doTick -> (not mapped yet)
//   Villager.getVillagerData -> (not mapped yet)
//   VillagerData.profession -> (not mapped yet)
//   VillagerProfession.FARMER -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/VillagersTargetRichSoilMixin.java:17:
// | @WrapOperation(
// |    method = "doTick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/npc/villager/Villager;)V",
// |    at = @At(value = "INVOKE", target = "Lcom/google/common/collect/ImmutableSet;contains(Ljava/lang/Object;)Z")
// | )
// | public boolean detectModdedFarmland(ImmutableSet<Block> instance, Object o, Operation<Boolean> original, @Local(argsOnly = true) Villager body) {
// |    return !body.getVillagerData().profession().is(VillagerProfession.FARMER)
// |       ? (Boolean)original.call(new Object[]{instance, o})
// |       : (Boolean)original.call(new Object[]{instance, o}) || o instanceof FarmlandBlock;
// | }
