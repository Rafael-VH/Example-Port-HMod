# Java translated to Rust

Translated 35 of 156 methods (1 more is done another way in Horizon) and 63 of 100 static fields to Rust (2 of those call the game, through Horizon's helpers); 0 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 94 | `PresetSelectorScreen.PresetSelectorScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) |
| Uses Minecraft classes (needs Horizon's game API) | 11 | `ConfigScreenBuilder.build(1)`: it uses Minecraft's `Screen` |
| Uses other libraries | 6 | `ConfigScreenBuilder.addButtons(1)`: it uses `ApollibRowHelper` (dev.worldgen.apollib.client.gui.element) |
| Shares one object between names (Java references) | 3 | `ConfigState.GlobalTerrain.GlobalTerrain(5)`: it stores a second reference to an object (Rust would need it copied or shared) |
| Uses null | 3 | `ConfigState.Biomes.Biomes(2)`: a NoiseState with no value (null in Java) |
| Other Java this translator doesn't handle yet | 2 | `V1ConfigState.upgrade(0)`: it creates a V2ConfigState with a constructor that isn't translated |
| Done another way in Horizon | 1 | `TectonicModMenuCompat.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Uses lambdas or method references | 1 | `TectonicCommand.getRegion(2)`: Component.withStyle of something other than ChatFormatting or a style lambda |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| dev/worldgen/tectonic/platform/fabric/TectonicFabric.java:16 | `init` | not translated: it calls Tectonic.init, which isn't translated |

## Every class


### `ConfigScreenBuilder`

- [ ] `ConfigScreenBuilder.build(1)`: it uses Minecraft's `Screen` (dev/worldgen/tectonic/client/ConfigScreenBuilder.java:24)
- [ ] `ConfigScreenBuilder.addButtons(1)`: it uses `ApollibRowHelper` (dev.worldgen.apollib.client.gui.element) (dev/worldgen/tectonic/client/ConfigScreenBuilder.java:40)
- [ ] `ConfigScreenBuilder.addNoise(4)`: it uses `ApollibRowHelper` (dev.worldgen.apollib.client.gui.element) (dev/worldgen/tectonic/client/ConfigScreenBuilder.java:124)
- [ ] `ConfigScreenBuilder.slider(9)`: it uses `ApollibSlider` (dev.worldgen.apollib.client.gui.element) (dev/worldgen/tectonic/client/ConfigScreenBuilder.java:132)
- [ ] `ConfigScreenBuilder.alignLeft(1)`: it uses `Builder` (dev.worldgen.apollib.client.gui.element.ApollibRowHelper) (dev/worldgen/tectonic/client/ConfigScreenBuilder.java:148)

### `PresetSelectorScreen`: it extends Minecraft's `Screen`

- [ ] `PresetSelectorScreen.PresetSelectorScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:27)
- [ ] `PresetSelectorScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:32)
- [ ] `PresetSelectorScreen.repositionElements(0)`: its class isn't translated (it extends Minecraft's `Screen`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:42)
- [ ] `PresetSelectorScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:49)
- [ ] `PresetSelectorScreen.text(1)`: it calls `Apollib` (dev.worldgen.apollib).isModLoaded (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:53)

### `PresetSelectorScreen.PresetList`: it extends Minecraft's `ContainerObjectSelectionList`

- [ ] `PresetSelectorScreen.PresetList.PresetList(3)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:66)
- [ ] `PresetSelectorScreen.PresetList.addEntry(3)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:70)
- [ ] `PresetSelectorScreen.PresetList.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:74)
- [ ] `PresetSelectorScreen.PresetList.updateSize(2)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:78)

### `PresetSelectorScreen.PresetList.Entry`: it extends `Entry`

- [ ] `PresetSelectorScreen.PresetList.Entry.Entry(3)`: its class isn't translated (it extends `Entry`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:87)
- [ ] `PresetSelectorScreen.PresetList.Entry.select(1)`: its class isn't translated (it extends `Entry`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:92)
- [ ] `PresetSelectorScreen.PresetList.Entry.extractContent(5)`: its class isn't translated (it extends `Entry`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:98)
- [ ] `PresetSelectorScreen.PresetList.Entry.children(0)`: its class isn't translated (it extends `Entry`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:103)
- [ ] `PresetSelectorScreen.PresetList.Entry.narratables(0)`: its class isn't translated (it extends `Entry`) (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java:107)

