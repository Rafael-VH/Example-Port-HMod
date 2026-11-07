# Horizon port report: Chunky

`chunky` 1.5.66 · Chunky-Fabric-1.5.66.jar · 171 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 0 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 14 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 1 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: org.popcraft.chunky.ChunkyFabric

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| CommandRegistrationCallback.EVENT | `ChunkyFabric` | hook `command` | needs-translation | Server commands. Horizon's command hook is client-side. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Accessor on ChunkHolder.pendingFullStateConfirmation | `ChunkHolderAccess.chunky$getPendingFullStateConfirmation` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ChunkHolder.pendingFullStateConfirmation: run `match`. |
| @ModifyArg(INVOKE) on ChunkHolder.updateFutures at Lnet/minecraft/server/level/ChunkHolder$LevelChangeListener;onLevelChange(Lnet/minecraft/world/level/ChunkPos;Ljava/util/function/IntSupplier;ILjava/util/function/IntConsumer;)V | `ChunkHolderMixin.boostUnloadedPriority` | MODIFY_ARG | needs-mapping | No Horizon function known for ChunkHolder.updateFutures: run `match`. The call it hooks (LevelChangeListener.onLevelChange) isn't mapped to a Rust function yet. |
| @Invoker on ChunkMap.getVisibleChunkIfPresent | `ChunkMapMixin.invokeGetVisibleChunkIfPresent` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ChunkMap.getVisibleChunkIfPresent: run `match`. |
| @Invoker on ChunkMap.readChunk | `ChunkMapMixin.invokeReadChunk` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ChunkMap.readChunk: run `match`. |
| @Invoker on ChunkMap.invokeTick | `ChunkMapMixin.invokeTick` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ChunkMap.invokeTick: run `match`. |
| @Accessor on ChunkMap.mainThreadExecutor | `ChunkMapMixin.getMainThreadExecutor` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ChunkMap.mainThreadExecutor: run `match`. |
| @WrapWithCondition(INVOKE) on ChunkMap.onLevelChange at Lnet/minecraft/server/level/ChunkTaskDispatcher;onLevelChange(Lnet/minecraft/world/level/ChunkPos;Ljava/util/function/IntSupplier;ILjava/util/function/IntConsumer;)V | `ChunkMapPatchMixin.suppressRaceCondition` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for ChunkMap.onLevelChange: run `match`. The call it hooks (ChunkTaskDispatcher.onLevelChange) isn't mapped to a Rust function yet. |
| @Accessor on MinecraftServer.emptyTicks | `MinecraftServerAccess.setEmptyTicks` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for MinecraftServer.emptyTicks: run `match`. |
| @Inject(INVOKE) on MinecraftServer.tickServer at Lnet/minecraft/server/MinecraftServer;tickConnection()V | `MinecraftServerMixin.tickPaused` | BEFORE_CALL on horizon_client::game::levels::Game<'_>::tick_server | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (MinecraftServer.tickConnection) isn't mapped to a Rust function yet. |
| @Invoker on ServerChunkCache.getChunkFutureMainThread | `ServerChunkCacheMixin.invokeGetChunkFutureMainThread` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ServerChunkCache.getChunkFutureMainThread: run `match`. |
| @Invoker on ServerChunkCache.invokeRunDistanceManagerUpdates | `ServerChunkCacheMixin.invokeRunDistanceManagerUpdates` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ServerChunkCache.invokeRunDistanceManagerUpdates: run `match`. |
| @Invoker on ServerChunkCache.invokeBroadcastChangedChunks | `ServerChunkCacheMixin.invokeBroadcastChangedChunks` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ServerChunkCache.invokeBroadcastChangedChunks: run `match`. |
| @Accessor on ServerLevel.entityManager | `ServerLevelMixin.getEntityManager` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ServerLevel.entityManager: run `match`. |
| @Inject(INVOKE) on IntegratedServer.tickServer at Lnet/minecraft/client/server/IntegratedServer;tickPaused()V | `IntegratedServerMixin.tickPaused` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for IntegratedServer.tickServer: run `match`. The call it hooks (IntegratedServer.tickPaused) isn't mapped to a Rust function yet. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 1 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

