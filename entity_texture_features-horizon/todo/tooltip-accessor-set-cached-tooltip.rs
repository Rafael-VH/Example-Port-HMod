// Not ported yet: the Java @Accessor on Tooltip.cachedTooltip (TooltipAccessor.setCachedTooltip).
// Why it's off: No Horizon function mapped for Tooltip.cachedTooltip (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Tooltip.cachedTooltip: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Tooltip.cachedTooltip -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/accessor/TooltipAccessor.java:11:
// | @Accessor
// | void setCachedTooltip(List<FormattedCharSequence> var1);