### `TectonicCommand`

- [ ] `TectonicCommand.register(1)`: it uses Minecraft's `CommandDispatcher` (dev/worldgen/tectonic/command/TectonicCommand.java:37)
- [ ] `TectonicCommand.addTarget(4)`: it uses Minecraft's `LiteralArgumentBuilder` (dev/worldgen/tectonic/command/TectonicCommand.java:62)
- [ ] `TectonicCommand.debugOutput(1)`: it uses Minecraft's `ServerLevel` (dev/worldgen/tectonic/command/TectonicCommand.java:68)
- [ ] `TectonicCommand.locate(4)`: it uses Minecraft's `InclusiveRange` (dev/worldgen/tectonic/command/TectonicCommand.java:98)
- [x] `TectonicCommand.dist(4)` (dev/worldgen/tectonic/command/TectonicCommand.java:151)
- [ ] `TectonicCommand.getRandomState(1)`: it uses Minecraft's `RandomState` (dev/worldgen/tectonic/command/TectonicCommand.java:157)
- [x] `TectonicCommand.message(2)` in game_side.rs (dev/worldgen/tectonic/command/TectonicCommand.java:171)
- [x] `TectonicCommand.failure(2)` in game_side.rs (dev/worldgen/tectonic/command/TectonicCommand.java:175)
- [ ] `TectonicCommand.get(4)`: it uses Minecraft's `Holder` (dev/worldgen/tectonic/command/TectonicCommand.java:179)
- [ ] `TectonicCommand.getRegion(2)`: Component.withStyle of something other than ChatFormatting or a style lambda (dev/worldgen/tectonic/command/TectonicCommand.java:184)
- [ ] `TectonicCommand.key(1)`: it uses Minecraft's `ResourceKey` (dev/worldgen/tectonic/command/TectonicCommand.java:211)

### `TectonicCommand.LocateTargets`: it's an interface

- [ ] `TectonicCommand.LocateTargets.range(2)`: it uses Minecraft's `InclusiveRange` (dev/worldgen/tectonic/command/TectonicCommand.java:299)

### `TectonicModMenuCompat`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `TectonicModMenuCompat.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (dev/worldgen/tectonic/compat/TectonicModMenuCompat.java:8)

### `ConfigPresets`: it's an interface

- [ ] `ConfigPresets.acceptPresets(1)`: it uses `TriConsumer` (org.apache.logging.log4j.util) (dev/worldgen/tectonic/config/ConfigPresets.java:64)

### `ConfigState`: it implements `ApollibCopyable` (dev.worldgen.apollib.config)

- [ ] `ConfigState.DEFAULT_STATE`: it uses ConfigState, which isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/ConfigState.java:11)
- [x] `ConfigState.MINOR_VERSION` (dev/worldgen/tectonic/config/ConfigState.java:22)
- [ ] `ConfigState.BASE_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:23)
- [ ] `ConfigState.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:37)
- [ ] `ConfigState.create(9)`: it uses ConfigState, which isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/ConfigState.java:47)
- [ ] `ConfigState.ConfigState(9)`: its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/ConfigState.java:61)
- [ ] `ConfigState.enabled(0)`: its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/ConfigState.java:85)
- [ ] `ConfigState.getValue(1)`: its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/ConfigState.java:89)
- [ ] `ConfigState.getNoiseState(1)`: its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/ConfigState.java:114)
- [ ] `ConfigState.test(1)`: its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/ConfigState.java:126)
- [ ] `ConfigState.copy(0)`: its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/ConfigState.java:140)

### `ConfigState.Biomes`

