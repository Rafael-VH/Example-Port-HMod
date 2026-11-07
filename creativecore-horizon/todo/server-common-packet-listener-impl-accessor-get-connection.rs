// Not ported yet: the Java @Accessor on ServerCommonPacketListenerImpl.connection (ServerCommonPacketListenerImplAccessor.getConnection).
// Why it's off: No Horizon function mapped for ServerCommonPacketListenerImpl.connection (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ServerCommonPacketListenerImpl.connection: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerCommonPacketListenerImpl.connection -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/ServerCommonPacketListenerImplAccessor.java:10:
// | @Accessor
// | Connection getConnection();