3 of 154 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/server/MinecraftServer#tickServer` | horizon_client::game::levels::Game<'_>::tick_server (suggested) |
| `net/minecraft/server/level/ServerLevel#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/server/level/ServerLevel#getMinY` | horizon_client::game::Dim::min_y (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `net/minecraft/client/server/IntegratedServer#tickPaused` | method | 1 |
| `net/minecraft/client/server/IntegratedServer#tickServer` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#checkPermission` | method | 2 |
| `net/minecraft/commands/CommandSourceStack#getEntity` | method | 2 |
| `net/minecraft/commands/CommandSourceStack#getLevel` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPlayer` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPosition` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getRotation` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getServer` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getTextName` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#permissions` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#sendSuccess` | method | 1 |
| `net/minecraft/commands/arguments/DimensionArgument#dimension` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 1 |
| `net/minecraft/core/Holder$Reference#value` | method | 1 |
| `net/minecraft/core/Registry#getOptional` | method | 1 |
| `net/minecraft/core/RegistryAccess$Frozen#get` | method | 1 |
| `net/minecraft/core/registries/Registries#DIMENSION` | field | 1 |
| `net/minecraft/core/registries/Registries#SOUND_EVENT` | field | 1 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getString` | method | 1 |
| `net/minecraft/nbt/StringTag#TYPE` | field | 1 |
| `net/minecraft/nbt/visitors/CollectFields#<init>` | method | 1 |
| `net/minecraft/nbt/visitors/CollectFields#getResult` | method | 1 |
| `net/minecraft/nbt/visitors/FieldSelector#<init>` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/network/chat/Component#nullToEmpty` | method | 3 |
| `net/minecraft/resources/Identifier#equals` | method | 1 |
| `net/minecraft/resources/Identifier#getPath` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 2 |
| `net/minecraft/resources/Identifier#tryBuild` | method | 1 |
| `net/minecraft/resources/Identifier#tryParse` | method | 4 |
| `net/minecraft/resources/ResourceKey#create` | method | 1 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 3 |
| `net/minecraft/server/MinecraftServer#createCommandSourceStack` | method | 1 |
| `net/minecraft/server/MinecraftServer#emptyTicks` | method | 1 |
| `net/minecraft/server/MinecraftServer#getAbsoluteMaxWorldSize` | method | 1 |
| `net/minecraft/server/MinecraftServer#getAllLevels` | method | 1 |
| `net/minecraft/server/MinecraftServer#getLevel` | method | 1 |
| `net/minecraft/server/MinecraftServer#getPlayerList` | method | 2 |
| `net/minecraft/server/MinecraftServer#getRunningThread` | method | 1 |
| `net/minecraft/server/MinecraftServer#getWorldPath` | method | 2 |
| `net/minecraft/server/MinecraftServer#isDedicatedServer` | method | 1 |
| `net/minecraft/server/MinecraftServer#isSingleplayer` | method | 2 |
| `net/minecraft/server/MinecraftServer#isSingleplayerOwner` | method | 2 |
| `net/minecraft/server/MinecraftServer#registryAccess` | method | 1 |
| `net/minecraft/server/MinecraftServer#tickConnection` | method | 1 |
| `net/minecraft/server/ServerInterface#getProperties` | method | 1 |
| `net/minecraft/server/dedicated/DedicatedServerProperties#maxWorldSize` | field | 1 |
| `net/minecraft/server/level/ChunkHolder#getLatestStatus` | method | 1 |
| `net/minecraft/server/level/ChunkHolder#pendingFullStateConfirmation` | method | 1 |
| `net/minecraft/server/level/ChunkHolder#updateFutures` | method | 1 |
| `net/minecraft/server/level/ChunkHolder$LevelChangeListener#onLevelChange` | method | 1 |
| `net/minecraft/server/level/ChunkLevel#MAX_LEVEL` | field | 1 |
| `net/minecraft/server/level/ChunkMap#getUpdatingChunkIfPresent` | method | 1 |
| `net/minecraft/server/level/ChunkMap#getVisibleChunkIfPresent` | method | 1 |
| `net/minecraft/server/level/ChunkMap#invokeTick` | method | 1 |
| `net/minecraft/server/level/ChunkMap#mainThreadExecutor` | method | 1 |
| `net/minecraft/server/level/ChunkMap#onLevelChange` | method | 1 |
| `net/minecraft/server/level/ChunkMap#readChunk` | method | 1 |
| `net/minecraft/server/level/ChunkTaskDispatcher#onLevelChange` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#<init>` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#addPlayer` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#isVisible` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#removeAllPlayers` | method | 2 |
| `net/minecraft/server/level/ServerBossEvent#removePlayer` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setCreateWorldFog` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setDarkenScreen` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setName` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setPlayBossMusic` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setProgress` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setVisible` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#addTicketWithRadius` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#chunkMap` | field | 2 |
| `net/minecraft/server/level/ServerChunkCache#chunkScanner` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#getChunkFutureMainThread` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#invokeBroadcastChangedChunks` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#invokeRunDistanceManagerUpdates` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#removeTicketWithRadius` | method | 1 |
| `net/minecraft/server/level/ServerLevel#dimension` | method | 2 |
| `net/minecraft/server/level/ServerLevel#entityManager` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockState` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunk` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getLogicalHeight` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getRespawnData` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getSeaLevel` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getWorldBorder` | method | 1 |
| `net/minecraft/server/level/ServerLevel#levelEvent` | method | 1 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#createCommandSourceStack` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getName` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getRootVehicle` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getUUID` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getX` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getXRot` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getY` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getYRot` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getZ` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#nameAndId` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#sendOverlayMessage` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#sendSystemMessage` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#teleportTo` | method | 1 |
| `net/minecraft/server/level/TicketType#<init>` | method | 1 |
| `net/minecraft/server/permissions/PermissionLevel#GAMEMASTERS` | field | 1 |
| `net/minecraft/server/permissions/PermissionSet#hasPermission` | method | 1 |
| `net/minecraft/server/permissions/Permissions#COMMANDS_GAMEMASTER` | field | 1 |
| `net/minecraft/server/players/PlayerList#getPlayerByName` | method | 1 |
| `net/minecraft/server/players/PlayerList#getPlayers` | method | 2 |
| `net/minecraft/sounds/SoundSource#MASTER` | field | 1 |
| `net/minecraft/util/profiling/InactiveProfiler#INSTANCE` | field | 1 |
| `net/minecraft/world/BossEvent$BossBarColor#BLUE` | field | 1 |
| `net/minecraft/world/BossEvent$BossBarColor#GREEN` | field | 1 |
| `net/minecraft/world/BossEvent$BossBarColor#PURPLE` | field | 1 |
| `net/minecraft/world/BossEvent$BossBarColor#RED` | field | 1 |
| `net/minecraft/world/BossEvent$BossBarOverlay#PROGRESS` | field | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 1 |
| `net/minecraft/world/entity/Entity#teleportTo` | method | 1 |
| `net/minecraft/world/level/ChunkPos#<init>` | method | 1 |
| `net/minecraft/world/level/ChunkPos#pack` | method | 1 |
| `net/minecraft/world/level/Level#END` | field | 1 |
| `net/minecraft/world/level/Level#NETHER` | field | 1 |
| `net/minecraft/world/level/Level#OVERWORLD` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isSolid` | method | 1 |
| `net/minecraft/world/level/border/WorldBorder#getCenterX` | method | 1 |
| `net/minecraft/world/level/border/WorldBorder#getCenterZ` | method | 1 |
| `net/minecraft/world/level/border/WorldBorder#getSize` | method | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#FULL` | field | 1 |
| `net/minecraft/world/level/chunk/storage/ChunkScanAccess#scanChunk` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#getStorageFolder` | method | 1 |
| `net/minecraft/world/level/entity/PersistentEntitySectionManager#tick` | method | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING` | field | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#pitch` | method | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#pos` | method | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#yaw` | method | 1 |
| `net/minecraft/world/level/storage/LevelResource#ROOT` | field | 2 |
| `net/minecraft/world/phys/Vec2#x` | field | 1 |
| `net/minecraft/world/phys/Vec2#y` | field | 1 |
| `net/minecraft/world/phys/Vec3#x` | method | 1 |
| `net/minecraft/world/phys/Vec3#y` | method | 1 |
| `net/minecraft/world/phys/Vec3#z` | method | 1 |

</details>