- [ ] `ConfigState.Biomes.DEFAULT`: it creates a Biomes with a constructor that isn't translated (dev/worldgen/tectonic/config/ConfigState.java:182)
- [ ] `ConfigState.Biomes.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:183)
- [ ] `ConfigState.Biomes.Biomes(2)`: a NoiseState with no value (null in Java) (dev/worldgen/tectonic/config/ConfigState.java:193)

### `ConfigState.Caves`

- [x] `ConfigState.Caves.DEPTH_CUTOFF_START` (dev/worldgen/tectonic/config/ConfigState.java:200)
- [x] `ConfigState.Caves.DEPTH_CUTOFF_SIZE` (dev/worldgen/tectonic/config/ConfigState.java:201)
- [x] `ConfigState.Caves.CHEESE_ENABLED` (dev/worldgen/tectonic/config/ConfigState.java:202)
- [x] `ConfigState.Caves.CHEESE_ADDITIVE` (dev/worldgen/tectonic/config/ConfigState.java:203)
- [x] `ConfigState.Caves.NOODLE_ENABLED` (dev/worldgen/tectonic/config/ConfigState.java:204)
- [x] `ConfigState.Caves.NOODLE_ADDITIVE` (dev/worldgen/tectonic/config/ConfigState.java:205)
- [x] `ConfigState.Caves.SPAGHETTI_ENABLED` (dev/worldgen/tectonic/config/ConfigState.java:206)
- [x] `ConfigState.Caves.CARVERS_ENABLED` (dev/worldgen/tectonic/config/ConfigState.java:207)
- [x] `ConfigState.Caves.ORE_FIX` (dev/worldgen/tectonic/config/ConfigState.java:208)
- [x] `ConfigState.Caves.DEFAULT` (dev/worldgen/tectonic/config/ConfigState.java:209)
- [ ] `ConfigState.Caves.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:210)
- [x] `ConfigState.Caves.Caves(9)` (dev/worldgen/tectonic/config/ConfigState.java:260)

### `ConfigState.Continents`

- [x] `ConfigState.Continents.OCEAN_OFFSET` (dev/worldgen/tectonic/config/ConfigState.java:284)
- [x] `ConfigState.Continents.CONTINENTS_SCALE` (dev/worldgen/tectonic/config/ConfigState.java:285)
- [x] `ConfigState.Continents.EROSION_SCALE` (dev/worldgen/tectonic/config/ConfigState.java:286)
- [x] `ConfigState.Continents.RIDGE_SCALE` (dev/worldgen/tectonic/config/ConfigState.java:287)
- [x] `ConfigState.Continents.UNDERGROUND_RIVERS` (dev/worldgen/tectonic/config/ConfigState.java:288)
- [x] `ConfigState.Continents.RIVER_LANTERNS` (dev/worldgen/tectonic/config/ConfigState.java:289)
- [x] `ConfigState.Continents.RIVER_ICE` (dev/worldgen/tectonic/config/ConfigState.java:290)
- [x] `ConfigState.Continents.FLAT_TERRAIN_SKEW` (dev/worldgen/tectonic/config/ConfigState.java:291)
- [x] `ConfigState.Continents.ROLLING_HILLS` (dev/worldgen/tectonic/config/ConfigState.java:292)
- [x] `ConfigState.Continents.JUNGLE_PILLARS` (dev/worldgen/tectonic/config/ConfigState.java:293)
- [x] `ConfigState.Continents.DEFAULT` (dev/worldgen/tectonic/config/ConfigState.java:294)
- [ ] `ConfigState.Continents.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:295)
- [x] `ConfigState.Continents.Continents(10)` (dev/worldgen/tectonic/config/ConfigState.java:361)

### `ConfigState.Experimental`

- [x] `ConfigState.Experimental.ALTERNATE_EROSION_SCALING` (dev/worldgen/tectonic/config/ConfigState.java:387)
- [x] `ConfigState.Experimental.ALTERNATE_CONTINENTS_SCALING` (dev/worldgen/tectonic/config/ConfigState.java:388)
- [x] `ConfigState.Experimental.DEFAULT` (dev/worldgen/tectonic/config/ConfigState.java:389)
- [ ] `ConfigState.Experimental.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:390)
- [x] `ConfigState.Experimental.Experimental(2)` (dev/worldgen/tectonic/config/ConfigState.java:400)

### `ConfigState.General`

