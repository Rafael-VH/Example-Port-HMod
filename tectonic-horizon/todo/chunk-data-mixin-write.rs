// Not ported yet: the Java @Inject(RETURN) on SerializableChunkData.write (ChunkDataMixin.tectonic$write).
// Why it's off: No Horizon function mapped for SerializableChunkData.write (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for SerializableChunkData.write: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CompoundTag.<init> -> (not mapped yet)
//   CompoundTag.getIntOr -> (not mapped yet)
//   CompoundTag.getListOrEmpty -> (not mapped yet)
//   CompoundTag.getStringOr -> (not mapped yet)
//   CompoundTag.put -> (not mapped yet)
//   CompoundTag.putInt -> (not mapped yet)
//   CompoundTag.remove -> (not mapped yet)
//   IntTag.intValue -> (not mapped yet)
//   ListTag.iterator -> (not mapped yet)
//   Level.OVERWORLD -> (not mapped yet)
//   Level.dimension -> (not mapped yet)
//   ChunkStatus.byName -> (not mapped yet)
//   ChunkStatus.toString -> (not mapped yet)
//   SerializableChunkData.parse -> (not mapped yet)
//   SerializableChunkData.write -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/ChunkDataMixin.java:55:
// | @Inject(method = "write", at = @At("RETURN"), cancellable = true)
// | private void tectonic$write(CallbackInfoReturnable<CompoundTag> cir) {
// |    if (Tectonic.BLENDING_VERSION != 0) {
// |       CompoundTag data = (CompoundTag)cir.getReturnValue();
// |       data.putInt(Tectonic.BLENDING_KEY, Tectonic.BLENDING_VERSION);
// |       cir.setReturnValue(data);
// |    }
// | }
