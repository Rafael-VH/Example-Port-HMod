# Horizon port report: Terralith

`terralith` 2.6.5 · Terralith_26.3_v2.6.5+26.3.jar · 11 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 0 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 0 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 3 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: net.stardustlabs.terralith.TerralithFabric
- `modmenu`: net.stardustlabs.terralith.compat.TerralithModMenu

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends Entry | `TerralithConfigList$Entry` | needs-translation | No mapping for this superclass yet. |
| extends ContainerObjectSelectionList | `TerralithConfigList` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `TerralithConfigScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 24 asset files, 2155 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

2 of 43 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `net/minecraft/ChatFormatting#DARK_RED` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 1 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setHeight` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setWidth` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setX` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setY` | method | 2 |
| `net/minecraft/client/gui/components/Button#builder` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#<init>` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#updateSize` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList$Entry#<init>` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton#onOffBuilder` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton$Builder#create` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton$Builder#withTooltip` | method | 1 |
| `net/minecraft/client/gui/components/MultiLineTextWidget#<init>` | method | 1 |
| `net/minecraft/client/gui/components/MultiLineTextWidget#setMaxWidth` | method | 1 |
| `net/minecraft/client/gui/components/StringWidget#<init>` | method | 1 |
| `net/minecraft/client/gui/components/Tooltip#create` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#<init>` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#addTitleHeader` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#addToContents` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#addToFooter` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#arrangeElements` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#getContentHeight` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#getHeaderHeight` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#visitWidgets` | method | 1 |
| `net/minecraft/client/gui/layouts/LinearLayout#addChild` | method | 1 |
| `net/minecraft/client/gui/layouts/LinearLayout#horizontal` | method | 1 |
| `net/minecraft/client/gui/layouts/LinearLayout#spacing` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#onClose` | method | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_DONE` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#NEW_LINE` | field | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 1 |
| `net/minecraft/network/chat/Component#empty` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 1 |
| `net/minecraft/network/chat/Component#translatable` | method | 2 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 1 |
| `net/minecraft/util/GsonHelper#writeValue` | method | 1 |

</details>