- [x] `ConfigState.General.MOD_ENABLED` (dev/worldgen/tectonic/config/ConfigState.java:407)
- [x] `ConfigState.General.SNOW_START_OFFSET` (dev/worldgen/tectonic/config/ConfigState.java:408)
- [x] `ConfigState.General.DEFAULT` (dev/worldgen/tectonic/config/ConfigState.java:409)
- [ ] `ConfigState.General.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:410)
- [x] `ConfigState.General.General(2)` (dev/worldgen/tectonic/config/ConfigState.java:423)

### `ConfigState.GlobalTerrain`

- [x] `ConfigState.GlobalTerrain.VERTICAL_SCALE` (dev/worldgen/tectonic/config/ConfigState.java:430)
- [x] `ConfigState.GlobalTerrain.ELEVATION_BOOST` (dev/worldgen/tectonic/config/ConfigState.java:431)
- [ ] `ConfigState.GlobalTerrain.HEIGHT_LIMITS`: it stores a second reference to an object (Rust would need it copied or shared) (dev/worldgen/tectonic/config/ConfigState.java:432)
- [x] `ConfigState.GlobalTerrain.LAVA_TUNNELS` (dev/worldgen/tectonic/config/ConfigState.java:433)
- [x] `ConfigState.GlobalTerrain.ULTRASMOOTH` (dev/worldgen/tectonic/config/ConfigState.java:434)
- [ ] `ConfigState.GlobalTerrain.DEFAULT`: it creates a GlobalTerrain with a constructor that isn't translated (dev/worldgen/tectonic/config/ConfigState.java:435)
- [ ] `ConfigState.GlobalTerrain.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:436)
- [ ] `ConfigState.GlobalTerrain.GlobalTerrain(5)`: it stores a second reference to an object (Rust would need it copied or shared) (dev/worldgen/tectonic/config/ConfigState.java:472)

### `ConfigState.Islands`

- [x] `ConfigState.Islands.ENABLED` (dev/worldgen/tectonic/config/ConfigState.java:482)
- [x] `ConfigState.Islands.NOISE` (dev/worldgen/tectonic/config/ConfigState.java:483)
- [ ] `ConfigState.Islands.DEFAULT`: it creates a Islands with a constructor that isn't translated (dev/worldgen/tectonic/config/ConfigState.java:484)
- [ ] `ConfigState.Islands.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:485)
- [ ] `ConfigState.Islands.Islands(2)`: a NoiseState with no value (null in Java) (dev/worldgen/tectonic/config/ConfigState.java:495)

### `ConfigState.Oceans`

- [x] `ConfigState.Oceans.OCEAN_DEPTH` (dev/worldgen/tectonic/config/ConfigState.java:502)
- [x] `ConfigState.Oceans.DEEP_OCEAN_DEPTH` (dev/worldgen/tectonic/config/ConfigState.java:503)
- [x] `ConfigState.Oceans.MONUMENT_OFFSET` (dev/worldgen/tectonic/config/ConfigState.java:504)
- [x] `ConfigState.Oceans.REMOVE_FROZEN_OCEAN_ICE` (dev/worldgen/tectonic/config/ConfigState.java:505)
- [x] `ConfigState.Oceans.DEFAULT` (dev/worldgen/tectonic/config/ConfigState.java:506)
- [ ] `ConfigState.Oceans.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/ConfigState.java:507)
- [x] `ConfigState.Oceans.Oceans(4)` (dev/worldgen/tectonic/config/ConfigState.java:531)

### `NewConfigState`

- [x] `NewConfigState.VERSION` (dev/worldgen/tectonic/config/NewConfigState.java:9)

### `NewConfigState.Features`

