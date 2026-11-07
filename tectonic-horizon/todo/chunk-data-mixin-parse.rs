// Not ported yet: the Java @Inject(HEAD) on SerializableChunkData.parse (ChunkDataMixin.tectonic$parse).
// Why it's off: No Horizon function mapped for SerializableChunkData.parse (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SerializableChunkData.parse: run `match`.
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
// Decompiled Java, dev/worldgen/tectonic/mixin/ChunkDataMixin.java:26:
// | @Inject(method = "parse", at = @At("HEAD"))
// | private static void tectonic$parse(
// |    LevelHeightAccessor heightAccessor, PalettedContainerFactory factory, CompoundTag nbt, CallbackInfoReturnable<SerializableChunkData> cir
// | ) {
// |    if (((Level)heightAccessor).dimension().equals(Level.OVERWORLD)) {
// |       if (!STATUSES_TO_SKIP_BLENDING.contains(ChunkStatus.byName(nbt.getStringOr("Status", "unknown")).toString())) {
// |          if (nbt.getIntOr(Tectonic.BLENDING_KEY, 0) != Tectonic.BLENDING_VERSION) {
// |             int min = 0;
// |             int max = 0;
// |
// |             for (Tag section : nbt.getListOrEmpty("sections")) {
// |                int y = section instanceof IntTag tag ? tag.intValue() : 0;
// |                min = Math.min(y, min);
// |                max = Math.max(y, max);
// |             }
// |
// |             min = Math.min(min, -4);
// |             max = Math.max(max, 20);
// |             CompoundTag blendingData = new CompoundTag();
// |             blendingData.putInt("min_section", min);
// |             blendingData.putInt("max_section", max);
// |             nbt.put("blending_data", blendingData);
// |             nbt.remove("Heightmaps");
// |             nbt.remove("isLightOn");
// |          }
// |       }
// |    }
// | }
