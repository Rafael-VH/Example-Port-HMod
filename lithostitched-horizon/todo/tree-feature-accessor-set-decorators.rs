// Not ported yet: the Java @Accessor on TreeFeature.decorators (TreeFeatureAccessor.lithostitched$setDecorators).
// Why it's off: No Horizon function mapped for TreeFeature.decorators (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for TreeFeature.decorators: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   TreeFeature.decorators -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/TreeFeatureAccessor.java:12:
// | @Accessor("decorators")
// | @Mutable
// | void lithostitched$setDecorators(List<TreeDecorator> var1);
