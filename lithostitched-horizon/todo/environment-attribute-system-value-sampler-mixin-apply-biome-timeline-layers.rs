// Not ported yet: the Java @WrapOperation(INVOKE) on ValueSampler.computeValuePositional (EnvironmentAttributeSystemValueSamplerMixin.applyBiomeTimelineLayers).
// Why it's off: No Horizon function mapped for ValueSampler.computeValuePositional (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for ValueSampler.computeValuePositional: run `match`.
// Note: The call it hooks (EnvironmentAttribute.sanitizeValue) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EnvironmentAttribute.sanitizeValue -> (not mapped yet)
//   ValueSampler.computeValueNotPositional -> (not mapped yet)
//   ValueSampler.computeValuePositional -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/timeline/EnvironmentAttributeSystemValueSamplerMixin.java:36:
// | @WrapOperation(
// |    method = "computeValuePositional",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/attribute/EnvironmentAttribute;sanitizeValue(Ljava/lang/Object;)Ljava/lang/Object;")
// | )
// | private Value applyBiomeTimelineLayers(
// |    EnvironmentAttribute<Value> attribute,
// |    Value result,
// |    Operation<Value> operation,
// |    @Local(ordinal = 0) Vec3 pos,
// |    @Local(ordinal = 0) SpatialAttributeInterpolator biomeInterpolator
// | ) {
// |    for (EnvironmentAttributeLayer<Value> layer : this.layers.stream().filter(layerx -> layerx instanceof PositionalTimeBasedLayer).toList()) {
// |       result = (Value)((PositionalTimeBasedLayer)layer).applyPositionalTimeBased(result, this.cacheTickId, pos, biomeInterpolator);
// |    }
// |
// |    return (Value)operation.call(new Object[]{attribute, result});
// | }
