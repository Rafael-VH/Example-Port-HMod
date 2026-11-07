# Java translated to Rust

Translated 214 of 2864 methods (1 more is done another way in Horizon) and 361 of 1891 static fields to Rust (8 of those call the game, through Horizon's helpers); 1 of 2 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 2067 | `BECriteria.register(0)`: it uses BECriteria.PORTAL_ON, which isn't translated (it uses Minecraft's `PlayerTrigger`) |
| Uses Minecraft classes (needs Horizon's game API) | 234 | `PedestalBlock.defaultShape(0)`: it uses Minecraft's `VoxelShape` |
| Uses other libraries | 197 | `PedestalBlock.provideBlockModel(3)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) |
| Other Java this translator doesn't handle yet | 131 | `BECriteria.register(2)`: it's generic |
| Uses null | 5 | `EndPortals.getWorldId(1)`: a nullable object kept in a changing static |
| Uses Fabric API | 4 | `BetterEndSkyRenderer.register(0)`: it uses Fabric's `LevelRenderEvents`.START_MAIN |
| Calls the game where Horizon has no helper yet | 2 | `ScreenWobble.at(1)`: it calls Minecraft's Options.screenEffectScale (no Horizon helper for it yet) |
| Uses files (java.nio / java.io) | 2 | `EndPortals.loadPortals(0)`: it uses `File` (java.io) |
| Uses inheritance (extends, implements, instanceof) | 2 | `EternalHint.streakToCentre(4)`: it uses `ClientLevelAccess` (org.betterx.bclib.interfaces) |
| Uses lambdas or method references | 2 | `EndFlowerPotModels.register(0)`: a method reference |
| Done another way in Horizon | 1 | `BetterEndClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook |
| Shares one object between names (Java references) | 1 | `FlowerPotMigration.readTable(3)`: it changes an object through a second name |
| Uses Java collections other than List (Map, Set...) | 1 | `CreativeTabs.inTabOrder(1)`: it uses `Stream` (java.util.stream) |
| Uses translated text (Component.translatable) | 1 | `LangUtil.getText(2)`: a translated text with a computed key |

## Close, not the same

Where the Rust does a little less than the Java, because Horizon doesn't have something:

- ClientHooks.showRecipes: opening InfusionRecipeScreen opens the mod's page in Options > Mods (its settings buttons) instead

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| org/betterx/betterend/client/render/BetterEndSkyRenderer.java:82 | `started` | not translated: it uses BetterEndSkyRenderer.NEBULA_1, which isn't translated (it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api))) |
| org/betterx/betterend/BetterEnd.java:62 | `init` | `on_init`, called from the hook, without: the call to `Patcher.register(..)`: it calls Patcher.register, which isn't translated; the call to `SERVER_STARTING.register(..)`: it uses Fabric's `ServerLifecycleEvents`.SERVER_STARTING; the call to `EndNumericProviders.register(..)`: it calls EndNumericProviders.register, which isn't translated; the call to `EndPortals.loadPortals(..)`: it calls EndPortals.loadPortals, which isn't translated; the call to `EndEntities.register(..)`: it calls EndEntities.register, which isn't translated; the call to `EndBiomes.register(..)`: it calls EndBiomes.register, which isn't translated; the call to `EndBlockEntities.addLateRegisteredBlocks(..)`: it calls EndBlockEntities.addLateRegisteredBlocks, which isn't translated; the call to `EndPotions.register(..)`: it calls EndPotions.register, which isn't translated; the call to `BonemealPlants.init(..)`: it calls BonemealPlants.init, which isn't translated; the call to `GeneratorOptions.init(..)`: it calls GeneratorOptions.init, which isn't translated; the call to `CommandRegistry.register(..)`: it calls CommandRegistry.register, which isn't translated; the call to `BECriteria.register(..)`: it calls BECriteria.register, which isn't translated; the call to `forEach(..)`: it calls Fabric's FabricLoader.getEntrypoints; the call to `Integrations.init(..)`: it calls Integrations.init, which isn't translated; the call to `Configs.saveConfigs(..)`: it calls Configs.saveConfigs, which isn't translated; the call to `CreativeTabs.register(..)`: it calls CreativeTabs.register, which isn't translated; the call to `BiomeDecider.registerHighPriorityDecider(..)`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)); the local `ignored`: it uses `ClientBoundMessage` (de.ambertation.wunderlib.network) |

## Every class


### `BECriteria`

- [ ] `BECriteria.PORTAL_ON_ID`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/advancements/BECriteria.java:14)
- [ ] `BECriteria.PORTAL_TRAVEL_ID`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/advancements/BECriteria.java:15)
- [ ] `BECriteria.INFUSION_FINISHED_ID`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/advancements/BECriteria.java:16)
- [ ] `BECriteria.PORTAL_ON`: it uses Minecraft's `PlayerTrigger` (org/betterx/betterend/advancements/BECriteria.java:17)
- [ ] `BECriteria.PORTAL_TRAVEL`: it uses Minecraft's `PlayerTrigger` (org/betterx/betterend/advancements/BECriteria.java:18)
- [ ] `BECriteria.INFUSION_FINISHED`: it uses Minecraft's `PlayerTrigger` (org/betterx/betterend/advancements/BECriteria.java:19)
- [ ] `BECriteria.PORTAL_ON_TRIGGER`: it uses Minecraft's `Criterion` (org/betterx/betterend/advancements/BECriteria.java:20)
- [ ] `BECriteria.PORTAL_TRAVEL_TRIGGER`: it uses Minecraft's `Criterion` (org/betterx/betterend/advancements/BECriteria.java:21)
- [ ] `BECriteria.INFUSION_FINISHED_TRIGGER`: it uses Minecraft's `Criterion` (org/betterx/betterend/advancements/BECriteria.java:22)
- [ ] `BECriteria.register(0)`: it uses BECriteria.PORTAL_ON, which isn't translated (it uses Minecraft's `PlayerTrigger`) (org/betterx/betterend/advancements/BECriteria.java:24)
- [ ] `BECriteria.register(2)`: it's generic (org/betterx/betterend/advancements/BECriteria.java:33)

### `BetterEndPlugin`: it's an interface

- [ ] `BetterEndPlugin.registerAlloyingRecipes(0)`: its class isn't translated (it's an interface) (org/betterx/betterend/api/BetterEndPlugin.java:4)
- [ ] `BetterEndPlugin.registerSmithingRecipes(0)`: its class isn't translated (it's an interface) (org/betterx/betterend/api/BetterEndPlugin.java:7)
- [ ] `BetterEndPlugin.registerEndBiomes(0)`: its class isn't translated (it's an interface) (org/betterx/betterend/api/BetterEndPlugin.java:10)
- [ ] `BetterEndPlugin.registerOthers(0)`: its class isn't translated (it's an interface) (org/betterx/betterend/api/BetterEndPlugin.java:13)
- [ ] `BetterEndPlugin.register(1)`: it uses BetterEndPlugin, which isn't translated (it's an interface) (org/betterx/betterend/api/BetterEndPlugin.java:16)

### `BetterEnd`: it implements Fabric's `ModInitializer`

- [ ] `BetterEnd.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betterend/BetterEnd.java:48)
- [ ] `BetterEnd.TRINKETS_CORE`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betterend/BetterEnd.java:49)
- [ ] `BetterEnd.MOD_ID`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/BetterEnd.java:50)
- [x] `BetterEnd.LOGGER` (org/betterx/betterend/BetterEnd.java:51)
- [ ] `BetterEnd.NOURISH`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betterend/BetterEnd.java:52)
- [ ] `BetterEnd.FLAMBOYANT`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betterend/BetterEnd.java:53)
- [ ] `BetterEnd.PATCHOULI`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betterend/BetterEnd.java:54)
- [ ] `BetterEnd.GLIMMERWILD`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betterend/BetterEnd.java:55)
- [ ] `BetterEnd.HYDROGEN`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betterend/BetterEnd.java:56)
- [ ] `BetterEnd.NOURISH_ADDITIONS_PACK`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/BetterEnd.java:57)
- [ ] `BetterEnd.FLAMBOYANT_ADDITIONS_PACK`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/BetterEnd.java:58)
- [ ] `BetterEnd.PATCHOULI_ADDITIONS_PACK`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/BetterEnd.java:59)
- [ ] `BetterEnd.GLIMMERWILD_ADDITIONS_PACK`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/BetterEnd.java:60)
- [ ] `BetterEnd.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (org/betterx/betterend/BetterEnd.java:62)

### `AeterniumAnvil`: it extends `LeveledAnvilBlock` (org.betterx.bclib.blocks)

- [x] `AeterniumAnvil.AETERNIUM_ANVIL_LEVEL` (org/betterx/betterend/blocks/AeterniumAnvil.java:7)
- [ ] `AeterniumAnvil.AeterniumAnvil(1)`: its class isn't translated (it extends `LeveledAnvilBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/AeterniumAnvil.java:9)
- [ ] `AeterniumAnvil.getMaxDurability(0)`: its class isn't translated (it extends `LeveledAnvilBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/AeterniumAnvil.java:13)

### `AeterniumBlock`: it extends Minecraft's `Block`

- [ ] `AeterniumBlock.AeterniumBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/AeterniumBlock.java:7)

### `AmaranitaCapBlock`: it extends Minecraft's `Block`

- [ ] `AmaranitaCapBlock.AmaranitaCapBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/AmaranitaCapBlock.java:7)

### `AmaranitaHymenophoreBlock`: it extends Minecraft's `Block`

- [ ] `AmaranitaHymenophoreBlock.AmaranitaHymenophoreBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/AmaranitaHymenophoreBlock.java:7)

### `AmaranitaStemBlock`: it extends Minecraft's `RotatedPillarBlock`

- [ ] `AmaranitaStemBlock.AmaranitaStemBlock(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betterend/blocks/AmaranitaStemBlock.java:7)

### `AmberBlock`: it extends Minecraft's `Block`

- [ ] `AmberBlock.AmberBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/AmberBlock.java:7)

### `AncientEmeraldIceBlock`: it extends Minecraft's `Block`

- [ ] `AncientEmeraldIceBlock.AncientEmeraldIceBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/AncientEmeraldIceBlock.java:19)
- [ ] `AncientEmeraldIceBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/AncientEmeraldIceBlock.java:23)
- [ ] `AncientEmeraldIceBlock.makeParticles(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/AncientEmeraldIceBlock.java:47)
- [ ] `AncientEmeraldIceBlock.stepOn(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/AncientEmeraldIceBlock.java:51)

### `AuroraCrystalBlock`: it extends Minecraft's `TransparentBlock`

- [ ] `AuroraCrystalBlock.COLORS`: it uses Minecraft's `Vec3i` (org/betterx/betterend/blocks/AuroraCrystalBlock.java:13)
- [ ] `AuroraCrystalBlock.AuroraCrystalBlock(1)`: its class isn't translated (it extends Minecraft's `TransparentBlock`) (org/betterx/betterend/blocks/AuroraCrystalBlock.java:15)
- [ ] `AuroraCrystalBlock.getVisualShape(4)`: its class isn't translated (it extends Minecraft's `TransparentBlock`) (org/betterx/betterend/blocks/AuroraCrystalBlock.java:19)

### `EndLanternBlock`: it extends Minecraft's `Block`

- [ ] `EndLanternBlock.IS_FLOOR`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/basis/EndLanternBlock.java:30)
- [ ] `EndLanternBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/basis/EndLanternBlock.java:31)
- [ ] `EndLanternBlock.EndLanternBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/EndLanternBlock.java:33)
- [ ] `EndLanternBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/EndLanternBlock.java:38)
- [ ] `EndLanternBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/EndLanternBlock.java:42)
- [ ] `EndLanternBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/EndLanternBlock.java:72)
- [ ] `EndLanternBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/EndLanternBlock.java:76)
- [ ] `EndLanternBlock.canPlaceLiquid(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/EndLanternBlock.java:98)
- [ ] `EndLanternBlock.placeLiquid(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/EndLanternBlock.java:102)
- [ ] `EndLanternBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/EndLanternBlock.java:106)

### `EndUnderwaterWallPlantBlock`: it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)

- [ ] `EndUnderwaterWallPlantBlock.EndUnderwaterWallPlantBlock(1)`: its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/EndUnderwaterWallPlantBlock.java:18)
- [ ] `EndUnderwaterWallPlantBlock.canPlaceLiquid(5)`: its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/EndUnderwaterWallPlantBlock.java:22)
- [ ] `EndUnderwaterWallPlantBlock.placeLiquid(4)`: its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/EndUnderwaterWallPlantBlock.java:26)
- [ ] `EndUnderwaterWallPlantBlock.getFluidState(1)`: its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/EndUnderwaterWallPlantBlock.java:30)
- [ ] `EndUnderwaterWallPlantBlock.canSurvive(3)`: its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/EndUnderwaterWallPlantBlock.java:34)

### `FurBlock`: it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)

- [ ] `FurBlock.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/basis/FurBlock.java:30)
- [ ] `FurBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/basis/FurBlock.java:31)
- [ ] `FurBlock.FurBlock(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/FurBlock.java:33)
- [ ] `FurBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/FurBlock.java:38)
- [ ] `FurBlock.getStateForPlacement(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/FurBlock.java:43)
- [ ] `FurBlock.getFluidState(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/FurBlock.java:53)
- [ ] `FurBlock.updateShape(8)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/FurBlock.java:58)
- [ ] `FurBlock.getShape(4)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/basis/FurBlock.java:81)

### `PedestalBlock`: it extends Minecraft's `Block`

- [ ] `PedestalBlock.STATE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/basis/PedestalBlock.java:63)
- [ ] `PedestalBlock.HAS_ITEM`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/basis/PedestalBlock.java:64)
- [ ] `PedestalBlock.HAS_LIGHT`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/basis/PedestalBlock.java:65)
- [ ] `PedestalBlock.SHAPE_DEFAULT`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/PedestalBlock.java:66)
- [ ] `PedestalBlock.SHAPE_COLUMN`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/PedestalBlock.java:67)
- [ ] `PedestalBlock.SHAPE_PILLAR`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/PedestalBlock.java:68)
- [ ] `PedestalBlock.SHAPE_PEDESTAL_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/PedestalBlock.java:69)
- [ ] `PedestalBlock.SHAPE_COLUMN_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/PedestalBlock.java:70)
- [ ] `PedestalBlock.SHAPE_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/PedestalBlock.java:71)
- [ ] `PedestalBlock.PedestalBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:74)
- [ ] `PedestalBlock.getHeight(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:83)
- [ ] `PedestalBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:89)
- [ ] `PedestalBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:120)
- [ ] `PedestalBlock.destroy(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:130)
- [ ] `PedestalBlock.checkRitual(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:145)
- [ ] `PedestalBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:158)
- [ ] `PedestalBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:180)
- [ ] `PedestalBlock.getUpdatedState(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:202)
- [ ] `PedestalBlock.getDrops(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:254)
- [ ] `PedestalBlock.moveStoredStack(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:271)
- [ ] `PedestalBlock.moveStoredStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:281)
- [ ] `PedestalBlock.dropStoredStack(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:300)
- [ ] `PedestalBlock.getDropPos(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:307)
- [ ] `PedestalBlock.defaultShape(0)`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/PedestalBlock.java:326)
- [ ] `PedestalBlock.isPlaceable(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:330)
- [ ] `PedestalBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:339)
- [ ] `PedestalBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:355)
- [ ] `PedestalBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:359)
- [ ] `PedestalBlock.hasUniqueEntity(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:363)
- [ ] `PedestalBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:367)
- [ ] `PedestalBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/basis/PedestalBlock.java:371)
- [ ] `PedestalBlock.provideBlockModel(3)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/basis/PedestalBlock.java:375)
- [ ] `PedestalBlock.provideBlockModel(4)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/basis/PedestalBlock.java:380)
- [ ] `PedestalBlock.createTextureMapping(1)`: it uses Minecraft's `TextureMapping` (org/betterx/betterend/blocks/basis/PedestalBlock.java:387)

### `PedestalBlock.ClientHooks`

- [ ] `PedestalBlock.ClientHooks.onSocketFilled(3)`: it uses Minecraft's `Level` (org/betterx/betterend/blocks/basis/PedestalBlock.java:417)

### `PedestalBlock.ClientModelBuilder`

- [ ] `PedestalBlock.ClientModelBuilder.buildBlockModel(4)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/basis/PedestalBlock.java:430)

### `PedestalBlock.Models`

- [ ] `PedestalBlock.Models.PEDESTAL_MODELS`: it uses PedestalState, which isn't translated (its constants carry values) (org/betterx/betterend/blocks/basis/PedestalBlock.java:453)

### `StoneLanternBlock`: it extends `EndLanternBlock`

- [ ] `StoneLanternBlock.SHAPE_CEIL`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:31)
- [ ] `StoneLanternBlock.SHAPE_FLOOR`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:32)
- [ ] `StoneLanternBlock.MODEL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:33)
- [ ] `StoneLanternBlock.StoneLanternBlock(1)`: its class isn't translated (it extends `EndLanternBlock`) (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:36)
- [ ] `StoneLanternBlock.getShape(4)`: its class isn't translated (it extends `EndLanternBlock`) (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:40)
- [ ] `StoneLanternBlock.buildModel(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:45)
- [ ] `StoneLanternBlock.provideBlockModel(3)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:49)

### `StoneLanternBlock.ClientModel`

- [ ] `StoneLanternBlock.ClientModel.buildBlockModel(3)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:56)
- [ ] `StoneLanternBlock.ClientModel.build(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/basis/StoneLanternBlock.java:66)

### `BlueVineBlock`: it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)

- [ ] `BlueVineBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/BlueVineBlock.java:15)
- [ ] `BlueVineBlock.BlueVineBlock(1)`: its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BlueVineBlock.java:17)
- [ ] `BlueVineBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BlueVineBlock.java:21)
- [ ] `BlueVineBlock.isTerrain(1)`: its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BlueVineBlock.java:25)

### `BlueVineLanternBlock`: it extends Minecraft's `Block`

- [ ] `BlueVineLanternBlock.NATURAL`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/BlueVineLanternBlock.java:22)
- [ ] `BlueVineLanternBlock.BlueVineLanternBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BlueVineLanternBlock.java:24)
- [ ] `BlueVineLanternBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BlueVineLanternBlock.java:29)
- [ ] `BlueVineLanternBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BlueVineLanternBlock.java:33)
- [ ] `BlueVineLanternBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BlueVineLanternBlock.java:46)
- [ ] `BlueVineLanternBlock.provideBlockModel(2)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/BlueVineLanternBlock.java:50)

### `BlueVineSeedBlock`: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `BlueVineSeedBlock.BlueVineSeedBlock(1)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BlueVineSeedBlock.java:19)
- [ ] `BlueVineSeedBlock.growAdult(3)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BlueVineSeedBlock.java:23)
- [ ] `BlueVineSeedBlock.placeLantern(2)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BlueVineSeedBlock.java:50)

### `BoluxMushroomBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `BoluxMushroomBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/BoluxMushroomBlock.java:17)
- [ ] `BoluxMushroomBlock.BoluxMushroomBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BoluxMushroomBlock.java:19)
- [ ] `BoluxMushroomBlock.getShape(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BoluxMushroomBlock.java:23)
- [ ] `BoluxMushroomBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BoluxMushroomBlock.java:27)
- [ ] `BoluxMushroomBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BoluxMushroomBlock.java:31)

### `BrimstoneBlock`: it extends Minecraft's `Block`

- [ ] `BrimstoneBlock.ACTIVATED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/BrimstoneBlock.java:27)
- [ ] `BrimstoneBlock.BrimstoneBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BrimstoneBlock.java:29)
- [ ] `BrimstoneBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BrimstoneBlock.java:34)
- [ ] `BrimstoneBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BrimstoneBlock.java:38)
- [ ] `BrimstoneBlock.destroy(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BrimstoneBlock.java:44)
- [ ] `BrimstoneBlock.updateChunks(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BrimstoneBlock.java:50)
- [ ] `BrimstoneBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/BrimstoneBlock.java:65)

### `BubbleCoralBlock`: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)

- [ ] `BubbleCoralBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/BubbleCoralBlock.java:20)
- [ ] `BubbleCoralBlock.BubbleCoralBlock(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/BubbleCoralBlock.java:22)
- [ ] `BubbleCoralBlock.animateTick(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/BubbleCoralBlock.java:26)
- [ ] `BubbleCoralBlock.getShape(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/BubbleCoralBlock.java:34)
- [ ] `BubbleCoralBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/BubbleCoralBlock.java:38)
- [ ] `BubbleCoralBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/BubbleCoralBlock.java:42)

### `BuddingSmaragdantCrystalBlock`: it extends Minecraft's `RotatedPillarBlock`

- [ ] `BuddingSmaragdantCrystalBlock.BuddingSmaragdantCrystalBlock(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betterend/blocks/BuddingSmaragdantCrystalBlock.java:16)
- [ ] `BuddingSmaragdantCrystalBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betterend/blocks/BuddingSmaragdantCrystalBlock.java:20)
- [ ] `BuddingSmaragdantCrystalBlock.canShardGrowAtState(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/blocks/BuddingSmaragdantCrystalBlock.java:33)

### `BulbVineBlock`: it extends `BaseVineBlock` (org.betterx.bclib.blocks)

- [ ] `BulbVineBlock.BulbVineBlock(1)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BulbVineBlock.java:25)
- [ ] `BulbVineBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BulbVineBlock.java:29)
- [ ] `BulbVineBlock.canSurvive(3)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BulbVineBlock.java:33)
- [ ] `BulbVineBlock.buildLoot(2)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/blocks/BulbVineBlock.java:38)

### `BulbVineLanternBlock`: it extends `EndLanternBlock` (org.betterx.betterend.blocks.basis)

- [ ] `BulbVineLanternBlock.SHAPE_CEIL`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/BulbVineLanternBlock.java:30)
- [ ] `BulbVineLanternBlock.SHAPE_FLOOR`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/BulbVineLanternBlock.java:31)
- [ ] `BulbVineLanternBlock.MODEL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/blocks/BulbVineLanternBlock.java:32)
- [ ] `BulbVineLanternBlock.BulbVineLanternBlock(1)`: its class isn't translated (it extends `EndLanternBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/BulbVineLanternBlock.java:35)
- [ ] `BulbVineLanternBlock.getShape(4)`: its class isn't translated (it extends `EndLanternBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/BulbVineLanternBlock.java:39)
- [x] `BulbVineLanternBlock.getMetalTexture(1)` (org/betterx/betterend/blocks/BulbVineLanternBlock.java:43)
- [ ] `BulbVineLanternBlock.getGlowTexture(0)`: its class isn't translated (it extends `EndLanternBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/BulbVineLanternBlock.java:49)
- [ ] `BulbVineLanternBlock.buildModel(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/BulbVineLanternBlock.java:53)

### `BulbVineLanternBlock.ClientModel`

- [ ] `BulbVineLanternBlock.ClientModel.build(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/BulbVineLanternBlock.java:59)

### `BulbVineLanternColoredBlock`: it extends `BulbVineLanternBlock`

- [ ] `BulbVineLanternColoredBlock.boostLanternColor(1)`: it calls `ColorUtilARGB32` (de.ambertation.wunderlib.utils).RGBtoHSB (org/betterx/betterend/blocks/BulbVineLanternColoredBlock.java:7)
- [ ] `BulbVineLanternColoredBlock.BulbVineLanternColoredBlock(1)`: its class isn't translated (it extends `BulbVineLanternBlock`) (org/betterx/betterend/blocks/BulbVineLanternColoredBlock.java:15)
- [ ] `BulbVineLanternColoredBlock.getGlowTexture(0)`: its class isn't translated (it extends `BulbVineLanternBlock`) (org/betterx/betterend/blocks/BulbVineLanternColoredBlock.java:19)

### `BulbVineSeedBlock`: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `BulbVineSeedBlock.BulbVineSeedBlock(1)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BulbVineSeedBlock.java:17)
- [ ] `BulbVineSeedBlock.canSurvive(3)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BulbVineSeedBlock.java:21)
- [ ] `BulbVineSeedBlock.growAdult(3)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/BulbVineSeedBlock.java:26)

### `CaveMossBlock`: it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)

- [ ] `CaveMossBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/CaveMossBlock.java:24)
- [ ] `CaveMossBlock.CaveMossBlock(2)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/CaveMossBlock.java:26)
- [ ] `CaveMossBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/CaveMossBlock.java:31)
- [ ] `CaveMossBlock.getStateForPlacement(1)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/CaveMossBlock.java:35)
- [ ] `CaveMossBlock.useWithoutItem(5)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/CaveMossBlock.java:41)
- [ ] `CaveMossBlock.randomTick(4)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/CaveMossBlock.java:45)
- [ ] `CaveMossBlock.canStayBottom(2)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/CaveMossBlock.java:68)
- [ ] `CaveMossBlock.isMiddle(1)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/CaveMossBlock.java:78)

### `CavePumpkinBlock`: it extends Minecraft's `Block`

- [ ] `CavePumpkinBlock.SMALL`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/CavePumpkinBlock.java:24)
- [ ] `CavePumpkinBlock.SHAPE_SMALL`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/CavePumpkinBlock.java:25)
- [ ] `CavePumpkinBlock.SHAPE_BIG`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/CavePumpkinBlock.java:26)
- [ ] `CavePumpkinBlock.CavePumpkinBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/CavePumpkinBlock.java:28)
- [ ] `CavePumpkinBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/CavePumpkinBlock.java:33)
- [ ] `CavePumpkinBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/CavePumpkinBlock.java:37)
- [ ] `CavePumpkinBlock.buildLoot(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/CavePumpkinBlock.java:41)

### `CavePumpkinVineBlock`: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `CavePumpkinVineBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/CavePumpkinVineBlock.java:24)
- [ ] `CavePumpkinVineBlock.CavePumpkinVineBlock(1)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/CavePumpkinVineBlock.java:26)
- [ ] `CavePumpkinVineBlock.canSurvive(3)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/CavePumpkinVineBlock.java:30)
- [ ] `CavePumpkinVineBlock.performBonemeal(5)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/CavePumpkinVineBlock.java:35)
- [ ] `CavePumpkinVineBlock.growAdult(3)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/CavePumpkinVineBlock.java:53)
- [ ] `CavePumpkinVineBlock.updateShape(8)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/CavePumpkinVineBlock.java:56)
- [ ] `CavePumpkinVineBlock.getShape(4)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/CavePumpkinVineBlock.java:78)

### `ChandelierBlock`: it extends `Metal` (org.betterx.bclib.blocks.BaseAttachedBlock)

- [ ] `ChandelierBlock.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/ChandelierBlock.java:35)
- [ ] `ChandelierBlock.MODEL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/blocks/ChandelierBlock.java:36)
- [ ] `ChandelierBlock.ChandelierBlock(1)`: its class isn't translated (it extends `Metal` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/ChandelierBlock.java:39)
- [ ] `ChandelierBlock.ChandelierBlock(1)`: its class isn't translated (it extends `Metal` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/ChandelierBlock.java:43)
- [ ] `ChandelierBlock.getShape(4)`: its class isn't translated (it extends `Metal` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/ChandelierBlock.java:47)
- [ ] `ChandelierBlock.buildModel(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/ChandelierBlock.java:51)

### `ChandelierBlock.ClientModel`

- [ ] `ChandelierBlock.ClientModel.build(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/ChandelierBlock.java:66)

### `CharcoalBlock`: it extends Minecraft's `Block`

- [ ] `CharcoalBlock.CharcoalBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/CharcoalBlock.java:8)
- [ ] `CharcoalBlock.getFuelTime(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/CharcoalBlock.java:12)

### `CharniaBlock`: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)

- [ ] `CharniaBlock.CharniaBlock(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/CharniaBlock.java:8)
- [ ] `CharniaBlock.isTerrain(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/CharniaBlock.java:12)

### `CrystalMossCoverBlock`: it extends Minecraft's `MultifaceSpreadeableBlock`

- [ ] `CrystalMossCoverBlock.CrystalMossCoverBlock(1)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:24)
- [ ] `CrystalMossCoverBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:29)
- [ ] `CrystalMossCoverBlock.canBeReplaced(2)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:46)
- [ ] `CrystalMossCoverBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:51)
- [ ] `CrystalMossCoverBlock.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:55)
- [ ] `CrystalMossCoverBlock.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:59)
- [ ] `CrystalMossCoverBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:63)
- [ ] `CrystalMossCoverBlock.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:67)
- [ ] `CrystalMossCoverBlock.getSpreader(0)`: its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`) (org/betterx/betterend/blocks/CrystalMossCoverBlock.java:71)

### `DragonTreeSaplingBlock`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `DragonTreeSaplingBlock.DragonTreeSaplingBlock(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betterend/blocks/DragonTreeSaplingBlock.java:8)

### `EmeraldIceBlock`: it extends Minecraft's `HalfTransparentBlock`

- [ ] `EmeraldIceBlock.EmeraldIceBlock(1)`: its class isn't translated (it extends Minecraft's `HalfTransparentBlock`) (org/betterx/betterend/blocks/EmeraldIceBlock.java:22)
- [ ] `EmeraldIceBlock.playerDestroy(6)`: its class isn't translated (it extends Minecraft's `HalfTransparentBlock`) (org/betterx/betterend/blocks/EmeraldIceBlock.java:26)
- [ ] `EmeraldIceBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `HalfTransparentBlock`) (org/betterx/betterend/blocks/EmeraldIceBlock.java:48)
- [ ] `EmeraldIceBlock.melt(2)`: its class isn't translated (it extends Minecraft's `HalfTransparentBlock`) (org/betterx/betterend/blocks/EmeraldIceBlock.java:54)

### `EndBlockProperties`: it extends `BlockProperties` (de.ambertation.wover.block.api)

- [ ] `EndBlockProperties.HYDRALUX_SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndBlockProperties.java:11)
- [ ] `EndBlockProperties.PEDESTAL_STATE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndBlockProperties.java:12)
- [ ] `EndBlockProperties.CACTUS_BOTTOM`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndBlockProperties.java:13)
- [ ] `EndBlockProperties.PORTAL`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/EndBlockProperties.java:14)
- [ ] `EndBlockProperties.PLANT_ID`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/EndBlockProperties.java:15)
- [ ] `EndBlockProperties.SOIL_ID`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/EndBlockProperties.java:16)
- [ ] `EndBlockProperties.POT_LIGHT`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/EndBlockProperties.java:17)
- [ ] `EndBlockProperties.HAS_ITEM`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/EndBlockProperties.java:18)

### `EndBlockProperties.CactusBottom`: its constants carry values

- [ ] `EndBlockProperties.CactusBottom.CactusBottom(1)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:27)
- [ ] `EndBlockProperties.CactusBottom.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:31)
- [ ] `EndBlockProperties.CactusBottom.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:35)

### `EndBlockProperties.HydraluxShape`: its constants carry values

- [ ] `EndBlockProperties.HydraluxShape.HydraluxShape(2)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:52)
- [ ] `EndBlockProperties.HydraluxShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:57)
- [ ] `EndBlockProperties.HydraluxShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:61)
- [ ] `EndBlockProperties.HydraluxShape.hasGlow(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:66)

### `EndBlockProperties.LumecornShape`: its constants carry values

- [ ] `EndBlockProperties.LumecornShape.LumecornShape(2)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:83)
- [ ] `EndBlockProperties.LumecornShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:88)
- [ ] `EndBlockProperties.LumecornShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:92)
- [ ] `EndBlockProperties.LumecornShape.getLight(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:97)

### `EndBlockProperties.PedestalState`: its constants carry values

- [ ] `EndBlockProperties.PedestalState.PedestalState(1)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:112)
- [ ] `EndBlockProperties.PedestalState.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:116)
- [ ] `EndBlockProperties.PedestalState.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/blocks/EndBlockProperties.java:120)

### `EnderBlock`: it extends Minecraft's `Block`

- [ ] `EnderBlock.EnderBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EnderBlock.java:12)
- [ ] `EnderBlock.getColor(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EnderBlock.java:16)

### `EndLilyBlock`: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)

- [ ] `EndLilyBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndLilyBlock.java:39)
- [ ] `EndLilyBlock.SHAPE_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/EndLilyBlock.java:40)
- [ ] `EndLilyBlock.SHAPE_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/EndLilyBlock.java:41)
- [ ] `EndLilyBlock.EndLilyBlock(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:43)
- [ ] `EndLilyBlock.updateShape(8)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:47)
- [ ] `EndLilyBlock.getShape(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:65)
- [ ] `EndLilyBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:71)
- [ ] `EndLilyBlock.getFluidState(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:75)
- [ ] `EndLilyBlock.canSurvive(3)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:79)
- [ ] `EndLilyBlock.buildLoot(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/EndLilyBlock.java:93)
- [ ] `EndLilyBlock.isTop(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/EndLilyBlock.java:113)
- [ ] `EndLilyBlock.getCloneItemStack(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:119)
- [ ] `EndLilyBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:123)
- [ ] `EndLilyBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/EndLilyBlock.java:127)

### `EndLilySeedBlock`: it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `EndLilySeedBlock.EndLilySeedBlock(1)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLilySeedBlock.java:16)
- [ ] `EndLilySeedBlock.grow(3)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLilySeedBlock.java:20)
- [ ] `EndLilySeedBlock.canGrow(2)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLilySeedBlock.java:37)
- [ ] `EndLilySeedBlock.isTerrain(1)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLilySeedBlock.java:47)

### `EndLotusFlowerBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `EndLotusFlowerBlock.SHAPE_OUTLINE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/EndLotusFlowerBlock.java:25)
- [ ] `EndLotusFlowerBlock.SHAPE_COLLISION`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/EndLotusFlowerBlock.java:26)
- [ ] `EndLotusFlowerBlock.EndLotusFlowerBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusFlowerBlock.java:28)
- [ ] `EndLotusFlowerBlock.getShape(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusFlowerBlock.java:32)
- [ ] `EndLotusFlowerBlock.getCollisionShape(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusFlowerBlock.java:36)
- [ ] `EndLotusFlowerBlock.buildLoot(2)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/blocks/EndLotusFlowerBlock.java:40)
- [ ] `EndLotusFlowerBlock.getCloneItemStack(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusFlowerBlock.java:54)

### `EndLotusLeafBlock`: it extends Minecraft's `Block`

- [ ] `EndLotusLeafBlock.HORIZONTAL_FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndLotusLeafBlock.java:27)
- [ ] `EndLotusLeafBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndLotusLeafBlock.java:28)
- [ ] `EndLotusLeafBlock.VSHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/EndLotusLeafBlock.java:29)
- [ ] `EndLotusLeafBlock.EndLotusLeafBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusLeafBlock.java:31)
- [ ] `EndLotusLeafBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusLeafBlock.java:35)
- [ ] `EndLotusLeafBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusLeafBlock.java:40)
- [ ] `EndLotusLeafBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusLeafBlock.java:44)
- [ ] `EndLotusLeafBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusLeafBlock.java:48)
- [ ] `EndLotusLeafBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusLeafBlock.java:52)
- [ ] `EndLotusLeafBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusLeafBlock.java:56)

### `EndLotusSeedBlock`: it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `EndLotusSeedBlock.EndLotusSeedBlock(1)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusSeedBlock.java:20)
- [ ] `EndLotusSeedBlock.grow(3)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusSeedBlock.java:24)
- [ ] `EndLotusSeedBlock.canGrow(2)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusSeedBlock.java:96)
- [ ] `EndLotusSeedBlock.generateLeaf(2)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusSeedBlock.java:107)
- [ ] `EndLotusSeedBlock.hasLeaf(2)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusSeedBlock.java:129)
- [ ] `EndLotusSeedBlock.isTerrain(1)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/EndLotusSeedBlock.java:148)

### `EndLotusStemBlock`: it extends Minecraft's `Block`

- [ ] `EndLotusStemBlock.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndLotusStemBlock.java:36)
- [ ] `EndLotusStemBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/EndLotusStemBlock.java:37)
- [ ] `EndLotusStemBlock.LEAF`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/EndLotusStemBlock.java:38)
- [ ] `EndLotusStemBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndLotusStemBlock.java:39)
- [ ] `EndLotusStemBlock.SHAPES`: it uses Minecraft's `Axis` (org/betterx/betterend/blocks/EndLotusStemBlock.java:40)
- [ ] `EndLotusStemBlock.EndLotusStemBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusStemBlock.java:42)
- [ ] `EndLotusStemBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusStemBlock.java:51)
- [ ] `EndLotusStemBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusStemBlock.java:55)
- [ ] `EndLotusStemBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusStemBlock.java:59)
- [ ] `EndLotusStemBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusStemBlock.java:63)
- [ ] `EndLotusStemBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusStemBlock.java:70)
- [ ] `EndLotusStemBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusStemBlock.java:74)
- [ ] `EndLotusStemBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/EndLotusStemBlock.java:78)

### `EndPathBlock`: it extends Minecraft's `PathBlock`

- [ ] `EndPathBlock.EndPathBlock(2)`: its class isn't translated (it extends Minecraft's `PathBlock`) (org/betterx/betterend/blocks/EndPathBlock.java:12)
- [ ] `EndPathBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `PathBlock`) (org/betterx/betterend/blocks/EndPathBlock.java:17)

### `EndPedestal`: it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)

- [ ] `EndPedestal.MODEL`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/blocks/EndPedestal.java:26)
- [ ] `EndPedestal.EndPedestal(1)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EndPedestal.java:35)
- [ ] `EndPedestal.createTextureMapping(1)`: it uses Minecraft's `TextureMapping` (org/betterx/betterend/blocks/EndPedestal.java:39)
- [ ] `EndPedestal.buildModel(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/EndPedestal.java:74)

### `EndPedestal.ClientModel`

- [ ] `EndPedestal.ClientModel.buildFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/EndPedestal.java:80)

### `EndPedestal.ModelParams`: it uses Minecraft's `Block`

- [ ] `EndPedestal.ModelParams.ModelParams(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/blocks/EndPedestal.java:85)
- [ ] `EndPedestal.ModelParams.base(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/blocks/EndPedestal.java:85)

### `EndPedestal.Pedestal`: it uses Minecraft's `Block`

- [ ] `EndPedestal.Pedestal.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/blocks/EndPedestal.java:89)
- [ ] `EndPedestal.Pedestal.Pedestal(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/blocks/EndPedestal.java:88)
- [ ] `EndPedestal.Pedestal.base(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/blocks/EndPedestal.java:88)

### `EndPortalBlock`: it extends Minecraft's `NetherPortalBlock`

- [ ] `EndPortalBlock.PORTAL`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/EndPortalBlock.java:47)
- [ ] `EndPortalBlock.EndPortalBlock(1)`: its class isn't translated (it extends Minecraft's `NetherPortalBlock`) (org/betterx/betterend/blocks/EndPortalBlock.java:49)
- [ ] `EndPortalBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `NetherPortalBlock`) (org/betterx/betterend/blocks/EndPortalBlock.java:53)
- [ ] `EndPortalBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `NetherPortalBlock`) (org/betterx/betterend/blocks/EndPortalBlock.java:58)
- [ ] `EndPortalBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `NetherPortalBlock`) (org/betterx/betterend/blocks/EndPortalBlock.java:79)
- [ ] `EndPortalBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `NetherPortalBlock`) (org/betterx/betterend/blocks/EndPortalBlock.java:82)
- [ ] `EndPortalBlock.entityInside(6)`: its class isn't translated (it extends Minecraft's `NetherPortalBlock`) (org/betterx/betterend/blocks/EndPortalBlock.java:95)
- [ ] `EndPortalBlock.validate(1)`: its class isn't translated (it extends Minecraft's `NetherPortalBlock`) (org/betterx/betterend/blocks/EndPortalBlock.java:101)
- [ ] `EndPortalBlock.getPortalDestination(3)`: its class isn't translated (it extends Minecraft's `NetherPortalBlock`) (org/betterx/betterend/blocks/EndPortalBlock.java:105)
- [ ] `EndPortalBlock.getDestinationLevels(2)`: it uses Minecraft's `ServerLevel` (org/betterx/betterend/blocks/EndPortalBlock.java:137)
- [ ] `EndPortalBlock.getStartPos(3)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/blocks/EndPortalBlock.java:160)
- [ ] `EndPortalBlock.getTeleportTransition(4)`: it uses Minecraft's `TeleportTransition` (org/betterx/betterend/blocks/EndPortalBlock.java:165)
- [ ] `EndPortalBlock.getTeleportTransitionFromExit(5)`: it uses Minecraft's `TeleportTransition` (org/betterx/betterend/blocks/EndPortalBlock.java:177)
- [ ] `EndPortalBlock.createTeleportTransition(9)`: it uses Minecraft's `TeleportTransition` (org/betterx/betterend/blocks/EndPortalBlock.java:199)

### `EndstoneDustBlock`: it extends Minecraft's `FallingBlock`

- [ ] `EndstoneDustBlock.COLOR`: it calls `ColorUtilARGB32` (de.ambertation.wunderlib.utils).color (org/betterx/betterend/blocks/EndstoneDustBlock.java:13)
- [ ] `EndstoneDustBlock.EndstoneDustBlock(1)`: its class isn't translated (it extends Minecraft's `FallingBlock`) (org/betterx/betterend/blocks/EndstoneDustBlock.java:15)
- [ ] `EndstoneDustBlock.getDustColor(3)`: its class isn't translated (it extends Minecraft's `FallingBlock`) (org/betterx/betterend/blocks/EndstoneDustBlock.java:19)

### `EndStoneSmelter`: it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)

- [ ] `EndStoneSmelter.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/EndStoneSmelter.java:46)
- [ ] `EndStoneSmelter.LIT`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/EndStoneSmelter.java:47)
- [x] `EndStoneSmelter.ID` (org/betterx/betterend/blocks/EndStoneSmelter.java:48)
- [ ] `EndStoneSmelter.EndStoneSmelter(1)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:50)
- [ ] `EndStoneSmelter.useWithoutItem(5)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:55)
- [ ] `EndStoneSmelter.openScreen(3)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:64)
- [ ] `EndStoneSmelter.getStateForPlacement(1)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:71)
- [ ] `EndStoneSmelter.newBlockEntity(2)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:75)
- [ ] `EndStoneSmelter.getDrops(2)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:79)
- [ ] `EndStoneSmelter.hasAnalogOutputSignal(1)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:94)
- [ ] `EndStoneSmelter.getAnalogOutputSignal(4)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:98)
- [ ] `EndStoneSmelter.getRenderShape(1)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:102)
- [ ] `EndStoneSmelter.rotate(2)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:106)
- [ ] `EndStoneSmelter.mirror(2)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:110)
- [ ] `EndStoneSmelter.createBlockStateDefinition(1)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:114)
- [ ] `EndStoneSmelter.animateTick(4)`: its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)) (org/betterx/betterend/blocks/EndStoneSmelter.java:118)
- [ ] `EndStoneSmelter.getTicker(3)`: it's generic (org/betterx/betterend/blocks/EndStoneSmelter.java:138)

### `AlloyingRecipeInfo`: it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)

- [ ] `AlloyingRecipeInfo.AlloyingRecipeInfo(2)`: its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)) (org/betterx/betterend/blocks/entities/AlloyingRecipeInfo.java:7)
- [ ] `AlloyingRecipeInfo.burnTimeMultiplicator(0)`: its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)) (org/betterx/betterend/blocks/entities/AlloyingRecipeInfo.java:8)
- [ ] `AlloyingRecipeInfo.cookTimeIncrement(0)`: its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)) (org/betterx/betterend/blocks/entities/AlloyingRecipeInfo.java:12)
- [ ] `AlloyingRecipeInfo.input(0)`: its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)) (org/betterx/betterend/blocks/entities/AlloyingRecipeInfo.java:7)
- [ ] `AlloyingRecipeInfo.holder(0)`: its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)) (org/betterx/betterend/blocks/entities/AlloyingRecipeInfo.java:7)

### `BlastingRecipeInfo`: it uses Minecraft's `SingleRecipeInput`

- [ ] `BlastingRecipeInfo.BlastingRecipeInfo(2)`: its class isn't translated (it uses Minecraft's `SingleRecipeInput`) (org/betterx/betterend/blocks/entities/BlastingRecipeInfo.java:7)
- [ ] `BlastingRecipeInfo.burnTimeMultiplicator(0)`: its class isn't translated (it uses Minecraft's `SingleRecipeInput`) (org/betterx/betterend/blocks/entities/BlastingRecipeInfo.java:8)
- [ ] `BlastingRecipeInfo.cookTimeIncrement(0)`: its class isn't translated (it uses Minecraft's `SingleRecipeInput`) (org/betterx/betterend/blocks/entities/BlastingRecipeInfo.java:12)
- [ ] `BlastingRecipeInfo.input(0)`: its class isn't translated (it uses Minecraft's `SingleRecipeInput`) (org/betterx/betterend/blocks/entities/BlastingRecipeInfo.java:7)
- [ ] `BlastingRecipeInfo.holder(0)`: its class isn't translated (it uses Minecraft's `SingleRecipeInput`) (org/betterx/betterend/blocks/entities/BlastingRecipeInfo.java:7)

### `BlockEntityHydrothermalVent`: it extends Minecraft's `BlockEntity`

- [ ] `BlockEntityHydrothermalVent.POSITIVE_Y`: it uses Minecraft's `Vec3` (org/betterx/betterend/blocks/entities/BlockEntityHydrothermalVent.java:22)
- [ ] `BlockEntityHydrothermalVent.BlockEntityHydrothermalVent(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/BlockEntityHydrothermalVent.java:24)
- [ ] `BlockEntityHydrothermalVent.tick(4)`: it's generic (org/betterx/betterend/blocks/entities/BlockEntityHydrothermalVent.java:28)
- [ ] `BlockEntityHydrothermalVent.clientTick(4)`: it uses Minecraft's `Level` (org/betterx/betterend/blocks/entities/BlockEntityHydrothermalVent.java:39)
- [ ] `BlockEntityHydrothermalVent.serverTick(4)`: it uses Minecraft's `Level` (org/betterx/betterend/blocks/entities/BlockEntityHydrothermalVent.java:49)
- [ ] `BlockEntityHydrothermalVent.hasElytra(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/BlockEntityHydrothermalVent.java:74)

### `EndStoneSmelterBlockEntity`: it extends Minecraft's `BaseContainerBlockEntity`

- [ ] `EndStoneSmelterBlockEntity.RECIPES_USED_CODEC`: it uses Minecraft's `Codec` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:73)
- [x] `EndStoneSmelterBlockEntity.TOP_SLOTS` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:74)
- [x] `EndStoneSmelterBlockEntity.BOTTOM_SLOTS` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:75)
- [x] `EndStoneSmelterBlockEntity.SIDE_SLOTS` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:76)
- [ ] `EndStoneSmelterBlockEntity.AVAILABLE_FUELS`: it uses Minecraft's `Item` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:77)
- [ ] `EndStoneSmelterBlockEntity.LONG_BURNING_FUELS`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:88)
- [ ] `EndStoneSmelterBlockEntity.EndStoneSmelterBlockEntity(2)`: its body couldn't be read: an anonymous class (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:99)
- [ ] `EndStoneSmelterBlockEntity.isLit(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:135)
- [ ] `EndStoneSmelterBlockEntity.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:139)
- [ ] `EndStoneSmelterBlockEntity.isEmpty(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:143)
- [ ] `EndStoneSmelterBlockEntity.getItem(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:153)
- [ ] `EndStoneSmelterBlockEntity.removeItem(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:158)
- [ ] `EndStoneSmelterBlockEntity.removeItemNoUpdate(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:163)
- [ ] `EndStoneSmelterBlockEntity.getTotalCookTime(1)`: it uses MultiRecipeInfo, which isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:168)
- [ ] `EndStoneSmelterBlockEntity.setLitState(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:178)
- [ ] `EndStoneSmelterBlockEntity.setItem(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:185)
- [ ] `EndStoneSmelterBlockEntity.getSmeltTime(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:197)
- [ ] `EndStoneSmelterBlockEntity.getRecipesToAwardAndPopExperience(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:211)
- [ ] `EndStoneSmelterBlockEntity.dropExperience(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:231)
- [ ] `EndStoneSmelterBlockEntity.dropExperience(4)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:244)
- [ ] `EndStoneSmelterBlockEntity.stillValid(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:254)
- [ ] `EndStoneSmelterBlockEntity.preRemoveSideEffects(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:260)
- [ ] `EndStoneSmelterBlockEntity.clearContent(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:267)
- [ ] `EndStoneSmelterBlockEntity.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:271)
- [ ] `EndStoneSmelterBlockEntity.getItems(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:276)
- [ ] `EndStoneSmelterBlockEntity.setItems(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:281)
- [ ] `EndStoneSmelterBlockEntity.createMenu(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:285)
- [ ] `EndStoneSmelterBlockEntity.serverTick(4)`: it uses Minecraft's `ServerLevel` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:290)
- [ ] `EndStoneSmelterBlockEntity.burn(3)`: it uses Minecraft's `RecipeHolder` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:365)
- [ ] `EndStoneSmelterBlockEntity.burn(4)`: it uses Minecraft's `RecipeHolder` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:371)
- [ ] `EndStoneSmelterBlockEntity.burn(4)`: it uses Minecraft's `RecipeHolder` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:383)
- [ ] `EndStoneSmelterBlockEntity.createResultItem(2)`: it uses Minecraft's `NonNullList` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:400)
- [ ] `EndStoneSmelterBlockEntity.canBurn(4)`: it uses Minecraft's `RegistryAccess` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:409)
- [ ] `EndStoneSmelterBlockEntity.canBurn(5)`: it's overloaded with the same number of parameters (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:415)
- [ ] `EndStoneSmelterBlockEntity.canBurn(5)`: it's overloaded with the same number of parameters (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:426)
- [ ] `EndStoneSmelterBlockEntity.canStore(3)`: it uses Minecraft's `NonNullList` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:437)
- [ ] `EndStoneSmelterBlockEntity.fillStackedContents(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:455)
- [ ] `EndStoneSmelterBlockEntity.setRecipeUsed(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:461)
- [ ] `EndStoneSmelterBlockEntity.getRecipeUsed(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:468)
- [ ] `EndStoneSmelterBlockEntity.getSlotsForFace(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:472)
- [ ] `EndStoneSmelterBlockEntity.canPlaceItemThroughFace(3)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:480)
- [ ] `EndStoneSmelterBlockEntity.canTakeItemThroughFace(3)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:484)
- [ ] `EndStoneSmelterBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:488)
- [ ] `EndStoneSmelterBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:505)
- [ ] `EndStoneSmelterBlockEntity.canPlaceItem(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:515)
- [ ] `EndStoneSmelterBlockEntity.canUseAsFuel(1)`: it uses EndStoneSmelterBlockEntity.AVAILABLE_FUELS, which isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:528)
- [ ] `EndStoneSmelterBlockEntity.longBurningTicks(1)`: it uses Minecraft's `CookingFuel` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:532)
- [ ] `EndStoneSmelterBlockEntity.smelterFuels(0)`: it uses Minecraft's `BuiltInRegistries`.ITEM (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:537)
- [ ] `EndStoneSmelterBlockEntity.getBurnDuration(3)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:556)
- [ ] `EndStoneSmelterBlockEntity.resolveVanillaBurnDuration(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:566)
- [ ] `EndStoneSmelterBlockEntity.lootContext(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:570)
- [ ] `EndStoneSmelterBlockEntity.registerFuel(2)`: it uses Minecraft's `ItemLike` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:582)
- [ ] `EndStoneSmelterBlockEntity.availableFuels(0)`: it uses EndStoneSmelterBlockEntity.AVAILABLE_FUELS, which isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java:586)

### `EternalPedestalEntity`: it extends `PedestalBlockEntity`

- [ ] `EternalPedestalEntity.EternalPedestalEntity(2)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/EternalPedestalEntity.java:15)
- [ ] `EternalPedestalEntity.hasRitual(0)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/EternalPedestalEntity.java:19)
- [ ] `EternalPedestalEntity.linkRitual(1)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/EternalPedestalEntity.java:23)
- [ ] `EternalPedestalEntity.getRitual(0)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/EternalPedestalEntity.java:27)
- [ ] `EternalPedestalEntity.setLevel(1)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/EternalPedestalEntity.java:31)
- [ ] `EternalPedestalEntity.saveAdditional(1)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/EternalPedestalEntity.java:38)
- [ ] `EternalPedestalEntity.loadAdditional(1)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/EternalPedestalEntity.java:47)

### `FlowerPotBlockEntity`: it extends Minecraft's `BlockEntity`

- [ ] `FlowerPotBlockEntity.FlowerPotBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:25)
- [ ] `FlowerPotBlockEntity.getPlant(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:29)
- [ ] `FlowerPotBlockEntity.getSoil(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:33)
- [ ] `FlowerPotBlockEntity.getPlantBlock(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:37)
- [ ] `FlowerPotBlockEntity.getSoilBlock(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:41)
- [ ] `FlowerPotBlockEntity.setPlant(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:45)
- [ ] `FlowerPotBlockEntity.setSoil(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:50)
- [ ] `FlowerPotBlockEntity.setChanged(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:55)
- [ ] `FlowerPotBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:71)
- [ ] `FlowerPotBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:77)
- [ ] `FlowerPotBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:83)
- [ ] `FlowerPotBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java:87)

### `InfusionPedestalEntity`: it extends `PedestalBlockEntity`

- [ ] `InfusionPedestalEntity.InfusionPedestalEntity(2)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:19)
- [ ] `InfusionPedestalEntity.setLevel(1)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:23)
- [ ] `InfusionPedestalEntity.linkRitual(3)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:32)
- [ ] `InfusionPedestalEntity.getRitual(0)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:38)
- [ ] `InfusionPedestalEntity.hasRitual(0)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:42)
- [ ] `InfusionPedestalEntity.saveAdditional(1)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:46)
- [ ] `InfusionPedestalEntity.loadAdditional(1)`: its class isn't translated (it extends `PedestalBlockEntity`) (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:55)
- [ ] `InfusionPedestalEntity.tickEntity(4)`: it's generic (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:68)

### `InfusionPedestalEntity.ClientHooks`

- [ ] `InfusionPedestalEntity.ClientHooks.spawnHintMist(2)`: it uses Minecraft's `Level` (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java:82)

### `InputState`: it uses Minecraft's `ItemStack`

- [ ] `InputState.InputState(2)`: its class isn't translated (it uses Minecraft's `ItemStack`) (org/betterx/betterend/blocks/entities/InputState.java:5)
- [ ] `InputState.singleInput(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (org/betterx/betterend/blocks/entities/InputState.java:6)
- [ ] `InputState.hasOneInput(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (org/betterx/betterend/blocks/entities/InputState.java:10)
- [ ] `InputState.hasBothInputs(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (org/betterx/betterend/blocks/entities/InputState.java:14)
- [ ] `InputState.hasInput(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (org/betterx/betterend/blocks/entities/InputState.java:18)
- [ ] `InputState.inputA(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (org/betterx/betterend/blocks/entities/InputState.java:5)
- [ ] `InputState.inputB(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (org/betterx/betterend/blocks/entities/InputState.java:5)

### `MultiRecipeInfo`: it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)

- [ ] `MultiRecipeInfo.MultiRecipeInfo(3)`: its class isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)) (org/betterx/betterend/blocks/entities/MultiRecipeInfo.java:14)
- [ ] `MultiRecipeInfo.isBlasting(0)`: its class isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)) (org/betterx/betterend/blocks/entities/MultiRecipeInfo.java:20)
- [ ] `MultiRecipeInfo.burnTimeMultiplicator(0)`: its class isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)) (org/betterx/betterend/blocks/entities/MultiRecipeInfo.java:24)
- [ ] `MultiRecipeInfo.cookTimeIncrement(0)`: its class isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)) (org/betterx/betterend/blocks/entities/MultiRecipeInfo.java:28)
- [ ] `MultiRecipeInfo.of(3)`: it uses MultiRecipeInfo, which isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)) (org/betterx/betterend/blocks/entities/MultiRecipeInfo.java:32)
- [ ] `MultiRecipeInfo.of(3)`: it uses MultiRecipeInfo, which isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)) (org/betterx/betterend/blocks/entities/MultiRecipeInfo.java:36)

### `PedestalBlockEntity`: it extends Minecraft's `BlockEntity`

- [ ] `PedestalBlockEntity.PedestalBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:23)
- [ ] `PedestalBlockEntity.PedestalBlockEntity(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:27)
- [ ] `PedestalBlockEntity.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:31)
- [ ] `PedestalBlockEntity.isEmpty(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:35)
- [ ] `PedestalBlockEntity.getItem(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:39)
- [ ] `PedestalBlockEntity.removeItem(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:44)
- [ ] `PedestalBlockEntity.canPlaceItem(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:49)
- [ ] `PedestalBlockEntity.clearContent(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:53)
- [ ] `PedestalBlockEntity.removeItemNoUpdate(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:58)
- [ ] `PedestalBlockEntity.setItem(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:65)
- [ ] `PedestalBlockEntity.setChanged(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:70)
- [ ] `PedestalBlockEntity.stillValid(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:85)
- [ ] `PedestalBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:89)
- [ ] `PedestalBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:94)
- [ ] `PedestalBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:102)
- [ ] `PedestalBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java:106)

### `EternalPedestal`: it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)

- [ ] `EternalPedestal.ACTIVATED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/EternalPedestal.java:63)
- [ ] `EternalPedestal.EternalPedestal(1)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:65)
- [ ] `EternalPedestal.checkRitual(3)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:70)
- [ ] `EternalPedestal.updateShape(8)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:120)
- [ ] `EternalPedestal.getDestroyProgress(4)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:140)
- [ ] `EternalPedestal.getExplosionResistance(0)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:145)
- [ ] `EternalPedestal.dropFromExplosion(1)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:149)
- [ ] `EternalPedestal.getDrops(2)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:153)
- [ ] `EternalPedestal.createBlockStateDefinition(1)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:172)
- [ ] `EternalPedestal.newBlockEntity(2)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:178)
- [ ] `EternalPedestal.hasUniqueEntity(0)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:183)
- [ ] `EternalPedestal.useItemOn(7)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:188)
- [ ] `EternalPedestal.dispatchParticles(3)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:202)
- [ ] `EternalPedestal.animateTick(4)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:256)
- [ ] `EternalPedestal.createVariants(6)`: it uses Minecraft's `MultiVariant` (org/betterx/betterend/blocks/EternalPedestal.java:263)
- [ ] `EternalPedestal.provideBlockModelsInstance(1)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/EternalPedestal.java:278)

### `EternalPedestal.ClientHooks`

- [ ] `EternalPedestal.ClientHooks.showVision(2)`: it uses Minecraft's `Level` (org/betterx/betterend/blocks/EternalPedestal.java:326)
- [ ] `EternalPedestal.ClientHooks.tickVision(3)`: it uses Minecraft's `Level` (org/betterx/betterend/blocks/EternalPedestal.java:330)

### `FilaluxBlock`: it extends `BaseVineBlock` (org.betterx.bclib.blocks)

- [ ] `FilaluxBlock.FilaluxBlock(1)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/FilaluxBlock.java:7)

### `FilaluxLanternBlock`: it extends Minecraft's `Block`

- [ ] `FilaluxLanternBlock.FilaluxLanternBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/FilaluxLanternBlock.java:7)

### `FilaluxWingsBlock`: it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)

- [ ] `FilaluxWingsBlock.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/FilaluxWingsBlock.java:16)
- [ ] `FilaluxWingsBlock.FilaluxWingsBlock(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/FilaluxWingsBlock.java:18)
- [ ] `FilaluxWingsBlock.getShape(4)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/FilaluxWingsBlock.java:22)

### `FlamaeaBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `FlamaeaBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/FlamaeaBlock.java:13)
- [ ] `FlamaeaBlock.FlamaeaBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/FlamaeaBlock.java:15)
- [ ] `FlamaeaBlock.getShape(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/FlamaeaBlock.java:19)

### `FlammalixBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `FlammalixBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/FlammalixBlock.java:13)
- [ ] `FlammalixBlock.FlammalixBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/FlammalixBlock.java:15)
- [ ] `FlammalixBlock.getShape(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/FlammalixBlock.java:19)

### `FlowerPotBlock`: it extends Minecraft's `Block`

- [ ] `FlowerPotBlock.POT_LIGHT`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/FlowerPotBlock.java:56)
- [ ] `FlowerPotBlock.SHAPE_EMPTY`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/FlowerPotBlock.java:57)
- [ ] `FlowerPotBlock.SHAPE_FULL`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/FlowerPotBlock.java:58)
- [ ] `FlowerPotBlock.MODEL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/blocks/FlowerPotBlock.java:59)
- [ ] `FlowerPotBlock.FlowerPotBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/FlowerPotBlock.java:62)
- [ ] `FlowerPotBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/FlowerPotBlock.java:67)
- [ ] `FlowerPotBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/FlowerPotBlock.java:72)
- [ ] `FlowerPotBlock.getDrops(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/FlowerPotBlock.java:76)
- [ ] `FlowerPotBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/FlowerPotBlock.java:87)
- [ ] `FlowerPotBlock.findByBlock(3)`: it's generic (org/betterx/betterend/blocks/FlowerPotBlock.java:166)
- [ ] `FlowerPotBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/FlowerPotBlock.java:176)
- [ ] `FlowerPotBlock.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/FlowerPotBlock.java:180)
- [ ] `FlowerPotBlock.buildModel(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/FlowerPotBlock.java:184)

### `FlowerPotBlock.ClientModel`

- [ ] `FlowerPotBlock.ClientModel.build(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/FlowerPotBlock.java:190)

### `GlowingHymenophoreBlock`: it extends Minecraft's `Block`

- [ ] `GlowingHymenophoreBlock.GlowingHymenophoreBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/GlowingHymenophoreBlock.java:13)
- [ ] `GlowingHymenophoreBlock.provideUnshadedCubeModel(2)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/GlowingHymenophoreBlock.java:17)

### `GlowingMossBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `GlowingMossBlock.GlowingMossBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingMossBlock.java:11)
- [ ] `GlowingMossBlock.hasEmissiveLighting(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingMossBlock.java:15)
- [ ] `GlowingMossBlock.getAmbientOcclusionLightLevel(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingMossBlock.java:20)

### `GlowingPillarLuminophorBlock`: it extends Minecraft's `Block`

- [ ] `GlowingPillarLuminophorBlock.NATURAL`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/GlowingPillarLuminophorBlock.java:18)
- [ ] `GlowingPillarLuminophorBlock.GlowingPillarLuminophorBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/GlowingPillarLuminophorBlock.java:20)
- [ ] `GlowingPillarLuminophorBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/GlowingPillarLuminophorBlock.java:25)
- [ ] `GlowingPillarLuminophorBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/GlowingPillarLuminophorBlock.java:29)
- [ ] `GlowingPillarLuminophorBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/GlowingPillarLuminophorBlock.java:42)

### `GlowingPillarRootsBlock`: it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)

- [ ] `GlowingPillarRootsBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/GlowingPillarRootsBlock.java:21)
- [ ] `GlowingPillarRootsBlock.GlowingPillarRootsBlock(1)`: its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingPillarRootsBlock.java:23)
- [ ] `GlowingPillarRootsBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingPillarRootsBlock.java:27)
- [ ] `GlowingPillarRootsBlock.isTerrain(1)`: its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingPillarRootsBlock.java:31)
- [ ] `GlowingPillarRootsBlock.getCloneItemStack(4)`: its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingPillarRootsBlock.java:35)

### `GlowingPillarSeedBlock`: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `GlowingPillarSeedBlock.GlowingPillarSeedBlock(1)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingPillarSeedBlock.java:21)
- [ ] `GlowingPillarSeedBlock.growAdult(3)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/GlowingPillarSeedBlock.java:25)

### `HelixTreeLeavesBlock`: it extends Minecraft's `Block`

- [ ] `HelixTreeLeavesBlock.COLOR`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/HelixTreeLeavesBlock.java:14)
- [ ] `HelixTreeLeavesBlock.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/blocks/HelixTreeLeavesBlock.java:15)
- [ ] `HelixTreeLeavesBlock.HelixTreeLeavesBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HelixTreeLeavesBlock.java:17)
- [ ] `HelixTreeLeavesBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HelixTreeLeavesBlock.java:21)
- [ ] `HelixTreeLeavesBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HelixTreeLeavesBlock.java:25)

### `HelixTreeSaplingBlock`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `HelixTreeSaplingBlock.HelixTreeSaplingBlock(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betterend/blocks/HelixTreeSaplingBlock.java:8)

### `HydraluxBlock`: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)

- [ ] `HydraluxBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/HydraluxBlock.java:32)
- [ ] `HydraluxBlock.HydraluxBlock(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/HydraluxBlock.java:34)
- [ ] `HydraluxBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/HydraluxBlock.java:38)
- [ ] `HydraluxBlock.canSurvive(3)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/HydraluxBlock.java:42)
- [ ] `HydraluxBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/HydraluxBlock.java:54)
- [ ] `HydraluxBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/HydraluxBlock.java:58)
- [ ] `HydraluxBlock.getCloneItemStack(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/HydraluxBlock.java:62)
- [ ] `HydraluxBlock.buildLoot(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/HydraluxBlock.java:67)
- [ ] `HydraluxBlock.shapeCond(3)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/HydraluxBlock.java:98)

### `HydraluxPetalBlock`: it extends Minecraft's `Block`

- [ ] `HydraluxPetalBlock.HydraluxPetalBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydraluxPetalBlock.java:12)
- [ ] `HydraluxPetalBlock.fallOn(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydraluxPetalBlock.java:16)

### `HydraluxPetalColoredBlock`: it extends `HydraluxPetalBlock`

- [x] `HydraluxPetalColoredBlock.PETAL_MODEL` (org/betterx/betterend/blocks/HydraluxPetalColoredBlock.java:17)
- [ ] `HydraluxPetalColoredBlock.HydraluxPetalColoredBlock(1)`: its class isn't translated (it extends `HydraluxPetalBlock`) (org/betterx/betterend/blocks/HydraluxPetalColoredBlock.java:19)
- [ ] `HydraluxPetalColoredBlock.provideBlockModel(2)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/HydraluxPetalColoredBlock.java:23)

### `HydraluxSaplingBlock`: it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `HydraluxSaplingBlock.HydraluxSaplingBlock(1)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/HydraluxSaplingBlock.java:16)
- [ ] `HydraluxSaplingBlock.grow(3)`: its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/HydraluxSaplingBlock.java:20)

### `HydrothermalVentBlock`: it extends Minecraft's `Block`

- [ ] `HydrothermalVentBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/HydrothermalVentBlock.java:45)
- [ ] `HydrothermalVentBlock.ACTIVATED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/HydrothermalVentBlock.java:46)
- [ ] `HydrothermalVentBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/HydrothermalVentBlock.java:47)
- [ ] `HydrothermalVentBlock.HydrothermalVentBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:49)
- [ ] `HydrothermalVentBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:54)
- [ ] `HydrothermalVentBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:58)
- [ ] `HydrothermalVentBlock.canPlaceLiquid(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:62)
- [ ] `HydrothermalVentBlock.placeLiquid(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:66)
- [ ] `HydrothermalVentBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:70)
- [ ] `HydrothermalVentBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:75)
- [ ] `HydrothermalVentBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:96)
- [ ] `HydrothermalVentBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:102)
- [ ] `HydrothermalVentBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:106)
- [ ] `HydrothermalVentBlock.tick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:110)
- [ ] `HydrothermalVentBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:118)
- [ ] `HydrothermalVentBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/HydrothermalVentBlock.java:124)
- [ ] `HydrothermalVentBlock.getTicker(3)`: it's generic (org/betterx/betterend/blocks/HydrothermalVentBlock.java:135)

### `InfusionPedestal`: it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)

- [ ] `InfusionPedestal.SHAPE_DEFAULT`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/InfusionPedestal.java:41)
- [ ] `InfusionPedestal.SHAPE_PEDESTAL_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/InfusionPedestal.java:42)
- [ ] `InfusionPedestal.InfusionPedestal(1)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:44)
- [ ] `InfusionPedestal.checkRitual(3)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:49)
- [ ] `InfusionPedestal.setPlacedBy(5)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:66)
- [ ] `InfusionPedestal.useItemOn(7)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:74)
- [ ] `InfusionPedestal.newBlockEntity(2)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:99)
- [ ] `InfusionPedestal.hasUniqueEntity(0)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:104)
- [ ] `InfusionPedestal.getShape(4)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:109)
- [ ] `InfusionPedestal.getTicker(3)`: it's generic (org/betterx/betterend/blocks/InfusionPedestal.java:124)
- [ ] `InfusionPedestal.createTextureMapping(0)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:131)
- [ ] `InfusionPedestal.provideBlockModelsInstance(1)`: its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)) (org/betterx/betterend/blocks/InfusionPedestal.java:141)

### `InfusionPedestal.ClientHooks`

- [ ] `InfusionPedestal.ClientHooks.showHint(2)`: it uses Minecraft's `Level` (org/betterx/betterend/blocks/InfusionPedestal.java:162)
- [x] `InfusionPedestal.ClientHooks.showRecipes(0)` in game_side.rs (org/betterx/betterend/blocks/InfusionPedestal.java:166)

### `InfusionPedestal.Models`

- [ ] `InfusionPedestal.Models.PEDESTAL_MODELS`: it uses PedestalState, which isn't translated (its constants carry values) (org/betterx/betterend/blocks/InfusionPedestal.java:173)

### `JellyshroomCapBlock`: it extends Minecraft's `SlimeBlock`

- [ ] `JellyshroomCapBlock.COLOR`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/JellyshroomCapBlock.java:24)
- [ ] `JellyshroomCapBlock.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/blocks/JellyshroomCapBlock.java:25)
- [ ] `JellyshroomCapBlock.JellyshroomCapBlock(1)`: its class isn't translated (it extends Minecraft's `SlimeBlock`) (org/betterx/betterend/blocks/JellyshroomCapBlock.java:27)
- [ ] `JellyshroomCapBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `SlimeBlock`) (org/betterx/betterend/blocks/JellyshroomCapBlock.java:31)
- [ ] `JellyshroomCapBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `SlimeBlock`) (org/betterx/betterend/blocks/JellyshroomCapBlock.java:38)
- [ ] `JellyshroomCapBlock.provideBlockModel(2)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/JellyshroomCapBlock.java:42)

### `LacugroveSaplingBlock`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `LacugroveSaplingBlock.LacugroveSaplingBlock(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betterend/blocks/LacugroveSaplingBlock.java:8)

### `LanceleafBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `LanceleafBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/LanceleafBlock.java:31)
- [ ] `LanceleafBlock.ROTATION`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/LanceleafBlock.java:32)
- [ ] `LanceleafBlock.LanceleafBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LanceleafBlock.java:34)
- [ ] `LanceleafBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LanceleafBlock.java:38)
- [ ] `LanceleafBlock.canSurvive(3)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LanceleafBlock.java:42)
- [ ] `LanceleafBlock.updateShape(8)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LanceleafBlock.java:53)
- [ ] `LanceleafBlock.isBottom(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/LanceleafBlock.java:66)
- [ ] `LanceleafBlock.buildLoot(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/LanceleafBlock.java:74)

### `LanceleafSeedBlock`: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `LanceleafSeedBlock.LanceleafSeedBlock(1)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LanceleafSeedBlock.java:18)
- [ ] `LanceleafSeedBlock.growAdult(3)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LanceleafSeedBlock.java:22)

### `LargeAmaranitaBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `LargeAmaranitaBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:29)
- [ ] `LargeAmaranitaBlock.SHAPE_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:30)
- [ ] `LargeAmaranitaBlock.SHAPE_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:31)
- [ ] `LargeAmaranitaBlock.LargeAmaranitaBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:33)
- [ ] `LargeAmaranitaBlock.getShape(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:38)
- [ ] `LargeAmaranitaBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:42)
- [ ] `LargeAmaranitaBlock.canSurvive(3)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:46)
- [ ] `LargeAmaranitaBlock.getStateForPlacement(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:51)
- [ ] `LargeAmaranitaBlock.updateShape(8)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:70)
- [ ] `LargeAmaranitaBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:101)
- [ ] `LargeAmaranitaBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LargeAmaranitaBlock.java:105)

### `LucerniaSaplingBlock`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `LucerniaSaplingBlock.LucerniaSaplingBlock(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betterend/blocks/LucerniaSaplingBlock.java:8)

### `LumecornBlock`: it extends Minecraft's `Block`

- [ ] `LumecornBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/LumecornBlock.java:34)
- [ ] `LumecornBlock.SHAPE_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/LumecornBlock.java:35)
- [ ] `LumecornBlock.SHAPE_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/LumecornBlock.java:36)
- [ ] `LumecornBlock.LumecornBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/LumecornBlock.java:38)
- [ ] `LumecornBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/LumecornBlock.java:42)
- [ ] `LumecornBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/LumecornBlock.java:46)
- [ ] `LumecornBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/LumecornBlock.java:50)
- [ ] `LumecornBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/LumecornBlock.java:61)
- [ ] `LumecornBlock.buildLoot(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/LumecornBlock.java:74)
- [ ] `LumecornBlock.shapeIs(3)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/LumecornBlock.java:97)
- [ ] `LumecornBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/LumecornBlock.java:105)

### `LumecornSeedBlock`: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)

- [ ] `LumecornSeedBlock.LumecornSeedBlock(1)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LumecornSeedBlock.java:11)
- [ ] `LumecornSeedBlock.growAdult(3)`: its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/LumecornSeedBlock.java:15)

### `MengerSpongeBlock`: it extends Minecraft's `Block`

- [ ] `MengerSpongeBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/MengerSpongeBlock.java:31)
- [ ] `MengerSpongeBlock.MengerSpongeBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeBlock.java:46)
- [ ] `MengerSpongeBlock.onPlace(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeBlock.java:50)
- [ ] `MengerSpongeBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeBlock.java:56)
- [ ] `MengerSpongeBlock.absorbWater(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeBlock.java:72)
- [ ] `MengerSpongeBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeBlock.java:119)

### `MengerSpongeWetBlock`: it extends Minecraft's `Block`

- [ ] `MengerSpongeWetBlock.MengerSpongeWetBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeWetBlock.java:32)
- [ ] `MengerSpongeWetBlock.onPlace(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeWetBlock.java:36)
- [ ] `MengerSpongeWetBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeWetBlock.java:44)
- [ ] `MengerSpongeWetBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeWetBlock.java:82)
- [ ] `MengerSpongeWetBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeWetBlock.java:98)
- [ ] `MengerSpongeWetBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MengerSpongeWetBlock.java:102)

### `MossyDragonBoneBlock`: it extends Minecraft's `RotatedPillarBlock`

- [ ] `MossyDragonBoneBlock.MossyDragonBoneBlock(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betterend/blocks/MossyDragonBoneBlock.java:15)
- [ ] `MossyDragonBoneBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betterend/blocks/MossyDragonBoneBlock.java:19)
- [ ] `MossyDragonBoneBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betterend/blocks/MossyDragonBoneBlock.java:25)

### `MossyGlowshroomCapBlock`: it extends Minecraft's `Block`

- [ ] `MossyGlowshroomCapBlock.TRANSITION`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/MossyGlowshroomCapBlock.java:13)
- [ ] `MossyGlowshroomCapBlock.MossyGlowshroomCapBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MossyGlowshroomCapBlock.java:15)
- [ ] `MossyGlowshroomCapBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MossyGlowshroomCapBlock.java:20)
- [ ] `MossyGlowshroomCapBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MossyGlowshroomCapBlock.java:25)

### `MossyGlowshroomSaplingBlock`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `MossyGlowshroomSaplingBlock.MossyGlowshroomSaplingBlock(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betterend/blocks/MossyGlowshroomSaplingBlock.java:8)

### `MossyObsidian`: it extends Minecraft's `Block`

- [ ] `MossyObsidian.MossyObsidian(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MossyObsidian.java:14)
- [ ] `MossyObsidian.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MossyObsidian.java:18)
- [ ] `MossyObsidian.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/MossyObsidian.java:24)

### `MurkweedBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `MurkweedBlock.MurkweedBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/MurkweedBlock.java:21)
- [ ] `MurkweedBlock.animateTick(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/MurkweedBlock.java:25)
- [ ] `MurkweedBlock.entityInside(6)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/MurkweedBlock.java:34)
- [ ] `MurkweedBlock.isPathfindable(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/MurkweedBlock.java:40)

### `NeedlegrassBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `NeedlegrassBlock.NeedlegrassBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/NeedlegrassBlock.java:23)
- [ ] `NeedlegrassBlock.entityInside(6)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/NeedlegrassBlock.java:27)
- [ ] `NeedlegrassBlock.buildLoot(2)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/blocks/NeedlegrassBlock.java:33)
- [ ] `NeedlegrassBlock.isPathfindable(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/NeedlegrassBlock.java:53)

### `NeonCactusPlantBlock`: it extends Minecraft's `Block`

- [ ] `NeonCactusPlantBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:47)
- [ ] `NeonCactusPlantBlock.CACTUS_BOTTOM`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:48)
- [ ] `NeonCactusPlantBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:49)
- [ ] `NeonCactusPlantBlock.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:50)
- [ ] `NeonCactusPlantBlock.BIG_SHAPES_OPEN`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:51)
- [ ] `NeonCactusPlantBlock.MEDIUM_SHAPES_OPEN`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:52)
- [ ] `NeonCactusPlantBlock.SMALL_SHAPES_OPEN`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:53)
- [ ] `NeonCactusPlantBlock.BIG_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:54)
- [ ] `NeonCactusPlantBlock.MEDIUM_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:55)
- [ ] `NeonCactusPlantBlock.SMALL_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:56)
- [x] `NeonCactusPlantBlock.MAX_LENGTH` (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:57)
- [ ] `NeonCactusPlantBlock.NeonCactusPlantBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:59)
- [ ] `NeonCactusPlantBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:67)
- [ ] `NeonCactusPlantBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:71)
- [ ] `NeonCactusPlantBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:89)
- [ ] `NeonCactusPlantBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:93)
- [ ] `NeonCactusPlantBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:97)
- [ ] `NeonCactusPlantBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:101)
- [ ] `NeonCactusPlantBlock.tick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:132)
- [ ] `NeonCactusPlantBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:138)
- [ ] `NeonCactusPlantBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:156)
- [ ] `NeonCactusPlantBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:163)
- [ ] `NeonCactusPlantBlock.growPlant(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:200)
- [ ] `NeonCactusPlantBlock.growPlant(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:204)
- [ ] `NeonCactusPlantBlock.growIteration(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:231)
- [ ] `NeonCactusPlantBlock.getSideDirection(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:273)
- [ ] `NeonCactusPlantBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:304)
- [ ] `NeonCactusPlantBlock.entityInside(6)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:308)
- [ ] `NeonCactusPlantBlock.getLength(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:314)
- [ ] `NeonCactusPlantBlock.getHorizontal(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:336)
- [ ] `NeonCactusPlantBlock.mutateStem(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/NeonCactusPlantBlock.java:359)

### `PallidiumBlock`: it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)

- [ ] `PallidiumBlock.PallidiumBlock(3)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/PallidiumBlock.java:25)
- [ ] `PallidiumBlock.getBaseBlock(0)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/PallidiumBlock.java:30)
- [ ] `PallidiumBlock.useItemOn(7)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/PallidiumBlock.java:34)

### `PondAnemoneBlock`: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)

- [ ] `PondAnemoneBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/PondAnemoneBlock.java:20)
- [ ] `PondAnemoneBlock.PondAnemoneBlock(1)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/PondAnemoneBlock.java:22)
- [ ] `PondAnemoneBlock.animateTick(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/PondAnemoneBlock.java:26)
- [ ] `PondAnemoneBlock.getShape(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/PondAnemoneBlock.java:34)
- [ ] `PondAnemoneBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/PondAnemoneBlock.java:38)
- [ ] `PondAnemoneBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/PondAnemoneBlock.java:42)

### `PythadendronSaplingBlock`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `PythadendronSaplingBlock.PythadendronSaplingBlock(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betterend/blocks/PythadendronSaplingBlock.java:8)

### `RespawnObeliskBlock`: it extends Minecraft's `Block`

- [ ] `RespawnObeliskBlock.VOXEL_SHAPE_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/RespawnObeliskBlock.java:48)
- [ ] `RespawnObeliskBlock.VOXEL_SHAPE_MIDDLE_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/RespawnObeliskBlock.java:49)
- [ ] `RespawnObeliskBlock.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/RespawnObeliskBlock.java:50)
- [ ] `RespawnObeliskBlock.RespawnObeliskBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RespawnObeliskBlock.java:52)
- [ ] `RespawnObeliskBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RespawnObeliskBlock.java:56)
- [ ] `RespawnObeliskBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RespawnObeliskBlock.java:60)
- [ ] `RespawnObeliskBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RespawnObeliskBlock.java:64)
- [ ] `RespawnObeliskBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RespawnObeliskBlock.java:74)
- [ ] `RespawnObeliskBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RespawnObeliskBlock.java:81)
- [ ] `RespawnObeliskBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RespawnObeliskBlock.java:101)
- [ ] `RespawnObeliskBlock.buildLoot(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/RespawnObeliskBlock.java:114)
- [ ] `RespawnObeliskBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RespawnObeliskBlock.java:130)

### `RunedFlavolite`: it extends Minecraft's `Block`

- [ ] `RunedFlavolite.ACTIVATED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/RunedFlavolite.java:14)
- [ ] `RunedFlavolite.RunedFlavolite(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RunedFlavolite.java:16)
- [ ] `RunedFlavolite.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RunedFlavolite.java:21)
- [ ] `RunedFlavolite.dropFromExplosion(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/RunedFlavolite.java:25)

### `ShadowBerryBlock`: it extends `PottableCropBlock` (de.ambertation.wover.block.api.plants)

- [ ] `ShadowBerryBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/ShadowBerryBlock.java:16)
- [ ] `ShadowBerryBlock.ShadowBerryBlock(1)`: its class isn't translated (it extends `PottableCropBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/ShadowBerryBlock.java:18)
- [ ] `ShadowBerryBlock.getShape(4)`: its class isn't translated (it extends `PottableCropBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/ShadowBerryBlock.java:22)

### `ShadowGrassBlock`: it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)

- [ ] `ShadowGrassBlock.ShadowGrassBlock(1)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/ShadowGrassBlock.java:15)
- [ ] `ShadowGrassBlock.animateTick(4)`: its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)) (org/betterx/betterend/blocks/ShadowGrassBlock.java:19)

### `SilkMothHiveBlock`: it extends Minecraft's `Block`

- [ ] `SilkMothHiveBlock.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/SilkMothHiveBlock.java:38)
- [ ] `SilkMothHiveBlock.FULLNESS`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/SilkMothHiveBlock.java:39)
- [ ] `SilkMothHiveBlock.SilkMothHiveBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothHiveBlock.java:41)
- [ ] `SilkMothHiveBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothHiveBlock.java:46)
- [ ] `SilkMothHiveBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothHiveBlock.java:50)
- [ ] `SilkMothHiveBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothHiveBlock.java:55)
- [ ] `SilkMothHiveBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothHiveBlock.java:59)
- [ ] `SilkMothHiveBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothHiveBlock.java:63)
- [ ] `SilkMothHiveBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothHiveBlock.java:79)

### `SilkMothNestBlock`: it extends Minecraft's `Block`

- [ ] `SilkMothNestBlock.ACTIVE`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/SilkMothNestBlock.java:51)
- [ ] `SilkMothNestBlock.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betterend/blocks/SilkMothNestBlock.java:52)
- [ ] `SilkMothNestBlock.FULLNESS`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/SilkMothNestBlock.java:53)
- [ ] `SilkMothNestBlock.TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/SilkMothNestBlock.java:54)
- [ ] `SilkMothNestBlock.BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/SilkMothNestBlock.java:55)
- [ ] `SilkMothNestBlock.SilkMothNestBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:57)
- [ ] `SilkMothNestBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:62)
- [ ] `SilkMothNestBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:66)
- [ ] `SilkMothNestBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:70)
- [ ] `SilkMothNestBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:75)
- [ ] `SilkMothNestBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:95)
- [ ] `SilkMothNestBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:99)
- [ ] `SilkMothNestBlock.buildLoot(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/SilkMothNestBlock.java:103)
- [ ] `SilkMothNestBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:119)
- [ ] `SilkMothNestBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:132)
- [ ] `SilkMothNestBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/SilkMothNestBlock.java:152)

### `SmallAmaranitaBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `SmallAmaranitaBlock.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:23)
- [ ] `SmallAmaranitaBlock.SmallAmaranitaBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:25)
- [ ] `SmallAmaranitaBlock.performBonemeal(5)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:29)
- [ ] `SmallAmaranitaBlock.getShape(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:41)
- [ ] `SmallAmaranitaBlock.growBig(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:46)
- [ ] `SmallAmaranitaBlock.checkFrame(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:59)
- [ ] `SmallAmaranitaBlock.replaceMushroom(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:66)
- [ ] `SmallAmaranitaBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:72)
- [ ] `SmallAmaranitaBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallAmaranitaBlock.java:76)

### `SmallJellyshroomBlock`: it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)

- [ ] `SmallJellyshroomBlock.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/SmallJellyshroomBlock.java:27)
- [ ] `SmallJellyshroomBlock.SmallJellyshroomBlock(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallJellyshroomBlock.java:29)
- [ ] `SmallJellyshroomBlock.getShape(4)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallJellyshroomBlock.java:33)
- [ ] `SmallJellyshroomBlock.canSurvive(3)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallJellyshroomBlock.java:37)
- [ ] `SmallJellyshroomBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallJellyshroomBlock.java:44)
- [ ] `SmallJellyshroomBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallJellyshroomBlock.java:48)
- [ ] `SmallJellyshroomBlock.performBonemeal(5)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmallJellyshroomBlock.java:52)

### `SmaragdantCrystalShardBlock`: it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)

- [ ] `SmaragdantCrystalShardBlock.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:31)
- [ ] `SmaragdantCrystalShardBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:32)
- [ ] `SmaragdantCrystalShardBlock.SmaragdantCrystalShardBlock(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:34)
- [ ] `SmaragdantCrystalShardBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:39)
- [ ] `SmaragdantCrystalShardBlock.canPlaceLiquid(5)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:44)
- [ ] `SmaragdantCrystalShardBlock.placeLiquid(4)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:48)
- [ ] `SmaragdantCrystalShardBlock.getStateForPlacement(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:52)
- [ ] `SmaragdantCrystalShardBlock.getFluidState(1)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:64)
- [ ] `SmaragdantCrystalShardBlock.getShape(4)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:68)
- [ ] `SmaragdantCrystalShardBlock.canSurvive(3)`: its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java:72)

### `SulphurCrystalBlock`: it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)

- [ ] `SulphurCrystalBlock.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:47)
- [x] `SulphurCrystalBlock.MAX_AGE` (org/betterx/betterend/blocks/SulphurCrystalBlock.java:48)
- [ ] `SulphurCrystalBlock.AGE`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/SulphurCrystalBlock.java:49)
- [ ] `SulphurCrystalBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/SulphurCrystalBlock.java:50)
- [ ] `SulphurCrystalBlock.SulphurCrystalBlock(1)`: its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:52)
- [ ] `SulphurCrystalBlock.naturalState(4)`: it uses Minecraft's `BlockState` (org/betterx/betterend/blocks/SulphurCrystalBlock.java:57)
- [ ] `SulphurCrystalBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:62)
- [ ] `SulphurCrystalBlock.ageCondition(3)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:67)
- [ ] `SulphurCrystalBlock.buildLoot(2)`: it uses Minecraft's `Builder` (org/betterx/betterend/blocks/SulphurCrystalBlock.java:73)
- [ ] `SulphurCrystalBlock.canPlaceLiquid(5)`: its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:128)
- [ ] `SulphurCrystalBlock.placeLiquid(4)`: its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:132)
- [ ] `SulphurCrystalBlock.getStateForPlacement(1)`: its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:136)
- [ ] `SulphurCrystalBlock.getFluidState(1)`: its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:148)
- [ ] `SulphurCrystalBlock.getShape(4)`: its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:153)
- [ ] `SulphurCrystalBlock.canSurvive(3)`: its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)) (org/betterx/betterend/blocks/SulphurCrystalBlock.java:158)

### `TenaneaFlowersBlock`: it extends `BaseVineBlock` (org.betterx.bclib.blocks)

- [ ] `TenaneaFlowersBlock.COLORS`: it uses Minecraft's `Vec3i` (org/betterx/betterend/blocks/TenaneaFlowersBlock.java:17)
- [ ] `TenaneaFlowersBlock.TenaneaFlowersBlock(1)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TenaneaFlowersBlock.java:19)
- [ ] `TenaneaFlowersBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TenaneaFlowersBlock.java:23)
- [ ] `TenaneaFlowersBlock.animateTick(4)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TenaneaFlowersBlock.java:27)

### `TenaneaSaplingBlock`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `TenaneaSaplingBlock.TenaneaSaplingBlock(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betterend/blocks/TenaneaSaplingBlock.java:8)

### `TwistedUmbrellaMossBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `TwistedUmbrellaMossBlock.TwistedUmbrellaMossBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TwistedUmbrellaMossBlock.java:20)
- [ ] `TwistedUmbrellaMossBlock.hasEmissiveLighting(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TwistedUmbrellaMossBlock.java:24)
- [ ] `TwistedUmbrellaMossBlock.getAmbientOcclusionLightLevel(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TwistedUmbrellaMossBlock.java:29)
- [ ] `TwistedUmbrellaMossBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TwistedUmbrellaMossBlock.java:34)
- [ ] `TwistedUmbrellaMossBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TwistedUmbrellaMossBlock.java:38)
- [ ] `TwistedUmbrellaMossBlock.performBonemeal(5)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/TwistedUmbrellaMossBlock.java:42)

### `TwistedUmbrellaMossTallBlock`: it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)

- [ ] `TwistedUmbrellaMossTallBlock.TwistedUmbrellaMossTallBlock(1)`: its class isn't translated (it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/TwistedUmbrellaMossTallBlock.java:15)
- [ ] `TwistedUmbrellaMossTallBlock.performBonemeal(5)`: its class isn't translated (it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/TwistedUmbrellaMossTallBlock.java:19)

### `UmbrellaMossBlock`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `UmbrellaMossBlock.UmbrellaMossBlock(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/UmbrellaMossBlock.java:20)
- [ ] `UmbrellaMossBlock.hasEmissiveLighting(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/UmbrellaMossBlock.java:24)
- [ ] `UmbrellaMossBlock.getAmbientOcclusionLightLevel(2)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/UmbrellaMossBlock.java:29)
- [ ] `UmbrellaMossBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/UmbrellaMossBlock.java:34)
- [ ] `UmbrellaMossBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/UmbrellaMossBlock.java:38)
- [ ] `UmbrellaMossBlock.performBonemeal(5)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betterend/blocks/UmbrellaMossBlock.java:42)

### `UmbrellaMossTallBlock`: it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)

- [ ] `UmbrellaMossTallBlock.UmbrellaMossTallBlock(1)`: its class isn't translated (it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/UmbrellaMossTallBlock.java:15)
- [ ] `UmbrellaMossTallBlock.performBonemeal(5)`: its class isn't translated (it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)) (org/betterx/betterend/blocks/UmbrellaMossTallBlock.java:19)

### `UmbrellaTreeClusterBlock`: it extends Minecraft's `Block`

- [ ] `UmbrellaTreeClusterBlock.NATURAL`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/UmbrellaTreeClusterBlock.java:25)
- [ ] `UmbrellaTreeClusterBlock.UmbrellaTreeClusterBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/UmbrellaTreeClusterBlock.java:27)
- [ ] `UmbrellaTreeClusterBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/UmbrellaTreeClusterBlock.java:32)
- [ ] `UmbrellaTreeClusterBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/UmbrellaTreeClusterBlock.java:36)
- [ ] `UmbrellaTreeClusterBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/UmbrellaTreeClusterBlock.java:40)

### `UmbrellaTreeClusterEmptyBlock`: it extends Minecraft's `Block`

- [ ] `UmbrellaTreeClusterEmptyBlock.NATURAL`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/blocks/UmbrellaTreeClusterEmptyBlock.java:17)
- [ ] `UmbrellaTreeClusterEmptyBlock.UmbrellaTreeClusterEmptyBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/UmbrellaTreeClusterEmptyBlock.java:19)
- [ ] `UmbrellaTreeClusterEmptyBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/UmbrellaTreeClusterEmptyBlock.java:24)
- [ ] `UmbrellaTreeClusterEmptyBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/UmbrellaTreeClusterEmptyBlock.java:28)

### `UmbrellaTreeMembraneBlock`: it extends Minecraft's `SlimeBlock`

- [ ] `UmbrellaTreeMembraneBlock.COLOR`: it uses Minecraft's `IntegerProperty` (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java:32)
- [ ] `UmbrellaTreeMembraneBlock.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java:33)
- [ ] `UmbrellaTreeMembraneBlock.UmbrellaTreeMembraneBlock(1)`: its class isn't translated (it extends Minecraft's `SlimeBlock`) (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java:35)
- [ ] `UmbrellaTreeMembraneBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `SlimeBlock`) (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java:39)
- [ ] `UmbrellaTreeMembraneBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `SlimeBlock`) (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java:46)
- [ ] `UmbrellaTreeMembraneBlock.provideBlockModel(2)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java:50)
- [ ] `UmbrellaTreeMembraneBlock.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `SlimeBlock`) (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java:75)
- [ ] `UmbrellaTreeMembraneBlock.skipRendering(3)`: its class isn't translated (it extends Minecraft's `SlimeBlock`) (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java:79)

### `UmbrellaTreeSaplingBlock`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `UmbrellaTreeSaplingBlock.UmbrellaTreeSaplingBlock(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betterend/blocks/UmbrellaTreeSaplingBlock.java:8)

### `VentBubbleColumnBlock`: it extends Minecraft's `Block`

- [ ] `VentBubbleColumnBlock.VentBubbleColumnBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:43)
- [ ] `VentBubbleColumnBlock.pickupBlock(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:47)
- [ ] `VentBubbleColumnBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:53)
- [ ] `VentBubbleColumnBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:57)
- [ ] `VentBubbleColumnBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:62)
- [ ] `VentBubbleColumnBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:66)
- [ ] `VentBubbleColumnBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:89)
- [ ] `VentBubbleColumnBlock.entityInside(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:112)
- [ ] `VentBubbleColumnBlock.canPlaceLiquid(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:150)
- [ ] `VentBubbleColumnBlock.placeLiquid(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:154)
- [ ] `VentBubbleColumnBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:158)
- [ ] `VentBubbleColumnBlock.getPickupSound(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betterend/blocks/VentBubbleColumnBlock.java:162)

### `BetterEndClient`: it implements Fabric's `ClientModInitializer`

- [ ] `BetterEndClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook (org/betterx/betterend/client/BetterEndClient.java:33)
- [ ] `BetterEndClient.registerTooltips(0)`: unknown static ItemTooltipCallback.EVENT (org/betterx/betterend/client/BetterEndClient.java:54)

### `EternalHint`

- [x] `EternalHint.DURATION` (org/betterx/betterend/client/effects/EternalHint.java:31)
- [x] `EternalHint.FADE_IN` (org/betterx/betterend/client/effects/EternalHint.java:32)
- [x] `EternalHint.FADE_OUT` (org/betterx/betterend/client/effects/EternalHint.java:33)
- [x] `EternalHint.MAX_DISTANCE_SQ` (org/betterx/betterend/client/effects/EternalHint.java:34)
- [x] `EternalHint.CRYSTAL_HEIGHT` (org/betterx/betterend/client/effects/EternalHint.java:35)
- [x] `EternalHint.STREAK_SPEED` (org/betterx/betterend/client/effects/EternalHint.java:36)
- [x] `EternalHint.STREAKS_PER_TICK` (org/betterx/betterend/client/effects/EternalHint.java:37)
- [x] `EternalHint.DRIP_CHANCE` (org/betterx/betterend/client/effects/EternalHint.java:38)
- [ ] `EternalHint.center`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/EternalHint.java:39)
- [ ] `EternalHint.source`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/EternalHint.java:40)
- [ ] `EternalHint.axis`: it uses Minecraft's `Axis` (org/betterx/betterend/client/effects/EternalHint.java:41)
- [x] `EternalHint.startTime` (org/betterx/betterend/client/effects/EternalHint.java:42)
- [x] `EternalHint.EternalHint(0)` (org/betterx/betterend/client/effects/EternalHint.java:44)
- [ ] `EternalHint.trigger(2)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/EternalHint.java:47)
- [ ] `EternalHint.clear(0)`: it uses EternalHint.center, which isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/client/effects/EternalHint.java:57)
- [ ] `EternalHint.anchorPedestal(1)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/EternalHint.java:62)
- [ ] `EternalHint.center(0)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/EternalHint.java:67)
- [ ] `EternalHint.age(1)`: it uses EternalHint.center, which isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/client/effects/EternalHint.java:72)
- [ ] `EternalHint.intensity(1)`: it calls EternalHint.age, which isn't translated (org/betterx/betterend/client/effects/EternalHint.java:98)
- [ ] `EternalHint.wobble(1)`: it calls ScreenWobble.at, which isn't translated (org/betterx/betterend/client/effects/EternalHint.java:109)
- [ ] `EternalHint.pedestalPositions(0)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/EternalHint.java:113)
- [ ] `EternalHint.missingFrame(1)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/EternalHint.java:129)
- [ ] `EternalHint.tickParticles(3)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/EternalHint.java:152)
- [ ] `EternalHint.streakToCentre(4)`: it uses `ClientLevelAccess` (org.betterx.bclib.interfaces) (org/betterx/betterend/client/effects/EternalHint.java:164)
- [ ] `EternalHint.dripFromGaps(4)`: it uses `ClientLevelAccess` (org.betterx.bclib.interfaces) (org/betterx/betterend/client/effects/EternalHint.java:179)

### `InfusionHint`

- [x] `InfusionHint.DURATION` (org/betterx/betterend/client/effects/InfusionHint.java:22)
- [x] `InfusionHint.FADE_IN` (org/betterx/betterend/client/effects/InfusionHint.java:23)
- [x] `InfusionHint.FADE_OUT` (org/betterx/betterend/client/effects/InfusionHint.java:24)
- [x] `InfusionHint.MAX_DISTANCE_SQ` (org/betterx/betterend/client/effects/InfusionHint.java:25)
- [x] `InfusionHint.MIST_SPAWN_HEIGHT` (org/betterx/betterend/client/effects/InfusionHint.java:26)
- [x] `InfusionHint.MIST_SPAWN_JITTER` (org/betterx/betterend/client/effects/InfusionHint.java:27)
- [x] `InfusionHint.MIST_SPREAD` (org/betterx/betterend/client/effects/InfusionHint.java:28)
- [x] `InfusionHint.BURST_COUNT` (org/betterx/betterend/client/effects/InfusionHint.java:29)
- [x] `InfusionHint.BURST_RADIUS` (org/betterx/betterend/client/effects/InfusionHint.java:30)
- [x] `InfusionHint.BURST_HEIGHT` (org/betterx/betterend/client/effects/InfusionHint.java:31)
- [x] `InfusionHint.SUBTLE_ALPHA` (org/betterx/betterend/client/effects/InfusionHint.java:32)
- [x] `InfusionHint.FLASH_PER_SPOKE` (org/betterx/betterend/client/effects/InfusionHint.java:33)
- [x] `InfusionHint.FLASH_PULSES` (org/betterx/betterend/client/effects/InfusionHint.java:34)
- [x] `InfusionHint.FLASH_PULSE_INTERVAL` (org/betterx/betterend/client/effects/InfusionHint.java:35)
- [x] `InfusionHint.FLASH_HEIGHT` (org/betterx/betterend/client/effects/InfusionHint.java:36)
- [ ] `InfusionHint.pedestalPos`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/InfusionHint.java:37)
- [x] `InfusionHint.startTime` (org/betterx/betterend/client/effects/InfusionHint.java:38)
- [x] `InfusionHint.subtle` (org/betterx/betterend/client/effects/InfusionHint.java:39)
- [ ] `InfusionHint.flashPos`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/InfusionHint.java:40)
- [x] `InfusionHint.flashStart` (org/betterx/betterend/client/effects/InfusionHint.java:41)
- [x] `InfusionHint.InfusionHint(0)` (org/betterx/betterend/client/effects/InfusionHint.java:43)
- [ ] `InfusionHint.trigger(2)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/InfusionHint.java:46)
- [ ] `InfusionHint.triggerSubtle(2)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/InfusionHint.java:50)
- [ ] `InfusionHint.start(3)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/InfusionHint.java:54)
- [ ] `InfusionHint.clear(0)`: it uses InfusionHint.pedestalPos, which isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/client/effects/InfusionHint.java:60)
- [ ] `InfusionHint.dismiss(1)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/InfusionHint.java:64)
- [ ] `InfusionHint.activePedestal(0)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/InfusionHint.java:74)
- [ ] `InfusionHint.isActiveAt(1)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/client/effects/InfusionHint.java:78)
- [ ] `InfusionHint.age(1)`: it uses InfusionHint.pedestalPos, which isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/client/effects/InfusionHint.java:82)
- [ ] `InfusionHint.spawnMist(2)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/InfusionHint.java:108)
- [ ] `InfusionHint.outline(1)`: it calls InfusionHint.age, which isn't translated (org/betterx/betterend/client/effects/InfusionHint.java:133)
- [ ] `InfusionHint.mistFor(1)`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/client/effects/InfusionHint.java:155)
- [ ] `InfusionHint.burstAtSocket(3)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/InfusionHint.java:159)
- [ ] `InfusionHint.beginFlash(2)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/InfusionHint.java:171)
- [ ] `InfusionHint.tickFlash(2)`: it uses Minecraft's `Level` (org/betterx/betterend/client/effects/InfusionHint.java:179)
- [ ] `InfusionHint.wobble(1)`: it calls ScreenWobble.at, which isn't translated (org/betterx/betterend/client/effects/InfusionHint.java:209)

### `ScreenWobble`

- [x] `ScreenWobble.DECAY` (org/betterx/betterend/client/effects/ScreenWobble.java:11)
- [x] `ScreenWobble.MAX_ROLL` (org/betterx/betterend/client/effects/ScreenWobble.java:12)
- [x] `ScreenWobble.MAX_PITCH` (org/betterx/betterend/client/effects/ScreenWobble.java:13)
- [x] `ScreenWobble.ScreenWobble(0)` (org/betterx/betterend/client/effects/ScreenWobble.java:15)
- [ ] `ScreenWobble.at(1)`: it calls Minecraft's Options.screenEffectScale (no Horizon helper for it yet) (org/betterx/betterend/client/effects/ScreenWobble.java:18)

### `EndStoneSmelterMenu`: it extends Minecraft's `RecipeBookMenu`

- [x] `EndStoneSmelterMenu.INGREDIENT_SLOT_A` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:39)
- [x] `EndStoneSmelterMenu.INGREDIENT_SLOT_B` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:40)
- [x] `EndStoneSmelterMenu.FUEL_SLOT` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:41)
- [x] `EndStoneSmelterMenu.RESULT_SLOT` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:42)
- [x] `EndStoneSmelterMenu.SLOT_COUNT` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:43)
- [x] `EndStoneSmelterMenu.INV_SLOT_START` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:44)
- [x] `EndStoneSmelterMenu.INV_SLOT_END` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:45)
- [x] `EndStoneSmelterMenu.USE_ROW_SLOT_START` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:46)
- [x] `EndStoneSmelterMenu.USE_ROW_SLOT_END` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:47)
- [ ] `EndStoneSmelterMenu.EndStoneSmelterMenu(2)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:52)
- [ ] `EndStoneSmelterMenu.EndStoneSmelterMenu(4)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:56)
- [ ] `EndStoneSmelterMenu.getType(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:78)
- [ ] `EndStoneSmelterMenu.fillCraftSlotsStackedContents(1)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:83)
- [ ] `EndStoneSmelterMenu.clearCraftingContent(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:89)
- [ ] `EndStoneSmelterMenu.recipeMatches(1)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:95)
- [ ] `EndStoneSmelterMenu.handlePlacement(5)`: its body couldn't be read: an anonymous class (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:99)
- [ ] `EndStoneSmelterMenu.getRecipeBookType(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:119)
- [ ] `EndStoneSmelterMenu.stillValid(1)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:124)
- [ ] `EndStoneSmelterMenu.isSmeltable(1)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:128)
- [ ] `EndStoneSmelterMenu.isFuel(1)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:147)
- [ ] `EndStoneSmelterMenu.quickMoveStack(2)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:151)
- [ ] `EndStoneSmelterMenu.getSmeltProgress(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:200)
- [ ] `EndStoneSmelterMenu.getFuelProgress(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:207)
- [ ] `EndStoneSmelterMenu.isBurning(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java:217)

### `EndStoneSmelterScreen`: it extends Minecraft's `AbstractContainerScreen`

- [ ] `EndStoneSmelterScreen.BACKGROUND_TEXTURE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/client/gui/EndStoneSmelterScreen.java:15)
- [ ] `EndStoneSmelterScreen.EndStoneSmelterScreen(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (org/betterx/betterend/client/gui/EndStoneSmelterScreen.java:17)
- [ ] `EndStoneSmelterScreen.init(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (org/betterx/betterend/client/gui/EndStoneSmelterScreen.java:21)
- [ ] `EndStoneSmelterScreen.extractContents(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (org/betterx/betterend/client/gui/EndStoneSmelterScreen.java:26)

### `InfusionRecipeScreen`: it extends Minecraft's `Screen`

- [x] `InfusionRecipeScreen.PAGE_COLOR` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:38)
- [x] `InfusionRecipeScreen.PAGE_SHADE_COLOR` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:39)
- [x] `InfusionRecipeScreen.FRAME_COLOR` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:40)
- [x] `InfusionRecipeScreen.FRAME_DARK_COLOR` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:41)
- [x] `InfusionRecipeScreen.TEXT_COLOR` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:42)
- [x] `InfusionRecipeScreen.TEXT_MUTED_COLOR` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:43)
- [x] `InfusionRecipeScreen.SLOT_SPRITE` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:44)
- [x] `InfusionRecipeScreen.ARROW_SPRITE` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:45)
- [ ] `InfusionRecipeScreen.COMPASS`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:46)
- [x] `InfusionRecipeScreen.COMPASS_SIZE` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:47)
- [x] `InfusionRecipeScreen.SLOT_SIZE` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:48)
- [x] `InfusionRecipeScreen.ARROW_WIDTH` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:49)
- [x] `InfusionRecipeScreen.ARROW_HEIGHT` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:50)
- [x] `InfusionRecipeScreen.RING_RADIUS` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:51)
- [x] `InfusionRecipeScreen.PANEL_WIDTH` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:52)
- [x] `InfusionRecipeScreen.PANEL_HEIGHT` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:53)
- [x] `InfusionRecipeScreen.MARGIN` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:54)
- [x] `InfusionRecipeScreen.HEADER_HEIGHT` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:55)
- [x] `InfusionRecipeScreen.LIST_WIDTH` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:56)
- [x] `InfusionRecipeScreen.ROW_HEIGHT` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:57)
- [x] `InfusionRecipeScreen.INGREDIENT_CYCLE_TICKS` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:58)
- [x] `InfusionRecipeScreen.ORIENTATIONS` in game_side.rs (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:59)
- [ ] `InfusionRecipeScreen.InfusionRecipeScreen(0)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:75)
- [ ] `InfusionRecipeScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:79)
- [ ] `InfusionRecipeScreen.loadRecipes(0)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:92)
- [ ] `InfusionRecipeScreen.outputOf(1)`: it uses Minecraft's `RecipeHolder` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:102)
- [ ] `InfusionRecipeScreen.displayName(1)`: ItemStack.has(DataComponents.CUSTOM_NAME) (not mapped yet) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:106)
- [ ] `InfusionRecipeScreen.enchantmentName(2)`: it uses Minecraft's `Holder` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:126)
- [ ] `InfusionRecipeScreen.displayStack(1)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:135)
- [ ] `InfusionRecipeScreen.ringCenterX(0)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:150)
- [ ] `InfusionRecipeScreen.ringCenterY(0)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:154)
- [x] `InfusionRecipeScreen.catalystX(2)` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:158)
- [x] `InfusionRecipeScreen.catalystY(2)` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:162)
- [ ] `InfusionRecipeScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:166)
- [ ] `InfusionRecipeScreen.extractPages(1)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:179)
- [ ] `InfusionRecipeScreen.extractRecipe(4)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:189)
- [ ] `InfusionRecipeScreen.extractSlot(7)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:208)
- [ ] `InfusionRecipeScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:226)
- [ ] `InfusionRecipeScreen.isPauseScreen(0)`: its class isn't translated (it extends Minecraft's `Screen`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:230)

### `InfusionRecipeScreen.RecipeList`: it extends Minecraft's `AbstractScrollArea`

- [ ] `InfusionRecipeScreen.RecipeList.RecipeList(4)`: its class isn't translated (it extends Minecraft's `AbstractScrollArea`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:236)
- [ ] `InfusionRecipeScreen.RecipeList.contentHeight(0)`: its class isn't translated (it extends Minecraft's `AbstractScrollArea`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:240)
- [ ] `InfusionRecipeScreen.RecipeList.rowWidth(0)`: its class isn't translated (it extends Minecraft's `AbstractScrollArea`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:244)
- [ ] `InfusionRecipeScreen.RecipeList.indexAt(2)`: its class isn't translated (it extends Minecraft's `AbstractScrollArea`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:248)
- [ ] `InfusionRecipeScreen.RecipeList.onClick(2)`: its class isn't translated (it extends Minecraft's `AbstractScrollArea`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:257)
- [ ] `InfusionRecipeScreen.RecipeList.extractWidgetRenderState(4)`: its class isn't translated (it extends Minecraft's `AbstractScrollArea`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:266)
- [ ] `InfusionRecipeScreen.RecipeList.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `AbstractScrollArea`) (org/betterx/betterend/client/gui/InfusionRecipeScreen.java:293)

### `SmelterFuelSlot`: it extends Minecraft's `Slot`

- [ ] `SmelterFuelSlot.SmelterFuelSlot(5)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterFuelSlot.java:12)
- [ ] `SmelterFuelSlot.mayPlace(1)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterFuelSlot.java:17)
- [ ] `SmelterFuelSlot.getMaxStackSize(1)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterFuelSlot.java:21)

### `SmelterOutputSlot`: it extends Minecraft's `Slot`

- [ ] `SmelterOutputSlot.SmelterOutputSlot(5)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterOutputSlot.java:14)
- [ ] `SmelterOutputSlot.mayPlace(1)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterOutputSlot.java:19)
- [ ] `SmelterOutputSlot.remove(1)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterOutputSlot.java:23)
- [ ] `SmelterOutputSlot.onTake(2)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterOutputSlot.java:31)
- [ ] `SmelterOutputSlot.onQuickCraft(2)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterOutputSlot.java:36)
- [ ] `SmelterOutputSlot.checkTakeAchievements(1)`: its class isn't translated (it extends Minecraft's `Slot`) (org/betterx/betterend/client/gui/slot/SmelterOutputSlot.java:41)

### `EndModels`

- [ ] `EndModels.ROOTS`: it uses Minecraft's `TextureSlot` (org/betterx/betterend/client/models/EndModels.java:9)
- [ ] `EndModels.BASE`: it uses Minecraft's `TextureSlot` (org/betterx/betterend/client/models/EndModels.java:10)
- [ ] `EndModels.PILLAR`: it uses Minecraft's `TextureSlot` (org/betterx/betterend/client/models/EndModels.java:11)
- [ ] `EndModels.FLOOR`: it uses Minecraft's `TextureSlot` (org/betterx/betterend/client/models/EndModels.java:12)
- [ ] `EndModels.CEIL`: it uses Minecraft's `TextureSlot` (org/betterx/betterend/client/models/EndModels.java:13)
- [ ] `EndModels.WALL`: it uses Minecraft's `TextureSlot` (org/betterx/betterend/client/models/EndModels.java:14)
- [ ] `EndModels.TWISTED_VINE`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:15)
- [ ] `EndModels.CROSS_NO_DISTORTION`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:18)
- [ ] `EndModels.CROSS_NO_DISTORTION_INVERTED`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:21)
- [ ] `EndModels.PATH`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:24)
- [ ] `EndModels.CUBE_NO_SHADE`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:27)
- [ ] `EndModels.FLOWER_POT`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:30)
- [ ] `EndModels.PEDESTAL_PILLAR`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:33)
- [ ] `EndModels.PEDESTAL_TOP`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:36)
- [ ] `EndModels.PEDESTAL_DEFAULT`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:39)
- [ ] `EndModels.PEDESTAL_COLUMN_TOP`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:42)
- [ ] `EndModels.PEDESTAL_COLUMN`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:45)
- [ ] `EndModels.PEDESTAL_BOTTOM`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:48)
- [ ] `EndModels.INFUSION_PEDESTAL_TOP`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:51)
- [ ] `EndModels.INFUSION_PEDESTAL_DEFAULT`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:54)
- [ ] `EndModels.PETAL_COLORED`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:59)
- [ ] `EndModels.LIT_STAIRS_INNER`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:62)
- [ ] `EndModels.LIT_STAIRS`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:65)
- [ ] `EndModels.LIT_STAIRS_OUTER`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:68)
- [ ] `EndModels.CHANDELIER_FLOOR`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:71)
- [ ] `EndModels.CHANDELIER_WALL`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:74)
- [ ] `EndModels.CHANDELIER_CEIL`: it uses Minecraft's `ModelTemplate` (org/betterx/betterend/client/models/EndModels.java:77)

### `EndModelTraits`

- [ ] `EndModelTraits.IDENTIFIER_SUPPLIER`: it uses Minecraft's `Codec` (org/betterx/betterend/client/models/EndModelTraits.java:54)
- [ ] `EndModelTraits.TWISTED_VINE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:55)
- [ ] `EndModelTraits.AMBER_MOSS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:57)
- [ ] `EndModelTraits.CAVE_MOSS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:59)
- [ ] `EndModelTraits.UNSHADED_CUBE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:61)
- [ ] `EndModelTraits.HYDRALUX_PETAL_COLORED`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:63)
- [ ] `EndModelTraits.ETERNAL_PEDESTAL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:65)
- [ ] `EndModelTraits.INFUSION_PEDESTAL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:67)
- [ ] `EndModelTraits.BLUE_VINE_LANTERN`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:69)
- [ ] `EndModelTraits.UMBRELLA_TREE_MEMBRANE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:71)
- [ ] `EndModelTraits.JELLYSHROOM_CAP`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:73)
- [ ] `EndModelTraits.AMBER_MOSS_PATH`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:75)
- [ ] `EndModelTraits.ROTATED_PILLAR`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:83)
- [ ] `EndModelTraits.LIT_STAIRS`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:91)
- [ ] `EndModelTraits.SLAB_FROM`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betterend/client/models/EndModelTraits.java:99)
- [x] `EndModelTraits.ensureLoaded(0)` (org/betterx/betterend/client/models/EndModelTraits.java:108)
- [ ] `EndModelTraits.twistedVine(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:111)
- [ ] `EndModelTraits.amberMoss(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:115)
- [ ] `EndModelTraits.caveMoss(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:119)
- [ ] `EndModelTraits.amberMossPath(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:123)
- [ ] `EndModelTraits.rotatedPillar(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:127)
- [ ] `EndModelTraits.unshadedCube(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:131)
- [ ] `EndModelTraits.litStairs(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:135)
- [ ] `EndModelTraits.slabFrom(2)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:139)
- [ ] `EndModelTraits.flavoliteRunedVariants(3)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:143)
- [ ] `EndModelTraits.flavoliteRuned(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:158)
- [ ] `EndModelTraits.bulbMossMeshMembers(2)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:214)
- [ ] `EndModelTraits.glowingPillarLeaves(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:225)
- [ ] `EndModelTraits.ruscus(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:242)
- [ ] `EndModelTraits.bulbMossTwoTexMembers(3)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:256)
- [ ] `EndModelTraits.twistedMoss(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:267)
- [ ] `EndModelTraits.bulbMossTemplateMembers(1)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:282)
- [ ] `EndModelTraits.bulbMoss(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:292)
- [ ] `EndModelTraits.jungleFernMembers(1)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:305)
- [ ] `EndModelTraits.jungleFern(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:329)
- [ ] `EndModelTraits.creepingMoss(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:342)
- [ ] `EndModelTraits.upLeafChild(1)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:355)
- [ ] `EndModelTraits.upLeaf(2)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:359)
- [ ] `EndModelTraits.lucerniaHorizontal(1)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:375)
- [ ] `EndModelTraits.lucerniaUp(1)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:388)
- [ ] `EndModelTraits.lucerniaOuterLeaves(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:394)
- [ ] `EndModelTraits.umbrellaMossSmallLayers(0)`: it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels) (org/betterx/betterend/client/models/EndModelTraits.java:409)
- [ ] `EndModelTraits.umbrellaMoss(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:423)
- [ ] `EndModelTraits.twistedUmbrellaMoss(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:427)
- [ ] `EndModelTraits.hydraluxPetalColored(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:453)
- [ ] `EndModelTraits.eternalPedestal(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:457)
- [ ] `EndModelTraits.infusionPedestal(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:461)
- [ ] `EndModelTraits.blueVineLantern(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:465)
- [ ] `EndModelTraits.umbrellaTreeMembrane(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:469)
- [ ] `EndModelTraits.jellyshroomCap(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:473)

### `EndModelTraits.AmberMossPath`: it uses Minecraft's `Block`

- [ ] `EndModelTraits.AmberMossPath.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/client/models/EndModelTraits.java:478)
- [ ] `EndModelTraits.AmberMossPath.AmberMossPath(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:477)
- [ ] `EndModelTraits.AmberMossPath.base(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:477)

### `EndModelTraits.AmberMossPathParams`: it uses Minecraft's `Block`

- [ ] `EndModelTraits.AmberMossPathParams.AmberMossPathParams(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:483)
- [ ] `EndModelTraits.AmberMossPathParams.base(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:483)

### `EndModelTraits.Impl`

- [ ] `EndModelTraits.Impl.amberMossPathFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:488)
- [ ] `EndModelTraits.Impl.rotatedPillarFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:492)
- [ ] `EndModelTraits.Impl.litStairsFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:496)
- [ ] `EndModelTraits.Impl.slabFromFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:500)
- [ ] `EndModelTraits.Impl.hydraluxPetalColored(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:504)
- [ ] `EndModelTraits.Impl.eternalPedestal(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:508)
- [ ] `EndModelTraits.Impl.infusionPedestal(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:512)
- [ ] `EndModelTraits.Impl.blueVineLantern(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:516)
- [ ] `EndModelTraits.Impl.umbrellaTreeMembrane(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:520)
- [ ] `EndModelTraits.Impl.jellyshroomCap(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:524)
- [ ] `EndModelTraits.Impl.buildRotated(3)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:528)
- [ ] `EndModelTraits.Impl.caveMoss(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:540)
- [ ] `EndModelTraits.Impl.amberMoss(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:575)
- [ ] `EndModelTraits.Impl.amberMossPath(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:600)
- [ ] `EndModelTraits.Impl.rotatedPillar(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:613)
- [ ] `EndModelTraits.Impl.unshadedCube(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:622)
- [ ] `EndModelTraits.Impl.litStairs(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:626)
- [ ] `EndModelTraits.Impl.slabFrom(2)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:641)
- [ ] `EndModelTraits.Impl.twistedVine(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/client/models/EndModelTraits.java:655)

### `EndModelTraits.LitStairs`: a function kept in a variable or field (only parameters can be)

- [ ] `EndModelTraits.LitStairs.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/client/models/EndModelTraits.java:683)
- [ ] `EndModelTraits.LitStairs.LitStairs(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/betterx/betterend/client/models/EndModelTraits.java:682)
- [ ] `EndModelTraits.LitStairs.texture(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/betterx/betterend/client/models/EndModelTraits.java:682)

### `EndModelTraits.LitStairsParams`: a function kept in a variable or field (only parameters can be)

- [ ] `EndModelTraits.LitStairsParams.LitStairsParams(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/betterx/betterend/client/models/EndModelTraits.java:689)
- [ ] `EndModelTraits.LitStairsParams.texture(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/betterx/betterend/client/models/EndModelTraits.java:689)

### `EndModelTraits.RotatedPillar`: a function kept in a variable or field (only parameters can be)

- [ ] `EndModelTraits.RotatedPillar.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/client/models/EndModelTraits.java:693)
- [ ] `EndModelTraits.RotatedPillar.RotatedPillar(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/betterx/betterend/client/models/EndModelTraits.java:692)
- [ ] `EndModelTraits.RotatedPillar.texture(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/betterx/betterend/client/models/EndModelTraits.java:692)

### `EndModelTraits.RotatedPillarParams`: a function kept in a variable or field (only parameters can be)

- [ ] `EndModelTraits.RotatedPillarParams.RotatedPillarParams(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/betterx/betterend/client/models/EndModelTraits.java:699)
- [ ] `EndModelTraits.RotatedPillarParams.texture(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/betterx/betterend/client/models/EndModelTraits.java:699)

### `EndModelTraits.SlabFrom`: it uses Minecraft's `Block`

- [ ] `EndModelTraits.SlabFrom.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/client/models/EndModelTraits.java:703)
- [ ] `EndModelTraits.SlabFrom.SlabFrom(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:702)
- [ ] `EndModelTraits.SlabFrom.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:702)
- [ ] `EndModelTraits.SlabFrom.texture(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:702)

### `EndModelTraits.SlabFromParams`: it uses Minecraft's `Block`

- [ ] `EndModelTraits.SlabFromParams.SlabFromParams(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:712)
- [ ] `EndModelTraits.SlabFromParams.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:712)
- [ ] `EndModelTraits.SlabFromParams.texture(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/client/models/EndModelTraits.java:712)

### `BeamRenderer`

- [x] `BeamRenderer.BEAM_TEXTURE` (org/betterx/betterend/client/render/BeamRenderer.java:15)
- [ ] `BeamRenderer.renderLightBeam(10)`: it uses Minecraft's `SubmitNodeCollector` (org/betterx/betterend/client/render/BeamRenderer.java:17)
- [ ] `BeamRenderer.renderBeam(20)`: it uses Minecraft's `Pose` (org/betterx/betterend/client/render/BeamRenderer.java:50)
- [ ] `BeamRenderer.renderBeam(17)`: it uses Minecraft's `Pose` (org/betterx/betterend/client/render/BeamRenderer.java:79)
- [ ] `BeamRenderer.addVertex(12)`: it uses Minecraft's `Pose` (org/betterx/betterend/client/render/BeamRenderer.java:104)

### `BetterEndSkyRenderer`: it implements Fabric's `StartMain`

- [ ] `BetterEndSkyRenderer.NEBULA_1`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:39)
- [ ] `BetterEndSkyRenderer.NEBULA_2`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:40)
- [ ] `BetterEndSkyRenderer.HORIZON`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:41)
- [ ] `BetterEndSkyRenderer.STARS`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:42)
- [ ] `BetterEndSkyRenderer.FOG`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:43)
- [ ] `BetterEndSkyRenderer.activeSolidRenderPass`: it uses Minecraft's `RenderPass` (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:57)
- [x] `BetterEndSkyRenderer.HORIZON_RED` (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:58)
- [x] `BetterEndSkyRenderer.HORIZON_GREEN` (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:59)
- [x] `BetterEndSkyRenderer.HORIZON_BLUE` (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:60)
- [ ] `BetterEndSkyRenderer.setActiveSolidRenderPass(1)`: it uses Minecraft's `RenderPass` (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:62)
- [ ] `BetterEndSkyRenderer.initialise(0)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:66)
- [ ] `BetterEndSkyRenderer.register(0)`: it uses Fabric's `LevelRenderEvents`.START_MAIN (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:82)
- [ ] `BetterEndSkyRenderer.startMain(1)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:91)
- [ ] `BetterEndSkyRenderer.renderTexturedMesh(7)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:161)
- [ ] `BetterEndSkyRenderer.renderStarMesh(6)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:176)
- [ ] `BetterEndSkyRenderer.initStars(0)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:189)
- [ ] `BetterEndSkyRenderer.buildStarMesh(4)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:200)
- [ ] `BetterEndSkyRenderer.buildTexturedMesh(5)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:227)
- [ ] `BetterEndSkyRenderer.uploadMesh(2)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:254)
- [ ] `BetterEndSkyRenderer.makeStars(5)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:281)
- [ ] `BetterEndSkyRenderer.makeUVStars(5)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:325)
- [ ] `BetterEndSkyRenderer.makeFarFog(5)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:373)
- [ ] `BetterEndSkyRenderer.makeCylinder(4)`: its class isn't translated (it implements Fabric's `StartMain`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:422)

### `BetterEndSkyRenderer.BufferFunction`: it's an interface

- [ ] `BetterEndSkyRenderer.BufferFunction.make(5)`: it has no body (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:445)

### `BetterEndSkyRenderer.SkyMesh`: it uses Minecraft's `GpuBuffer`

- [ ] `BetterEndSkyRenderer.SkyMesh.SkyMesh(2)`: its class isn't translated (it uses Minecraft's `GpuBuffer`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:448)
- [ ] `BetterEndSkyRenderer.SkyMesh.buffer(0)`: its class isn't translated (it uses Minecraft's `GpuBuffer`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:448)
- [ ] `BetterEndSkyRenderer.SkyMesh.indexCount(0)`: its class isn't translated (it uses Minecraft's `GpuBuffer`) (org/betterx/betterend/client/render/BetterEndSkyRenderer.java:448)

### `EndCrystalRenderer`

- [x] `EndCrystalRenderer.CRYSTAL_TEXTURE` (org/betterx/betterend/client/render/EndCrystalRenderer.java:19)
- [ ] `EndCrystalRenderer.END_CRYSTAL`: it uses Minecraft's `RenderType` (org/betterx/betterend/client/render/EndCrystalRenderer.java:20)
- [ ] `EndCrystalRenderer.CORE`: it uses Minecraft's `ModelPart` (org/betterx/betterend/client/render/EndCrystalRenderer.java:21)
- [ ] `EndCrystalRenderer.FRAME`: it uses Minecraft's `ModelPart` (org/betterx/betterend/client/render/EndCrystalRenderer.java:22)
- [x] `EndCrystalRenderer.AGE_CYCLE` (org/betterx/betterend/client/render/EndCrystalRenderer.java:23)
- [x] `EndCrystalRenderer.SINE_45_DEGREES` (org/betterx/betterend/client/render/EndCrystalRenderer.java:24)
- [ ] `EndCrystalRenderer.ROTATOR`: JOML's Quaternionf.setAngleAxis (org/betterx/betterend/client/render/EndCrystalRenderer.java:25)
- [ ] `EndCrystalRenderer.render(6)`: it uses Minecraft's `SubmitNodeCollector` (org/betterx/betterend/client/render/EndCrystalRenderer.java:27)
- [ ] `EndCrystalRenderer.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/client/render/EndCrystalRenderer.java:50)

### `EndFlowerPotModels`

- [x] `EndFlowerPotModels.MODELS_PREFIX` (org/betterx/betterend/client/render/EndFlowerPotModels.java:26)
- [x] `EndFlowerPotModels.MODELS_DIR` (org/betterx/betterend/client/render/EndFlowerPotModels.java:27)
- [x] `EndFlowerPotModels.POTTED_SUFFIX` (org/betterx/betterend/client/render/EndFlowerPotModels.java:28)
- [x] `EndFlowerPotModels.JSON_SUFFIX` (org/betterx/betterend/client/render/EndFlowerPotModels.java:29)
- [x] `EndFlowerPotModels.MINECRAFT_NAMESPACE` (org/betterx/betterend/client/render/EndFlowerPotModels.java:30)
- [x] `EndFlowerPotModels.VANILLA_POTTED_PREFIX` (org/betterx/betterend/client/render/EndFlowerPotModels.java:31)
- [x] `EndFlowerPotModels.VANILLA_POTTED_OVERRIDES` (org/betterx/betterend/client/render/EndFlowerPotModels.java:32)
- [ ] `EndFlowerPotModels.POTTED_KEYS`: it uses Fabric's `ExtraModelKey` (org/betterx/betterend/client/render/EndFlowerPotModels.java:35)
- [x] `EndFlowerPotModels.EndFlowerPotModels(0)` (org/betterx/betterend/client/render/EndFlowerPotModels.java:37)
- [ ] `EndFlowerPotModels.register(0)`: a method reference (org/betterx/betterend/client/render/EndFlowerPotModels.java:40)
- [ ] `EndFlowerPotModels.scan(2)`: it uses Minecraft's `SharedState` (org/betterx/betterend/client/render/EndFlowerPotModels.java:44)
- [ ] `EndFlowerPotModels.initialize(2)`: it uses Fabric's `Context` (org/betterx/betterend/client/render/EndFlowerPotModels.java:62)
- [ ] `EndFlowerPotModels.getPottedModel(1)`: it uses Minecraft's `BlockStateModel` (org/betterx/betterend/client/render/EndFlowerPotModels.java:72)
- [ ] `EndFlowerPotModels.getVanillaPottedModel(1)`: it uses Minecraft's `BlockStateModel` (org/betterx/betterend/client/render/EndFlowerPotModels.java:92)
- [ ] `EndFlowerPotModels.hasVanillaPottedModel(1)`: it uses Minecraft's `Block` (org/betterx/betterend/client/render/EndFlowerPotModels.java:96)

### `EndTinterKeys`

- [ ] `EndTinterKeys.END_PORTAL`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/client/render/EndTinterKeys.java:7)
- [x] `EndTinterKeys.EndTinterKeys(0)` (org/betterx/betterend/client/render/EndTinterKeys.java:9)

### `EternalCrystalRenderer`

- [ ] `EternalCrystalRenderer.RENDER_LAYER`: it uses Minecraft's `RenderType` (org/betterx/betterend/client/render/EternalCrystalRenderer.java:24)
- [ ] `EternalCrystalRenderer.SHARDS`: it uses Minecraft's `ModelPart` (org/betterx/betterend/client/render/EternalCrystalRenderer.java:25)
- [ ] `EternalCrystalRenderer.CORE`: it uses Minecraft's `ModelPart` (org/betterx/betterend/client/render/EternalCrystalRenderer.java:26)
- [ ] `EternalCrystalRenderer.render(5)`: it uses Minecraft's `SubmitNodeCollector` (org/betterx/betterend/client/render/EternalCrystalRenderer.java:28)
- [ ] `EternalCrystalRenderer.render(6)`: it uses Minecraft's `SubmitNodeCollector` (org/betterx/betterend/client/render/EternalCrystalRenderer.java:32)
- [ ] `EternalCrystalRenderer.colors(1)`: it calls `MHelper` (org.betterx.bclib.util).floor (org/betterx/betterend/client/render/EternalCrystalRenderer.java:52)
- [ ] `EternalCrystalRenderer.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/client/render/EternalCrystalRenderer.java:66)

### `FlowerPotItemRenderer`: it implements Minecraft's `BlockEntityRenderer`

- [x] `FlowerPotItemRenderer.INNER_MIN` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:38)
- [x] `FlowerPotItemRenderer.INNER_MAX` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:39)
- [x] `FlowerPotItemRenderer.SOIL_Y` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:40)
- [x] `FlowerPotItemRenderer.PLANT_SCALE` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:41)
- [x] `FlowerPotItemRenderer.FALLBACK_PLANT_SCALE` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:42)
- [ ] `FlowerPotItemRenderer.FlowerPotItemRenderer(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:45)
- [ ] `FlowerPotItemRenderer.createRenderState(0)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:49)
- [ ] `FlowerPotItemRenderer.extractRenderState(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:53)
- [ ] `FlowerPotItemRenderer.submit(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:75)
- [ ] `FlowerPotItemRenderer.renderSoilPlane(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:85)
- [ ] `FlowerPotItemRenderer.emitVertex(10)`: it uses Minecraft's `VertexConsumer` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:110)
- [ ] `FlowerPotItemRenderer.renderPlantModel(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:119)
- [ ] `FlowerPotItemRenderer.renderBakedModel(6)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:133)
- [ ] `FlowerPotItemRenderer.renderFallbackModel(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:152)
- [ ] `FlowerPotItemRenderer.collectModelParts(2)`: it uses Minecraft's `BlockStateModelPart` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:163)
- [ ] `FlowerPotItemRenderer.topSprite(1)`: it uses Minecraft's `TextureAtlasSprite` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:174)
- [ ] `FlowerPotItemRenderer.firstRenderType(1)`: it uses Minecraft's `RenderType` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java:185)

### `PedestalItemRenderer`: it implements Minecraft's `BlockEntityRenderer`

- [x] `PedestalItemRenderer.SOCKET_MISSING_RGB` (org/betterx/betterend/client/render/PedestalItemRenderer.java:43)
- [x] `PedestalItemRenderer.SOCKET_PLACED_RGB` (org/betterx/betterend/client/render/PedestalItemRenderer.java:44)
- [x] `PedestalItemRenderer.SOCKET_NORTH_MISSING_RGB` (org/betterx/betterend/client/render/PedestalItemRenderer.java:45)
- [x] `PedestalItemRenderer.SOCKET_NORTH_PLACED_RGB` (org/betterx/betterend/client/render/PedestalItemRenderer.java:46)
- [x] `PedestalItemRenderer.FRAME_MISSING_RGB` (org/betterx/betterend/client/render/PedestalItemRenderer.java:47)
- [x] `PedestalItemRenderer.CRYSTAL_GHOST_ALPHA` (org/betterx/betterend/client/render/PedestalItemRenderer.java:48)
- [ ] `PedestalItemRenderer.PedestalItemRenderer(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/PedestalItemRenderer.java:51)
- [ ] `PedestalItemRenderer.createRenderState(0)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/PedestalItemRenderer.java:55)
- [ ] `PedestalItemRenderer.extractRenderState(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/PedestalItemRenderer.java:59)
- [ ] `PedestalItemRenderer.extractInfusionHint(4)`: it uses Minecraft's `Level` (org/betterx/betterend/client/render/PedestalItemRenderer.java:83)
- [ ] `PedestalItemRenderer.extractEternalVision(4)`: it uses Minecraft's `Level` (org/betterx/betterend/client/render/PedestalItemRenderer.java:96)
- [ ] `PedestalItemRenderer.submitEternalVision(3)`: it uses PedestalRenderState, which isn't translated (it extends Minecraft's `BlockEntityRenderState`) (org/betterx/betterend/client/render/PedestalItemRenderer.java:122)
- [ ] `PedestalItemRenderer.submitInfusionHint(3)`: it uses PedestalRenderState, which isn't translated (it extends Minecraft's `BlockEntityRenderState`) (org/betterx/betterend/client/render/PedestalItemRenderer.java:141)
- [ ] `PedestalItemRenderer.submitOutline(5)`: it uses Minecraft's `SubmitNodeCollector` (org/betterx/betterend/client/render/PedestalItemRenderer.java:158)
- [ ] `PedestalItemRenderer.submit(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (org/betterx/betterend/client/render/PedestalItemRenderer.java:162)
- [ ] `PedestalItemRenderer.getGemAge(0)`: it calls Minecraft's ClientLevel.getGameTime (no Horizon helper for it yet) (org/betterx/betterend/client/render/PedestalItemRenderer.java:202)

### `FlowerPotRenderState`: it extends Minecraft's `BlockEntityRenderState`


### `PedestalRenderState`: it extends Minecraft's `BlockEntityRenderState`


### `CommandRegistry`

- [x] `CommandRegistry.biomeIndex` (org/betterx/betterend/commands/CommandRegistry.java:47)
- [x] `CommandRegistry.MAX_SEARCH_RADIUS` (org/betterx/betterend/commands/CommandRegistry.java:48)
- [x] `CommandRegistry.SAMPLE_RESOLUTION_HORIZONTAL` (org/betterx/betterend/commands/CommandRegistry.java:49)
- [x] `CommandRegistry.SAMPLE_RESOLUTION_VERTICAL` (org/betterx/betterend/commands/CommandRegistry.java:50)
- [ ] `CommandRegistry.ERROR_BIOME_NOT_FOUND`: it uses Minecraft's `DynamicCommandExceptionType` (org/betterx/betterend/commands/CommandRegistry.java:51)
- [ ] `CommandRegistry.register(0)`: it uses Fabric's `CommandRegistrationCallback`.EVENT (org/betterx/betterend/commands/CommandRegistry.java:55)
- [ ] `CommandRegistry.register(3)`: it uses Minecraft's `CommandDispatcher` (org/betterx/betterend/commands/CommandRegistry.java:59)
- [ ] `CommandRegistry.find_poi(2)`: it uses `WoverPoiType` (de.ambertation.wover.poi.api) (org/betterx/betterend/commands/CommandRegistry.java:78)
- [ ] `CommandRegistry.teleportToNextBiome(1)`: its body couldn't be read: an anonymous class (org/betterx/betterend/commands/CommandRegistry.java:97)

### `ColoredMaterial`: it uses `BlockKey` (de.ambertation.wover.block.api.classify)

- [ ] `ColoredMaterial.DYES`: it uses Minecraft's `ItemLike` (org/betterx/betterend/complexmaterials/ColoredMaterial.java:28)
- [ ] `ColoredMaterial.COLORS`: it calls `Maps` (com.google.common.collect).newLinkedHashMap (org/betterx/betterend/complexmaterials/ColoredMaterial.java:29)
- [ ] `ColoredMaterial.VANILLA_COLORS`: it uses Minecraft's `DyeColor` (org/betterx/betterend/complexmaterials/ColoredMaterial.java:40)
- [ ] `ColoredMaterial.ColoredMaterial(8)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:59)
- [ ] `ColoredMaterial.ColoredMaterial(10)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:72)
- [ ] `ColoredMaterial.register(1)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:104)
- [ ] `ColoredMaterial.keys(0)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:121)
- [ ] `ColoredMaterial.dye(1)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:125)
- [ ] `ColoredMaterial.source(0)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:129)
- [ ] `ColoredMaterial.craftsEight(0)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:133)
- [ ] `ColoredMaterial.getByColor(1)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:137)
- [ ] `ColoredMaterial.getByColor(1)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/ColoredMaterial.java:141)

### `CrystalSubblocksMaterial`: it uses `BlockKey` (de.ambertation.wover.block.api.classify)

- [ ] `CrystalSubblocksMaterial.BRICK_SLAB_MODEL`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:36)
- [ ] `CrystalSubblocksMaterial.CrystalSubblocksMaterial(2)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:45)
- [ ] `CrystalSubblocksMaterial.key(1)`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:62)

### `CrystalSubblocksMaterial.BuildPedestalParams`: it uses Minecraft's `Block`

- [ ] `CrystalSubblocksMaterial.BuildPedestalParams.BuildPedestalParams(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:66)
- [ ] `CrystalSubblocksMaterial.BuildPedestalParams.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:66)

### `CrystalSubblocksMaterial.BuildSlabParams`: it uses Minecraft's `Block`

- [ ] `CrystalSubblocksMaterial.BuildSlabParams.BuildSlabParams(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:69)
- [ ] `CrystalSubblocksMaterial.BuildSlabParams.baseBlock(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:69)

### `CrystalSubblocksMaterial.ClientModel`

- [ ] `CrystalSubblocksMaterial.ClientModel.buildSlabFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:74)
- [ ] `CrystalSubblocksMaterial.ClientModel.buildPedestalFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:78)
- [ ] `CrystalSubblocksMaterial.ClientModel.buildSlab(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:82)
- [ ] `CrystalSubblocksMaterial.ClientModel.buildPedestal(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:86)

### `CrystalSubblocksMaterial.CrystalBrickSlab`: it uses Minecraft's `Block`

- [ ] `CrystalSubblocksMaterial.CrystalBrickSlab.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:92)
- [ ] `CrystalSubblocksMaterial.CrystalBrickSlab.CrystalBrickSlab(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:91)
- [ ] `CrystalSubblocksMaterial.CrystalBrickSlab.baseBlock(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java:91)

### `EndWoodenComplexMaterial`: it uses Minecraft's `MapColor`

- [ ] `EndWoodenComplexMaterial.EndWoodenComplexMaterial(3)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:48)
- [ ] `EndWoodenComplexMaterial.setFurnitureCloth(1)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:54)
- [ ] `EndWoodenComplexMaterial.setLogVariants(1)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:59)
- [ ] `EndWoodenComplexMaterial.setStrippedVariants(1)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:64)
- [ ] `EndWoodenComplexMaterial.useRaft(0)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:69)
- [ ] `EndWoodenComplexMaterial.build(0)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:74)
- [ ] `EndWoodenComplexMaterial.register(1)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:95)
- [ ] `EndWoodenComplexMaterial.defineBlocks(1)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:99)
- [ ] `EndWoodenComplexMaterial.addFurniture(3)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:119)
- [ ] `EndWoodenComplexMaterial.furniture(3)`: it uses `EntryConfig` (de.ambertation.wover.sets.api.classify.BlockSet) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:126)
- [ ] `EndWoodenComplexMaterial.weightedLogAndBark(3)`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:131)
- [ ] `EndWoodenComplexMaterial.isLogKind(1)`: it uses `Kind` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:149)
- [ ] `EndWoodenComplexMaterial.isFurniture(1)`: it uses `Kind` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:153)
- [ ] `EndWoodenComplexMaterial.blocks(0)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:157)
- [ ] `EndWoodenComplexMaterial.woodType(0)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:161)
- [ ] `EndWoodenComplexMaterial.isTreeLog(1)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:165)
- [ ] `EndWoodenComplexMaterial.isTreeLog(1)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:169)
- [ ] `EndWoodenComplexMaterial.getLog(0)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:173)
- [ ] `EndWoodenComplexMaterial.getBark(0)`: its class isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java:177)

### `JellyLucerniaWoodMaterial`: it extends `EndWoodenComplexMaterial`

- [ ] `JellyLucerniaWoodMaterial.JellyLucerniaWoodMaterial(0)`: its class isn't translated (it extends `EndWoodenComplexMaterial`) (org/betterx/betterend/complexmaterials/JellyLucerniaWoodMaterial.java:14)
- [ ] `JellyLucerniaWoodMaterial.defineBlocks(1)`: its class isn't translated (it extends `EndWoodenComplexMaterial`) (org/betterx/betterend/complexmaterials/JellyLucerniaWoodMaterial.java:18)

### `MetalMaterial`: it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))

- [ ] `MetalMaterial.makeNormal(6)`: it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/complexmaterials/MetalMaterial.java:59)
- [ ] `MetalMaterial.makeNormal(7)`: it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/complexmaterials/MetalMaterial.java:65)
- [ ] `MetalMaterial.makeOreless(8)`: it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/complexmaterials/MetalMaterial.java:89)
- [ ] `MetalMaterial.makeOreless(9)`: it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/complexmaterials/MetalMaterial.java:102)
- [ ] `MetalMaterial.MetalMaterial(11)`: its class isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/complexmaterials/MetalMaterial.java:128)
- [ ] `MetalMaterial.oreInstance(0)`: its class isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/complexmaterials/MetalMaterial.java:236)
- [ ] `MetalMaterial.harvestTierTag(0)`: its class isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/complexmaterials/MetalMaterial.java:241)
- [ ] `MetalMaterial.registerRecipes(1)`: its class isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/complexmaterials/MetalMaterial.java:246)

### `StoneMaterial`

- [x] `StoneMaterial.StoneMaterial(0)` (org/betterx/betterend/complexmaterials/StoneMaterial.java:22)
- [ ] `StoneMaterial.declare(2)`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/complexmaterials/StoneMaterial.java:25)

### `VanillaMetalSet`

- [x] `VanillaMetalSet.VanillaMetalSet(0)` (org/betterx/betterend/complexmaterials/VanillaMetalSet.java:16)
- [ ] `VanillaMetalSet.declare(5)`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/complexmaterials/VanillaMetalSet.java:19)

### `VanillaStoneSet`

- [x] `VanillaStoneSet.VanillaStoneSet(0)` (org/betterx/betterend/complexmaterials/VanillaStoneSet.java:15)
- [ ] `VanillaStoneSet.declare(3)`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/complexmaterials/VanillaStoneSet.java:18)

### `VanillaVariantStoneMaterial`

- [x] `VanillaVariantStoneMaterial.VanillaVariantStoneMaterial(0)` (org/betterx/betterend/complexmaterials/VanillaVariantStoneMaterial.java:17)
- [ ] `VanillaVariantStoneMaterial.declare(3)`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/complexmaterials/VanillaVariantStoneMaterial.java:20)

### `ClientConfig`: it extends `ConfigFile` (de.ambertation.wunderlib.configs)

- [ ] `ClientConfig.ClientConfig(0)`: its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betterend/config/ClientConfig.java:21)

### `Configs`

- [ ] `Configs.GENERATOR_CONFIG`: it uses GeneratorConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betterend/config/Configs.java:4)
- [ ] `Configs.CLIENT_CONFIG`: it uses ClientConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betterend/config/Configs.java:5)
- [ ] `Configs.saveConfigs(0)`: unknown name `de` (org/betterx/betterend/config/Configs.java:7)

### `GeneratorConfig`: it extends `ConfigFile` (de.ambertation.wunderlib.configs)

- [ ] `GeneratorConfig.GENERATOR`: it uses `Group` (de.ambertation.wunderlib.configs.AbstractConfig) (org/betterx/betterend/config/GeneratorConfig.java:20)
- [ ] `GeneratorConfig.GeneratorConfig(0)`: its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betterend/config/GeneratorConfig.java:95)

### `GeneratorConfig.LayerOptionsValue`: it extends `Value`

- [ ] `GeneratorConfig.LayerOptionsValue.LayerOptionsValue(3)`: its class isn't translated (it extends `Value`) (org/betterx/betterend/config/GeneratorConfig.java:100)
- [ ] `GeneratorConfig.LayerOptionsValue.LayerOptionsValue(4)`: its class isn't translated (it extends `Value`) (org/betterx/betterend/config/GeneratorConfig.java:104)
- [ ] `GeneratorConfig.LayerOptionsValue.LayerOptionsValue(1)`: its class isn't translated (it extends `Value`) (org/betterx/betterend/config/GeneratorConfig.java:108)
- [ ] `GeneratorConfig.LayerOptionsValue.LayerOptionsValue(2)`: its class isn't translated (it extends `Value`) (org/betterx/betterend/config/GeneratorConfig.java:112)
- [ ] `GeneratorConfig.LayerOptionsValue.convert(1)`: its class isn't translated (it extends `Value`) (org/betterx/betterend/config/GeneratorConfig.java:116)
- [ ] `GeneratorConfig.LayerOptionsValue.convert(1)`: its class isn't translated (it extends `Value`) (org/betterx/betterend/config/GeneratorConfig.java:120)
- [ ] `GeneratorConfig.LayerOptionsValue.parseString(1)`: its class isn't translated (it extends `Value`) (org/betterx/betterend/config/GeneratorConfig.java:125)

### `ConfigScreen`: it extends `ConfigScreen`

- [ ] `ConfigScreen.BE_LOGO_LOCATION`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/config/screen/ConfigScreen.java:11)
- [ ] `ConfigScreen.ConfigScreen(1)`: its class isn't translated (it extends `ConfigScreen`) (org/betterx/betterend/config/screen/ConfigScreen.java:13)
- [ ] `ConfigScreen.onClose(0)`: its class isn't translated (it extends `ConfigScreen`) (org/betterx/betterend/config/screen/ConfigScreen.java:17)

### `EndPotions`

- [ ] `EndPotions.END_VEIL`: it uses Minecraft's `Holder` (org/betterx/betterend/effects/EndPotions.java:16)
- [ ] `EndPotions.LONG_END_VEIL`: it uses Minecraft's `Holder` (org/betterx/betterend/effects/EndPotions.java:17)
- [ ] `EndPotions.register(0)`: it uses `PotionManager` (de.ambertation.wover.potions.api).BOOTSTRAP_POTIONS (org/betterx/betterend/effects/EndPotions.java:19)
- [ ] `EndPotions.bootstrap(1)`: it uses `BrewingBuilder` (de.ambertation.wover.potions.api) (org/betterx/betterend/effects/EndPotions.java:24)

### `EndStatusEffects`

- [ ] `EndStatusEffects.CRYSTALITE_HEALTH_REGEN`: it uses Minecraft's `MobEffectInstance` (org/betterx/betterend/effects/EndStatusEffects.java:14)
- [ ] `EndStatusEffects.CRYSTALITE_DIG_SPEED`: it uses Minecraft's `MobEffectInstance` (org/betterx/betterend/effects/EndStatusEffects.java:15)
- [ ] `EndStatusEffects.CRYSTALITE_MOVE_SPEED`: it uses Minecraft's `MobEffectInstance` (org/betterx/betterend/effects/EndStatusEffects.java:16)
- [ ] `EndStatusEffects.END_VEIL`: it uses Minecraft's `Holder` (org/betterx/betterend/effects/EndStatusEffects.java:17)
- [ ] `EndStatusEffects.registerEffect(2)`: it's generic (org/betterx/betterend/effects/EndStatusEffects.java:19)
- [x] `EndStatusEffects.ensureStaticallyLoaded(0)` (org/betterx/betterend/effects/EndStatusEffects.java:23)

### `EndVeilEffect`: it extends Minecraft's `MobEffect`

- [ ] `EndVeilEffect.EndVeilEffect(0)`: its class isn't translated (it extends Minecraft's `MobEffect`) (org/betterx/betterend/effects/status/EndVeilEffect.java:7)
- [ ] `EndVeilEffect.shouldApplyEffectTickThisTick(2)`: its class isn't translated (it extends Minecraft's `MobEffect`) (org/betterx/betterend/effects/status/EndVeilEffect.java:11)

### `CubozoaEntity`: it extends Minecraft's `AbstractSchoolingFish`

- [ ] `CubozoaEntity.VARIANT`: it uses Minecraft's `EntityDataAccessor` (org/betterx/betterend/entity/CubozoaEntity.java:44)
- [ ] `CubozoaEntity.SCALE`: it uses Minecraft's `EntityDataAccessor` (org/betterx/betterend/entity/CubozoaEntity.java:45)
- [ ] `CubozoaEntity.CubozoaEntity(2)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:47)
- [ ] `CubozoaEntity.finalizeSpawn(4)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:52)
- [ ] `CubozoaEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:66)
- [ ] `CubozoaEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:72)
- [ ] `CubozoaEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:78)
- [ ] `CubozoaEntity.saveToBucketTag(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:85)
- [ ] `CubozoaEntity.getBucketItemStack(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:93)
- [ ] `CubozoaEntity.createMobAttributes(0)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/entity/CubozoaEntity.java:98)
- [ ] `CubozoaEntity.getVariant(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:102)
- [ ] `CubozoaEntity.updateScaleAttribute(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:106)
- [ ] `CubozoaEntity.getStandingEyeHeight(2)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:113)
- [ ] `CubozoaEntity.dropFromLootTable(3)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:117)
- [ ] `CubozoaEntity.getFlopSound(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:125)
- [ ] `CubozoaEntity.playerTouch(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/CubozoaEntity.java:129)

### `DragonflyEntity`: it extends `DespawnableAnimal` (org.betterx.bclib.entity)

- [ ] `DragonflyEntity.DragonflyEntity(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:42)
- [ ] `DragonflyEntity.createMobAttributes(0)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/entity/DragonflyEntity.java:51)
- [ ] `DragonflyEntity.canBeLeashed(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:59)
- [ ] `DragonflyEntity.createNavigation(1)`: its body couldn't be read: an anonymous class (org/betterx/betterend/entity/DragonflyEntity.java:63)
- [ ] `DragonflyEntity.getWalkTargetValue(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:80)
- [ ] `DragonflyEntity.isFood(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:84)
- [ ] `DragonflyEntity.registerGoals(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:88)
- [ ] `DragonflyEntity.isPushable(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:95)
- [ ] `DragonflyEntity.causeFallDamage(3)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:99)
- [ ] `DragonflyEntity.getMovementEmission(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:103)
- [ ] `DragonflyEntity.omnidirectionalAirMover(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:107)
- [ ] `DragonflyEntity.isNoGravity(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:111)
- [ ] `DragonflyEntity.getAmbientSound(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:115)
- [ ] `DragonflyEntity.getSoundVolume(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:119)
- [ ] `DragonflyEntity.getBreedOffspring(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betterend/entity/DragonflyEntity.java:123)

### `DragonflyEntity.DragonflyLookControl`: it extends Minecraft's `LookControl`

- [ ] `DragonflyEntity.DragonflyLookControl.DragonflyLookControl(1)`: its class isn't translated (it extends Minecraft's `LookControl`) (org/betterx/betterend/entity/DragonflyEntity.java:128)
- [ ] `DragonflyEntity.DragonflyLookControl.resetXRotOnTick(0)`: its class isn't translated (it extends Minecraft's `LookControl`) (org/betterx/betterend/entity/DragonflyEntity.java:132)

### `DragonflyEntity.WanderAroundGoal`: it extends Minecraft's `Goal`

- [ ] `DragonflyEntity.WanderAroundGoal.WanderAroundGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/DragonflyEntity.java:138)
- [ ] `DragonflyEntity.WanderAroundGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/DragonflyEntity.java:142)
- [ ] `DragonflyEntity.WanderAroundGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/DragonflyEntity.java:146)
- [ ] `DragonflyEntity.WanderAroundGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/DragonflyEntity.java:150)
- [ ] `DragonflyEntity.WanderAroundGoal.getRandomLocation(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/DragonflyEntity.java:167)
- [ ] `DragonflyEntity.WanderAroundGoal.isInVoid(1)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/DragonflyEntity.java:193)

### `EndFishEntity`: it extends Minecraft's `AbstractSchoolingFish`

- [x] `EndFishEntity.VARIANTS_NORMAL` (org/betterx/betterend/entity/EndFishEntity.java:43)
- [x] `EndFishEntity.VARIANTS_SULPHUR` (org/betterx/betterend/entity/EndFishEntity.java:44)
- [x] `EndFishEntity.VARIANTS` (org/betterx/betterend/entity/EndFishEntity.java:45)
- [ ] `EndFishEntity.VARIANT`: it uses Minecraft's `EntityDataAccessor` (org/betterx/betterend/entity/EndFishEntity.java:46)
- [ ] `EndFishEntity.SCALE`: it uses Minecraft's `EntityDataAccessor` (org/betterx/betterend/entity/EndFishEntity.java:47)
- [ ] `EndFishEntity.EndFishEntity(2)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:49)
- [ ] `EndFishEntity.finalizeSpawn(4)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:54)
- [ ] `EndFishEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:65)
- [ ] `EndFishEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:71)
- [ ] `EndFishEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:77)
- [ ] `EndFishEntity.saveToBucketTag(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:84)
- [ ] `EndFishEntity.getBucketItemStack(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:92)
- [ ] `EndFishEntity.getFlopSound(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:97)
- [ ] `EndFishEntity.getAmbientSound(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:102)
- [ ] `EndFishEntity.getDeathSound(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:106)
- [ ] `EndFishEntity.getHurtSound(1)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:110)
- [ ] `EndFishEntity.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:114)
- [ ] `EndFishEntity.createMobAttributes(0)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/entity/EndFishEntity.java:124)
- [ ] `EndFishEntity.getVariant(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:128)
- [ ] `EndFishEntity.updateScaleAttribute(0)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:132)
- [ ] `EndFishEntity.dropFromLootTable(3)`: its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`) (org/betterx/betterend/entity/EndFishEntity.java:139)

### `EndSlimeEntity`: it extends Minecraft's `Slime`

- [ ] `EndSlimeEntity.VARIANT`: it uses Minecraft's `EntityDataAccessor` (org/betterx/betterend/entity/EndSlimeEntity.java:53)
- [ ] `EndSlimeEntity.EndSlimeEntity(2)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:55)
- [ ] `EndSlimeEntity.registerGoals(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:60)
- [ ] `EndSlimeEntity.createMobAttributes(0)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/entity/EndSlimeEntity.java:73)
- [ ] `EndSlimeEntity.finalizeSpawn(4)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:81)
- [ ] `EndSlimeEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:102)
- [ ] `EndSlimeEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:107)
- [ ] `EndSlimeEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:112)
- [ ] `EndSlimeEntity.getParticleType(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:117)
- [ ] `EndSlimeEntity.setUpSplitCube(4)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:121)
- [ ] `EndSlimeEntity.dropFromLootTable(3)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:129)
- [ ] `EndSlimeEntity.getSlimeType(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:147)
- [ ] `EndSlimeEntity.setSlimeType(1)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:151)
- [ ] `EndSlimeEntity.setMossy(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:155)
- [ ] `EndSlimeEntity.isMossy(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:159)
- [ ] `EndSlimeEntity.setLake(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:163)
- [ ] `EndSlimeEntity.isLake(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:167)
- [ ] `EndSlimeEntity.setAmber(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:171)
- [ ] `EndSlimeEntity.isAmber(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:175)
- [ ] `EndSlimeEntity.isChorus(0)`: its class isn't translated (it extends Minecraft's `Slime`) (org/betterx/betterend/entity/EndSlimeEntity.java:179)
- [ ] `EndSlimeEntity.canSpawn(5)`: it uses Minecraft's `EntityType` (org/betterx/betterend/entity/EndSlimeEntity.java:183)
- [ ] `EndSlimeEntity.isWaterNear(2)`: it uses Minecraft's `LevelAccessor` (org/betterx/betterend/entity/EndSlimeEntity.java:206)

### `EndSlimeEntity.EndSlimeMoveControl`: it extends Minecraft's `MoveControl`

- [ ] `EndSlimeEntity.EndSlimeMoveControl.EndSlimeMoveControl(1)`: its class isn't translated (it extends Minecraft's `MoveControl`) (org/betterx/betterend/entity/EndSlimeEntity.java:232)
- [ ] `EndSlimeEntity.EndSlimeMoveControl.look(2)`: its class isn't translated (it extends Minecraft's `MoveControl`) (org/betterx/betterend/entity/EndSlimeEntity.java:237)
- [ ] `EndSlimeEntity.EndSlimeMoveControl.move(1)`: its class isn't translated (it extends Minecraft's `MoveControl`) (org/betterx/betterend/entity/EndSlimeEntity.java:242)
- [ ] `EndSlimeEntity.EndSlimeMoveControl.tick(0)`: its class isn't translated (it extends Minecraft's `MoveControl`) (org/betterx/betterend/entity/EndSlimeEntity.java:247)
- [ ] `EndSlimeEntity.EndSlimeMoveControl.getJumpSoundPitch(0)`: its class isn't translated (it extends Minecraft's `MoveControl`) (org/betterx/betterend/entity/EndSlimeEntity.java:278)

### `EndSlimeEntity.FaceTowardTargetGoal`: it extends Minecraft's `Goal`

- [ ] `EndSlimeEntity.FaceTowardTargetGoal.FaceTowardTargetGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:287)
- [ ] `EndSlimeEntity.FaceTowardTargetGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:291)
- [ ] `EndSlimeEntity.FaceTowardTargetGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:303)
- [ ] `EndSlimeEntity.FaceTowardTargetGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:308)
- [ ] `EndSlimeEntity.FaceTowardTargetGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:319)

### `EndSlimeEntity.MoveGoal`: it extends Minecraft's `Goal`

- [ ] `EndSlimeEntity.MoveGoal.MoveGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:326)
- [ ] `EndSlimeEntity.MoveGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:330)
- [ ] `EndSlimeEntity.MoveGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:348)

### `EndSlimeEntity.RandomLookGoal`: it extends Minecraft's `Goal`

- [ ] `EndSlimeEntity.RandomLookGoal.RandomLookGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:357)
- [ ] `EndSlimeEntity.RandomLookGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:361)
- [ ] `EndSlimeEntity.RandomLookGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:372)

### `EndSlimeEntity.SwimmingGoal`: it extends Minecraft's `Goal`

- [ ] `EndSlimeEntity.SwimmingGoal.SwimmingGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:383)
- [ ] `EndSlimeEntity.SwimmingGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:388)
- [ ] `EndSlimeEntity.SwimmingGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/EndSlimeEntity.java:393)

### `CubozoaEntityModel`: it extends Minecraft's `EntityModel`

- [x] `CubozoaEntityModel.TENTACLE_COUNT` (org/betterx/betterend/entity/model/CubozoaEntityModel.java:15)
- [ ] `CubozoaEntityModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/entity/model/CubozoaEntityModel.java:20)
- [ ] `CubozoaEntityModel.CubozoaEntityModel(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/CubozoaEntityModel.java:44)
- [ ] `CubozoaEntityModel.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/CubozoaEntityModel.java:55)

### `DragonflyEntityModel`: it extends Minecraft's `EntityModel`

- [ ] `DragonflyEntityModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/entity/model/DragonflyEntityModel.java:25)
- [ ] `DragonflyEntityModel.DragonflyEntityModel(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/DragonflyEntityModel.java:67)
- [ ] `DragonflyEntityModel.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/DragonflyEntityModel.java:81)

### `EndFishEntityModel`: it extends Minecraft's `EntityModel`

- [ ] `EndFishEntityModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/entity/model/EndFishEntityModel.java:21)
- [ ] `EndFishEntityModel.EndFishEntityModel(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/EndFishEntityModel.java:55)
- [ ] `EndFishEntityModel.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/EndFishEntityModel.java:65)

### `EndSlimeEntityModel`: it extends Minecraft's `EntityModel`

- [ ] `EndSlimeEntityModel.getShellOnlyTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:29)
- [ ] `EndSlimeEntityModel.getCompleteTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:33)
- [ ] `EndSlimeEntityModel.getTexturedModelData(1)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:37)
- [ ] `EndSlimeEntityModel.EndSlimeEntityModel(2)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:65)
- [ ] `EndSlimeEntityModel.EndSlimeEntityModel(2)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:69)
- [ ] `EndSlimeEntityModel.buildRenderRoot(2)`: it uses Minecraft's `ModelPart` (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:87)
- [ ] `EndSlimeEntityModel.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:99)
- [ ] `EndSlimeEntityModel.renderFlower(4)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:102)
- [ ] `EndSlimeEntityModel.renderCrop(4)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/EndSlimeEntityModel.java:106)

### `SilkMothEntityModel`: it extends Minecraft's `EntityModel`

- [ ] `SilkMothEntityModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/entity/model/SilkMothEntityModel.java:35)
- [ ] `SilkMothEntityModel.SilkMothEntityModel(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/SilkMothEntityModel.java:108)
- [ ] `SilkMothEntityModel.buildRenderRoot(1)`: it uses Minecraft's `ModelPart` (org/betterx/betterend/entity/model/SilkMothEntityModel.java:127)
- [ ] `SilkMothEntityModel.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betterend/entity/model/SilkMothEntityModel.java:136)

### `RendererEntityCubozoa`: it extends Minecraft's `MobRenderer`

- [x] `RendererEntityCubozoa.TEXTURE` (org/betterx/betterend/entity/render/RendererEntityCubozoa.java:20)
- [ ] `RendererEntityCubozoa.GLOW`: it uses Minecraft's `RenderType` (org/betterx/betterend/entity/render/RendererEntityCubozoa.java:21)
- [ ] `RendererEntityCubozoa.RendererEntityCubozoa(1)`: its body couldn't be read: an anonymous class (org/betterx/betterend/entity/render/RendererEntityCubozoa.java:23)
- [ ] `RendererEntityCubozoa.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityCubozoa.java:48)
- [ ] `RendererEntityCubozoa.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityCubozoa.java:52)
- [ ] `RendererEntityCubozoa.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityCubozoa.java:57)

### `RendererEntityDragonfly`: it extends Minecraft's `MobRenderer`

- [ ] `RendererEntityDragonfly.TEXTURE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/entity/render/RendererEntityDragonfly.java:17)
- [ ] `RendererEntityDragonfly.GLOW`: it uses Minecraft's `RenderType` (org/betterx/betterend/entity/render/RendererEntityDragonfly.java:18)
- [ ] `RendererEntityDragonfly.RendererEntityDragonfly(1)`: its body couldn't be read: an anonymous class (org/betterx/betterend/entity/render/RendererEntityDragonfly.java:20)
- [ ] `RendererEntityDragonfly.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityDragonfly.java:29)
- [ ] `RendererEntityDragonfly.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityDragonfly.java:33)

### `RendererEntityEndFish`: it extends Minecraft's `MobRenderer`

- [x] `RendererEntityEndFish.TEXTURE` (org/betterx/betterend/entity/render/RendererEntityEndFish.java:20)
- [ ] `RendererEntityEndFish.GLOW`: it uses Minecraft's `RenderType` (org/betterx/betterend/entity/render/RendererEntityEndFish.java:21)
- [ ] `RendererEntityEndFish.RendererEntityEndFish(1)`: its body couldn't be read: an anonymous class (org/betterx/betterend/entity/render/RendererEntityEndFish.java:23)
- [ ] `RendererEntityEndFish.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityEndFish.java:48)
- [ ] `RendererEntityEndFish.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityEndFish.java:52)
- [ ] `RendererEntityEndFish.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityEndFish.java:57)

### `RendererEntityEndSlime`: it extends Minecraft's `MobRenderer`

- [x] `RendererEntityEndSlime.TEXTURE` (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:22)
- [ ] `RendererEntityEndSlime.GLOW`: it uses Minecraft's `RenderType` (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:23)
- [ ] `RendererEntityEndSlime.RendererEntityEndSlime(1)`: its body couldn't be read: an anonymous class (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:25)
- [ ] `RendererEntityEndSlime.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:51)
- [ ] `RendererEntityEndSlime.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:55)
- [ ] `RendererEntityEndSlime.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:65)
- [ ] `RendererEntityEndSlime.getShadowRadius(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:69)
- [ ] `RendererEntityEndSlime.scale(2)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:73)

### `RendererEntityEndSlime.OverlayFeatureRenderer`: it extends Minecraft's `RenderLayer`

- [ ] `RendererEntityEndSlime.OverlayFeatureRenderer.OverlayFeatureRenderer(2)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:97)
- [ ] `RendererEntityEndSlime.OverlayFeatureRenderer.submit(6)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:103)
- [ ] `RendererEntityEndSlime.OverlayFeatureRenderer.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (org/betterx/betterend/entity/render/RendererEntityEndSlime.java:134)

### `RendererEntityShadowWalker`: it extends Minecraft's `HumanoidMobRenderer`

- [ ] `RendererEntityShadowWalker.TEXTURE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/entity/render/RendererEntityShadowWalker.java:13)
- [ ] `RendererEntityShadowWalker.RendererEntityShadowWalker(1)`: its class isn't translated (it extends Minecraft's `HumanoidMobRenderer`) (org/betterx/betterend/entity/render/RendererEntityShadowWalker.java:15)
- [ ] `RendererEntityShadowWalker.createRenderState(0)`: its class isn't translated (it extends Minecraft's `HumanoidMobRenderer`) (org/betterx/betterend/entity/render/RendererEntityShadowWalker.java:19)
- [ ] `RendererEntityShadowWalker.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `HumanoidMobRenderer`) (org/betterx/betterend/entity/render/RendererEntityShadowWalker.java:23)

### `SilkMothEntityRenderer`: it extends Minecraft's `MobRenderer`

- [ ] `SilkMothEntityRenderer.TEXTURE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/entity/render/SilkMothEntityRenderer.java:13)
- [ ] `SilkMothEntityRenderer.SilkMothEntityRenderer(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/SilkMothEntityRenderer.java:15)
- [ ] `SilkMothEntityRenderer.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/SilkMothEntityRenderer.java:19)
- [ ] `SilkMothEntityRenderer.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betterend/entity/render/SilkMothEntityRenderer.java:23)

### `CubozoaRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `DragonflyRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `EndFishRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `EndSlimeRenderState`: it extends Minecraft's `SlimeRenderState`


### `ShadowWalkerRenderState`: it extends Minecraft's `HumanoidRenderState`


### `SilkMothRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `ShadowWalkerEntity`: it extends Minecraft's `Monster`

- [ ] `ShadowWalkerEntity.ShadowWalkerEntity(2)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:29)
- [ ] `ShadowWalkerEntity.registerGoals(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:33)
- [ ] `ShadowWalkerEntity.createMobAttributes(0)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/entity/ShadowWalkerEntity.java:41)
- [ ] `ShadowWalkerEntity.tick(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:50)
- [ ] `ShadowWalkerEntity.getAmbientSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:84)
- [ ] `ShadowWalkerEntity.getHurtSound(1)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:88)
- [ ] `ShadowWalkerEntity.getDeathSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:92)
- [ ] `ShadowWalkerEntity.playStepSound(2)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:96)
- [ ] `ShadowWalkerEntity.getSoundVolume(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:99)
- [ ] `ShadowWalkerEntity.getVoicePitch(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:103)
- [ ] `ShadowWalkerEntity.doHurtTarget(2)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:107)

### `ShadowWalkerEntity.AttackGoal`: it extends Minecraft's `MeleeAttackGoal`

- [ ] `ShadowWalkerEntity.AttackGoal.AttackGoal(3)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:120)
- [ ] `ShadowWalkerEntity.AttackGoal.start(0)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:125)
- [ ] `ShadowWalkerEntity.AttackGoal.stop(0)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:130)
- [ ] `ShadowWalkerEntity.AttackGoal.tick(0)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (org/betterx/betterend/entity/ShadowWalkerEntity.java:135)

### `SilkMothEntity`: it extends Minecraft's `Animal`

- [ ] `SilkMothEntity.SilkMothEntity(2)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:64)
- [ ] `SilkMothEntity.createMobAttributes(0)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/entity/SilkMothEntity.java:73)
- [ ] `SilkMothEntity.setHive(2)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:82)
- [ ] `SilkMothEntity.canBeLeashed(0)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:87)
- [ ] `SilkMothEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:91)
- [ ] `SilkMothEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:99)
- [ ] `SilkMothEntity.registerGoals(0)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:114)
- [ ] `SilkMothEntity.createNavigation(1)`: its body couldn't be read: an anonymous class (org/betterx/betterend/entity/SilkMothEntity.java:123)
- [ ] `SilkMothEntity.isPushable(0)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:139)
- [ ] `SilkMothEntity.causeFallDamage(3)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:143)
- [ ] `SilkMothEntity.getMovementEmission(0)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:147)
- [ ] `SilkMothEntity.omnidirectionalAirMover(0)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:151)
- [ ] `SilkMothEntity.isNoGravity(0)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:155)
- [ ] `SilkMothEntity.getBreedOffspring(2)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:159)
- [ ] `SilkMothEntity.dropFromLootTable(3)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:163)
- [ ] `SilkMothEntity.isFood(1)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:181)
- [ ] `SilkMothEntity.mobInteract(2)`: its class isn't translated (it extends Minecraft's `Animal`) (org/betterx/betterend/entity/SilkMothEntity.java:185)

### `SilkMothEntity.MothLookControl`: it extends Minecraft's `LookControl`

- [ ] `SilkMothEntity.MothLookControl.MothLookControl(1)`: its class isn't translated (it extends Minecraft's `LookControl`) (org/betterx/betterend/entity/SilkMothEntity.java:190)
- [ ] `SilkMothEntity.MothLookControl.resetXRotOnTick(0)`: its class isn't translated (it extends Minecraft's `LookControl`) (org/betterx/betterend/entity/SilkMothEntity.java:194)

### `SilkMothEntity.ReturnToHiveGoal`: it extends Minecraft's `Goal`

- [ ] `SilkMothEntity.ReturnToHiveGoal.ReturnToHiveGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:200)
- [ ] `SilkMothEntity.ReturnToHiveGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:204)
- [ ] `SilkMothEntity.ReturnToHiveGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:214)
- [ ] `SilkMothEntity.ReturnToHiveGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:223)
- [ ] `SilkMothEntity.ReturnToHiveGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:236)

### `SilkMothEntity.WanderAroundGoal`: it extends Minecraft's `Goal`

- [ ] `SilkMothEntity.WanderAroundGoal.WanderAroundGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:269)
- [ ] `SilkMothEntity.WanderAroundGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:273)
- [ ] `SilkMothEntity.WanderAroundGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:277)
- [ ] `SilkMothEntity.WanderAroundGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:281)
- [ ] `SilkMothEntity.WanderAroundGoal.getRandomLocation(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betterend/entity/SilkMothEntity.java:300)

### `ItemTooltipCallback`: it's an interface

- [ ] `ItemTooltipCallback.getTooltip(4)`: it has no body (org/betterx/betterend/events/ItemTooltipCallback.java:21)

### `PlayerAdvancementsCallback`: it's an interface

- [ ] `PlayerAdvancementsCallback.onAdvancementComplete(3)`: it has no body (org/betterx/betterend/events/PlayerAdvancementsCallback.java:17)

### `FlowerPotMigration`

- [x] `FlowerPotMigration.NAMESPACE` (org/betterx/betterend/FlowerPotMigration.java:22)
- [x] `FlowerPotMigration.BLOCK_SUFFIX` (org/betterx/betterend/FlowerPotMigration.java:23)
- [x] `FlowerPotMigration.BLOCK_ENTITY` (org/betterx/betterend/FlowerPotMigration.java:24)
- [x] `FlowerPotMigration.PLANT_ID` (org/betterx/betterend/FlowerPotMigration.java:25)
- [x] `FlowerPotMigration.SOIL_ID` (org/betterx/betterend/FlowerPotMigration.java:26)
- [x] `FlowerPotMigration.DEFAULT_PLANTS` (org/betterx/betterend/FlowerPotMigration.java:27)
- [x] `FlowerPotMigration.DEFAULT_SOILS` (org/betterx/betterend/FlowerPotMigration.java:75)
- [x] `FlowerPotMigration.plants` (org/betterx/betterend/FlowerPotMigration.java:89)
- [x] `FlowerPotMigration.soils` (org/betterx/betterend/FlowerPotMigration.java:90)
- [ ] `FlowerPotMigration.loadTables(0)`: new HashMap<>() without its types (org/betterx/betterend/FlowerPotMigration.java:92)
- [ ] `FlowerPotMigration.readTable(3)`: it changes an object through a second name (org/betterx/betterend/FlowerPotMigration.java:133)
- [ ] `FlowerPotMigration.patchChunk(2)`: it uses Minecraft's `CompoundTag` (org/betterx/betterend/FlowerPotMigration.java:151)
- [ ] `FlowerPotMigration.lookup(2)`: it calls `Integer`.parseInt (org/betterx/betterend/FlowerPotMigration.java:243)
- [ ] `FlowerPotMigration.hasEntityAt(2)`: it uses Minecraft's `ListTag` (org/betterx/betterend/FlowerPotMigration.java:256)
- [ ] `FlowerPotMigration.unpack(2)`: it uses Minecraft's `CompoundTag` (org/betterx/betterend/FlowerPotMigration.java:271)

### `EndBiomeIntegration`: it's an interface

- [ ] `EndBiomeIntegration.addBiomes(0)`: it has no body (org/betterx/betterend/integration/EndBiomeIntegration.java:4)

### `FlamboyantColors`

- [x] `FlamboyantColors.ALL` (org/betterx/betterend/integration/FlamboyantColors.java:11)
- [x] `FlamboyantColors.FlamboyantColors(0)` (org/betterx/betterend/integration/FlamboyantColors.java:30)

### `FlamboyantColors.Color`

- [x] `FlamboyantColors.Color.Color(2)` (org/betterx/betterend/integration/FlamboyantColors.java:33)
- [ ] `FlamboyantColors.Color.argb(0)`: it calls `ColorUtilARGB32` (de.ambertation.wunderlib.utils).color (org/betterx/betterend/integration/FlamboyantColors.java:34)
- [ ] `FlamboyantColors.Color.dye(0)`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/integration/FlamboyantColors.java:38)
- [x] `FlamboyantColors.Color.hex(0)` (org/betterx/betterend/integration/FlamboyantColors.java:33)
- [x] `FlamboyantColors.Color.name(0)` (org/betterx/betterend/integration/FlamboyantColors.java:33)

### `FlamboyantModels`

- [ ] `FlamboyantModels.PETAL_MODEL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/integration/FlamboyantModels.java:13)
- [x] `FlamboyantModels.FlamboyantModels(0)` (org/betterx/betterend/integration/FlamboyantModels.java:16)
- [x] `FlamboyantModels.ensureLoaded(0)` (org/betterx/betterend/integration/FlamboyantModels.java:19)

### `FlamboyantModels.ClientModel`

- [ ] `FlamboyantModels.ClientModel.build(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betterend/integration/FlamboyantModels.java:24)

### `Integrations`

- [ ] `Integrations.init(0)`: it uses BetterEnd.PATCHOULI, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/integration/Integrations.java:11)

### `AlloyingFuelCategory`: it implements `IRecipeCategory` (mezz.jei.api.recipe.category)

- [ ] `AlloyingFuelCategory.TYPE`: it uses `IRecipeType` (mezz.jei.api.recipe.types) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:21)
- [x] `AlloyingFuelCategory.TEXT_X` (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:22)
- [x] `AlloyingFuelCategory.HEIGHT` (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:23)
- [ ] `AlloyingFuelCategory.AlloyingFuelCategory(1)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:27)
- [ ] `AlloyingFuelCategory.burnTimeText(1)`: it uses `NumberFormat` (java.text) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:32)
- [ ] `AlloyingFuelCategory.getRecipeType(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:39)
- [ ] `AlloyingFuelCategory.getTitle(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:43)
- [ ] `AlloyingFuelCategory.getWidth(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:47)
- [ ] `AlloyingFuelCategory.getHeight(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:51)
- [ ] `AlloyingFuelCategory.getIcon(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:55)
- [ ] `AlloyingFuelCategory.setRecipe(3)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:59)
- [ ] `AlloyingFuelCategory.createRecipeExtras(3)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java:63)

### `FuelEntry`: it uses Minecraft's `Item`

- [ ] `FuelEntry.FuelEntry(2)`: its class isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/integration/jei/FuelEntry.java:5)
- [ ] `FuelEntry.item(0)`: its class isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/integration/jei/FuelEntry.java:5)
- [ ] `FuelEntry.burnTimeTicks(0)`: its class isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/integration/jei/FuelEntry.java:5)

### `InfusionCategory`: it implements `IRecipeCategory` (mezz.jei.api.recipe.category)

- [ ] `InfusionCategory.TYPE`: it uses `IRecipeHolderType` (mezz.jei.api.recipe.types) (org/betterx/betterend/integration/jei/InfusionCategory.java:27)
- [ ] `InfusionCategory.BACKGROUND`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/integration/jei/InfusionCategory.java:28)
- [x] `InfusionCategory.ORIENTATIONS` in game_side.rs (org/betterx/betterend/integration/jei/InfusionCategory.java:29)
- [x] `InfusionCategory.LEFT` (org/betterx/betterend/integration/jei/InfusionCategory.java:39)
- [x] `InfusionCategory.TOP` (org/betterx/betterend/integration/jei/InfusionCategory.java:40)
- [x] `InfusionCategory.BOX_SIZE` (org/betterx/betterend/integration/jei/InfusionCategory.java:41)
- [x] `InfusionCategory.RADIUS` (org/betterx/betterend/integration/jei/InfusionCategory.java:42)
- [x] `InfusionCategory.HALF_SLOT` (org/betterx/betterend/integration/jei/InfusionCategory.java:43)
- [x] `InfusionCategory.CENTER_X` (org/betterx/betterend/integration/jei/InfusionCategory.java:44)
- [x] `InfusionCategory.CENTER_Y` (org/betterx/betterend/integration/jei/InfusionCategory.java:45)
- [x] `InfusionCategory.RIGHT` (org/betterx/betterend/integration/jei/InfusionCategory.java:46)
- [x] `InfusionCategory.RING_ITEM_X_OFFSET` (org/betterx/betterend/integration/jei/InfusionCategory.java:47)
- [x] `InfusionCategory.OUTPUT_X` (org/betterx/betterend/integration/jei/InfusionCategory.java:48)
- [x] `InfusionCategory.WIDTH` (org/betterx/betterend/integration/jei/InfusionCategory.java:49)
- [x] `InfusionCategory.HEIGHT` (org/betterx/betterend/integration/jei/InfusionCategory.java:50)
- [ ] `InfusionCategory.InfusionCategory(1)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:54)
- [ ] `InfusionCategory.getRecipeType(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:59)
- [ ] `InfusionCategory.getTitle(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:63)
- [ ] `InfusionCategory.getWidth(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:67)
- [ ] `InfusionCategory.getHeight(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:71)
- [ ] `InfusionCategory.getIcon(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:75)
- [x] `InfusionCategory.catalystX(1)` (org/betterx/betterend/integration/jei/InfusionCategory.java:79)
- [x] `InfusionCategory.catalystY(1)` (org/betterx/betterend/integration/jei/InfusionCategory.java:83)
- [ ] `InfusionCategory.setRecipe(3)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:87)
- [ ] `InfusionCategory.draw(5)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:104)
- [ ] `InfusionCategory.createRecipeExtras(3)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (org/betterx/betterend/integration/jei/InfusionCategory.java:108)

### `JeiPlugin`: it implements `IModPlugin` (mezz.jei.api)

- [ ] `JeiPlugin.UID`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/integration/jei/JeiPlugin.java:25)
- [ ] `JeiPlugin.getPluginUid(0)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (org/betterx/betterend/integration/jei/JeiPlugin.java:27)
- [ ] `JeiPlugin.registerCategories(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (org/betterx/betterend/integration/jei/JeiPlugin.java:31)
- [ ] `JeiPlugin.registerRecipes(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (org/betterx/betterend/integration/jei/JeiPlugin.java:36)
- [ ] `JeiPlugin.registerRecipeCatalysts(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (org/betterx/betterend/integration/jei/JeiPlugin.java:46)

### `BETargetChecker`: it's an interface

- [ ] `BETargetChecker.be_isTarget(0)`: it has no body (org/betterx/betterend/interfaces/BETargetChecker.java:4)
- [ ] `BETargetChecker.be_setTarget(1)`: it has no body (org/betterx/betterend/interfaces/BETargetChecker.java:6)

### `BetterEndElytra`: it's an interface

- [ ] `BetterEndElytra.getModelTexture(0)`: it has no body (org/betterx/betterend/interfaces/BetterEndElytra.java:7)

### `ISlime`: it's an interface

- [ ] `ISlime.be_setSlimeSize(2)`: it has no body (org/betterx/betterend/interfaces/ISlime.java:6)
- [ ] `ISlime.entityRemove(1)`: it has no body (org/betterx/betterend/interfaces/ISlime.java:8)

### `MobEffectApplier`: it's an interface

- [ ] `MobEffectApplier.applyEffect(1)`: it has no body (org/betterx/betterend/interfaces/MobEffectApplier.java:6)

### `MultiModelItem`: it's an interface

- [ ] `MultiModelItem.registerModelPredicate(0)`: it has no body (org/betterx/betterend/interfaces/MultiModelItem.java:8)
- [ ] `MultiModelItem.register(0)`: it calls EndItems.getModItems, which isn't translated (org/betterx/betterend/interfaces/MultiModelItem.java:11)

### `ArmoredElytra`: it extends Minecraft's `Item`

- [ ] `ArmoredElytra.ArmoredElytra(3)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betterend/item/ArmoredElytra.java:16)
- [ ] `ArmoredElytra.getMovementFactor(0)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betterend/item/ArmoredElytra.java:23)
- [ ] `ArmoredElytra.getModelTexture(0)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betterend/item/ArmoredElytra.java:27)

### `CrystaliteArmor`: it extends Minecraft's `Item`

- [x] `CrystaliteArmor.CHEST_DESC` in game_side.rs (org/betterx/betterend/item/CrystaliteArmor.java:16)
- [x] `CrystaliteArmor.ELYTRA_DESC` in game_side.rs (org/betterx/betterend/item/CrystaliteArmor.java:17)
- [x] `CrystaliteArmor.BOOTS_DESC` in game_side.rs (org/betterx/betterend/item/CrystaliteArmor.java:18)
- [x] `CrystaliteArmor.HELMET_DESC` in game_side.rs (org/betterx/betterend/item/CrystaliteArmor.java:19)
- [x] `CrystaliteArmor.LEGGINGS_DESC` in game_side.rs (org/betterx/betterend/item/CrystaliteArmor.java:20)
- [ ] `CrystaliteArmor.SET_SLOTS`: it uses Minecraft's `EquipmentSlot` (org/betterx/betterend/item/CrystaliteArmor.java:21)
- [ ] `CrystaliteArmor.CrystaliteArmor(1)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betterend/item/CrystaliteArmor.java:23)
- [ ] `CrystaliteArmor.hasFullSet(1)`: it uses Minecraft's `LivingEntity` (org/betterx/betterend/item/CrystaliteArmor.java:27)
- [ ] `CrystaliteArmor.applySetEffect(1)`: it uses Minecraft's `LivingEntity` (org/betterx/betterend/item/CrystaliteArmor.java:37)

### `CrystaliteBoots`: it extends `CrystaliteArmor`

- [ ] `CrystaliteBoots.CrystaliteBoots(1)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteBoots.java:17)
- [ ] `CrystaliteBoots.applyEffect(1)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteBoots.java:21)
- [ ] `CrystaliteBoots.appendHoverText(5)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteBoots.java:28)

### `CrystaliteChestplate`: it extends `CrystaliteArmor`

- [ ] `CrystaliteChestplate.CrystaliteChestplate(1)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteChestplate.java:17)
- [ ] `CrystaliteChestplate.applyEffect(1)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteChestplate.java:21)
- [ ] `CrystaliteChestplate.appendHoverText(5)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteChestplate.java:26)

### `CrystaliteElytra`: it extends `ArmoredElytra`

- [ ] `CrystaliteElytra.CrystaliteElytra(3)`: its class isn't translated (it extends `ArmoredElytra`) (org/betterx/betterend/item/CrystaliteElytra.java:18)
- [ ] `CrystaliteElytra.applyEffect(1)`: its class isn't translated (it extends `ArmoredElytra`) (org/betterx/betterend/item/CrystaliteElytra.java:22)
- [ ] `CrystaliteElytra.appendHoverText(5)`: its class isn't translated (it extends `ArmoredElytra`) (org/betterx/betterend/item/CrystaliteElytra.java:27)

### `CrystaliteHelmet`: it extends `CrystaliteArmor`

- [ ] `CrystaliteHelmet.CrystaliteHelmet(1)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteHelmet.java:13)
- [ ] `CrystaliteHelmet.appendHoverText(5)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteHelmet.java:17)

### `CrystaliteLeggings`: it extends `CrystaliteArmor`

- [ ] `CrystaliteLeggings.CrystaliteLeggings(1)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteLeggings.java:13)
- [ ] `CrystaliteLeggings.appendHoverText(5)`: its class isn't translated (it extends `CrystaliteArmor`) (org/betterx/betterend/item/CrystaliteLeggings.java:17)

### `EndArmorItem`: it extends Minecraft's `Item`

- [ ] `EndArmorItem.BASE_BLINDNESS_RESISTANCE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/item/EndArmorItem.java:17)
- [ ] `EndArmorItem.BASE_KNOCKBACK_RESISTANCE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/item/EndArmorItem.java:18)
- [ ] `EndArmorItem.MAX_HEALTH_BOOST`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/item/EndArmorItem.java:19)
- [ ] `EndArmorItem.TOUGHNESS_BOOST`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/item/EndArmorItem.java:20)
- [ ] `EndArmorItem.ARMOR_BOOST`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/item/EndArmorItem.java:21)
- [ ] `EndArmorItem.armorBoostId(1)`: it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/EndArmorItem.java:23)
- [ ] `EndArmorItem.toughnessBoostId(1)`: it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/EndArmorItem.java:27)
- [ ] `EndArmorItem.knockbackBoostId(1)`: it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/EndArmorItem.java:31)
- [ ] `EndArmorItem.createDefaultEndArmorSettings(2)`: it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/EndArmorItem.java:35)
- [ ] `EndArmorItem.createDefaultEndArmorSettings(3)`: it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/EndArmorItem.java:44)
- [ ] `EndArmorItem.EndArmorItem(3)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betterend/item/EndArmorItem.java:53)

### `EndAttribute`: it extends Minecraft's `Attribute`

- [ ] `EndAttribute.EndAttribute(2)`: its class isn't translated (it extends Minecraft's `Attribute`) (org/betterx/betterend/item/EndAttribute.java:6)

### `EndBucketItem`: it extends Minecraft's `MobBucketItem`

- [ ] `EndBucketItem.EndBucketItem(2)`: its class isn't translated (it extends Minecraft's `MobBucketItem`) (org/betterx/betterend/item/EndBucketItem.java:11)

### `GuideBookItem`: it extends Minecraft's `Item`

- [ ] `GuideBookItem.GUIDE_BOOK`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/item/GuideBookItem.java:22)
- [x] `GuideBookItem.register(0)` (org/betterx/betterend/item/GuideBookItem.java:24)
- [ ] `GuideBookItem.GuideBookItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betterend/item/GuideBookItem.java:27)
- [ ] `GuideBookItem.use(3)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betterend/item/GuideBookItem.java:31)
- [ ] `GuideBookItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betterend/item/GuideBookItem.java:35)
- [x] `GuideBookItem.ensureStaticallyLoaded(0)` (org/betterx/betterend/item/GuideBookItem.java:41)

### `AeterniumSet`: it extends `ToolsWithHeadsSet`

- [ ] `AeterniumSet.AeterniumSet(0)`: its class isn't translated (it extends `ToolsWithHeadsSet`) (org/betterx/betterend/item/material/AeterniumSet.java:14)
- [ ] `AeterniumSet.commonToolProperties(1)`: its class isn't translated (it extends `ToolsWithHeadsSet`) (org/betterx/betterend/item/material/AeterniumSet.java:28)
- [ ] `AeterniumSet.hasAutoHeadRecipes(0)`: its class isn't translated (it extends `ToolsWithHeadsSet`) (org/betterx/betterend/item/material/AeterniumSet.java:33)
- [ ] `AeterniumSet.hasAutoSwordHandleRecipe(0)`: its class isn't translated (it extends `ToolsWithHeadsSet`) (org/betterx/betterend/item/material/AeterniumSet.java:38)
- [ ] `AeterniumSet.pickaxe(0)`: its class isn't translated (it extends `ToolsWithHeadsSet`) (org/betterx/betterend/item/material/AeterniumSet.java:43)
- [ ] `AeterniumSet.axe(0)`: its class isn't translated (it extends `ToolsWithHeadsSet`) (org/betterx/betterend/item/material/AeterniumSet.java:48)

### `EndArmorMaterial`

- [ ] `EndArmorMaterial.THALLASIUM`: it uses Minecraft's `Holder` (org/betterx/betterend/item/material/EndArmorMaterial.java:10)
- [ ] `EndArmorMaterial.TERMINITE`: it uses Minecraft's `Holder` (org/betterx/betterend/item/material/EndArmorMaterial.java:20)
- [ ] `EndArmorMaterial.AETERNIUM`: it uses Minecraft's `Holder` (org/betterx/betterend/item/material/EndArmorMaterial.java:30)
- [ ] `EndArmorMaterial.CRYSTALITE`: it uses Minecraft's `Holder` (org/betterx/betterend/item/material/EndArmorMaterial.java:40)
- [ ] `EndArmorMaterial.AETERNIUM_ELYTRA`: it uses Minecraft's `Holder` (org/betterx/betterend/item/material/EndArmorMaterial.java:50)
- [ ] `EndArmorMaterial.CRYSTALITE_ELYTRA`: it uses Minecraft's `Holder` (org/betterx/betterend/item/material/EndArmorMaterial.java:60)

### `EndArmorTier`

- [ ] `EndArmorTier.THALLASIUM`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndArmorTier.java:11)
- [ ] `EndArmorTier.TERMINITE`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndArmorTier.java:15)
- [ ] `EndArmorTier.CRYSTALITE`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndArmorTier.java:19)
- [ ] `EndArmorTier.AETERNIUM`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndArmorTier.java:23)
- [ ] `EndArmorTier.AETERNIUM_ELYTRA`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndArmorTier.java:27)
- [ ] `EndArmorTier.CRYSTALITE_ELYTRA`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndArmorTier.java:30)

### `EndToolMaterial`

- [ ] `EndToolMaterial.THALLASIUM`: it uses Minecraft's `ToolMaterial` (org/betterx/betterend/item/material/EndToolMaterial.java:9)
- [ ] `EndToolMaterial.TERMINITE`: it uses Minecraft's `ToolMaterial` (org/betterx/betterend/item/material/EndToolMaterial.java:12)
- [ ] `EndToolMaterial.AETERNIUM`: it uses Minecraft's `ToolMaterial` (org/betterx/betterend/item/material/EndToolMaterial.java:15)

### `EndToolTier`

- [ ] `EndToolTier.THALLASIUM`: it uses `ToolTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndToolTier.java:11)
- [ ] `EndToolTier.TERMINITE`: it uses `ToolTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndToolTier.java:17)
- [ ] `EndToolTier.AETERNIUM`: it uses `ToolTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betterend/item/material/EndToolTier.java:23)

### `ToolsWithHeadsSet`: it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)

- [ ] `ToolsWithHeadsSet.ToolsWithHeadsSet(9)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betterend/item/material/ToolsWithHeadsSet.java:40)
- [ ] `ToolsWithHeadsSet.buildArmor(0)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betterend/item/material/ToolsWithHeadsSet.java:141)
- [ ] `ToolsWithHeadsSet.registerRecipes(1)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betterend/item/material/ToolsWithHeadsSet.java:148)
- [ ] `ToolsWithHeadsSet.hasAutoHeadRecipes(0)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betterend/item/material/ToolsWithHeadsSet.java:190)
- [ ] `ToolsWithHeadsSet.hasAutoSwordHandleRecipe(0)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betterend/item/material/ToolsWithHeadsSet.java:194)

### `CrystaliteArmorParts`: it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)

- [ ] `CrystaliteArmorParts.HELMET`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:20)
- [ ] `CrystaliteArmorParts.CHESTPLATE`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:21)
- [ ] `CrystaliteArmorParts.CHESTPLATE_SLIM`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:22)
- [ ] `CrystaliteArmorParts.LEGGINGS`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:23)
- [ ] `CrystaliteArmorParts.BOOTS`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:24)
- [ ] `CrystaliteArmorParts.CrystaliteArmorParts(1)`: its class isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:27)
- [ ] `CrystaliteArmorParts.forSlot(2)`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:31)
- [ ] `CrystaliteArmorParts.forEachVisible(3)`: it uses Minecraft's `HumanoidModel` (org/betterx/betterend/item/model/CrystaliteArmorParts.java:48)
- [ ] `CrystaliteArmorParts.attach(2)`: it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:68)
- [ ] `CrystaliteArmorParts.detach(1)`: it uses Minecraft's `ModelPart` (org/betterx/betterend/item/model/CrystaliteArmorParts.java:72)
- [ ] `CrystaliteArmorParts.helmet(1)`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:79)
- [ ] `CrystaliteArmorParts.chestplate(1)`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:83)
- [ ] `CrystaliteArmorParts.leggings(1)`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:87)
- [ ] `CrystaliteArmorParts.boots(1)`: it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:91)

### `CrystaliteArmorParts.Attachment`: it uses Minecraft's `HumanoidModel`

- [ ] `CrystaliteArmorParts.Attachment.Attachment(3)`: its class isn't translated (it uses Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:95)
- [ ] `CrystaliteArmorParts.Attachment.bone(0)`: its class isn't translated (it uses Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:95)
- [ ] `CrystaliteArmorParts.Attachment.insideHat(0)`: its class isn't translated (it uses Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:95)
- [ ] `CrystaliteArmorParts.Attachment.crystal(0)`: its class isn't translated (it uses Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteArmorParts.java:95)

### `CrystaliteArmorRenderer`: it implements Fabric's `ArmorRenderer`

- [ ] `CrystaliteArmorRenderer.FIRST_LAYER`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/item/model/CrystaliteArmorRenderer.java:21)
- [ ] `CrystaliteArmorRenderer.SECOND_LAYER`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/item/model/CrystaliteArmorRenderer.java:22)
- [ ] `CrystaliteArmorRenderer.INSTANCE`: it uses CrystaliteArmorRenderer, which isn't translated (it implements Fabric's `ArmorRenderer`) (org/betterx/betterend/item/model/CrystaliteArmorRenderer.java:23)
- [ ] `CrystaliteArmorRenderer.register(0)`: it uses CrystaliteArmorRenderer.INSTANCE, which isn't translated (it uses CrystaliteArmorRenderer, which isn't translated (it implements Fabric's `ArmorRenderer`)) (org/betterx/betterend/item/model/CrystaliteArmorRenderer.java:25)
- [ ] `CrystaliteArmorRenderer.getTextureForSlot(1)`: its class isn't translated (it implements Fabric's `ArmorRenderer`) (org/betterx/betterend/item/model/CrystaliteArmorRenderer.java:40)
- [ ] `CrystaliteArmorRenderer.render(7)`: its class isn't translated (it implements Fabric's `ArmorRenderer`) (org/betterx/betterend/item/model/CrystaliteArmorRenderer.java:44)
- [ ] `CrystaliteArmorRenderer.submitLayer(6)`: it uses Minecraft's `SubmitNodeCollector` (org/betterx/betterend/item/model/CrystaliteArmorRenderer.java:60)

### `CrystaliteBootsModel`: it extends Minecraft's `HumanoidModel`

- [ ] `CrystaliteBootsModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/item/model/CrystaliteBootsModel.java:20)
- [ ] `CrystaliteBootsModel.createModel(1)`: it uses CrystaliteBootsModel, which isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteBootsModel.java:41)
- [ ] `CrystaliteBootsModel.CrystaliteBootsModel(1)`: its class isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteBootsModel.java:47)

### `CrystaliteChestplateModel`: it extends Minecraft's `HumanoidModel`

- [ ] `CrystaliteChestplateModel.getRegularTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/item/model/CrystaliteChestplateModel.java:24)
- [ ] `CrystaliteChestplateModel.getThinTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/item/model/CrystaliteChestplateModel.java:28)
- [ ] `CrystaliteChestplateModel.getTexturedModelData(2)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/item/model/CrystaliteChestplateModel.java:32)
- [ ] `CrystaliteChestplateModel.createRegularModel(1)`: it uses CrystaliteChestplateModel, which isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteChestplateModel.java:74)
- [ ] `CrystaliteChestplateModel.createThinModel(1)`: it uses CrystaliteChestplateModel, which isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteChestplateModel.java:80)
- [ ] `CrystaliteChestplateModel.CrystaliteChestplateModel(2)`: its class isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteChestplateModel.java:86)
- [ ] `CrystaliteChestplateModel.translateToHand(3)`: its class isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteChestplateModel.java:94)

### `CrystaliteHelmetModel`: it extends Minecraft's `HumanoidModel`

- [ ] `CrystaliteHelmetModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/item/model/CrystaliteHelmetModel.java:22)
- [ ] `CrystaliteHelmetModel.createModel(1)`: it uses CrystaliteHelmetModel, which isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteHelmetModel.java:39)
- [ ] `CrystaliteHelmetModel.CrystaliteHelmetModel(1)`: its class isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteHelmetModel.java:45)

### `CrystaliteLeggingsModel`: it extends Minecraft's `HumanoidModel`

- [ ] `CrystaliteLeggingsModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:21)
- [ ] `CrystaliteLeggingsModel.createModel(1)`: it uses CrystaliteLeggingsModel, which isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:42)
- [ ] `CrystaliteLeggingsModel.CrystaliteLeggingsModel(1)`: its class isn't translated (it extends Minecraft's `HumanoidModel`) (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:48)
- [ ] `AbstractSoundInstanceAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:8)
- [ ] `ArmorStandRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:12)
- [ ] `BiomeColorsMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:21)
- [ ] `BlockStateModelLoaderMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:20)
- [ ] `CapeLayerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:15)
- [ ] `GameRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:16)
- [ ] `HumanoidMobRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:12)
- [ ] `ItemStackMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:17)
- [ ] `LevelRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:14)
- [ ] `MinecraftClientMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:23)
- [ ] `MusicTrackerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:25)
- [ ] `PlayerRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:12)
- [ ] `ChorusFlowerBlockMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:31)
- [ ] `ChorusPlantBlockMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:25)
- [ ] `ChorusPlantFeatureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:21)
- [ ] `CraftingMenuMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:15)
- [ ] `EndCityFeatureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:14)
- [ ] `EndDragonFightMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:26)
- [ ] `EnderManMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:16)
- [ ] `EndPlatformFeatureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:13)
- [ ] `EndPodiumFeatureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:27)
- [ ] `EndPortalBlockMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:15)
- [ ] `EndSpikeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:16)
- [ ] `LevelMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:14)
- [ ] `LivingEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:30)
- [ ] `NoiseBasedChunkGeneratorAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:10)
- [ ] `NoiseBasedChunkGeneratorHeightMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:24)
- [ ] `NoiseBasedChunkGeneratorTerrainMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:16)
- [ ] `NoiseGeneratorSettingsMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:8)
- [ ] `PlayerAdvancementsMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:15)
- [ ] `PlayerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:27)
- [ ] `EntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:8)
- [ ] `ServerLevelMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:34)
- [ ] `ServerPlayerGameModeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:27)
- [ ] `SlimeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:13)
- [ ] `SpikeFeatureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:35)
- [ ] `WorldGenRegionMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java:15)

### `RitualUpdate`: it uses Minecraft's `BlockPos`

- [x] `RitualUpdate.ACTIVE_FLAG` (org/betterx/betterend/network/RitualUpdate.java:15)
- [x] `RitualUpdate.WILL_ACTIVATE_FLAG` (org/betterx/betterend/network/RitualUpdate.java:16)
- [ ] `RitualUpdate.AXIS_CODEC`: it uses Minecraft's `StreamCodec` (org/betterx/betterend/network/RitualUpdate.java:17)
- [ ] `RitualUpdate.KEY`: it uses `ClientBoundMessage` (de.ambertation.wunderlib.network) (org/betterx/betterend/network/RitualUpdate.java:18)
- [ ] `RitualUpdate.RitualUpdate(3)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/network/RitualUpdate.java:14)
- [ ] `RitualUpdate.of(1)`: it uses RitualUpdate, which isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/network/RitualUpdate.java:25)
- [ ] `RitualUpdate.isActive(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/network/RitualUpdate.java:38)
- [ ] `RitualUpdate.willActivate(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/network/RitualUpdate.java:42)
- [ ] `RitualUpdate.send(2)`: it uses Minecraft's `ServerLevel` (org/betterx/betterend/network/RitualUpdate.java:46)
- [ ] `RitualUpdate.center(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/network/RitualUpdate.java:14)
- [ ] `RitualUpdate.axis(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/network/RitualUpdate.java:14)
- [ ] `RitualUpdate.flags(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/network/RitualUpdate.java:14)

### `OpenSimplexNoise`

- [x] `OpenSimplexNoise.STRETCH_CONSTANT_2D` (org/betterx/betterend/noise/OpenSimplexNoise.java:4)
- [x] `OpenSimplexNoise.SQUISH_CONSTANT_2D` (org/betterx/betterend/noise/OpenSimplexNoise.java:5)
- [x] `OpenSimplexNoise.STRETCH_CONSTANT_3D` (org/betterx/betterend/noise/OpenSimplexNoise.java:6)
- [x] `OpenSimplexNoise.SQUISH_CONSTANT_3D` (org/betterx/betterend/noise/OpenSimplexNoise.java:7)
- [x] `OpenSimplexNoise.STRETCH_CONSTANT_4D` (org/betterx/betterend/noise/OpenSimplexNoise.java:8)
- [x] `OpenSimplexNoise.SQUISH_CONSTANT_4D` (org/betterx/betterend/noise/OpenSimplexNoise.java:9)
- [x] `OpenSimplexNoise.NORM_CONSTANT_2D` (org/betterx/betterend/noise/OpenSimplexNoise.java:10)
- [x] `OpenSimplexNoise.NORM_CONSTANT_3D` (org/betterx/betterend/noise/OpenSimplexNoise.java:11)
- [x] `OpenSimplexNoise.NORM_CONSTANT_4D` (org/betterx/betterend/noise/OpenSimplexNoise.java:12)
- [x] `OpenSimplexNoise.DEFAULT_SEED` (org/betterx/betterend/noise/OpenSimplexNoise.java:13)
- [x] `OpenSimplexNoise.gradients2D` (org/betterx/betterend/noise/OpenSimplexNoise.java:16)
- [x] `OpenSimplexNoise.gradients3D` (org/betterx/betterend/noise/OpenSimplexNoise.java:17)
- [x] `OpenSimplexNoise.gradients4D` (org/betterx/betterend/noise/OpenSimplexNoise.java:91)
- [ ] `OpenSimplexNoise.OpenSimplexNoise(0)`: its constructors are overloaded with the same number of parameters (org/betterx/betterend/noise/OpenSimplexNoise.java:350)
- [ ] `OpenSimplexNoise.OpenSimplexNoise(1)`: its constructors are overloaded with the same number of parameters (org/betterx/betterend/noise/OpenSimplexNoise.java:354)
- [ ] `OpenSimplexNoise.OpenSimplexNoise(1)`: its constructors are overloaded with the same number of parameters (org/betterx/betterend/noise/OpenSimplexNoise.java:363)
- [x] `OpenSimplexNoise.eval(2)` (org/betterx/betterend/noise/OpenSimplexNoise.java:390)
- [x] `OpenSimplexNoise.eval(3)` (org/betterx/betterend/noise/OpenSimplexNoise.java:483)
- [x] `OpenSimplexNoise.eval(4)` (org/betterx/betterend/noise/OpenSimplexNoise.java:996)
- [x] `OpenSimplexNoise.extrapolate(4)` (org/betterx/betterend/noise/OpenSimplexNoise.java:2274)
- [x] `OpenSimplexNoise.extrapolate(6)` (org/betterx/betterend/noise/OpenSimplexNoise.java:2279)
- [x] `OpenSimplexNoise.extrapolate(8)` (org/betterx/betterend/noise/OpenSimplexNoise.java:2284)
- [x] `OpenSimplexNoise.fastFloor(1)` (org/betterx/betterend/noise/OpenSimplexNoise.java:2289)

### `FireflyParticle`: it extends Minecraft's `SimpleAnimatedParticle`

- [ ] `FireflyParticle.FireflyParticle(8)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/FireflyParticle.java:24)
- [ ] `FireflyParticle.tick(0)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/FireflyParticle.java:40)

### `FireflyParticle.FireflyParticleFactory`: it implements Minecraft's `ParticleProvider`

- [ ] `FireflyParticle.FireflyParticleFactory.FireflyParticleFactory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/FireflyParticle.java:68)
- [ ] `FireflyParticle.FireflyParticleFactory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/FireflyParticle.java:72)

### `InfusionMistParticle`: it extends Minecraft's `SingleQuadParticle`

- [x] `InfusionMistParticle.SINK_SPEED_MIN` (org/betterx/betterend/particle/InfusionMistParticle.java:19)
- [x] `InfusionMistParticle.SINK_SPEED_MAX` (org/betterx/betterend/particle/InfusionMistParticle.java:20)
- [x] `InfusionMistParticle.SWAY_STRENGTH` (org/betterx/betterend/particle/InfusionMistParticle.java:21)
- [x] `InfusionMistParticle.SWAY_SPEED_MIN` (org/betterx/betterend/particle/InfusionMistParticle.java:22)
- [x] `InfusionMistParticle.SWAY_SPEED_MAX` (org/betterx/betterend/particle/InfusionMistParticle.java:23)
- [x] `InfusionMistParticle.MAX_FALL_DRIFT` (org/betterx/betterend/particle/InfusionMistParticle.java:24)
- [x] `InfusionMistParticle.GROUND_ACCELERATION` (org/betterx/betterend/particle/InfusionMistParticle.java:25)
- [x] `InfusionMistParticle.MAX_GROUND_SPEED` (org/betterx/betterend/particle/InfusionMistParticle.java:26)
- [x] `InfusionMistParticle.GROUND_GROWTH` (org/betterx/betterend/particle/InfusionMistParticle.java:27)
- [x] `InfusionMistParticle.MAX_QUAD_SIZE` (org/betterx/betterend/particle/InfusionMistParticle.java:28)
- [x] `InfusionMistParticle.MAX_ALPHA` (org/betterx/betterend/particle/InfusionMistParticle.java:29)
- [x] `InfusionMistParticle.FADE_TICKS` (org/betterx/betterend/particle/InfusionMistParticle.java:30)
- [x] `InfusionMistParticle.DEFAULT_RGB` (org/betterx/betterend/particle/InfusionMistParticle.java:31)
- [x] `InfusionMistParticle.NORTH_RGB` (org/betterx/betterend/particle/InfusionMistParticle.java:32)
- [ ] `InfusionMistParticle.InfusionMistParticle(9)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/InfusionMistParticle.java:42)
- [ ] `InfusionMistParticle.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/InfusionMistParticle.java:71)
- [ ] `InfusionMistParticle.getLightCoords(1)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/InfusionMistParticle.java:99)
- [ ] `InfusionMistParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/InfusionMistParticle.java:103)

### `InfusionMistParticle.FactoryInfusionMist`: it implements Minecraft's `ParticleProvider`

- [ ] `InfusionMistParticle.FactoryInfusionMist.FactoryInfusionMist(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/InfusionMistParticle.java:112)
- [ ] `InfusionMistParticle.FactoryInfusionMist.FactoryInfusionMist(2)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/InfusionMistParticle.java:116)
- [ ] `InfusionMistParticle.FactoryInfusionMist.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/InfusionMistParticle.java:121)

### `InfusionParticle`: it extends Minecraft's `SingleQuadParticle`

- [ ] `InfusionParticle.InfusionParticle(9)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/InfusionParticle.java:16)
- [ ] `InfusionParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/InfusionParticle.java:31)
- [ ] `InfusionParticle.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/InfusionParticle.java:35)

### `InfusionParticle.InfusionFactory`: it implements Minecraft's `ParticleProvider`

- [ ] `InfusionParticle.InfusionFactory.InfusionFactory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/InfusionParticle.java:54)
- [ ] `InfusionParticle.InfusionFactory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/InfusionParticle.java:58)

### `InfusionParticleType`: it extends Minecraft's `ParticleType`

- [ ] `InfusionParticleType.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/particle/InfusionParticleType.java:19)
- [ ] `InfusionParticleType.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (org/betterx/betterend/particle/InfusionParticleType.java:22)
- [ ] `InfusionParticleType.InfusionParticleType(2)`: its class isn't translated (it extends Minecraft's `ParticleType`) (org/betterx/betterend/particle/InfusionParticleType.java:27)
- [ ] `InfusionParticleType.InfusionParticleType(1)`: its class isn't translated (it extends Minecraft's `ParticleType`) (org/betterx/betterend/particle/InfusionParticleType.java:33)
- [ ] `InfusionParticleType.getPalette(0)`: its class isn't translated (it extends Minecraft's `ParticleType`) (org/betterx/betterend/particle/InfusionParticleType.java:37)
- [ ] `InfusionParticleType.getType(0)`: its class isn't translated (it extends Minecraft's `ParticleType`) (org/betterx/betterend/particle/InfusionParticleType.java:43)
- [ ] `InfusionParticleType.codec(0)`: its class isn't translated (it extends Minecraft's `ParticleType`) (org/betterx/betterend/particle/InfusionParticleType.java:48)
- [ ] `InfusionParticleType.streamCodec(0)`: its class isn't translated (it extends Minecraft's `ParticleType`) (org/betterx/betterend/particle/InfusionParticleType.java:53)

### `ParticleBlackSpore`: it extends Minecraft's `SimpleAnimatedParticle`

- [ ] `ParticleBlackSpore.ParticleBlackSpore(8)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/ParticleBlackSpore.java:24)
- [ ] `ParticleBlackSpore.tick(0)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/ParticleBlackSpore.java:39)

### `ParticleBlackSpore.FactoryBlackSpore`: it implements Minecraft's `ParticleProvider`

- [ ] `ParticleBlackSpore.FactoryBlackSpore.FactoryBlackSpore(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleBlackSpore.java:71)
- [ ] `ParticleBlackSpore.FactoryBlackSpore.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleBlackSpore.java:75)

### `ParticleGeyser`: it extends Minecraft's `SingleQuadParticle`

- [ ] `ParticleGeyser.ParticleGeyser(8)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleGeyser.java:22)
- [ ] `ParticleGeyser.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleGeyser.java:31)
- [ ] `ParticleGeyser.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleGeyser.java:56)

### `ParticleGeyser.FactoryGeyser`: it implements Minecraft's `ParticleProvider`

- [ ] `ParticleGeyser.FactoryGeyser.FactoryGeyser(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleGeyser.java:64)
- [ ] `ParticleGeyser.FactoryGeyser.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleGeyser.java:68)

### `ParticleGlowingSphere`: it extends Minecraft's `SimpleAnimatedParticle`

- [ ] `ParticleGlowingSphere.ParticleGlowingSphere(8)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/ParticleGlowingSphere.java:25)
- [ ] `ParticleGlowingSphere.tick(0)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/ParticleGlowingSphere.java:40)

### `ParticleGlowingSphere.FactoryGlowingSphere`: it implements Minecraft's `ParticleProvider`

- [ ] `ParticleGlowingSphere.FactoryGlowingSphere.FactoryGlowingSphere(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleGlowingSphere.java:63)
- [ ] `ParticleGlowingSphere.FactoryGlowingSphere.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleGlowingSphere.java:67)

### `ParticleJungleSpore`: it extends Minecraft's `SimpleAnimatedParticle`

- [ ] `ParticleJungleSpore.ParticleJungleSpore(8)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/ParticleJungleSpore.java:16)
- [ ] `ParticleJungleSpore.tick(0)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/ParticleJungleSpore.java:26)

### `ParticleJungleSpore.FactoryJungleSpore`: it implements Minecraft's `ParticleProvider`

- [ ] `ParticleJungleSpore.FactoryJungleSpore.FactoryJungleSpore(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleJungleSpore.java:56)
- [ ] `ParticleJungleSpore.FactoryJungleSpore.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleJungleSpore.java:60)

### `ParticleSnowflake`: it extends Minecraft's `SingleQuadParticle`

- [ ] `ParticleSnowflake.ParticleSnowflake(8)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleSnowflake.java:26)
- [ ] `ParticleSnowflake.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleSnowflake.java:39)
- [ ] `ParticleSnowflake.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleSnowflake.java:72)

### `ParticleSnowflake.FactorySnowflake`: it implements Minecraft's `ParticleProvider`

- [ ] `ParticleSnowflake.FactorySnowflake.FactorySnowflake(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleSnowflake.java:80)
- [ ] `ParticleSnowflake.FactorySnowflake.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleSnowflake.java:84)

### `ParticleSulphur`: it extends Minecraft's `SingleQuadParticle`

- [ ] `ParticleSulphur.ParticleSulphur(8)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleSulphur.java:26)
- [ ] `ParticleSulphur.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleSulphur.java:40)
- [ ] `ParticleSulphur.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleSulphur.java:73)

### `ParticleSulphur.FactorySulphur`: it implements Minecraft's `ParticleProvider`

- [ ] `ParticleSulphur.FactorySulphur.FactorySulphur(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleSulphur.java:81)
- [ ] `ParticleSulphur.FactorySulphur.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleSulphur.java:85)

### `ParticleTenaneaPetal`: it extends Minecraft's `SingleQuadParticle`

- [ ] `ParticleTenaneaPetal.provider`: it uses Minecraft's `BlockTintSource` (org/betterx/betterend/particle/ParticleTenaneaPetal.java:22)
- [ ] `ParticleTenaneaPetal.ParticleTenaneaPetal(8)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleTenaneaPetal.java:30)
- [ ] `ParticleTenaneaPetal.getLightCoords(1)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleTenaneaPetal.java:51)
- [ ] `ParticleTenaneaPetal.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleTenaneaPetal.java:55)
- [ ] `ParticleTenaneaPetal.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (org/betterx/betterend/particle/ParticleTenaneaPetal.java:83)

### `ParticleTenaneaPetal.FactoryTenaneaPetal`: it implements Minecraft's `ParticleProvider`

- [ ] `ParticleTenaneaPetal.FactoryTenaneaPetal.FactoryTenaneaPetal(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleTenaneaPetal.java:91)
- [ ] `ParticleTenaneaPetal.FactoryTenaneaPetal.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/ParticleTenaneaPetal.java:95)

### `PaticlePortalSphere`: it extends Minecraft's `SimpleAnimatedParticle`

- [ ] `PaticlePortalSphere.PaticlePortalSphere(5)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/PaticlePortalSphere.java:24)
- [ ] `PaticlePortalSphere.tick(0)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/PaticlePortalSphere.java:40)

### `PaticlePortalSphere.FactoryPortalSphere`: it implements Minecraft's `ParticleProvider`

- [ ] `PaticlePortalSphere.FactoryPortalSphere.FactoryPortalSphere(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/PaticlePortalSphere.java:63)
- [ ] `PaticlePortalSphere.FactoryPortalSphere.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/PaticlePortalSphere.java:67)

### `SmaragdantParticle`: it extends Minecraft's `SimpleAnimatedParticle`

- [ ] `SmaragdantParticle.SmaragdantParticle(8)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/SmaragdantParticle.java:24)
- [ ] `SmaragdantParticle.tick(0)`: its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`) (org/betterx/betterend/particle/SmaragdantParticle.java:39)

### `SmaragdantParticle.SmaragdantParticleFactory`: it implements Minecraft's `ParticleProvider`

- [ ] `SmaragdantParticle.SmaragdantParticleFactory.SmaragdantParticleFactory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/SmaragdantParticle.java:71)
- [ ] `SmaragdantParticle.SmaragdantParticleFactory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betterend/particle/SmaragdantParticle.java:75)

### `Patcher`

- [ ] `Patcher.register(0)`: a method reference (org/betterx/betterend/Patcher.java:6)

### `Patcher_001`: it extends `Patch` (org.betterx.bclib.api.v2.datafixer)

- [ ] `Patcher_001.Patcher_001(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betterend/Patcher_001.java:10)
- [ ] `Patcher_001.getIDReplacements(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betterend/Patcher_001.java:14)
- [ ] `Patcher_001.getChunkPatcher(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betterend/Patcher_001.java:61)

### `PortalBuilder`: it uses Minecraft's `ServerLevel`

- [ ] `PortalBuilder.FRAME_POSITIONS`: it uses `Point` (java.awt) (org/betterx/betterend/portal/PortalBuilder.java:38)
- [ ] `PortalBuilder.PORTAL_POSITIONS`: it uses `Point` (java.awt) (org/betterx/betterend/portal/PortalBuilder.java:51)
- [ ] `PortalBuilder.BASE_POSITIONS`: it uses `Point` (java.awt) (org/betterx/betterend/portal/PortalBuilder.java:68)
- [ ] `PortalBuilder.BASE`: it uses Minecraft's `Block` (org/betterx/betterend/portal/PortalBuilder.java:71)
- [ ] `PortalBuilder.FRAME`: it uses Minecraft's `Block` (org/betterx/betterend/portal/PortalBuilder.java:72)
- [ ] `PortalBuilder.PORTAL`: it uses Minecraft's `Block` (org/betterx/betterend/portal/PortalBuilder.java:73)
- [x] `PortalBuilder.SPIRAL_SEARCH_RADIUS` (org/betterx/betterend/portal/PortalBuilder.java:74)
- [x] `PortalBuilder.GENERATE_SEARCH_RADIUS` (org/betterx/betterend/portal/PortalBuilder.java:75)
- [x] `PortalBuilder.PROBE_FOOTPRINT` (org/betterx/betterend/portal/PortalBuilder.java:76)
- [ ] `PortalBuilder.PortalBuilder(2)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:80)
- [ ] `PortalBuilder.generatePortal(4)`: it uses Minecraft's `Level` (org/betterx/betterend/portal/PortalBuilder.java:85)
- [ ] `PortalBuilder.generateBase(3)`: it uses Minecraft's `Level` (org/betterx/betterend/portal/PortalBuilder.java:105)
- [ ] `PortalBuilder.checkIsAreaValid(3)`: it uses Minecraft's `Level` (org/betterx/betterend/portal/PortalBuilder.java:120)
- [ ] `PortalBuilder.isBaseValid(3)`: it uses Minecraft's `Level` (org/betterx/betterend/portal/PortalBuilder.java:128)
- [ ] `PortalBuilder.validBlock(3)`: it uses Minecraft's `Level` (org/betterx/betterend/portal/PortalBuilder.java:151)
- [ ] `PortalBuilder.checkArea(3)`: it uses Minecraft's `Level` (org/betterx/betterend/portal/PortalBuilder.java:155)
- [ ] `PortalBuilder.isStateInvalid(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/portal/PortalBuilder.java:177)
- [ ] `PortalBuilder.findPortalAround(2)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:181)
- [ ] `PortalBuilder.getPortalAt(1)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:191)
- [ ] `PortalBuilder.getStartingPos(4)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/portal/PortalBuilder.java:201)
- [ ] `PortalBuilder.createPortal(3)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:206)
- [ ] `PortalBuilder.buildPortal(3)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:287)
- [ ] `PortalBuilder.getPortalID(1)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:292)
- [ ] `PortalBuilder.isColumnLoaded(6)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:297)
- [ ] `PortalBuilder.canPortalReplaceBlock(1)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:312)
- [ ] `PortalBuilder.canHostFrame(4)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/portal/PortalBuilder.java:317)

### `PortalBuilder.FoundPortalRect`: it uses Minecraft's `FoundRectangle`

- [ ] `PortalBuilder.FoundPortalRect.FoundPortalRect(2)`: its class isn't translated (it uses Minecraft's `FoundRectangle`) (org/betterx/betterend/portal/PortalBuilder.java:338)
- [ ] `PortalBuilder.FoundPortalRect.rect(0)`: its class isn't translated (it uses Minecraft's `FoundRectangle`) (org/betterx/betterend/portal/PortalBuilder.java:338)
- [ ] `PortalBuilder.FoundPortalRect.pos(0)`: its class isn't translated (it uses Minecraft's `FoundRectangle`) (org/betterx/betterend/portal/PortalBuilder.java:338)

### `TravelerState`


### `TravelingEntity`: it's an interface


### `InfusionRecipe`: it implements Minecraft's `Recipe`

- [x] `InfusionRecipe.GROUP` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:49)
- [ ] `InfusionRecipe.INFUSION_CATEGORY`: it uses Minecraft's `RecipeBookCategory` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:50)
- [ ] `InfusionRecipe.TYPE`: it uses Minecraft's `RecipeType` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:51)
- [ ] `InfusionRecipe.SERIALIZER`: it uses Minecraft's `RecipeSerializer` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:52)
- [ ] `InfusionRecipe.InfusionRecipe(7)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:64)
- [ ] `InfusionRecipe.create(2)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:74)
- [ ] `InfusionRecipe.create(2)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:78)
- [ ] `InfusionRecipe.create(2)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:82)
- [ ] `InfusionRecipe.create(2)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:86)
- [ ] `InfusionRecipe.create(4)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:90)
- [ ] `InfusionRecipe.create(4)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:94)
- [ ] `InfusionRecipe.enchantmentPatch(3)`: it uses Minecraft's `DataComponentPatch` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:100)
- [ ] `InfusionRecipe.getInfusionTime(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:106)
- [ ] `InfusionRecipe.getInput(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:110)
- [ ] `InfusionRecipe.getCatalysts(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:114)
- [ ] `InfusionRecipe.matches(2)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:118)
- [ ] `InfusionRecipe.assemble(1)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:131)
- [ ] `InfusionRecipe.showNotification(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:141)
- [ ] `InfusionRecipe.group(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:145)
- [ ] `InfusionRecipe.getSerializer(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:150)
- [ ] `InfusionRecipe.getType(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:155)
- [ ] `InfusionRecipe.placementInfo(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:160)
- [ ] `InfusionRecipe.recipeBookCategory(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:178)
- [x] `InfusionRecipe.register(0)` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:183)

### `InfusionRecipe.Builder`: it's an interface

- [ ] `InfusionRecipe.Builder.group(1)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:187)
- [ ] `InfusionRecipe.Builder.setPrimaryInput(1)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:189)
- [ ] `InfusionRecipe.Builder.setPrimaryInput(1)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:191)
- [ ] `InfusionRecipe.Builder.setPrimaryInputAndUnlock(1)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:193)
- [ ] `InfusionRecipe.Builder.setPrimaryInputAndUnlock(1)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:195)
- [ ] `InfusionRecipe.Builder.setTime(1)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:197)
- [ ] `InfusionRecipe.Builder.addCatalyst(2)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:199)
- [ ] `InfusionRecipe.Builder.addCatalyst(2)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:201)
- [ ] `InfusionRecipe.Builder.addCatalyst(2)`: it has no body (org/betterx/betterend/recipe/builders/InfusionRecipe.java:203)

### `InfusionRecipe.BuilderImpl`: it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)

- [ ] `InfusionRecipe.BuilderImpl.BuilderImpl(2)`: its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:211)
- [ ] `InfusionRecipe.BuilderImpl.BuilderImpl(2)`: its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:217)
- [ ] `InfusionRecipe.BuilderImpl.setTime(1)`: its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:224)
- [ ] `InfusionRecipe.BuilderImpl.addCatalyst(2)`: its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:230)
- [ ] `InfusionRecipe.BuilderImpl.addCatalyst(2)`: its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:236)
- [ ] `InfusionRecipe.BuilderImpl.addCatalyst(2)`: its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:242)
- [ ] `InfusionRecipe.BuilderImpl.validate(0)`: its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:248)
- [ ] `InfusionRecipe.BuilderImpl.createRecipe(1)`: its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:255)

### `InfusionRecipe.CatalystSlot`: its constants carry values

- [ ] `InfusionRecipe.CatalystSlot.CODEC`: it uses Minecraft's `Codec` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:278)
- [ ] `InfusionRecipe.CatalystSlot.CatalystSlot(2)`: its class isn't translated (its constants carry values) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:282)
- [ ] `InfusionRecipe.CatalystSlot.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:287)

### `InfusionRecipe.Serializer`

- [ ] `InfusionRecipe.Serializer.CODEC_CATALYSTS`: it uses Minecraft's `MapCodec` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:294)
- [ ] `InfusionRecipe.Serializer.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:335)
- [ ] `InfusionRecipe.Serializer.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:350)
- [ ] `InfusionRecipe.Serializer.fromNetwork(1)`: it uses InfusionRecipe, which isn't translated (it implements Minecraft's `Recipe`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:354)
- [ ] `InfusionRecipe.Serializer.toNetwork(2)`: it uses Minecraft's `RegistryFriendlyByteBuf` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:371)

### `InfusionRecipe.Serializer.Result`: it uses Minecraft's `Item`

- [ ] `InfusionRecipe.Serializer.Result.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/recipe/builders/InfusionRecipe.java:385)
- [ ] `InfusionRecipe.Serializer.Result.Result(3)`: its class isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:384)
- [ ] `InfusionRecipe.Serializer.Result.item(0)`: its class isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:384)
- [ ] `InfusionRecipe.Serializer.Result.count(0)`: its class isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:384)
- [ ] `InfusionRecipe.Serializer.Result.patch(0)`: its class isn't translated (it uses Minecraft's `Item`) (org/betterx/betterend/recipe/builders/InfusionRecipe.java:384)

### `EndBlockItemFactories`

- [ ] `EndBlockItemFactories.FIRE_RESISTANT_ANVIL`: it uses `BlockItemFactory` (de.ambertation.wover.block.api.BlockDefinition) (org/betterx/betterend/registry/block/EndBlockItemFactories.java:11)
- [ ] `EndBlockItemFactories.FIRE_RESISTANT`: it uses `BlockItemFactory` (de.ambertation.wover.block.api.BlockDefinition) (org/betterx/betterend/registry/block/EndBlockItemFactories.java:14)
- [ ] `EndBlockItemFactories.PLACE_ON_WATER`: it uses `BlockItemFactory` (de.ambertation.wover.block.api.BlockDefinition) (org/betterx/betterend/registry/block/EndBlockItemFactories.java:17)
- [x] `EndBlockItemFactories.EndBlockItemFactories(0)` (org/betterx/betterend/registry/block/EndBlockItemFactories.java:21)
- [x] `EndBlockItemFactories.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndBlockItemFactories.java:24)

### `EndCropBlocks`

- [ ] `EndCropBlocks.SHADOW_BERRY`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCropBlocks.java:11)
- [ ] `EndCropBlocks.BLOSSOM_BERRY`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCropBlocks.java:12)
- [ ] `EndCropBlocks.AMBER_ROOT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCropBlocks.java:13)
- [ ] `EndCropBlocks.CHORUS_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCropBlocks.java:14)
- [ ] `EndCropBlocks.CAVE_PUMPKIN_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCropBlocks.java:15)
- [ ] `EndCropBlocks.CAVE_PUMPKIN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCropBlocks.java:16)
- [x] `EndCropBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndCropBlocks.java:18)

### `EndCrystalBlocks`

- [ ] `EndCrystalBlocks.AURORA_PALETTE`: it uses `PaletteCycle` (de.ambertation.wover.block.impl.classify.TintTrait) (org/betterx/betterend/registry/block/EndCrystalBlocks.java:13)
- [ ] `EndCrystalBlocks.AURORA_CRYSTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCrystalBlocks.java:14)
- [ ] `EndCrystalBlocks.SMARAGDANT_CRYSTAL_SHARD`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCrystalBlocks.java:15)
- [ ] `EndCrystalBlocks.SMARAGDANT_CRYSTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCrystalBlocks.java:18)
- [ ] `EndCrystalBlocks.SMARAGDANT_SUBBLOCKS`: it uses CrystalSubblocksMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndCrystalBlocks.java:19)
- [ ] `EndCrystalBlocks.BUDDING_SMARAGDANT_CRYSTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndCrystalBlocks.java:20)
- [x] `EndCrystalBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndCrystalBlocks.java:24)

### `EndDecorBlocks`

- [ ] `EndDecorBlocks.DENSE_SNOW`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:30)
- [ ] `EndDecorBlocks.EMERALD_ICE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:31)
- [ ] `EndDecorBlocks.DENSE_EMERALD_ICE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:32)
- [ ] `EndDecorBlocks.ANCIENT_EMERALD_ICE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:33)
- [ ] `EndDecorBlocks.NEON_CACTUS_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:34)
- [ ] `EndDecorBlocks.NEON_CACTUS_BLOCK_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:35)
- [ ] `EndDecorBlocks.NEON_CACTUS_BLOCK_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:36)
- [ ] `EndDecorBlocks.MENGER_SPONGE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:37)
- [ ] `EndDecorBlocks.MENGER_SPONGE_WET`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:38)
- [ ] `EndDecorBlocks.HYDRALUX_PETAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:39)
- [ ] `EndDecorBlocks.HYDRALUX_PETAL_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:45)
- [ ] `EndDecorBlocks.HYDRALUX_PETAL_COLORED`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndDecorBlocks.java:46)
- [ ] `EndDecorBlocks.HYDRALUX_PETAL_BLOCK_COLORED`: it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndDecorBlocks.java:49)
- [x] `EndDecorBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndDecorBlocks.java:63)

### `EndFactories`

- [ ] `EndFactories.AETERNIUM_ANVIL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:115)
- [ ] `EndFactories.AETERNIUM_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:116)
- [ ] `EndFactories.AMARANITA_CAP_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:117)
- [ ] `EndFactories.AMARANITA_HYMENOPHORE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:118)
- [ ] `EndFactories.AMARANITA_STEM_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:119)
- [ ] `EndFactories.AMBER_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:120)
- [ ] `EndFactories.ANCIENT_EMERALD_ICE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:121)
- [ ] `EndFactories.AURORA_CRYSTAL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:122)
- [ ] `EndFactories.BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:123)
- [ ] `EndFactories.BLUE_VINE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:124)
- [ ] `EndFactories.BLUE_VINE_LANTERN_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:125)
- [ ] `EndFactories.BLUE_VINE_SEED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:126)
- [ ] `EndFactories.BOLUX_MUSHROOM_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:127)
- [ ] `EndFactories.BRIMSTONE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:128)
- [ ] `EndFactories.BUBBLE_CORAL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:129)
- [ ] `EndFactories.BUDDING_SMARAGDANT_CRYSTAL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:130)
- [ ] `EndFactories.BULB_VINE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:133)
- [ ] `EndFactories.BULB_VINE_LANTERN_COLORED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:134)
- [ ] `EndFactories.BULB_VINE_SEED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:137)
- [ ] `EndFactories.CAVE_PUMPKIN_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:138)
- [ ] `EndFactories.CAVE_PUMPKIN_VINE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:139)
- [ ] `EndFactories.CHARCOAL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:140)
- [ ] `EndFactories.CHARNIA_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:141)
- [ ] `EndFactories.CRYSTAL_MOSS_COVER_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:142)
- [ ] `EndFactories.DRAGON_TREE_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:143)
- [ ] `EndFactories.EMERALD_ICE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:144)
- [ ] `EndFactories.ENDER_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:145)
- [ ] `EndFactories.ENDSTONE_DUST_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:146)
- [ ] `EndFactories.END_LILY_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:147)
- [ ] `EndFactories.END_LILY_SEED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:148)
- [ ] `EndFactories.END_LOTUS_FLOWER_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:149)
- [ ] `EndFactories.END_LOTUS_LEAF_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:150)
- [ ] `EndFactories.END_LOTUS_SEED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:151)
- [ ] `EndFactories.END_LOTUS_STEM_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:152)
- [ ] `EndFactories.END_PORTAL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:153)
- [ ] `EndFactories.END_STONE_SMELTER`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:154)
- [ ] `EndFactories.END_UNDERWATER_WALL_PLANT_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:155)
- [ ] `EndFactories.ETERNAL_PEDESTAL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:158)
- [ ] `EndFactories.FILALUX_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:159)
- [ ] `EndFactories.FILALUX_LANTERN_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:160)
- [ ] `EndFactories.FILALUX_WINGS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:161)
- [ ] `EndFactories.FLAMAEA_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:162)
- [ ] `EndFactories.FLAMMALIX_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:163)
- [ ] `EndFactories.FUR_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:164)
- [ ] `EndFactories.GLOWING_HYMENOPHORE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:165)
- [ ] `EndFactories.GLOWING_MOSS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:166)
- [ ] `EndFactories.GLOWING_PILLAR_LUMINOPHOR_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:167)
- [ ] `EndFactories.GLOWING_PILLAR_ROOTS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:170)
- [ ] `EndFactories.GLOWING_PILLAR_SEED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:171)
- [ ] `EndFactories.HELIX_TREE_LEAVES_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:172)
- [ ] `EndFactories.HELIX_TREE_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:173)
- [ ] `EndFactories.HYDRALUX_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:174)
- [ ] `EndFactories.HYDRALUX_PETAL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:175)
- [ ] `EndFactories.HYDRALUX_PETAL_COLORED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:176)
- [ ] `EndFactories.HYDRALUX_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:179)
- [ ] `EndFactories.HYDROTHERMAL_VENT_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:180)
- [ ] `EndFactories.INFUSION_PEDESTAL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:181)
- [ ] `EndFactories.JELLYSHROOM_CAP_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:182)
- [ ] `EndFactories.LACUGROVE_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:183)
- [ ] `EndFactories.LANCELEAF_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:184)
- [ ] `EndFactories.LANCELEAF_SEED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:185)
- [ ] `EndFactories.LARGE_AMARANITA_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:186)
- [ ] `EndFactories.LUCERNIA_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:187)
- [ ] `EndFactories.LUMECORN_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:188)
- [ ] `EndFactories.LUMECORN_SEED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:189)
- [ ] `EndFactories.MENGER_SPONGE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:190)
- [ ] `EndFactories.MENGER_SPONGE_WET_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:191)
- [ ] `EndFactories.MOSSY_DRAGON_BONE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:192)
- [ ] `EndFactories.MOSSY_GLOWSHROOM_CAP_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:193)
- [ ] `EndFactories.MOSSY_GLOWSHROOM_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:194)
- [ ] `EndFactories.MOSSY_OBSIDIAN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:197)
- [ ] `EndFactories.MURKWEED_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:198)
- [ ] `EndFactories.NEEDLEGRASS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:199)
- [ ] `EndFactories.NEON_CACTUS_PLANT_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:200)
- [ ] `EndFactories.POND_ANEMONE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:201)
- [ ] `EndFactories.PYTHADENDRON_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:202)
- [ ] `EndFactories.RESPAWN_OBELISK_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:203)
- [ ] `EndFactories.ROTATED_PILLAR_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:204)
- [ ] `EndFactories.RUNED_FLAVOLITE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:205)
- [ ] `EndFactories.SHADOW_BERRY_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:206)
- [ ] `EndFactories.SHADOW_GRASS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:207)
- [ ] `EndFactories.SILK_MOTH_HIVE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:208)
- [ ] `EndFactories.SILK_MOTH_NEST_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:209)
- [ ] `EndFactories.SMALL_AMARANITA_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:210)
- [ ] `EndFactories.SMALL_JELLYSHROOM_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:211)
- [ ] `EndFactories.SMARAGDANT_CRYSTAL_SHARD_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:212)
- [ ] `EndFactories.SULPHUR_CRYSTAL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:215)
- [ ] `EndFactories.TENANEA_FLOWERS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:216)
- [ ] `EndFactories.TENANEA_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:217)
- [ ] `EndFactories.TWISTED_UMBRELLA_MOSS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:218)
- [ ] `EndFactories.TWISTED_UMBRELLA_MOSS_TALL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:219)
- [ ] `EndFactories.UMBRELLA_MOSS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:222)
- [ ] `EndFactories.UMBRELLA_MOSS_TALL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:223)
- [ ] `EndFactories.UMBRELLA_TREE_CLUSTER_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:224)
- [ ] `EndFactories.UMBRELLA_TREE_CLUSTER_EMPTY_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:225)
- [ ] `EndFactories.UMBRELLA_TREE_MEMBRANE_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:228)
- [ ] `EndFactories.UMBRELLA_TREE_SAPLING_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:231)
- [ ] `EndFactories.VENT_BUBBLE_COLUMN_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:232)
- [ ] `EndFactories.END_TERRAIN_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:233)
- [ ] `EndFactories.CAVE_MOSS_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:234)
- [ ] `EndFactories.CHORUS_MUSHROOM_CROP_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:235)
- [ ] `EndFactories.BLOSSOM_BERRY_CROP_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:238)
- [ ] `EndFactories.AMBER_ROOT_CROP_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:241)
- [ ] `EndFactories.PALLIDIUM_TINY_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:244)
- [ ] `EndFactories.PALLIDIUM_THIN_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:247)
- [ ] `EndFactories.PALLIDIUM_HEAVY_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:250)
- [ ] `EndFactories.PALLIDIUM_FULL_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:253)
- [ ] `EndFactories.END_PATH_BLOCK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFactories.java:254)
- [x] `EndFactories.EndFactories(0)` (org/betterx/betterend/registry/block/EndFactories.java:258)
- [x] `EndFactories.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndFactories.java:261)

### `EndFunctionalBlocks`

- [ ] `EndFunctionalBlocks.SILK_MOTH_NEST`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:15)
- [ ] `EndFunctionalBlocks.SILK_MOTH_HIVE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:16)
- [ ] `EndFunctionalBlocks.RESPAWN_OBELISK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:17)
- [ ] `EndFunctionalBlocks.END_STONE_SMELTER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:18)
- [ ] `EndFunctionalBlocks.ETERNAL_PEDESTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:19)
- [ ] `EndFunctionalBlocks.INFUSION_PEDESTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:20)
- [ ] `EndFunctionalBlocks.AETERNIUM_ANVIL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:21)
- [ ] `EndFunctionalBlocks.END_PORTAL_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:22)
- [x] `EndFunctionalBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndFunctionalBlocks.java:24)

### `EndKinds`

- [ ] `EndKinds.OWN_PEDESTAL_MODEL`: a raw Supplier (org/betterx/betterend/registry/block/EndKinds.java:67)
- [ ] `EndKinds.PEDESTAL`: it uses PedestalKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:68)
- [ ] `EndKinds.STONE_LANTERN`: it uses StoneLanternKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:69)
- [ ] `EndKinds.FURNACE`: it uses FurnaceKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:70)
- [ ] `EndKinds.FLOWER_POT`: it uses FlowerPotKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:71)
- [ ] `EndKinds.METAL_BLOCK`: it uses MetalBlockKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:72)
- [ ] `EndKinds.TILE`: it uses TileKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:73)
- [ ] `EndKinds.ORE`: it uses OreKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:74)
- [ ] `EndKinds.CHANDELIER`: it uses ChandelierKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:75)
- [ ] `EndKinds.BULB_LANTERN`: it uses BulbLanternKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:76)
- [ ] `EndKinds.ANVIL`: it uses AnvilKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:77)
- [x] `EndKinds.EndKinds(0)` (org/betterx/betterend/registry/block/EndKinds.java:79)
- [ ] `EndKinds.optionalBlock(1)`: it uses `InputKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndKinds.java:82)
- [ ] `EndKinds.optionalItem(1)`: it uses `InputKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndKinds.java:86)
- [ ] `EndKinds.physical(1)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/registry/block/EndKinds.java:90)
- [ ] `EndKinds.light(2)`: it uses `ResolveContext` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndKinds.java:96)

### `EndKinds.AnvilKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.AnvilKind.AnvilKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:109)
- [ ] `EndKinds.AnvilKind.AnvilKind(7)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:121)
- [ ] `EndKinds.AnvilKind.createItem(3)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:147)
- [ ] `EndKinds.AnvilKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:151)

### `EndKinds.BulbLanternKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.BulbLanternKind.BulbLanternKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:180)
- [ ] `EndKinds.BulbLanternKind.BulbLanternKind(7)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:192)
- [ ] `EndKinds.BulbLanternKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:218)

### `EndKinds.ChandelierKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.ChandelierKind.ChandelierKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:244)
- [ ] `EndKinds.ChandelierKind.ChandelierKind(4)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:253)
- [ ] `EndKinds.ChandelierKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:268)

### `EndKinds.FlowerPotKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.FlowerPotKind.FlowerPotKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:299)
- [ ] `EndKinds.FlowerPotKind.FlowerPotKind(4)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:308)
- [ ] `EndKinds.FlowerPotKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:323)

### `EndKinds.FurnaceKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.FurnaceKind.FurnaceKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:345)
- [ ] `EndKinds.FurnaceKind.FurnaceKind(4)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:354)
- [ ] `EndKinds.FurnaceKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:369)

### `EndKinds.MetalBlockKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.MetalBlockKind.MetalBlockKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:395)
- [ ] `EndKinds.MetalBlockKind.MetalBlockKind(4)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:404)
- [ ] `EndKinds.MetalBlockKind.slotName(2)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:421)
- [ ] `EndKinds.MetalBlockKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:425)

### `EndKinds.OreKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.OreKind.OreKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:454)
- [ ] `EndKinds.OreKind.OreKind(8)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:467)
- [ ] `EndKinds.OreKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:495)

### `EndKinds.PedestalKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.PedestalKind.PedestalKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:523)
- [ ] `EndKinds.PedestalKind.PedestalKind(7)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:535)
- [ ] `EndKinds.PedestalKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:561)

### `EndKinds.StoneLanternKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.StoneLanternKind.StoneLanternKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:586)
- [ ] `EndKinds.StoneLanternKind.StoneLanternKind(4)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:595)
- [ ] `EndKinds.StoneLanternKind.slotName(2)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:610)
- [ ] `EndKinds.StoneLanternKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:614)

### `EndKinds.TileKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `EndKinds.TileKind.TileKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:637)
- [ ] `EndKinds.TileKind.TileKind(4)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:641)
- [ ] `EndKinds.TileKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndKinds.java:658)

### `EndLightBlocks`

- [ ] `EndLightBlocks.BLUE_VINE_LANTERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndLightBlocks.java:18)
- [ ] `EndLightBlocks.GLOWING_PILLAR_LUMINOPHOR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndLightBlocks.java:19)
- [ ] `EndLightBlocks.AMARANITA_LANTERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndLightBlocks.java:22)
- [ ] `EndLightBlocks.FILALUX_LANTERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndLightBlocks.java:23)
- [ ] `EndLightBlocks.IRON_BULB_LANTERN_COLORED`: it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/EndLightBlocks.java:24)
- [x] `EndLightBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndLightBlocks.java:35)

### `EndLoot`

- [ ] `EndLoot.BULB_VINE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:24)
- [ ] `EndLoot.CAVE_PUMPKIN`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:26)
- [ ] `EndLoot.CRYSTAL_SHARDS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:28)
- [ ] `EndLoot.EMPTY`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:34)
- [ ] `EndLoot.END_LILY`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:36)
- [ ] `EndLoot.END_LOTUS_FLOWER`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:38)
- [ ] `EndLoot.HYDRALUX`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:40)
- [ ] `EndLoot.LANCELEAF`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:42)
- [ ] `EndLoot.LUMECORN`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:44)
- [ ] `EndLoot.NEEDLEGRASS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:46)
- [ ] `EndLoot.NONE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:48)
- [ ] `EndLoot.POTTABLE_CROP`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:50)
- [ ] `EndLoot.RESPAWN_OBELISK`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:52)
- [ ] `EndLoot.SILK_MOTH_NEST`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:54)
- [ ] `EndLoot.SULPHUR_CRYSTAL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndLoot.java:56)
- [x] `EndLoot.EndLoot(0)` (org/betterx/betterend/registry/block/EndLoot.java:59)
- [x] `EndLoot.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndLoot.java:62)

### `EndMaterials`

- [ ] `EndMaterials.WOOD`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:21)
- [ ] `EndMaterials.FUR`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:24)
- [ ] `EndMaterials.NONFLAMMABLE_PLANT`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:26)
- [ ] `EndMaterials.SHEARED_PLANT`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:30)
- [ ] `EndMaterials.TERRAIN_PATH`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:35)
- [ ] `EndMaterials.PALLIDIUM`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:40)
- [ ] `EndMaterials.CRYSTAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:44)
- [ ] `EndMaterials.ENDSTONE_DUST`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:52)
- [ ] `EndMaterials.CAVE_PUMPKIN`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:58)
- [ ] `EndMaterials.DENSE_SNOW`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:64)
- [ ] `EndMaterials.EMERALD_ICE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:69)
- [ ] `EndMaterials.DENSE_EMERALD_ICE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:70)
- [ ] `EndMaterials.ANCIENT_EMERALD_ICE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:71)
- [ ] `EndMaterials.MENGER_SPONGE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:72)
- [ ] `EndMaterials.MENGER_SPONGE_WET`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:73)
- [ ] `EndMaterials.NEON_CACTUS_BLOCK`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:74)
- [ ] `EndMaterials.NEON_CACTUS`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:78)
- [ ] `EndMaterials.END_PORTAL_BLOCK`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:83)
- [ ] `EndMaterials.VENT_BUBBLE_COLUMN`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:87)
- [ ] `EndMaterials.ETERNAL_PEDESTAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:91)
- [ ] `EndMaterials.INFUSION_PEDESTAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:95)
- [ ] `EndMaterials.END_STONE_STALACTITE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:99)
- [ ] `EndMaterials.END_STONE_STALACTITE_CAVEMOSS`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:103)
- [ ] `EndMaterials.ice(2)`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:108)
- [ ] `EndMaterials.sponge(2)`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMaterials.java:112)
- [x] `EndMaterials.EndMaterials(0)` (org/betterx/betterend/registry/block/EndMaterials.java:118)

### `EndMetalBlocks`

- [ ] `EndMetalBlocks.IRON_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndMetalBlocks.java:23)
- [ ] `EndMetalBlocks.GOLD_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndMetalBlocks.java:26)
- [ ] `EndMetalBlocks.TERMINITE`: it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/registry/block/EndMetalBlocks.java:27)
- [ ] `EndMetalBlocks.THALLASIUM`: it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))) (org/betterx/betterend/registry/block/EndMetalBlocks.java:38)
- [ ] `EndMetalBlocks.AETERNIUM_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMetalBlocks.java:47)
- [ ] `EndMetalBlocks.CHARCOAL_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMetalBlocks.java:48)
- [ ] `EndMetalBlocks.ENDER_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMetalBlocks.java:49)
- [ ] `EndMetalBlocks.AMBER_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMetalBlocks.java:50)
- [x] `EndMetalBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndMetalBlocks.java:52)

### `EndMushroomBlocks`

- [ ] `EndMushroomBlocks.GLOWING_PILLAR_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:19)
- [ ] `EndMushroomBlocks.GLOWING_PILLAR_ROOTS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:20)
- [ ] `EndMushroomBlocks.GLOWING_PILLAR_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:21)
- [ ] `EndMushroomBlocks.SMALL_JELLYSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:22)
- [ ] `EndMushroomBlocks.BOLUX_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:23)
- [ ] `EndMushroomBlocks.LUMECORN_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:24)
- [ ] `EndMushroomBlocks.LUMECORN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:25)
- [ ] `EndMushroomBlocks.SMALL_AMARANITA_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:26)
- [ ] `EndMushroomBlocks.LARGE_AMARANITA_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:27)
- [ ] `EndMushroomBlocks.AMARANITA_STEM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:28)
- [ ] `EndMushroomBlocks.AMARANITA_HYPHAE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:29)
- [ ] `EndMushroomBlocks.AMARANITA_HYMENOPHORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:30)
- [ ] `EndMushroomBlocks.AMARANITA_FUR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:31)
- [ ] `EndMushroomBlocks.AMARANITA_CAP`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndMushroomBlocks.java:32)
- [x] `EndMushroomBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndMushroomBlocks.java:34)

### `EndOreBlocks`

- [ ] `EndOreBlocks.ENDER_ORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndOreBlocks.java:8)
- [ ] `EndOreBlocks.AMBER_ORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndOreBlocks.java:9)
- [x] `EndOreBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndOreBlocks.java:11)

### `EndPlantBlocks`

- [ ] `EndPlantBlocks.UMBRELLA_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:21)
- [ ] `EndPlantBlocks.UMBRELLA_MOSS_TALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:22)
- [ ] `EndPlantBlocks.CREEPING_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:23)
- [ ] `EndPlantBlocks.CHORUS_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:24)
- [ ] `EndPlantBlocks.CAVE_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:25)
- [ ] `EndPlantBlocks.CRYSTAL_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:26)
- [ ] `EndPlantBlocks.SHADOW_PLANT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:27)
- [ ] `EndPlantBlocks.BUSHY_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:28)
- [ ] `EndPlantBlocks.AMBER_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:29)
- [ ] `EndPlantBlocks.TWISTED_UMBRELLA_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:30)
- [ ] `EndPlantBlocks.TWISTED_UMBRELLA_MOSS_TALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:31)
- [ ] `EndPlantBlocks.JUNGLE_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:34)
- [ ] `EndPlantBlocks.BLOOMING_COOKSONIA`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:35)
- [ ] `EndPlantBlocks.SALTEAGO`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:36)
- [ ] `EndPlantBlocks.VAIOLUSH_FERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:37)
- [ ] `EndPlantBlocks.FRACTURN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:38)
- [ ] `EndPlantBlocks.CLAWFERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:39)
- [ ] `EndPlantBlocks.GLOBULAGUS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:40)
- [ ] `EndPlantBlocks.ORANGO`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:41)
- [ ] `EndPlantBlocks.AERIDIUM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:42)
- [ ] `EndPlantBlocks.LUTEBUS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:43)
- [ ] `EndPlantBlocks.LAMELLARIUM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:44)
- [ ] `EndPlantBlocks.INFLEXIA`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:45)
- [ ] `EndPlantBlocks.FLAMMALIX`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:46)
- [ ] `EndPlantBlocks.CRYSTAL_MOSS_COVER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:47)
- [ ] `EndPlantBlocks.LANCELEAF_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:48)
- [ ] `EndPlantBlocks.LANCELEAF`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:49)
- [ ] `EndPlantBlocks.NEON_CACTUS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:50)
- [ ] `EndPlantBlocks.CAVE_BUSH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:51)
- [ ] `EndPlantBlocks.MURKWEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:52)
- [ ] `EndPlantBlocks.NEEDLEGRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndPlantBlocks.java:53)
- [x] `EndPlantBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndPlantBlocks.java:55)

### `EndRecipes`

- [ ] `EndRecipes.PEDESTAL`: it uses `Template3` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:28)
- [ ] `EndRecipes.STONE_LANTERN`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:52)
- [ ] `EndRecipes.FURNACE`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:68)
- [ ] `EndRecipes.FLOWER_POT`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:83)
- [ ] `EndRecipes.METAL_BLOCK`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:99)
- [ ] `EndRecipes.TILE`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:114)
- [ ] `EndRecipes.CHANDELIER`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:133)
- [ ] `EndRecipes.BULB_LANTERN`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:149)
- [ ] `EndRecipes.ANVIL`: it uses `Template3` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:166)
- [ ] `EndRecipes.COLORED_WITH_DYE`: it uses `Template3` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:184)
- [ ] `EndRecipes.CRYSTAL_SUBBLOCK`: it uses `Template4` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:212)
- [ ] `EndRecipes.JELLYSHROOM_HANGING_SIGN`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betterend/registry/block/EndRecipes.java:227)
- [x] `EndRecipes.EndRecipes(0)` (org/betterx/betterend/registry/block/EndRecipes.java:248)
- [x] `EndRecipes.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndRecipes.java:251)
- [ ] `EndRecipes.pedestal(3)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:254)
- [ ] `EndRecipes.stoneLantern(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:258)
- [ ] `EndRecipes.furnace(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:262)
- [ ] `EndRecipes.flowerPot(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:266)
- [ ] `EndRecipes.metalBlock(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:270)
- [ ] `EndRecipes.tile(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:274)
- [ ] `EndRecipes.chandelier(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:278)
- [ ] `EndRecipes.bulbLantern(2)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:282)
- [ ] `EndRecipes.anvil(3)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:286)
- [ ] `EndRecipes.coloredWithDye(3)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:290)
- [ ] `EndRecipes.crystalSubblock(4)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:294)
- [ ] `EndRecipes.jellyshroomHangingSign(2)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betterend/registry/block/EndRecipes.java:298)

### `EndRecipes.Anvil`: it uses Minecraft's `Block`

- [ ] `EndRecipes.Anvil.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:303)
- [ ] `EndRecipes.Anvil.Anvil(3)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:302)
- [ ] `EndRecipes.Anvil.base(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:302)
- [ ] `EndRecipes.Anvil.tile(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:302)
- [ ] `EndRecipes.Anvil.ingot(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:302)

### `EndRecipes.BulbLantern`: it uses Minecraft's `Block`

- [ ] `EndRecipes.BulbLantern.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:314)
- [ ] `EndRecipes.BulbLantern.BulbLantern(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:313)
- [ ] `EndRecipes.BulbLantern.chain(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:313)
- [ ] `EndRecipes.BulbLantern.ingot(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:313)

### `EndRecipes.Chandelier`: it uses Minecraft's `ItemLike`

- [ ] `EndRecipes.Chandelier.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:324)
- [ ] `EndRecipes.Chandelier.Chandelier(1)`: its class isn't translated (it uses Minecraft's `ItemLike`) (org/betterx/betterend/registry/block/EndRecipes.java:323)
- [ ] `EndRecipes.Chandelier.ingot(0)`: its class isn't translated (it uses Minecraft's `ItemLike`) (org/betterx/betterend/registry/block/EndRecipes.java:323)

### `EndRecipes.ColoredWithDye`: it uses Minecraft's `Block`

- [ ] `EndRecipes.ColoredWithDye.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:330)
- [ ] `EndRecipes.ColoredWithDye.ColoredWithDye(3)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:329)
- [ ] `EndRecipes.ColoredWithDye.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:329)
- [ ] `EndRecipes.ColoredWithDye.dye(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:329)
- [ ] `EndRecipes.ColoredWithDye.craftEight(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:329)

### `EndRecipes.CrystalSubblock`: it uses Minecraft's `Block`

- [ ] `EndRecipes.CrystalSubblock.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:341)
- [ ] `EndRecipes.CrystalSubblock.CrystalSubblock(4)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:340)
- [ ] `EndRecipes.CrystalSubblock.ingredient(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:340)
- [ ] `EndRecipes.CrystalSubblock.count(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:340)
- [ ] `EndRecipes.CrystalSubblock.group(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:340)
- [ ] `EndRecipes.CrystalSubblock.shape(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:340)

### `EndRecipes.FlowerPot`: it uses Minecraft's `Block`

- [ ] `EndRecipes.FlowerPot.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:353)
- [ ] `EndRecipes.FlowerPot.FlowerPot(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:352)
- [ ] `EndRecipes.FlowerPot.brick(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:352)

### `EndRecipes.Furnace`: it uses Minecraft's `Block`

- [ ] `EndRecipes.Furnace.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:359)
- [ ] `EndRecipes.Furnace.Furnace(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:358)
- [ ] `EndRecipes.Furnace.base(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:358)

### `EndRecipes.JellyshroomHangingSign`: it uses Minecraft's `Block`

- [ ] `EndRecipes.JellyshroomHangingSign.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:365)
- [ ] `EndRecipes.JellyshroomHangingSign.JellyshroomHangingSign(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:364)
- [ ] `EndRecipes.JellyshroomHangingSign.strippedLog(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:364)
- [ ] `EndRecipes.JellyshroomHangingSign.log(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:364)

### `EndRecipes.MetalBlock`: it uses Minecraft's `ItemLike`

- [ ] `EndRecipes.MetalBlock.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:375)
- [ ] `EndRecipes.MetalBlock.MetalBlock(1)`: its class isn't translated (it uses Minecraft's `ItemLike`) (org/betterx/betterend/registry/block/EndRecipes.java:374)
- [ ] `EndRecipes.MetalBlock.ingot(0)`: its class isn't translated (it uses Minecraft's `ItemLike`) (org/betterx/betterend/registry/block/EndRecipes.java:374)

### `EndRecipes.Pedestal`: it uses Minecraft's `Block`

- [ ] `EndRecipes.Pedestal.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:381)
- [ ] `EndRecipes.Pedestal.Pedestal(3)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:380)
- [ ] `EndRecipes.Pedestal.slab(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:380)
- [ ] `EndRecipes.Pedestal.pillar(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:380)
- [ ] `EndRecipes.Pedestal.group(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:380)

### `EndRecipes.StoneLantern`: it uses Minecraft's `Block`

- [ ] `EndRecipes.StoneLantern.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:392)
- [ ] `EndRecipes.StoneLantern.StoneLantern(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:391)
- [ ] `EndRecipes.StoneLantern.slab(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:391)

### `EndRecipes.Tile`: it uses Minecraft's `Block`

- [ ] `EndRecipes.Tile.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/block/EndRecipes.java:398)
- [ ] `EndRecipes.Tile.Tile(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:397)
- [ ] `EndRecipes.Tile.base(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betterend/registry/block/EndRecipes.java:397)

### `EndSaplingBlocks`

- [ ] `EndSaplingBlocks.MOSSY_GLOWSHROOM_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:16)
- [ ] `EndSaplingBlocks.PYTHADENDRON_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:19)
- [ ] `EndSaplingBlocks.LACUGROVE_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:20)
- [ ] `EndSaplingBlocks.DRAGON_TREE_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:21)
- [ ] `EndSaplingBlocks.TENANEA_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:22)
- [ ] `EndSaplingBlocks.HELIX_TREE_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:23)
- [ ] `EndSaplingBlocks.UMBRELLA_TREE_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:24)
- [ ] `EndSaplingBlocks.LUCERNIA_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:25)
- [ ] `EndSaplingBlocks.HYDRALUX_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndSaplingBlocks.java:26)
- [x] `EndSaplingBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndSaplingBlocks.java:28)

### `EndStoneBlocks`

- [ ] `EndStoneBlocks.MOSSY_OBSIDIAN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:26)
- [ ] `EndStoneBlocks.DRAGON_BONE_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:27)
- [ ] `EndStoneBlocks.DRAGON_BONE_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:28)
- [ ] `EndStoneBlocks.DRAGON_BONE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:29)
- [ ] `EndStoneBlocks.MOSSY_DRAGON_BONE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:30)
- [ ] `EndStoneBlocks.FLAVOLITE`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:31)
- [ ] `EndStoneBlocks.VIOLECITE`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:32)
- [ ] `EndStoneBlocks.SULPHURIC_ROCK`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:33)
- [ ] `EndStoneBlocks.VIRID_JADESTONE`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:34)
- [ ] `EndStoneBlocks.AZURE_JADESTONE`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:35)
- [ ] `EndStoneBlocks.SANDY_JADESTONE`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:36)
- [ ] `EndStoneBlocks.UMBRALITH`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:37)
- [ ] `EndStoneBlocks.BRIMSTONE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:38)
- [ ] `EndStoneBlocks.SULPHUR_CRYSTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:39)
- [ ] `EndStoneBlocks.MISSING_TILE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:40)
- [ ] `EndStoneBlocks.END_STONE_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:41)
- [ ] `EndStoneBlocks.ANDESITE_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:44)
- [ ] `EndStoneBlocks.DIORITE_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:45)
- [ ] `EndStoneBlocks.GRANITE_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:46)
- [ ] `EndStoneBlocks.QUARTZ_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:47)
- [ ] `EndStoneBlocks.PURPUR_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:48)
- [ ] `EndStoneBlocks.BLACKSTONE_SET`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:49)
- [ ] `EndStoneBlocks.FLAVOLITE_RUNED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:50)
- [ ] `EndStoneBlocks.FLAVOLITE_RUNED_ETERNAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:51)
- [ ] `EndStoneBlocks.HYDROTHERMAL_VENT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:52)
- [ ] `EndStoneBlocks.VENT_BUBBLE_COLUMN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:53)
- [ ] `EndStoneBlocks.END_STONE_STALACTITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:54)
- [ ] `EndStoneBlocks.END_STONE_STALACTITE_CAVEMOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:55)
- [ ] `EndStoneBlocks.END_STONE_BRICK_VARIATIONS`: it uses `BlockSet` (de.ambertation.wover.sets.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:56)
- [ ] `EndStoneBlocks.END_STONE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:57)
- [ ] `EndStoneBlocks.END_STONE_STAIR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:58)
- [ ] `EndStoneBlocks.END_STONE_WALLS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndStoneBlocks.java:59)
- [x] `EndStoneBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndStoneBlocks.java:61)

### `EndTerrainBlocks`

- [ ] `EndTerrainBlocks.ENDSTONE_DUST`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:13)
- [ ] `EndTerrainBlocks.END_MYCELIUM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:14)
- [ ] `EndTerrainBlocks.END_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:15)
- [ ] `EndTerrainBlocks.CHORUS_NYLIUM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:16)
- [ ] `EndTerrainBlocks.CAVE_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:17)
- [ ] `EndTerrainBlocks.CRYSTAL_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:18)
- [ ] `EndTerrainBlocks.SHADOW_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:19)
- [ ] `EndTerrainBlocks.PINK_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:20)
- [ ] `EndTerrainBlocks.AMBER_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:21)
- [ ] `EndTerrainBlocks.JUNGLE_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:22)
- [ ] `EndTerrainBlocks.SANGNUM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:23)
- [ ] `EndTerrainBlocks.RUTISCUS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:24)
- [ ] `EndTerrainBlocks.PALLIDIUM_FULL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:25)
- [ ] `EndTerrainBlocks.PALLIDIUM_HEAVY`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:26)
- [ ] `EndTerrainBlocks.PALLIDIUM_THIN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:27)
- [ ] `EndTerrainBlocks.PALLIDIUM_TINY`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:28)
- [ ] `EndTerrainBlocks.END_MYCELIUM_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:29)
- [ ] `EndTerrainBlocks.END_MOSS_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:30)
- [ ] `EndTerrainBlocks.CHORUS_NYLIUM_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:31)
- [ ] `EndTerrainBlocks.CAVE_MOSS_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:32)
- [ ] `EndTerrainBlocks.CRYSTAL_MOSS_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:33)
- [ ] `EndTerrainBlocks.SHADOW_GRASS_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:34)
- [ ] `EndTerrainBlocks.PINK_MOSS_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:35)
- [ ] `EndTerrainBlocks.AMBER_MOSS_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:36)
- [ ] `EndTerrainBlocks.JUNGLE_MOSS_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:37)
- [ ] `EndTerrainBlocks.SANGNUM_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:38)
- [ ] `EndTerrainBlocks.RUTISCUS_PATH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndTerrainBlocks.java:39)
- [x] `EndTerrainBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndTerrainBlocks.java:41)

### `EndTints`

- [ ] `EndTints.AURORA_PALETTE`: it uses `PaletteCycle` (de.ambertation.wover.block.impl.classify.TintTrait) (org/betterx/betterend/registry/block/EndTints.java:9)
- [x] `EndTints.EndTints(0)` (org/betterx/betterend/registry/block/EndTints.java:17)

### `EndVineBlocks`

- [ ] `EndVineBlocks.BLUE_VINE_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:15)
- [ ] `EndVineBlocks.BLUE_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:16)
- [ ] `EndVineBlocks.BLUE_VINE_FUR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:17)
- [ ] `EndVineBlocks.DENSE_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:18)
- [ ] `EndVineBlocks.TWISTED_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:19)
- [ ] `EndVineBlocks.BULB_VINE_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:20)
- [ ] `EndVineBlocks.BULB_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:21)
- [ ] `EndVineBlocks.JUNGLE_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:22)
- [ ] `EndVineBlocks.RUBINEA`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:23)
- [ ] `EndVineBlocks.MAGNULA`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:24)
- [ ] `EndVineBlocks.FILALUX`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:25)
- [ ] `EndVineBlocks.FILALUX_WINGS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndVineBlocks.java:26)
- [x] `EndVineBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndVineBlocks.java:28)

### `EndWallPlantBlocks`

- [ ] `EndWallPlantBlocks.PURPLE_POLYPORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:9)
- [ ] `EndWallPlantBlocks.AURANT_POLYPORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:10)
- [ ] `EndWallPlantBlocks.TAIL_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:11)
- [ ] `EndWallPlantBlocks.CYAN_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:12)
- [ ] `EndWallPlantBlocks.TWISTED_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:13)
- [ ] `EndWallPlantBlocks.TUBE_WORM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:14)
- [ ] `EndWallPlantBlocks.BULB_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:15)
- [ ] `EndWallPlantBlocks.JUNGLE_FERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:16)
- [ ] `EndWallPlantBlocks.RUSCUS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:17)
- [x] `EndWallPlantBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndWallPlantBlocks.java:19)

### `EndWaterPlantBlocks`

- [ ] `EndWaterPlantBlocks.END_LOTUS_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:17)
- [ ] `EndWaterPlantBlocks.END_LOTUS_LEAF`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:18)
- [ ] `EndWaterPlantBlocks.END_LOTUS_FLOWER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:19)
- [ ] `EndWaterPlantBlocks.BUBBLE_CORAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:20)
- [ ] `EndWaterPlantBlocks.CHARNIA_RED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:21)
- [ ] `EndWaterPlantBlocks.CHARNIA_PURPLE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:22)
- [ ] `EndWaterPlantBlocks.CHARNIA_ORANGE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:23)
- [ ] `EndWaterPlantBlocks.CHARNIA_LIGHT_BLUE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:24)
- [ ] `EndWaterPlantBlocks.CHARNIA_CYAN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:25)
- [ ] `EndWaterPlantBlocks.CHARNIA_GREEN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:26)
- [ ] `EndWaterPlantBlocks.END_LILY`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:27)
- [ ] `EndWaterPlantBlocks.END_LILY_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:28)
- [ ] `EndWaterPlantBlocks.HYDRALUX`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:29)
- [ ] `EndWaterPlantBlocks.POND_ANEMONE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:30)
- [ ] `EndWaterPlantBlocks.FLAMAEA`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:31)
- [x] `EndWaterPlantBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java:33)

### `EndWoodBlocks`

- [ ] `EndWoodBlocks.MOSSY_GLOWSHROOM_CAP`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:23)
- [ ] `EndWoodBlocks.MOSSY_GLOWSHROOM_HYMENOPHORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:24)
- [ ] `EndWoodBlocks.MOSSY_GLOWSHROOM_FUR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:27)
- [ ] `EndWoodBlocks.MOSSY_GLOWSHROOM`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:28)
- [ ] `EndWoodBlocks.PYTHADENDRON_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:33)
- [ ] `EndWoodBlocks.PYTHADENDRON`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:34)
- [ ] `EndWoodBlocks.END_LOTUS_STEM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:38)
- [ ] `EndWoodBlocks.END_LOTUS`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:39)
- [ ] `EndWoodBlocks.LACUGROVE_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:43)
- [ ] `EndWoodBlocks.LACUGROVE`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:44)
- [ ] `EndWoodBlocks.DRAGON_TREE_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:48)
- [ ] `EndWoodBlocks.DRAGON_TREE`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:49)
- [ ] `EndWoodBlocks.TENANEA_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:53)
- [ ] `EndWoodBlocks.TENANEA_FLOWERS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:54)
- [ ] `EndWoodBlocks.TENANEA_OUTER_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:55)
- [ ] `EndWoodBlocks.TENANEA`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:56)
- [ ] `EndWoodBlocks.HELIX_TREE_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:59)
- [ ] `EndWoodBlocks.HELIX_TREE`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:60)
- [ ] `EndWoodBlocks.UMBRELLA_TREE_MEMBRANE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:63)
- [ ] `EndWoodBlocks.UMBRELLA_TREE_CLUSTER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:64)
- [ ] `EndWoodBlocks.UMBRELLA_TREE_CLUSTER_EMPTY`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:65)
- [ ] `EndWoodBlocks.UMBRELLA_TREE`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:68)
- [ ] `EndWoodBlocks.JELLYSHROOM_CAP_PURPLE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:71)
- [ ] `EndWoodBlocks.JELLYSHROOM`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:72)
- [ ] `EndWoodBlocks.LUCERNIA_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:75)
- [ ] `EndWoodBlocks.LUCERNIA_OUTER_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/EndWoodBlocks.java:76)
- [ ] `EndWoodBlocks.LUCERNIA`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:77)
- [ ] `EndWoodBlocks.LUCERNIA_JELLY`: it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`) (org/betterx/betterend/registry/block/EndWoodBlocks.java:81)
- [x] `EndWoodBlocks.ensureLoaded(0)` (org/betterx/betterend/registry/block/EndWoodBlocks.java:83)

### `ExtraTagsTrait`: it extends `BlockTrait` (de.ambertation.wover.block.api.classify)

- [ ] `ExtraTagsTrait.TYPE`: it uses `TraitType` (de.ambertation.wover.block.api.classify) (org/betterx/betterend/registry/block/ExtraTagsTrait.java:14)
- [ ] `ExtraTagsTrait.ExtraTagsTrait(2)`: its class isn't translated (it extends `BlockTrait` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/ExtraTagsTrait.java:18)
- [ ] `ExtraTagsTrait.type(0)`: its class isn't translated (it extends `BlockTrait` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/ExtraTagsTrait.java:23)
- [ ] `ExtraTagsTrait.datagen(1)`: its class isn't translated (it extends `BlockTrait` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/block/ExtraTagsTrait.java:27)

### `EndAttributes`

- [ ] `EndAttributes.BLINDNESS_RESISTANCE`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndAttributes.java:13)
- [ ] `EndAttributes.registerAttribute(3)`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndAttributes.java:15)
- [ ] `EndAttributes.addLivingEntityAttributes(1)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/registry/EndAttributes.java:21)

### `EndBiomeBuildings`

- [ ] `EndBiomeBuildings.ALL`: it uses Building, which isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:18)
- [ ] `EndBiomeBuildings.variants(6)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/registry/EndBiomeBuildings.java:20)
- [ ] `EndBiomeBuildings.single(5)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/registry/EndBiomeBuildings.java:32)
- [ ] `EndBiomeBuildings.add(7)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/betterend/registry/EndBiomeBuildings.java:36)
- [ ] `EndBiomeBuildings.templateCount(1)`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomeBuildings.java:60)
- [ ] `EndBiomeBuildings.all(0)`: it uses Building, which isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:64)
- [ ] `EndBiomeBuildings.of(1)`: it uses Building, which isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:68)
- [x] `EndBiomeBuildings.register(0)` (org/betterx/betterend/registry/EndBiomeBuildings.java:72)
- [x] `EndBiomeBuildings.EndBiomeBuildings(0)` (org/betterx/betterend/registry/EndBiomeBuildings.java:75)

### `EndBiomeBuildings.Building`: it uses EndBiomeKey, which isn't translated (it's generic)

- [ ] `EndBiomeBuildings.Building.Building(8)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)
- [ ] `EndBiomeBuildings.Building.templateIds(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:115)
- [ ] `EndBiomeBuildings.Building.biome(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)
- [ ] `EndBiomeBuildings.Building.name(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)
- [ ] `EndBiomeBuildings.Building.templates(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)
- [ ] `EndBiomeBuildings.Building.offsetY(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)
- [ ] `EndBiomeBuildings.Building.adjustment(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)
- [ ] `EndBiomeBuildings.Building.mound(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)
- [ ] `EndBiomeBuildings.Building.pool(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)
- [ ] `EndBiomeBuildings.Building.structure(0)`: its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)) (org/betterx/betterend/registry/EndBiomeBuildings.java:105)

### `EndBiomes`

- [ ] `EndBiomes.AMBER_LAND`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:38)
- [ ] `EndBiomes.BLOSSOMING_SPIRES`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:39)
- [ ] `EndBiomes.CHORUS_FOREST`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:40)
- [ ] `EndBiomes.CRYSTAL_MOUNTAINS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:41)
- [ ] `EndBiomes.DRAGON_GRAVEYARDS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:42)
- [ ] `EndBiomes.DRY_SHRUBLAND`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:43)
- [ ] `EndBiomes.DUST_WASTELANDS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:44)
- [ ] `EndBiomes.FOGGY_MUSHROOMLAND`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:45)
- [ ] `EndBiomes.GLOWING_GRASSLANDS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:46)
- [ ] `EndBiomes.ICE_STARFIELD`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:47)
- [ ] `EndBiomes.FLOWER_ISLETS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:48)
- [ ] `EndBiomes.WATERFALL_PONDS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:49)
- [ ] `EndBiomes.LANTERN_WOODS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:50)
- [ ] `EndBiomes.MEGALAKE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:51)
- [ ] `EndBiomes.SHADOW_FOREST`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:52)
- [ ] `EndBiomes.SULPHUR_SPRINGS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:53)
- [ ] `EndBiomes.UMBRELLA_JUNGLE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:54)
- [ ] `EndBiomes.UMBRA_VALLEY`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:55)
- [ ] `EndBiomes.MEGALAKE_GROVE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:56)
- [ ] `EndBiomes.NEON_OASIS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:57)
- [ ] `EndBiomes.PAINTED_MOUNTAINS`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:58)
- [ ] `EndBiomes.EMPTY_END_CAVE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:61)
- [ ] `EndBiomes.EMPTY_SMARAGDANT_CAVE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:62)
- [ ] `EndBiomes.LUSH_SMARAGDANT_CAVE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:63)
- [ ] `EndBiomes.EMPTY_AURORA_CAVE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:64)
- [ ] `EndBiomes.LUSH_AURORA_CAVE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:65)
- [ ] `EndBiomes.JADE_CAVE`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/betterend/registry/EndBiomes.java:66)
- [ ] `EndBiomes.register(0)`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/registry/EndBiomes.java:68)

### `EndBlockEntities`

- [ ] `EndBlockEntities.END_STONE_SMELTER`: it uses Minecraft's `BlockEntityType` (org/betterx/betterend/registry/EndBlockEntities.java:22)
- [ ] `EndBlockEntities.PEDESTAL`: it uses Minecraft's `BlockEntityType` (org/betterx/betterend/registry/EndBlockEntities.java:25)
- [ ] `EndBlockEntities.ETERNAL_PEDESTAL`: it uses Minecraft's `BlockEntityType` (org/betterx/betterend/registry/EndBlockEntities.java:28)
- [ ] `EndBlockEntities.INFUSION_PEDESTAL`: it uses Minecraft's `BlockEntityType` (org/betterx/betterend/registry/EndBlockEntities.java:31)
- [ ] `EndBlockEntities.HYDROTHERMAL_VENT`: it uses Minecraft's `BlockEntityType` (org/betterx/betterend/registry/EndBlockEntities.java:34)
- [ ] `EndBlockEntities.FLOWER_POT`: it uses Minecraft's `BlockEntityType` (org/betterx/betterend/registry/EndBlockEntities.java:37)
- [ ] `EndBlockEntities.registerBlockEntity(2)`: it's generic (org/betterx/betterend/registry/EndBlockEntities.java:41)
- [x] `EndBlockEntities.register(0)` (org/betterx/betterend/registry/EndBlockEntities.java:45)
- [ ] `EndBlockEntities.addLateRegisteredBlocks(0)`: it calls EndBlockEntities.getPedestals, which isn't translated (org/betterx/betterend/registry/EndBlockEntities.java:48)
- [ ] `EndBlockEntities.getPedestals(0)`: it uses Minecraft's `Block` (org/betterx/betterend/registry/EndBlockEntities.java:58)
- [ ] `EndBlockEntities.getFlowerPots(0)`: it uses Minecraft's `Block` (org/betterx/betterend/registry/EndBlockEntities.java:65)

### `EndBlockEntityRenders`

- [ ] `EndBlockEntityRenders.register(0)`: it uses Minecraft's `BlockEntityRendererProvider` (org/betterx/betterend/registry/EndBlockEntityRenders.java:14)

### `EndBlocks`

- [ ] `EndBlocks.BLOCKS_REGISTRY`: it uses `BlockRegistry` (de.ambertation.wover.block.api) (org/betterx/betterend/registry/EndBlocks.java:28)
- [ ] `EndBlocks.getModBlocks(0)`: it uses Minecraft's `Block` (org/betterx/betterend/registry/EndBlocks.java:30)
- [ ] `EndBlocks.key(2)`: it's generic (org/betterx/betterend/registry/EndBlocks.java:34)
- [ ] `EndBlocks.getBlockRegistry(0)`: it uses `BlockRegistry` (de.ambertation.wover.block.api) (org/betterx/betterend/registry/EndBlocks.java:38)
- [x] `EndBlocks.ensureStaticallyLoaded(0)` (org/betterx/betterend/registry/EndBlocks.java:47)

### `EndCarvers`

- [ ] `EndCarvers.END_ROUND_CAVE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndCarvers.java:14)
- [ ] `EndCarvers.END_TUNNEL_CAVE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndCarvers.java:15)
- [ ] `EndCarvers.ROUND_CAVE`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndCarvers.java:16)
- [ ] `EndCarvers.TUNNEL_CAVE`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndCarvers.java:17)
- [ ] `EndCarvers.registerType(2)`: it's generic (org/betterx/betterend/registry/EndCarvers.java:19)
- [ ] `EndCarvers.carverKey(1)`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndCarvers.java:23)
- [x] `EndCarvers.ensureStaticallyLoaded(0)` (org/betterx/betterend/registry/EndCarvers.java:27)

### `EndClassification`: it implements `WoverBlocksEntrypoint` (de.ambertation.wover.block.api.classify)

- [ ] `EndClassification.registerClassification(0)`: its class isn't translated (it implements `WoverBlocksEntrypoint` (de.ambertation.wover.block.api.classify)) (org/betterx/betterend/registry/EndClassification.java:42)
- [ ] `EndClassification.init(1)`: it uses `Class` (org/betterx/betterend/registry/EndClassification.java:84)

### `EndEnchantments`

- [ ] `EndEnchantments.END_VEIL`: it uses `EnchantmentKey` (de.ambertation.wover.enchantment.api) (org/betterx/betterend/registry/EndEnchantments.java:12)
- [ ] `EndEnchantments.RESONANCE`: it uses `EnchantmentKey` (de.ambertation.wover.enchantment.api) (org/betterx/betterend/registry/EndEnchantments.java:13)
- [ ] `EndEnchantments.END_VEIL_STATE`: it uses Minecraft's `DataComponentType` (org/betterx/betterend/registry/EndEnchantments.java:14)
- [x] `EndEnchantments.ensureStaticallyLoaded(0)` (org/betterx/betterend/registry/EndEnchantments.java:18)

### `EndEntities`

- [ ] `EndEntities.SPAWN_EGGS`: it uses SpawnEgg, which isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/registry/EndEntities.java:32)
- [ ] `EndEntities.DRAGONFLY`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betterend/registry/EndEntities.java:33)
- [ ] `EndEntities.END_SLIME`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betterend/registry/EndEntities.java:44)
- [ ] `EndEntities.END_FISH`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betterend/registry/EndEntities.java:55)
- [ ] `EndEntities.SHADOW_WALKER`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betterend/registry/EndEntities.java:66)
- [ ] `EndEntities.CUBOZOA`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betterend/registry/EndEntities.java:77)
- [ ] `EndEntities.SILK_MOTH`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betterend/registry/EndEntities.java:88)
- [ ] `EndEntities.spawnEggs(0)`: it uses SpawnEgg, which isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/registry/EndEntities.java:100)
- [ ] `EndEntities.register(0)`: it uses EndEntities.DRAGONFLY, which isn't translated (it uses `BCLEntityWrapper` (org.betterx.bclib.entity)) (org/betterx/betterend/registry/EndEntities.java:104)
- [ ] `EndEntities.register(5)`: it's generic (org/betterx/betterend/registry/EndEntities.java:118)
- [ ] `EndEntities.register(9)`: it's generic (org/betterx/betterend/registry/EndEntities.java:125)

### `EndEntities.SpawnEgg`: it uses `ItemKey` (de.ambertation.wover.item.api.classify)

- [ ] `EndEntities.SpawnEgg.SpawnEgg(4)`: its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/registry/EndEntities.java:145)
- [ ] `EndEntities.SpawnEgg.key(0)`: its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/registry/EndEntities.java:145)
- [ ] `EndEntities.SpawnEgg.type(0)`: its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/registry/EndEntities.java:145)
- [ ] `EndEntities.SpawnEgg.background(0)`: its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/registry/EndEntities.java:145)
- [ ] `EndEntities.SpawnEgg.dots(0)`: its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/registry/EndEntities.java:145)

### `EndEntitiesRenders`

- [ ] `EndEntitiesRenders.DRAGONFLY_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:29)
- [ ] `EndEntitiesRenders.END_SLIME_SHELL_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:30)
- [ ] `EndEntitiesRenders.END_SLIME_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:31)
- [ ] `EndEntitiesRenders.END_FISH_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:32)
- [ ] `EndEntitiesRenders.CUBOZOA_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:33)
- [ ] `EndEntitiesRenders.SILK_MOTH_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:34)
- [ ] `EndEntitiesRenders.TEST_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:35)
- [ ] `EndEntitiesRenders.CRYSTALITE_CHESTPLATE`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:36)
- [ ] `EndEntitiesRenders.CRYSTALITE_CHESTPLATE_THIN`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:37)
- [ ] `EndEntitiesRenders.CRYSTALITE_HELMET`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:38)
- [ ] `EndEntitiesRenders.CRYSTALITE_LEGGINGS`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:39)
- [ ] `EndEntitiesRenders.CRYSTALITE_BOOTS`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:40)
- [ ] `EndEntitiesRenders.register(0)`: it calls EndEntitiesRenders.register, which isn't translated (org/betterx/betterend/registry/EndEntitiesRenders.java:42)
- [ ] `EndEntitiesRenders.register(2)`: it uses Minecraft's `EntityType` (org/betterx/betterend/registry/EndEntitiesRenders.java:62)
- [ ] `EndEntitiesRenders.registerMain(1)`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betterend/registry/EndEntitiesRenders.java:66)

### `EndFeatures`

- [ ] `EndFeatures.STALACTITE_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:73)
- [ ] `EndFeatures.VINE_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:74)
- [ ] `EndFeatures.WALL_PLANT_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:75)
- [ ] `EndFeatures.WALL_PLANT_ON_LOG_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:76)
- [ ] `EndFeatures.GLOW_PILLAR_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:77)
- [ ] `EndFeatures.HYDRALUX_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:78)
- [ ] `EndFeatures.LANCELEAF_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:79)
- [ ] `EndFeatures.MENGER_SPONGE_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:80)
- [ ] `EndFeatures.SINGLE_PLANT_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:81)
- [ ] `EndFeatures.SINGLE_INVERTED_SCATTER_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:82)
- [ ] `EndFeatures.DOUBLE_PLANT_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:85)
- [ ] `EndFeatures.UNDERWATER_PLANT_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:86)
- [ ] `EndFeatures.ARCH_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:87)
- [ ] `EndFeatures.THIN_ARCH_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:88)
- [ ] `EndFeatures.CHARNIA_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:89)
- [ ] `EndFeatures.BLUE_VINE_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:90)
- [ ] `EndFeatures.FILALUX_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:91)
- [ ] `EndFeatures.END_LILY_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:92)
- [ ] `EndFeatures.END_LOTUS_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:93)
- [ ] `EndFeatures.END_LOTUS_LEAF_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:94)
- [ ] `EndFeatures.BUSH_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:95)
- [ ] `EndFeatures.SINGLE_BLOCK_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:96)
- [ ] `EndFeatures.BUSH_WITH_OUTER_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:97)
- [ ] `EndFeatures.STALACTITE_CLUSTER`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:98)
- [ ] `EndFeatures.CAVE_SURFACE_COAT`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:99)
- [ ] `EndFeatures.CAVE_VEGETATION_PATCH`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:100)
- [ ] `EndFeatures.LAYERED_ORE_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:101)
- [ ] `EndFeatures.ICE_STAR_FEATURE`: it uses Minecraft's `MapCodec` (org/betterx/betterend/registry/EndFeatures.java:102)
- [ ] `EndFeatures.MOSSY_GLOWSHROOM_FEATURE`: it uses MossyGlowshroomFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:103)
- [ ] `EndFeatures.PYTHADENDRON_TREE_FEATURE`: it uses PythadendronTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:106)
- [ ] `EndFeatures.LACUGROVE_FEATURE`: it uses LacugroveFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:109)
- [ ] `EndFeatures.DRAGON_TREE_FEATURE`: it uses DragonTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:110)
- [ ] `EndFeatures.TENANEA_FEATURE`: it uses TenaneaFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:111)
- [ ] `EndFeatures.HELIX_TREE_FEATURE`: it uses HelixTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:112)
- [ ] `EndFeatures.DRAGON_HELIX_TREE_FEATURE`: it uses DragonHelixTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:113)
- [ ] `EndFeatures.UMBRELLA_TREE_FEATURE`: it uses UmbrellaTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:116)
- [ ] `EndFeatures.JELLYSHROOM_FEATURE`: it uses JellyshroomFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:117)
- [ ] `EndFeatures.GIGANTIC_AMARANITA_FEATURE`: it uses GiganticAmaranitaFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:118)
- [ ] `EndFeatures.LUCERNIA_FEATURE`: it uses LucerniaFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:121)
- [ ] `EndFeatures.TENANEA_BUSH_FEATURE`: it uses TenaneaBushFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:122)
- [ ] `EndFeatures.LUMECORN_FEATURE`: it uses Lumecorn, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:123)
- [ ] `EndFeatures.LARGE_AMARANITA_FEATURE`: it uses LargeAmaranitaFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:124)
- [ ] `EndFeatures.AMARANITA_PATCH_FEATURE`: it uses AmaranitaPatchFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:125)
- [ ] `EndFeatures.NEON_CACTUS_FEATURE`: it uses NeonCactusFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:126)
- [ ] `EndFeatures.DESERT_LAKE_FEATURE`: it uses DesertLakeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:127)
- [ ] `EndFeatures.SULPHURIC_LAKE_FEATURE`: it uses SulphuricLakeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:128)
- [ ] `EndFeatures.SURFACE_VENT_FEATURE`: it uses SurfaceVentFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:129)
- [ ] `EndFeatures.SULPHUR_HILL_FEATURE`: it uses SulphurHillFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:130)
- [ ] `EndFeatures.OBSIDIAN_PILLAR_FEATURE`: it uses ObsidianPillarBasementFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:131)
- [ ] `EndFeatures.OBSIDIAN_BOULDER_FEATURE`: it uses ObsidianBoulderFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:134)
- [ ] `EndFeatures.FALLEN_PILLAR_FEATURE`: it uses FallenPillarFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:137)
- [ ] `EndFeatures.SILK_MOTH_NEST_FEATURE`: it uses SilkMothNestFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:138)
- [ ] `EndFeatures.SPIRE_FEATURE`: it uses SpireFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:139)
- [ ] `EndFeatures.FLOATING_SPIRE_FEATURE`: it uses FloatingSpireFeature, which isn't translated (it extends `SpireFeature`) (org/betterx/betterend/registry/EndFeatures.java:140)
- [ ] `EndFeatures.GEYSER_FEATURE`: it uses GeyserFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:141)
- [ ] `EndFeatures.POND_WITH_WATERFALL_FEATURE`: it uses PondWithWaterfallFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:142)
- [ ] `EndFeatures.OVERWORLD_ISLAND`: it uses BiomeIslandFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:145)
- [ ] `EndFeatures.SMARAGDANT_CRYSTAL_FEATURE`: it uses SmaragdantCrystalFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:146)
- [ ] `EndFeatures.BIG_AURORA_CRYSTAL_FEATURE`: it uses BigAuroraCrystalFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:149)
- [ ] `EndFeatures.CAVE_PUMPKIN_FEATURE`: it uses CavePumpkinFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/registry/EndFeatures.java:152)
- [ ] `EndFeatures.registerType(2)`: it's generic (org/betterx/betterend/registry/EndFeatures.java:154)
- [ ] `EndFeatures.register(3)`: it's generic (org/betterx/betterend/registry/EndFeatures.java:159)
- [ ] `EndFeatures.addDefaultFeatures(2)`: it uses EndBiomeBuilder, which isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betterend/registry/EndFeatures.java:164)
- [x] `EndFeatures.register(0)` (org/betterx/betterend/registry/EndFeatures.java:171)

### `EndItems`

- [ ] `EndItems.ITEMS_REGISTRY`: it uses `ItemRegistry` (de.ambertation.wover.item.api) (org/betterx/betterend/registry/EndItems.java:24)
- [ ] `EndItems.getModItems(0)`: it calls EndItems.getItemRegistry, which isn't translated (org/betterx/betterend/registry/EndItems.java:26)
- [ ] `EndItems.key(2)`: it's generic (org/betterx/betterend/registry/EndItems.java:30)
- [x] `EndItems.makeEndItemSettings(0)` (org/betterx/betterend/registry/EndItems.java:34)
- [ ] `EndItems.getItemRegistry(0)`: it uses `ItemRegistry` (de.ambertation.wover.item.api) (org/betterx/betterend/registry/EndItems.java:38)
- [x] `EndItems.ensureStaticallyLoaded(0)` (org/betterx/betterend/registry/EndItems.java:47)
- [x] `EndItems.defaultSettings(0)` (org/betterx/betterend/registry/EndItems.java:56)

### `EndItems.ExternalItemModel`

- [ ] `EndItems.ExternalItemModel.build(0)`: it uses `ItemModel` (de.ambertation.wover.item.api.model) (org/betterx/betterend/registry/EndItems.java:62)
- [ ] `EndItems.ExternalItemModel.factory(0)`: it uses `ItemModelFactory` (de.ambertation.wover.item.api.model) (org/betterx/betterend/registry/EndItems.java:66)

### `EndMenuTypes`

- [ ] `EndMenuTypes.END_STONE_SMELTER`: it uses Minecraft's `MenuType` (org/betterx/betterend/registry/EndMenuTypes.java:15)
- [ ] `EndMenuTypes.registerSimple(2)`: it's generic (org/betterx/betterend/registry/EndMenuTypes.java:17)
- [x] `EndMenuTypes.ensureStaticallyLoaded(0)` (org/betterx/betterend/registry/EndMenuTypes.java:22)

### `EndModelProviders`

- [ ] `EndModelProviders.register(0)`: it calls CrystaliteArmorRenderer.register, which isn't translated (org/betterx/betterend/registry/EndModelProviders.java:9)

### `EndNumericProviders`

- [ ] `EndNumericProviders.SULPHURIC_SURF`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndNumericProviders.java:14)
- [ ] `EndNumericProviders.VERTICAL_BAND`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndNumericProviders.java:15)
- [ ] `EndNumericProviders.SPLIT_NOISE`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndNumericProviders.java:16)
- [ ] `EndNumericProviders.UMBRA_SRF`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndNumericProviders.java:17)
- [ ] `EndNumericProviders.register(0)`: it uses EndNumericProviders.SULPHURIC_SURF, which isn't translated (it uses Minecraft's `ResourceKey`) (org/betterx/betterend/registry/EndNumericProviders.java:19)

### `EndParticleProviders`

- [ ] `EndParticleProviders.register(0)`: it calls Fabric's `ParticleProviderRegistry`.getInstance (org/betterx/betterend/registry/EndParticleProviders.java:21)

### `EndParticles`

- [ ] `EndParticles.GLOWING_SPHERE`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:10)
- [ ] `EndParticles.PORTAL_SPHERE`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:11)
- [ ] `EndParticles.INFUSION`: it uses Minecraft's `ParticleType` (org/betterx/betterend/registry/EndParticles.java:12)
- [ ] `EndParticles.SULPHUR_PARTICLE`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:15)
- [ ] `EndParticles.GEYSER_PARTICLE`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:16)
- [ ] `EndParticles.SNOWFLAKE`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:17)
- [ ] `EndParticles.AMBER_SPHERE`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:18)
- [ ] `EndParticles.BLACK_SPORE`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:19)
- [ ] `EndParticles.TENANEA_PETAL`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:20)
- [ ] `EndParticles.JUNGLE_SPORE`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:21)
- [ ] `EndParticles.FIREFLY`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:22)
- [ ] `EndParticles.SMARAGDANT`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:23)
- [ ] `EndParticles.INFUSION_MIST`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:24)
- [ ] `EndParticles.INFUSION_MIST_NORTH`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:25)
- [ ] `EndParticles.register(1)`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:27)
- [ ] `EndParticles.registerFar(1)`: it uses Minecraft's `SimpleParticleType` (org/betterx/betterend/registry/EndParticles.java:31)
- [x] `EndParticles.ensureStaticallyLoadedServerside(0)` (org/betterx/betterend/registry/EndParticles.java:35)

### `EndPoiTypes`

- [ ] `EndPoiTypes.ETERNAL_PORTAL`: it uses `WoverPoiType` (de.ambertation.wover.poi.api) (org/betterx/betterend/registry/EndPoiTypes.java:15)
- [ ] `EndPoiTypes.ETERNAL_PORTAL_FRAME`: it uses `WoverPoiType` (de.ambertation.wover.poi.api) (org/betterx/betterend/registry/EndPoiTypes.java:21)
- [x] `EndPoiTypes.register(0)` (org/betterx/betterend/registry/EndPoiTypes.java:28)

### `EndPortalLooks`

- [ ] `EndPortalLooks.ETERNAL`: it uses `PortalLook` (de.ambertation.wover.portal.api) (org/betterx/betterend/registry/EndPortalLooks.java:9)
- [x] `EndPortalLooks.EndPortalLooks(0)` (org/betterx/betterend/registry/EndPortalLooks.java:15)
- [x] `EndPortalLooks.ensureLoaded(0)` (org/betterx/betterend/registry/EndPortalLooks.java:18)

### `EndPortals`

- [ ] `EndPortals.OVERWORLD_ID`: it uses Minecraft's `Level`.OVERWORLD (org/betterx/betterend/registry/EndPortals.java:17)
- [ ] `EndPortals.portals`: it uses PortalInfo, which isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/registry/EndPortals.java:18)
- [ ] `EndPortals.loadPortals(0)`: it uses `File` (java.io) (org/betterx/betterend/registry/EndPortals.java:20)
- [ ] `EndPortals.getCount(0)`: it uses EndPortals.portals, which isn't translated (it uses PortalInfo, which isn't translated (it uses Minecraft's `ServerLevel`)) (org/betterx/betterend/registry/EndPortals.java:47)
- [ ] `EndPortals.getWorld(2)`: it uses Minecraft's `ServerLevel` (org/betterx/betterend/registry/EndPortals.java:55)
- [ ] `EndPortals.getWorldId(1)`: a nullable object kept in a changing static (org/betterx/betterend/registry/EndPortals.java:59)
- [ ] `EndPortals.getPortalIdByItem(1)`: a nullable object kept in a changing static (org/betterx/betterend/registry/EndPortals.java:63)
- [ ] `EndPortals.getPortalIdByWorld(1)`: a nullable object kept in a changing static (org/betterx/betterend/registry/EndPortals.java:73)
- [ ] `EndPortals.getColor(1)`: a nullable object kept in a changing static (org/betterx/betterend/registry/EndPortals.java:83)
- [ ] `EndPortals.isAvailableItem(1)`: a nullable object kept in a changing static (org/betterx/betterend/registry/EndPortals.java:87)
- [ ] `EndPortals.makeDefault(1)`: it uses `File` (java.io) (org/betterx/betterend/registry/EndPortals.java:97)
- [ ] `EndPortals.makeDefault(0)`: it uses PortalInfo, which isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/registry/EndPortals.java:107)

### `EndPortals.PortalInfo`: it uses Minecraft's `ServerLevel`

- [ ] `EndPortals.PortalInfo.PortalInfo(1)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/registry/EndPortals.java:117)
- [ ] `EndPortals.PortalInfo.PortalInfo(5)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/registry/EndPortals.java:127)
- [ ] `EndPortals.PortalInfo.getWorld(1)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/registry/EndPortals.java:133)
- [ ] `EndPortals.PortalInfo.toJson(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (org/betterx/betterend/registry/EndPortals.java:148)

### `EndProcessors`

- [ ] `EndProcessors.WEATHERED_10_PERCENT`: it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors) (org/betterx/betterend/registry/EndProcessors.java:8)
- [ ] `EndProcessors.CRACK_20_PERCENT`: it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors) (org/betterx/betterend/registry/EndProcessors.java:9)
- [ ] `EndProcessors.CRACK_AND_WEATHER`: it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors) (org/betterx/betterend/registry/EndProcessors.java:10)
- [ ] `EndProcessors.END_STREET`: it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors) (org/betterx/betterend/registry/EndProcessors.java:11)
- [ ] `EndProcessors.BIOME_BUILDING`: it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors) (org/betterx/betterend/registry/EndProcessors.java:12)
- [ ] `EndProcessors.BIOME_BUILDING_MOUND`: it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors) (org/betterx/betterend/registry/EndProcessors.java:13)
- [ ] `EndProcessors.CRASHED_SHIP`: it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors) (org/betterx/betterend/registry/EndProcessors.java:14)
- [ ] `EndProcessors.CRYING_10_PERCENT`: it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors) (org/betterx/betterend/registry/EndProcessors.java:15)

### `EndScreens`

- [ ] `EndScreens.register(0)`: it uses EndMenuTypes.END_STONE_SMELTER, which isn't translated (it uses Minecraft's `MenuType`) (org/betterx/betterend/registry/EndScreens.java:7)

### `EndSounds`

- [ ] `EndSounds.MUSIC_FOREST`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:15)
- [ ] `EndSounds.MUSIC_WATER`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:16)
- [ ] `EndSounds.MUSIC_DARK`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:17)
- [ ] `EndSounds.MUSIC_OPENSPACE`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:18)
- [ ] `EndSounds.MUSIC_CAVES`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:19)
- [ ] `EndSounds.AMBIENT_FOGGY_MUSHROOMLAND`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:20)
- [ ] `EndSounds.AMBIENT_CHORUS_FOREST`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:21)
- [ ] `EndSounds.AMBIENT_MEGALAKE`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:22)
- [ ] `EndSounds.AMBIENT_DUST_WASTELANDS`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:23)
- [ ] `EndSounds.AMBIENT_MEGALAKE_GROVE`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:24)
- [ ] `EndSounds.AMBIENT_BLOSSOMING_SPIRES`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:25)
- [ ] `EndSounds.AMBIENT_SULPHUR_SPRINGS`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:26)
- [ ] `EndSounds.AMBIENT_UMBRELLA_JUNGLE`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:27)
- [ ] `EndSounds.AMBIENT_GLOWING_GRASSLANDS`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:28)
- [ ] `EndSounds.AMBIENT_CAVES`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:29)
- [ ] `EndSounds.AMBIENT_AMBER_LAND`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:30)
- [ ] `EndSounds.UMBRA_VALLEY`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:31)
- [ ] `EndSounds.ENTITY_DRAGONFLY`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:32)
- [ ] `EndSounds.ENTITY_SHADOW_WALKER`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:33)
- [ ] `EndSounds.ENTITY_SHADOW_WALKER_DAMAGE`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:34)
- [ ] `EndSounds.ENTITY_SHADOW_WALKER_DEATH`: it uses Minecraft's `Holder` (org/betterx/betterend/registry/EndSounds.java:35)
- [ ] `EndSounds.RECORD_STRANGE_AND_ALIEN_SOUND`: it uses Minecraft's `Reference` (org/betterx/betterend/registry/EndSounds.java:36)
- [ ] `EndSounds.RECORD_GRASPING_AT_STARS_SOUND`: it uses Minecraft's `Reference` (org/betterx/betterend/registry/EndSounds.java:37)
- [ ] `EndSounds.RECORD_ENDSEEKER_SOUND`: it uses Minecraft's `Reference` (org/betterx/betterend/registry/EndSounds.java:38)
- [ ] `EndSounds.RECORD_EO_DRACONA_SOUND`: it uses Minecraft's `Reference` (org/betterx/betterend/registry/EndSounds.java:39)
- [ ] `EndSounds.RECORD_STRANGE_AND_ALIEN`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndSounds.java:40)
- [ ] `EndSounds.RECORD_GRASPING_AT_STARS`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndSounds.java:41)
- [ ] `EndSounds.RECORD_ENDSEEKER`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndSounds.java:42)
- [ ] `EndSounds.RECORD_EO_DRACONA`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndSounds.java:43)
- [x] `EndSounds.register(0)` (org/betterx/betterend/registry/EndSounds.java:45)
- [ ] `EndSounds.createSongKey(1)`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/EndSounds.java:48)
- [ ] `EndSounds.register(2)`: it uses Minecraft's `Reference` (org/betterx/betterend/registry/EndSounds.java:52)

### `EndStructures`

- [ ] `EndStructures.VOXEL_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:35)
- [ ] `EndStructures.MOUNTAIN_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:36)
- [ ] `EndStructures.CAVE_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:37)
- [ ] `EndStructures.LAKE_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:38)
- [ ] `EndStructures.END_LAKE_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:39)
- [ ] `EndStructures.PAINTED_MOUNTAIN_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:40)
- [ ] `EndStructures.NBT_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:43)
- [ ] `EndStructures.END_BRIDGE_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:44)
- [ ] `EndStructures.SULPHURIC_CAVE_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betterend/registry/EndStructures.java:45)
- [ ] `EndStructures.GIANT_MOSSY_GLOWSHROOM`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:46)
- [ ] `EndStructures.MEGALAKE`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:50)
- [ ] `EndStructures.MEGALAKE_SMALL`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:54)
- [ ] `EndStructures.END_LAKE`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:58)
- [ ] `EndStructures.END_LAKE_NORMAL`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:62)
- [ ] `EndStructures.END_LAKE_RARE`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:66)
- [ ] `EndStructures.END_BRIDGE`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:70)
- [ ] `EndStructures.MOUNTAIN`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:74)
- [ ] `EndStructures.PAINTED_MOUNTAIN`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:78)
- [ ] `EndStructures.ETERNAL_PORTAL`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:82)
- [ ] `EndStructures.GIANT_ICE_STAR`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:86)
- [ ] `EndStructures.SMALL_ISLAND`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:90)
- [ ] `EndStructures.SULPHURIC_CAVE`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:94)
- [ ] `EndStructures.END_VILLAGE`: it uses `Jigsaw` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:98)
- [ ] `EndStructures.BLOSSOMING_SPIRES_HOUSE_POOL`: it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools) (org/betterx/betterend/registry/EndStructures.java:99)
- [ ] `EndStructures.BLOSSOMING_SPIRES_HOUSE`: it uses `Jigsaw` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:100)
- [ ] `EndStructures.CRASHED_SHIP_POOL`: it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools) (org/betterx/betterend/registry/EndStructures.java:102)
- [ ] `EndStructures.CRASHED_SHIP`: it uses `Jigsaw` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betterend/registry/EndStructures.java:103)
- [x] `EndStructures.register(0)` (org/betterx/betterend/registry/EndStructures.java:105)

### `EndTags`

- [ ] `EndTags.IS_END_CAVE`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:11)
- [ ] `EndTags.IS_END_HIGH_OR_MIDLAND`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:12)
- [ ] `EndTags.PEDESTALS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:13)
- [ ] `EndTags.ALLOYING_IRON`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:14)
- [ ] `EndTags.ALLOYING_GOLD`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:15)
- [ ] `EndTags.ALLOYING_COPPER`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:16)
- [ ] `EndTags.ANVIL_AETERNIUM_TOOL`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:17)
- [ ] `EndTags.ANVIL_IRON_TOOL`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:18)
- [ ] `EndTags.ANVIL_NETHERITE_TOOL`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:19)
- [ ] `EndTags.ANVIL_DIAMOND_TOOL`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:20)
- [ ] `EndTags.CRYSTALITE_SET`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:21)
- [ ] `EndTags.CRASHED_SHIP_ROTS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:22)
- [ ] `EndTags.BONEMEAL_SOURCE_DRAGON_BONE`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:23)
- [ ] `EndTags.BONEMEAL_TARGET_DRAGON_BONE`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:24)
- [ ] `EndTags.BONEMEAL_SOURCE_WATER_GRASS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:25)
- [ ] `EndTags.BONEMEAL_TARGET_WATER_GRASS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:26)
- [ ] `EndTags.INCORRECT_FOR_AETERNIUM_TOOL`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:27)
- [ ] `EndTags.SURVIVES_ON_PALLIDIUM`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:28)
- [ ] `EndTags.SURVIVES_ON_AMBER_MOSS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:29)
- [ ] `EndTags.SURVIVES_ON_BRIMSTONE`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:30)
- [ ] `EndTags.SURVIVES_ON_CHORUS_NYLIUM`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:31)
- [ ] `EndTags.SURVIVES_ON_END_BONE`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:32)
- [ ] `EndTags.SURVIVES_ON_END_MOSS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:33)
- [ ] `EndTags.SURVIVES_ON_END_STONE`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:34)
- [ ] `EndTags.SURVIVES_ON_END_STONE_OR_TREES`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:35)
- [ ] `EndTags.SURVIVES_ON_JUNGLE_MOSS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:36)
- [ ] `EndTags.SURVIVES_ON_JUNGLE_MOSS_OR_MYCELIUM`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:37)
- [ ] `EndTags.SURVIVES_ON_MOSS_OR_DUST`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:38)
- [ ] `EndTags.SURVIVES_ON_MOSS_OR_MYCELIUM`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:39)
- [ ] `EndTags.SURVIVES_ON_PINK_MOSS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:40)
- [ ] `EndTags.SURVIVES_ON_RUTISCUS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:41)
- [ ] `EndTags.SURVIVES_ON_SHADOW_GRASS`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:42)
- [ ] `EndTags.SURVIVES_ON_SULPHURIC_ROCK`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:43)
- [ ] `EndTags.SURVIVES_ON_DIRT`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:44)
- [ ] `EndTags.SCATTER_FEATURE_EXTRA_GROUND`: it uses Minecraft's `TagKey` (org/betterx/betterend/registry/EndTags.java:45)
- [x] `EndTags.register(0)` (org/betterx/betterend/registry/EndTags.java:47)

### `EndTemplates`

- [ ] `EndTemplates.LEATHER_HANDLE_ATTACHMENT`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/EndTemplates.java:7)
- [ ] `EndTemplates.TOOL_ASSEMBLY`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/EndTemplates.java:10)
- [ ] `EndTemplates.HANDLE_ATTACHMENT`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/EndTemplates.java:11)
- [ ] `EndTemplates.PLATE_UPGRADE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/EndTemplates.java:12)
- [ ] `EndTemplates.THALLASIUM_UPGRADE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/EndTemplates.java:13)
- [ ] `EndTemplates.TERMINITE_UPGRADE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/EndTemplates.java:14)
- [ ] `EndTemplates.AETERNIUM_UPGRADE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/EndTemplates.java:15)
- [ ] `EndTemplates.NETHERITE_UPGRADE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/EndTemplates.java:16)
- [x] `EndTemplates.ensureStaticallyLoaded(0)` (org/betterx/betterend/registry/EndTemplates.java:18)

### `EndVillagerProfessions`

- [ ] `EndVillagerProfessions.END_TRADER`: it uses `VillagerProfessionKey` (de.ambertation.wover.villager.api) (org/betterx/betterend/registry/EndVillagerProfessions.java:7)
- [x] `EndVillagerProfessions.EndVillagerProfessions(0)` (org/betterx/betterend/registry/EndVillagerProfessions.java:9)

### `EndVillagerTypes`

- [ ] `EndVillagerTypes.END`: it uses `VillagerTypeKey` (de.ambertation.wover.villager.api) (org/betterx/betterend/registry/EndVillagerTypes.java:7)
- [x] `EndVillagerTypes.EndVillagerTypes(0)` (org/betterx/betterend/registry/EndVillagerTypes.java:9)

### `EndBonemealFeature`

- [ ] `EndBonemealFeature.BONEMEAL_END_MOSS_NOT_GLOWING_GRASSLANDS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndBonemealFeature.java:8)
- [ ] `EndBonemealFeature.BONEMEAL_END_MOSS_GLOWING_GRASSLANDS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndBonemealFeature.java:11)
- [ ] `EndBonemealFeature.BONEMEAL_RUTISCUS_NOT_LANTERN_WOODS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndBonemealFeature.java:14)
- [ ] `EndBonemealFeature.BONEMEAL_RUTISCUS_LANTERN_WOODS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndBonemealFeature.java:17)

### `EndConfiguredBonemealFeature`

- [ ] `EndConfiguredBonemealFeature.BONEMEAL_END_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:11)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_RUTISCUS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:14)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_END_MYCELIUM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:17)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_JUNGLE_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:18)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_SANGNUM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:19)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_MOSSY_DRAGON_BONE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:20)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_MOSSY_OBSIDIAN`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:21)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_CAVE_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:22)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_CHORUS_NYLIUM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:23)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_CRYSTAL_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:24)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_SHADOW_GRASS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:25)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_PINK_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:26)
- [ ] `EndConfiguredBonemealFeature.BONEMEAL_AMBER_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java:27)

### `EndConfiguredCaveFeatures`

- [ ] `EndConfiguredCaveFeatures.SMARAGDANT_CRYSTAL`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:22)
- [ ] `EndConfiguredCaveFeatures.SMARAGDANT_CRYSTAL_SHARD`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:25)
- [ ] `EndConfiguredCaveFeatures.BIG_AURORA_CRYSTAL`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:26)
- [ ] `EndConfiguredCaveFeatures.CAVE_BUSH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:29)
- [ ] `EndConfiguredCaveFeatures.CAVE_GRASS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:30)
- [ ] `EndConfiguredCaveFeatures.RUBINEA`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:31)
- [ ] `EndConfiguredCaveFeatures.MAGNULA`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:32)
- [ ] `EndConfiguredCaveFeatures.END_STONE_STALACTITE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:33)
- [ ] `EndConfiguredCaveFeatures.END_STONE_STALAGMITE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:34)
- [ ] `EndConfiguredCaveFeatures.END_STONE_STALACTITE_CAVEMOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:35)
- [ ] `EndConfiguredCaveFeatures.END_STONE_STALAGMITE_CAVEMOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:36)
- [ ] `EndConfiguredCaveFeatures.END_STONE_WITH_CAVEMOSS_STALACTITE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:37)
- [ ] `EndConfiguredCaveFeatures.END_STONE_WITH_CAVEMOSS_STALAGMITE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:40)
- [ ] `EndConfiguredCaveFeatures.CAVE_PUMPKIN`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:43)
- [ ] `EndConfiguredCaveFeatures.STALACTITE_CLUSTER_PLAIN`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:46)
- [ ] `EndConfiguredCaveFeatures.STALACTITE_CLUSTER_CAVEMOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:47)
- [ ] `EndConfiguredCaveFeatures.CAVE_SURFACE_COAT`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:48)
- [ ] `EndConfiguredCaveFeatures.CAVE_LUSH_FLOOR_PATCH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:49)
- [ ] `EndConfiguredCaveFeatures.CAVE_LUSH_CEILING_PATCH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:50)
- [ ] `EndConfiguredCaveFeatures.lateBound(1)`: it's generic (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java:52)

### `EndConfiguredLakeFeature`

- [ ] `EndConfiguredLakeFeature.SULPHURIC_LAKE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredLakeFeature.java:11)

### `EndConfiguredVegetation`

- [ ] `EndConfiguredVegetation.DRAGON_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:24)
- [ ] `EndConfiguredVegetation.GIGANTIC_AMARANITA`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:27)
- [ ] `EndConfiguredVegetation.HELIX_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:30)
- [ ] `EndConfiguredVegetation.DRAGON_HELIX_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:33)
- [ ] `EndConfiguredVegetation.JELLYSHROOM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:36)
- [ ] `EndConfiguredVegetation.LACUGROVE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:39)
- [ ] `EndConfiguredVegetation.LUCERNIA`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:42)
- [ ] `EndConfiguredVegetation.MOSSY_GLOWSHROOM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:45)
- [ ] `EndConfiguredVegetation.PYTHADENDRON_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:48)
- [ ] `EndConfiguredVegetation.TENANEA`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:51)
- [ ] `EndConfiguredVegetation.UMBRELLA_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:54)
- [ ] `EndConfiguredVegetation.LARGE_AMARANITA`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:57)
- [ ] `EndConfiguredVegetation.LUMECORN`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:60)
- [ ] `EndConfiguredVegetation.TENANEA_BUSH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betterend/registry/features/EndConfiguredVegetation.java:61)

### `EndLakeFeatures`

- [ ] `EndLakeFeatures.DESERT_LAKE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndLakeFeatures.java:10)
- [ ] `EndLakeFeatures.SULPHURIC_LAKE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndLakeFeatures.java:12)

### `EndOreFeatures`

- [ ] `EndOreFeatures.THALLASIUM_ORE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndOreFeatures.java:9)
- [ ] `EndOreFeatures.ENDER_ORE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndOreFeatures.java:11)
- [ ] `EndOreFeatures.AMBER_ORE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndOreFeatures.java:13)
- [ ] `EndOreFeatures.DRAGON_BONE_BLOCK_ORE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndOreFeatures.java:15)
- [ ] `EndOreFeatures.VIOLECITE_LAYER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndOreFeatures.java:17)
- [ ] `EndOreFeatures.FLAVOLITE_LAYER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndOreFeatures.java:19)

### `EndPlacedCaveFeatures`

- [ ] `EndPlacedCaveFeatures.STALACTITE_CLUSTER_PLAIN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:9)
- [ ] `EndPlacedCaveFeatures.STALACTITE_CLUSTER_CAVEMOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:10)
- [ ] `EndPlacedCaveFeatures.STALAGMITE_SCATTER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:11)
- [ ] `EndPlacedCaveFeatures.STALACTITE_SCATTER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:12)
- [ ] `EndPlacedCaveFeatures.BIG_AURORA_CRYSTAL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:13)
- [ ] `EndPlacedCaveFeatures.SMARAGDANT_CRYSTAL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:14)
- [ ] `EndPlacedCaveFeatures.SMARAGDANT_SHARD_SCATTER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:15)
- [ ] `EndPlacedCaveFeatures.CAVE_SURFACE_COAT`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:16)
- [ ] `EndPlacedCaveFeatures.CAVE_LUSH_FLOOR_PATCH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:17)
- [ ] `EndPlacedCaveFeatures.CAVE_LUSH_CEILING_PATCH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:18)
- [ ] `EndPlacedCaveFeatures.CAVE_GRASS_SCATTER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:19)
- [ ] `EndPlacedCaveFeatures.CAVE_BUSH_SCATTER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:20)
- [ ] `EndPlacedCaveFeatures.CAVE_PUMPKIN_PLACED`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:21)
- [ ] `EndPlacedCaveFeatures.key(1)`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java:23)

### `EndTerrainFeatures`

- [ ] `EndTerrainFeatures.SURFACE_VENT`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:9)
- [ ] `EndTerrainFeatures.SULPHUR_HILL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:11)
- [ ] `EndTerrainFeatures.SULPHUR_SPIKE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:13)
- [ ] `EndTerrainFeatures.SULPHUR_SPIKE_HANGING`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:15)
- [ ] `EndTerrainFeatures.OBSIDIAN_PILLAR_BASEMENT`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:17)
- [ ] `EndTerrainFeatures.OBSIDIAN_BOULDER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:19)
- [ ] `EndTerrainFeatures.FALLEN_PILLAR`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:21)
- [ ] `EndTerrainFeatures.UMBRALITH_ARCH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:23)
- [ ] `EndTerrainFeatures.THIN_UMBRALITH_ARCH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:25)
- [ ] `EndTerrainFeatures.SILK_MOTH_NEST`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:27)
- [ ] `EndTerrainFeatures.SPIRE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:29)
- [ ] `EndTerrainFeatures.FLOATING_SPIRE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:31)
- [ ] `EndTerrainFeatures.GEYSER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:33)
- [ ] `EndTerrainFeatures.ICE_STAR`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:34)
- [ ] `EndTerrainFeatures.ICE_STAR_SMALL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:36)
- [ ] `EndTerrainFeatures.POND_WITH_WATERFALL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:38)
- [ ] `EndTerrainFeatures.BIOME_ISLAND`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndTerrainFeatures.java:40)

### `EndVegetationFeatures`

- [ ] `EndVegetationFeatures.MOSSY_GLOWSHROOM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:10)
- [ ] `EndVegetationFeatures.PYTHADENDRON_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:11)
- [ ] `EndVegetationFeatures.LACUGROVE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:12)
- [ ] `EndVegetationFeatures.DRAGON_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:13)
- [ ] `EndVegetationFeatures.TENANEA`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:14)
- [ ] `EndVegetationFeatures.HELIX_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:15)
- [ ] `EndVegetationFeatures.DRAGON_HELIX_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:16)
- [ ] `EndVegetationFeatures.UMBRELLA_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:17)
- [ ] `EndVegetationFeatures.JELLYSHROOM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:18)
- [ ] `EndVegetationFeatures.GIGANTIC_AMARANITA`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:19)
- [ ] `EndVegetationFeatures.LUCERNIA`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:20)
- [ ] `EndVegetationFeatures.TENANEA_BUSH`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:21)
- [ ] `EndVegetationFeatures.LUMECORN`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:22)
- [ ] `EndVegetationFeatures.LARGE_AMARANITA`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:23)
- [ ] `EndVegetationFeatures.NEON_CACTUS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:24)
- [ ] `EndVegetationFeatures.PYTHADENDRON_BUSH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:25)
- [ ] `EndVegetationFeatures.DRAGON_TREE_BUSH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:26)
- [ ] `EndVegetationFeatures.LUCERNIA_BUSH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:27)
- [ ] `EndVegetationFeatures.LUCERNIA_BUSH_RARE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:28)
- [ ] `EndVegetationFeatures.DENSE_VINE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:29)
- [ ] `EndVegetationFeatures.TWISTED_VINE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:30)
- [ ] `EndVegetationFeatures.BULB_VINE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:31)
- [ ] `EndVegetationFeatures.JUNGLE_VINE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:32)
- [ ] `EndVegetationFeatures.BLUE_VINE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:33)
- [ ] `EndVegetationFeatures.SMALL_JELLYSHROOM_CEIL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:34)
- [ ] `EndVegetationFeatures.PURPLE_POLYPORE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:35)
- [ ] `EndVegetationFeatures.AURANT_POLYPORE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:36)
- [ ] `EndVegetationFeatures.TAIL_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:37)
- [ ] `EndVegetationFeatures.CYAN_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:38)
- [ ] `EndVegetationFeatures.TAIL_MOSS_WOOD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:39)
- [ ] `EndVegetationFeatures.CYAN_MOSS_WOOD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:40)
- [ ] `EndVegetationFeatures.TWISTED_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:41)
- [ ] `EndVegetationFeatures.TWISTED_MOSS_WOOD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:42)
- [ ] `EndVegetationFeatures.BULB_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:43)
- [ ] `EndVegetationFeatures.BULB_MOSS_WOOD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:44)
- [ ] `EndVegetationFeatures.SMALL_JELLYSHROOM_WALL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:45)
- [ ] `EndVegetationFeatures.SMALL_JELLYSHROOM_WOOD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:46)
- [ ] `EndVegetationFeatures.JUNGLE_FERN_WOOD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:47)
- [ ] `EndVegetationFeatures.RUSCUS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:48)
- [ ] `EndVegetationFeatures.RUSCUS_WOOD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:49)
- [ ] `EndVegetationFeatures.FILALUX`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:50)
- [ ] `EndVegetationFeatures.BUBBLE_CORAL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:51)
- [ ] `EndVegetationFeatures.BUBBLE_CORAL_RARE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:52)
- [ ] `EndVegetationFeatures.END_LILY`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:53)
- [ ] `EndVegetationFeatures.END_LILY_RARE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:54)
- [ ] `EndVegetationFeatures.END_LOTUS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:55)
- [ ] `EndVegetationFeatures.END_LOTUS_LEAF`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:56)
- [ ] `EndVegetationFeatures.HYDRALUX`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:57)
- [ ] `EndVegetationFeatures.POND_ANEMONE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:58)
- [ ] `EndVegetationFeatures.MENGER_SPONGE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:59)
- [ ] `EndVegetationFeatures.FLAMAEA`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:60)
- [ ] `EndVegetationFeatures.CHARNIA_RED`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:61)
- [ ] `EndVegetationFeatures.CHARNIA_PURPLE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:62)
- [ ] `EndVegetationFeatures.CHARNIA_CYAN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:63)
- [ ] `EndVegetationFeatures.CHARNIA_LIGHT_BLUE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:64)
- [ ] `EndVegetationFeatures.CHARNIA_ORANGE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:65)
- [ ] `EndVegetationFeatures.CHARNIA_GREEN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:66)
- [ ] `EndVegetationFeatures.CHARNIA_RED_RARE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:67)
- [ ] `EndVegetationFeatures.UMBRELLA_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:68)
- [ ] `EndVegetationFeatures.CREEPING_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:69)
- [ ] `EndVegetationFeatures.CHORUS_GRASS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:70)
- [ ] `EndVegetationFeatures.CRYSTAL_GRASS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:71)
- [ ] `EndVegetationFeatures.CRYSTAL_MOSS_COVER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:72)
- [ ] `EndVegetationFeatures.SHADOW_PLANT`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:73)
- [ ] `EndVegetationFeatures.MURKWEED`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:74)
- [ ] `EndVegetationFeatures.NEEDLEGRASS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:75)
- [ ] `EndVegetationFeatures.SHADOW_BERRY`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:76)
- [ ] `EndVegetationFeatures.BUSHY_GRASS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:77)
- [ ] `EndVegetationFeatures.BUSHY_GRASS_WG`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:78)
- [ ] `EndVegetationFeatures.AMBER_GRASS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:79)
- [ ] `EndVegetationFeatures.TWISTED_UMBRELLA_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:80)
- [ ] `EndVegetationFeatures.JUNGLE_GRASS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:81)
- [ ] `EndVegetationFeatures.SMALL_JELLYSHROOM_FLOOR`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:82)
- [ ] `EndVegetationFeatures.BLOSSOM_BERRY`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:83)
- [ ] `EndVegetationFeatures.BLOOMING_COOKSONIA`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:84)
- [ ] `EndVegetationFeatures.SALTEAGO`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:85)
- [ ] `EndVegetationFeatures.VAIOLUSH_FERN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:86)
- [ ] `EndVegetationFeatures.FRACTURN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:87)
- [ ] `EndVegetationFeatures.UMBRELLA_MOSS_RARE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:88)
- [ ] `EndVegetationFeatures.CREEPING_MOSS_RARE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:89)
- [ ] `EndVegetationFeatures.TWISTED_UMBRELLA_MOSS_RARE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:90)
- [ ] `EndVegetationFeatures.ORANGO`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:91)
- [ ] `EndVegetationFeatures.AERIDIUM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:92)
- [ ] `EndVegetationFeatures.LUTEBUS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:93)
- [ ] `EndVegetationFeatures.LAMELLARIUM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:94)
- [ ] `EndVegetationFeatures.SMALL_AMARANITA`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:95)
- [ ] `EndVegetationFeatures.AMARANITA_PATCH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:96)
- [ ] `EndVegetationFeatures.GLOBULAGUS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:97)
- [ ] `EndVegetationFeatures.CLAWFERN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:98)
- [ ] `EndVegetationFeatures.BOLUX_MUSHROOM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:99)
- [ ] `EndVegetationFeatures.CHORUS_MUSHROOM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:100)
- [ ] `EndVegetationFeatures.AMBER_ROOT`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:101)
- [ ] `EndVegetationFeatures.INFLEXIA`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:102)
- [ ] `EndVegetationFeatures.FLAMMALIX`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:103)
- [ ] `EndVegetationFeatures.LANCELEAF`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:104)
- [ ] `EndVegetationFeatures.GLOW_PILLAR`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:105)
- [ ] `EndVegetationFeatures.createVegetationKey(1)`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/EndVegetationFeatures.java:107)

### `GlimmerwildVegetationFeatures`

- [ ] `GlimmerwildVegetationFeatures.WOOL_FLOWER_MEADOW`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:13)
- [ ] `GlimmerwildVegetationFeatures.UMBRELLA_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:14)
- [ ] `GlimmerwildVegetationFeatures.TWISTED_UMBRELLA_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:15)
- [ ] `GlimmerwildVegetationFeatures.CREEPING_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:16)
- [ ] `GlimmerwildVegetationFeatures.BLOOMING_COOKSONIA`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:17)
- [ ] `GlimmerwildVegetationFeatures.SALTEAGO`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:18)
- [ ] `GlimmerwildVegetationFeatures.VAIOLUSH_FERN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:19)
- [ ] `GlimmerwildVegetationFeatures.FRACTURN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:20)
- [ ] `GlimmerwildVegetationFeatures.JUNGLE_GRASS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:21)
- [ ] `GlimmerwildVegetationFeatures.ORANGO`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:22)
- [ ] `GlimmerwildVegetationFeatures.AERIDIUM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:23)
- [ ] `GlimmerwildVegetationFeatures.LUTEBUS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:24)
- [ ] `GlimmerwildVegetationFeatures.LAMELLARIUM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:25)
- [ ] `GlimmerwildVegetationFeatures.AMBER_GRASS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:26)
- [ ] `GlimmerwildVegetationFeatures.FLAMMALIX`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:27)
- [ ] `GlimmerwildVegetationFeatures.ALL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:28)
- [ ] `GlimmerwildVegetationFeatures.createKey(1)`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java:45)

### `EndDiscItems`

- [ ] `EndDiscItems.MUSIC_DISC_STRANGE_AND_ALIEN`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndDiscItems.java:8)
- [ ] `EndDiscItems.MUSIC_DISC_GRASPING_AT_STARS`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndDiscItems.java:9)
- [ ] `EndDiscItems.MUSIC_DISC_ENDSEEKER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndDiscItems.java:10)
- [ ] `EndDiscItems.MUSIC_DISC_EO_DRACONA`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndDiscItems.java:11)
- [x] `EndDiscItems.ensureLoaded(0)` (org/betterx/betterend/registry/item/EndDiscItems.java:13)

### `EndEquipmentItems`

- [ ] `EndEquipmentItems.ATTACK_KNOCKBACK_MODIFIER_ID`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betterend/registry/item/EndEquipmentItems.java:17)
- [ ] `EndEquipmentItems.AETERNIUM_SET`: it uses AeterniumSet, which isn't translated (it extends `ToolsWithHeadsSet`) (org/betterx/betterend/registry/item/EndEquipmentItems.java:18)
- [ ] `EndEquipmentItems.CRYSTALITE_HELMET`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:19)
- [ ] `EndEquipmentItems.CRYSTALITE_CHESTPLATE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:20)
- [ ] `EndEquipmentItems.CRYSTALITE_LEGGINGS`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:21)
- [ ] `EndEquipmentItems.CRYSTALITE_BOOTS`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:22)
- [ ] `EndEquipmentItems.ARMORED_ELYTRA`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:23)
- [ ] `EndEquipmentItems.CRYSTALITE_ELYTRA`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:24)
- [ ] `EndEquipmentItems.IRON_HAMMER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:25)
- [ ] `EndEquipmentItems.GOLDEN_HAMMER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:26)
- [ ] `EndEquipmentItems.DIAMOND_HAMMER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:27)
- [ ] `EndEquipmentItems.NETHERITE_HAMMER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndEquipmentItems.java:28)
- [x] `EndEquipmentItems.ensureLoaded(0)` (org/betterx/betterend/registry/item/EndEquipmentItems.java:30)

### `EndFoodItems`

- [ ] `EndFoodItems.SHADOW_BERRY_RAW`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:9)
- [ ] `EndFoodItems.SHADOW_BERRY_COOKED`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:10)
- [ ] `EndFoodItems.END_FISH_RAW`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:11)
- [ ] `EndFoodItems.END_FISH_COOKED`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:12)
- [ ] `EndFoodItems.BUCKET_END_FISH`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:13)
- [ ] `EndFoodItems.BUCKET_CUBOZOA`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:14)
- [ ] `EndFoodItems.SWEET_BERRY_JELLY`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:15)
- [ ] `EndFoodItems.SHADOW_BERRY_JELLY`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:16)
- [ ] `EndFoodItems.BLOSSOM_BERRY_JELLY`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:17)
- [ ] `EndFoodItems.BLOSSOM_BERRY`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:18)
- [ ] `EndFoodItems.AMBER_ROOT_RAW`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:19)
- [ ] `EndFoodItems.CHORUS_MUSHROOM_RAW`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:20)
- [ ] `EndFoodItems.CHORUS_MUSHROOM_COOKED`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:21)
- [ ] `EndFoodItems.BOLUX_MUSHROOM_COOKED`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:22)
- [ ] `EndFoodItems.CAVE_PUMPKIN_PIE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:23)
- [ ] `EndFoodItems.UMBRELLA_CLUSTER_JUICE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndFoodItems.java:24)
- [x] `EndFoodItems.ensureLoaded(0)` (org/betterx/betterend/registry/item/EndFoodItems.java:26)

### `EndItemFactories`

- [ ] `EndItemFactories.BUCKET_END_FISH`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:17)
- [ ] `EndItemFactories.BUCKET_CUBOZOA`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:20)
- [ ] `EndItemFactories.CRYSTALITE_HELMET`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:23)
- [ ] `EndItemFactories.CRYSTALITE_CHESTPLATE`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:26)
- [ ] `EndItemFactories.CRYSTALITE_LEGGINGS`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:29)
- [ ] `EndItemFactories.CRYSTALITE_BOOTS`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:32)
- [ ] `EndItemFactories.ARMORED_ELYTRA`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:35)
- [ ] `EndItemFactories.CRYSTALITE_ELYTRA`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:38)
- [ ] `EndItemFactories.GUIDEBOOK`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndItemFactories.java:41)
- [x] `EndItemFactories.EndItemFactories(0)` (org/betterx/betterend/registry/item/EndItemFactories.java:45)
- [x] `EndItemFactories.ensureLoaded(0)` (org/betterx/betterend/registry/item/EndItemFactories.java:48)

### `EndResourceItems`

- [ ] `EndResourceItems.ENDER_DUST`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:8)
- [ ] `EndResourceItems.ENDER_SHARD`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:9)
- [ ] `EndResourceItems.END_LILY_LEAF`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:10)
- [ ] `EndResourceItems.END_LILY_LEAF_DRIED`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:11)
- [ ] `EndResourceItems.CRYSTAL_SHARDS`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:12)
- [ ] `EndResourceItems.RAW_AMBER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:13)
- [ ] `EndResourceItems.AMBER_GEM`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:14)
- [ ] `EndResourceItems.GLOWING_BULB`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:15)
- [ ] `EndResourceItems.CRYSTALLINE_SULPHUR`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:16)
- [ ] `EndResourceItems.HYDRALUX_PETAL`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:17)
- [ ] `EndResourceItems.GELATINE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:18)
- [ ] `EndResourceItems.ETERNAL_CRYSTAL`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:19)
- [ ] `EndResourceItems.ENCHANTED_PETAL`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:20)
- [ ] `EndResourceItems.LEATHER_STRIPE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:21)
- [ ] `EndResourceItems.LEATHER_WRAPPED_STICK`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:22)
- [ ] `EndResourceItems.SILK_FIBER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:23)
- [ ] `EndResourceItems.LUMECORN_ROD`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:24)
- [ ] `EndResourceItems.SILK_MOTH_MATRIX`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:25)
- [ ] `EndResourceItems.ENCHANTED_MEMBRANE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betterend/registry/item/EndResourceItems.java:26)
- [x] `EndResourceItems.ensureLoaded(0)` (org/betterx/betterend/registry/item/EndResourceItems.java:28)

### `EternalRitual`: it uses Minecraft's `Level`

- [ ] `EternalRitual.PEDESTAL_POSITIONS`: it uses `Point` (java.awt) (org/betterx/betterend/rituals/EternalRitual.java:45)
- [ ] `EternalRitual.PEDESTAL`: it uses Minecraft's `Block` (org/betterx/betterend/rituals/EternalRitual.java:48)
- [ ] `EternalRitual.ACTIVE`: it uses Minecraft's `BooleanProperty` (org/betterx/betterend/rituals/EternalRitual.java:49)
- [ ] `EternalRitual.pedestalOffsets(0)`: it uses `Point` (java.awt) (org/betterx/betterend/rituals/EternalRitual.java:58)
- [ ] `EternalRitual.EternalRitual(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:62)
- [ ] `EternalRitual.EternalRitual(2)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:66)
- [ ] `EternalRitual.getCenter(0)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:71)
- [ ] `EternalRitual.getAxis(0)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:75)
- [ ] `EternalRitual.setWorld(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:79)
- [ ] `EternalRitual.getWorld(0)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:83)
- [ ] `EternalRitual.getTargetWorldId(0)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:87)
- [ ] `EternalRitual.isInvalid(0)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:92)
- [ ] `EternalRitual.checkStructure(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:96)
- [ ] `EternalRitual.updateActiveStateOnPedestals(0)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:137)
- [ ] `EternalRitual.updateActiveStateOnPedestals(6)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/rituals/EternalRitual.java:146)
- [ ] `EternalRitual.checkFrame(2)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:183)
- [ ] `EternalRitual.isActive(0)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:199)
- [ ] `EternalRitual.willActivate(0)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:203)
- [ ] `EternalRitual.activatePortal(2)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:207)
- [ ] `EternalRitual.initPortal(3)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:249)
- [ ] `EternalRitual.doEffects(2)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:265)
- [ ] `EternalRitual.activatePortal(3)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:286)
- [ ] `EternalRitual.disablePortal(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:326)
- [ ] `EternalRitual.removePortal(2)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:336)
- [ ] `EternalRitual.getTargetWorld(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:367)
- [ ] `EternalRitual.configure(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:373)
- [ ] `EternalRitual.hasPedestal(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:443)
- [ ] `EternalRitual.isActive(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:447)
- [ ] `EternalRitual.toTag(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:468)
- [ ] `EternalRitual.fromTag(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:489)
- [ ] `EternalRitual.equals(1)`: its class isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:502)
- [ ] `EternalRitual.findRitualForActivePortal(2)`: it uses EternalRitual, which isn't translated (it uses Minecraft's `Level`) (org/betterx/betterend/rituals/EternalRitual.java:514)
- [ ] `EternalRitual.isActivePortalBlock(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/rituals/EternalRitual.java:549)

### `InfusionRitual`: it implements Minecraft's `Container`

- [ ] `InfusionRitual.PEDESTALS_MAP`: it uses `Point` (java.awt) (org/betterx/betterend/rituals/InfusionRitual.java:32)
- [ ] `InfusionRitual.InfusionRitual(3)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:45)
- [ ] `InfusionRitual.configure(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:51)
- [ ] `InfusionRitual.checkRecipe(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:68)
- [ ] `InfusionRitual.updateRecipe(1)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:95)
- [ ] `InfusionRitual.resetTimer(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:102)
- [ ] `InfusionRitual.reset(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:107)
- [ ] `InfusionRitual.tick(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:114)
- [ ] `InfusionRitual.canPlaceItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:151)
- [ ] `InfusionRitual.isValid(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:155)
- [ ] `InfusionRitual.hasRecipe(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:159)
- [ ] `InfusionRitual.setLocation(2)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:163)
- [ ] `InfusionRitual.clearContent(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:169)
- [ ] `InfusionRitual.getContainerSize(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:176)
- [ ] `InfusionRitual.isEmpty(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:180)
- [ ] `InfusionRitual.getItem(1)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:184)
- [ ] `InfusionRitual.removeItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:193)
- [ ] `InfusionRitual.removeItemNoUpdate(1)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:198)
- [ ] `InfusionRitual.setItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:207)
- [ ] `InfusionRitual.setChanged(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:217)
- [ ] `InfusionRitual.setDirty(0)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:224)
- [ ] `InfusionRitual.stillValid(1)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:228)
- [ ] `InfusionRitual.fromTag(1)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:232)
- [ ] `InfusionRitual.toTag(1)`: its class isn't translated (it implements Minecraft's `Container`) (org/betterx/betterend/rituals/InfusionRitual.java:240)
- [ ] `InfusionRitual.getMap(0)`: it uses `Point` (java.awt) (org/betterx/betterend/rituals/InfusionRitual.java:250)
- [ ] `InfusionRitual.socketPos(2)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/rituals/InfusionRitual.java:254)
- [ ] `InfusionRitual.socketsPresent(2)`: it uses Minecraft's `BlockGetter` (org/betterx/betterend/rituals/InfusionRitual.java:259)
- [ ] `InfusionRitual.socketAt(2)`: it uses Socket, which isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/rituals/InfusionRitual.java:269)
- [ ] `InfusionRitual.allSocketsPresent(2)`: it uses Minecraft's `BlockGetter` (org/betterx/betterend/rituals/InfusionRitual.java:282)

### `InfusionRitual.InfusionInput`: it implements Minecraft's `RecipeInput`

- [ ] `InfusionRitual.InfusionInput.getItem(1)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (org/betterx/betterend/rituals/InfusionRitual.java:293)
- [ ] `InfusionRitual.InfusionInput.size(0)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (org/betterx/betterend/rituals/InfusionRitual.java:298)

### `InfusionRitual.Socket`: it uses Minecraft's `BlockPos`

- [ ] `InfusionRitual.Socket.Socket(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/rituals/InfusionRitual.java:303)
- [ ] `InfusionRitual.Socket.isNorth(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/rituals/InfusionRitual.java:304)
- [ ] `InfusionRitual.Socket.pedestal(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/rituals/InfusionRitual.java:303)
- [ ] `InfusionRitual.Socket.index(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betterend/rituals/InfusionRitual.java:303)

### `CreativeTabs`

- [ ] `CreativeTabs.register(0)`: unknown name `de` (org/betterx/betterend/tab/CreativeTabs.java:30)
- [ ] `CreativeTabs.inTabOrder(1)`: it uses `Stream` (java.util.stream) (org/betterx/betterend/tab/CreativeTabs.java:48)
- [ ] `CreativeTabs.keepTogether(2)`: it uses Minecraft's `BuiltInRegistries`.ITEM (org/betterx/betterend/tab/CreativeTabs.java:84)
- [ ] `CreativeTabs.range(3)`: it uses Minecraft's `BuiltInRegistries`.ITEM (org/betterx/betterend/tab/CreativeTabs.java:93)
- [ ] `CreativeTabs.isPlantLike(1)`: it uses Minecraft's `Block` (org/betterx/betterend/tab/CreativeTabs.java:99)

### `BlockFixer`

- [ ] `BlockFixer.AIR`: it uses Minecraft's `BlockState` (org/betterx/betterend/util/BlockFixer.java:31)
- [ ] `BlockFixer.WATER`: it uses Minecraft's `BlockState` (org/betterx/betterend/util/BlockFixer.java:32)
- [ ] `BlockFixer.fixBlocks(3)`: it uses Minecraft's `LevelAccessor` (org/betterx/betterend/util/BlockFixer.java:34)
- [ ] `BlockFixer.fixBlocksInZone(4)`: it uses Minecraft's `LevelAccessor` (org/betterx/betterend/util/BlockFixer.java:38)
- [ ] `BlockFixer.fixBlocks(4)`: it uses Minecraft's `LevelAccessor` (org/betterx/betterend/util/BlockFixer.java:57)
- [ ] `BlockFixer.getAirOrFluid(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/util/BlockFixer.java:243)
- [ ] `BlockFixer.setWithoutUpdate(3)`: it uses Minecraft's `LevelAccessor` (org/betterx/betterend/util/BlockFixer.java:247)

### `BonemealPlants`

- [ ] `BonemealPlants.init(0)`: it uses `BonemealAPI` (org.betterx.bclib.api.v3.bonemeal).INSTANCE (org/betterx/betterend/util/BonemealPlants.java:11)

### `DebugHelpers`

- [ ] `DebugHelpers.ITEMS`: it uses DebugItem, which isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/util/DebugHelpers.java:19)
- [x] `DebugHelpers.ensureLoaded(0)` (org/betterx/betterend/util/DebugHelpers.java:21)
- [ ] `DebugHelpers.items(0)`: it uses DebugItem, which isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/util/DebugHelpers.java:24)
- [ ] `DebugHelpers.add(2)`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/DebugHelpers.java:28)

### `DebugHelpers.DebugItem`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify)

- [ ] `DebugHelpers.DebugItem.DebugItem(2)`: its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/util/DebugHelpers.java:72)
- [ ] `DebugHelpers.DebugItem.path(0)`: its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/util/DebugHelpers.java:72)
- [ ] `DebugHelpers.DebugItem.factory(0)`: its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betterend/util/DebugHelpers.java:72)

### `GlobalState`: it uses Minecraft's `MutableBlockPos`

- [ ] `GlobalState.STATE`: it uses `ThreadLocal` (org/betterx/betterend/util/GlobalState.java:6)
- [ ] `GlobalState.stateForThread(0)`: it uses GlobalState, which isn't translated (it uses Minecraft's `MutableBlockPos`) (org/betterx/betterend/util/GlobalState.java:9)

### `GrowthPlacer`: it uses Minecraft's `LevelAccessor`

- [ ] `GrowthPlacer.GrowthPlacer(1)`: its class isn't translated (it uses Minecraft's `LevelAccessor`) (org/betterx/betterend/util/GrowthPlacer.java:15)
- [ ] `GrowthPlacer.set(2)`: its class isn't translated (it uses Minecraft's `LevelAccessor`) (org/betterx/betterend/util/GrowthPlacer.java:20)
- [ ] `GrowthPlacer.notifyNeighbours(0)`: its class isn't translated (it uses Minecraft's `LevelAccessor`) (org/betterx/betterend/util/GrowthPlacer.java:27)

### `LangUtil`

- [x] `LangUtil.CONFIG_ELEMENT` (org/betterx/betterend/util/LangUtil.java:8)
- [x] `LangUtil.LangUtil(1)` (org/betterx/betterend/util/LangUtil.java:11)
- [x] `LangUtil.setElement(1)` (org/betterx/betterend/util/LangUtil.java:15)
- [ ] `LangUtil.getString(1)`: it calls LangUtil.getString, which isn't translated (org/betterx/betterend/util/LangUtil.java:19)
- [ ] `LangUtil.getText(1)`: it uses Minecraft's `MutableComponent` (org/betterx/betterend/util/LangUtil.java:23)
- [ ] `LangUtil.translate(1)`: it uses `Object` (org/betterx/betterend/util/LangUtil.java:27)
- [ ] `LangUtil.getString(2)`: it calls LangUtil.translate, which isn't translated (org/betterx/betterend/util/LangUtil.java:31)
- [ ] `LangUtil.getText(2)`: a translated text with a computed key (org/betterx/betterend/util/LangUtil.java:35)

### `LootTableUtil`

- [ ] `LootTableUtil.VILLAGE_LOOT`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:12)
- [ ] `LootTableUtil.VILLAGE_TEMPLATE_LOOT`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:13)
- [ ] `LootTableUtil.VILLAGE_BONUS_LOOT`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:14)
- [ ] `LootTableUtil.COMMON`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:15)
- [ ] `LootTableUtil.FOGGY_MUSHROOMLAND`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:16)
- [ ] `LootTableUtil.CHORUS_FOREST`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:17)
- [ ] `LootTableUtil.SHADOW_FOREST`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:18)
- [ ] `LootTableUtil.LANTERN_WOODS`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:19)
- [ ] `LootTableUtil.UMBRELLA_JUNGLE`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:20)
- [ ] `LootTableUtil.BIOME_CHEST`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:21)
- [ ] `LootTableUtil.END_CITY_EXTRA`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:22)
- [ ] `LootTableUtil.FISHING_FISH`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:23)
- [ ] `LootTableUtil.FISHING_TREASURE`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:24)
- [ ] `LootTableUtil.FISHING_JUNK`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:25)
- [ ] `LootTableUtil.getTable(1)`: it uses Minecraft's `ResourceKey` (org/betterx/betterend/util/LootTableUtil.java:27)

### `WorldgenDebug`

- [x] `WorldgenDebug.ENABLED` (org/betterx/betterend/util/WorldgenDebug.java:6)
- [x] `WorldgenDebug.WorldgenDebug(0)` (org/betterx/betterend/util/WorldgenDebug.java:8)
- [ ] `WorldgenDebug.log(2)`: it uses `Object` (org/betterx/betterend/util/WorldgenDebug.java:11)

### `BiomeIceStarfield`: it extends `Config`

- [ ] `BiomeIceStarfield.hasCaves(0)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/air/BiomeIceStarfield.java:11)
- [ ] `BiomeIceStarfield.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/air/BiomeIceStarfield.java:16)

### `FlowerIsletsBiome`: it extends `Config`

- [ ] `FlowerIsletsBiome.hasCaves(0)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/air/FlowerIsletsBiome.java:16)
- [ ] `FlowerIsletsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/air/FlowerIsletsBiome.java:21)
- [ ] `FlowerIsletsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/air/FlowerIsletsBiome.java:49)

### `WaterfallPondsBiome`: it extends `Config`

- [ ] `WaterfallPondsBiome.hasCaves(0)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/air/WaterfallPondsBiome.java:17)
- [ ] `WaterfallPondsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/air/WaterfallPondsBiome.java:22)
- [ ] `WaterfallPondsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/air/WaterfallPondsBiome.java:50)

### `EmptyAuroraCaveBiome`: it extends `Config`

- [ ] `EmptyAuroraCaveBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/biome/cave/EmptyAuroraCaveBiome.java:17)
- [ ] `EmptyAuroraCaveBiome.instantiateBiome(11)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptyAuroraCaveBiome.java:19)
- [ ] `EmptyAuroraCaveBiome.EmptyAuroraCaveBiome(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptyAuroraCaveBiome.java:37)
- [ ] `EmptyAuroraCaveBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptyAuroraCaveBiome.java:41)

### `EmptyAuroraCaveBiome.Biome`: it extends `EndCaveBiome`

- [ ] `EmptyAuroraCaveBiome.Biome.codec(0)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/EmptyAuroraCaveBiome.java:54)
- [ ] `EmptyAuroraCaveBiome.Biome.Biome(11)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/EmptyAuroraCaveBiome.java:59)

### `EmptyEndCaveBiome`: it extends `Config`

- [ ] `EmptyEndCaveBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/biome/cave/EmptyEndCaveBiome.java:16)
- [ ] `EmptyEndCaveBiome.EmptyEndCaveBiome(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptyEndCaveBiome.java:18)
- [ ] `EmptyEndCaveBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptyEndCaveBiome.java:22)
- [ ] `EmptyEndCaveBiome.instantiateBiome(11)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptyEndCaveBiome.java:30)

### `EmptyEndCaveBiome.Biome`: it extends `EndCaveBiome`

- [ ] `EmptyEndCaveBiome.Biome.codec(0)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/EmptyEndCaveBiome.java:49)
- [ ] `EmptyEndCaveBiome.Biome.Biome(11)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/EmptyEndCaveBiome.java:54)

### `EmptySmaragdantCaveBiome`: it extends `Config`

- [ ] `EmptySmaragdantCaveBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/biome/cave/EmptySmaragdantCaveBiome.java:17)
- [ ] `EmptySmaragdantCaveBiome.EmptySmaragdantCaveBiome(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptySmaragdantCaveBiome.java:19)
- [ ] `EmptySmaragdantCaveBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptySmaragdantCaveBiome.java:23)
- [ ] `EmptySmaragdantCaveBiome.instantiateBiome(11)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/EmptySmaragdantCaveBiome.java:36)

### `EmptySmaragdantCaveBiome.Biome`: it extends `EndCaveBiome`

- [ ] `EmptySmaragdantCaveBiome.Biome.codec(0)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/EmptySmaragdantCaveBiome.java:57)
- [ ] `EmptySmaragdantCaveBiome.Biome.Biome(11)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/EmptySmaragdantCaveBiome.java:62)

### `EndCaveBiome`: it extends `EndBiome` (org.betterx.betterend.world.biome)

- [ ] `EndCaveBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:23)
- [ ] `EndCaveBiome.simpleCaveBiomeCodec(1)`: it's generic (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:25)
- [ ] `EndCaveBiome.codec(0)`: its class isn't translated (it extends `EndBiome` (org.betterx.betterend.world.biome)) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:35)
- [ ] `EndCaveBiome.EndCaveBiome(11)`: its class isn't translated (it extends `EndBiome` (org.betterx.betterend.world.biome)) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:40)
- [ ] `EndCaveBiome.getCeil(1)`: its class isn't translated (it extends `EndBiome` (org.betterx.betterend.world.biome)) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:56)
- [ ] `EndCaveBiome.getWall(1)`: its class isn't translated (it extends `EndBiome` (org.betterx.betterend.world.biome)) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:60)

### `EndCaveBiome.Config`: it's generic

- [ ] `EndCaveBiome.Config.Config(1)`: its class isn't translated (it's generic) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:65)
- [ ] `EndCaveBiome.Config.addCustomBuildData(1)`: its class isn't translated (it's generic) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:68)
- [ ] `EndCaveBiome.Config.hasCaves(0)`: its class isn't translated (it's generic) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:75)
- [ ] `EndCaveBiome.Config.hasReturnGateway(0)`: its class isn't translated (it's generic) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:80)
- [ ] `EndCaveBiome.Config.instantiateBiome(11)`: its class isn't translated (it's generic) (org/betterx/betterend/world/biome/cave/EndCaveBiome.java:85)

### `JadeCaveBiome`: it extends `Config`

- [ ] `JadeCaveBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:22)
- [ ] `JadeCaveBiome.JadeCaveBiome(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:24)
- [ ] `JadeCaveBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:28)
- [ ] `JadeCaveBiome.instantiateBiome(11)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:35)

### `JadeCaveBiome.Biome`: it extends `EndCaveBiome`

- [ ] `JadeCaveBiome.Biome.WALL_NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:54)
- [ ] `JadeCaveBiome.Biome.DEPTH_NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:55)
- [ ] `JadeCaveBiome.Biome.JADE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:56)
- [ ] `JadeCaveBiome.Biome.codec(0)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:58)
- [ ] `JadeCaveBiome.Biome.Biome(11)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:63)
- [ ] `JadeCaveBiome.Biome.getWall(1)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java:79)

### `LushAuroraCaveBiome`: it extends `Config`

- [ ] `LushAuroraCaveBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java:22)
- [ ] `LushAuroraCaveBiome.LushAuroraCaveBiome(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java:24)
- [ ] `LushAuroraCaveBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java:28)
- [ ] `LushAuroraCaveBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java:45)
- [ ] `LushAuroraCaveBiome.instantiateBiome(11)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java:55)

### `LushAuroraCaveBiome.Biome`: it extends `EndCaveBiome`

- [ ] `LushAuroraCaveBiome.Biome.codec(0)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java:74)
- [ ] `LushAuroraCaveBiome.Biome.Biome(11)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java:79)
- [ ] `LushAuroraCaveBiome.Biome.getCeil(1)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java:95)

### `LushSmaragdantCaveBiome`: it extends `Config`

- [ ] `LushSmaragdantCaveBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java:20)
- [ ] `LushSmaragdantCaveBiome.LushSmaragdantCaveBiome(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java:22)
- [ ] `LushSmaragdantCaveBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java:26)
- [ ] `LushSmaragdantCaveBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java:39)
- [ ] `LushSmaragdantCaveBiome.instantiateBiome(11)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java:49)

### `LushSmaragdantCaveBiome.Biome`: it extends `EndCaveBiome`

- [ ] `LushSmaragdantCaveBiome.Biome.codec(0)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java:68)
- [ ] `LushSmaragdantCaveBiome.Biome.Biome(11)`: its class isn't translated (it extends `EndCaveBiome`) (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java:73)

### `EndBiome`: it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)

- [ ] `EndBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/biome/EndBiome.java:34)
- [ ] `EndBiome.EndBiome(11)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:42)
- [ ] `EndBiome.datagenSetup(1)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:60)
- [ ] `EndBiome.codec(0)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:63)
- [ ] `EndBiome.setHasCaves(1)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:67)
- [ ] `EndBiome.hasCaves(0)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:71)
- [ ] `EndBiome.getTopMaterial(0)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:75)
- [ ] `EndBiome.getUnderMaterial(0)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:79)
- [ ] `EndBiome.getAltTopMaterial(0)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:83)
- [ ] `EndBiome.generateFloorRule(0)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:87)
- [ ] `EndBiome.surface(0)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betterend/world/biome/EndBiome.java:91)
- [ ] `EndBiome.findTopMaterial(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/biome/EndBiome.java:95)
- [ ] `EndBiome.findTopMaterial(2)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/biome/EndBiome.java:101)
- [ ] `EndBiome.sampleTopMaterial(2)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/biome/EndBiome.java:107)
- [ ] `EndBiome.findUnderMaterial(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/biome/EndBiome.java:112)
- [ ] `EndBiome.findUnderMaterial(2)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/biome/EndBiome.java:118)

### `EndBiome.Config`: it implements `BiomeFactory`

- [ ] `EndBiome.Config.DEFAULT_MATERIAL`: it uses `SurfaceMaterialProvider` (org.betterx.bclib.interfaces) (org/betterx/betterend/world/biome/EndBiome.java:125)
- [ ] `EndBiome.Config.END_STONE`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:126)
- [ ] `EndBiome.Config.END_MOSS`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:127)
- [ ] `EndBiome.Config.ENDSTONE_DUST`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:128)
- [ ] `EndBiome.Config.END_MYCELIUM`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:129)
- [ ] `EndBiome.Config.FLAVOLITE`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:130)
- [ ] `EndBiome.Config.SULPHURIC_ROCK`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:131)
- [ ] `EndBiome.Config.BRIMSTONE`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:132)
- [ ] `EndBiome.Config.PALLIDIUM_FULL`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:133)
- [ ] `EndBiome.Config.PALLIDIUM_HEAVY`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:134)
- [ ] `EndBiome.Config.PALLIDIUM_THIN`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:135)
- [ ] `EndBiome.Config.PALLIDIUM_TINY`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:136)
- [ ] `EndBiome.Config.UMBRALITH`: it uses Minecraft's `MaterialRule` (org/betterx/betterend/world/biome/EndBiome.java:137)
- [ ] `EndBiome.Config.Config(0)`: its class isn't translated (it implements `BiomeFactory`) (org/betterx/betterend/world/biome/EndBiome.java:139)
- [ ] `EndBiome.Config.addCustomBuildData(1)`: it has no body (org/betterx/betterend/world/biome/EndBiome.java:142)
- [ ] `EndBiome.Config.hasCaves(0)`: its class isn't translated (it implements `BiomeFactory`) (org/betterx/betterend/world/biome/EndBiome.java:144)
- [ ] `EndBiome.Config.hasReturnGateway(0)`: its class isn't translated (it implements `BiomeFactory`) (org/betterx/betterend/world/biome/EndBiome.java:148)
- [ ] `EndBiome.Config.surfaceMaterial(0)`: its class isn't translated (it implements `BiomeFactory`) (org/betterx/betterend/world/biome/EndBiome.java:152)
- [ ] `EndBiome.Config.instantiateBiome(11)`: its class isn't translated (it implements `BiomeFactory`) (org/betterx/betterend/world/biome/EndBiome.java:156)

### `EndBiome.DefaultSurfaceMaterialProvider`: it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)

- [ ] `EndBiome.DefaultSurfaceMaterialProvider.END_STONE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/biome/EndBiome.java:176)
- [ ] `EndBiome.DefaultSurfaceMaterialProvider.getTopMaterial(0)`: its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)) (org/betterx/betterend/world/biome/EndBiome.java:178)
- [ ] `EndBiome.DefaultSurfaceMaterialProvider.getAltTopMaterial(0)`: its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)) (org/betterx/betterend/world/biome/EndBiome.java:182)
- [ ] `EndBiome.DefaultSurfaceMaterialProvider.getUnderMaterial(0)`: its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)) (org/betterx/betterend/world/biome/EndBiome.java:186)
- [ ] `EndBiome.DefaultSurfaceMaterialProvider.generateFloorRule(0)`: its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)) (org/betterx/betterend/world/biome/EndBiome.java:190)
- [ ] `EndBiome.DefaultSurfaceMaterialProvider.surface(0)`: its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)) (org/betterx/betterend/world/biome/EndBiome.java:194)

### `EndBiomeBuilder`: it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)

- [x] `EndBiomeBuilder.GEN_CHANCE_FACTOR` (org/betterx/betterend/world/biome/EndBiomeBuilder.java:31)
- [ ] `EndBiomeBuilder.ON_FLOOR`: it uses Minecraft's `MaterialCondition` (org/betterx/betterend/world/biome/EndBiomeBuilder.java:32)
- [ ] `EndBiomeBuilder.UNDER_FLOOR`: it uses Minecraft's `MaterialCondition` (org/betterx/betterend/world/biome/EndBiomeBuilder.java:33)
- [ ] `EndBiomeBuilder.ON_CEILING`: it uses Minecraft's `MaterialCondition` (org/betterx/betterend/world/biome/EndBiomeBuilder.java:34)
- [ ] `EndBiomeBuilder.UNDER_CEILING`: it uses Minecraft's `MaterialCondition` (org/betterx/betterend/world/biome/EndBiomeBuilder.java:35)
- [ ] `EndBiomeBuilder.EndBiomeBuilder(3)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betterend/world/biome/EndBiomeBuilder.java:40)
- [ ] `EndBiomeBuilder.configure(1)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betterend/world/biome/EndBiomeBuilder.java:57)
- [ ] `EndBiomeBuilder.createKey(1)`: it's generic (org/betterx/betterend/world/biome/EndBiomeBuilder.java:75)
- [ ] `EndBiomeBuilder.createKey(2)`: it's generic (org/betterx/betterend/world/biome/EndBiomeBuilder.java:79)
- [ ] `EndBiomeBuilder.registerBiomeData(1)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betterend/world/biome/EndBiomeBuilder.java:85)

### `EndBiomeBuilder.BiomeFactory`: it's an interface

- [ ] `EndBiomeBuilder.BiomeFactory.instantiateBiome(11)`: it has no body (org/betterx/betterend/world/biome/EndBiomeBuilder.java:106)

### `EndBiomeKey`: it's generic

- [ ] `EndBiomeKey.EndBiomeKey(2)`: its class isn't translated (it's generic) (org/betterx/betterend/world/biome/EndBiomeKey.java:15)
- [ ] `EndBiomeKey.bootstrap(1)`: its class isn't translated (it's generic) (org/betterx/betterend/world/biome/EndBiomeKey.java:20)
- [ ] `EndBiomeKey.bootstrap(3)`: its class isn't translated (it's generic) (org/betterx/betterend/world/biome/EndBiomeKey.java:25)

### `EndBiomeKey.ConfigSupplier`: it's an interface

- [ ] `EndBiomeKey.ConfigSupplier.get(1)`: it has no body (org/betterx/betterend/world/biome/EndBiomeKey.java:31)

### `AmberLandBiome`: it extends `Config`

- [ ] `AmberLandBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/AmberLandBiome.java:19)
- [ ] `AmberLandBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/AmberLandBiome.java:46)

### `BlossomingSpiresBiome`: it extends `Config`

- [ ] `BlossomingSpiresBiome.hasCaves(0)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/BlossomingSpiresBiome.java:17)
- [ ] `BlossomingSpiresBiome.hasReturnGateway(0)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/BlossomingSpiresBiome.java:22)
- [ ] `BlossomingSpiresBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/BlossomingSpiresBiome.java:27)
- [ ] `BlossomingSpiresBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/BlossomingSpiresBiome.java:52)

### `ChorusForestBiome`: it extends `Config`

- [ ] `ChorusForestBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/ChorusForestBiome.java:21)
- [ ] `ChorusForestBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/ChorusForestBiome.java:50)

### `CrystalMountainsBiome`: it extends `Config`

- [ ] `CrystalMountainsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/CrystalMountainsBiome.java:22)
- [ ] `CrystalMountainsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/CrystalMountainsBiome.java:32)

### `DragonGraveyardsBiome`: it extends `Config`

- [ ] `DragonGraveyardsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/DragonGraveyardsBiome.java:18)
- [ ] `DragonGraveyardsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/DragonGraveyardsBiome.java:43)

### `DryShrublandBiome`: it extends `Config`

- [ ] `DryShrublandBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/DryShrublandBiome.java:16)
- [ ] `DryShrublandBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/DryShrublandBiome.java:35)

### `DustWastelandsBiome`: it extends `Config`

- [ ] `DustWastelandsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/DustWastelandsBiome.java:21)
- [ ] `DustWastelandsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/DustWastelandsBiome.java:38)

### `FoggyMushroomlandBiome`: it extends `Config`

- [ ] `FoggyMushroomlandBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/FoggyMushroomlandBiome.java:18)
- [ ] `FoggyMushroomlandBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/FoggyMushroomlandBiome.java:52)

### `GlowingGrasslandsBiome`: it extends `Config`

- [ ] `GlowingGrasslandsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/GlowingGrasslandsBiome.java:17)
- [ ] `GlowingGrasslandsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/GlowingGrasslandsBiome.java:46)

### `LanternWoodsBiome`: it extends `Config`

- [ ] `LanternWoodsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/LanternWoodsBiome.java:17)
- [ ] `LanternWoodsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/LanternWoodsBiome.java:46)

### `MegalakeBiome`: it extends `Config`

- [ ] `MegalakeBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/MegalakeBiome.java:20)
- [ ] `MegalakeBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/MegalakeBiome.java:49)

### `MegalakeGroveBiome`: it extends `Config`

- [ ] `MegalakeGroveBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/MegalakeGroveBiome.java:17)
- [ ] `MegalakeGroveBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/MegalakeGroveBiome.java:48)

### `NeonOasisBiome`: it extends `Config`

- [ ] `NeonOasisBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/NeonOasisBiome.java:29)
- [ ] `NeonOasisBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/NeonOasisBiome.java:53)

### `PaintedMountainsBiome`: it extends `Config`

- [ ] `PaintedMountainsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/PaintedMountainsBiome.java:23)
- [ ] `PaintedMountainsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/PaintedMountainsBiome.java:37)

### `ShadowForestBiome`: it extends `Config`

- [ ] `ShadowForestBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/ShadowForestBiome.java:19)
- [ ] `ShadowForestBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/ShadowForestBiome.java:51)

### `SulphurSpringsBiome`: it extends `Config`

- [ ] `SulphurSpringsBiome.hasCaves(0)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/SulphurSpringsBiome.java:28)
- [ ] `SulphurSpringsBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/SulphurSpringsBiome.java:33)
- [ ] `SulphurSpringsBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/SulphurSpringsBiome.java:61)

### `UmbraValleyBiome`: it extends `Config`

- [ ] `UmbraValleyBiome.SURFACE_BLOCKS`: it uses Minecraft's `Block` (org/betterx/betterend/world/biome/land/UmbraValleyBiome.java:24)
- [ ] `UmbraValleyBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/UmbraValleyBiome.java:32)
- [ ] `UmbraValleyBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/UmbraValleyBiome.java:48)
- [ ] `UmbraValleyBiome.getSurface(2)`: it uses Minecraft's `Block` (org/betterx/betterend/world/biome/land/UmbraValleyBiome.java:94)

### `UmbrellaJungleBiome`: it extends `Config`

- [ ] `UmbrellaJungleBiome.addCustomBuildData(1)`: its class isn't translated (it extends `Config`) (org/betterx/betterend/world/biome/land/UmbrellaJungleBiome.java:17)
- [ ] `UmbrellaJungleBiome.surfaceMaterial(0)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/biome/land/UmbrellaJungleBiome.java:50)

### `EndCaveCarver`: it implements Minecraft's `WorldCarver`

- [ ] `EndCaveCarver.MAP_CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/carvers/EndCaveCarver.java:20)
- [x] `EndCaveCarver.DEFAULT_BAND_CEILING` (org/betterx/betterend/world/carvers/EndCaveCarver.java:29)
- [ ] `EndCaveCarver.EndCaveCarver(4)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:19)
- [ ] `EndCaveCarver.codec(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:31)
- [ ] `EndCaveCarver.isStartChunk(1)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:35)
- [ ] `EndCaveCarver.getRange(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:39)
- [ ] `EndCaveCarver.caveBandCeiling(0)`: it uses `WoverEndConfig` (de.ambertation.wover.generator.api.biomesource.end) (org/betterx/betterend/world/carvers/EndCaveCarver.java:43)
- [ ] `EndCaveCarver.carve(5)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:48)
- [ ] `EndCaveCarver.probability(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:19)
- [ ] `EndCaveCarver.y(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:19)
- [ ] `EndCaveCarver.radius(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:19)
- [ ] `EndCaveCarver.verticalSquash(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndCaveCarver.java:19)

### `EndTunnelCarver`: it implements Minecraft's `WorldCarver`

- [ ] `EndTunnelCarver.MAP_CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:17)
- [x] `EndTunnelCarver.DENSITY` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:24)
- [x] `EndTunnelCarver.cached` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:25)
- [ ] `EndTunnelCarver.EndTunnelCarver(2)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndTunnelCarver.java:16)
- [ ] `EndTunnelCarver.codec(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndTunnelCarver.java:27)
- [ ] `EndTunnelCarver.isStartChunk(1)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndTunnelCarver.java:31)
- [ ] `EndTunnelCarver.getRange(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndTunnelCarver.java:35)
- [ ] `EndTunnelCarver.carve(5)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndTunnelCarver.java:39)
- [ ] `EndTunnelCarver.getNoises(0)`: it creates Minecraft's `LegacyRandomSource` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:83)
- [ ] `EndTunnelCarver.probability(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndTunnelCarver.java:16)
- [ ] `EndTunnelCarver.threshold(0)`: its class isn't translated (it implements Minecraft's `WorldCarver`) (org/betterx/betterend/world/carvers/EndTunnelCarver.java:16)

### `EndTunnelCarver.Noises3`

- [x] `EndTunnelCarver.Noises3.Noises3(4)` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:102)
- [x] `EndTunnelCarver.Noises3.seed(0)` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:102)
- [x] `EndTunnelCarver.Noises3.h(0)` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:102)
- [x] `EndTunnelCarver.Noises3.v(0)` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:102)
- [x] `EndTunnelCarver.Noises3.d(0)` (org/betterx/betterend/world/carvers/EndTunnelCarver.java:102)

### `BiomeIslandFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `BiomeIslandFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/BiomeIslandFeature.java:22)
- [ ] `BiomeIslandFeature.CENTER`: it uses Minecraft's `MutableBlockPos` (org/betterx/betterend/world/features/BiomeIslandFeature.java:23)
- [ ] `BiomeIslandFeature.ISLAND`: it uses `SDF` (org.betterx.bclib.sdf) (org/betterx/betterend/world/features/BiomeIslandFeature.java:24)
- [ ] `BiomeIslandFeature.simplexNoise`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/features/BiomeIslandFeature.java:25)
- [ ] `BiomeIslandFeature.topBlock`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/BiomeIslandFeature.java:26)
- [ ] `BiomeIslandFeature.underBlock`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/BiomeIslandFeature.java:27)
- [ ] `BiomeIslandFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/BiomeIslandFeature.java:29)
- [ ] `BiomeIslandFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/BiomeIslandFeature.java:33)
- [ ] `BiomeIslandFeature.createSDFIsland(0)`: it uses `SDF` (org.betterx.bclib.sdf) (org/betterx/betterend/world/features/BiomeIslandFeature.java:50)

### `BlueVineFeature`: it extends `ScatterFeature`

- [ ] `BlueVineFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/BlueVineFeature.java:16)
- [ ] `BlueVineFeature.BlueVineFeature(1)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/BlueVineFeature.java:18)
- [ ] `BlueVineFeature.codec(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/BlueVineFeature.java:22)
- [ ] `BlueVineFeature.canGenerate(6)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/BlueVineFeature.java:26)
- [ ] `BlueVineFeature.generate(4)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/BlueVineFeature.java:33)

### `AmaranitaPatchFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `AmaranitaPatchFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java:24)
- [x] `AmaranitaPatchFeature.MIN_COUNT` (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java:25)
- [x] `AmaranitaPatchFeature.MAX_COUNT` (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java:26)
- [x] `AmaranitaPatchFeature.PATCH_RADIUS` (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java:27)
- [ ] `AmaranitaPatchFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java:29)
- [ ] `AmaranitaPatchFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java:33)
- [ ] `AmaranitaPatchFeature.isGround(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java:58)
- [ ] `AmaranitaPatchFeature.growMushroom(4)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java:62)

### `BushFeature`: it implements Minecraft's `Feature`

- [ ] `BushFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/bushes/BushFeature.java:29)
- [ ] `BushFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/bushes/BushFeature.java:31)
- [ ] `BushFeature.BushFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/bushes/BushFeature.java:33)
- [ ] `BushFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/bushes/BushFeature.java:37)
- [ ] `BushFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/bushes/BushFeature.java:41)

### `BushFeatureConfig`: it uses Minecraft's `BlockStateProvider`

- [ ] `BushFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/bushes/BushFeatureConfig.java:9)
- [ ] `BushFeatureConfig.BushFeatureConfig(2)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/bushes/BushFeatureConfig.java:18)
- [ ] `BushFeatureConfig.BushFeatureConfig(2)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/bushes/BushFeatureConfig.java:22)

### `BushWithOuterFeature`: it implements Minecraft's `Feature`

- [ ] `BushWithOuterFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/bushes/BushWithOuterFeature.java:32)
- [ ] `BushWithOuterFeature.DIRECTIONS`: it uses Minecraft's `Direction` (org/betterx/betterend/world/features/bushes/BushWithOuterFeature.java:34)
- [ ] `BushWithOuterFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/bushes/BushWithOuterFeature.java:35)
- [ ] `BushWithOuterFeature.BushWithOuterFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/bushes/BushWithOuterFeature.java:37)
- [ ] `BushWithOuterFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/bushes/BushWithOuterFeature.java:41)
- [ ] `BushWithOuterFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/bushes/BushWithOuterFeature.java:45)

### `BushWithOuterFeatureConfig`: it extends `BushFeatureConfig`

- [ ] `BushWithOuterFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/bushes/BushWithOuterFeatureConfig.java:9)
- [ ] `BushWithOuterFeatureConfig.BushWithOuterFeatureConfig(3)`: its class isn't translated (it extends `BushFeatureConfig`) (org/betterx/betterend/world/features/bushes/BushWithOuterFeatureConfig.java:19)
- [ ] `BushWithOuterFeatureConfig.BushWithOuterFeatureConfig(3)`: its class isn't translated (it extends `BushFeatureConfig`) (org/betterx/betterend/world/features/bushes/BushWithOuterFeatureConfig.java:23)

### `LargeAmaranitaFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `LargeAmaranitaFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/bushes/LargeAmaranitaFeature.java:21)
- [ ] `LargeAmaranitaFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/bushes/LargeAmaranitaFeature.java:23)
- [ ] `LargeAmaranitaFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/bushes/LargeAmaranitaFeature.java:27)

### `Lumecorn`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `Lumecorn.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/bushes/Lumecorn.java:20)
- [ ] `Lumecorn.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/bushes/Lumecorn.java:22)
- [ ] `Lumecorn.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/bushes/Lumecorn.java:26)

### `TenaneaBushFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `TenaneaBushFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/bushes/TenaneaBushFeature.java:36)
- [ ] `TenaneaBushFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/bushes/TenaneaBushFeature.java:37)
- [ ] `TenaneaBushFeature.DIRECTIONS`: it uses Minecraft's `Direction` (org/betterx/betterend/world/features/bushes/TenaneaBushFeature.java:38)
- [ ] `TenaneaBushFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/bushes/TenaneaBushFeature.java:40)
- [ ] `TenaneaBushFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/bushes/TenaneaBushFeature.java:44)

### `CavePumpkinFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `CavePumpkinFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/CavePumpkinFeature.java:18)
- [ ] `CavePumpkinFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/CavePumpkinFeature.java:20)
- [ ] `CavePumpkinFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/CavePumpkinFeature.java:24)

### `CharniaFeature`: it extends `UnderwaterPlantFeature`

- [ ] `CharniaFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/CharniaFeature.java:6)
- [ ] `CharniaFeature.CharniaFeature(1)`: its class isn't translated (it extends `UnderwaterPlantFeature`) (org/betterx/betterend/world/features/CharniaFeature.java:8)
- [ ] `CharniaFeature.codec(0)`: its class isn't translated (it extends `UnderwaterPlantFeature`) (org/betterx/betterend/world/features/CharniaFeature.java:12)
- [ ] `CharniaFeature.getChance(0)`: its class isn't translated (it extends `UnderwaterPlantFeature`) (org/betterx/betterend/world/features/CharniaFeature.java:17)

### `DoublePlantFeature`: it extends `ScatterFeature`

- [ ] `DoublePlantFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/DoublePlantFeature.java:14)
- [ ] `DoublePlantFeature.DoublePlantFeature(1)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/DoublePlantFeature.java:16)
- [ ] `DoublePlantFeature.codec(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/DoublePlantFeature.java:20)
- [ ] `DoublePlantFeature.canGenerate(6)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/DoublePlantFeature.java:24)
- [ ] `DoublePlantFeature.generate(4)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/DoublePlantFeature.java:30)

### `DoublePlantFeatureConfig`: it extends `ScatterFeatureConfig`

- [ ] `DoublePlantFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/DoublePlantFeatureConfig.java:14)
- [ ] `DoublePlantFeatureConfig.DoublePlantFeatureConfig(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/DoublePlantFeatureConfig.java:25)
- [ ] `DoublePlantFeatureConfig.DoublePlantFeatureConfig(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/DoublePlantFeatureConfig.java:29)
- [ ] `DoublePlantFeatureConfig.getLargePlantState(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/DoublePlantFeatureConfig.java:35)
- [ ] `DoublePlantFeatureConfig.getSmallPlantState(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/DoublePlantFeatureConfig.java:39)

### `EndLilyFeature`: it extends `UnderwaterPlantScatter`

- [ ] `EndLilyFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/EndLilyFeature.java:11)
- [ ] `EndLilyFeature.EndLilyFeature(1)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/EndLilyFeature.java:13)
- [ ] `EndLilyFeature.codec(0)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/EndLilyFeature.java:17)
- [ ] `EndLilyFeature.generate(4)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/EndLilyFeature.java:21)
- [ ] `EndLilyFeature.getChance(0)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/EndLilyFeature.java:27)

### `EndLotusFeature`: it extends `UnderwaterPlantScatter`

- [ ] `EndLotusFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/EndLotusFeature.java:11)
- [ ] `EndLotusFeature.EndLotusFeature(1)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/EndLotusFeature.java:13)
- [ ] `EndLotusFeature.codec(0)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/EndLotusFeature.java:17)
- [ ] `EndLotusFeature.generate(4)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/EndLotusFeature.java:21)
- [ ] `EndLotusFeature.getChance(0)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/EndLotusFeature.java:27)

### `EndLotusLeafFeature`: it extends `ScatterFeature`

- [ ] `EndLotusLeafFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/EndLotusLeafFeature.java:19)
- [ ] `EndLotusLeafFeature.EndLotusLeafFeature(1)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/EndLotusLeafFeature.java:21)
- [ ] `EndLotusLeafFeature.codec(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/EndLotusLeafFeature.java:25)
- [ ] `EndLotusLeafFeature.generate(4)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/EndLotusLeafFeature.java:29)
- [ ] `EndLotusLeafFeature.getChance(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/EndLotusLeafFeature.java:39)
- [ ] `EndLotusLeafFeature.getCenterGround(3)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/EndLotusLeafFeature.java:44)
- [ ] `EndLotusLeafFeature.generateLeaf(2)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/EndLotusLeafFeature.java:49)
- [ ] `EndLotusLeafFeature.canGenerate(2)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/EndLotusLeafFeature.java:73)
- [ ] `EndLotusLeafFeature.canGenerate(6)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/EndLotusLeafFeature.java:92)

### `FilaluxFeature`: it extends `SkyScatterFeature`

- [ ] `FilaluxFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/FilaluxFeature.java:20)
- [ ] `FilaluxFeature.FilaluxFeature(1)`: its class isn't translated (it extends `SkyScatterFeature`) (org/betterx/betterend/world/features/FilaluxFeature.java:22)
- [ ] `FilaluxFeature.codec(0)`: its class isn't translated (it extends `SkyScatterFeature`) (org/betterx/betterend/world/features/FilaluxFeature.java:26)
- [ ] `FilaluxFeature.generate(4)`: its class isn't translated (it extends `SkyScatterFeature`) (org/betterx/betterend/world/features/FilaluxFeature.java:30)

### `GlowPillarFeature`: it extends `ScatterFeature`

- [ ] `GlowPillarFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/GlowPillarFeature.java:12)
- [ ] `GlowPillarFeature.GlowPillarFeature(1)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/GlowPillarFeature.java:14)
- [ ] `GlowPillarFeature.codec(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/GlowPillarFeature.java:18)
- [ ] `GlowPillarFeature.canGenerate(6)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/GlowPillarFeature.java:22)
- [ ] `GlowPillarFeature.generate(4)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/GlowPillarFeature.java:27)
- [ ] `GlowPillarFeature.getChance(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/GlowPillarFeature.java:33)

### `HydraluxFeature`: it extends `UnderwaterPlantScatter`

- [ ] `HydraluxFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/HydraluxFeature.java:11)
- [ ] `HydraluxFeature.HydraluxFeature(1)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/HydraluxFeature.java:13)
- [ ] `HydraluxFeature.codec(0)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/HydraluxFeature.java:17)
- [ ] `HydraluxFeature.generate(4)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/HydraluxFeature.java:21)
- [ ] `HydraluxFeature.getChance(0)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/HydraluxFeature.java:27)

### `InvertedScatterFeature`: it's generic

- [ ] `InvertedScatterFeature.InvertedScatterFeature(1)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/InvertedScatterFeature.java:18)
- [ ] `InvertedScatterFeature.canGenerate(6)`: it has no body (org/betterx/betterend/world/features/InvertedScatterFeature.java:22)
- [ ] `InvertedScatterFeature.generate(4)`: it has no body (org/betterx/betterend/world/features/InvertedScatterFeature.java:24)
- [ ] `InvertedScatterFeature.place(4)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/InvertedScatterFeature.java:26)

### `LanceleafFeature`: it extends `ScatterFeature`

- [ ] `LanceleafFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/LanceleafFeature.java:12)
- [ ] `LanceleafFeature.LanceleafFeature(1)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/LanceleafFeature.java:14)
- [ ] `LanceleafFeature.codec(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/LanceleafFeature.java:18)
- [ ] `LanceleafFeature.canGenerate(6)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/LanceleafFeature.java:22)
- [ ] `LanceleafFeature.generate(4)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/LanceleafFeature.java:27)
- [ ] `LanceleafFeature.getChance(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/LanceleafFeature.java:33)

### `MengerSpongeFeature`: it extends `UnderwaterPlantScatter`

- [ ] `MengerSpongeFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/MengerSpongeFeature.java:15)
- [ ] `MengerSpongeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/MengerSpongeFeature.java:18)
- [ ] `MengerSpongeFeature.MengerSpongeFeature(1)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/MengerSpongeFeature.java:20)
- [ ] `MengerSpongeFeature.codec(0)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/MengerSpongeFeature.java:24)
- [ ] `MengerSpongeFeature.generate(4)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/MengerSpongeFeature.java:28)

### `NeonCactusFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `NeonCactusFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/NeonCactusFeature.java:15)
- [ ] `NeonCactusFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/NeonCactusFeature.java:17)
- [ ] `NeonCactusFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/NeonCactusFeature.java:21)

### `ScatterFeature`: it's generic

- [ ] `ScatterFeature.ScatterFeature(1)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/ScatterFeature.java:21)
- [ ] `ScatterFeature.canGenerate(6)`: it has no body (org/betterx/betterend/world/features/ScatterFeature.java:25)
- [ ] `ScatterFeature.generate(4)`: it has no body (org/betterx/betterend/world/features/ScatterFeature.java:27)
- [ ] `ScatterFeature.getCenterGround(3)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/ScatterFeature.java:29)
- [ ] `ScatterFeature.canSpawn(3)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/ScatterFeature.java:33)
- [ ] `ScatterFeature.getGroundPlant(3)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/ScatterFeature.java:42)
- [ ] `ScatterFeature.getYOffset(0)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/ScatterFeature.java:52)
- [ ] `ScatterFeature.getChance(0)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/ScatterFeature.java:56)
- [ ] `ScatterFeature.place(4)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/ScatterFeature.java:60)

### `ScatterFeatureConfig`

- [ ] `ScatterFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/ScatterFeatureConfig.java:8)
- [x] `ScatterFeatureConfig.ScatterFeatureConfig(1)` (org/betterx/betterend/world/features/ScatterFeatureConfig.java:13)
- [x] `ScatterFeatureConfig.blueVine(0)` (org/betterx/betterend/world/features/ScatterFeatureConfig.java:17)
- [x] `ScatterFeatureConfig.filalux(0)` (org/betterx/betterend/world/features/ScatterFeatureConfig.java:21)
- [x] `ScatterFeatureConfig.glowPillar(0)` (org/betterx/betterend/world/features/ScatterFeatureConfig.java:25)
- [x] `ScatterFeatureConfig.lanceleaf(0)` (org/betterx/betterend/world/features/ScatterFeatureConfig.java:29)

### `SilkMothNestFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `SilkMothNestFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/SilkMothNestFeature.java:23)
- [ ] `SilkMothNestFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/SilkMothNestFeature.java:25)
- [ ] `SilkMothNestFeature.canGenerate(2)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/SilkMothNestFeature.java:29)
- [ ] `SilkMothNestFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/SilkMothNestFeature.java:47)

### `SingleInvertedScatterFeature`: it extends `InvertedScatterFeature`

- [ ] `SingleInvertedScatterFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/SingleInvertedScatterFeature.java:15)
- [ ] `SingleInvertedScatterFeature.SingleInvertedScatterFeature(1)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/SingleInvertedScatterFeature.java:17)
- [ ] `SingleInvertedScatterFeature.codec(0)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/SingleInvertedScatterFeature.java:21)
- [ ] `SingleInvertedScatterFeature.canGenerate(6)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/SingleInvertedScatterFeature.java:25)
- [ ] `SingleInvertedScatterFeature.generate(4)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/SingleInvertedScatterFeature.java:39)

### `SinglePlantFeature`: it extends `ScatterFeature`

- [ ] `SinglePlantFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/SinglePlantFeature.java:16)
- [ ] `SinglePlantFeature.SinglePlantFeature(1)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SinglePlantFeature.java:18)
- [ ] `SinglePlantFeature.codec(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SinglePlantFeature.java:22)
- [ ] `SinglePlantFeature.getCenterGround(3)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SinglePlantFeature.java:26)
- [ ] `SinglePlantFeature.canGenerate(6)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SinglePlantFeature.java:30)
- [ ] `SinglePlantFeature.generate(4)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SinglePlantFeature.java:35)

### `SinglePlantFeatureConfig`: it extends `ScatterFeatureConfig`

- [ ] `SinglePlantFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java:14)
- [ ] `SinglePlantFeatureConfig.SinglePlantFeatureConfig(2)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java:27)
- [ ] `SinglePlantFeatureConfig.SinglePlantFeatureConfig(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java:31)
- [ ] `SinglePlantFeatureConfig.SinglePlantFeatureConfig(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java:35)
- [ ] `SinglePlantFeatureConfig.SinglePlantFeatureConfig(4)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java:39)
- [ ] `SinglePlantFeatureConfig.SinglePlantFeatureConfig(4)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java:43)
- [ ] `SinglePlantFeatureConfig.getPlantState(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java:50)
- [ ] `SinglePlantFeatureConfig.charnia(1)`: it uses SinglePlantFeatureConfig, which isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java:54)

### `SkyScatterFeature`: it extends `ScatterFeature`

- [ ] `SkyScatterFeature.SkyScatterFeature(1)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SkyScatterFeature.java:12)
- [ ] `SkyScatterFeature.getChance(0)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SkyScatterFeature.java:16)
- [ ] `SkyScatterFeature.canGenerate(6)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SkyScatterFeature.java:21)
- [ ] `SkyScatterFeature.canSpawn(3)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SkyScatterFeature.java:38)
- [ ] `SkyScatterFeature.getCenterGround(3)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SkyScatterFeature.java:43)
- [ ] `SkyScatterFeature.getGroundPlant(3)`: its class isn't translated (it extends `ScatterFeature`) (org/betterx/betterend/world/features/SkyScatterFeature.java:48)

### `ArchFeature`: it implements Minecraft's `Feature`

- [ ] `ArchFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/ArchFeature.java:23)
- [ ] `ArchFeature.ArchFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/ArchFeature.java:26)
- [ ] `ArchFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/ArchFeature.java:30)
- [ ] `ArchFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/ArchFeature.java:34)

### `ArchFeatureConfig`: it uses Minecraft's `BlockStateProvider`

- [ ] `ArchFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java:15)
- [ ] `ArchFeatureConfig.ArchFeatureConfig(2)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java:25)
- [ ] `ArchFeatureConfig.ArchFeatureConfig(2)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java:29)

### `ArchFeatureConfig.SurfaceFunction`: its constants carry values

- [ ] `ArchFeatureConfig.SurfaceFunction.CODEC`: it uses Minecraft's `Codec` (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java:37)
- [ ] `ArchFeatureConfig.SurfaceFunction.SurfaceFunction(2)`: its class isn't translated (its constants carry values) (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java:41)
- [ ] `ArchFeatureConfig.SurfaceFunction.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java:46)
- [ ] `ArchFeatureConfig.SurfaceFunction.apply(1)`: its class isn't translated (its constants carry values) (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java:50)

### `BigAuroraCrystalFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `BigAuroraCrystalFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/BigAuroraCrystalFeature.java:20)
- [x] `BigAuroraCrystalFeature.RARITY_DIVISOR` (org/betterx/betterend/world/features/terrain/BigAuroraCrystalFeature.java:21)
- [ ] `BigAuroraCrystalFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/BigAuroraCrystalFeature.java:23)
- [ ] `BigAuroraCrystalFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/BigAuroraCrystalFeature.java:27)

### `CaveSurfaceCoatConfig`: it uses Minecraft's `HolderSet`

- [ ] `CaveSurfaceCoatConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatConfig.java:12)
- [ ] `CaveSurfaceCoatConfig.CaveSurfaceCoatConfig(5)`: its class isn't translated (it uses Minecraft's `HolderSet`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatConfig.java:11)
- [ ] `CaveSurfaceCoatConfig.replaceable(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatConfig.java:11)
- [ ] `CaveSurfaceCoatConfig.protect(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatConfig.java:11)
- [ ] `CaveSurfaceCoatConfig.minY(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatConfig.java:11)
- [ ] `CaveSurfaceCoatConfig.maxY(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatConfig.java:11)
- [ ] `CaveSurfaceCoatConfig.shellDepth(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatConfig.java:11)

### `CaveSurfaceCoatFeature`: it implements Minecraft's `Feature`

- [ ] `CaveSurfaceCoatFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:26)
- [x] `CaveSurfaceCoatFeature.SOLID` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:27)
- [x] `CaveSurfaceCoatFeature.ROCK` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:28)
- [x] `CaveSurfaceCoatFeature.CAVE_AIR` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:29)
- [x] `CaveSurfaceCoatFeature.OPEN_AIR` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:30)
- [x] `CaveSurfaceCoatFeature.MARGIN` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:31)
- [x] `CaveSurfaceCoatFeature.SPAN` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:32)
- [x] `CaveSurfaceCoatFeature.NO_RUN` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:33)
- [x] `CaveSurfaceCoatFeature.OPEN_RUN` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:34)
- [ ] `CaveSurfaceCoatFeature.CaveSurfaceCoatFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:37)
- [ ] `CaveSurfaceCoatFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:41)
- [ ] `CaveSurfaceCoatFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:45)
- [ ] `CaveSurfaceCoatFeature.readColumn(9)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:178)
- [ ] `CaveSurfaceCoatFeature.expand(7)`: it uses `IntArrayList` (it.unimi.dsi.fastutil.ints) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:203)
- [x] `CaveSurfaceCoatFeature.isCaveAir(5)` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:213)
- [x] `CaveSurfaceCoatFeature.touchesOpenAir(5)` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:217)
- [x] `CaveSurfaceCoatFeature.isOpenAir(5)` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:226)
- [x] `CaveSurfaceCoatFeature.inWindow(4)` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:230)
- [x] `CaveSurfaceCoatFeature.index(4)` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:234)
- [ ] `CaveSurfaceCoatFeature.toPos(5)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:238)
- [ ] `CaveSurfaceCoatFeature.coatable(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:246)
- [ ] `CaveSurfaceCoatFeature.isDecoration(3)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:250)
- [ ] `CaveSurfaceCoatFeature.paintsAnything(2)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:254)

### `CaveSurfaceCoatFeature.Columns`: it uses Minecraft's `WorldGenLevel`

- [ ] `CaveSurfaceCoatFeature.Columns.Columns(2)`: its class isn't translated (it uses Minecraft's `WorldGenLevel`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:269)
- [x] `CaveSurfaceCoatFeature.Columns.key(2)` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:274)
- [ ] `CaveSurfaceCoatFeature.Columns.chunkAt(2)`: its class isn't translated (it uses Minecraft's `WorldGenLevel`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:278)
- [ ] `CaveSurfaceCoatFeature.Columns.biomeAt(2)`: its class isn't translated (it uses Minecraft's `WorldGenLevel`) (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java:292)

### `CaveVegetationPatchFeature`: it extends Minecraft's `VegetationPatchFeature`

- [ ] `CaveVegetationPatchFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/caves/CaveVegetationPatchFeature.java:25)
- [ ] `CaveVegetationPatchFeature.CaveVegetationPatchFeature(10)`: its class isn't translated (it extends Minecraft's `VegetationPatchFeature`) (org/betterx/betterend/world/features/terrain/caves/CaveVegetationPatchFeature.java:27)
- [ ] `CaveVegetationPatchFeature.codec(0)`: its class isn't translated (it extends Minecraft's `VegetationPatchFeature`) (org/betterx/betterend/world/features/terrain/caves/CaveVegetationPatchFeature.java:44)
- [ ] `CaveVegetationPatchFeature.placeGroundPatch(6)`: its class isn't translated (it extends Minecraft's `VegetationPatchFeature`) (org/betterx/betterend/world/features/terrain/caves/CaveVegetationPatchFeature.java:48)
- [ ] `CaveVegetationPatchFeature.placeGround(5)`: its class isn't translated (it extends Minecraft's `VegetationPatchFeature`) (org/betterx/betterend/world/features/terrain/caves/CaveVegetationPatchFeature.java:92)
- [ ] `CaveVegetationPatchFeature.isExposed(4)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/terrain/caves/CaveVegetationPatchFeature.java:109)

### `StalactiteClusterConfig`: it uses Minecraft's `IntProvider`

- [ ] `StalactiteClusterConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterConfig.java:14)
- [ ] `StalactiteClusterConfig.StalactiteClusterConfig(5)`: its class isn't translated (it uses Minecraft's `IntProvider`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterConfig.java:13)
- [ ] `StalactiteClusterConfig.radius(0)`: its class isn't translated (it uses Minecraft's `IntProvider`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterConfig.java:13)
- [ ] `StalactiteClusterConfig.density(0)`: its class isn't translated (it uses Minecraft's `IntProvider`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterConfig.java:13)
- [ ] `StalactiteClusterConfig.pairChance(0)`: its class isn't translated (it uses Minecraft's `IntProvider`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterConfig.java:13)
- [ ] `StalactiteClusterConfig.floorFeature(0)`: its class isn't translated (it uses Minecraft's `IntProvider`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterConfig.java:13)
- [ ] `StalactiteClusterConfig.ceilFeature(0)`: its class isn't translated (it uses Minecraft's `IntProvider`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterConfig.java:13)

### `StalactiteClusterFeature`: it implements Minecraft's `Feature`

- [ ] `StalactiteClusterFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterFeature.java:15)
- [x] `StalactiteClusterFeature.SEARCH_RANGE` (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterFeature.java:17)
- [ ] `StalactiteClusterFeature.StalactiteClusterFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterFeature.java:19)
- [ ] `StalactiteClusterFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterFeature.java:23)
- [ ] `StalactiteClusterFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterFeature.java:27)
- [ ] `StalactiteClusterFeature.findFloor(5)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterFeature.java:66)
- [ ] `StalactiteClusterFeature.findCeiling(5)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterFeature.java:80)

### `DesertLakeFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `DesertLakeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/DesertLakeFeature.java:25)
- [ ] `DesertLakeFeature.END_STONE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/DesertLakeFeature.java:26)
- [ ] `DesertLakeFeature.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/features/terrain/DesertLakeFeature.java:27)
- [ ] `DesertLakeFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/DesertLakeFeature.java:29)
- [ ] `DesertLakeFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/DesertLakeFeature.java:33)
- [ ] `DesertLakeFeature.canReplace(1)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/DesertLakeFeature.java:227)

### `FallenPillarFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `FallenPillarFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/FallenPillarFeature.java:26)
- [ ] `FallenPillarFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/FallenPillarFeature.java:28)
- [ ] `FallenPillarFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/FallenPillarFeature.java:32)

### `FloatingSpireFeature`: it extends `SpireFeature`

- [ ] `FloatingSpireFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/FloatingSpireFeature.java:25)
- [ ] `FloatingSpireFeature.codec(0)`: its class isn't translated (it extends `SpireFeature`) (org/betterx/betterend/world/features/terrain/FloatingSpireFeature.java:27)
- [ ] `FloatingSpireFeature.place(4)`: its class isn't translated (it extends `SpireFeature`) (org/betterx/betterend/world/features/terrain/FloatingSpireFeature.java:32)

### `GeyserFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `GeyserFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/GeyserFeature.java:48)
- [ ] `GeyserFeature.REPLACE1`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/GeyserFeature.java:49)
- [ ] `GeyserFeature.REPLACE2`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/GeyserFeature.java:52)
- [ ] `GeyserFeature.IGNORE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/GeyserFeature.java:58)
- [ ] `GeyserFeature.HORIZONTAL`: it uses Minecraft's `Direction` (org/betterx/betterend/world/features/terrain/GeyserFeature.java:63)
- [ ] `GeyserFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/GeyserFeature.java:65)
- [ ] `GeyserFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/GeyserFeature.java:69)

### `IceStarFeature`: it implements Minecraft's `Feature`

- [ ] `IceStarFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/IceStarFeature.java:26)
- [ ] `IceStarFeature.IceStarFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/IceStarFeature.java:29)
- [ ] `IceStarFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/IceStarFeature.java:33)
- [ ] `IceStarFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/IceStarFeature.java:37)
- [ ] `IceStarFeature.getFibonacciPoints(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/IceStarFeature.java:90)

### `IceStarFeatureConfig`

- [ ] `IceStarFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/IceStarFeatureConfig.java:8)
- [x] `IceStarFeatureConfig.IceStarFeatureConfig(4)` (org/betterx/betterend/world/features/terrain/IceStarFeatureConfig.java:22)

### `ObsidianBoulderFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `ObsidianBoulderFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/ObsidianBoulderFeature.java:24)
- [ ] `ObsidianBoulderFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/ObsidianBoulderFeature.java:26)
- [ ] `ObsidianBoulderFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/ObsidianBoulderFeature.java:30)
- [ ] `ObsidianBoulderFeature.makeBoulder(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/ObsidianBoulderFeature.java:46)

### `ObsidianPillarBasementFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `ObsidianPillarBasementFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/ObsidianPillarBasementFeature.java:28)
- [ ] `ObsidianPillarBasementFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/ObsidianPillarBasementFeature.java:30)
- [ ] `ObsidianPillarBasementFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/ObsidianPillarBasementFeature.java:34)

### `OreLayerFeature`: it implements Minecraft's `Feature`

- [ ] `OreLayerFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/OreLayerFeature.java:17)
- [ ] `OreLayerFeature.SPHERE`: it uses `SDFSphere` (org.betterx.bclib.sdf.primitive) (org/betterx/betterend/world/features/terrain/OreLayerFeature.java:19)
- [ ] `OreLayerFeature.NOISE`: it uses `SDFCoordModify` (org.betterx.bclib.sdf.operator) (org/betterx/betterend/world/features/terrain/OreLayerFeature.java:20)
- [ ] `OreLayerFeature.FUNCTION`: it uses `SDF` (org.betterx.bclib.sdf) (org/betterx/betterend/world/features/terrain/OreLayerFeature.java:21)
- [ ] `OreLayerFeature.OreLayerFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/OreLayerFeature.java:23)
- [ ] `OreLayerFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/OreLayerFeature.java:27)
- [ ] `OreLayerFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/OreLayerFeature.java:31)

### `OreLayerFeatureConfig`: it uses Minecraft's `BlockState`

- [ ] `OreLayerFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/OreLayerFeatureConfig.java:10)
- [ ] `OreLayerFeatureConfig.OreLayerFeatureConfig(4)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/features/terrain/OreLayerFeatureConfig.java:25)
- [ ] `OreLayerFeatureConfig.getNoise(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/features/terrain/OreLayerFeatureConfig.java:32)

### `PondWithWaterfallFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `PondWithWaterfallFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:23)
- [ ] `PondWithWaterfallFeature.END_STONE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:24)
- [ ] `PondWithWaterfallFeature.END_MOSS`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:25)
- [ ] `PondWithWaterfallFeature.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:26)
- [x] `PondWithWaterfallFeature.MIN_POND` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:27)
- [x] `PondWithWaterfallFeature.MAX_POND` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:28)
- [x] `PondWithWaterfallFeature.DRY_RIM` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:29)
- [x] `PondWithWaterfallFeature.MAX_PLATEAU_PROBE` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:30)
- [x] `PondWithWaterfallFeature.PLATEAU_FLAT_TOL` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:31)
- [x] `PondWithWaterfallFeature.JITTER` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:32)
- [x] `PondWithWaterfallFeature.MAX_FLATNESS_VARIANCE` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:33)
- [x] `PondWithWaterfallFeature.MIN_TERRAIN_ABOVE_FLOOR` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:34)
- [x] `PondWithWaterfallFeature.MIN_DEPTH` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:35)
- [x] `PondWithWaterfallFeature.MAX_DEPTH` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:36)
- [x] `PondWithWaterfallFeature.FALLBACK_DEPTH` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:37)
- [x] `PondWithWaterfallFeature.MIN_SUPPORT_BELOW_BOWL` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:38)
- [x] `PondWithWaterfallFeature.MAX_THICKNESS_PROBE` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:39)
- [ ] `PondWithWaterfallFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:41)
- [ ] `PondWithWaterfallFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:45)
- [ ] `PondWithWaterfallFeature.probeFlatPlateau(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:141)
- [ ] `PondWithWaterfallFeature.probeSolidThickness(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:158)
- [ ] `PondWithWaterfallFeature.spillWaterfall(7)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:180)
- [ ] `PondWithWaterfallFeature.shuffledHorizontals(1)`: it uses Minecraft's `Direction` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java:205)

### `SingleBlockFeature`: it implements Minecraft's `Feature`

- [ ] `SingleBlockFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/SingleBlockFeature.java:17)
- [ ] `SingleBlockFeature.SingleBlockFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/SingleBlockFeature.java:22)
- [ ] `SingleBlockFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/SingleBlockFeature.java:26)
- [ ] `SingleBlockFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/SingleBlockFeature.java:30)

### `SmaragdantCrystalFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `SmaragdantCrystalFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/SmaragdantCrystalFeature.java:24)
- [x] `SmaragdantCrystalFeature.SURFACE_REJECT_CHANCE` (org/betterx/betterend/world/features/terrain/SmaragdantCrystalFeature.java:25)
- [ ] `SmaragdantCrystalFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SmaragdantCrystalFeature.java:27)
- [ ] `SmaragdantCrystalFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SmaragdantCrystalFeature.java:31)
- [ ] `SmaragdantCrystalFeature.touchesOpenSky(2)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/terrain/SmaragdantCrystalFeature.java:96)

### `SpireFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `SpireFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/SpireFeature.java:35)
- [ ] `SpireFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/SpireFeature.java:36)
- [ ] `SpireFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SpireFeature.java:44)
- [ ] `SpireFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SpireFeature.java:48)
- [ ] `SpireFeature.addSegment(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SpireFeature.java:97)

### `StalactiteFeature`: it implements Minecraft's `Feature`

- [ ] `StalactiteFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/StalactiteFeature.java:18)
- [ ] `StalactiteFeature.StalactiteFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/StalactiteFeature.java:21)
- [ ] `StalactiteFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/StalactiteFeature.java:25)
- [ ] `StalactiteFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/StalactiteFeature.java:29)

### `StalactiteFeatureConfig`: it uses Minecraft's `BlockStateProvider`

- [ ] `StalactiteFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/StalactiteFeatureConfig.java:13)
- [ ] `StalactiteFeatureConfig.StalactiteFeatureConfig(3)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/terrain/StalactiteFeatureConfig.java:25)
- [ ] `StalactiteFeatureConfig.StalactiteFeatureConfig(6)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/terrain/StalactiteFeatureConfig.java:29)
- [ ] `StalactiteFeatureConfig.StalactiteFeatureConfig(3)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/terrain/StalactiteFeatureConfig.java:37)

### `SulphurFloorMix`

- [ ] `SulphurFloorMix.TYPE_NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:14)
- [ ] `SulphurFloorMix.CINNABAR_NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:15)
- [x] `SulphurFloorMix.MIN_DEPTH` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:16)
- [x] `SulphurFloorMix.TYPE_SCALE` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:17)
- [x] `SulphurFloorMix.CINNABAR_SCALE` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:18)
- [x] `SulphurFloorMix.CINNABAR_THRESHOLD` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:19)
- [x] `SulphurFloorMix.SulphurFloorMix(0)` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:21)
- [ ] `SulphurFloorMix.pick(3)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:24)
- [ ] `SulphurFloorMix.pickByNoise(3)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:33)
- [ ] `SulphurFloorMix.apply(3)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:44)
- [ ] `SulphurFloorMix.isBuried(2)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:51)
- [ ] `SulphurFloorMix.isDepositBlock(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:64)
- [ ] `SulphurFloorMix.facesOpenSpace(3)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java:68)

### `SulphurHillFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `SulphurHillFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/SulphurHillFeature.java:22)
- [ ] `SulphurHillFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphurHillFeature.java:24)
- [ ] `SulphurHillFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphurHillFeature.java:28)
- [ ] `SulphurHillFeature.makeCircle(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphurHillFeature.java:50)

### `SulphuricLakeFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `SulphuricLakeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:28)
- [ ] `SulphuricLakeFeature.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:29)
- [ ] `SulphuricLakeFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:31)
- [ ] `SulphuricLakeFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:35)
- [ ] `SulphuricLakeFeature.clampedRelative(3)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:146)
- [ ] `SulphuricLakeFeature.isBorder(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:150)
- [ ] `SulphuricLakeFeature.isAbsoluteBorder(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:164)
- [ ] `SulphuricLakeFeature.isDeepWater(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:178)
- [ ] `SulphuricLakeFeature.placeBrimstone(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:196)
- [ ] `SulphuricLakeFeature.getBrimstone(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:204)
- [ ] `SulphuricLakeFeature.makeShards(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java:215)

### `SurfaceVentFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `SurfaceVentFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/SurfaceVentFeature.java:19)
- [ ] `SurfaceVentFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SurfaceVentFeature.java:21)
- [ ] `SurfaceVentFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/terrain/SurfaceVentFeature.java:25)

### `ThinArchFeature`: it implements Minecraft's `Feature`

- [ ] `ThinArchFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/ThinArchFeature.java:27)
- [ ] `ThinArchFeature.ThinArchFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/ThinArchFeature.java:30)
- [ ] `ThinArchFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/ThinArchFeature.java:34)
- [ ] `ThinArchFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betterend/world/features/terrain/ThinArchFeature.java:38)

### `ThinArchFeatureConfig`: it uses Minecraft's `BlockStateProvider`

- [ ] `ThinArchFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/terrain/ThinArchFeatureConfig.java:9)
- [ ] `ThinArchFeatureConfig.ThinArchFeatureConfig(1)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/terrain/ThinArchFeatureConfig.java:14)
- [ ] `ThinArchFeatureConfig.ThinArchFeatureConfig(1)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (org/betterx/betterend/world/features/terrain/ThinArchFeatureConfig.java:18)

### `DragonHelixTreeFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `DragonHelixTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java:27)
- [x] `DragonHelixTreeFeature.LIGHT_CHANCE` (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java:28)
- [x] `DragonHelixTreeFeature.HELIX_TURN` (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java:29)
- [x] `DragonHelixTreeFeature.HELIX_REACH` (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java:30)
- [ ] `DragonHelixTreeFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java:32)
- [ ] `DragonHelixTreeFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java:36)
- [ ] `DragonHelixTreeFeature.placeLeafRing(7)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java:88)
- [ ] `DragonHelixTreeFeature.hangBulbVine(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java:113)

### `DragonTreeFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `DragonTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:37)
- [ ] `DragonTreeFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:38)
- [ ] `DragonTreeFeature.IGNORE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:41)
- [ ] `DragonTreeFeature.POST`: it uses `PosInfo` (org.betterx.bclib.sdf) (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:42)
- [ ] `DragonTreeFeature.BRANCH`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:46)
- [ ] `DragonTreeFeature.SIDE1`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:55)
- [ ] `DragonTreeFeature.SIDE2`: it uses DragonTreeFeature.SIDE1, which isn't translated (it calls `Lists` (com.google.common.collect).newArrayList) (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:58)
- [ ] `DragonTreeFeature.ROOT`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:59)
- [x] `DragonTreeFeature.LEAF_BALL_BULGE` (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:68)
- [ ] `DragonTreeFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:70)
- [ ] `DragonTreeFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:74)
- [ ] `DragonTreeFeature.makeCap(6)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:103)
- [ ] `DragonTreeFeature.makeRoots(5)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:132)
- [ ] `DragonTreeFeature.leavesBall(6)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/DragonTreeFeature.java:149)

### `EndTreeHelper`

- [x] `EndTreeHelper.SCAN_DOWN` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:20)
- [x] `EndTreeHelper.SCAN_UP` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:21)
- [x] `EndTreeHelper.GATE_DOWN` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:22)
- [x] `EndTreeHelper.GATE_UP` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:23)
- [x] `EndTreeHelper.EndTreeHelper(0)` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:25)
- [ ] `EndTreeHelper.fitBallRadius(5)`: it uses `WriteZone` (de.ambertation.wover.feature.api) (org/betterx/betterend/world/features/trees/EndTreeHelper.java:28)
- [ ] `EndTreeHelper.pruneUnsupportedFur(2)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:37)
- [ ] `EndTreeHelper.dropIfUnsupported(3)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:55)
- [ ] `EndTreeHelper.waterlogSubmerged(3)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:69)
- [ ] `EndTreeHelper.hasWater(8)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:144)
- [ ] `EndTreeHelper.isLakeWater(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/EndTreeHelper.java:158)

### `GiganticAmaranitaFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `GiganticAmaranitaFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/GiganticAmaranitaFeature.java:33)
- [ ] `GiganticAmaranitaFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/GiganticAmaranitaFeature.java:34)
- [ ] `GiganticAmaranitaFeature.IGNORE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/GiganticAmaranitaFeature.java:35)
- [ ] `GiganticAmaranitaFeature.POST`: it uses `PosInfo` (org.betterx.bclib.sdf) (org/betterx/betterend/world/features/trees/GiganticAmaranitaFeature.java:36)
- [ ] `GiganticAmaranitaFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/GiganticAmaranitaFeature.java:41)
- [ ] `GiganticAmaranitaFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/GiganticAmaranitaFeature.java:45)
- [ ] `GiganticAmaranitaFeature.makeHead(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/GiganticAmaranitaFeature.java:79)

### `HelixTreeFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `HelixTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/HelixTreeFeature.java:36)
- [ ] `HelixTreeFeature.POST`: it uses `PosInfo` (org.betterx.bclib.sdf) (org/betterx/betterend/world/features/trees/HelixTreeFeature.java:37)
- [ ] `HelixTreeFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/HelixTreeFeature.java:42)
- [ ] `HelixTreeFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/HelixTreeFeature.java:46)
- [ ] `HelixTreeFeature.fillLine(6)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/HelixTreeFeature.java:161)

### `JellyshroomFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `JellyshroomFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/JellyshroomFeature.java:32)
- [ ] `JellyshroomFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/JellyshroomFeature.java:33)
- [ ] `JellyshroomFeature.ROOT`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/JellyshroomFeature.java:34)
- [ ] `JellyshroomFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/JellyshroomFeature.java:38)
- [ ] `JellyshroomFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/JellyshroomFeature.java:42)
- [ ] `JellyshroomFeature.makeRoots(6)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/JellyshroomFeature.java:86)
- [ ] `JellyshroomFeature.makeCap(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/JellyshroomFeature.java:102)

### `LacugroveFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `LacugroveFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/LacugroveFeature.java:36)
- [ ] `LacugroveFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/LacugroveFeature.java:37)
- [ ] `LacugroveFeature.IGNORE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/LacugroveFeature.java:44)
- [ ] `LacugroveFeature.POST`: it uses `PosInfo` (org.betterx.bclib.sdf) (org/betterx/betterend/world/features/trees/LacugroveFeature.java:45)
- [x] `LacugroveFeature.LEAF_BALL_BULGE` (org/betterx/betterend/world/features/trees/LacugroveFeature.java:49)
- [ ] `LacugroveFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LacugroveFeature.java:51)
- [ ] `LacugroveFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LacugroveFeature.java:55)
- [ ] `LacugroveFeature.grow(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LacugroveFeature.java:59)
- [ ] `LacugroveFeature.generate(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LacugroveFeature.java:67)
- [ ] `LacugroveFeature.leavesBall(6)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LacugroveFeature.java:132)

### `LucerniaFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `LucerniaFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/LucerniaFeature.java:40)
- [ ] `LucerniaFeature.DIRECTIONS`: it uses Minecraft's `Direction` (org/betterx/betterend/world/features/trees/LucerniaFeature.java:41)
- [ ] `LucerniaFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/LucerniaFeature.java:42)
- [ ] `LucerniaFeature.IGNORE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/LucerniaFeature.java:45)
- [ ] `LucerniaFeature.SPLINE`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/LucerniaFeature.java:46)
- [ ] `LucerniaFeature.ROOT`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/LucerniaFeature.java:56)
- [x] `LucerniaFeature.LEAF_BALL_BULGE` (org/betterx/betterend/world/features/trees/LucerniaFeature.java:59)
- [ ] `LucerniaFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LucerniaFeature.java:61)
- [ ] `LucerniaFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LucerniaFeature.java:65)
- [ ] `LucerniaFeature.leavesBall(8)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LucerniaFeature.java:96)
- [ ] `LucerniaFeature.makeRoots(5)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/LucerniaFeature.java:212)

### `MossyGlowshroomFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `MossyGlowshroomFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:42)
- [ ] `MossyGlowshroomFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:43)
- [x] `MossyGlowshroomFeature.CENTER` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:44)
- [ ] `MossyGlowshroomFeature.FUNCTION`: it uses `SDFBinary` (org.betterx.bclib.sdf.operator) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:45)
- [ ] `MossyGlowshroomFeature.HEAD_POS`: it uses `SDFTranslate` (org.betterx.bclib.sdf.operator) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:46)
- [ ] `MossyGlowshroomFeature.ROOTS_ROT`: it uses `SDFFlatWave` (org.betterx.bclib.sdf.operator) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:47)
- [ ] `MossyGlowshroomFeature.CONE1`: it uses `SDFPrimitive` (org.betterx.bclib.sdf.primitive) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:48)
- [ ] `MossyGlowshroomFeature.CONE2`: it uses `SDFPrimitive` (org.betterx.bclib.sdf.primitive) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:49)
- [ ] `MossyGlowshroomFeature.CONE_GLOW`: it uses `SDFPrimitive` (org.betterx.bclib.sdf.primitive) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:50)
- [ ] `MossyGlowshroomFeature.ROOTS`: it uses `SDFPrimitive` (org.betterx.bclib.sdf.primitive) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:51)
- [x] `MossyGlowshroomFeature.CAP_RADIUS` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:52)
- [x] `MossyGlowshroomFeature.GLOW_RADIUS` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:53)
- [x] `MossyGlowshroomFeature.INNER_CONE_SCALE` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:54)
- [x] `MossyGlowshroomFeature.CAP_BULGE` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:55)
- [x] `MossyGlowshroomFeature.CAP_REACH` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:56)
- [x] `MossyGlowshroomFeature.MIN_SCALE` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:57)
- [ ] `MossyGlowshroomFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:59)
- [ ] `MossyGlowshroomFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:63)
- [ ] `MossyGlowshroomFeature.grow(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:68)
- [ ] `MossyGlowshroomFeature.generate(3)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java:76)

### `PythadendronTreeFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `PythadendronTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:34)
- [ ] `PythadendronTreeFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:35)
- [ ] `PythadendronTreeFeature.IGNORE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:38)
- [ ] `PythadendronTreeFeature.POST`: it uses `PosInfo` (org.betterx.bclib.sdf) (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:39)
- [x] `PythadendronTreeFeature.LEAF_BALL_BULGE` (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:43)
- [ ] `PythadendronTreeFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:45)
- [ ] `PythadendronTreeFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:49)
- [ ] `PythadendronTreeFeature.branch(10)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:71)
- [ ] `PythadendronTreeFeature.leavesBall(5)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java:115)

### `TenaneaFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `TenaneaFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/TenaneaFeature.java:38)
- [ ] `TenaneaFeature.DIRECTIONS`: it uses Minecraft's `Direction` (org/betterx/betterend/world/features/trees/TenaneaFeature.java:39)
- [ ] `TenaneaFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/TenaneaFeature.java:40)
- [ ] `TenaneaFeature.IGNORE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/TenaneaFeature.java:43)
- [ ] `TenaneaFeature.SPLINE`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/TenaneaFeature.java:44)
- [x] `TenaneaFeature.LEAF_BALL_BULGE` (org/betterx/betterend/world/features/trees/TenaneaFeature.java:54)
- [ ] `TenaneaFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/TenaneaFeature.java:56)
- [ ] `TenaneaFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/TenaneaFeature.java:60)
- [ ] `TenaneaFeature.leavesBall(7)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/TenaneaFeature.java:90)

### `UmbrellaTreeFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `UmbrellaTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:38)
- [ ] `UmbrellaTreeFeature.REPLACE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:39)
- [ ] `UmbrellaTreeFeature.SPLINE`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:42)
- [ ] `UmbrellaTreeFeature.ROOT`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:52)
- [x] `UmbrellaTreeFeature.MEMBRANE_BULGE` (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:55)
- [ ] `UmbrellaTreeFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:57)
- [ ] `UmbrellaTreeFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:61)
- [ ] `UmbrellaTreeFeature.fitBranch(5)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:173)
- [ ] `UmbrellaTreeFeature.makeRoots(6)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:190)
- [ ] `UmbrellaTreeFeature.makeMembrane(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:207)
- [ ] `UmbrellaTreeFeature.makeFruits(5)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:224)

### `UmbrellaTreeFeature.Center`

- [x] `UmbrellaTreeFeature.Center.Center(4)` (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:249)
- [ ] `UmbrellaTreeFeature.Center.distance(2)`: it calls `MHelper` (org.betterx.bclib.util).length (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java:256)

### `UnderwaterPlantFeature`: it extends `UnderwaterPlantScatter`

- [ ] `UnderwaterPlantFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/UnderwaterPlantFeature.java:14)
- [ ] `UnderwaterPlantFeature.UnderwaterPlantFeature(1)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/UnderwaterPlantFeature.java:16)
- [ ] `UnderwaterPlantFeature.codec(0)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/UnderwaterPlantFeature.java:20)
- [ ] `UnderwaterPlantFeature.canGenerate(6)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/UnderwaterPlantFeature.java:24)
- [ ] `UnderwaterPlantFeature.generate(4)`: its class isn't translated (it extends `UnderwaterPlantScatter`) (org/betterx/betterend/world/features/UnderwaterPlantFeature.java:29)

### `UnderwaterPlantScatter`: it's generic

- [ ] `UnderwaterPlantScatter.UnderwaterPlantScatter(1)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/UnderwaterPlantScatter.java:11)
- [ ] `UnderwaterPlantScatter.getCenterGround(3)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/UnderwaterPlantScatter.java:15)
- [ ] `UnderwaterPlantScatter.canGenerate(6)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/UnderwaterPlantScatter.java:24)
- [ ] `UnderwaterPlantScatter.canSpawn(3)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/UnderwaterPlantScatter.java:29)
- [ ] `UnderwaterPlantScatter.getGroundPlant(3)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/UnderwaterPlantScatter.java:34)
- [ ] `UnderwaterPlantScatter.getYOffset(0)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/UnderwaterPlantScatter.java:39)
- [ ] `UnderwaterPlantScatter.getChance(0)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/UnderwaterPlantScatter.java:44)
- [ ] `UnderwaterPlantScatter.getGround(2)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/UnderwaterPlantScatter.java:49)

### `VineFeature`: it extends `InvertedScatterFeature`

- [ ] `VineFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/VineFeature.java:16)
- [ ] `VineFeature.VineFeature(1)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/VineFeature.java:18)
- [ ] `VineFeature.codec(0)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/VineFeature.java:22)
- [ ] `VineFeature.canGenerate(6)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/VineFeature.java:26)
- [ ] `VineFeature.generate(4)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/VineFeature.java:32)
- [ ] `VineFeature.canPlaceBlock(3)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/VineFeature.java:48)
- [ ] `VineFeature.getTopState(0)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/VineFeature.java:60)
- [ ] `VineFeature.getMiddleState(0)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/VineFeature.java:65)
- [ ] `VineFeature.getBottomState(0)`: its class isn't translated (it extends `InvertedScatterFeature`) (org/betterx/betterend/world/features/VineFeature.java:70)

### `VineFeatureConfig`: it extends `ScatterFeatureConfig`

- [ ] `VineFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/VineFeatureConfig.java:14)
- [ ] `VineFeatureConfig.VineFeatureConfig(2)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/VineFeatureConfig.java:25)
- [ ] `VineFeatureConfig.VineFeatureConfig(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/VineFeatureConfig.java:29)
- [ ] `VineFeatureConfig.getPlantState(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/VineFeatureConfig.java:35)

### `WallPlantFeature`: it extends `WallScatterFeature`

- [ ] `WallPlantFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/WallPlantFeature.java:18)
- [ ] `WallPlantFeature.WallPlantFeature(1)`: its class isn't translated (it extends `WallScatterFeature`) (org/betterx/betterend/world/features/WallPlantFeature.java:20)
- [ ] `WallPlantFeature.codec(0)`: its class isn't translated (it extends `WallScatterFeature`) (org/betterx/betterend/world/features/WallPlantFeature.java:24)
- [ ] `WallPlantFeature.canGenerate(5)`: its class isn't translated (it extends `WallScatterFeature`) (org/betterx/betterend/world/features/WallPlantFeature.java:28)
- [ ] `WallPlantFeature.generate(5)`: its class isn't translated (it extends `WallScatterFeature`) (org/betterx/betterend/world/features/WallPlantFeature.java:42)

### `WallPlantFeatureConfig`: it extends `ScatterFeatureConfig`

- [ ] `WallPlantFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/WallPlantFeatureConfig.java:14)
- [ ] `WallPlantFeatureConfig.WallPlantFeatureConfig(2)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/WallPlantFeatureConfig.java:20)
- [ ] `WallPlantFeatureConfig.WallPlantFeatureConfig(2)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/WallPlantFeatureConfig.java:24)
- [ ] `WallPlantFeatureConfig.getPlantState(3)`: its class isn't translated (it extends `ScatterFeatureConfig`) (org/betterx/betterend/world/features/WallPlantFeatureConfig.java:29)

### `WallPlantOnLogFeature`: it extends `WallPlantFeature`

- [ ] `WallPlantOnLogFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/features/WallPlantOnLogFeature.java:12)
- [ ] `WallPlantOnLogFeature.WallPlantOnLogFeature(1)`: its class isn't translated (it extends `WallPlantFeature`) (org/betterx/betterend/world/features/WallPlantOnLogFeature.java:14)
- [ ] `WallPlantOnLogFeature.codec(0)`: its class isn't translated (it extends `WallPlantFeature`) (org/betterx/betterend/world/features/WallPlantOnLogFeature.java:18)
- [ ] `WallPlantOnLogFeature.canGenerate(5)`: its class isn't translated (it extends `WallPlantFeature`) (org/betterx/betterend/world/features/WallPlantOnLogFeature.java:23)

### `WallScatterFeature`: it's generic

- [ ] `WallScatterFeature.DIR`: it uses Minecraft's `Direction` (org/betterx/betterend/world/features/WallScatterFeature.java:17)
- [ ] `WallScatterFeature.WallScatterFeature(1)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/WallScatterFeature.java:20)
- [ ] `WallScatterFeature.canGenerate(5)`: it has no body (org/betterx/betterend/world/features/WallScatterFeature.java:24)
- [ ] `WallScatterFeature.generate(5)`: it has no body (org/betterx/betterend/world/features/WallScatterFeature.java:26)
- [ ] `WallScatterFeature.place(4)`: its class isn't translated (it's generic) (org/betterx/betterend/world/features/WallScatterFeature.java:28)
- [ ] `WallScatterFeature.overLakeWater(2)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/features/WallScatterFeature.java:72)
- [ ] `WallScatterFeature.shuffle(2)`: it uses Minecraft's `Direction` (org/betterx/betterend/world/features/WallScatterFeature.java:90)

### `EndCaveBiomeDecider`: it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)

- [x] `EndCaveBiomeDecider.CAVE_REGION_FREQ` (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:22)
- [x] `EndCaveBiomeDecider.CAVE_REGION_THRESHOLD` (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:23)
- [ ] `EndCaveBiomeDecider.EndCaveBiomeDecider(0)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:29)
- [ ] `EndCaveBiomeDecider.EndCaveBiomeDecider(1)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:33)
- [ ] `EndCaveBiomeDecider.canProvideFor(1)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:41)
- [ ] `EndCaveBiomeDecider.createInstance(1)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:45)
- [ ] `EndCaveBiomeDecider.pickerTag(0)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:49)
- [ ] `EndCaveBiomeDecider.createMap(2)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:54)
- [ ] `EndCaveBiomeDecider.createMap(1)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:61)
- [ ] `EndCaveBiomeDecider.suggestType(10)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:65)
- [ ] `EndCaveBiomeDecider.inCaveRegion(2)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:91)
- [ ] `EndCaveBiomeDecider.isLandSuggestion(1)`: it uses Minecraft's `TagKey` (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:95)
- [ ] `EndCaveBiomeDecider.caveTopY(2)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:102)
- [ ] `EndCaveBiomeDecider.columnHasCaves(2)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:107)
- [ ] `EndCaveBiomeDecider.canProvideBiome(1)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:116)
- [ ] `EndCaveBiomeDecider.provideBiome(4)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java:120)

### `EndLandBiomeDecider`: it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)

- [ ] `EndLandBiomeDecider.EndLandBiomeDecider(0)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndLandBiomeDecider.java:14)
- [ ] `EndLandBiomeDecider.canProvideFor(1)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndLandBiomeDecider.java:18)
- [ ] `EndLandBiomeDecider.createInstance(1)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndLandBiomeDecider.java:22)
- [ ] `EndLandBiomeDecider.suggestType(10)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndLandBiomeDecider.java:26)
- [ ] `EndLandBiomeDecider.canProvideBiome(1)`: its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)) (org/betterx/betterend/world/generator/EndLandBiomeDecider.java:45)

### `GeneratorOptions`

- [x] `GeneratorOptions.biomeSizeCaves` (org/betterx/betterend/world/generator/GeneratorOptions.java:8)
- [x] `GeneratorOptions.hasPortal` (org/betterx/betterend/world/generator/GeneratorOptions.java:9)
- [x] `GeneratorOptions.hasPillars` (org/betterx/betterend/world/generator/GeneratorOptions.java:10)
- [x] `GeneratorOptions.hasDragonFights` (org/betterx/betterend/world/generator/GeneratorOptions.java:11)
- [x] `GeneratorOptions.changeChorusPlant` (org/betterx/betterend/world/generator/GeneratorOptions.java:12)
- [x] `GeneratorOptions.newGenerator` (org/betterx/betterend/world/generator/GeneratorOptions.java:13)
- [x] `GeneratorOptions.generateCentralIsland` (org/betterx/betterend/world/generator/GeneratorOptions.java:14)
- [x] `GeneratorOptions.generateObsidianPlatform` (org/betterx/betterend/world/generator/GeneratorOptions.java:15)
- [x] `GeneratorOptions.endCityFailChance` (org/betterx/betterend/world/generator/GeneratorOptions.java:16)
- [x] `GeneratorOptions.bigOptions` (org/betterx/betterend/world/generator/GeneratorOptions.java:17)
- [x] `GeneratorOptions.mediumOptions` (org/betterx/betterend/world/generator/GeneratorOptions.java:18)
- [x] `GeneratorOptions.smallOptions` (org/betterx/betterend/world/generator/GeneratorOptions.java:19)
- [x] `GeneratorOptions.changeSpawn` (org/betterx/betterend/world/generator/GeneratorOptions.java:20)
- [ ] `GeneratorOptions.spawn`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/generator/GeneratorOptions.java:21)
- [x] `GeneratorOptions.replacePortal` (org/betterx/betterend/world/generator/GeneratorOptions.java:22)
- [x] `GeneratorOptions.replacePillars` (org/betterx/betterend/world/generator/GeneratorOptions.java:23)
- [x] `GeneratorOptions.directSpikeHeight` (org/betterx/betterend/world/generator/GeneratorOptions.java:24)
- [ ] `GeneratorOptions.init(0)`: it uses `Object` (org/betterx/betterend/world/generator/GeneratorOptions.java:26)
- [ ] `GeneratorOptions.updatedValue(1)`: it uses `Object` (org/betterx/betterend/world/generator/GeneratorOptions.java:30)
- [x] `GeneratorOptions.getBiomeSizeCaves(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:49)
- [x] `GeneratorOptions.hasPortal(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:53)
- [x] `GeneratorOptions.hasPillars(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:57)
- [x] `GeneratorOptions.hasDragonFights(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:61)
- [x] `GeneratorOptions.changeChorusPlant(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:65)
- [x] `GeneratorOptions.useNewGenerator(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:69)
- [x] `GeneratorOptions.generateObsidianPlatform(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:73)
- [x] `GeneratorOptions.getEndCityFailChance(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:77)
- [x] `GeneratorOptions.changeSpawn(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:81)
- [ ] `GeneratorOptions.getSpawn(0)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/generator/GeneratorOptions.java:85)
- [x] `GeneratorOptions.replacePortal(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:89)
- [x] `GeneratorOptions.replacePillars(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:93)
- [x] `GeneratorOptions.setDirectSpikeHeight(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:97)
- [x] `GeneratorOptions.isDirectSpikeHeight(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:101)
- [x] `GeneratorOptions.hasCentralIsland(0)` (org/betterx/betterend/world/generator/GeneratorOptions.java:107)

### `IslandLayer`: it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)

- [ ] `IslandLayer.IslandLayer(2)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:30)
- [ ] `IslandLayer.getSeed(2)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:44)
- [ ] `IslandLayer.updatePositions(3)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:50)
- [ ] `IslandLayer.getIsland(1)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:97)
- [ ] `IslandLayer.getRelativeDistance(5)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:114)
- [ ] `IslandLayer.calculateSDF(3)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:121)
- [ ] `IslandLayer.getDensity(3)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:133)
- [ ] `IslandLayer.getDensity(4)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:137)
- [ ] `IslandLayer.clearCache(0)`: its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/IslandLayer.java:143)
- [ ] `IslandLayer.makeCone(4)`: it uses `SDF` (org.betterx.bclib.sdf) (org/betterx/betterend/world/generator/IslandLayer.java:149)

### `IslandTerrainDensityFunction`: it implements Minecraft's `DensityFunction`

- [ ] `IslandTerrainDensityFunction.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:18)
- [ ] `IslandTerrainDensityFunction.IslandTerrainDensityFunction(4)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:17)
- [ ] `IslandTerrainDensityFunction.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:28)
- [ ] `IslandTerrainDensityFunction.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:32)
- [ ] `IslandTerrainDensityFunction.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:36)
- [ ] `IslandTerrainDensityFunction.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:40)
- [ ] `IslandTerrainDensityFunction.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:44)
- [ ] `IslandTerrainDensityFunction.cellSizeXz(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:17)
- [ ] `IslandTerrainDensityFunction.cellSizeY(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:17)
- [ ] `IslandTerrainDensityFunction.noiseMinY(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:17)
- [ ] `IslandTerrainDensityFunction.noiseHeight(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:17)

### `IslandTerrainDensityFunction.Sampler`: it implements Minecraft's `DensitySampler`

- [ ] `IslandTerrainDensityFunction.Sampler.Sampler(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:48)
- [ ] `IslandTerrainDensityFunction.Sampler.newColumn(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:49)
- [ ] `IslandTerrainDensityFunction.Sampler.fillColumn(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:53)
- [ ] `IslandTerrainDensityFunction.Sampler.valueAt(2)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:65)
- [ ] `IslandTerrainDensityFunction.Sampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:70)
- [ ] `IslandTerrainDensityFunction.Sampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:86)
- [ ] `IslandTerrainDensityFunction.Sampler.cellSizeXz(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:48)
- [ ] `IslandTerrainDensityFunction.Sampler.cellSizeY(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:48)
- [ ] `IslandTerrainDensityFunction.Sampler.noiseMinY(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:48)
- [ ] `IslandTerrainDensityFunction.Sampler.noiseHeight(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java:48)

### `LayerOptions`

- [x] `LayerOptions.LayerOptions(5)` (org/betterx/betterend/world/generator/LayerOptions.java:18)
- [x] `LayerOptions.LayerOptions(1)` (org/betterx/betterend/world/generator/LayerOptions.java:31)
- [x] `LayerOptions.clampDistance(1)` (org/betterx/betterend/world/generator/LayerOptions.java:44)
- [x] `LayerOptions.clampScale(1)` (org/betterx/betterend/world/generator/LayerOptions.java:48)
- [x] `LayerOptions.clampCoverage(1)` (org/betterx/betterend/world/generator/LayerOptions.java:52)
- [x] `LayerOptions.clampAverageHeight(1)` (org/betterx/betterend/world/generator/LayerOptions.java:56)
- [x] `LayerOptions.clampVariation(1)` (org/betterx/betterend/world/generator/LayerOptions.java:60)
- [x] `LayerOptions.toJson(0)` (org/betterx/betterend/world/generator/LayerOptions.java:64)

### `TerrainBoolCache`

- [x] `TerrainBoolCache.scaleCoordinate(1)` (org/betterx/betterend/world/generator/TerrainBoolCache.java:6)
- [x] `TerrainBoolCache.getIndex(2)` (org/betterx/betterend/world/generator/TerrainBoolCache.java:10)
- [x] `TerrainBoolCache.setData(3)` (org/betterx/betterend/world/generator/TerrainBoolCache.java:14)
- [x] `TerrainBoolCache.getData(2)` (org/betterx/betterend/world/generator/TerrainBoolCache.java:18)

### `TerrainGenerator`

- [ ] `TerrainGenerator.TERRAIN_BOOL_CACHE_MAP`: it uses `Point` (java.awt) (org/betterx/betterend/world/generator/TerrainGenerator.java:45)
- [ ] `TerrainGenerator.LOCKER`: it uses `ReentrantLock` (java.util.concurrent.locks) (org/betterx/betterend/world/generator/TerrainGenerator.java:46)
- [ ] `TerrainGenerator.POS`: it uses `Point` (java.awt) (org/betterx/betterend/world/generator/TerrainGenerator.java:47)
- [x] `TerrainGenerator.SCALE_XZ` (org/betterx/betterend/world/generator/TerrainGenerator.java:48)
- [x] `TerrainGenerator.SCALE_Y` (org/betterx/betterend/world/generator/TerrainGenerator.java:49)
- [x] `TerrainGenerator.COEF` (org/betterx/betterend/world/generator/TerrainGenerator.java:50)
- [ ] `TerrainGenerator.OFFS`: it uses `Point` (java.awt) (org/betterx/betterend/world/generator/TerrainGenerator.java:51)
- [ ] `TerrainGenerator.largeIslands`: it uses IslandLayer, which isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/TerrainGenerator.java:52)
- [ ] `TerrainGenerator.mediumIslands`: it uses IslandLayer, which isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/TerrainGenerator.java:53)
- [ ] `TerrainGenerator.smallIslands`: it uses IslandLayer, which isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)) (org/betterx/betterend/world/generator/TerrainGenerator.java:54)
- [x] `TerrainGenerator.noise1` (org/betterx/betterend/world/generator/TerrainGenerator.java:55)
- [x] `TerrainGenerator.noise2` (org/betterx/betterend/world/generator/TerrainGenerator.java:56)
- [ ] `TerrainGenerator.biomeSource`: it uses Minecraft's `BiomeSource` (org/betterx/betterend/world/generator/TerrainGenerator.java:57)
- [ ] `TerrainGenerator.config`: it uses `WoverEndConfig` (de.ambertation.wover.generator.api.biomesource.end) (org/betterx/betterend/world/generator/TerrainGenerator.java:58)
- [ ] `TerrainGenerator.biomeResolver`: it uses Minecraft's `BiomeResolver` (org/betterx/betterend/world/generator/TerrainGenerator.java:59)
- [x] `TerrainGenerator.noiseSeed` (org/betterx/betterend/world/generator/TerrainGenerator.java:60)
- [x] `TerrainGenerator.DEFAULT_CELL_SIZE_XZ` (org/betterx/betterend/world/generator/TerrainGenerator.java:61)
- [x] `TerrainGenerator.DEFAULT_CELL_SIZE_Y` (org/betterx/betterend/world/generator/TerrainGenerator.java:62)
- [ ] `TerrainGenerator.islandTerrain`: it uses IslandTerrain, which isn't translated (it uses Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/TerrainGenerator.java:63)
- [ ] `TerrainGenerator.endConfigOf(1)`: it uses `WoverEndConfig` (de.ambertation.wover.generator.api.biomesource.end) (org/betterx/betterend/world/generator/TerrainGenerator.java:65)
- [ ] `TerrainGenerator.disableTerrainGenerator(0)`: it uses TerrainGenerator.LOCKER, which isn't translated (it uses `ReentrantLock` (java.util.concurrent.locks)) (org/betterx/betterend/world/generator/TerrainGenerator.java:70)
- [ ] `TerrainGenerator.initNoise(3)`: it uses Minecraft's `BiomeSource` (org/betterx/betterend/world/generator/TerrainGenerator.java:90)
- [ ] `TerrainGenerator.fillTerrainDensity(6)`: it uses TerrainGenerator.LOCKER, which isn't translated (it uses `ReentrantLock` (java.util.concurrent.locks)) (org/betterx/betterend/world/generator/TerrainGenerator.java:109)
- [ ] `TerrainGenerator.computeColumnDensity(6)`: it uses TerrainGenerator.largeIslands, which isn't translated (it uses IslandLayer, which isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator))) (org/betterx/betterend/world/generator/TerrainGenerator.java:122)
- [ ] `TerrainGenerator.getSurfaceHeight(6)`: it uses TerrainGenerator.LOCKER, which isn't translated (it uses `ReentrantLock` (java.util.concurrent.locks)) (org/betterx/betterend/world/generator/TerrainGenerator.java:160)
- [ ] `TerrainGenerator.getAverageDepth(2)`: it uses TerrainGenerator.biomeSource, which isn't translated (it uses Minecraft's `BiomeSource`) (org/betterx/betterend/world/generator/TerrainGenerator.java:185)
- [ ] `TerrainGenerator.getBiomeData(3)`: it uses `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource) (org/betterx/betterend/world/generator/TerrainGenerator.java:207)
- [ ] `TerrainGenerator.isLand(3)`: it uses TerrainGenerator.LOCKER, which isn't translated (it uses `ReentrantLock` (java.util.concurrent.locks)) (org/betterx/betterend/world/generator/TerrainGenerator.java:213)
- [ ] `TerrainGenerator.onServerLevelInit(3)`: it uses Minecraft's `ServerLevel` (org/betterx/betterend/world/generator/TerrainGenerator.java:275)
- [ ] `TerrainGenerator.makeObsidianPlatform(2)`: it uses Minecraft's `ServerLevelAccessor` (org/betterx/betterend/world/generator/TerrainGenerator.java:300)
- [ ] `TerrainGenerator.withIslandTerrain(2)`: it uses Minecraft's `DensityFunction` (org/betterx/betterend/world/generator/TerrainGenerator.java:315)
- [ ] `TerrainGenerator.islandCellSize(1)`: it uses Minecraft's `NoiseGeneratorSettings` (org/betterx/betterend/world/generator/TerrainGenerator.java:319)
- [ ] `TerrainGenerator.islandTerrainFor(2)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/generator/TerrainGenerator.java:324)

### `TerrainGenerator.IslandTerrain`: it uses Minecraft's `DensityFunction`

- [ ] `TerrainGenerator.IslandTerrain.IslandTerrain(5)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/TerrainGenerator.java:385)
- [ ] `TerrainGenerator.IslandTerrain.source(0)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/TerrainGenerator.java:385)
- [ ] `TerrainGenerator.IslandTerrain.noiseSettings(0)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/TerrainGenerator.java:385)
- [ ] `TerrainGenerator.IslandTerrain.withIslands(0)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/TerrainGenerator.java:385)
- [ ] `TerrainGenerator.IslandTerrain.cellSizeXz(0)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/TerrainGenerator.java:385)
- [ ] `TerrainGenerator.IslandTerrain.cellSizeY(0)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (org/betterx/betterend/world/generator/TerrainGenerator.java:385)

### `EndBridgeStructure`: it extends `FeatureBaseStructure`

- [x] `EndBridgeStructure.SCAN_DIRECTIONS` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:25)
- [x] `EndBridgeStructure.SCAN_MIN_RADIUS` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:26)
- [x] `EndBridgeStructure.SCAN_MAX_RADIUS` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:27)
- [x] `EndBridgeStructure.SCAN_RADIUS_STEP` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:28)
- [x] `EndBridgeStructure.TERRAIN_MIN_Y` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:29)
- [x] `EndBridgeStructure.TERRAIN_MAX_Y` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:30)
- [x] `EndBridgeStructure.VOID_MARGIN` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:31)
- [x] `EndBridgeStructure.MIN_SPAN` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:32)
- [x] `EndBridgeStructure.MAX_SPAN` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:33)
- [x] `EndBridgeStructure.MAX_HEIGHT_DELTA` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:34)
- [x] `EndBridgeStructure.MIN_DIRECTION_SEPARATION` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:35)
- [ ] `EndBridgeStructure.EndBridgeStructure(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:37)
- [ ] `EndBridgeStructure.type(0)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:41)
- [ ] `EndBridgeStructure.findGenerationPoint(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:45)
- [ ] `EndBridgeStructure.generatePieces(2)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:52)
- [x] `EndBridgeStructure.hasTerrain(2)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:113)
- [x] `EndBridgeStructure.angularSeparation(2)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:117)

### `EndBridgeStructure.Anchor`

- [x] `EndBridgeStructure.Anchor.Anchor(5)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:126)
- [x] `EndBridgeStructure.Anchor.distSqTo(1)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:127)
- [x] `EndBridgeStructure.Anchor.x(0)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:126)
- [x] `EndBridgeStructure.Anchor.y(0)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:126)
- [x] `EndBridgeStructure.Anchor.z(0)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:126)
- [x] `EndBridgeStructure.Anchor.angle(0)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:126)
- [x] `EndBridgeStructure.Anchor.island(0)` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java:126)

### `EndLakeNormalStructure`: it extends `EndLakeStructure`

- [ ] `EndLakeNormalStructure.EndLakeNormalStructure(1)`: its class isn't translated (it extends `EndLakeStructure`) (org/betterx/betterend/world/structures/features/EndLakeNormalStructure.java:8)
- [ ] `EndLakeNormalStructure.type(0)`: its class isn't translated (it extends `EndLakeStructure`) (org/betterx/betterend/world/structures/features/EndLakeNormalStructure.java:12)

### `EndLakeRareStructure`: it extends `EndLakeStructure`

- [ ] `EndLakeRareStructure.EndLakeRareStructure(1)`: its class isn't translated (it extends `EndLakeStructure`) (org/betterx/betterend/world/structures/features/EndLakeRareStructure.java:8)
- [ ] `EndLakeRareStructure.type(0)`: its class isn't translated (it extends `EndLakeStructure`) (org/betterx/betterend/world/structures/features/EndLakeRareStructure.java:12)

### `EndLakeStructure`: it extends `FeatureBaseStructure`

- [ ] `EndLakeStructure.EndLakeStructure(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndLakeStructure.java:21)
- [ ] `EndLakeStructure.type(0)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndLakeStructure.java:25)
- [ ] `EndLakeStructure.generatePieces(2)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndLakeStructure.java:29)

### `EndSulphuricCaveStructure`: it extends `FeatureBaseStructure`

- [ ] `EndSulphuricCaveStructure.EndSulphuricCaveStructure(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndSulphuricCaveStructure.java:19)
- [ ] `EndSulphuricCaveStructure.type(0)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndSulphuricCaveStructure.java:23)
- [ ] `EndSulphuricCaveStructure.generatePieces(2)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/EndSulphuricCaveStructure.java:27)

### `EternalPortalStructure`: it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)

- [ ] `EternalPortalStructure.EternalPortalStructure(1)`: its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)) (org/betterx/betterend/world/structures/features/EternalPortalStructure.java:24)
- [ ] `EternalPortalStructure.findGenerationPoint(1)`: its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)) (org/betterx/betterend/world/structures/features/EternalPortalStructure.java:28)
- [ ] `EternalPortalStructure.type(0)`: its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)) (org/betterx/betterend/world/structures/features/EternalPortalStructure.java:50)
- [ ] `EternalPortalStructure.erosion(1)`: its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)) (org/betterx/betterend/world/structures/features/EternalPortalStructure.java:54)
- [ ] `EternalPortalStructure.cover(1)`: its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)) (org/betterx/betterend/world/structures/features/EternalPortalStructure.java:58)
- [ ] `EternalPortalStructure.alwaysBroken(0)`: its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)) (org/betterx/betterend/world/structures/features/EternalPortalStructure.java:62)
- [ ] `EternalPortalStructure.cfg(4)`: it uses Config, which isn't translated (it's generic) (org/betterx/betterend/world/structures/features/EternalPortalStructure.java:66)

### `FeatureBaseStructure`: it extends Minecraft's `Structure`

- [ ] `FeatureBaseStructure.AIR`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:31)
- [x] `FeatureBaseStructure.MIN_TERRAIN_Y` (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:32)
- [x] `FeatureBaseStructure.HEIGHT_CACHE_SIZE` (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:33)
- [ ] `FeatureBaseStructure.HEIGHT_CACHE`: it uses Minecraft's `RandomState` (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:34)
- [ ] `FeatureBaseStructure.getCachedFirstOccupiedHeight(5)`: its body couldn't be read: an anonymous class (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:36)
- [ ] `FeatureBaseStructure.FeatureBaseStructure(1)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:54)
- [ ] `FeatureBaseStructure.findGenerationPoint(1)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:58)
- [ ] `FeatureBaseStructure.findVoidGenerationPoint(2)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:81)
- [ ] `FeatureBaseStructure.getNoiseBiome(5)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:87)
- [ ] `FeatureBaseStructure.generatePieces(2)`: it has no body (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:91)
- [ ] `FeatureBaseStructure.anchorColumn(1)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:93)
- [ ] `FeatureBaseStructure.mayStartInBiome(2)`: it uses Minecraft's `GenerationContext` (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:112)
- [ ] `FeatureBaseStructure.getGenerationHeight(4)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java:122)

### `GiantIceStarStructure`: it extends `SDFStructureFeature`

- [x] `GiantIceStarStructure.minSize` (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:33)
- [x] `GiantIceStarStructure.maxSize` (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:34)
- [x] `GiantIceStarStructure.minCount` (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:35)
- [x] `GiantIceStarStructure.maxCount` (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:36)
- [ ] `GiantIceStarStructure.GiantIceStarStructure(1)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:38)
- [ ] `GiantIceStarStructure.type(0)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:42)
- [ ] `GiantIceStarStructure.findGenerationPoint(1)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:46)
- [ ] `GiantIceStarStructure.getSDF(2)`: it uses `SDF` (org.betterx.bclib.sdf) (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:51)
- [ ] `GiantIceStarStructure.getFibonacciPoints(1)`: it uses `MHelper` (org.betterx.bclib.util).PHI (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:98)
- [ ] `GiantIceStarStructure.generatePieces(2)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java:114)

### `GiantMossyGlowshroomStructure`: it extends `SDFStructureFeature`

- [ ] `GiantMossyGlowshroomStructure.GiantMossyGlowshroomStructure(1)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/GiantMossyGlowshroomStructure.java:37)
- [ ] `GiantMossyGlowshroomStructure.generatePieces(2)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/GiantMossyGlowshroomStructure.java:41)
- [ ] `GiantMossyGlowshroomStructure.getSDF(2)`: it uses `SDF` (org.betterx.bclib.sdf) (org/betterx/betterend/world/structures/features/GiantMossyGlowshroomStructure.java:46)
- [ ] `GiantMossyGlowshroomStructure.type(0)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/GiantMossyGlowshroomStructure.java:146)

### `MegaLakeSmallStructure`: it extends `FeatureBaseStructure`

- [ ] `MegaLakeSmallStructure.MegaLakeSmallStructure(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MegaLakeSmallStructure.java:21)
- [ ] `MegaLakeSmallStructure.type(0)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MegaLakeSmallStructure.java:25)
- [ ] `MegaLakeSmallStructure.generatePieces(2)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MegaLakeSmallStructure.java:29)

### `MegaLakeStructure`: it extends `FeatureBaseStructure`

- [ ] `MegaLakeStructure.MegaLakeStructure(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MegaLakeStructure.java:21)
- [ ] `MegaLakeStructure.type(0)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MegaLakeStructure.java:25)
- [ ] `MegaLakeStructure.generatePieces(2)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MegaLakeStructure.java:29)

### `MountainStructure`: it extends `FeatureBaseStructure`

- [ ] `MountainStructure.MountainStructure(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MountainStructure.java:21)
- [ ] `MountainStructure.type(0)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MountainStructure.java:25)
- [ ] `MountainStructure.generatePieces(2)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/MountainStructure.java:29)

### `PaintedMountainStructure`: it extends `FeatureBaseStructure`

- [ ] `PaintedMountainStructure.VARIANTS`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/PaintedMountainStructure.java:28)
- [ ] `PaintedMountainStructure.PaintedMountainStructure(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/PaintedMountainStructure.java:34)
- [ ] `PaintedMountainStructure.type(0)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/PaintedMountainStructure.java:38)
- [ ] `PaintedMountainStructure.generatePieces(2)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/PaintedMountainStructure.java:42)

### `SDFStructureFeature`: it extends `FeatureBaseStructure`

- [ ] `SDFStructureFeature.SDFStructureFeature(1)`: its class isn't translated (it extends `FeatureBaseStructure`) (org/betterx/betterend/world/structures/features/SDFStructureFeature.java:18)
- [ ] `SDFStructureFeature.generatePieces(3)`: it uses Minecraft's `StructurePiecesBuilder` (org/betterx/betterend/world/structures/features/SDFStructureFeature.java:22)

### `SmallIslandStructure`: it extends `SDFStructureFeature`

- [x] `SmallIslandStructure.MIN_RADIUS` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:48)
- [x] `SmallIslandStructure.MAX_RADIUS` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:49)
- [x] `SmallIslandStructure.WATERFALL_MIN_RADIUS` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:50)
- [x] `SmallIslandStructure.WATERFALL_MAX_RADIUS` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:51)
- [ ] `SmallIslandStructure.END_STONE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:52)
- [ ] `SmallIslandStructure.END_MOSS`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:53)
- [ ] `SmallIslandStructure.SANGNUM`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:54)
- [ ] `SmallIslandStructure.PALLIDIUM_FULL`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:55)
- [ ] `SmallIslandStructure.PALLIDIUM_HEAVY`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:56)
- [ ] `SmallIslandStructure.PALLIDIUM_THIN`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:57)
- [ ] `SmallIslandStructure.PALLIDIUM_TINY`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:58)
- [ ] `SmallIslandStructure.UMBRALITH`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:59)
- [ ] `SmallIslandStructure.UNDERSIDE_PATCH`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:60)
- [x] `SmallIslandStructure.PATCH_FREQ` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:61)
- [x] `SmallIslandStructure.PATCH_THRESHOLD` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:62)
- [ ] `SmallIslandStructure.SmallIslandStructure(1)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:64)
- [ ] `SmallIslandStructure.type(0)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:68)
- [ ] `SmallIslandStructure.findGenerationPoint(1)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:72)
- [ ] `SmallIslandStructure.drawGeometry(2)`: it calls `MHelper` (org.betterx.bclib.util).randRange (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:77)
- [ ] `SmallIslandStructure.getSDF(3)`: it uses `SDF` (org.betterx.bclib.sdf) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:91)
- [ ] `SmallIslandStructure.waterfallCoat(2)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:112)
- [ ] `SmallIslandStructure.inUndersidePatch(4)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:129)
- [ ] `SmallIslandStructure.hangStalactite(2)`: it uses `PosInfo` (org.betterx.bclib.sdf) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:134)
- [ ] `SmallIslandStructure.flowerCoat(2)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:148)
- [ ] `SmallIslandStructure.hangVineStrand(2)`: it uses `PosInfo` (org.betterx.bclib.sdf) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:177)
- [ ] `SmallIslandStructure.pickVine(1)`: it uses Minecraft's `Block` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:188)
- [ ] `SmallIslandStructure.makeCone(4)`: it uses `SDF` (org.betterx.bclib.sdf) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:197)
- [ ] `SmallIslandStructure.generatePieces(2)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:203)
- [ ] `SmallIslandStructure.overlapsNativeLand(4)`: it calls TerrainGenerator.isLand, which isn't translated (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:228)
- [ ] `SmallIslandStructure.terrainMaxHeight(1)`: it uses Minecraft's `GenerationContext` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:242)
- [ ] `SmallIslandStructure.isFlowerIslets(4)`: its class isn't translated (it extends `SDFStructureFeature`) (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:249)

### `SmallIslandStructure.IslandGeometry`

- [x] `SmallIslandStructure.IslandGeometry.IslandGeometry(6)` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:254)
- [x] `SmallIslandStructure.IslandGeometry.radius(0)` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:254)
- [x] `SmallIslandStructure.IslandGeometry.scaleX(0)` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:254)
- [x] `SmallIslandStructure.IslandGeometry.scale(0)` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:254)
- [x] `SmallIslandStructure.IslandGeometry.scaleZ(0)` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:254)
- [x] `SmallIslandStructure.IslandGeometry.noiseIntensity(0)` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:254)
- [x] `SmallIslandStructure.IslandGeometry.noiseSeed(0)` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java:254)

### `BasePiece`: it extends Minecraft's `StructurePiece`

- [ ] `BasePiece.BasePiece(3)`: its class isn't translated (it extends Minecraft's `StructurePiece`) (org/betterx/betterend/world/structures/piece/BasePiece.java:20)
- [ ] `BasePiece.BasePiece(2)`: its class isn't translated (it extends Minecraft's `StructurePiece`) (org/betterx/betterend/world/structures/piece/BasePiece.java:24)
- [ ] `BasePiece.fromNbt(1)`: it has no body (org/betterx/betterend/world/structures/piece/BasePiece.java:29)
- [ ] `BasePiece.worldgenSurfaceHeight(3)`: its class isn't translated (it extends Minecraft's `StructurePiece`) (org/betterx/betterend/world/structures/piece/BasePiece.java:31)
- [ ] `BasePiece.setChunkBlock(3)`: it uses Minecraft's `ChunkAccess` (org/betterx/betterend/world/structures/piece/BasePiece.java:48)
- [ ] `BasePiece.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `StructurePiece`) (org/betterx/betterend/world/structures/piece/BasePiece.java:56)
- [ ] `BasePiece.addAdditionalSaveData(2)`: its class isn't translated (it extends Minecraft's `StructurePiece`) (org/betterx/betterend/world/structures/piece/BasePiece.java:59)

### `CavePiece`: it extends `BasePiece`

- [ ] `CavePiece.CavePiece(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/CavePiece.java:27)
- [ ] `CavePiece.CavePiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/CavePiece.java:35)
- [ ] `CavePiece.postProcess(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/CavePiece.java:40)
- [ ] `CavePiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/CavePiece.java:83)
- [ ] `CavePiece.fromNbt(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/CavePiece.java:89)
- [ ] `CavePiece.makeBoundingBox(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/CavePiece.java:96)

### `CrystalMountainPiece`: it extends `MountainPiece`

- [x] `CrystalMountainPiece.CRYSTAL_END_STONE_CRYSTAL_CHANCE` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:32)
- [x] `CrystalMountainPiece.CRYSTAL_MOSS_CRYSTAL_CHANCE` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:33)
- [x] `CrystalMountainPiece.CRYSTAL_BIG_MAX_COUNT` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:34)
- [x] `CrystalMountainPiece.CROWN_MIN_COUNT` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:35)
- [x] `CrystalMountainPiece.CROWN_MAX_COUNT` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:36)
- [x] `CrystalMountainPiece.CROWN_SEARCH_RADIUS` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:37)
- [x] `CrystalMountainPiece.CROWN_SPREAD` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:38)
- [x] `CrystalMountainPiece.CROWN_MIN_PEAK_HEIGHT` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:39)
- [ ] `CrystalMountainPiece.CrystalMountainPiece(5)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:43)
- [ ] `CrystalMountainPiece.CrystalMountainPiece(2)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:48)
- [ ] `CrystalMountainPiece.fromNbt(1)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:53)
- [ ] `CrystalMountainPiece.postProcess(7)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:58)
- [ ] `CrystalMountainPiece.crystalHeight(3)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:129)
- [ ] `CrystalMountainPiece.placeCrown(3)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:134)
- [ ] `CrystalMountainPiece.findPeak(0)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:179)
- [ ] `CrystalMountainPiece.placeMountain(4)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:209)
- [ ] `CrystalMountainPiece.crystal(7)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:265)
- [ ] `CrystalMountainPiece.crystal(8)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java:269)

### `EndBridgeGeometry`

- [x] `EndBridgeGeometry.LANDING_LEN` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:7)
- [x] `EndBridgeGeometry.DECK_HALF_WIDTH` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:8)
- [x] `EndBridgeGeometry.LANDING_HALF_WIDTH` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:9)
- [ ] `EndBridgeGeometry.EndBridgeGeometry(2)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:20)
- [x] `EndBridgeGeometry.lengthSq(0)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:32)
- [x] `EndBridgeGeometry.t(2)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:36)
- [x] `EndBridgeGeometry.perp(3)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:40)
- [x] `EndBridgeGeometry.along(1)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:46)
- [x] `EndBridgeGeometry.isLanding(1)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:50)
- [x] `EndBridgeGeometry.halfWidth(1)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:55)
- [x] `EndBridgeGeometry.isDeck(2)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:59)
- [x] `EndBridgeGeometry.isEdge(2)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:64)
- [x] `EndBridgeGeometry.deckY(1)` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java:68)

### `EndBridgePiece`: it extends `BasePiece`

- [ ] `EndBridgePiece.DECK`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:24)
- [ ] `EndBridgePiece.CRACKED`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:25)
- [ ] `EndBridgePiece.WEATHERED`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:26)
- [ ] `EndBridgePiece.RAILING`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:27)
- [x] `EndBridgePiece.PILLAR_SPACING` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:28)
- [x] `EndBridgePiece.PILLAR_MAX_DROP` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:29)
- [x] `EndBridgePiece.WEATHERED_CHANCE` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:30)
- [x] `EndBridgePiece.CRACKED_CHANCE` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:31)
- [x] `EndBridgePiece.RAILING_GAP_CHANCE` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:32)
- [x] `EndBridgePiece.EROSION_CHANCE` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:33)
- [ ] `EndBridgePiece.EndBridgePiece(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:38)
- [ ] `EndBridgePiece.EndBridgePiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:46)
- [ ] `EndBridgePiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:51)
- [ ] `EndBridgePiece.fromNbt(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:58)
- [ ] `EndBridgePiece.makeBoundingBox(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:65)
- [ ] `EndBridgePiece.isReplaceable(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:75)
- [ ] `EndBridgePiece.isTerrain(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:79)
- [ ] `EndBridgePiece.deckMaterial(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:83)
- [ ] `EndBridgePiece.postProcess(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndBridgePiece.java:92)

### `EndLakePiece`: it extends `BasePiece`

- [ ] `EndLakePiece.END_STONE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:35)
- [ ] `EndLakePiece.AIR`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:36)
- [ ] `EndLakePiece.WATER`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:37)
- [ ] `EndLakePiece.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/structures/piece/EndLakePiece.java:38)
- [x] `EndLakePiece.SHORE_DUST_CHANCE` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:39)
- [x] `EndLakePiece.SHORE_WIDTH` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:47)
- [x] `EndLakePiece.OUTER_WATER_WEIGHT_MIN` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:48)
- [x] `EndLakePiece.OUTER_WATER_WEIGHT_MAX` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:49)
- [x] `EndLakePiece.MAX_AIR_FILL` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:50)
- [x] `EndLakePiece.GROUND_SEARCH_DEPTH` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:51)
- [x] `EndLakePiece.BRIDGE_DEPTH` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:52)
- [x] `EndLakePiece.SHELL_DEPTH` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:53)
- [x] `EndLakePiece.NO_GROUND` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:54)
- [x] `EndLakePiece.FLUID_BELOW` (org/betterx/betterend/world/structures/piece/EndLakePiece.java:55)
- [ ] `EndLakePiece.EndLakePiece(6)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:57)
- [ ] `EndLakePiece.EndLakePiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:67)
- [ ] `EndLakePiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:72)
- [ ] `EndLakePiece.fromNbt(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:81)
- [ ] `EndLakePiece.makeBoundingBox(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:90)
- [ ] `EndLakePiece.borderMaterial(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:102)
- [ ] `EndLakePiece.sampleSurface(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:124)
- [ ] `EndLakePiece.shoreMaterial(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:130)
- [ ] `EndLakePiece.canReplace(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:134)
- [ ] `EndLakePiece.groundColumn(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:141)
- [ ] `EndLakePiece.findGroundBelow(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:179)
- [ ] `EndLakePiece.fillDownToGround(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:197)
- [ ] `EndLakePiece.fillBelowWater(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:228)
- [ ] `EndLakePiece.postProcess(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/EndLakePiece.java:248)

### `LakeFloorLife`

- [ ] `LakeFloorLife.CLUSTER`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:29)
- [x] `LakeFloorLife.LIFE_PER_COLUMN` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:30)
- [x] `LakeFloorLife.BIOME_TOPUP` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:31)
- [ ] `LakeFloorLife.plants`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:32)
- [x] `LakeFloorLife.weights` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:33)
- [x] `LakeFloorLife.LOTUS` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:34)
- [x] `LakeFloorLife.LakeFloorLife(0)` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:36)
- [ ] `LakeFloorLife.init(0)`: it uses LakeFloorLife.plants, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:39)
- [ ] `LakeFloorLife.isLakeBiome(1)`: it uses Minecraft's `Holder` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:58)
- [ ] `LakeFloorLife.place(7)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:62)
- [x] `LakeFloorLife.hash64(4)` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:136)
- [x] `LakeFloorLife.hash01(4)` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java:143)

### `LakePiece`: it extends `BasePiece`

- [ ] `LakePiece.ENDSTONE`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/LakePiece.java:49)
- [ ] `LakePiece.WATER`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/LakePiece.java:50)
- [x] `LakePiece.SHORE_WIDTH` (org/betterx/betterend/world/structures/piece/LakePiece.java:51)
- [x] `LakePiece.GROUND_SEARCH_DEPTH` (org/betterx/betterend/world/structures/piece/LakePiece.java:52)
- [x] `LakePiece.BRIDGE_DEPTH` (org/betterx/betterend/world/structures/piece/LakePiece.java:53)
- [x] `LakePiece.SHELL_DEPTH` (org/betterx/betterend/world/structures/piece/LakePiece.java:54)
- [x] `LakePiece.SKIRT_FLUSH` (org/betterx/betterend/world/structures/piece/LakePiece.java:55)
- [x] `LakePiece.MAX_BANK_DISTANCE` (org/betterx/betterend/world/structures/piece/LakePiece.java:56)
- [x] `LakePiece.BANK_MAX_RAISE` (org/betterx/betterend/world/structures/piece/LakePiece.java:57)
- [x] `LakePiece.DIST_RELAX` (org/betterx/betterend/world/structures/piece/LakePiece.java:58)
- [x] `LakePiece.DIKE_KEEP` (org/betterx/betterend/world/structures/piece/LakePiece.java:59)
- [ ] `LakePiece.rimPlants`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/LakePiece.java:60)
- [x] `LakePiece.rimWeights` (org/betterx/betterend/world/structures/piece/LakePiece.java:61)
- [x] `LakePiece.rimSoilOnly` (org/betterx/betterend/world/structures/piece/LakePiece.java:62)
- [x] `LakePiece.RIM_FUR_CHANCE` (org/betterx/betterend/world/structures/piece/LakePiece.java:63)
- [x] `LakePiece.NO_GROUND` (org/betterx/betterend/world/structures/piece/LakePiece.java:74)
- [x] `LakePiece.FLUID_BELOW` (org/betterx/betterend/world/structures/piece/LakePiece.java:75)
- [x] `LakePiece.MAX_AIR_FILL` (org/betterx/betterend/world/structures/piece/LakePiece.java:76)
- [x] `LakePiece.SHORE_PLANTS_DENSE` (org/betterx/betterend/world/structures/piece/LakePiece.java:77)
- [x] `LakePiece.SHORE_PLANTS_END` (org/betterx/betterend/world/structures/piece/LakePiece.java:78)
- [x] `LakePiece.SHORE_PLANTS_TAIL` (org/betterx/betterend/world/structures/piece/LakePiece.java:79)
- [x] `LakePiece.SHORE_PLANTS_TAIL_FULL` (org/betterx/betterend/world/structures/piece/LakePiece.java:80)
- [x] `LakePiece.SHORE_PLANTS_TAIL_END` (org/betterx/betterend/world/structures/piece/LakePiece.java:81)
- [x] `LakePiece.SHORE_MOSS_PATCH` (org/betterx/betterend/world/structures/piece/LakePiece.java:82)
- [ ] `LakePiece.rimPlants(0)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/LakePiece.java:85)
- [ ] `LakePiece.LakePiece(6)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:108)
- [ ] `LakePiece.LakePiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:121)
- [ ] `LakePiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:126)
- [ ] `LakePiece.fromNbt(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:136)
- [ ] `LakePiece.biomeMaterial(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:148)
- [ ] `LakePiece.skirt(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:159)
- [ ] `LakePiece.shorePatch(4)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:167)
- [ ] `LakePiece.postProcess(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:171)
- [ ] `LakePiece.placeShorePlants(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:283)
- [ ] `LakePiece.groundColumn(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:323)
- [ ] `LakePiece.cutBack(4)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:339)
- [ ] `LakePiece.nearFluid(3)`: it uses Minecraft's `ChunkAccess` (org/betterx/betterend/world/structures/piece/LakePiece.java:400)
- [ ] `LakePiece.edgeFunction(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:425)
- [ ] `LakePiece.columnMaterials(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:440)
- [ ] `LakePiece.shoreDistances(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:467)
- [ ] `LakePiece.fillDownToGround(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:512)
- [ ] `LakePiece.findGroundBelow(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:543)
- [ ] `LakePiece.fillBelow(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:561)
- [ ] `LakePiece.placeRimPlant(4)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:581)
- [ ] `LakePiece.fixWater(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:620)
- [ ] `LakePiece.shorePlantHere(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:683)
- [ ] `LakePiece.isBareGround(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/LakePiece.java:703)
- [ ] `LakePiece.shorePatchHere(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:707)
- [ ] `LakePiece.hash01(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:721)
- [ ] `LakePiece.staysSupported(3)`: it uses Minecraft's `WorldGenLevel` (org/betterx/betterend/world/structures/piece/LakePiece.java:729)
- [ ] `LakePiece.isLoosePlant(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/LakePiece.java:733)
- [ ] `LakePiece.markForPostProcessing(2)`: it uses Minecraft's `ChunkAccess` (org/betterx/betterend/world/structures/piece/LakePiece.java:737)
- [ ] `LakePiece.makeEndstonePillar(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:743)
- [ ] `LakePiece.getHeight(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:753)
- [ ] `LakePiece.getHeightClamp(4)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:770)
- [ ] `LakePiece.computeHeightClamp(4)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:785)
- [ ] `LakePiece.carveMinY(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:811)
- [ ] `LakePiece.makeBoundingBox(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/LakePiece.java:815)

### `MountainPiece`: it extends `BasePiece`

- [ ] `MountainPiece.MountainPiece(6)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/MountainPiece.java:34)
- [ ] `MountainPiece.MountainPiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/MountainPiece.java:48)
- [ ] `MountainPiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/MountainPiece.java:53)
- [ ] `MountainPiece.fromNbt(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/MountainPiece.java:63)
- [ ] `MountainPiece.getWorldgenSurface(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/MountainPiece.java:76)
- [ ] `MountainPiece.getHeight(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/MountainPiece.java:87)
- [ ] `MountainPiece.getHeightClamp(4)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/MountainPiece.java:115)
- [ ] `MountainPiece.makeBoundingBox(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/MountainPiece.java:140)

### `NBTPiece`: it extends `BasePiece`

- [ ] `NBTPiece.NBTPiece(6)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/NBTPiece.java:34)
- [ ] `NBTPiece.NBTPiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/NBTPiece.java:46)
- [ ] `NBTPiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/NBTPiece.java:51)
- [ ] `NBTPiece.fromNbt(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/NBTPiece.java:62)
- [ ] `NBTPiece.postProcess(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/NBTPiece.java:73)
- [ ] `NBTPiece.makeBoundingBox(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/NBTPiece.java:95)
- [ ] `NBTPiece.offsetPos(4)`: it uses Minecraft's `BlockPos` (org/betterx/betterend/world/structures/piece/NBTPiece.java:99)
- [ ] `NBTPiece.getStructureBounds(4)`: it uses Minecraft's `BoundingBox` (org/betterx/betterend/world/structures/piece/NBTPiece.java:104)

### `PaintedMountainPiece`: it extends `MountainPiece`

- [ ] `PaintedMountainPiece.PaintedMountainPiece(6)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/PaintedMountainPiece.java:32)
- [ ] `PaintedMountainPiece.PaintedMountainPiece(2)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/PaintedMountainPiece.java:37)
- [ ] `PaintedMountainPiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/PaintedMountainPiece.java:41)
- [ ] `PaintedMountainPiece.fromNbt(1)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/PaintedMountainPiece.java:53)
- [ ] `PaintedMountainPiece.postProcess(7)`: its class isn't translated (it extends `MountainPiece`) (org/betterx/betterend/world/structures/piece/PaintedMountainPiece.java:65)

### `SteppedSkirt`

- [x] `SteppedSkirt.NO_GROUND` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:12)
- [x] `SteppedSkirt.NO_FLUID` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:13)
- [x] `SteppedSkirt.RUN` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:14)
- [x] `SteppedSkirt.JITTER` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:15)
- [x] `SteppedSkirt.SteppedSkirt(2)` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:19)
- [x] `SteppedSkirt.height(4)` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:24)
- [x] `SteppedSkirt.taper(3)` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:30)
- [ ] `SteppedSkirt.probe(4)`: it uses Ground, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:40)
- [ ] `SteppedSkirt.fill(6)`: it uses Minecraft's `ChunkAccess` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:76)
- [ ] `SteppedSkirt.passable(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:101)

### `SteppedSkirt.Ground`: it uses Minecraft's `BlockState`

- [ ] `SteppedSkirt.Ground.Ground(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:105)
- [ ] `SteppedSkirt.Ground.found(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:106)
- [ ] `SteppedSkirt.Ground.overFluid(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:110)
- [ ] `SteppedSkirt.Ground.reference(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:114)
- [ ] `SteppedSkirt.Ground.groundY(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:105)
- [ ] `SteppedSkirt.Ground.fluidTop(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:105)
- [ ] `SteppedSkirt.Ground.under(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betterend/world/structures/piece/SteppedSkirt.java:105)

### `SulphuricCavePiece`: it extends `BasePiece`

- [ ] `SulphuricCavePiece.CAVE_AIR`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:43)
- [ ] `SulphuricCavePiece.WATER`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:44)
- [ ] `SulphuricCavePiece.HORIZONTAL`: it uses Minecraft's `Direction` (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:45)
- [ ] `SulphuricCavePiece.SulphuricCavePiece(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:52)
- [ ] `SulphuricCavePiece.SulphuricCavePiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:61)
- [ ] `SulphuricCavePiece.resolveVentColumns(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:66)
- [ ] `SulphuricCavePiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:82)
- [ ] `SulphuricCavePiece.fromNbt(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:91)
- [ ] `SulphuricCavePiece.makeBoundingBox(0)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:100)
- [ ] `SulphuricCavePiece.isReplaceable(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:111)
- [ ] `SulphuricCavePiece.postProcess(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:122)
- [ ] `SulphuricCavePiece.placeVentColumn(9)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:204)
- [ ] `SulphuricCavePiece.placeBrimstone(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:261)
- [ ] `SulphuricCavePiece.placeSulfurSpikeClusters(8)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:277)
- [ ] `SulphuricCavePiece.isSpikeAnchor(1)`: it uses Minecraft's `BlockState` (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:314)
- [ ] `SulphuricCavePiece.placeSpike(9)`: it uses Minecraft's `ChunkAccess` (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:321)
- [ ] `SulphuricCavePiece.isBuriedInChunk(2)`: it uses Minecraft's `ChunkAccess` (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:359)
- [ ] `SulphuricCavePiece.facesOpenSpaceInChunk(2)`: it uses Minecraft's `ChunkAccess` (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:377)
- [ ] `SulphuricCavePiece.getBrimstone(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:399)
- [ ] `SulphuricCavePiece.makeShards(3)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java:409)

### `VoxelPiece`: it extends `BasePiece`

- [ ] `VoxelPiece.VoxelPiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/VoxelPiece.java:23)
- [ ] `VoxelPiece.VoxelPiece(2)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/VoxelPiece.java:30)
- [ ] `VoxelPiece.addAdditionalSaveData(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/VoxelPiece.java:35)
- [ ] `VoxelPiece.fromNbt(1)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/VoxelPiece.java:40)
- [ ] `VoxelPiece.postProcess(7)`: its class isn't translated (it extends `BasePiece`) (org/betterx/betterend/world/structures/piece/VoxelPiece.java:45)

### `VillagePools`

- [ ] `VillagePools.CHORUS_VILLAGE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betterend/world/structures/village/VillagePools.java:10)
- [ ] `VillagePools.TERMINATORS_KEY`: it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools) (org/betterx/betterend/world/structures/village/VillagePools.java:11)
- [ ] `VillagePools.START`: it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools) (org/betterx/betterend/world/structures/village/VillagePools.java:12)
- [ ] `VillagePools.HOUSES_KEY`: it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools) (org/betterx/betterend/world/structures/village/VillagePools.java:13)
- [ ] `VillagePools.STREET_KEY`: it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools) (org/betterx/betterend/world/structures/village/VillagePools.java:14)
- [ ] `VillagePools.STREET_DECO_KEY`: it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools) (org/betterx/betterend/world/structures/village/VillagePools.java:15)
- [ ] `VillagePools.DECORATIONS_KEY`: it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools) (org/betterx/betterend/world/structures/village/VillagePools.java:16)

### `SplitNoiseCondition`: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)

- [ ] `SplitNoiseCondition.DEFAULT`: it uses SplitNoiseCondition, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/SplitNoiseCondition.java:13)
- [ ] `SplitNoiseCondition.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/surface/SplitNoiseCondition.java:14)
- [ ] `SplitNoiseCondition.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/surface/SplitNoiseCondition.java:15)
- [x] `SplitNoiseCondition.SEED` (org/betterx/betterend/world/surface/SplitNoiseCondition.java:16)
- [ ] `SplitNoiseCondition.getNumber(1)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/SplitNoiseCondition.java:18)
- [ ] `SplitNoiseCondition.getNoise(2)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/SplitNoiseCondition.java:26)
- [ ] `SplitNoiseCondition.getNoise(3)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/SplitNoiseCondition.java:31)
- [ ] `SplitNoiseCondition.pcodec(0)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/SplitNoiseCondition.java:37)

### `SulphuricSurfaceNoiseCondition`: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)

- [ ] `SulphuricSurfaceNoiseCondition.DEFAULT`: it uses SulphuricSurfaceNoiseCondition, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/SulphuricSurfaceNoiseCondition.java:13)
- [ ] `SulphuricSurfaceNoiseCondition.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/surface/SulphuricSurfaceNoiseCondition.java:14)
- [ ] `SulphuricSurfaceNoiseCondition.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/surface/SulphuricSurfaceNoiseCondition.java:15)
- [x] `SulphuricSurfaceNoiseCondition.SEED` (org/betterx/betterend/world/surface/SulphuricSurfaceNoiseCondition.java:16)
- [ ] `SulphuricSurfaceNoiseCondition.getNumber(1)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/SulphuricSurfaceNoiseCondition.java:18)
- [ ] `SulphuricSurfaceNoiseCondition.pcodec(0)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/SulphuricSurfaceNoiseCondition.java:32)

### `UmbraSurfaceNoiseCondition`: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)

- [ ] `UmbraSurfaceNoiseCondition.DEFAULT`: it uses UmbraSurfaceNoiseCondition, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java:13)
- [ ] `UmbraSurfaceNoiseCondition.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java:14)
- [ ] `UmbraSurfaceNoiseCondition.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java:15)
- [x] `UmbraSurfaceNoiseCondition.SEED` (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java:16)
- [ ] `UmbraSurfaceNoiseCondition.getNumber(1)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java:18)
- [ ] `UmbraSurfaceNoiseCondition.getDepth(2)`: it calls UmbraSurfaceNoiseCondition.getDepth, which isn't translated (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java:22)
- [ ] `UmbraSurfaceNoiseCondition.getDepth(3)`: it calls `MathHelper` (de.ambertation.wover.math.api).getSeed (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java:27)
- [ ] `UmbraSurfaceNoiseCondition.pcodec(0)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java:41)

### `VerticalBandNoiseCondition`: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)

- [ ] `VerticalBandNoiseCondition.DEFAULT`: it uses VerticalBandNoiseCondition, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/VerticalBandNoiseCondition.java:15)
- [ ] `VerticalBandNoiseCondition.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betterend/world/surface/VerticalBandNoiseCondition.java:16)
- [ ] `VerticalBandNoiseCondition.VerticalBandNoiseCondition(5)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/VerticalBandNoiseCondition.java:32)
- [ ] `VerticalBandNoiseCondition.getNumber(1)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/VerticalBandNoiseCondition.java:40)
- [ ] `VerticalBandNoiseCondition.pcodec(0)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betterend/world/surface/VerticalBandNoiseCondition.java:47)

### `EndAdvancementDataProvider`: it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)

- [ ] `EndAdvancementDataProvider.EndAdvancementDataProvider(2)`: its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/advancement/EndAdvancementDataProvider.java:40)
- [ ] `EndAdvancementDataProvider.bootstrap(1)`: its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/advancement/EndAdvancementDataProvider.java:44)
- [ ] `EndAdvancementDataProvider.addTools(1)`: its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/advancement/EndAdvancementDataProvider.java:267)
- [ ] `EndAdvancementDataProvider.addToolHeads(1)`: its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/advancement/EndAdvancementDataProvider.java:278)
- [ ] `EndAdvancementDataProvider.addArmor(1)`: its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/advancement/EndAdvancementDataProvider.java:289)

### `BetterEndDatagen`: it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)

- [ ] `BetterEndDatagen.onInitializeProviders(1)`: its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/BetterEndDatagen.java:58)
- [ ] `BetterEndDatagen.modCore(0)`: its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/BetterEndDatagen.java:122)
- [ ] `BetterEndDatagen.onBuildRegistry(1)`: its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/BetterEndDatagen.java:126)

### `CrystalSubblocksDefinitions`

- [ ] `CrystalSubblocksDefinitions.LIT_STAIRS`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:24)
- [ ] `CrystalSubblocksDefinitions.LIT_STAIRS_INNER`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:25)
- [ ] `CrystalSubblocksDefinitions.LIT_STAIRS_OUTER`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:26)
- [ ] `CrystalSubblocksDefinitions.LIT_WALL_POST`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:27)
- [ ] `CrystalSubblocksDefinitions.LIT_WALL_SIDE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:28)
- [ ] `CrystalSubblocksDefinitions.LIT_WALL_SIDE_TALL`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:29)
- [x] `CrystalSubblocksDefinitions.CrystalSubblocksDefinitions(0)` (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:31)
- [ ] `CrystalSubblocksDefinitions.define(2)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:34)
- [ ] `CrystalSubblocksDefinitions.block(4)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:119)
- [ ] `CrystalSubblocksDefinitions.recipe(4)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java:123)

### `EndBlockDefinitions`: it implements `BlockDefinitionsEntrypoint` (de.ambertation.wover.block.api.classify)

- [ ] `EndBlockDefinitions.defineBlocks(1)`: its class isn't translated (it implements `BlockDefinitionsEntrypoint` (de.ambertation.wover.block.api.classify)) (org/betterx/datagen/betterend/blocks/EndBlockDefinitions.java:15)

### `EndCropBlockDefinitions`

- [x] `EndCropBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndCropBlockDefinitions.java:37)
- [x] `EndCropBlockDefinitions.EndCropBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndCropBlockDefinitions.java:39)
- [ ] `EndCropBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndCropBlockDefinitions.java:42)
- [ ] `EndCropBlockDefinitions.crop(7)`: it's generic (org/betterx/datagen/betterend/blocks/EndCropBlockDefinitions.java:213)
- [ ] `EndCropBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndCropBlockDefinitions.java:227)

### `EndCrystalBlockDefinitions`

- [x] `EndCrystalBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndCrystalBlockDefinitions.java:33)
- [x] `EndCrystalBlockDefinitions.EndCrystalBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndCrystalBlockDefinitions.java:35)
- [ ] `EndCrystalBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndCrystalBlockDefinitions.java:38)
- [ ] `EndCrystalBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndCrystalBlockDefinitions.java:95)

### `EndDecorBlockDefinitions`

- [x] `EndDecorBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndDecorBlockDefinitions.java:23)
- [x] `EndDecorBlockDefinitions.EndDecorBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndDecorBlockDefinitions.java:25)
- [ ] `EndDecorBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndDecorBlockDefinitions.java:28)
- [ ] `EndDecorBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndDecorBlockDefinitions.java:96)

### `EndFunctionalBlockDefinitions`

- [x] `EndFunctionalBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndFunctionalBlockDefinitions.java:46)
- [x] `EndFunctionalBlockDefinitions.EndFunctionalBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndFunctionalBlockDefinitions.java:48)
- [ ] `EndFunctionalBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndFunctionalBlockDefinitions.java:51)
- [ ] `EndFunctionalBlockDefinitions.pedestal(6)`: it's generic (org/betterx/datagen/betterend/blocks/EndFunctionalBlockDefinitions.java:144)
- [ ] `EndFunctionalBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndFunctionalBlockDefinitions.java:154)

### `EndLightBlockDefinitions`

- [x] `EndLightBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndLightBlockDefinitions.java:21)
- [x] `EndLightBlockDefinitions.EndLightBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndLightBlockDefinitions.java:23)
- [ ] `EndLightBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndLightBlockDefinitions.java:26)
- [ ] `EndLightBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndLightBlockDefinitions.java:67)

### `EndMetalBlockDefinitions`

- [x] `EndMetalBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndMetalBlockDefinitions.java:20)
- [x] `EndMetalBlockDefinitions.EndMetalBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndMetalBlockDefinitions.java:22)
- [ ] `EndMetalBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndMetalBlockDefinitions.java:25)

### `EndMushroomBlockDefinitions`

- [x] `EndMushroomBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndMushroomBlockDefinitions.java:35)
- [x] `EndMushroomBlockDefinitions.EndMushroomBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndMushroomBlockDefinitions.java:37)
- [ ] `EndMushroomBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndMushroomBlockDefinitions.java:40)
- [ ] `EndMushroomBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndMushroomBlockDefinitions.java:224)

### `EndOreBlockDefinitions`

- [x] `EndOreBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndOreBlockDefinitions.java:21)
- [x] `EndOreBlockDefinitions.EndOreBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndOreBlockDefinitions.java:23)
- [ ] `EndOreBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndOreBlockDefinitions.java:26)
- [ ] `EndOreBlockDefinitions.ore(6)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betterend/blocks/EndOreBlockDefinitions.java:34)

### `EndPlantBlockDefinitions`

- [x] `EndPlantBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndPlantBlockDefinitions.java:37)
- [x] `EndPlantBlockDefinitions.EndPlantBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndPlantBlockDefinitions.java:39)
- [ ] `EndPlantBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndPlantBlockDefinitions.java:42)
- [ ] `EndPlantBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndPlantBlockDefinitions.java:591)

### `EndSaplingBlockDefinitions`

- [x] `EndSaplingBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndSaplingBlockDefinitions.java:32)
- [x] `EndSaplingBlockDefinitions.EndSaplingBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndSaplingBlockDefinitions.java:34)
- [ ] `EndSaplingBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndSaplingBlockDefinitions.java:37)
- [ ] `EndSaplingBlockDefinitions.sapling(5)`: it's generic (org/betterx/datagen/betterend/blocks/EndSaplingBlockDefinitions.java:97)
- [ ] `EndSaplingBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndSaplingBlockDefinitions.java:109)

### `EndStoneBlockDefinitions`

- [x] `EndStoneBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndStoneBlockDefinitions.java:47)
- [ ] `EndStoneBlockDefinitions.NO_RECIPE`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/datagen/betterend/blocks/EndStoneBlockDefinitions.java:48)
- [x] `EndStoneBlockDefinitions.EndStoneBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndStoneBlockDefinitions.java:50)
- [ ] `EndStoneBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndStoneBlockDefinitions.java:53)
- [ ] `EndStoneBlockDefinitions.runedFlavolite(4)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betterend/blocks/EndStoneBlockDefinitions.java:181)
- [ ] `EndStoneBlockDefinitions.stalactite(3)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betterend/blocks/EndStoneBlockDefinitions.java:189)
- [ ] `EndStoneBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndStoneBlockDefinitions.java:196)

### `EndTerrainBlockDefinitions`

- [x] `EndTerrainBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndTerrainBlockDefinitions.java:33)
- [x] `EndTerrainBlockDefinitions.EndTerrainBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndTerrainBlockDefinitions.java:35)
- [ ] `EndTerrainBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndTerrainBlockDefinitions.java:38)
- [ ] `EndTerrainBlockDefinitions.terrain(6)`: it's generic (org/betterx/datagen/betterend/blocks/EndTerrainBlockDefinitions.java:123)
- [ ] `EndTerrainBlockDefinitions.terrain(8)`: it's generic (org/betterx/datagen/betterend/blocks/EndTerrainBlockDefinitions.java:129)
- [ ] `EndTerrainBlockDefinitions.path(4)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betterend/blocks/EndTerrainBlockDefinitions.java:155)
- [ ] `EndTerrainBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndTerrainBlockDefinitions.java:164)

### `EndVineBlockDefinitions`

- [x] `EndVineBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndVineBlockDefinitions.java:37)
- [x] `EndVineBlockDefinitions.EndVineBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndVineBlockDefinitions.java:39)
- [ ] `EndVineBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndVineBlockDefinitions.java:42)
- [ ] `EndVineBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndVineBlockDefinitions.java:237)

### `EndWallPlantBlockDefinitions`

- [x] `EndWallPlantBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndWallPlantBlockDefinitions.java:20)
- [x] `EndWallPlantBlockDefinitions.EndWallPlantBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndWallPlantBlockDefinitions.java:22)
- [ ] `EndWallPlantBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndWallPlantBlockDefinitions.java:25)
- [ ] `EndWallPlantBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndWallPlantBlockDefinitions.java:99)

### `EndWaterPlantBlockDefinitions`

- [x] `EndWaterPlantBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndWaterPlantBlockDefinitions.java:42)
- [x] `EndWaterPlantBlockDefinitions.EndWaterPlantBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndWaterPlantBlockDefinitions.java:44)
- [ ] `EndWaterPlantBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndWaterPlantBlockDefinitions.java:47)
- [ ] `EndWaterPlantBlockDefinitions.charnia(3)`: it uses Minecraft's `Block` (org/betterx/datagen/betterend/blocks/EndWaterPlantBlockDefinitions.java:273)
- [ ] `EndWaterPlantBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndWaterPlantBlockDefinitions.java:281)

### `EndWoodBlockDefinitions`

- [x] `EndWoodBlockDefinitions.defined` (org/betterx/datagen/betterend/blocks/EndWoodBlockDefinitions.java:37)
- [x] `EndWoodBlockDefinitions.CANOPY_SAPLING_CHANCE` (org/betterx/datagen/betterend/blocks/EndWoodBlockDefinitions.java:38)
- [ ] `EndWoodBlockDefinitions.TENANEA_PALETTE`: it uses `PaletteCycle` (de.ambertation.wover.block.impl.classify.TintTrait) (org/betterx/datagen/betterend/blocks/EndWoodBlockDefinitions.java:39)
- [x] `EndWoodBlockDefinitions.EndWoodBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/EndWoodBlockDefinitions.java:47)
- [ ] `EndWoodBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/EndWoodBlockDefinitions.java:50)
- [ ] `EndWoodBlockDefinitions.leaves(5)`: it uses Minecraft's `Block` (org/betterx/datagen/betterend/blocks/EndWoodBlockDefinitions.java:190)
- [ ] `EndWoodBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betterend/blocks/EndWoodBlockDefinitions.java:203)

### `FlamboyantBlockDefinitions`

- [ ] `FlamboyantBlockDefinitions.petals`: it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/datagen/betterend/blocks/FlamboyantBlockDefinitions.java:21)
- [x] `FlamboyantBlockDefinitions.FlamboyantBlockDefinitions(0)` (org/betterx/datagen/betterend/blocks/FlamboyantBlockDefinitions.java:23)
- [ ] `FlamboyantBlockDefinitions.petals(0)`: it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/datagen/betterend/blocks/FlamboyantBlockDefinitions.java:26)
- [ ] `FlamboyantBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/FlamboyantBlockDefinitions.java:30)

### `MetalMaterialDefinitions`

- [x] `MetalMaterialDefinitions.MetalMaterialDefinitions(0)` (org/betterx/datagen/betterend/blocks/MetalMaterialDefinitions.java:14)
- [ ] `MetalMaterialDefinitions.define(2)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/blocks/MetalMaterialDefinitions.java:17)

### `EndModelProvider`: it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `EndModelProvider.bootstrapItemModels(1)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/EndModelProvider.java:27)
- [ ] `EndModelProvider.bootstrapBlockStateModels(1)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/EndModelProvider.java:43)
- [ ] `EndModelProvider.buildModelOverrides(1)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/EndModelProvider.java:51)
- [ ] `EndModelProvider.EndModelProvider(1)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/EndModelProvider.java:66)

### `EndPottablePlantProvider`: it extends `WoverPottablePlantRegistryProvider` (de.ambertation.wover.pottable.api.datagen)

- [ ] `EndPottablePlantProvider.VANILLA_PLANTS`: it uses Minecraft's `Block` (org/betterx/datagen/betterend/EndPottablePlantProvider.java:17)
- [ ] `EndPottablePlantProvider.EndPottablePlantProvider(1)`: its class isn't translated (it extends `WoverPottablePlantRegistryProvider` (de.ambertation.wover.pottable.api.datagen)) (org/betterx/datagen/betterend/EndPottablePlantProvider.java:57)
- [ ] `EndPottablePlantProvider.bootstrap(1)`: its class isn't translated (it extends `WoverPottablePlantRegistryProvider` (de.ambertation.wover.pottable.api.datagen)) (org/betterx/datagen/betterend/EndPottablePlantProvider.java:61)

### `EndPottableSoilProvider`: it extends `WoverPottableSoilRegistryProvider` (de.ambertation.wover.pottable.api.datagen)

- [ ] `EndPottableSoilProvider.VANILLA_SOILS`: it uses Minecraft's `Block` (org/betterx/datagen/betterend/EndPottableSoilProvider.java:16)
- [ ] `EndPottableSoilProvider.EndPottableSoilProvider(1)`: its class isn't translated (it extends `WoverPottableSoilRegistryProvider` (de.ambertation.wover.pottable.api.datagen)) (org/betterx/datagen/betterend/EndPottableSoilProvider.java:20)
- [ ] `EndPottableSoilProvider.bootstrap(1)`: its class isn't translated (it extends `WoverPottableSoilRegistryProvider` (de.ambertation.wover.pottable.api.datagen)) (org/betterx/datagen/betterend/EndPottableSoilProvider.java:24)

### `EndRegistrySupplier`: it extends `RegistrySupplier` (org.betterx.bclib.api.v3.datagen)

- [ ] `EndRegistrySupplier.INSTANCE`: it uses EndRegistrySupplier, which isn't translated (it extends `RegistrySupplier` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/EndRegistrySupplier.java:10)
- [ ] `EndRegistrySupplier.EndRegistrySupplier(0)`: its class isn't translated (it extends `RegistrySupplier` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/EndRegistrySupplier.java:12)
- [ ] `EndRegistrySupplier.initializeRegistryList(1)`: its class isn't translated (it extends `RegistrySupplier` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/EndRegistrySupplier.java:16)

### `GlimmerwildBiomeModificationProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [ ] `GlimmerwildBiomeModificationProvider.GlimmerwildBiomeModificationProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/glimmerwild/GlimmerwildBiomeModificationProvider.java:14)
- [ ] `GlimmerwildBiomeModificationProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/glimmerwild/GlimmerwildBiomeModificationProvider.java:18)

### `GlimmerwildBlockTagProvider`: it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `GlimmerwildBlockTagProvider.WOOLLY_GROUND`: it uses Minecraft's `TagKey` (org/betterx/datagen/betterend/glimmerwild/GlimmerwildBlockTagProvider.java:14)
- [ ] `GlimmerwildBlockTagProvider.GlimmerwildBlockTagProvider(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/glimmerwild/GlimmerwildBlockTagProvider.java:16)
- [ ] `GlimmerwildBlockTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/glimmerwild/GlimmerwildBlockTagProvider.java:20)

### `GlimmerwildFeaturesProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `GlimmerwildFeaturesProvider.GlimmerwildFeaturesProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/glimmerwild/GlimmerwildFeaturesProvider.java:20)
- [ ] `GlimmerwildFeaturesProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/glimmerwild/GlimmerwildFeaturesProvider.java:24)
- [ ] `GlimmerwildFeaturesProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/glimmerwild/GlimmerwildFeaturesProvider.java:27)
- [ ] `GlimmerwildFeaturesProvider.single(5)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/glimmerwild/GlimmerwildFeaturesProvider.java:51)
- [ ] `GlimmerwildFeaturesProvider.double_(6)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/glimmerwild/GlimmerwildFeaturesProvider.java:55)
- [ ] `GlimmerwildFeaturesProvider.register(4)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/glimmerwild/GlimmerwildFeaturesProvider.java:59)

### `CrystaliteDefinitions`

- [x] `CrystaliteDefinitions.CrystaliteDefinitions(0)` (org/betterx/datagen/betterend/items/CrystaliteDefinitions.java:29)
- [ ] `CrystaliteDefinitions.helmet(2)`: it uses `ItemDefinition` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/CrystaliteDefinitions.java:32)
- [ ] `CrystaliteDefinitions.chestplate(2)`: it uses `ItemDefinition` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/CrystaliteDefinitions.java:48)
- [ ] `CrystaliteDefinitions.leggings(2)`: it uses `ItemDefinition` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/CrystaliteDefinitions.java:52)
- [ ] `CrystaliteDefinitions.boots(2)`: it uses `ItemDefinition` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/CrystaliteDefinitions.java:62)
- [ ] `CrystaliteDefinitions.armor(5)`: it's generic (org/betterx/datagen/betterend/items/CrystaliteDefinitions.java:66)

### `EndArmorStats`

- [x] `EndArmorStats.EndArmorStats(0)` (org/betterx/datagen/betterend/items/EndArmorStats.java:22)
- [ ] `EndArmorStats.apply(3)`: it's generic (org/betterx/datagen/betterend/items/EndArmorStats.java:25)
- [ ] `EndArmorStats.apply(4)`: it's generic (org/betterx/datagen/betterend/items/EndArmorStats.java:29)
- [ ] `EndArmorStats.apply(8)`: it's generic (org/betterx/datagen/betterend/items/EndArmorStats.java:33)
- [ ] `EndArmorStats.apply(9)`: it's generic (org/betterx/datagen/betterend/items/EndArmorStats.java:46)

### `EndDebugItemDefinitions`

- [x] `EndDebugItemDefinitions.EndDebugItemDefinitions(0)` (org/betterx/datagen/betterend/items/EndDebugItemDefinitions.java:12)
- [ ] `EndDebugItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndDebugItemDefinitions.java:15)

### `EndDiscItemDefinitions`

- [x] `EndDiscItemDefinitions.defined` (org/betterx/datagen/betterend/items/EndDiscItemDefinitions.java:8)
- [x] `EndDiscItemDefinitions.EndDiscItemDefinitions(0)` (org/betterx/datagen/betterend/items/EndDiscItemDefinitions.java:10)
- [ ] `EndDiscItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndDiscItemDefinitions.java:13)

### `EndEquipmentItemDefinitions`

- [x] `EndEquipmentItemDefinitions.defined` (org/betterx/datagen/betterend/items/EndEquipmentItemDefinitions.java:31)
- [x] `EndEquipmentItemDefinitions.EndEquipmentItemDefinitions(0)` (org/betterx/datagen/betterend/items/EndEquipmentItemDefinitions.java:33)
- [ ] `EndEquipmentItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndEquipmentItemDefinitions.java:36)
- [ ] `EndEquipmentItemDefinitions.hammer(4)`: it uses `ItemDefinition` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndEquipmentItemDefinitions.java:81)

### `EndFoodItemDefinitions`

- [x] `EndFoodItemDefinitions.defined` (org/betterx/datagen/betterend/items/EndFoodItemDefinitions.java:16)
- [x] `EndFoodItemDefinitions.EndFoodItemDefinitions(0)` (org/betterx/datagen/betterend/items/EndFoodItemDefinitions.java:18)
- [ ] `EndFoodItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndFoodItemDefinitions.java:21)

### `EndItemDefinitions`

- [x] `EndItemDefinitions.EndItemDefinitions(0)` (org/betterx/datagen/betterend/items/EndItemDefinitions.java:19)
- [ ] `EndItemDefinitions.registerEndDisc(3)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndItemDefinitions.java:22)
- [ ] `EndItemDefinitions.defineEndItem(2)`: it uses `ItemDefinition` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndItemDefinitions.java:26)
- [ ] `EndItemDefinitions.registerEndItem(2)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndItemDefinitions.java:30)
- [ ] `EndItemDefinitions.registerEndFood(5)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndItemDefinitions.java:34)
- [ ] `EndItemDefinitions.registerEndFood(3)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndItemDefinitions.java:43)
- [ ] `EndItemDefinitions.registerEndDrink(4)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndItemDefinitions.java:50)

### `EndResourceItemDefinitions`

- [x] `EndResourceItemDefinitions.defined` (org/betterx/datagen/betterend/items/EndResourceItemDefinitions.java:12)
- [x] `EndResourceItemDefinitions.EndResourceItemDefinitions(0)` (org/betterx/datagen/betterend/items/EndResourceItemDefinitions.java:14)
- [ ] `EndResourceItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndResourceItemDefinitions.java:17)

### `EndSpawnEggDefinitions`

- [x] `EndSpawnEggDefinitions.defined` (org/betterx/datagen/betterend/items/EndSpawnEggDefinitions.java:8)
- [x] `EndSpawnEggDefinitions.EndSpawnEggDefinitions(0)` (org/betterx/datagen/betterend/items/EndSpawnEggDefinitions.java:10)
- [ ] `EndSpawnEggDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndSpawnEggDefinitions.java:13)

### `EndTemplatesDefinitions`

- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_STICK`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:15)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_HANDLE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:16)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_SWORD_HANDLE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:17)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_SWORD_BLADE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:18)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_SPEAR_TIP`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:19)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_PLATE`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:20)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_HAMMER`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:21)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_HAMMER_HEAD`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:22)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_PICKAXE_HEAD`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:23)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_AXE_HEAD`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:24)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_HOE_HEAD`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:25)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_SHOVEL_HEAD`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:26)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_ANVIL`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:27)
- [ ] `EndTemplatesDefinitions.EMPTY_SLOT_ELYTRA`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:28)
- [x] `EndTemplatesDefinitions.defined` (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:29)
- [x] `EndTemplatesDefinitions.EndTemplatesDefinitions(0)` (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:31)
- [ ] `EndTemplatesDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:34)
- [ ] `EndTemplatesDefinitions.smithingTemplate(2)`: it uses `ItemDefinition` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java:78)

### `GuideBookItemDefinitions`

- [x] `GuideBookItemDefinitions.defined` (org/betterx/datagen/betterend/items/GuideBookItemDefinitions.java:10)
- [x] `GuideBookItemDefinitions.GuideBookItemDefinitions(0)` (org/betterx/datagen/betterend/items/GuideBookItemDefinitions.java:12)
- [ ] `GuideBookItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/GuideBookItemDefinitions.java:15)

### `ToolsWithHeadsSetDefinitions`

- [x] `ToolsWithHeadsSetDefinitions.ToolsWithHeadsSetDefinitions(0)` (org/betterx/datagen/betterend/items/ToolsWithHeadsSetDefinitions.java:15)
- [ ] `ToolsWithHeadsSetDefinitions.define(2)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/ToolsWithHeadsSetDefinitions.java:18)
- [ ] `ToolsWithHeadsSetDefinitions.head(4)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/items/ToolsWithHeadsSetDefinitions.java:40)

### `AlloyingRecipesProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `AlloyingRecipesProvider.AlloyingRecipesProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/AlloyingRecipesProvider.java:16)
- [ ] `AlloyingRecipesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/AlloyingRecipesProvider.java:20)

### `AnvilRecipesProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `AnvilRecipesProvider.AnvilRecipesProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/AnvilRecipesProvider.java:17)
- [ ] `AnvilRecipesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/AnvilRecipesProvider.java:21)

### `EndBlockLootTableProvider`: it extends `BlockLootTableProvider` (org.betterx.bclib.api.v3.datagen)

- [ ] `EndBlockLootTableProvider.EndBlockLootTableProvider(2)`: its class isn't translated (it extends `BlockLootTableProvider` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betterend/recipes/EndBlockLootTableProvider.java:11)

### `EndChestLootTableProvider`: it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `EndChestLootTableProvider.IN_FOGGY_MUSHROOMLAND`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:56)
- [ ] `EndChestLootTableProvider.IN_CHORUS_FOREST`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:57)
- [ ] `EndChestLootTableProvider.IN_AMBER_LAND`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:58)
- [ ] `EndChestLootTableProvider.IN_GLOWING_GRASSLANDS`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:59)
- [ ] `EndChestLootTableProvider.IN_LANTERN_WOODS`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:60)
- [ ] `EndChestLootTableProvider.IN_MEGALAKE`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:61)
- [ ] `EndChestLootTableProvider.IN_MEGALAKE_GROVE`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:62)
- [ ] `EndChestLootTableProvider.IN_NEON_OASIS`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:63)
- [ ] `EndChestLootTableProvider.IN_SHADOW_FOREST`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:64)
- [ ] `EndChestLootTableProvider.IN_SULPHUR_SPRINGS`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:65)
- [ ] `EndChestLootTableProvider.IN_UMBRELLA_JUNGLE`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:66)
- [ ] `EndChestLootTableProvider.EndChestLootTableProvider(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:68)
- [ ] `EndChestLootTableProvider.endCityGhastTear(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:72)
- [ ] `EndChestLootTableProvider.endCityMusicDiscs(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:79)
- [ ] `EndChestLootTableProvider.endCitySmithingTemplates(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:88)
- [ ] `EndChestLootTableProvider.endCityEndVeilBook(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:101)
- [ ] `EndChestLootTableProvider.resonanceBook(2)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:114)
- [ ] `EndChestLootTableProvider.fishing(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:127)
- [ ] `EndChestLootTableProvider.fishingJunk(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:131)
- [ ] `EndChestLootTableProvider.enchantments(1)`: it uses Minecraft's `HolderGetter` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:147)
- [ ] `EndChestLootTableProvider.fishingTreasure(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:151)
- [ ] `EndChestLootTableProvider.foggyMushroomland(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:178)
- [ ] `EndChestLootTableProvider.chorusForest(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:186)
- [ ] `EndChestLootTableProvider.shadowForest(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:195)
- [ ] `EndChestLootTableProvider.lanternWoods(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:204)
- [ ] `EndChestLootTableProvider.umbrellaJungle(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:212)
- [ ] `EndChestLootTableProvider.upgradeLootPool(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:220)
- [ ] `EndChestLootTableProvider.plateUpgradeLootPool(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:245)
- [ ] `EndChestLootTableProvider.leatherHandleLootPool(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:257)
- [ ] `EndChestLootTableProvider.handleLootPool(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:269)
- [ ] `EndChestLootTableProvider.villageBonusLoot(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:284)
- [ ] `EndChestLootTableProvider.elytraLoot(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:309)
- [ ] `EndChestLootTableProvider.simpleVillageLoot(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:325)
- [ ] `EndChestLootTableProvider.addCharnia(1)`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:362)
- [ ] `EndChestLootTableProvider.includeCommonItems(1)`: it uses Builder, which isn't translated (it's an interface) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:389)
- [ ] `EndChestLootTableProvider.biomePredicate(2)`: it uses Minecraft's `Builder` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:427)
- [ ] `EndChestLootTableProvider.boostrap(2)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java:435)

### `EndCraftingRecipesProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `EndCraftingRecipesProvider.EndCraftingRecipesProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndCraftingRecipesProvider.java:35)
- [ ] `EndCraftingRecipesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndCraftingRecipesProvider.java:39)
- [ ] `EndCraftingRecipesProvider.registerLantern(4)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betterend/recipes/EndCraftingRecipesProvider.java:339)
- [ ] `EndCraftingRecipesProvider.registerPedestal(5)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betterend/recipes/EndCraftingRecipesProvider.java:347)
- [ ] `EndCraftingRecipesProvider.registerHammer(4)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betterend/recipes/EndCraftingRecipesProvider.java:355)

### `EndEnchantmentProvider`: it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `EndEnchantmentProvider.EndEnchantmentProvider(1)`: its class isn't translated (it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndEnchantmentProvider.java:16)
- [ ] `EndEnchantmentProvider.bootstrap(1)`: its class isn't translated (it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndEnchantmentProvider.java:20)

### `EndFurnaceRecipeProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `EndFurnaceRecipeProvider.EndFurnaceRecipeProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndFurnaceRecipeProvider.java:18)
- [ ] `EndFurnaceRecipeProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndFurnaceRecipeProvider.java:22)

### `EndLootAdditionProvider`: it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `EndLootAdditionProvider.EndLootAdditionProvider(1)`: its class isn't translated (it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndLootAdditionProvider.java:21)
- [ ] `EndLootAdditionProvider.bootstrap(2)`: its class isn't translated (it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndLootAdditionProvider.java:25)

### `EndMaterialRecipesProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `EndMaterialRecipesProvider.EndMaterialRecipesProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndMaterialRecipesProvider.java:10)
- [ ] `EndMaterialRecipesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/EndMaterialRecipesProvider.java:14)

### `FlamboyantRecipeProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `FlamboyantRecipeProvider.FlamboyantRecipeProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/FlamboyantRecipeProvider.java:13)
- [ ] `FlamboyantRecipeProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/FlamboyantRecipeProvider.java:17)

### `InfusionRecipesProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `InfusionRecipesProvider.InfusionRecipesProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/InfusionRecipesProvider.java:25)
- [ ] `InfusionRecipesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/InfusionRecipesProvider.java:29)

### `JukeboxRegistryProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [ ] `JukeboxRegistryProvider.JukeboxRegistryProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/recipes/JukeboxRegistryProvider.java:16)
- [ ] `JukeboxRegistryProvider.register(5)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/recipes/JukeboxRegistryProvider.java:20)
- [ ] `JukeboxRegistryProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/recipes/JukeboxRegistryProvider.java:33)

### `PatchouliBookProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `PatchouliBookProvider.PatchouliBookProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/PatchouliBookProvider.java:14)
- [ ] `PatchouliBookProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/PatchouliBookProvider.java:18)

### `SmithingRecipesProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `SmithingRecipesProvider.SmithingRecipesProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/SmithingRecipesProvider.java:20)
- [ ] `SmithingRecipesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betterend/recipes/SmithingRecipesProvider.java:24)

### `BiomeTagProvider`: it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `BiomeTagProvider.WAYSTONES_END_STONE_WAYSTONE`: it uses Minecraft's `TagKey` (org/betterx/datagen/betterend/tags/BiomeTagProvider.java:20)
- [ ] `BiomeTagProvider.BiomeTagProvider(1)`: its class isn't translated (it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/BiomeTagProvider.java:24)
- [ ] `BiomeTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/BiomeTagProvider.java:28)

### `BlockTagProvider`: it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `BlockTagProvider.BlockTagProvider(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/BlockTagProvider.java:25)
- [ ] `BlockTagProvider.addEndGround(2)`: it uses `TagBootstrapContext` (de.ambertation.wover.tag.api.event.context) (org/betterx/datagen/betterend/tags/BlockTagProvider.java:29)
- [ ] `BlockTagProvider.addSurvivesOn(1)`: it uses `TagBootstrapContext` (de.ambertation.wover.tag.api.event.context) (org/betterx/datagen/betterend/tags/BlockTagProvider.java:33)
- [ ] `BlockTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/BlockTagProvider.java:81)

### `EnchantmentTagProvider`: it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `EnchantmentTagProvider.EnchantmentTagProvider(1)`: its class isn't translated (it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/EnchantmentTagProvider.java:12)
- [ ] `EnchantmentTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/EnchantmentTagProvider.java:16)

### `ItemTagProvider`: it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `ItemTagProvider.CAPE_SLOT`: it uses Minecraft's `TagKey` (org/betterx/datagen/betterend/tags/ItemTagProvider.java:22)
- [ ] `ItemTagProvider.ItemTagProvider(1)`: its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/ItemTagProvider.java:24)
- [ ] `ItemTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/ItemTagProvider.java:28)

### `NourishItemTagProvider`: it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `NourishItemTagProvider.NourishItemTagProvider(1)`: its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/NourishItemTagProvider.java:13)
- [ ] `NourishItemTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betterend/tags/NourishItemTagProvider.java:17)

### `EndTradeSetProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [ ] `EndTradeSetProvider.EndTradeSetProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/villagers/EndTradeSetProvider.java:12)
- [ ] `EndTradeSetProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/villagers/EndTradeSetProvider.java:16)

### `EndVillagerDefinitions`

- [x] `EndVillagerDefinitions.EndVillagerDefinitions(0)` (org/betterx/datagen/betterend/villagers/EndVillagerDefinitions.java:10)
- [ ] `EndVillagerDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betterend/villagers/EndVillagerDefinitions.java:13)

### `EndVillagerTradeProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [x] `EndVillagerTradeProvider.EMERALD` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:23)
- [ ] `EndVillagerTradeProvider.TRADES`: it calls EndVillagerTradeProvider.end, which isn't translated (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:24)
- [ ] `EndVillagerTradeProvider.EndVillagerTradeProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:55)
- [ ] `EndVillagerTradeProvider.end(1)`: it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:59)
- [x] `EndVillagerTradeProvider.vanilla(1)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:63)
- [x] `EndVillagerTradeProvider.buy(6)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:67)
- [x] `EndVillagerTradeProvider.sell(7)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:71)
- [ ] `EndVillagerTradeProvider.item(1)`: it uses Minecraft's `BuiltInRegistries`.ITEM (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:75)
- [ ] `EndVillagerTradeProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:81)

### `EndVillagerTradeProvider.Trade`

- [x] `EndVillagerTradeProvider.Trade.Trade(10)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [ ] `EndVillagerTradeProvider.Trade.key(0)`: it uses Minecraft's `ResourceKey` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:119)
- [x] `EndVillagerTradeProvider.Trade.level(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.name(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.wants(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.wantsCount(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.alsoWants(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.gives(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.givesCount(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.maxUses(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.xp(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)
- [x] `EndVillagerTradeProvider.Trade.chance(0)` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java:107)

### `EndVillagerTradeTagProvider`: it extends `WoverTagProvider` (de.ambertation.wover.datagen.api)

- [ ] `EndVillagerTradeTagProvider.VILLAGER_TRADES`: it uses `TagRegistry` (de.ambertation.wover.tag.api) (org/betterx/datagen/betterend/villagers/EndVillagerTradeTagProvider.java:15)
- [ ] `EndVillagerTradeTagProvider.EndVillagerTradeTagProvider(1)`: its class isn't translated (it extends `WoverTagProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/villagers/EndVillagerTradeTagProvider.java:17)
- [ ] `EndVillagerTradeTagProvider.level(1)`: it uses Minecraft's `TagKey` (org/betterx/datagen/betterend/villagers/EndVillagerTradeTagProvider.java:21)
- [ ] `EndVillagerTradeTagProvider.prepareTags(1)`: its class isn't translated (it extends `WoverTagProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/villagers/EndVillagerTradeTagProvider.java:25)

### `CarverProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [ ] `CarverProvider.CarverProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/worldgen/CarverProvider.java:16)
- [ ] `CarverProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/worldgen/CarverProvider.java:20)

### `EndBiomeModificationProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [ ] `EndBiomeModificationProvider.EndBiomeModificationProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/worldgen/EndBiomeModificationProvider.java:17)
- [ ] `EndBiomeModificationProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betterend/worldgen/EndBiomeModificationProvider.java:21)

### `EndBiomesProvider`: it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `EndBiomesProvider.BIOMES`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java:48)
- [ ] `EndBiomesProvider.EndBiomesProvider(1)`: its class isn't translated (it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java:50)
- [ ] `EndBiomesProvider.loadAllBiomeConfigs(0)`: it calls EndBiomesProvider.putBiome, which isn't translated (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java:54)
- [ ] `EndBiomesProvider.putBiome(3)`: it uses EndBiomeKey, which isn't translated (it's generic) (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java:84)
- [ ] `EndBiomesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java:89)

### `EndBiomesProvider.BiomeInfo`: it uses Config, which isn't translated (it implements `BiomeFactory`)

- [ ] `EndBiomesProvider.BiomeInfo.BiomeInfo(2)`: its class isn't translated (it uses Config, which isn't translated (it implements `BiomeFactory`)) (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java:101)
- [ ] `EndBiomesProvider.BiomeInfo.config(0)`: its class isn't translated (it uses Config, which isn't translated (it implements `BiomeFactory`)) (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java:101)
- [ ] `EndBiomesProvider.BiomeInfo.tags(0)`: its class isn't translated (it uses Config, which isn't translated (it implements `BiomeFactory`)) (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java:101)

### `BonemealFeatureProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `BonemealFeatureProvider.BonemealFeatureProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/BonemealFeatureProvider.java:21)
- [ ] `BonemealFeatureProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/BonemealFeatureProvider.java:25)
- [ ] `BonemealFeatureProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/BonemealFeatureProvider.java:89)

### `CaveFeatureProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [x] `CaveFeatureProvider.LUSH_PATCH_VERTICAL_RANGE` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:54)
- [ ] `CaveFeatureProvider.CaveFeatureProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:56)
- [ ] `CaveFeatureProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:60)
- [ ] `CaveFeatureProvider.surfaceCoat(1)`: it uses CaveSurfaceCoatFeature, which isn't translated (it implements Minecraft's `Feature`) (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:150)
- [ ] `CaveFeatureProvider.lushPatch(3)`: it uses CaveVegetationPatchFeature, which isn't translated (it extends Minecraft's `VegetationPatchFeature`) (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:162)
- [ ] `CaveFeatureProvider.floorVegetation(1)`: it uses Minecraft's `Holder` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:181)
- [ ] `CaveFeatureProvider.ceilingVegetation(1)`: it uses Minecraft's `Holder` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:192)
- [ ] `CaveFeatureProvider.inlinePlaced(2)`: it uses Minecraft's `Holder` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:203)
- [ ] `CaveFeatureProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:207)
- [ ] `CaveFeatureProvider.surfaceCoat(3)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:231)
- [ ] `CaveFeatureProvider.cluster(4)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:239)
- [ ] `CaveFeatureProvider.patch(4)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:243)
- [ ] `CaveFeatureProvider.floorScatter(4)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:247)
- [ ] `CaveFeatureProvider.ceilScatter(4)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:258)
- [ ] `CaveFeatureProvider.caveHeight(0)`: it uses Minecraft's `HeightRangePlacement` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:262)
- [ ] `CaveFeatureProvider.underCover(1)`: it uses Minecraft's `SurfaceRelativeThresholdFilter` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java:266)

### `LakeFeatureProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `LakeFeatureProvider.LakeFeatureProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/LakeFeatureProvider.java:16)
- [ ] `LakeFeatureProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/LakeFeatureProvider.java:20)
- [ ] `LakeFeatureProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/LakeFeatureProvider.java:24)
- [ ] `LakeFeatureProvider.registerLake(4)`: it's generic (org/betterx/datagen/betterend/worldgen/features/LakeFeatureProvider.java:29)

### `OreFeatureProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `OreFeatureProvider.OreFeatureProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/OreFeatureProvider.java:22)
- [ ] `OreFeatureProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/OreFeatureProvider.java:26)
- [ ] `OreFeatureProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/OreFeatureProvider.java:29)
- [ ] `OreFeatureProvider.registerOre(5)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/OreFeatureProvider.java:38)
- [ ] `OreFeatureProvider.registerLayer(7)`: it's overloaded with the same number of parameters (org/betterx/datagen/betterend/worldgen/features/OreFeatureProvider.java:52)
- [ ] `OreFeatureProvider.registerLayer(7)`: it's overloaded with the same number of parameters (org/betterx/datagen/betterend/worldgen/features/OreFeatureProvider.java:61)

### `TerrainFeatureProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `TerrainFeatureProvider.TerrainFeatureProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/TerrainFeatureProvider.java:33)
- [ ] `TerrainFeatureProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/TerrainFeatureProvider.java:37)
- [ ] `TerrainFeatureProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/TerrainFeatureProvider.java:40)
- [ ] `TerrainFeatureProvider.surfaceSpike(0)`: it uses Minecraft's `SpeleothemFeature` (org/betterx/datagen/betterend/worldgen/features/TerrainFeatureProvider.java:102)
- [ ] `TerrainFeatureProvider.hangingSpike(0)`: it uses Minecraft's `SpeleothemFeature` (org/betterx/datagen/betterend/worldgen/features/TerrainFeatureProvider.java:116)
- [ ] `TerrainFeatureProvider.registerChanced(4)`: it's generic (org/betterx/datagen/betterend/worldgen/features/TerrainFeatureProvider.java:128)
- [ ] `TerrainFeatureProvider.registerCounted(4)`: it's generic (org/betterx/datagen/betterend/worldgen/features/TerrainFeatureProvider.java:132)

### `VegetationFeaturesProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `VegetationFeaturesProvider.VegetationFeaturesProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/VegetationFeaturesProvider.java:55)
- [ ] `VegetationFeaturesProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/VegetationFeaturesProvider.java:59)
- [ ] `VegetationFeaturesProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/features/VegetationFeaturesProvider.java:76)
- [ ] `VegetationFeaturesProvider.registerVegetation(4)`: it's generic (org/betterx/datagen/betterend/worldgen/features/VegetationFeaturesProvider.java:368)
- [ ] `VegetationFeaturesProvider.registerVegetation(3)`: it uses Minecraft's `BootstrapContext` (org/betterx/datagen/betterend/worldgen/features/VegetationFeaturesProvider.java:372)

### `StructureDataProvider`: it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [x] `StructureDataProvider.BUILDING_MIN_FLOOR_Y` (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java:52)
- [x] `StructureDataProvider.CRASHED_SHIP_MIN_FLOOR_Y` (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java:53)
- [ ] `StructureDataProvider.StructureDataProvider(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java:55)
- [ ] `StructureDataProvider.bootstrapSturctures(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java:59)
- [ ] `StructureDataProvider.bootstrapSets(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java:103)
- [ ] `StructureDataProvider.bootstrapPools(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java:125)
- [ ] `StructureDataProvider.bootstrapProcessors(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java:354)
- [ ] `StructureDataProvider.prepareBiomeTags(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java:450)
