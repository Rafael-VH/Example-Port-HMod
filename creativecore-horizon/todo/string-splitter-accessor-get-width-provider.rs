// Not ported yet: the Java @Accessor on StringSplitter.widthProvider (StringSplitterAccessor.getWidthProvider).
// Why it's off: No Horizon function mapped for StringSplitter.widthProvider (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for StringSplitter.widthProvider: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   StringSplitter.widthProvider -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/StringSplitterAccessor.java:10:
// | @Accessor
// | WidthProvider getWidthProvider();
