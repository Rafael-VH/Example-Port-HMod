// Not ported yet: the Java @Invoker on TypeSettings.codec (RecipeBookSettingsTypeSettingsInvoker.fdrf$invokeCodec).
// Why it's off: No Horizon function mapped for TypeSettings.codec (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for TypeSettings.codec: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   TypeSettings.codec -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/RecipeBookSettingsTypeSettingsInvoker.java:10:
// | @Invoker("codec")
// | static MapCodec<TypeSettings> fdrf$invokeCodec(String openField, String filteringField) {
// |    throw new RuntimeException("");
// | }
