// Not ported yet: the Java @Inject(HEAD) on MapItem.update (MapMixin.bn_update).
// Why it's off: No Horizon function mapped for MapItem.update (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MapItem.update: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MutableBlockPos.<init> -> (not mapped yet)
//   MutableBlockPos.getX -> (not mapped yet)
//   MutableBlockPos.getZ -> (not mapped yet)
//   MutableBlockPos.set -> (not mapped yet)
//   MutableBlockPos.setY -> (not mapped yet)
//   SectionPos.blockToSectionCoord -> (not mapped yet)
//   Mth.floor -> horizon_client_render::gfx::occlusion::floor (suggested, high)
//   Mth.square -> (not mapped yet)
//   Entity.getX -> `self.native_player()?.x`. The local player only.
//   Entity.getZ -> `self.native_player()?.z`. The local player only.
//   Item.<init> -> content. An item's properties become its content-mod JSON.
//   MapItem.update -> (not mapped yet)
//   Level.dimension -> (not mapped yet)
//   Level.dimensionType -> (not mapped yet)
//   Level.getChunk -> (not mapped yet)
//   Level.getMinY -> horizon_client::game::Dim::min_y (suggested, high)
//   Block.defaultBlockState -> (not mapped yet)
//   Blocks.BEDROCK -> (not mapped yet)
//   BlockState.getFluidState -> (not mapped yet)
//   BlockState.getMapColor -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   LevelChunk.getBlockState -> (not mapped yet)
//   LevelChunk.getHeight -> (not mapped yet)
//   LevelChunk.isEmpty -> (not mapped yet)
//   DimensionType.hasCeiling -> (not mapped yet)
//   Types.WORLD_SURFACE -> (not mapped yet)
//   FluidState.isEmpty -> (not mapped yet)
//   MapColor.NONE -> (not mapped yet)
//   MapColor.WATER -> (not mapped yet)
//   MapColor.getPackedId -> (not mapped yet)
//   Brightness.HIGH -> (not mapped yet)
//   Brightness.LOW -> (not mapped yet)
//   Brightness.NORMAL -> (not mapped yet)
//   MapItemSavedData.centerX -> (not mapped yet)
//   MapItemSavedData.centerZ -> (not mapped yet)
//   MapItemSavedData.checkBanners -> (not mapped yet)
//   MapItemSavedData.dimension -> (not mapped yet)
//   MapItemSavedData.getHoldingPlayer -> (not mapped yet)
//   MapItemSavedData.scale -> (not mapped yet)
//   MapItemSavedData.updateColor -> (not mapped yet)
//   ... 1 more: see the report's call list.
//
// Decompiled Java, org/betterx/betternether/mixin/common/MapMixin.java:42:
// | @Inject(method = "update", at = @At("HEAD"), cancellable = true)
// | private void bn_update(Level level, Entity entity, MapItemSavedData state, CallbackInfo info) {
// |    if (level.dimensionType().hasCeiling() && level.dimension() == state.dimension && entity instanceof Player) {
// |       int scale = 1 << state.scale;
// |       int sx = state.centerX;
// |       int sz = state.centerZ;
// |       int px = Mth.floor(entity.getX() - sx) / scale + 64;
// |       int pz = Mth.floor(entity.getZ() - sz) / scale + 64;
// |       int stepWidth = 128 / scale;
// |       if (level.dimensionType().hasCeiling()) {
// |          stepWidth /= 2;
// |       }
// |
// |       HoldingPlayer holdingPlayer = state.getHoldingPlayer((Player)entity);
// |       holdingPlayer.step++;
// |       MutableBlockPos POS = new MutableBlockPos();
// |       MutableBlockPos POS2 = new MutableBlockPos();
// |       boolean bl = false;
// |
// |       for (int xx = px - stepWidth + 1; xx < px + stepWidth; xx++) {
// |          if ((xx & 15) == (holdingPlayer.step & 15) || bl) {
// |             bl = false;
// |             double d = 0.0;
// |
// |             for (int zz = pz - stepWidth - 1; zz < pz + stepWidth; zz++) {
// |                if (xx >= 0 && zz >= -1 && xx < 128 && zz < 128) {
// |                   int q = Mth.square(xx - px) + Mth.square(zz - pz);
// |                   boolean bl2 = q > (stepWidth - 2) * (stepWidth - 2);
// |                   int x = (sx / scale + xx - 64) * scale;
// |                   int z = (sz / scale + zz - 64) * scale;
// |                   Multiset<MapColor> multiset = LinkedHashMultiset.create();
// |                   LevelChunk levelChunk = level.getChunk(SectionPos.blockToSectionCoord(x), SectionPos.blockToSectionCoord(z));
// |                   if (!levelChunk.isEmpty()) {
// |                      int w = 0;
// |                      double height = 0.0;
// |
// |                      for (int bx = 0; bx < scale; bx++) {
// |                         for (int bz = 0; bz < scale; bz++) {
// |                            POS.set(x + bx, 0, z + bz);
// |                            int testY = levelChunk.getHeight(Types.WORLD_SURFACE, POS.getX(), POS.getZ()) + 1;
// |                            BlockState blockState;
// |                            if (testY <= level.getMinY() + 1) {
// |                               blockState = Blocks.BEDROCK.defaultBlockState();
// |                            } else {
// |                               do {
// |                                  POS.setY(--testY);
// |                                  blockState = levelChunk.getBlockState(POS);
// |                               } while (blockState.is(Blocks.BEDROCK) || blockState.getMapColor(level, POS) != MapColor.NONE && testY > level.getMinY());
// |
// |                               do {
// |                                  POS.setY(--testY);
// |                                  blockState = levelChunk.getBlockState(POS);
// |                               } while (blockState.getMapColor(level, POS) == MapColor.NONE && testY > level.getMinY());
// |
// |                               if (testY > level.getMinY() && !blockState.getFluidState().isEmpty()) {
// |                                  int ab = testY - 1;
// |                                  POS2.set(POS);
// |
// |                                  BlockState blockState2;
// |                                  do {
// |                                     POS2.setY(ab--);
// |                                     blockState2 = levelChunk.getBlockState(POS2);
// |                                     w++;
// |                                  } while (ab > level.getMinY() && !blockState2.getFluidState().isEmpty());
// |
// |                                  blockState = this.getCorrectStateForFluidBlock(level, blockState, POS);
// |                               }
// |                            }
// |
// |                            state.checkBanners(level, POS.getX(), POS.getZ());
// |                            height += (double)testY / (scale * scale);
// |                            multiset.add(blockState.getMapColor(level, POS));
// |                         }
// |                      }
// |
// |                      w /= scale * scale;
// |                      MapColor mapColor = (MapColor)Iterables.getFirst(Multisets.copyHighestCountFirst(multiset), MapColor.NONE);
// |                      Brightness brightness;
// |                      if (mapColor == MapColor.WATER) {
// |                         double f = w * 0.1 + (xx + zz & 1) * 0.2;
// |                         if (f < 0.5) {
// |                            brightness = Brightness.HIGH;
// |                         } else if (f > 0.9) {
// |                            brightness = Brightness.LOW;
// |                         } else {
// |                            brightness = Brightness.NORMAL;
// |                         }
// |                      } else {
// |                         double f = (height - d) * 4.0 / (scale + 4) + ((xx + zz & 1) - 0.5) * 0.4;
// |                         if (f > 0.6) {
// |                            brightness = Brightness.HIGH;
// |                         } else if (f < -0.6) {
// |                            brightness = Brightness.LOW;
// |                         } else {
// |                            brightness = Brightness.NORMAL;
// |                         }
// |                      }
// |
// |                      d = height;
// |                      if (zz >= 0 && q < stepWidth * stepWidth && (!bl2 || (xx + zz & 1) != 0)) {
// |                         bl |= state.updateColor(xx, zz, mapColor.getPackedId(brightness));
// |                      }
// |                   }
// |                }
// |             }
// |          }
// |       }
// |
// |       info.cancel();
// |    }
// | }
