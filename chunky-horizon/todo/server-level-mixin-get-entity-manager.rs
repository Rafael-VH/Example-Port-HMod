// Not ported yet: the Java @Accessor on ServerLevel.entityManager (ServerLevelMixin.getEntityManager).
// Why it's off: No Horizon function mapped for ServerLevel.entityManager (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ServerLevel.entityManager: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerLevel.entityManager -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ServerLevelMixin.java:11:
// | @Accessor
// | PersistentEntitySectionManager<Entity> getEntityManager();