- [x] `NewConfigState.Features.UNDERGROUND_RIVERS` (dev/worldgen/tectonic/config/NewConfigState.java:13)
- [x] `NewConfigState.Features.RIVER_ICE` (dev/worldgen/tectonic/config/NewConfigState.java:14)
- [x] `NewConfigState.Features.LAVA_TUNNELS` (dev/worldgen/tectonic/config/NewConfigState.java:15)
- [x] `NewConfigState.Features.JUNGLE_PILLARS` (dev/worldgen/tectonic/config/NewConfigState.java:16)
- [ ] `NewConfigState.Features.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/NewConfigState.java:17)
- [x] `NewConfigState.Features.Features(4)` (dev/worldgen/tectonic/config/NewConfigState.java:41)

### `NewConfigState.Noises`

- [x] `NewConfigState.Noises.TEMPERATURE` (dev/worldgen/tectonic/config/NewConfigState.java:50)
- [x] `NewConfigState.Noises.VEGETATION` (dev/worldgen/tectonic/config/NewConfigState.java:51)
- [x] `NewConfigState.Noises.CONTINENTS` (dev/worldgen/tectonic/config/NewConfigState.java:52)
- [x] `NewConfigState.Noises.EROSION` (dev/worldgen/tectonic/config/NewConfigState.java:53)
- [x] `NewConfigState.Noises.RIDGES` (dev/worldgen/tectonic/config/NewConfigState.java:54)
- [x] `NewConfigState.Noises.ISLANDS` (dev/worldgen/tectonic/config/NewConfigState.java:55)
- [ ] `NewConfigState.Noises.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/NewConfigState.java:56)
- [ ] `NewConfigState.Noises.Noises(6)`: a NoiseState with no value (null in Java) (dev/worldgen/tectonic/config/NewConfigState.java:74)

### `NewConfigState.TerrainScales`

- [x] `NewConfigState.TerrainScales.VERTICAL_SCALE` (dev/worldgen/tectonic/config/NewConfigState.java:85)
- [x] `NewConfigState.TerrainScales.OCEAN_DEPTH` (dev/worldgen/tectonic/config/NewConfigState.java:86)
- [x] `NewConfigState.TerrainScales.DEEP_OCEAN_DEPTH` (dev/worldgen/tectonic/config/NewConfigState.java:87)

### `HeightLimits`

- [x] `HeightLimits.DEFAULT` (dev/worldgen/tectonic/config/object/HeightLimits.java:9)
- [x] `HeightLimits.INCREASED_HEIGHT` (dev/worldgen/tectonic/config/object/HeightLimits.java:10)
- [ ] `HeightLimits.OLD_CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/config/object/HeightLimits.java:11)
- [ ] `HeightLimits.NEW_CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/config/object/HeightLimits.java:14)
- [ ] `HeightLimits.FULL_CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/config/object/HeightLimits.java:18)
- [x] `HeightLimits.HeightLimits(2)` (dev/worldgen/tectonic/config/object/HeightLimits.java:23)
- [x] `HeightLimits.copy(0)` (dev/worldgen/tectonic/config/object/HeightLimits.java:44)
- [x] `HeightLimits.isVanilla(0)` (dev/worldgen/tectonic/config/object/HeightLimits.java:48)
- [ ] `HeightLimits.defaultLimits(1)`: it stores a second reference to an object (Rust would need it copied or shared) (dev/worldgen/tectonic/config/object/HeightLimits.java:52)
- [x] `HeightLimits.getHeight(0)` (dev/worldgen/tectonic/config/object/HeightLimits.java:56)

### `NoiseState`

- [x] `NoiseState.NoiseState(3)` (dev/worldgen/tectonic/config/object/NoiseState.java:13)
- [x] `NoiseState.NoiseState(4)` (dev/worldgen/tectonic/config/object/NoiseState.java:20)
- [x] `NoiseState.create(0)` (dev/worldgen/tectonic/config/object/NoiseState.java:27)
- [x] `NoiseState.create(1)` (dev/worldgen/tectonic/config/object/NoiseState.java:31)
- [x] `NoiseState.copy(0)` (dev/worldgen/tectonic/config/object/NoiseState.java:35)
- [ ] `NoiseState.legacyCodec(1)`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/config/object/NoiseState.java:39)
- [ ] `NoiseState.codec(2)`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/config/object/NoiseState.java:50)

### `V1ConfigState`

- [ ] `V1ConfigState.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/V1ConfigState.java:7)
- [x] `V1ConfigState.V1ConfigState(3)` (dev/worldgen/tectonic/config/V1ConfigState.java:6)
- [ ] `V1ConfigState.upgrade(0)`: it creates a V2ConfigState with a constructor that isn't translated (dev/worldgen/tectonic/config/V1ConfigState.java:16)
- [x] `V1ConfigState.modEnabled(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:6)
- [x] `V1ConfigState.features(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:6)
- [x] `V1ConfigState.experimental(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:6)

### `V1ConfigState.Experimental`

- [ ] `V1ConfigState.Experimental.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/V1ConfigState.java:33)
- [x] `V1ConfigState.Experimental.DEFAULT` (dev/worldgen/tectonic/config/V1ConfigState.java:41)
- [x] `V1ConfigState.Experimental.Experimental(3)` (dev/worldgen/tectonic/config/V1ConfigState.java:32)
- [x] `V1ConfigState.Experimental.horizontalMountainScale(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:32)
- [x] `V1ConfigState.Experimental.increasedHeight(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:32)
- [x] `V1ConfigState.Experimental.terrainScale(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:32)

### `V1ConfigState.Features`

- [ ] `V1ConfigState.Features.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/V1ConfigState.java:45)
- [x] `V1ConfigState.Features.DEFAULT` (dev/worldgen/tectonic/config/V1ConfigState.java:55)
- [x] `V1ConfigState.Features.Features(5)` (dev/worldgen/tectonic/config/V1ConfigState.java:44)
- [x] `V1ConfigState.Features.deeperOceans(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:44)
- [x] `V1ConfigState.Features.desertDunes(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:44)
- [x] `V1ConfigState.Features.lavaRivers(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:44)
- [x] `V1ConfigState.Features.snowStartOffset(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:44)
- [x] `V1ConfigState.Features.undergroundRivers(0)` (dev/worldgen/tectonic/config/V1ConfigState.java:44)

