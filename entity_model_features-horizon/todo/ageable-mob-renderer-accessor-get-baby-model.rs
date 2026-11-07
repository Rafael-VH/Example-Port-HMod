// Not ported yet: the Java @Accessor on AgeableMobRenderer.babyModel (AgeableMobRendererAccessor.getBabyModel).
// Why it's off: No Horizon function mapped for AgeableMobRenderer.babyModel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for AgeableMobRenderer.babyModel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AgeableMobRenderer.babyModel -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/accessor/AgeableMobRendererAccessor.java:11:
// | @Accessor
// | <S extends LivingEntityRenderState, M extends EntityModel<? super S>> M getBabyModel();
