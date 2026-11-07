// Not ported yet: the Java @ModifyExpressionValue(NEW) on ResourceManagerRegistryLoadTask.lambda$load$2 (ResourceManagerRegistryLoadTaskMixin.load).
// Why it's off: No Horizon function mapped for ResourceManagerRegistryLoadTask.lambda$load$2 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for ResourceManagerRegistryLoadTask.lambda$load$2: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PendingRegistration.value -> (not mapped yet)
//   ResourceManagerRegistryLoadTask.lambda$load$2 -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/predicate/ResourceManagerRegistryLoadTaskMixin.java:12:
// | @ModifyExpressionValue(method = "lambda$load$2", at = @At(value = "NEW", target = "net/minecraft/resources/RegistryLoadTask$PendingRegistration"))
// | private PendingRegistration<?> load(PendingRegistration<?> original) {
// |    return original != null && (!original.value().right().isPresent() || !(original.value().right().get() instanceof StubException)) ? original : null;
// | }