### `V2ConfigState`

- [ ] `V2ConfigState.BASE_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/V2ConfigState.java:9)
- [ ] `V2ConfigState.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/V2ConfigState.java:18)
- [ ] `V2ConfigState.V2ConfigState(4)`: it stores a second reference to an object (Rust would need it copied or shared) (dev/worldgen/tectonic/config/V2ConfigState.java:24)
- [ ] `V2ConfigState.upgrade(0)`: it uses ConfigState, which isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/config/V2ConfigState.java:31)

### `V2ConfigState.Scales`

- [ ] `V2ConfigState.Scales.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/V2ConfigState.java:48)
- [x] `V2ConfigState.Scales.DEFAULT` (dev/worldgen/tectonic/config/V2ConfigState.java:57)
- [x] `V2ConfigState.Scales.Scales(4)` (dev/worldgen/tectonic/config/V2ConfigState.java:63)

### `V2ConfigState.Toggles`

- [ ] `V2ConfigState.Toggles.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/tectonic/config/V2ConfigState.java:72)
- [x] `V2ConfigState.Toggles.DEFAULT` (dev/worldgen/tectonic/config/V2ConfigState.java:82)
- [x] `V2ConfigState.Toggles.Toggles(5)` (dev/worldgen/tectonic/config/V2ConfigState.java:89)

### `ConfigLoadPredicate`: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)

- [ ] `ConfigLoadPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/lithostitched/ConfigLoadPredicate.java:10)
- [ ] `ConfigLoadPredicate.ConfigLoadPredicate(1)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/tectonic/lithostitched/ConfigLoadPredicate.java:9)
- [ ] `ConfigLoadPredicate.test(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/tectonic/lithostitched/ConfigLoadPredicate.java:12)
- [ ] `ConfigLoadPredicate.codec(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/tectonic/lithostitched/ConfigLoadPredicate.java:16)
- [ ] `ConfigLoadPredicate.key(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/tectonic/lithostitched/ConfigLoadPredicate.java:9)

### `SetHeightLimitsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetHeightLimitsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:23)
- [ ] `SetHeightLimitsModifier.SetHeightLimitsModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:20)
- [ ] `SetHeightLimitsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:33)
- [ ] `SetHeightLimitsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:49)
- [ ] `SetHeightLimitsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:20)
- [ ] `SetHeightLimitsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:20)
- [ ] `SetHeightLimitsModifier.dimensionType(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:20)
- [ ] `SetHeightLimitsModifier.noiseSettings(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:20)
- [ ] `BiomeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:12)
- [ ] `BlendingDataMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:14)
- [ ] `ChunkAccessMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:12)
- [ ] `ChunkDataMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:20)
- [ ] `WorldCreationUiStateMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:21)
- [ ] `DimensionTypeAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:9)
- [ ] `HeightmapMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:12)
- [ ] `IOWorkerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:12)
- [ ] `NoiseBasedChunkGeneratorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:25)
- [ ] `NoiseSettingsAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:9)
- [ ] `NoisesMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:10)
- [ ] `StructurePieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:13)
- [ ] `TemperatureModifierMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java:11)

