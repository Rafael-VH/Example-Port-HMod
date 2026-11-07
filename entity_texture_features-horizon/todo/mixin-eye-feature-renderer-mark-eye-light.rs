// Not ported yet: the Java @ModifyVariable(HEAD) on EyesLayer.submit (MixinEyeFeatureRenderer.emf$markEyeLight).
// Why it's off: No Horizon function mapped for EyesLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for EyesLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EyesLayer.submit -> (not mapped yet)
//   RenderTypes.eyes -> (not mapped yet)
//   Identifier.equals -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/feature/MixinEyeFeatureRenderer.java:39:
// | @ModifyVariable(method = "submit", at = @At("HEAD"), argsOnly = true, ordinal = 0)
// | private int emf$markEyeLight(int i) {
// |    return 15728881;
// | }
