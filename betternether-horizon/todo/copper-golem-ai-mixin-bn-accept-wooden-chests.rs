// Not ported yet: the Java @ModifyArg(INVOKE) on CopperGolemAi.initIdleActivity (CopperGolemAiMixin.bn_acceptWoodenChests).
// Why it's off: No Horizon function mapped for CopperGolemAi.initIdleActivity (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for CopperGolemAi.initIdleActivity: run `match`.
// Note: The call it hooks (TransportItemsBetweenContainers.<init>) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   TransportItemsBetweenContainers.<init> -> (not mapped yet)
//   CopperGolemAi.initIdleActivity -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/CopperGolemAiMixin.java:13:
// | @ModifyArg(
// |    method = "initIdleActivity",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/entity/ai/behavior/TransportItemsBetweenContainers;<init>(FLjava/util/function/Predicate;Ljava/util/function/Predicate;IILjava/util/Map;Ljava/util/function/Consumer;Ljava/util/function/Predicate;)V"
// |    ),
// |    index = 2
// | )
// | private static Predicate<BlockState> bn_acceptWoodenChests(Predicate<BlockState> destination) {
// |    return destination.or(state -> state.is(CommonBlockTags.WOODEN_CHEST));
// | }