### `ConfigResourceCondition`: it implements Fabric's `ResourceCondition`

- [ ] `ConfigResourceCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/platform/fabric/ConfigResourceCondition.java:13)
- [ ] `ConfigResourceCondition.TYPE`: it uses Fabric's `ResourceConditionType` (dev/worldgen/tectonic/platform/fabric/ConfigResourceCondition.java:16)
- [ ] `ConfigResourceCondition.ConfigResourceCondition(1)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (dev/worldgen/tectonic/platform/fabric/ConfigResourceCondition.java:12)
- [ ] `ConfigResourceCondition.getType(0)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (dev/worldgen/tectonic/platform/fabric/ConfigResourceCondition.java:18)
- [ ] `ConfigResourceCondition.test(1)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (dev/worldgen/tectonic/platform/fabric/ConfigResourceCondition.java:22)
- [ ] `ConfigResourceCondition.key(0)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (dev/worldgen/tectonic/platform/fabric/ConfigResourceCondition.java:12)

### `TectonicFabric`: it implements Fabric's `ModInitializer`

- [ ] `TectonicFabric.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (dev/worldgen/tectonic/platform/fabric/TectonicFabric.java:16)

### `TectonicRegistrations`

- [ ] `TectonicRegistrations.init(0)`: it calls TectonicRegistrations.register, which isn't translated (dev/worldgen/tectonic/registry/TectonicRegistrations.java:15)
- [ ] `TectonicRegistrations.register(3)`: it's generic (dev/worldgen/tectonic/registry/TectonicRegistrations.java:25)

### `Tectonic`

- [x] `Tectonic.MOD_ID` (dev/worldgen/tectonic/Tectonic.java:13)
- [x] `Tectonic.LOGGER` (dev/worldgen/tectonic/Tectonic.java:14)
- [x] `Tectonic.BLENDING_VERSION` (dev/worldgen/tectonic/Tectonic.java:15)
- [x] `Tectonic.BLENDING_KEY` (dev/worldgen/tectonic/Tectonic.java:16)
- [ ] `Tectonic.CONFIG`: it uses `ApollibConfigHolder` (dev.worldgen.apollib.config) (dev/worldgen/tectonic/Tectonic.java:17)
- [ ] `Tectonic.REGISTRAR`: it uses `ApollibRegistrar` (dev.worldgen.apollib.registry) (dev/worldgen/tectonic/Tectonic.java:20)
- [ ] `Tectonic.init(0)`: it uses Tectonic.CONFIG, which isn't translated (it uses `ApollibConfigHolder` (dev.worldgen.apollib.config)) (dev/worldgen/tectonic/Tectonic.java:22)
- [x] `Tectonic.id(1)` (dev/worldgen/tectonic/Tectonic.java:27)

### `TectonicTags`: it's an interface


### `ConfigClamp`: it implements Minecraft's `DensityFunction`

- [ ] `ConfigClamp.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:14)
- [ ] `ConfigClamp.ConfigClamp(3)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:13)
- [ ] `ConfigClamp.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:23)
- [ ] `ConfigClamp.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:27)
- [ ] `ConfigClamp.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:31)
- [ ] `ConfigClamp.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:35)
- [ ] `ConfigClamp.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:39)
- [ ] `ConfigClamp.input(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:13)
- [ ] `ConfigClamp.min(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:13)
- [ ] `ConfigClamp.max(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java:13)

### `ConfigConstant`: it implements Minecraft's `DensityFunction`

