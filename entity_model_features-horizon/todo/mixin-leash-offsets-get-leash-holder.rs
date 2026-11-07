// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on EntityRenderer.extractRenderState (MixinLeashOffsets.getLeashHolder).
// Why it's off: No Horizon function mapped for EntityRenderer.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for EntityRenderer.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderer.extractRenderState -> (not mapped yet)
//   Vec3.add -> (not mapped yet)
//   Vec3.equals -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinLeashOffsets.java:24:
// | @ModifyExpressionValue(
// |    method = "extractRenderState",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/Leashable;getLeashHolder()Lnet/minecraft/world/entity/Entity;")
// | )
// | private Entity getLeashHolder(Entity original, @Share("holder") LocalRef<Entity> roper) {
// |    roper.set(original);
// |    return original;
// | }
