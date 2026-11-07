// Not ported yet: the Java @Inject(HEAD) on EndSpikeFeature.placeSpike (SpikeFeatureMixin.be_placeSpike).
// Why it's off: No Horizon function mapped for EndSpikeFeature.placeSpike (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EndSpikeFeature.placeSpike: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.<init> -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   MutableBlockPos.<init> -> (not mapped yet)
//   MutableBlockPos.set -> (not mapped yet)
//   MutableBlockPos.setX -> (not mapped yet)
//   MutableBlockPos.setY -> (not mapped yet)
//   MutableBlockPos.setZ -> (not mapped yet)
//   Vec3i.getX -> (not mapped yet)
//   Vec3i.getY -> (not mapped yet)
//   Vec3i.getZ -> (not mapped yet)
//   CompoundTag.contains -> (not mapped yet)
//   CompoundTag.getInt -> (not mapped yet)
//   CompoundTag.putInt -> (not mapped yet)
//   Mth.abs -> (not mapped yet)
//   RandomSource.nextFloat -> (not mapped yet)
//   RandomSource.nextInt -> (not mapped yet)
//   EntitySpawnReason.STRUCTURE -> (not mapped yet)
//   EntityType.create -> horizon_client::mobs::finalize::create (suggested, high)
//   EntityTypes.END_CRYSTAL -> (not mapped yet)
//   EndCrystal.setBeamTarget -> (not mapped yet)
//   EndCrystal.setPermanentlyInvulnerable -> (not mapped yet)
//   EndCrystal.snapTo -> (not mapped yet)
//   ServerLevelAccessor.addFreshEntity -> (not mapped yet)
//   ServerLevelAccessor.getBlockState -> (not mapped yet)
//   ServerLevelAccessor.getChunk -> (not mapped yet)
//   ServerLevelAccessor.getLevel -> (not mapped yet)
//   Block.defaultBlockState -> (not mapped yet)
//   Blocks.BEDROCK -> (not mapped yet)
//   Blocks.CRYING_OBSIDIAN -> (not mapped yet)
//   Blocks.IRON_BARS -> (not mapped yet)
//   Blocks.OBSIDIAN -> (not mapped yet)
//   IronBarsBlock.EAST -> (not mapped yet)
//   IronBarsBlock.NORTH -> (not mapped yet)
//   IronBarsBlock.SOUTH -> (not mapped yet)
//   IronBarsBlock.WEST -> (not mapped yet)
//   BlockState.canBeReplaced -> horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested, high)
//   BlockState.setValue -> (not mapped yet)
//   ChunkAccess.getHeight -> (not mapped yet)
//   Types.WORLD_SURFACE -> (not mapped yet)
//   EndSpikeFeature.crystalBeamTarget -> (not mapped yet)
//   ... 11 more: see the report's call list.
//
// Decompiled Java, org/betterx/betterend/mixin/common/SpikeFeatureMixin.java:43:
// | @Inject(method = "placeSpike", at = @At("HEAD"), cancellable = true)
// | private void be_placeSpike(ServerLevelAccessor world, RandomSource random, EndSpike spike, CallbackInfo info) {
// |    EndSpikeFeature self = (EndSpikeFeature)this;
// |    int x = spike.getCenterX();
// |    int z = spike.getCenterZ();
// |    int radius = spike.getRadius();
// |    int minY = 0;
// |    long lx = x;
// |    long lz = z;
// |    if (lx * lx + lz * lz < 10000L) {
// |       String pillarID = String.format("%d_%d", x, z);
// |       CompoundTag pillar = WorldConfig.getCompoundTag(BetterEnd.C, "pillars");
// |       boolean haveValue = pillar.contains(pillarID);
// |       minY = haveValue ? pillar.getInt(pillarID).orElse(0) : world.getChunk(x >> 4, z >> 4).getHeight(Types.WORLD_SURFACE, x & 15, z);
// |       if (!haveValue) {
// |          pillar.putInt(pillarID, minY);
// |          WorldConfig.saveFile(BetterEnd.C);
// |       }
// |    } else {
// |       minY = world.getChunk(x >> 4, z >> 4).getHeight(Types.WORLD_SURFACE, x & 15, z);
// |    }
// |
// |    GeneratorOptions.setDirectSpikeHeight();
// |    int maxY = minY + spike.getHeight() - 64;
// |    if (GeneratorOptions.replacePillars() && this.be_radiusInRange(radius)) {
// |       StructureTemplate base = StructureNBT.readTemplate(BetterEnd.C.mk("pillars/pillar_base_" + --radius));
// |       String path = "pillars/pillar_top_" + radius + (spike.isGuarded() ? "_cage" : "");
// |       StructureTemplate top = StructureNBT.readTemplate(BetterEnd.C.mk(path));
// |       Vec3i side = base.getSize();
// |       BlockPos pos1 = new BlockPos(x - (side.getX() >> 1), minY - 3, z - (side.getZ() >> 1));
// |       minY = pos1.getY() + side.getY();
// |       side = top.getSize();
// |       BlockPos pos2 = new BlockPos(x - (side.getX() >> 1), maxY, z - (side.getZ() >> 1));
// |       maxY = pos2.getY();
// |       StructurePlaceSettings data = new StructurePlaceSettings();
// |       base.placeInWorld(world, pos1, pos1, data, random, 2);
// |       top.placeInWorld(world, pos2, pos2, data, random, 2);
// |       int r2 = radius * radius + 1;
// |       MutableBlockPos mut = new MutableBlockPos();
// |
// |       for (int px = -radius; px <= radius; px++) {
// |          mut.setX(x + px);
// |          int x2 = px * px;
// |
// |          for (int pz = -radius; pz <= radius; pz++) {
// |             mut.setZ(z + pz);
// |             int z2 = pz * pz;
// |             if (x2 + z2 <= r2) {
// |                for (int py = minY; py < maxY; py++) {
// |                   mut.setY(py);
// |                   if (world.getBlockState(mut).canBeReplaced()) {
// |                      if ((px == radius || px == -radius || pz == radius || pz == -radius) && random.nextInt(24) == 0) {
// |                         BlocksHelper.setWithoutUpdate(world, mut, Blocks.CRYING_OBSIDIAN);
// |                      } else {
// |                         BlocksHelper.setWithoutUpdate(world, mut, Blocks.OBSIDIAN);
// |                      }
// |                   }
// |                }
// |             }
// |          }
// |       }
// |    } else {
// |       minY -= 15;
// |       int r2 = radius * radius + 1;
// |       MutableBlockPos mut = new MutableBlockPos();
// |
// |       for (int px = -radius; px <= radius; px++) {
// |          mut.setX(x + px);
// |          int x2 = px * px;
// |
// |          for (int pz = -radius; pz <= radius; pz++) {
// |             mut.setZ(z + pz);
// |             int z2 = pz * pz;
// |             if (x2 + z2 <= r2) {
// |                for (int py = minY; py < maxY; py++) {
// |                   mut.setY(py);
// |                   if (world.getBlockState(mut).canBeReplaced()) {
// |                      BlocksHelper.setWithoutUpdate(world, mut, Blocks.OBSIDIAN);
// |                   }
// |                }
// |             }
// |          }
// |       }
// |
// |       mut.setX(x);
// |       mut.setZ(z);
// |       mut.setY(maxY);
// |       BlocksHelper.setWithoutUpdate(world, mut, Blocks.BEDROCK);
// |       EndCrystal crystal = (EndCrystal)EntityTypes.END_CRYSTAL.create(world.getLevel(), EntitySpawnReason.STRUCTURE);
// |       crystal.setBeamTarget((BlockPos)self.crystalBeamTarget().orElse(null));
// |       crystal.setPermanentlyInvulnerable(self.crystalInvulnerable());
// |       crystal.snapTo(x + 0.5, maxY + 1, z + 0.5, random.nextFloat() * 360.0F, 0.0F);
// |       world.addFreshEntity(crystal);
// |       if (spike.isGuarded()) {
// |          for (int px = -2; px <= 2; px++) {
// |             boolean bl = Mth.abs(px) == 2;
// |
// |             for (int pz = -2; pz <= 2; pz++) {
// |                boolean bl2 = Mth.abs(pz) == 2;
// |
// |                for (int py = 0; py <= 3; py++) {
// |                   boolean bl3 = py == 3;
// |                   if (bl || bl2 || bl3) {
// |                      boolean bl4 = px == -2 || px == 2 || bl3;
// |                      boolean bl5 = pz == -2 || pz == 2 || bl3;
// |                      BlockState blockState = (BlockState)((BlockState)((BlockState)((BlockState)Blocks.IRON_BARS
// |                                  .defaultBlockState()
// |                                  .setValue(IronBarsBlock.NORTH, bl4 && pz != -2))
// |                               .setValue(IronBarsBlock.SOUTH, bl4 && pz != 2))
// |                            .setValue(IronBarsBlock.WEST, bl5 && px != -2))
// |                         .setValue(IronBarsBlock.EAST, bl5 && px != 2);
// |                      BlocksHelper.setWithoutUpdate(world, mut.set(spike.getCenterX() + px, maxY + py, spike.getCenterZ() + pz), blockState);
// |                   }
// |                }
// |             }
// |          }
// |       }
// |    }
// |
// |    info.cancel();
// | }