- [ ] `ConfigConstant.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:18)
- [ ] `ConfigConstant.ConfigConstant(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:17)
- [ ] `ConfigConstant.create(1)`: it uses ConfigConstant, which isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:22)
- [ ] `ConfigConstant.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:26)
- [ ] `ConfigConstant.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:30)
- [ ] `ConfigConstant.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:34)
- [ ] `ConfigConstant.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:38)
- [ ] `ConfigConstant.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:42)
- [ ] `ConfigConstant.value(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java:17)

### `ConfigNoise`: it implements Minecraft's `DensityFunction`

- [ ] `ConfigNoise.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:26)
- [ ] `ConfigNoise.ConfigNoise(7)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:23)
- [ ] `ConfigNoise.create(4)`: it uses ConfigNoise, which isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:36)
- [ ] `ConfigNoise.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:41)
- [ ] `ConfigNoise.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:50)
- [ ] `ConfigNoise.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:58)
- [ ] `ConfigNoise.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:62)
- [ ] `ConfigNoise.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:66)
- [ ] `ConfigNoise.noise(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:23)
- [ ] `ConfigNoise.shiftX(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:23)
- [ ] `ConfigNoise.shiftZ(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:23)
- [ ] `ConfigNoise.scale(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:23)
- [ ] `ConfigNoise.multiplier(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:23)
- [ ] `ConfigNoise.offset(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:23)
- [ ] `ConfigNoise.smootherScaling(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:23)

### `ConfigNoise.NormalSampler`: it implements Minecraft's `DensitySampler`

- [ ] `ConfigNoise.NormalSampler.NormalSampler(6)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:70)
- [ ] `ConfigNoise.NormalSampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:72)
- [ ] `ConfigNoise.NormalSampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:111)
- [ ] `ConfigNoise.NormalSampler.noise(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:70)
- [ ] `ConfigNoise.NormalSampler.shiftX(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:70)
- [ ] `ConfigNoise.NormalSampler.shiftZ(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:70)
- [ ] `ConfigNoise.NormalSampler.scale(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:70)
- [ ] `ConfigNoise.NormalSampler.multiplier(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:70)
- [ ] `ConfigNoise.NormalSampler.offset(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:70)

### `ConfigNoise.SmoothSampler`: it implements Minecraft's `DensitySampler`

- [ ] `ConfigNoise.SmoothSampler.SmoothSampler(6)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:118)
- [ ] `ConfigNoise.SmoothSampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:120)
- [ ] `ConfigNoise.SmoothSampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:159)
- [ ] `ConfigNoise.SmoothSampler.noise(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:118)
- [ ] `ConfigNoise.SmoothSampler.shiftX(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:118)
- [ ] `ConfigNoise.SmoothSampler.shiftZ(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:118)
- [ ] `ConfigNoise.SmoothSampler.scale(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:118)
- [ ] `ConfigNoise.SmoothSampler.multiplier(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:118)
- [ ] `ConfigNoise.SmoothSampler.offset(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java:118)

### `HeightStabilizedCount`: it implements Minecraft's `PlacementModifier`

- [ ] `HeightStabilizedCount.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/tectonic/worldgen/placementmodifier/HeightStabilizedCount.java:17)
- [ ] `HeightStabilizedCount.HeightStabilizedCount(3)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/tectonic/worldgen/placementmodifier/HeightStabilizedCount.java:16)
- [ ] `HeightStabilizedCount.modify(4)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/tectonic/worldgen/placementmodifier/HeightStabilizedCount.java:26)
- [ ] `HeightStabilizedCount.codec(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/tectonic/worldgen/placementmodifier/HeightStabilizedCount.java:41)
- [ ] `HeightStabilizedCount.maxY(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/tectonic/worldgen/placementmodifier/HeightStabilizedCount.java:16)
- [ ] `HeightStabilizedCount.countPerSection(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/tectonic/worldgen/placementmodifier/HeightStabilizedCount.java:16)
- [ ] `HeightStabilizedCount.biasedToBottom(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/tectonic/worldgen/placementmodifier/HeightStabilizedCount.java:16)
