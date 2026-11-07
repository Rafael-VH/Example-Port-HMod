// Not ported yet: the Java @ModifyExpressionValue(MIXINEXTRAS:EXPRESSION) on ValueSampler.computeValuePositional (EnvironmentAttributeSystemValueSamplerMixin.removeBiomeTimelineLayers).
// Why it's off: No Horizon function mapped for ValueSampler.computeValuePositional (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for ValueSampler.computeValuePositional: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EnvironmentAttribute.sanitizeValue -> (not mapped yet)
//   ValueSampler.computeValueNotPositional -> (not mapped yet)
//   ValueSampler.computeValuePositional -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/timeline/EnvironmentAttributeSystemValueSamplerMixin.java:29:
// | @ModifyExpressionValue(method = {"computeValuePositional", "computeValueNotPositional"}, at = @At("MIXINEXTRAS:EXPRESSION"))
// | @Definition(id = "layers", field = "Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$ValueSampler;layers:Ljava/util/List;")
// | @Expression("this.layers")
// | private List<EnvironmentAttributeLayer<Value>> removeBiomeTimelineLayers(List<EnvironmentAttributeLayer<Value>> layers) {
// |    return layers.stream().filter(layer -> !(layer instanceof PositionalTimeBasedLayer)).toList();
// | }
