// Not ported yet: the Java @Inject(INVOKE) on EnvironmentAttributeSystem.addDynamicLayers (EnvironmentAttributeSystemMixin.applyStructureAttributes).
// Why it's off: No Horizon function mapped for EnvironmentAttributeSystem.addDynamicLayers (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for EnvironmentAttributeSystem.addDynamicLayers: run `match`.
// Note: The call it hooks (DimensionType.timelines) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.value -> (not mapped yet)
//   Registry.listElements -> (not mapped yet)
//   RegistryAccess.lookupOrThrow -> (not mapped yet)
//   Registries.ENVIRONMENT_ATTRIBUTE -> (not mapped yet)
//   AttributeType.stateChangeLerp -> (not mapped yet)
//   EnvironmentAttribute.isPositional -> (not mapped yet)
//   EnvironmentAttribute.type -> (not mapped yet)
//   EnvironmentAttributeMap.get -> (not mapped yet)
//   Entry.applyModifier -> (not mapped yet)
//   EnvironmentAttributeSystem.addDynamicLayers -> (not mapped yet)
//   Builder.addTimeBasedLayer -> (not mapped yet)
//   LerpFunction.apply -> (not mapped yet)
//   Level.registryAccess -> (not mapped yet)
//   DimensionType.timelines -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/client/EnvironmentAttributeSystemMixin.java:21:
// | @Inject(
// |    method = "addDynamicLayers",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/dimension/DimensionType;timelines()Lnet/minecraft/core/HolderSet;")
// | )
// | private static void applyStructureAttributes(Builder builder, Level level, CallbackInfo ci) {
// |    Registry<EnvironmentAttribute<?>> attributeRegistry = level.registryAccess().lookupOrThrow(Registries.ENVIRONMENT_ATTRIBUTE);
// |
// |    for (Holder<EnvironmentAttribute<?>> holder : attributeRegistry.listElements().toList()) {
// |       if (((EnvironmentAttribute)holder.value()).isPositional()) {
// |          addLayer(builder, level, (EnvironmentAttribute)holder.value());
// |       }
// |    }
// | }
