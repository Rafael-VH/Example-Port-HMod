// Farmer's Delight, ported to Horizon by horizon-port from FarmersDelight-26.3-3.6.30+refabricated.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_farmersdelight). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "farmersdelight";
pub const NAME: &str = "Farmer's Delight";
pub const VERSION: &str = "26.3-3.6.30+refabricated";
pub const DESCRIPTION: &str = "A cozy expansion to farming and cooking!";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[];

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `ClientSetupEvents` (vectorwing/farmersdelight/client/event/ClientSetupEvents.java)
    pub mod client_setup_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientSetupEvents {
        }
        impl ClientSetupEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register_recipe_book_categories() {
        }
        // ClientSetupEvents.registerCustomTooltipRenderers(1): not translated, it uses Minecraft's `ClientTooltipComponent`.
        pub fn register_gui_layers() {
        }
        // ClientSetupEvents.onRegisterRenderers(0): not translated, it uses Minecraft's `EntityType`.
        // ClientSetupEvents.registerMenuScreens(0): not translated, it uses Minecraft's `MenuType`.
        // ClientSetupEvents.registerParticles(0): not translated, it calls Fabric's `ParticleProviderRegistry`.getInstance.
    }

    /// `TooltipEvents` (vectorwing/farmersdelight/client/event/TooltipEvents.java)
    pub mod tooltip_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TooltipEvents {
        }
        impl TooltipEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TooltipEvents.addTooltipToVanillaSoups(4): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `FarmersDelightClient` (vectorwing/farmersdelight/client/FarmersDelightClient.java)
    pub mod farmers_delight_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        pub static NATURAL_REGENERATION_ENABLED: JCell<bool> = JCell::new(false);
        // FarmersDelightClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `CookingPotRecipeBookComponent` (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java)
    pub mod cooking_pot_recipe_book_component {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RecipeBookComponent`. Its static parts are below.
        // COOKING_SEARCH_CATEGORY: not translated, it uses Minecraft's `SearchRecipeBookCategory`.
        // RECIPE_BOOK_BUTTONS: not translated, it uses Minecraft's `WidgetSprites`.
        // TABS: not translated, it uses Minecraft's `TabInfo`.
        // CookingPotRecipeBookComponent.CookingPotRecipeBookComponent(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookComponent`).
        // CookingPotRecipeBookComponent.getFilterButtonTextures(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookComponent`).
        // CookingPotRecipeBookComponent.isCraftingSlot(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookComponent`).
        // CookingPotRecipeBookComponent.selectMatchingRecipes(2): not translated, its class isn't translated (it extends Minecraft's `RecipeBookComponent`).
        // CookingPotRecipeBookComponent.getRecipeFilterName(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookComponent`).
        // CookingPotRecipeBookComponent.fillGhostRecipe(3): not translated, its class isn't translated (it extends Minecraft's `RecipeBookComponent`).
    }

    /// `CookingPotScreen` (vectorwing/farmersdelight/client/gui/CookingPotScreen.java)
    pub mod cooking_pot_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractRecipeBookScreen`. Its static parts are below.
        pub static BACKGROUND_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "textures/gui/cooking_pot.png"));
        // HEAT_ICON: not translated, it uses `Rectangle` (java.awt).
        // PROGRESS_ARROW: not translated, it uses `Rectangle` (java.awt).
        // CookingPotScreen.CookingPotScreen(3): not translated, its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`).
        // CookingPotScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`).
        // CookingPotScreen.getRecipeBookButtonPosition(0): not translated, its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`).
        // CookingPotScreen.renderHeatIndicatorTooltip(3): not translated, its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`).
        // CookingPotScreen.extractTooltip(3): not translated, its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`).
        // CookingPotScreen.renderMealDisplayTooltip(3): not translated, its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`).
        // CookingPotScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`).
    }

    /// `CookingPotTooltip` (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java)
    pub mod cooking_pot_tooltip {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ClientTooltipComponent`. Its static parts are below.
        pub const ITEM_SIZE: i32 = 16;
        pub const MARGIN: i32 = 4;
        // CookingPotTooltip.CookingPotTooltip(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // CookingPotTooltip.getHeight(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // CookingPotTooltip.getWidth(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // CookingPotTooltip.extractImage(6): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // CookingPotTooltip.extractText(4): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
    }

    /// `CookingPotTooltip.CookingPotTooltipComponent` (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java)
    pub mod cooking_pot_tooltip_cooking_pot_tooltip_component {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `TooltipComponent`. Its static parts are below.
        // CookingPotTooltip.CookingPotTooltipComponent.CookingPotTooltipComponent(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // CookingPotTooltip.CookingPotTooltipComponent.mealStack(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
    }

    /// `HUDOverlays` (vectorwing/farmersdelight/client/gui/HUDOverlays.java)
    pub mod hud_overlays {
        use super::*;
        pub static HEALTH_ICONS_OFFSET: JCell<i32> = JCell::new(39);
        pub static FOOD_ICONS_OFFSET: JCell<i32> = JCell::new(39);
        pub static MOD_ICONS_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "textures/gui/fd_icons.png"));
        #[derive(Clone, Debug)]
        pub struct HUDOverlays {
        }
        impl HUDOverlays {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
        }
        // HUDOverlays.drawNourishmentOverlay(6): not translated, it uses Minecraft's `FoodData`.
        // HUDOverlays.drawComfortOverlay(5): not translated, its body couldn't be read: an assert.
    }

    /// `HUDOverlays.BaseOverlay` (vectorwing/farmersdelight/client/gui/HUDOverlays.java)
    pub mod hud_overlays_base_overlay {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BaseOverlay {
        }
        impl BaseOverlay {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // HUDOverlays.BaseOverlay.render(7): not translated, it has no body.
            // HUDOverlays.BaseOverlay.render(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
            // HUDOverlays.BaseOverlay.shouldRenderOverlay(4): not translated, it uses Minecraft's `Minecraft`.
        }
    }

    /// `HUDOverlays.ComfortOverlay` (vectorwing/farmersdelight/client/gui/HUDOverlays.java)
    pub mod hud_overlays_comfort_overlay {
        use super::*;
        // The class itself isn't translated: it extends `BaseOverlay`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "comfort"));
        // INSTANCE: not translated, it uses ComfortOverlay, which isn't translated (it extends `BaseOverlay`).
        // HUDOverlays.ComfortOverlay.render(7): not translated, its class isn't translated (it extends `BaseOverlay`).
        // HUDOverlays.ComfortOverlay.shouldRenderOverlay(4): not translated, its class isn't translated (it extends `BaseOverlay`).
    }

    /// `HUDOverlays.NourishmentOverlay` (vectorwing/farmersdelight/client/gui/HUDOverlays.java)
    pub mod hud_overlays_nourishment_overlay {
        use super::*;
        // The class itself isn't translated: it extends `BaseOverlay`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "nourishment"));
        // INSTANCE: not translated, it uses NourishmentOverlay, which isn't translated (it extends `BaseOverlay`).
        // HUDOverlays.NourishmentOverlay.render(7): not translated, its class isn't translated (it extends `BaseOverlay`).
        // HUDOverlays.NourishmentOverlay.shouldRenderOverlay(4): not translated, its class isn't translated (it extends `BaseOverlay`).
    }

    /// `SkilletCookingConditionalItemModelProperty` (vectorwing/farmersdelight/client/model/SkilletCookingConditionalItemModelProperty.java)
    pub mod skillet_cooking_conditional_item_model_property {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ConditionalItemModelProperty`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SkilletCookingConditionalItemModelProperty.get(5): not translated, its class isn't translated (it implements Minecraft's `ConditionalItemModelProperty`).
        // SkilletCookingConditionalItemModelProperty.type(0): not translated, its class isn't translated (it implements Minecraft's `ConditionalItemModelProperty`).
    }

    /// `SparkleParticle` (vectorwing/farmersdelight/client/particle/SparkleParticle.java)
    pub mod sparkle_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        // SparkleParticle.SparkleParticle(6): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // SparkleParticle.getLightCoords(1): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // SparkleParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // SparkleParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `SparkleParticle.Factory` (vectorwing/farmersdelight/client/particle/SparkleParticle.java)
    pub mod sparkle_particle_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // SparkleParticle.Factory.Factory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // SparkleParticle.Factory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `StarParticle` (vectorwing/farmersdelight/client/particle/StarParticle.java)
    pub mod star_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        // StarParticle.StarParticle(5): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // StarParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // StarParticle.getQuadSize(1): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // StarParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `StarParticle.Factory` (vectorwing/farmersdelight/client/particle/StarParticle.java)
    pub mod star_particle_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // StarParticle.Factory.Factory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // StarParticle.Factory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `SteamParticle` (vectorwing/farmersdelight/client/particle/SteamParticle.java)
    pub mod steam_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        // SteamParticle.SteamParticle(8): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // SteamParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // SteamParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `SteamParticle.Factory` (vectorwing/farmersdelight/client/particle/SteamParticle.java)
    pub mod steam_particle_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // SteamParticle.Factory.Factory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // SteamParticle.Factory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `CookingPotRecipeDisplay` (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java)
    pub mod cooking_pot_recipe_display {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RecipeDisplay`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CookingPotRecipeDisplay.CookingPotRecipeDisplay(6): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
        // CookingPotRecipeDisplay.type(0): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
        // CookingPotRecipeDisplay.isEnabled(1): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
        // CookingPotRecipeDisplay.ingredients(0): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
        // CookingPotRecipeDisplay.container(0): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
        // CookingPotRecipeDisplay.result(0): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
        // CookingPotRecipeDisplay.craftingStation(0): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
        // CookingPotRecipeDisplay.duration(0): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
        // CookingPotRecipeDisplay.experience(0): not translated, its class isn't translated (it implements Minecraft's `RecipeDisplay`).
    }

    /// `AbstractStoveRenderer` (vectorwing/farmersdelight/client/renderer/AbstractStoveRenderer.java)
    pub mod abstract_stove_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const SIZE: f32 = 0.375f32;
        // AbstractStoveRenderer.AbstractStoveRenderer(1): not translated, its class isn't translated (it's generic).
        // AbstractStoveRenderer.extractRenderState(5): not translated, its class isn't translated (it's generic).
        // AbstractStoveRenderer.submit(4): not translated, its class isn't translated (it's generic).
        // AbstractStoveRenderer.getStoveItemOffset(1): not translated, it has no body.
    }

    /// `CuttingBoardRenderer` (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java)
    pub mod cutting_board_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        // CuttingBoardRenderer.CuttingBoardRenderer(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // CuttingBoardRenderer.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // CuttingBoardRenderer.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // CuttingBoardRenderer.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // CuttingBoardRenderer.renderItemLayingDown(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // CuttingBoardRenderer.renderBlock(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // CuttingBoardRenderer.renderItemCarved(3): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // CuttingBoardRenderer.getModelCount(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
    }

    /// `DefaultStoveRenderer` (vectorwing/farmersdelight/client/renderer/DefaultStoveRenderer.java)
    pub mod default_stove_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // DefaultStoveRenderer.DefaultStoveRenderer(1): not translated, its class isn't translated (it's generic).
        // DefaultStoveRenderer.createRenderState(0): not translated, its class isn't translated (it's generic).
        // DefaultStoveRenderer.getStoveItemOffset(1): not translated, its class isn't translated (it's generic).
    }

    /// `SkilletFlipItemRenderer` (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java)
    pub mod skillet_flip_item_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ItemModel`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| farmers_delight::id("skillet_flip"));
        // SPECIAL_RENDERER: not translated, it uses SpecialRenderer, which isn't translated (it implements Minecraft's `SpecialModelRenderer`).
        // SkilletFlipItemRenderer.SkilletFlipItemRenderer(2): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
        // SkilletFlipItemRenderer.update(7): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
        // SkilletFlipItemRenderer.applyTransform(4): not translated, it uses Minecraft's `ClientLevel`.
    }

    /// `SkilletFlipItemRenderer.Argument` (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java)
    pub mod skillet_flip_item_renderer_argument {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStackRenderState`. Its static parts are below.
    }

    /// `SkilletFlipItemRenderer.SpecialRenderer` (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java)
    pub mod skillet_flip_item_renderer_special_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SpecialModelRenderer`. Its static parts are below.
        // SkilletFlipItemRenderer.SpecialRenderer.submit(7): not translated, its class isn't translated (it implements Minecraft's `SpecialModelRenderer`).
        // SkilletFlipItemRenderer.SpecialRenderer.getExtents(1): not translated, its class isn't translated (it implements Minecraft's `SpecialModelRenderer`).
        // SkilletFlipItemRenderer.SpecialRenderer.extractArgument(1): not translated, its class isn't translated (it implements Minecraft's `SpecialModelRenderer`).
    }

    /// `SkilletFlipItemRenderer.Unbaked` (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java)
    pub mod skillet_flip_item_renderer_unbaked {
        use super::*;
        // The class itself isn't translated: it implements `Unbaked`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SkilletFlipItemRenderer.Unbaked.Unbaked(1): not translated, its class isn't translated (it implements `Unbaked`).
        // SkilletFlipItemRenderer.Unbaked.bake(2): not translated, its class isn't translated (it implements `Unbaked`).
        // SkilletFlipItemRenderer.Unbaked.resolveDependencies(1): not translated, its class isn't translated (it implements `Unbaked`).
        // SkilletFlipItemRenderer.Unbaked.type(0): not translated, its class isn't translated (it implements `Unbaked`).
        // SkilletFlipItemRenderer.Unbaked.model(0): not translated, its class isn't translated (it implements `Unbaked`).
    }

    /// `SkilletRenderer` (vectorwing/farmersdelight/client/renderer/SkilletRenderer.java)
    pub mod skillet_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        // SkilletRenderer.SkilletRenderer(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // SkilletRenderer.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // SkilletRenderer.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // SkilletRenderer.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // SkilletRenderer.getModelCount(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
    }

    /// `CuttingBoardRenderState` (vectorwing/farmersdelight/client/renderer/state/CuttingBoardRenderState.java)
    pub mod cutting_board_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `SkilletRenderState` (vectorwing/farmersdelight/client/renderer/state/SkilletRenderState.java)
    pub mod skillet_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `StoveRenderState` (vectorwing/farmersdelight/client/renderer/state/StoveRenderState.java)
    pub mod stove_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `CuttingBoardTrigger` (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java)
    pub mod cutting_board_trigger {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleCriterionTrigger`. Its static parts are below.
        // CuttingBoardTrigger.codec(0): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // CuttingBoardTrigger.trigger(1): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
    }

    /// `CuttingBoardTrigger.TriggerInstance` (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java)
    pub mod cutting_board_trigger_trigger_instance {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SimpleInstance`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // CuttingBoardTrigger.TriggerInstance.TriggerInstance(1): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // CuttingBoardTrigger.TriggerInstance.simple(0): not translated, it uses Minecraft's `Criterion`.
        // CuttingBoardTrigger.TriggerInstance.test(0): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // CuttingBoardTrigger.TriggerInstance.player(0): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
    }

    /// `BlockShapes` (vectorwing/farmersdelight/common/BlockShapes.java)
    pub mod block_shapes {
        use super::*;
        // TRAY_OUTER_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // TRAY_INNER_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // TRAY_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // ROAST_CHICKEN_SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        // HONEY_GLAZED_HAM_SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        // SHEPHERDS_PIE_SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        #[derive(Clone, Debug)]
        pub struct BlockShapes {
        }
        impl BlockShapes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `AbstractStoveBlock` (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java)
    pub mod abstract_stove_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `Property`.
        // LIT: not translated, it uses Minecraft's `BooleanProperty`.
        // GRILLING_AREA: not translated, it uses Minecraft's `VoxelShape`.
        // AbstractStoveBlock.AbstractStoveBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.tryToIgnite(7): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.tryToExtinguish(7): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.tryToPlaceFoodItem(7): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.ignite(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.extinguish(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.stepOn(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.burnEntitySteppingOnStove(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.isStoveTopCovered(3): not translated, it uses Minecraft's `Level`.
        // AbstractStoveBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.createStoveTicker(2): not translated, it's generic.
        // AbstractStoveBlock.getBlockPathType(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // AbstractStoveBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `BasketBlock` (vectorwing/farmersdelight/common/block/BasketBlock.java)
    pub mod basket_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `Property`.
        // ENABLED: not translated, it uses Minecraft's `BooleanProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // OUT_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // RENDER_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // COLLISION_SHAPE_FACING: not translated, it uses `ImmutableMap` (com.google.common.collect).
        // BasketBlock.makeHollowCubeShape(1): not translated, it uses Minecraft's `VoxelShape`.
        // BasketBlock.BasketBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.neighborChanged(6): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.useShapeForLightOcclusion(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.getOcclusionShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.getInteractionShape(3): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.getTicker(3): not translated, it's generic.
        // BasketBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BasketBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `BuddingBushBlock` (vectorwing/farmersdelight/common/block/BuddingBushBlock.java)
    pub mod budding_bush_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `VegetationBlock`. Its static parts are below.
        pub const MAX_AGE: i32 = 3;
        // AGE: not translated, it uses Minecraft's `IntegerProperty`.
        // SHAPE_BY_AGE: not translated, it uses Minecraft's `VoxelShape`.
        // BuddingBushBlock.BuddingBushBlock(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.mayPlaceOn(3): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.getAgeProperty(0): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.getMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.getAge(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.getStateForAge(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.isMaxAge(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.isRandomlyTicking(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.canGrowPastMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.growPastMaxAge(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.getGrowthSpeed(3): not translated, it uses Minecraft's `BlockState`.
        // BuddingBushBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.entityInside(6): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.getBaseSeedId(0): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // BuddingBushBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
    }

    /// `BuddingTomatoBlock` (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java)
    pub mod budding_tomato_block {
        use super::*;
        // The class itself isn't translated: it extends `BuddingBushBlock`. Its static parts are below.
        // BuddingTomatoBlock.BuddingTomatoBlock(1): not translated, its class isn't translated (it extends `BuddingBushBlock`).
        // BuddingTomatoBlock.updateShape(8): not translated, its class isn't translated (it extends `BuddingBushBlock`).
        // BuddingTomatoBlock.canGrowPastMaxAge(0): not translated, its class isn't translated (it extends `BuddingBushBlock`).
        // BuddingTomatoBlock.growPastMaxAge(4): not translated, its class isn't translated (it extends `BuddingBushBlock`).
        // BuddingTomatoBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BuddingBushBlock`).
        // BuddingTomatoBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `BuddingBushBlock`).
        // BuddingTomatoBlock.getBonemealAgeIncrease(1): not translated, its class isn't translated (it extends `BuddingBushBlock`).
        // BuddingTomatoBlock.performBonemeal(5): not translated, its class isn't translated (it extends `BuddingBushBlock`).
    }

    /// `CabbageBlock` (vectorwing/farmersdelight/common/block/CabbageBlock.java)
    pub mod cabbage_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CropBlock`. Its static parts are below.
        // SHAPE_BY_AGE: not translated, it uses Minecraft's `VoxelShape`.
        // CabbageBlock.CabbageBlock(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // CabbageBlock.getBaseSeedId(0): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // CabbageBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
    }

    /// `CabinetBlock` (vectorwing/farmersdelight/common/block/CabinetBlock.java)
    pub mod cabinet_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `Property`.
        // OPEN: not translated, it uses Minecraft's `BooleanProperty`.
        // CabinetBlock.CabinetBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.tick(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CabinetBlock.affectNeighborsAfterRemoval(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `CanvasRugBlock` (vectorwing/farmersdelight/common/block/CanvasRugBlock.java)
    pub mod canvas_rug_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // CanvasRugBlock.CanvasRugBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CanvasRugBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CanvasRugBlock.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `CeilingHangingCanvasSignBlock` (vectorwing/farmersdelight/common/block/CeilingHangingCanvasSignBlock.java)
    pub mod ceiling_hanging_canvas_sign_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CeilingHangingSignBlock`. Its static parts are below.
        // CeilingHangingCanvasSignBlock.CeilingHangingCanvasSignBlock(2): not translated, its class isn't translated (it extends Minecraft's `CeilingHangingSignBlock`).
        // CeilingHangingCanvasSignBlock.getBackgroundColor(0): not translated, its class isn't translated (it extends Minecraft's `CeilingHangingSignBlock`).
    }

    /// `CookingPotBlock` (vectorwing/farmersdelight/common/block/CookingPotBlock.java)
    pub mod cooking_pot_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // SUPPORT: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_WITH_TRAY: not translated, it uses Minecraft's `VoxelShape`.
        // CookingPotBlock.CookingPotBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.affectNeighborsAfterRemoval(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getTrayState(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CookingPotBlock.getTicker(3): not translated, it's generic.
    }

    /// `CuttingBoardBlock` (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java)
    pub mod cutting_board_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `Property`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // CuttingBoardBlock.CuttingBoardBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.init(0): not translated, it uses Fabric's `UseBlockCallback`.EVENT.
        // CuttingBoardBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.affectNeighborsAfterRemoval(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.isPossibleToRespawnInThis(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // CuttingBoardBlock.spawnCuttingParticles(4): not translated, it uses Minecraft's `Level`.
    }

    /// `CuttingBoardBlock.ToolCarvingEvent` (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java)
    pub mod cutting_board_block_tool_carving_event {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ToolCarvingEvent {
        }
        impl ToolCarvingEvent {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CuttingBoardBlock.ToolCarvingEvent.onSneakPlaceTool(4): not translated, it uses Minecraft's `InteractionResult`.
    }

    /// `AbstractStoveBlockEntity` (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java)
    pub mod abstract_stove_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // AbstractStoveBlockEntity.AbstractStoveBlockEntity(4): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.getInventorySlotCount(0): not translated, it has no body.
        // AbstractStoveBlockEntity.getStoveItemOffset(1): not translated, it has no body.
        // AbstractStoveBlockEntity.getItems(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.writeItems(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.preRemoveSideEffects(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.serverTick(4): not translated, it uses Minecraft's `ServerLevel`.
        // AbstractStoveBlockEntity.cookAndOutputItems(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.coolItems(0): not translated, its body couldn't be read: an assert.
        // AbstractStoveBlockEntity.getCookingRecipe(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.getNextEmptySlot(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.placeFood(3): not translated, its body couldn't be read: an assert.
        // AbstractStoveBlockEntity.shouldDropItems(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.streamItems(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.isEmpty(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.isFull(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.dropAllItems(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.extinguish(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.ignite(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.clearContent(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // AbstractStoveBlockEntity.createHandler(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `Basket` (vectorwing/farmersdelight/common/block/entity/Basket.java)
    pub mod basket {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Basket.getFacingCollectionArea(1): not translated, its class isn't translated (it's an interface).
        // Basket.getLevelX(0): not translated, it has no body.
        // Basket.getLevelY(0): not translated, it has no body.
        // Basket.getLevelZ(0): not translated, it has no body.
        // Basket.setCooldown(1): not translated, it has no body.
        // Basket.isOnCooldown(0): not translated, it has no body.
        // Basket.isOnCustomCooldown(0): not translated, it has no body.
        // Basket.tryTransfer(1): not translated, it has no body.
        // Basket.collectItems(2): not translated, its class isn't translated (it's an interface).
        // Basket.getItemsToCollect(2): not translated, its class isn't translated (it's an interface).
        // Basket.collectItem(1): not translated, its class isn't translated (it's an interface).
        // Basket.insert(1): not translated, its class isn't translated (it's an interface).
        // Basket.insert(2): not translated, its class isn't translated (it's an interface).
        // Basket.canMergeItems(2): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `BasketBlockEntity` (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java)
    pub mod basket_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RandomizableContainerBlockEntity`. Its static parts are below.
        // BasketBlockEntity.BasketBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.init(0): not translated, it uses Fabric's `ItemStorage`.SIDED.
        // BasketBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.removeItem(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.setItem(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.getItems(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.setItems(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.setCooldown(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.isOnCooldown(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.isOnCustomCooldown(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.tryTransfer(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.isFull(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.getLevelX(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.getLevelY(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.getLevelZ(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BasketBlockEntity.pushItemsTick(4): not translated, it uses Minecraft's `Level`.
    }

    /// `CabinetBlockEntity` (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java)
    pub mod cabinet_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RandomizableContainerBlockEntity`. Its static parts are below.
        // CabinetBlockEntity.CabinetBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.init(0): not translated, it uses Fabric's `ItemStorage`.SIDED.
        // CabinetBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.getItems(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.setItems(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.startOpen(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.stopOpen(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.recheckOpen(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.updateBlockState(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // CabinetBlockEntity.playSound(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
    }

    /// `CookingPotMealSlot` (vectorwing/farmersdelight/common/block/entity/container/CookingPotMealSlot.java)
    pub mod cooking_pot_meal_slot {
        use super::*;
        // The class itself isn't translated: it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory). Its static parts are below.
        // CookingPotMealSlot.CookingPotMealSlot(4): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotMealSlot.mayPlace(1): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotMealSlot.mayPickup(1): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
    }

    /// `CookingPotMenu` (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java)
    pub mod cooking_pot_menu {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RecipeBookMenu`. Its static parts are below.
        pub static EMPTY_CONTAINER_SLOT_BOWL: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "container/slot/bowl"));
        pub const INDEX_MEAL: i32 = 6;
        pub const INDEX_CONTAINER: i32 = 7;
        pub const INDEX_OUTPUT: i32 = 8;
        // CookingPotMenu.CookingPotMenu(3): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // CookingPotMenu.CookingPotMenu(4): not translated, its body couldn't be read: an anonymous class.
        // CookingPotMenu.getTileEntity(2): not translated, it uses CookingPotBlockEntity, which isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotMenu.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // CookingPotMenu.quickMoveStack(2): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // CookingPotMenu.getCookProgressionScaled(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // CookingPotMenu.isHeated(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // CookingPotMenu.handlePlacement(5): not translated, its body couldn't be read: an anonymous class.
        // CookingPotMenu.fillCraftSlotsStackedContents(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // CookingPotMenu.getRecipeBookType(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
    }

    /// `CookingPotResultSlot` (vectorwing/farmersdelight/common/block/entity/container/CookingPotResultSlot.java)
    pub mod cooking_pot_result_slot {
        use super::*;
        // The class itself isn't translated: it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory). Its static parts are below.
        // CookingPotResultSlot.CookingPotResultSlot(6): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotResultSlot.mayPlace(1): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotResultSlot.remove(1): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotResultSlot.onTake(2): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotResultSlot.onQuickCraft(2): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotResultSlot.checkTakeAchievements(1): not translated, its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)).
    }

    /// `CookingPotBlockEntity` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java)
    pub mod cooking_pot_block_entity {
        use super::*;
        // The class itself isn't translated: it extends `SyncedBlockEntity`. Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // RECIPES_USED_CODEC: not translated, it uses Minecraft's `Codec`.
        pub const MEAL_DISPLAY_SLOT: i32 = 6;
        pub const CONTAINER_SLOT: i32 = 7;
        pub const OUTPUT_SLOT: i32 = 8;
        pub const INVENTORY_SIZE: i32 = 9;
        // INGREDIENT_REMAINDER_OVERRIDES: not translated, it uses Minecraft's `Item`.
        // CookingPotBlockEntity.CookingPotBlockEntity(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.init(0): not translated, it uses Fabric's `ItemStorage`.SIDED.
        // CookingPotBlockEntity.getMealFromItem(1): not translated, it uses Minecraft's `ItemStack`.
        // CookingPotBlockEntity.takeServingFromItem(1): not translated, it uses Minecraft's `ItemStack`.
        // CookingPotBlockEntity.getContainerFromItem(1): not translated, it uses Minecraft's `ItemStack`.
        // CookingPotBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.writeItems(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.preRemoveSideEffects(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.writeMeal(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getAsItem(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.cookingTick(4): not translated, it uses Minecraft's `ServerLevel`.
        // CookingPotBlockEntity.animationTick(4): not translated, it uses Minecraft's `Level`.
        // CookingPotBlockEntity.getMatchingRecipe(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getContainer(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.hasInput(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.canCook(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.processCooking(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.ejectIngredientRemainder(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.setRecipeUsed(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getRecipeUsed(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.awardUsedRecipes(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getUsedRecipesAndPopExperience(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.splitAndSpawnExperience(4): not translated, it uses Minecraft's `ServerLevel`.
        // CookingPotBlockEntity.isHeated(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getInventory(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getMeal(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getDroppableInventory(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.moveMealToOutput(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.useStoredContainersOnMeal(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.useHeldItemOnMeal(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.doesMealHaveContainer(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.isContainerValid(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getName(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getDisplayName(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getCustomName(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.createMenu(3): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getStorage(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.setRemoved(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.applyImplicitComponents(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.collectImplicitComponents(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CookingPotBlockEntity.createHandler(0): not translated, its body couldn't be read: an anonymous class.
        // CookingPotBlockEntity.createIntArray(0): not translated, its body couldn't be read: an anonymous class.
        // CookingPotBlockEntity.getScreenOpeningData(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
    }

    /// `CuttingBoardBlockEntity` (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java)
    pub mod cutting_board_block_entity {
        use super::*;
        // The class itself isn't translated: it extends `SyncedBlockEntity`. Its static parts are below.
        // CuttingBoardBlockEntity.CuttingBoardBlockEntity(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.init(0): not translated, it uses Fabric's `ItemStorage`.SIDED.
        // CuttingBoardBlockEntity.getStorage(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.preRemoveSideEffects(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.processStoredItemUsingTool(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.getMatchingRecipe(3): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.spawnCuttingParticles(3): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.playProcessingSound(3): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.playSound(3): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.canAddItem(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.addItem(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.removeItem(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.carveToolOnBoard(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.getInventory(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.getStoredItem(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.getMaxStackSize(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.isEmpty(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.isItemCarvingBoard(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.setRemoved(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // CuttingBoardBlockEntity.createHandler(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `CuttingBoardDispenseBehavior` (vectorwing/farmersdelight/common/block/entity/dispenser/CuttingBoardDispenseBehavior.java)
    pub mod cutting_board_dispense_behavior {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `OptionalDispenseItemBehavior`. Its static parts are below.
        // DISPENSE_ITEM_BEHAVIOR_HASH_MAP: not translated, it uses Minecraft's `Item`.
        // INSTANCE: not translated, it uses CuttingBoardDispenseBehavior, which isn't translated (it extends Minecraft's `OptionalDispenseItemBehavior`).
        // CuttingBoardDispenseBehavior.registerBehaviour(2): not translated, it uses Minecraft's `Item`.
        // CuttingBoardDispenseBehavior.dispense(2): not translated, its class isn't translated (it extends Minecraft's `OptionalDispenseItemBehavior`).
        // CuttingBoardDispenseBehavior.tryDispenseStackOnCuttingBoard(2): not translated, its class isn't translated (it extends Minecraft's `OptionalDispenseItemBehavior`).
    }

    /// `HeatableBlockEntity` (vectorwing/farmersdelight/common/block/entity/HeatableBlockEntity.java)
    pub mod heatable_block_entity {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HeatableBlockEntity.isHeated(2): not translated, its class isn't translated (it's an interface).
        // HeatableBlockEntity.requiresDirectHeat(0): not translated, its class isn't translated (it's an interface).
    }

    /// `BasketInvWrapper` (vectorwing/farmersdelight/common/block/entity/inventory/BasketInvWrapper.java)
    pub mod basket_inv_wrapper {
        use super::*;
        // The class itself isn't translated: it extends `InvWrapper` (vectorwing.farmersdelight.refabricated.inventory). Its static parts are below.
        // BasketInvWrapper.BasketInvWrapper(1): not translated, its class isn't translated (it extends `InvWrapper` (vectorwing.farmersdelight.refabricated.inventory)).
        // BasketInvWrapper.insertItem(3): not translated, its class isn't translated (it extends `InvWrapper` (vectorwing.farmersdelight.refabricated.inventory)).
    }

    /// `CookingPotItemHandler` (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java)
    pub mod cooking_pot_item_handler {
        use super::*;
        // The class itself isn't translated: it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory). Its static parts are below.
        pub const SLOTS_INPUT: i32 = 6;
        pub const SLOT_CONTAINER_INPUT: i32 = 7;
        pub const SLOT_MEAL_OUTPUT: i32 = 8;
        // CookingPotItemHandler.CookingPotItemHandler(2): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.isItemValid(2): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.getSlotCount(0): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.getSlot(1): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.getStackInSlot(1): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.setStackInSlot(2): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.insertItem(3): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.extractItem(3): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.getSlotLimit(1): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.insert(3): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.extract(3): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.iterator(0): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.getInsertableSlotsFor(1): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
        // CookingPotItemHandler.getSlotsContaining(1): not translated, its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)).
    }

    /// `SkilletBlockEntity` (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java)
    pub mod skillet_block_entity {
        use super::*;
        // The class itself isn't translated: it extends `SyncedBlockEntity`. Its static parts are below.
        // SkilletBlockEntity.SkilletBlockEntity(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.cookingTick(4): not translated, it uses Minecraft's `ServerLevel`.
        // SkilletBlockEntity.animationTick(4): not translated, it uses Minecraft's `Level`.
        // SkilletBlockEntity.cookAndOutputItems(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.isCooking(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.isHeated(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.getMatchingRecipe(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.preRemoveSideEffects(2): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.getSkilletAsItem(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.applyImplicitComponents(1): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.addItemToCook(3): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.removeItem(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.getInventory(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.getStoredStack(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.hasStoredStack(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
        // SkilletBlockEntity.createHandler(0): not translated, its body couldn't be read: an anonymous class.
        // SkilletBlockEntity.setRemoved(0): not translated, its class isn't translated (it extends `SyncedBlockEntity`).
    }

    /// `StoveBlockEntity` (vectorwing/farmersdelight/common/block/entity/StoveBlockEntity.java)
    pub mod stove_block_entity {
        use super::*;
        // The class itself isn't translated: it extends `AbstractStoveBlockEntity`. Its static parts are below.
        // StoveBlockEntity.StoveBlockEntity(2): not translated, its class isn't translated (it extends `AbstractStoveBlockEntity`).
        // StoveBlockEntity.particleTick(4): not translated, it uses Minecraft's `Level`.
        // StoveBlockEntity.addSmokeParticles(0): not translated, its body couldn't be read: an assert.
        // StoveBlockEntity.getInventorySlotCount(0): not translated, its class isn't translated (it extends `AbstractStoveBlockEntity`).
        // StoveBlockEntity.getStoveItemOffset(1): not translated, its class isn't translated (it extends `AbstractStoveBlockEntity`).
    }

    /// `SyncedBlockEntity` (vectorwing/farmersdelight/common/block/entity/SyncedBlockEntity.java)
    pub mod synced_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // SyncedBlockEntity.SyncedBlockEntity(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // SyncedBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // SyncedBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // SyncedBlockEntity.inventoryChanged(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `FeastBlock` (vectorwing/farmersdelight/common/block/FeastBlock.java)
    pub mod feast_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `Property`.
        // SERVINGS: not translated, it uses Minecraft's `IntegerProperty`.
        // SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        // FeastBlock.FeastBlock(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.FeastBlock(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.FeastBlock(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.getServingsProperty(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.getMaxServings(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.getServingItem(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.takeServing(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FeastBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `GleamingSaladBlock` (vectorwing/farmersdelight/common/block/GleamingSaladBlock.java)
    pub mod gleaming_salad_block {
        use super::*;
        // The class itself isn't translated: it extends `FeastBlock`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // GleamingSaladBlock.GleamingSaladBlock(3): not translated, its class isn't translated (it extends `FeastBlock`).
        // GleamingSaladBlock.GleamingSaladBlock(3): not translated, its class isn't translated (it extends `FeastBlock`).
        // GleamingSaladBlock.getShape(4): not translated, its class isn't translated (it extends `FeastBlock`).
        // GleamingSaladBlock.animateTick(4): not translated, its class isn't translated (it extends `FeastBlock`).
    }

    /// `HangingTomatoBlock` (vectorwing/farmersdelight/common/block/HangingTomatoBlock.java)
    pub mod hanging_tomato_block {
        use super::*;
        // The class itself isn't translated: it extends `TomatoBlock`. Its static parts are below.
        // HangingTomatoBlock.HangingTomatoBlock(1): not translated, its class isn't translated (it extends `TomatoBlock`).
        // HangingTomatoBlock.tick(4): not translated, its class isn't translated (it extends `TomatoBlock`).
        // HangingTomatoBlock.onRemove(5): not translated, its class isn't translated (it extends `TomatoBlock`).
        // HangingTomatoBlock.placeRope(2): not translated, it uses Minecraft's `LevelAccessor`.
        // HangingTomatoBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `TomatoBlock`).
    }

    /// `MushroomColonyBlock` (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java)
    pub mod mushroom_colony_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `VegetationBlock`. Its static parts are below.
        pub const PLACING_LIGHT_LEVEL: i32 = 13;
        // COLONY_AGE: not translated, it uses Minecraft's `IntegerProperty`.
        // SHAPE_BY_AGE: not translated, it uses Minecraft's `VoxelShape`.
        // MushroomColonyBlock.MushroomColonyBlock(2): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.getAgeProperty(0): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.mayPlaceOn(3): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.getMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.getBonemealAgeIncrease(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // MushroomColonyBlock.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
    }

    /// `OnionBlock` (vectorwing/farmersdelight/common/block/OnionBlock.java)
    pub mod onion_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CropBlock`. Its static parts are below.
        // SHAPE_BY_AGE: not translated, it uses Minecraft's `VoxelShape`.
        // OnionBlock.OnionBlock(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // OnionBlock.getBaseSeedId(0): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // OnionBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
    }

    /// `OrganicCompostBlock` (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java)
    pub mod organic_compost_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // COMPOSTING: not translated, it uses Minecraft's `IntegerProperty`.
        // OrganicCompostBlock.OrganicCompostBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OrganicCompostBlock.isRandomlyTicking(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OrganicCompostBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OrganicCompostBlock.getMaxCompostingStage(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OrganicCompostBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OrganicCompostBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OrganicCompostBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OrganicCompostBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `PieBlock` (vectorwing/farmersdelight/common/block/PieBlock.java)
    pub mod pie_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `Property`.
        // BITES: not translated, it uses Minecraft's `IntegerProperty`.
        // SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        // ROTATED_SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        // PieBlock.PieBlock(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.getPieSliceItem(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.getMaxBites(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.buildShapes(0): not translated, it uses Minecraft's `VoxelShape`.
        // PieBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.consumeBite(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.cutSlice(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PieBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `RiceBaleBlock` (vectorwing/farmersdelight/common/block/RiceBaleBlock.java)
    pub mod rice_bale_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `Property`.
        // RiceBaleBlock.RiceBaleBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RiceBaleBlock.fallOn(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RiceBaleBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RiceBaleBlock.getFireSpreadSpeed(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RiceBaleBlock.getFlammability(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RiceBaleBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RiceBaleBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RiceBaleBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `RiceBlock` (vectorwing/farmersdelight/common/block/RiceBlock.java)
    pub mod rice_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `VegetationBlock`. Its static parts are below.
        // AGE: not translated, it uses Minecraft's `IntegerProperty`.
        // SUPPORTING: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE_BY_AGE: not translated, it uses Minecraft's `VoxelShape`.
        // RiceBlock.RiceBlock(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.mayPlaceOn(3): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.getAgeProperty(0): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.getAge(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.getMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.withAge(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.isSupportingRiceUpper(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.getBonemealAgeIncrease(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.canPlaceLiquid(5): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // RiceBlock.placeLiquid(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
    }

    /// `RicePaniclesBlock` (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java)
    pub mod rice_panicles_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CropBlock`. Its static parts are below.
        // RICE_AGE: not translated, it uses Minecraft's `IntegerProperty`.
        // SHAPE_BY_AGE: not translated, it uses Minecraft's `VoxelShape`.
        // RicePaniclesBlock.RicePaniclesBlock(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // RicePaniclesBlock.getAgeProperty(0): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // RicePaniclesBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // RicePaniclesBlock.getMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // RicePaniclesBlock.getBaseSeedId(0): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // RicePaniclesBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // RicePaniclesBlock.getBonemealAgeIncrease(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // RicePaniclesBlock.mayPlaceOn(3): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // RicePaniclesBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
    }

    /// `RiceRollMedleyBlock` (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java)
    pub mod rice_roll_medley_block {
        use super::*;
        // The class itself isn't translated: it extends `FeastBlock`. Its static parts are below.
        // ROLL_SERVINGS: not translated, it uses Minecraft's `IntegerProperty`.
        // SHAPE_NORTH_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_EAST_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // RiceRollMedleyBlock.RiceRollMedleyBlock(1): not translated, its class isn't translated (it extends `FeastBlock`).
        // RiceRollMedleyBlock.getServingsProperty(0): not translated, its class isn't translated (it extends `FeastBlock`).
        // RiceRollMedleyBlock.getMaxServings(0): not translated, its class isn't translated (it extends `FeastBlock`).
        // RiceRollMedleyBlock.getServingItem(1): not translated, its class isn't translated (it extends `FeastBlock`).
        // RiceRollMedleyBlock.getShape(4): not translated, its class isn't translated (it extends `FeastBlock`).
        // RiceRollMedleyBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `FeastBlock`).
    }

    /// `RichSoilBlock` (vectorwing/farmersdelight/common/block/RichSoilBlock.java)
    pub mod rich_soil_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // RichSoilBlock.RichSoilBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RichSoilBlock.init(0): not translated, it uses Minecraft's `Block`.
        // RichSoilBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RichSoilBlock.tryBoostingPlantsAboveAndBelow(3): not translated, it uses Minecraft's `ServerLevel`.
        // RichSoilBlock.boostPlant(4): not translated, it uses Minecraft's `BlockState`.
        // RichSoilBlock.convertMushroomToColony(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `RichSoilFarmlandBlock` (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java)
    pub mod rich_soil_farmland_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FarmlandBlock`. Its static parts are below.
        // RichSoilFarmlandBlock.RichSoilFarmlandBlock(1): not translated, its class isn't translated (it extends Minecraft's `FarmlandBlock`).
        // RichSoilFarmlandBlock.isNearWater(2): not translated, it uses Minecraft's `LevelReader`.
        // RichSoilFarmlandBlock.turnToRichSoil(4): not translated, it uses Minecraft's `Entity`.
        // RichSoilFarmlandBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `FarmlandBlock`).
        // RichSoilFarmlandBlock.tick(4): not translated, its class isn't translated (it extends Minecraft's `FarmlandBlock`).
        // RichSoilFarmlandBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `FarmlandBlock`).
        // RichSoilFarmlandBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `FarmlandBlock`).
        // RichSoilFarmlandBlock.fallOn(5): not translated, its class isn't translated (it extends Minecraft's `FarmlandBlock`).
    }

    /// `RopeBlock` (vectorwing/farmersdelight/common/block/RopeBlock.java)
    pub mod rope_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `IronBarsBlock`. Its static parts are below.
        // TIED_TO_BELL: not translated, it uses Minecraft's `BooleanProperty`.
        // LOWER_SUPPORT_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // RopeBlock.RopeBlock(1): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.getStateWithConnections(4): not translated, it uses Minecraft's `BlockState`.
        // RopeBlock.tieToAnythingValid(2): not translated, it uses Minecraft's `BlockState`.
        // RopeBlock.tieToRopeAndWalls(1): not translated, it uses Minecraft's `BlockState`.
        // RopeBlock.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.getBlockSupportShape(3): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.canBeReplaced(2): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // RopeBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
    }

    /// `RopeFenceBlock` (vectorwing/farmersdelight/common/block/RopeFenceBlock.java)
    pub mod rope_fence_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CrossCollisionBlock`. Its static parts are below.
        // POST: not translated, it uses Minecraft's `VoxelShape`.
        // RopeFenceBlock.RopeFenceBlock(1): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
        // RopeFenceBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
        // RopeFenceBlock.getVisualShape(4): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
        // RopeFenceBlock.getOcclusionShape(1): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
        // RopeFenceBlock.connectsTo(3): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
        // RopeFenceBlock.isSameFence(1): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
        // RopeFenceBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
        // RopeFenceBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
        // RopeFenceBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `CrossCollisionBlock`).
    }

    /// `RopeFenceGateBlock` (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java)
    pub mod rope_fence_gate_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FenceGateBlock`. Its static parts are below.
        // Z_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // X_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // Z_COLLISION_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // X_COLLISION_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // RopeFenceGateBlock.RopeFenceGateBlock(1): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
        // RopeFenceGateBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
        // RopeFenceGateBlock.isWall(1): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
        // RopeFenceGateBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
        // RopeFenceGateBlock.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
    }

    /// `RotatedFeastBlock` (vectorwing/farmersdelight/common/block/RotatedFeastBlock.java)
    pub mod rotated_feast_block {
        use super::*;
        // The class itself isn't translated: it extends `FeastBlock`. Its static parts are below.
        // RotatedFeastBlock.RotatedFeastBlock(5): not translated, its class isn't translated (it extends `FeastBlock`).
        // RotatedFeastBlock.RotatedFeastBlock(5): not translated, its class isn't translated (it extends `FeastBlock`).
        // RotatedFeastBlock.getShape(4): not translated, its class isn't translated (it extends `FeastBlock`).
    }

    /// `SafetyNetBlock` (vectorwing/farmersdelight/common/block/SafetyNetBlock.java)
    pub mod safety_net_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SafetyNetBlock.SafetyNetBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SafetyNetBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SafetyNetBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SafetyNetBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SafetyNetBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SafetyNetBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SafetyNetBlock.fallOn(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `SandyShrubBlock` (vectorwing/farmersdelight/common/block/SandyShrubBlock.java)
    pub mod sandy_shrub_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `VegetationBlock`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SandyShrubBlock.SandyShrubBlock(1): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // SandyShrubBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // SandyShrubBlock.mayPlaceOn(3): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // SandyShrubBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // SandyShrubBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
        // SandyShrubBlock.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `VegetationBlock`).
    }

    /// `SkilletBlock` (vectorwing/farmersdelight/common/block/SkilletBlock.java)
    pub mod skillet_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        pub const MINIMUM_COOKING_TIME: i32 = 60;
        // FACING: not translated, it uses Minecraft's `Property`.
        // SUPPORT: not translated, it uses Minecraft's `BooleanProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_WITH_TRAY: not translated, it uses Minecraft's `VoxelShape`.
        // SkilletBlock.SkilletBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SkilletBlock.getTicker(3): not translated, it's generic.
        // SkilletBlock.getTrayState(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        pub fn get_skillet_cooking_time(original_cooking_time: i32, fire_aspect_level: i32) -> i32 {
            let cooking_time: i32 = if original_cooking_time > 0 { original_cooking_time } else { 600 };
            let cooking_seconds: i32 = cooking_time / 20;
            let mut cooking_time_reduction: f32 = 0.2f32;
            if fire_aspect_level > 0 {
                cooking_time_reduction = (((cooking_time_reduction as f64) - ((fire_aspect_level as f64) * 0.05f64)) as f32);
            }
            let result: i32 = (((cooking_seconds as f32) * cooking_time_reduction) as i32) * 20;
            return j_clamp(result, 60, original_cooking_time);
        }
    }

    /// `StandingCanvasSignBlock` (vectorwing/farmersdelight/common/block/StandingCanvasSignBlock.java)
    pub mod standing_canvas_sign_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StandingSignBlock`. Its static parts are below.
        // StandingCanvasSignBlock.StandingCanvasSignBlock(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // StandingCanvasSignBlock.getBackgroundColor(0): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
    }

    /// `CanvasSign` (vectorwing/farmersdelight/common/block/state/CanvasSign.java)
    pub mod canvas_sign {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CanvasSign.getBackgroundColor(0): not translated, it has no body.
        // CanvasSign.isDarkBackground(0): not translated, its class isn't translated (it's an interface).
    }

    /// `CookingPotSupport` (vectorwing/farmersdelight/common/block/state/CookingPotSupport.java)
    pub mod cooking_pot_support {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CookingPotSupport.CookingPotSupport(1): not translated, its class isn't translated (its constants carry values).
        // CookingPotSupport.toString(0): not translated, its class isn't translated (its constants carry values).
        // CookingPotSupport.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `StoveBlock` (vectorwing/farmersdelight/common/block/StoveBlock.java)
    pub mod stove_block {
        use super::*;
        // The class itself isn't translated: it extends `AbstractStoveBlock`. Its static parts are below.
        // StoveBlock.StoveBlock(1): not translated, its class isn't translated (it extends `AbstractStoveBlock`).
        // StoveBlock.newBlockEntity(2): not translated, its class isn't translated (it extends `AbstractStoveBlock`).
        // StoveBlock.getTicker(3): not translated, it's generic.
        // StoveBlock.animateTick(4): not translated, its class isn't translated (it extends `AbstractStoveBlock`).
    }

    /// `StrawBaleBlock` (vectorwing/farmersdelight/common/block/StrawBaleBlock.java)
    pub mod straw_bale_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HayBlock`. Its static parts are below.
        // StrawBaleBlock.StrawBaleBlock(1): not translated, its class isn't translated (it extends Minecraft's `HayBlock`).
        // StrawBaleBlock.getFireSpreadSpeed(4): not translated, its class isn't translated (it extends Minecraft's `HayBlock`).
        // StrawBaleBlock.getFlammability(4): not translated, its class isn't translated (it extends Minecraft's `HayBlock`).
    }

    /// `TatamiBlock` (vectorwing/farmersdelight/common/block/TatamiBlock.java)
    pub mod tatami_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `Property`.
        // PAIRED: not translated, it uses Minecraft's `BooleanProperty`.
        // TatamiBlock.TatamiBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TatamiBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TatamiBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TatamiBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TatamiBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TatamiBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TatamiBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `TatamiHalfMatBlock` (vectorwing/farmersdelight/common/block/TatamiHalfMatBlock.java)
    pub mod tatami_half_mat_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HorizontalDirectionalBlock`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // TatamiHalfMatBlock.TatamiHalfMatBlock(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiHalfMatBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiHalfMatBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiHalfMatBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiHalfMatBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiHalfMatBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
    }

    /// `TatamiMatBlock` (vectorwing/farmersdelight/common/block/TatamiMatBlock.java)
    pub mod tatami_mat_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HorizontalDirectionalBlock`. Its static parts are below.
        // PART: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // TatamiMatBlock.TatamiMatBlock(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiMatBlock.getDirectionToOther(2): not translated, it uses Minecraft's `Direction`.
        // TatamiMatBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiMatBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiMatBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiMatBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiMatBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiMatBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiMatBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // TatamiMatBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
    }

    /// `TomatoBlock` (vectorwing/farmersdelight/common/block/TomatoBlock.java)
    pub mod tomato_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CropBlock`. Its static parts are below.
        // VINE_AGE: not translated, it uses Minecraft's `IntegerProperty`.
        // ROPELOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // TomatoBlock.TomatoBlock(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.TomatoBlock(2): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.isRandomlyTicking(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.tick(4): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.getStateForAge(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.getAgeProperty(0): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.getMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.getBaseSeedId(0): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.canClimbBlock(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.getClimbingState(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.climbRopeAbove(2): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.getBonemealAgeIncrease(1): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.hasGoodCropConditions(2): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.playerDestroy(6): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
        // TomatoBlock.destroyAndPlaceRope(2): not translated, it uses Minecraft's `Level`.
        // TomatoBlock.getGrowthSpeed(3): not translated, its class isn't translated (it extends Minecraft's `CropBlock`).
    }

    /// `TomatoVineBlock` (vectorwing/farmersdelight/common/block/TomatoVineBlock.java)
    pub mod tomato_vine_block {
        use super::*;
        // The class itself isn't translated: it extends `TomatoBlock`. Its static parts are below.
        // TomatoVineBlock.TomatoVineBlock(1): not translated, its class isn't translated (it extends `TomatoBlock`).
        // TomatoVineBlock.TomatoVineBlock(2): not translated, its class isn't translated (it extends `TomatoBlock`).
        // TomatoVineBlock.attemptRopeClimb(3): not translated, its class isn't translated (it extends `TomatoBlock`).
    }

    /// `WallCanvasSignBlock` (vectorwing/farmersdelight/common/block/WallCanvasSignBlock.java)
    pub mod wall_canvas_sign_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `WallSignBlock`. Its static parts are below.
        // WallCanvasSignBlock.WallCanvasSignBlock(2): not translated, its class isn't translated (it extends Minecraft's `WallSignBlock`).
        // WallCanvasSignBlock.getBackgroundColor(0): not translated, its class isn't translated (it extends Minecraft's `WallSignBlock`).
    }

    /// `WallHangingCanvasSignBlock` (vectorwing/farmersdelight/common/block/WallHangingCanvasSignBlock.java)
    pub mod wall_hanging_canvas_sign_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `WallHangingSignBlock`. Its static parts are below.
        // WallHangingCanvasSignBlock.WallHangingCanvasSignBlock(2): not translated, its class isn't translated (it extends Minecraft's `WallHangingSignBlock`).
        // WallHangingCanvasSignBlock.getBackgroundColor(0): not translated, its class isn't translated (it extends Minecraft's `WallHangingSignBlock`).
    }

    /// `WildCropBlock` (vectorwing/farmersdelight/common/block/WildCropBlock.java)
    pub mod wild_crop_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FlowerBlock`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // WildCropBlock.WildCropBlock(3): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
        // WildCropBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
        // WildCropBlock.mayPlaceOn(3): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
        // WildCropBlock.canBeReplaced(2): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
        // WildCropBlock.getFireSpreadSpeed(4): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
        // WildCropBlock.getFlammability(4): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
        // WildCropBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
        // WildCropBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
        // WildCropBlock.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `FlowerBlock`).
    }

    /// `WildRiceBlock` (vectorwing/farmersdelight/common/block/WildRiceBlock.java)
    pub mod wild_rice_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DoublePlantBlock`. Its static parts are below.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // WildRiceBlock.WildRiceBlock(1): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.mayPlaceOn(3): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.canBeReplaced(2): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.canPlaceLiquid(5): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
        // WildRiceBlock.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `DoublePlantBlock`).
    }

    /// `CommonSetup` (vectorwing/farmersdelight/common/CommonSetup.java)
    pub mod common_setup {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonSetup {
        }
        impl CommonSetup {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonSetup.init(0): not translated, it calls CommonSetup.registerDispenserBehaviors, which isn't translated.
        // CommonSetup.registerDispenserBehaviors(0): not translated, it uses Minecraft's `Item`.
        pub fn register_item_set_additions() {
        }
    }

    /// `Configuration` (vectorwing/farmersdelight/common/Configuration.java)
    pub mod configuration {
        use super::*;
        // COMMON_CONFIG: not translated, it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`).
        // CLIENT_CONFIG: not translated, it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`).
        pub const CATEGORY_SETTINGS: &str = "settings";
        // ENABLE_ROPE_REELING: not translated, a function kept in a variable or field (only parameters can be).
        // CANVAS_SIGN_DARK_BACKGROUND_LIST: not translated, a function kept in a variable or field (only parameters can be).
        pub const CATEGORY_FARMING: &str = "farming";
        // RICH_SOIL_BOOST_CHANCE: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_TOMATO_VINE_CLIMBING_TAGGED_ROPES: not translated, a function kept in a variable or field (only parameters can be).
        // DEFAULT_TOMATO_VINE_ROPE: not translated, a function kept in a variable or field (only parameters can be).
        pub const CATEGORY_CRAFTING: &str = "crafting";
        // ENABLE_COOKING_POT_RECIPE_BOOK: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_VANILLA_CROP_CRATES: not translated, a function kept in a variable or field (only parameters can be).
        // CUTTING_BOARD_FORTUNE_BONUS: not translated, a function kept in a variable or field (only parameters can be).
        pub const CATEGORY_OVERRIDES: &str = "overrides";
        // ENABLE_VANILLA_STRAW_BED_RENAME: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_VANILLA_SOUP_EXTRA_EFFECTS: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_RABBIT_STEW_BUFF: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_PUMPKIN_PIE_SNEAK_TO_PLACE: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_DISPENSER_TOOLS_CUTTING_BOARD: not translated, a function kept in a variable or field (only parameters can be).
        pub const CATEGORY_OVERRIDES_STACK_SIZE: &str = "stack_size";
        // ENABLE_STACKABLE_SOUP_ITEMS: not translated, a function kept in a variable or field (only parameters can be).
        // SOUP_ITEM_LIST: not translated, a function kept in a variable or field (only parameters can be).
        pub const CATEGORY_WORLD: &str = "world";
        // GENERATE_FD_CHEST_LOOT: not translated, a function kept in a variable or field (only parameters can be).
        // GENERATE_VILLAGE_COMPOST_HEAPS: not translated, a function kept in a variable or field (only parameters can be).
        // GENERATE_VILLAGE_FARM_FD_CROPS: not translated, a function kept in a variable or field (only parameters can be).
        pub const CATEGORY_DEBUG: &str = "debug";
        // ENABLE_TOMATO_ROPE_PERMANENCE: not translated, a function kept in a variable or field (only parameters can be).
        pub const CATEGORY_CLIENT: &str = "client";
        // ENABLE_NOURISHMENT_HUNGER_OVERLAY: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_COMFORT_HEALTH_OVERLAY: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_FOOD_EFFECT_TOOLTIP: not translated, a function kept in a variable or field (only parameters can be).
        // ENABLE_RECIPE_BOOK_COOKING_POT: not translated, a function kept in a variable or field (only parameters can be).
        // VANILLA_SOUP_EXTRA_EFFECTS: not translated, a function kept in a variable or field (only parameters can be).
        // RABBIT_STEW_BUFF: not translated, a function kept in a variable or field (only parameters can be).
        // DISPENSER_TOOLS_CUTTING_BOARD: not translated, a function kept in a variable or field (only parameters can be).
        // NOURISHMENT_HUNGER_OVERLAY: not translated, a function kept in a variable or field (only parameters can be).
        // COMFORT_HEALTH_OVERLAY: not translated, a function kept in a variable or field (only parameters can be).
        // FOOD_EFFECT_TOOLTIP: not translated, a function kept in a variable or field (only parameters can be).
        #[derive(Clone, Debug)]
        pub struct Configuration {
        }
        impl Configuration {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `VanillaCrateEnabledCondition` (vectorwing/farmersdelight/common/crafting/condition/VanillaCrateEnabledCondition.java)
    pub mod vanilla_crate_enabled_condition {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ResourceCondition`. Its static parts are below.
        // INSTANCE: not translated, it uses VanillaCrateEnabledCondition, which isn't translated (it implements Fabric's `ResourceCondition`).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Fabric's `ResourceConditionType`.
        // VanillaCrateEnabledCondition.init(0): not translated, it uses VanillaCrateEnabledCondition.TYPE, which isn't translated (it uses Fabric's `ResourceConditionType`).
        // VanillaCrateEnabledCondition.getType(0): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
        // VanillaCrateEnabledCondition.test(1): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
    }

    /// `CookingPotBookCategory` (vectorwing/farmersdelight/common/crafting/CookingPotBookCategory.java)
    pub mod cooking_pot_book_category {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // BY_ID: not translated, it uses CookingPotBookCategory, which isn't translated (its constants carry values).
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // CookingPotBookCategory.CookingPotBookCategory(1): not translated, its class isn't translated (its constants carry values).
        // CookingPotBookCategory.findByName(1): not translated, it uses CookingPotBookCategory, which isn't translated (its constants carry values).
        // CookingPotBookCategory.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `CookingPotRecipe` (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java)
    pub mod cooking_pot_recipe {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Recipe`. Its static parts are below.
        pub const INPUT_SLOTS: i32 = 6;
        // CookingPotRecipe.CookingPotRecipe(7): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.input(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.result(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.category(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.container(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.containerOverride(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.assemble(1): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.showNotification(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.group(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.getSerializer(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.getType(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.display(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.placementInfo(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.recipeBookCategory(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.getExperience(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.getCookTime(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.matches(2): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.equals(1): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.hashCode(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
    }

    /// `CookingPotRecipe.Serializer` (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java)
    pub mod cooking_pot_recipe_serializer {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        #[derive(Clone, Debug)]
        pub struct Serializer {
        }
        impl Serializer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CookingPotRecipe.Serializer.codec(0): not translated, it uses Minecraft's `MapCodec`.
        // CookingPotRecipe.Serializer.streamCodec(0): not translated, it uses Minecraft's `StreamCodec`.
        // CookingPotRecipe.Serializer.fromNetwork(1): not translated, it uses CookingPotRecipe, which isn't translated (it implements Minecraft's `Recipe`).
        // CookingPotRecipe.Serializer.toNetwork(2): not translated, it uses Minecraft's `RegistryFriendlyByteBuf`.
    }

    /// `CuttingBoardRecipe` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java)
    pub mod cutting_board_recipe {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Recipe`. Its static parts are below.
        pub const MAX_RESULTS: i32 = 4;
        // CuttingBoardRecipe.CuttingBoardRecipe(5): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.CuttingBoardRecipe(5): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.matches(2): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.assemble(1): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.isSpecial(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.showNotification(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.group(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.getInput(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.getTool(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.getResults(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.getRollableResults(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.rollResults(2): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.getSoundEvent(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.getSoundEventHolder(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.getSerializer(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.getType(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.placementInfo(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.recipeBookCategory(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.equals(1): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.hashCode(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
    }

    /// `CuttingBoardRecipe.Serializer` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java)
    pub mod cutting_board_recipe_serializer {
        use super::*;
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug)]
        pub struct Serializer {
        }
        impl Serializer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CuttingBoardRecipe.Serializer.fromNetwork(1): not translated, it uses CuttingBoardRecipe, which isn't translated (it implements Minecraft's `Recipe`).
        // CuttingBoardRecipe.Serializer.toNetwork(2): not translated, it uses Minecraft's `RegistryFriendlyByteBuf`.
        // CuttingBoardRecipe.Serializer.codec(0): not translated, it uses Minecraft's `MapCodec`.
        // CuttingBoardRecipe.Serializer.streamCodec(0): not translated, it uses Minecraft's `StreamCodec`.
    }

    /// `CuttingBoardRecipeInput` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipeInput.java)
    pub mod cutting_board_recipe_input {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RecipeInput`. Its static parts are below.
        // CuttingBoardRecipeInput.CuttingBoardRecipeInput(2): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // CuttingBoardRecipeInput.getItem(1): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // CuttingBoardRecipeInput.size(0): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // CuttingBoardRecipeInput.item(0): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // CuttingBoardRecipeInput.tool(0): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
    }

    /// `DoughRecipe` (vectorwing/farmersdelight/common/crafting/DoughRecipe.java)
    pub mod dough_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CustomRecipe`. Its static parts are below.
        // INSTANCE: not translated, it uses DoughRecipe, which isn't translated (it extends Minecraft's `CustomRecipe`).
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SERIALIZER: not translated, it uses Minecraft's `RecipeSerializer`.
        // DoughRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // DoughRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // DoughRecipe.getRemainingItems(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // DoughRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
    }

    /// `FoodServingRecipe` (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java)
    pub mod food_serving_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CustomRecipe`. Its static parts are below.
        // INSTANCE: not translated, it uses FoodServingRecipe, which isn't translated (it extends Minecraft's `CustomRecipe`).
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SERIALIZER: not translated, it uses Minecraft's `RecipeSerializer`.
        // FoodServingRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // FoodServingRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // FoodServingRecipe.getRemainingItems(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // FoodServingRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
    }

    /// `ChanceResult` (vectorwing/farmersdelight/common/crafting/ingredient/ChanceResult.java)
    pub mod chance_result {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStackTemplate`. Its static parts are below.
        // EMPTY: not translated, it uses ChanceResult, which isn't translated (it uses Minecraft's `ItemStackTemplate`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ChanceResult.ChanceResult(2): not translated, its class isn't translated (it uses Minecraft's `ItemStackTemplate`).
        // ChanceResult.rollOutput(2): not translated, its class isn't translated (it uses Minecraft's `ItemStackTemplate`).
        // ChanceResult.stack(0): not translated, its class isn't translated (it uses Minecraft's `ItemStackTemplate`).
        // ChanceResult.chance(0): not translated, its class isn't translated (it uses Minecraft's `ItemStackTemplate`).
    }

    /// `ItemAbilityIngredient` (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java)
    pub mod item_ability_ingredient {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `CustomIngredient`. Its static parts are below.
        // SERIALIZER: not translated, it uses Serializer, which isn't translated (it implements Fabric's `CustomIngredientSerializer`).
        pub static SERIALIZER_ID: LazyLock<JIdentifier> = LazyLock::new(|| farmers_delight::id("item_ability"));
        // ItemAbilityIngredient.ItemAbilityIngredient(1): not translated, its class isn't translated (it implements Fabric's `CustomIngredient`).
        // ItemAbilityIngredient.touch(0): not translated, it uses ItemAbilityIngredient.SERIALIZER, which isn't translated (it uses Serializer, which isn't translated (it implements Fabric's `CustomIngredientSerializer`)).
        // ItemAbilityIngredient.dissolve(0): not translated, its class isn't translated (it implements Fabric's `CustomIngredient`).
        // ItemAbilityIngredient.test(1): not translated, its class isn't translated (it implements Fabric's `CustomIngredient`).
        // ItemAbilityIngredient.items(0): not translated, its class isn't translated (it implements Fabric's `CustomIngredient`).
        // ItemAbilityIngredient.getItemAbility(0): not translated, its class isn't translated (it implements Fabric's `CustomIngredient`).
        // ItemAbilityIngredient.requiresTesting(0): not translated, its class isn't translated (it implements Fabric's `CustomIngredient`).
        // ItemAbilityIngredient.getSerializer(0): not translated, its class isn't translated (it implements Fabric's `CustomIngredient`).
    }

    /// `ItemAbilityIngredient.Serializer` (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java)
    pub mod item_ability_ingredient_serializer {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `CustomIngredientSerializer`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ItemAbilityIngredient.Serializer.getIdentifier(0): not translated, its class isn't translated (it implements Fabric's `CustomIngredientSerializer`).
        // ItemAbilityIngredient.Serializer.getCodec(0): not translated, its class isn't translated (it implements Fabric's `CustomIngredientSerializer`).
        // ItemAbilityIngredient.Serializer.getStreamCodec(0): not translated, its class isn't translated (it implements Fabric's `CustomIngredientSerializer`).
    }

    /// `ComfortEffect` (vectorwing/farmersdelight/common/effect/ComfortEffect.java)
    pub mod comfort_effect {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobEffect`. Its static parts are below.
        // ComfortEffect.ComfortEffect(0): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // ComfortEffect.applyEffectTick(3): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // ComfortEffect.shouldApplyEffectTickThisTick(2): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
    }

    /// `NourishmentEffect` (vectorwing/farmersdelight/common/effect/NourishmentEffect.java)
    pub mod nourishment_effect {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobEffect`. Its static parts are below.
        // NourishmentEffect.NourishmentEffect(0): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // NourishmentEffect.applyEffectTick(3): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // NourishmentEffect.shouldApplyEffectTickThisTick(2): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
    }

    /// `RottenTomatoEntity` (vectorwing/farmersdelight/common/entity/RottenTomatoEntity.java)
    pub mod rotten_tomato_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ThrowableItemProjectile`. Its static parts are below.
        // RottenTomatoEntity.RottenTomatoEntity(2): not translated, its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`).
        // RottenTomatoEntity.RottenTomatoEntity(2): not translated, its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`).
        // RottenTomatoEntity.RottenTomatoEntity(4): not translated, its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`).
        // RottenTomatoEntity.getDefaultItem(0): not translated, its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`).
        // RottenTomatoEntity.handleEntityEvent(1): not translated, its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`).
        // RottenTomatoEntity.onHitEntity(1): not translated, its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`).
        // RottenTomatoEntity.onHit(1): not translated, its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`).
    }

    /// `EnumParameters` (vectorwing/farmersdelight/common/EnumParameters.java)
    pub mod enum_parameters {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EnumParameters {
        }
        impl EnumParameters {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `CommonEvents` (vectorwing/farmersdelight/common/event/CommonEvents.java)
    pub mod common_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonEvents {
        }
        impl CommonEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonEvents.onItemUseFinished(3): not translated, it uses Minecraft's `Level`.
        // CommonEvents.handleVanillaSoupEffects(3): not translated, it uses Minecraft's `Level`.
    }

    /// `CommonModBusEvents` (vectorwing/farmersdelight/common/event/CommonModBusEvents.java)
    pub mod common_mod_bus_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonModBusEvents {
        }
        impl CommonModBusEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonModBusEvents.init(0): not translated, it uses Fabric's `DefaultItemComponentEvents`.MODIFY.
        // CommonModBusEvents.onModifyDefaultComponents(1): not translated, it uses Fabric's `ModifyContext`.
        // CommonModBusEvents.addVanillaSoupEffect(3): not translated, it uses Minecraft's `Consumable`.
    }

    /// `FoodValues` (vectorwing/farmersdelight/common/FoodValues.java)
    pub mod food_values {
        use super::*;
        pub const BRIEF_DURATION: i32 = 600;
        pub const SHORT_DURATION: i32 = 1200;
        pub const MEDIUM_DURATION: i32 = 3600;
        pub const LONG_DURATION: i32 = 6000;
        // CABBAGE: not translated, it uses Minecraft's `FoodProperties`.
        // TOMATO: not translated, it uses Minecraft's `FoodProperties`.
        // ONION: not translated, it uses Minecraft's `FoodProperties`.
        // APPLE_CIDER: not translated, it uses Minecraft's `FoodProperties`.
        // FRIED_EGG: not translated, it uses Minecraft's `FoodProperties`.
        // TOMATO_SAUCE: not translated, it uses Minecraft's `FoodProperties`.
        // WHEAT_DOUGH: not translated, it uses Minecraft's `FoodProperties`.
        // RAW_PASTA: not translated, it uses Minecraft's `FoodProperties`.
        // PIE_CRUST: not translated, it uses Minecraft's `FoodProperties`.
        // PUMPKIN_SLICE: not translated, it uses Minecraft's `FoodProperties`.
        // CABBAGE_LEAF: not translated, it uses Minecraft's `FoodProperties`.
        // MINCED_BEEF: not translated, it uses Minecraft's `FoodProperties`.
        // BEEF_PATTY: not translated, it uses Minecraft's `FoodProperties`.
        // CHICKEN_CUTS: not translated, it uses Minecraft's `FoodProperties`.
        // COOKED_CHICKEN_CUTS: not translated, it uses Minecraft's `FoodProperties`.
        // BACON: not translated, it uses Minecraft's `FoodProperties`.
        // COOKED_BACON: not translated, it uses Minecraft's `FoodProperties`.
        // COD_SLICE: not translated, it uses Minecraft's `FoodProperties`.
        // COOKED_COD_SLICE: not translated, it uses Minecraft's `FoodProperties`.
        // SALMON_SLICE: not translated, it uses Minecraft's `FoodProperties`.
        // COOKED_SALMON_SLICE: not translated, it uses Minecraft's `FoodProperties`.
        // MUTTON_CHOPS: not translated, it uses Minecraft's `FoodProperties`.
        // COOKED_MUTTON_CHOPS: not translated, it uses Minecraft's `FoodProperties`.
        // HAM: not translated, it uses Minecraft's `FoodProperties`.
        // SMOKED_HAM: not translated, it uses Minecraft's `FoodProperties`.
        // POPSICLE: not translated, it uses Minecraft's `FoodProperties`.
        // COOKIES: not translated, it uses Minecraft's `FoodProperties`.
        // CAKE_SLICE: not translated, it uses Minecraft's `FoodProperties`.
        // PIE_SLICE: not translated, it uses Minecraft's `FoodProperties`.
        // FRUIT_SALAD: not translated, it uses Minecraft's `FoodProperties`.
        // GLOW_BERRY_CUSTARD: not translated, it uses Minecraft's `FoodProperties`.
        // MIXED_SALAD: not translated, it uses Minecraft's `FoodProperties`.
        // NETHER_SALAD: not translated, it uses Minecraft's `FoodProperties`.
        // BARBECUE_STICK: not translated, it uses Minecraft's `FoodProperties`.
        // EGG_SANDWICH: not translated, it uses Minecraft's `FoodProperties`.
        // CHICKEN_SANDWICH: not translated, it uses Minecraft's `FoodProperties`.
        // HAMBURGER: not translated, it uses Minecraft's `FoodProperties`.
        // BACON_SANDWICH: not translated, it uses Minecraft's `FoodProperties`.
        // MUTTON_WRAP: not translated, it uses Minecraft's `FoodProperties`.
        // DUMPLINGS: not translated, it uses Minecraft's `FoodProperties`.
        // STUFFED_POTATO: not translated, it uses Minecraft's `FoodProperties`.
        // CABBAGE_ROLLS: not translated, it uses Minecraft's `FoodProperties`.
        // SALMON_ROLL: not translated, it uses Minecraft's `FoodProperties`.
        // COD_ROLL: not translated, it uses Minecraft's `FoodProperties`.
        // KELP_ROLL: not translated, it uses Minecraft's `FoodProperties`.
        // KELP_ROLL_SLICE: not translated, it uses Minecraft's `FoodProperties`.
        // COOKED_RICE: not translated, it uses Minecraft's `FoodProperties`.
        // BONE_BROTH: not translated, it uses Minecraft's `FoodProperties`.
        // BEEF_STEW: not translated, it uses Minecraft's `FoodProperties`.
        // VEGETABLE_SOUP: not translated, it uses Minecraft's `FoodProperties`.
        // FISH_STEW: not translated, it uses Minecraft's `FoodProperties`.
        // CHICKEN_SOUP: not translated, it uses Minecraft's `FoodProperties`.
        // FRIED_RICE: not translated, it uses Minecraft's `FoodProperties`.
        // PUMPKIN_SOUP: not translated, it uses Minecraft's `FoodProperties`.
        // BAKED_COD_STEW: not translated, it uses Minecraft's `FoodProperties`.
        // NOODLE_SOUP: not translated, it uses Minecraft's `FoodProperties`.
        // BACON_AND_EGGS: not translated, it uses Minecraft's `FoodProperties`.
        // RATATOUILLE: not translated, it uses Minecraft's `FoodProperties`.
        // STEAK_AND_POTATOES: not translated, it uses Minecraft's `FoodProperties`.
        // PASTA_WITH_MEATBALLS: not translated, it uses Minecraft's `FoodProperties`.
        // PASTA_WITH_MUTTON_CHOP: not translated, it uses Minecraft's `FoodProperties`.
        // MUSHROOM_RICE: not translated, it uses Minecraft's `FoodProperties`.
        // ROASTED_MUTTON_CHOPS: not translated, it uses Minecraft's `FoodProperties`.
        // VEGETABLE_NOODLES: not translated, it uses Minecraft's `FoodProperties`.
        // SQUID_INK_PASTA: not translated, it uses Minecraft's `FoodProperties`.
        // GRILLED_SALMON: not translated, it uses Minecraft's `FoodProperties`.
        // ROAST_CHICKEN: not translated, it uses Minecraft's `FoodProperties`.
        // STUFFED_PUMPKIN: not translated, it uses Minecraft's `FoodProperties`.
        // HONEY_GLAZED_HAM: not translated, it uses Minecraft's `FoodProperties`.
        // SHEPHERDS_PIE: not translated, it uses Minecraft's `FoodProperties`.
        // GLEAMING_SALAD: not translated, it uses Minecraft's `FoodProperties`.
        // DOG_FOOD: not translated, it uses Minecraft's `FoodProperties`.
        // RABBIT_STEW_BUFF: not translated, it uses Minecraft's `FoodProperties`.
        #[derive(Clone, Debug)]
        pub struct FoodValues {
        }
        impl FoodValues {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FoodValues.nourishment(1): not translated, it uses Minecraft's `MobEffectInstance`.
    }

    /// `FoodValues.ConsumableValues` (vectorwing/farmersdelight/common/FoodValues.java)
    pub mod food_values_consumable_values {
        use super::*;
        // FAST_FOOD: not translated, it uses Minecraft's `Consumable`.
        // HUNGER_INDUCING_FOOD: not translated, it uses Minecraft's `Consumable`.
        // NOURISHMENT_BRIEF_DURATION: not translated, it uses Minecraft's `Consumable`.
        // NOURISHMENT_SHORT_DURATION: not translated, it uses Minecraft's `Consumable`.
        // NOURISHMENT_MEDIUM_DURATION: not translated, it uses Minecraft's `Consumable`.
        // NOURISHMENT_LONG_DURATION: not translated, it uses Minecraft's `Consumable`.
        // BONE_BROTH: not translated, it uses Minecraft's `Consumable`.
        // HOT_COCOA: not translated, it uses Minecraft's `Consumable`.
        // MILK_BOTTLE: not translated, it uses Minecraft's `Consumable`.
        // MELON_JUICE: not translated, it uses Minecraft's `Consumable`.
        // MELON_POPSICLE: not translated, it uses Minecraft's `Consumable`.
        // APPLE_CIDER: not translated, it uses Minecraft's `Consumable`.
        // CAKE_SLICE: not translated, it uses Minecraft's `Consumable`.
        // PIE_SLICE: not translated, it uses Minecraft's `Consumable`.
        // CHICKEN_CUTS: not translated, it uses Minecraft's `Consumable`.
        // GLOW_BERRY_CUSTARD: not translated, it uses Minecraft's `Consumable`.
        // FRUIT_SALAD: not translated, it uses Minecraft's `Consumable`.
        // NETHER_SALAD: not translated, it uses Minecraft's `Consumable`.
        // MIXED_SALAD: not translated, it uses Minecraft's `Consumable`.
        // KELP_ROLL: not translated, it uses Minecraft's `Consumable`.
        // VANILLA_SOUP_EFFECTS: not translated, it uses Minecraft's `Item`.
        #[derive(Clone, Debug)]
        pub struct ConsumableValues {
        }
        impl ConsumableValues {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ExtinguishConsumeEffect` (vectorwing/farmersdelight/common/item/component/consumable/ExtinguishConsumeEffect.java)
    pub mod extinguish_consume_effect {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ConsumeEffect`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ExtinguishConsumeEffect.ExtinguishConsumeEffect(0): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // ExtinguishConsumeEffect.getType(0): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // ExtinguishConsumeEffect.apply(3): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
    }

    /// `HealConsumeEffect` (vectorwing/farmersdelight/common/item/component/consumable/HealConsumeEffect.java)
    pub mod heal_consume_effect {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ConsumeEffect`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // HealConsumeEffect.HealConsumeEffect(1): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // HealConsumeEffect.getType(0): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // HealConsumeEffect.apply(3): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // HealConsumeEffect.amount(0): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
    }

    /// `RemoveRandomStatusEffectsConsumeEffect` (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java)
    pub mod remove_random_status_effects_consume_effect {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ConsumeEffect`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // RemoveRandomStatusEffectsConsumeEffect.RemoveRandomStatusEffectsConsumeEffect(2): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // RemoveRandomStatusEffectsConsumeEffect.RemoveRandomStatusEffectsConsumeEffect(1): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // RemoveRandomStatusEffectsConsumeEffect.getType(0): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // RemoveRandomStatusEffectsConsumeEffect.apply(3): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // RemoveRandomStatusEffectsConsumeEffect.excluded(0): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
        // RemoveRandomStatusEffectsConsumeEffect.harmfulOnly(0): not translated, its class isn't translated (it implements Minecraft's `ConsumeEffect`).
    }

    /// `ItemStackWrapper` (vectorwing/farmersdelight/common/item/component/ItemStackWrapper.java)
    pub mod item_stack_wrapper {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // EMPTY: not translated, it uses ItemStackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ItemStackWrapper.ItemStackWrapper(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // ItemStackWrapper.getStack(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // ItemStackWrapper.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // ItemStackWrapper.equals(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `ConsumableItem` (vectorwing/farmersdelight/common/item/ConsumableItem.java)
    pub mod consumable_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // ConsumableItem.ConsumableItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ConsumableItem.ConsumableItem(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ConsumableItem.ConsumableItem(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ConsumableItem.finishUsingItem(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ConsumableItem.affectConsumer(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ConsumableItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `CookingPotItem` (vectorwing/farmersdelight/common/item/CookingPotItem.java)
    pub mod cooking_pot_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // BAR_COLOR: not translated, it calls Minecraft's `ARGB`.colorFromFloat.
        // CookingPotItem.CookingPotItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // CookingPotItem.isBarVisible(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // CookingPotItem.getBarWidth(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // CookingPotItem.getBarColor(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // CookingPotItem.getTooltipImage(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // CookingPotItem.getServingCount(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `DogFoodItem` (vectorwing/farmersdelight/common/item/DogFoodItem.java)
    pub mod dog_food_item {
        use super::*;
        // The class itself isn't translated: it extends `ConsumableItem`. Its static parts are below.
        // EFFECTS: not translated, it uses Minecraft's `MobEffectInstance`.
        // DogFoodItem.DogFoodItem(1): not translated, its class isn't translated (it extends `ConsumableItem`).
        // DogFoodItem.init(0): not translated, it uses Fabric's `UseEntityCallback`.EVENT.
        // DogFoodItem.appendHoverText(5): not translated, its class isn't translated (it extends `ConsumableItem`).
        // DogFoodItem.interactLivingEntity(4): not translated, its class isn't translated (it extends `ConsumableItem`).
    }

    /// `DogFoodItem.DogFoodEvent` (vectorwing/farmersdelight/common/item/DogFoodItem.java)
    pub mod dog_food_item_dog_food_event {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DogFoodEvent {
        }
        impl DogFoodEvent {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DogFoodItem.DogFoodEvent.onDogFoodApplied(5): not translated, it uses Minecraft's `InteractionResult`.
    }

    /// `BackstabbingEnchantment` (vectorwing/farmersdelight/common/item/enchantment/BackstabbingEnchantment.java)
    pub mod backstabbing_enchantment {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BackstabbingEnchantment {
        }
        impl BackstabbingEnchantment {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BackstabbingEnchantment.isLookingBehindTarget(2): not translated, it uses Minecraft's `LivingEntity`.
        pub fn get_backstabbing_damage_per_level(amount: f32, level: i32) -> f32 {
            let multiplier: f32 = ((level as f32) * 0.2f32) + 1.2f32;
            return amount * multiplier;
        }
    }

    /// `BackstabbingEnchantment.BackstabbingEvent` (vectorwing/farmersdelight/common/item/enchantment/BackstabbingEnchantment.java)
    pub mod backstabbing_enchantment_backstabbing_event {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BackstabbingEvent {
        }
        impl BackstabbingEvent {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BackstabbingEnchantment.BackstabbingEvent.onKnifeBackstab(3): not translated, it uses Minecraft's `LivingEntity`.
    }

    /// `FuelBlockItem` (vectorwing/farmersdelight/common/item/FuelBlockItem.java)
    pub mod fuel_block_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // FuelBlockItem.FuelBlockItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // FuelBlockItem.FuelBlockItem(3): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // FuelBlockItem.getBurnTime(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `FuelItem` (vectorwing/farmersdelight/common/item/FuelItem.java)
    pub mod fuel_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // FuelItem.FuelItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // FuelItem.FuelItem(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // FuelItem.getBurnTime(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `HorseFeedItem` (vectorwing/farmersdelight/common/item/HorseFeedItem.java)
    pub mod horse_feed_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // EFFECTS: not translated, it uses Minecraft's `MobEffectInstance`.
        // HorseFeedItem.HorseFeedItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HorseFeedItem.init(0): not translated, it uses Fabric's `UseEntityCallback`.EVENT.
        // HorseFeedItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HorseFeedItem.interactLivingEntity(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `HorseFeedItem.HorseFeedEvent` (vectorwing/farmersdelight/common/item/HorseFeedItem.java)
    pub mod horse_feed_item_horse_feed_event {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct HorseFeedEvent {
        }
        impl HorseFeedEvent {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // HorseFeedItem.HorseFeedEvent.onHorseFeedApplied(5): not translated, it uses Minecraft's `InteractionResult`.
    }

    /// `KnifeItem` (vectorwing/farmersdelight/common/item/KnifeItem.java)
    pub mod knife_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // KNIFE_DIG: not translated, it uses ItemAbility, which isn't translated (it implements Minecraft's `StringRepresentable`).
        // KNIFE_HARVEST: not translated, it uses ItemAbility, which isn't translated (it implements Minecraft's `StringRepresentable`).
        // KNIFE_ACTIONS: not translated, it uses ItemAbility, which isn't translated (it implements Minecraft's `StringRepresentable`).
        // KnifeItem.KnifeItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // KnifeItem.init(0): not translated, it uses Fabric's `UseBlockCallback`.EVENT.
        // KnifeItem.createAttributes(3): not translated, it uses Minecraft's `ItemAttributeModifiers`.
        // KnifeItem.canBeEnchantedWith(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // KnifeItem.useOn(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `KnifeItem.KnifeEvents` (vectorwing/farmersdelight/common/item/KnifeItem.java)
    pub mod knife_item_knife_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct KnifeEvents {
        }
        impl KnifeEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // KnifeItem.KnifeEvents.onKnifeKnockback(2): not translated, it uses Minecraft's `LivingEntity`.
        // KnifeItem.KnifeEvents.onCakeInteraction(4): not translated, it uses Minecraft's `InteractionResult`.
    }

    /// `MushroomColonyItem` (vectorwing/farmersdelight/common/item/MushroomColonyItem.java)
    pub mod mushroom_colony_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // MushroomColonyItem.MushroomColonyItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // MushroomColonyItem.getPlacementState(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `PlaceableItem` (vectorwing/farmersdelight/common/item/PlaceableItem.java)
    pub mod placeable_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // PlaceableItem.PlaceableItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // PlaceableItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `RiceItem` (vectorwing/farmersdelight/common/item/RiceItem.java)
    pub mod rice_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // RiceItem.RiceItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // RiceItem.useOn(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `RopeItem` (vectorwing/farmersdelight/common/item/RopeItem.java)
    pub mod rope_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // RopeItem.RopeItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // RopeItem.updatePlacementContext(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // RopeItem.mustSurvive(0): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `RottenTomatoItem` (vectorwing/farmersdelight/common/item/RottenTomatoItem.java)
    pub mod rotten_tomato_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // RottenTomatoItem.RottenTomatoItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // RottenTomatoItem.use(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // RottenTomatoItem.asProjectile(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `SkilletItem` (vectorwing/farmersdelight/common/item/SkilletItem.java)
    pub mod skillet_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        pub const FLIP_TIME: f32 = 12.0f32;
        // SKILLET_MATERIAL: not translated, it uses Minecraft's `ToolMaterial`.
        pub static FD_ATTACK_KNOCKBACK_UUID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "base_attack_knockback"));
        // SkilletItem.SkilletItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.createAttributes(3): not translated, it uses Minecraft's `ItemAttributeModifiers`.
        // SkilletItem.allowComponentsUpdateAnimation(4): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.compareComponents(3): not translated, it uses Minecraft's `ItemStack`.
        // SkilletItem.postHurtEnemy(3): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.isPlayerNearHeatSource(2): not translated, it uses Minecraft's `Player`.
        // SkilletItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.getUseDuration(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.use(3): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.onUseTick(4): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.releaseUsing(4): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.finishUsingItem(3): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.getBarWidth(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.getClientPlayerHack(0): needs the game, so it's in game_side.rs.
        // SkilletItem.getBarColor(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.isBarVisible(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.getCookingRecipe(2): not translated, it uses Minecraft's `RecipeHolder`.
        // SkilletItem.mineBlock(5): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.place(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SkilletItem.canBeEnchantedWith(3): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `SkilletItem.SkilletEvents` (vectorwing/farmersdelight/common/item/SkilletItem.java)
    pub mod skillet_item_skillet_events {
        use super::*;
        pub static ATTACK_POWER: JCell<f32> = JCell::new(0.0f32);
        #[derive(Clone, Debug)]
        pub struct SkilletEvents {
        }
        impl SkilletEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SkilletItem.SkilletEvents.playSkilletAttackSound(2): not translated, it uses Minecraft's `LivingEntity`.
    }

    /// `CopySkilletFunction` (vectorwing/farmersdelight/common/loot/function/CopySkilletFunction.java)
    pub mod copy_skillet_function {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LootItemConditionalFunction`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "copy_skillet"));
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CopySkilletFunction.CopySkilletFunction(1): not translated, its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`).
        // CopySkilletFunction.builder(0): not translated, it uses Minecraft's `Builder`.
        // CopySkilletFunction.run(2): not translated, its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`).
        // CopySkilletFunction.codec(0): not translated, its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`).
    }

    /// `SmokerCookFunction` (vectorwing/farmersdelight/common/loot/function/SmokerCookFunction.java)
    pub mod smoker_cook_function {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LootItemConditionalFunction`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "smoker_cook"));
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SmokerCookFunction.SmokerCookFunction(1): not translated, its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`).
        // SmokerCookFunction.run(2): not translated, its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`).
        // SmokerCookFunction.codec(0): not translated, its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`).
    }

    /// `AddItemModifier` (vectorwing/farmersdelight/common/loot/modifier/AddItemModifier.java)
    pub mod add_item_modifier {
        use super::*;
        // The class itself isn't translated: it extends `LootModifier` (vectorwing.farmersdelight.refabricated). Its static parts are below.
        // AddItemModifier.AddItemModifier(3): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
        // AddItemModifier.doApply(2): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
    }

    /// `FDAddTableLootModifier` (vectorwing/farmersdelight/common/loot/modifier/FDAddTableLootModifier.java)
    pub mod fd_add_table_loot_modifier {
        use super::*;
        // The class itself isn't translated: it extends `LootModifier` (vectorwing.farmersdelight.refabricated). Its static parts are below.
        // FDAddTableLootModifier.FDAddTableLootModifier(2): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
        // FDAddTableLootModifier.doApply(2): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
        // FDAddTableLootModifier.codec(0): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
    }

    /// `PastrySlicingModifier` (vectorwing/farmersdelight/common/loot/modifier/PastrySlicingModifier.java)
    pub mod pastry_slicing_modifier {
        use super::*;
        // The class itself isn't translated: it extends `LootModifier` (vectorwing.farmersdelight.refabricated). Its static parts are below.
        pub const MAX_CAKE_BITES: i32 = 7;
        pub const MAX_PIE_BITES: i32 = 4;
        // PastrySlicingModifier.PastrySlicingModifier(2): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
        // PastrySlicingModifier.doApply(2): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
    }

    /// `ReplaceItemModifier` (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java)
    pub mod replace_item_modifier {
        use super::*;
        // The class itself isn't translated: it extends `LootModifier` (vectorwing.farmersdelight.refabricated). Its static parts are below.
        // ReplaceItemModifier.ReplaceItemModifier(4): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
        // ReplaceItemModifier.doApply(2): not translated, its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)).
    }

    /// `FarmersDelightMixinPlugin` (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java)
    pub mod farmers_delight_mixin_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility). Its static parts are below.
        // FarmersDelightMixinPlugin.onLoad(1): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // FarmersDelightMixinPlugin.getRefMapperConfig(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // FarmersDelightMixinPlugin.shouldApplyMixin(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // FarmersDelightMixinPlugin.acceptTargets(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // FarmersDelightMixinPlugin.getMixins(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // FarmersDelightMixinPlugin.preApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // FarmersDelightMixinPlugin.postApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
    }

    /// `ModNetworking` (vectorwing/farmersdelight/common/network/ModNetworking.java)
    pub mod mod_networking {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModNetworking {
        }
        impl ModNetworking {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModNetworking.init(0): not translated, it calls Fabric's `PayloadTypeRegistry`.clientboundPlay.
        // ModNetworking.initClient(0): not translated, it uses Minecraft's `Type`.
    }

    /// `ModNetworking.ClientPayloadHandler` (vectorwing/farmersdelight/common/network/ModNetworking.java)
    pub mod mod_networking_client_payload_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientPayloadHandler {
        }
        impl ClientPayloadHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModNetworking.ClientPayloadHandler.handleRichSoilBoostParticles(2): not translated, it uses RichSoilBoostParticlesPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `FlipSkilletPayload` (vectorwing/farmersdelight/common/network/payload/FlipSkilletPayload.java)
    pub mod flip_skillet_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| farmers_delight::id("flip_skillet"));
        // INSTANCE: not translated, it uses FlipSkilletPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // FlipSkilletPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FlipSkilletPayload.handle(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `RichSoilBoostParticlesPayload` (vectorwing/farmersdelight/common/network/payload/RichSoilBoostParticlesPayload.java)
    pub mod rich_soil_boost_particles_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "rich_soil_boost_particles"));
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // RichSoilBoostParticlesPayload.RichSoilBoostParticlesPayload(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RichSoilBoostParticlesPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RichSoilBoostParticlesPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `SendNaturalRegenerationValuePayload` (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java)
    pub mod send_natural_regeneration_value_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| farmers_delight::id("send_natural_regeneration_value"));
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SendNaturalRegenerationValuePayload.SendNaturalRegenerationValuePayload(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SendNaturalRegenerationValuePayload.write(2): not translated, it uses Minecraft's `RegistryFriendlyByteBuf`.
        // SendNaturalRegenerationValuePayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SendNaturalRegenerationValuePayload.handle(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SendNaturalRegenerationValuePayload.value(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `SendRecipeBookValuesPayload` (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java)
    pub mod send_recipe_book_values_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| farmers_delight::id("send_recipe_book_values"));
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SendRecipeBookValuesPayload.SendRecipeBookValuesPayload(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SendRecipeBookValuesPayload.SendRecipeBookValuesPayload(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SendRecipeBookValuesPayload.write(2): not translated, it uses Minecraft's `RegistryFriendlyByteBuf`.
        // SendRecipeBookValuesPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SendRecipeBookValuesPayload.handle(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SendRecipeBookValuesPayload.open(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SendRecipeBookValuesPayload.filtering(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ModBlockIds` (vectorwing/farmersdelight/common/references/ModBlockIds.java)
    pub mod mod_block_ids {
        use super::*;
        // CANVAS_WALL_SIGN: not translated, it uses Minecraft's `ResourceKey`.
        // DYED_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `ColorCollection`.
        // HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `ResourceKey`.
        // DYED_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `ColorCollection`.
        // PUMPKIN_PIE: not translated, it uses Minecraft's `ResourceKey`.
        // BUDDING_TOMATO_CROP: not translated, it uses Minecraft's `ResourceKey`.
        // TOMATO_CROP_ON_ROPE: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct ModBlockIds {
        }
        impl ModBlockIds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBlockIds.createSimpleColored(1): not translated, it uses Minecraft's `ColorCollection`.
        // ModBlockIds.key(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `ModBlockItemIds` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java)
    pub mod mod_block_item_ids {
        use super::*;
        // STOVE: not translated, it uses Minecraft's `BlockItemId`.
        // COOKING_POT: not translated, it uses Minecraft's `BlockItemId`.
        // SKILLET: not translated, it uses Minecraft's `BlockItemId`.
        // WOODEN_BASKET: not translated, it uses Minecraft's `BlockItemId`.
        // BAMBOO_BASKET: not translated, it uses Minecraft's `BlockItemId`.
        // CUTTING_BOARD: not translated, it uses Minecraft's `BlockItemId`.
        // CARROT_CRATE: not translated, it uses Minecraft's `BlockItemId`.
        // POTATO_CRATE: not translated, it uses Minecraft's `BlockItemId`.
        // BEETROOT_CRATE: not translated, it uses Minecraft's `BlockItemId`.
        // CABBAGE_CRATE: not translated, it uses Minecraft's `BlockItemId`.
        // TOMATO_CRATE: not translated, it uses Minecraft's `BlockItemId`.
        // ONION_CRATE: not translated, it uses Minecraft's `BlockItemId`.
        // RICE_BALE: not translated, it uses Minecraft's `BlockItemId`.
        // RICE_BAG: not translated, it uses Minecraft's `BlockItemId`.
        // STRAW_BALE: not translated, it uses Minecraft's `BlockItemId`.
        // ROPE: not translated, it uses Minecraft's `BlockItemId`.
        // SAFETY_NET: not translated, it uses Minecraft's `BlockItemId`.
        // ROPE_FENCE: not translated, it uses Minecraft's `BlockItemId`.
        // ROPE_FENCE_GATE: not translated, it uses Minecraft's `BlockItemId`.
        // OAK_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // SPRUCE_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // BIRCH_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // JUNGLE_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // ACACIA_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // DARK_OAK_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // MANGROVE_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // CHERRY_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // BAMBOO_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // PALE_OAK_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // POPLAR_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // CRIMSON_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // WARPED_CABINET: not translated, it uses Minecraft's `BlockItemId`.
        // CANVAS_RUG: not translated, it uses Minecraft's `BlockItemId`.
        // TATAMI: not translated, it uses Minecraft's `BlockItemId`.
        // FULL_TATAMI_MAT: not translated, it uses Minecraft's `BlockItemId`.
        // HALF_TATAMI_MAT: not translated, it uses Minecraft's `BlockItemId`.
        // CANVAS_SIGN: not translated, it uses Minecraft's `BlockItemId`.
        // DYED_CANVAS_SIGN: not translated, it uses Minecraft's `ColorCollection`.
        // HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `BlockItemId`.
        // DYED_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `ColorCollection`.
        // BROWN_MUSHROOM_COLONY: not translated, it uses Minecraft's `BlockItemId`.
        // RED_MUSHROOM_COLONY: not translated, it uses Minecraft's `BlockItemId`.
        // ORGANIC_COMPOST: not translated, it uses Minecraft's `BlockItemId`.
        // RICH_SOIL: not translated, it uses Minecraft's `BlockItemId`.
        // RICH_SOIL_FARMLAND: not translated, it uses Minecraft's `BlockItemId`.
        // APPLE_PIE: not translated, it uses Minecraft's `BlockItemId`.
        // SWEET_BERRY_CHEESECAKE: not translated, it uses Minecraft's `BlockItemId`.
        // CHOCOLATE_PIE: not translated, it uses Minecraft's `BlockItemId`.
        // SANDY_SHRUB: not translated, it uses Minecraft's `BlockItemId`.
        // WILD_CABBAGES: not translated, it uses Minecraft's `BlockItemId`.
        // WILD_ONIONS: not translated, it uses Minecraft's `BlockItemId`.
        // WILD_TOMATOES: not translated, it uses Minecraft's `BlockItemId`.
        // WILD_CARROTS: not translated, it uses Minecraft's `BlockItemId`.
        // WILD_POTATOES: not translated, it uses Minecraft's `BlockItemId`.
        // WILD_BEETROOTS: not translated, it uses Minecraft's `BlockItemId`.
        // WILD_RICE: not translated, it uses Minecraft's `BlockItemId`.
        // CABBAGE_CROP: not translated, it uses Minecraft's `BlockItemId`.
        // ONION_CROP: not translated, it uses Minecraft's `BlockItemId`.
        // TOMATO_CROP: not translated, it uses Minecraft's `BlockItemId`.
        // RICE_CROP: not translated, it uses Minecraft's `BlockItemId`.
        // RICE_CROP_PANICLES: not translated, it uses Minecraft's `BlockItemId`.
        // ROAST_CHICKEN_BLOCK: not translated, it uses Minecraft's `BlockItemId`.
        // STUFFED_PUMPKIN_BLOCK: not translated, it uses Minecraft's `BlockItemId`.
        // HONEY_GLAZED_HAM_BLOCK: not translated, it uses Minecraft's `BlockItemId`.
        // SHEPHERDS_PIE_BLOCK: not translated, it uses Minecraft's `BlockItemId`.
        // GLEAMING_SALAD_BLOCK: not translated, it uses Minecraft's `BlockItemId`.
        // RICE_ROLL_MEDLEY_BLOCK: not translated, it uses Minecraft's `BlockItemId`.
        // STRAW_BED: not translated, it uses Minecraft's `BlockItemId`.
        #[derive(Clone, Debug)]
        pub struct ModBlockItemIds {
        }
        impl ModBlockItemIds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBlockItemIds.createSimpleColored(1): not translated, it uses Minecraft's `ColorCollection`.
        // ModBlockItemIds.id(2): not translated, it uses Minecraft's `BlockItemId`.
        // ModBlockItemIds.id(1): not translated, it uses Minecraft's `BlockItemId`.
    }

    /// `ModEntityTypeIds` (vectorwing/farmersdelight/common/references/ModEntityTypeIds.java)
    pub mod mod_entity_type_ids {
        use super::*;
        // ROTTEN_TOMATO: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct ModEntityTypeIds {
        }
        impl ModEntityTypeIds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModEntityTypeIds.key(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `ModItemIds` (vectorwing/farmersdelight/common/references/ModItemIds.java)
    pub mod mod_item_ids {
        use super::*;
        // FLINT_KNIFE: not translated, it uses Minecraft's `ResourceKey`.
        // COPPER_KNIFE: not translated, it uses Minecraft's `ResourceKey`.
        // IRON_KNIFE: not translated, it uses Minecraft's `ResourceKey`.
        // DIAMOND_KNIFE: not translated, it uses Minecraft's `ResourceKey`.
        // NETHERITE_KNIFE: not translated, it uses Minecraft's `ResourceKey`.
        // GOLDEN_KNIFE: not translated, it uses Minecraft's `ResourceKey`.
        // STRAW: not translated, it uses Minecraft's `ResourceKey`.
        // CANVAS: not translated, it uses Minecraft's `ResourceKey`.
        // TREE_BARK: not translated, it uses Minecraft's `ResourceKey`.
        // CABBAGE: not translated, it uses Minecraft's `ResourceKey`.
        // TOMATO: not translated, it uses Minecraft's `ResourceKey`.
        // ROTTEN_TOMATO: not translated, it uses Minecraft's `ResourceKey`.
        // FRIED_EGG: not translated, it uses Minecraft's `ResourceKey`.
        // MILK_BOTTLE: not translated, it uses Minecraft's `ResourceKey`.
        // HOT_COCOA: not translated, it uses Minecraft's `ResourceKey`.
        // APPLE_CIDER: not translated, it uses Minecraft's `ResourceKey`.
        // MELON_JUICE: not translated, it uses Minecraft's `ResourceKey`.
        // TOMATO_SAUCE: not translated, it uses Minecraft's `ResourceKey`.
        // WHEAT_DOUGH: not translated, it uses Minecraft's `ResourceKey`.
        // RAW_PASTA: not translated, it uses Minecraft's `ResourceKey`.
        // PUMPKIN_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // CABBAGE_LEAF: not translated, it uses Minecraft's `ResourceKey`.
        // MINCED_BEEF: not translated, it uses Minecraft's `ResourceKey`.
        // BEEF_PATTY: not translated, it uses Minecraft's `ResourceKey`.
        // CHICKEN_CUTS: not translated, it uses Minecraft's `ResourceKey`.
        // COOKED_CHICKEN_CUTS: not translated, it uses Minecraft's `ResourceKey`.
        // BACON: not translated, it uses Minecraft's `ResourceKey`.
        // COOKED_BACON: not translated, it uses Minecraft's `ResourceKey`.
        // COD_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // COOKED_COD_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // SALMON_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // COOKED_SALMON_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // MUTTON_CHOPS: not translated, it uses Minecraft's `ResourceKey`.
        // COOKED_MUTTON_CHOPS: not translated, it uses Minecraft's `ResourceKey`.
        // HAM: not translated, it uses Minecraft's `ResourceKey`.
        // SMOKED_HAM: not translated, it uses Minecraft's `ResourceKey`.
        // PIE_CRUST: not translated, it uses Minecraft's `ResourceKey`.
        // CAKE_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // APPLE_PIE_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // SWEET_BERRY_CHEESECAKE_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // CHOCOLATE_PIE_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // PUMPKIN_PIE_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // SWEET_BERRY_COOKIE: not translated, it uses Minecraft's `ResourceKey`.
        // HONEY_COOKIE: not translated, it uses Minecraft's `ResourceKey`.
        // MELON_POPSICLE: not translated, it uses Minecraft's `ResourceKey`.
        // GLOW_BERRY_CUSTARD: not translated, it uses Minecraft's `ResourceKey`.
        // FRUIT_SALAD: not translated, it uses Minecraft's `ResourceKey`.
        // MIXED_SALAD: not translated, it uses Minecraft's `ResourceKey`.
        // NETHER_SALAD: not translated, it uses Minecraft's `ResourceKey`.
        // BARBECUE_STICK: not translated, it uses Minecraft's `ResourceKey`.
        // EGG_SANDWICH: not translated, it uses Minecraft's `ResourceKey`.
        // CHICKEN_SANDWICH: not translated, it uses Minecraft's `ResourceKey`.
        // HAMBURGER: not translated, it uses Minecraft's `ResourceKey`.
        // BACON_SANDWICH: not translated, it uses Minecraft's `ResourceKey`.
        // MUTTON_WRAP: not translated, it uses Minecraft's `ResourceKey`.
        // DUMPLINGS: not translated, it uses Minecraft's `ResourceKey`.
        // STUFFED_POTATO: not translated, it uses Minecraft's `ResourceKey`.
        // CABBAGE_ROLLS: not translated, it uses Minecraft's `ResourceKey`.
        // SALMON_ROLL: not translated, it uses Minecraft's `ResourceKey`.
        // COD_ROLL: not translated, it uses Minecraft's `ResourceKey`.
        // KELP_ROLL: not translated, it uses Minecraft's `ResourceKey`.
        // KELP_ROLL_SLICE: not translated, it uses Minecraft's `ResourceKey`.
        // COOKED_RICE: not translated, it uses Minecraft's `ResourceKey`.
        // BONE_BROTH: not translated, it uses Minecraft's `ResourceKey`.
        // BEEF_STEW: not translated, it uses Minecraft's `ResourceKey`.
        // CHICKEN_SOUP: not translated, it uses Minecraft's `ResourceKey`.
        // VEGETABLE_SOUP: not translated, it uses Minecraft's `ResourceKey`.
        // FISH_STEW: not translated, it uses Minecraft's `ResourceKey`.
        // FRIED_RICE: not translated, it uses Minecraft's `ResourceKey`.
        // PUMPKIN_SOUP: not translated, it uses Minecraft's `ResourceKey`.
        // BAKED_COD_STEW: not translated, it uses Minecraft's `ResourceKey`.
        // NOODLE_SOUP: not translated, it uses Minecraft's `ResourceKey`.
        // ONION_SOUP: not translated, it uses Minecraft's `ResourceKey`.
        // BACON_AND_EGGS: not translated, it uses Minecraft's `ResourceKey`.
        // PASTA_WITH_MEATBALLS: not translated, it uses Minecraft's `ResourceKey`.
        // PASTA_WITH_MUTTON_CHOP: not translated, it uses Minecraft's `ResourceKey`.
        // MUSHROOM_RICE: not translated, it uses Minecraft's `ResourceKey`.
        // ROASTED_MUTTON_CHOPS: not translated, it uses Minecraft's `ResourceKey`.
        // VEGETABLE_NOODLES: not translated, it uses Minecraft's `ResourceKey`.
        // STEAK_AND_POTATOES: not translated, it uses Minecraft's `ResourceKey`.
        // RATATOUILLE: not translated, it uses Minecraft's `ResourceKey`.
        // SQUID_INK_PASTA: not translated, it uses Minecraft's `ResourceKey`.
        // GRILLED_SALMON: not translated, it uses Minecraft's `ResourceKey`.
        // ROAST_CHICKEN: not translated, it uses Minecraft's `ResourceKey`.
        // STUFFED_PUMPKIN: not translated, it uses Minecraft's `ResourceKey`.
        // HONEY_GLAZED_HAM: not translated, it uses Minecraft's `ResourceKey`.
        // SHEPHERDS_PIE: not translated, it uses Minecraft's `ResourceKey`.
        // GLEAMING_SALAD: not translated, it uses Minecraft's `ResourceKey`.
        // DOG_FOOD: not translated, it uses Minecraft's `ResourceKey`.
        // HORSE_FEED: not translated, it uses Minecraft's `ResourceKey`.
        // DEBUG_PUMPKIN_PIE: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct ModItemIds {
        }
        impl ModItemIds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModItemIds.key(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `ModAdvancements` (vectorwing/farmersdelight/common/registry/ModAdvancements.java)
    pub mod mod_advancements {
        use super::*;
        // USE_CUTTING_BOARD: not translated, it uses CuttingBoardTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        #[derive(Clone, Debug)]
        pub struct ModAdvancements {
        }
        impl ModAdvancements {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModBiomeFeatures` (vectorwing/farmersdelight/common/registry/ModBiomeFeatures.java)
    pub mod mod_biome_features {
        use super::*;
        // IN_ORDER: not translated, it uses Minecraft's `MapCodec`.
        // WILD_RICE: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug)]
        pub struct ModBiomeFeatures {
        }
        impl ModBiomeFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModBiomeModifiers` (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java)
    pub mod mod_biome_modifiers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModBiomeModifiers {
        }
        impl ModBiomeModifiers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBiomeModifiers.modFeature(1): not translated, it uses Minecraft's `ResourceKey`.
        // ModBiomeModifiers.init(0): not translated, it creates a FDBiomeSelector, which isn't translated (it implements `Predicate` (java.util.function)).
    }

    /// `ModBiomeModifiers.FDBiomeSelector` (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java)
    pub mod mod_biome_modifiers_fd_biome_selector {
        use super::*;
        // The class itself isn't translated: it implements `Predicate` (java.util.function). Its static parts are below.
        // ModBiomeModifiers.FDBiomeSelector.FDBiomeSelector(4): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
        // ModBiomeModifiers.FDBiomeSelector.FDBiomeSelector(1): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
        // ModBiomeModifiers.FDBiomeSelector.test(1): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
        // ModBiomeModifiers.FDBiomeSelector.minTemperature(0): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
        // ModBiomeModifiers.FDBiomeSelector.maxTemperature(0): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
        // ModBiomeModifiers.FDBiomeSelector.allowed(0): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
        // ModBiomeModifiers.FDBiomeSelector.denied(0): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
    }

    /// `ModBlockEntityTypes` (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java)
    pub mod mod_block_entity_types {
        use super::*;
        // STOVE: not translated, it uses Minecraft's `BlockEntityType`.
        // COOKING_POT: not translated, it uses Minecraft's `BlockEntityType`.
        // BASKET: not translated, it uses Minecraft's `BlockEntityType`.
        // CUTTING_BOARD: not translated, it uses Minecraft's `BlockEntityType`.
        // SKILLET: not translated, it uses Minecraft's `BlockEntityType`.
        // CABINET: not translated, it uses Minecraft's `BlockEntityType`.
        #[derive(Clone, Debug)]
        pub struct ModBlockEntityTypes {
        }
        impl ModBlockEntityTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBlockEntityTypes.addSupportedBlocks(2): not translated, it uses Minecraft's `BlockEntityType`.
        // ModBlockEntityTypes.touch(0): not translated, it calls ModBlockEntityTypes.addSupportedBlocks, which isn't translated.
    }

    /// `ModBlocks` (vectorwing/farmersdelight/common/registry/ModBlocks.java)
    pub mod mod_blocks {
        use super::*;
        // STOVE: not translated, it uses Minecraft's `Block`.
        // COOKING_POT: not translated, it uses Minecraft's `Block`.
        // SKILLET: not translated, it uses Minecraft's `Block`.
        // WOODEN_BASKET: not translated, it uses Minecraft's `Block`.
        // BAMBOO_BASKET: not translated, it uses Minecraft's `Block`.
        // CUTTING_BOARD: not translated, it uses Minecraft's `Block`.
        // BASKET: not translated, it uses Minecraft's `Block`.
        // CARROT_CRATE: not translated, it uses Minecraft's `Block`.
        // POTATO_CRATE: not translated, it uses Minecraft's `Block`.
        // BEETROOT_CRATE: not translated, it uses Minecraft's `Block`.
        // CABBAGE_CRATE: not translated, it uses Minecraft's `Block`.
        // TOMATO_CRATE: not translated, it uses Minecraft's `Block`.
        // ONION_CRATE: not translated, it uses Minecraft's `Block`.
        // RICE_BALE: not translated, it uses Minecraft's `Block`.
        // RICE_BAG: not translated, it uses Minecraft's `Block`.
        // STRAW_BALE: not translated, it uses Minecraft's `Block`.
        // ROPE: not translated, it uses Minecraft's `Block`.
        // SAFETY_NET: not translated, it uses Minecraft's `Block`.
        // ROPE_FENCE: not translated, it uses Minecraft's `Block`.
        // ROPE_FENCE_GATE: not translated, it uses Minecraft's `Block`.
        // OAK_CABINET: not translated, it uses Minecraft's `Block`.
        // SPRUCE_CABINET: not translated, it uses Minecraft's `Block`.
        // BIRCH_CABINET: not translated, it uses Minecraft's `Block`.
        // JUNGLE_CABINET: not translated, it uses Minecraft's `Block`.
        // ACACIA_CABINET: not translated, it uses Minecraft's `Block`.
        // DARK_OAK_CABINET: not translated, it uses Minecraft's `Block`.
        // MANGROVE_CABINET: not translated, it uses Minecraft's `Block`.
        // CHERRY_CABINET: not translated, it uses Minecraft's `Block`.
        // BAMBOO_CABINET: not translated, it uses Minecraft's `Block`.
        // PALE_OAK_CABINET: not translated, it uses Minecraft's `Block`.
        // POPLAR_CABINET: not translated, it uses Minecraft's `Block`.
        // CRIMSON_CABINET: not translated, it uses Minecraft's `Block`.
        // WARPED_CABINET: not translated, it uses Minecraft's `Block`.
        // CANVAS_RUG: not translated, it uses Minecraft's `Block`.
        // TATAMI: not translated, it uses Minecraft's `Block`.
        // FULL_TATAMI_MAT: not translated, it uses Minecraft's `Block`.
        // HALF_TATAMI_MAT: not translated, it uses Minecraft's `Block`.
        // CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // WHITE_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // ORANGE_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // MAGENTA_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // LIGHT_BLUE_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // YELLOW_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // LIME_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // PINK_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // GRAY_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // LIGHT_GRAY_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // CYAN_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // PURPLE_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // BLUE_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // BROWN_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // GREEN_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // RED_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // BLACK_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // WHITE_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // ORANGE_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // MAGENTA_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // LIGHT_BLUE_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // YELLOW_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // LIME_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // PINK_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // GRAY_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // LIGHT_GRAY_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // CYAN_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // PURPLE_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // BLUE_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // BROWN_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // GREEN_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // RED_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // BLACK_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // WHITE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // ORANGE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // MAGENTA_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // LIGHT_BLUE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // YELLOW_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // LIME_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // PINK_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // GRAY_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // LIGHT_GRAY_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // CYAN_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // PURPLE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // BLUE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // BROWN_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // GREEN_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // RED_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // BLACK_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Block`.
        // HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // WHITE_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // ORANGE_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // MAGENTA_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // LIGHT_BLUE_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // YELLOW_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // LIME_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // PINK_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // GRAY_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // LIGHT_GRAY_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // CYAN_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // PURPLE_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // BLUE_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // BROWN_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // GREEN_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // RED_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // BLACK_HANGING_CANVAS_WALL_SIGN: not translated, it uses Minecraft's `Block`.
        // BROWN_MUSHROOM_COLONY: not translated, it uses Minecraft's `Block`.
        // RED_MUSHROOM_COLONY: not translated, it uses Minecraft's `Block`.
        // ORGANIC_COMPOST: not translated, it uses Minecraft's `Block`.
        // RICH_SOIL: not translated, it uses Minecraft's `Block`.
        // RICH_SOIL_FARMLAND: not translated, it uses Minecraft's `Block`.
        // APPLE_PIE: not translated, it uses Minecraft's `Block`.
        // SWEET_BERRY_CHEESECAKE: not translated, it uses Minecraft's `Block`.
        // CHOCOLATE_PIE: not translated, it uses Minecraft's `Block`.
        // PUMPKIN_PIE: not translated, it uses Minecraft's `Block`.
        // SANDY_SHRUB: not translated, it uses Minecraft's `Block`.
        // WILD_CABBAGES: not translated, it uses Minecraft's `Block`.
        // WILD_ONIONS: not translated, it uses Minecraft's `Block`.
        // WILD_TOMATOES: not translated, it uses Minecraft's `Block`.
        // WILD_CARROTS: not translated, it uses Minecraft's `Block`.
        // WILD_POTATOES: not translated, it uses Minecraft's `Block`.
        // WILD_BEETROOTS: not translated, it uses Minecraft's `Block`.
        // WILD_RICE: not translated, it uses Minecraft's `Block`.
        // CABBAGE_CROP: not translated, it uses Minecraft's `Block`.
        // ONION_CROP: not translated, it uses Minecraft's `Block`.
        // BUDDING_TOMATO_CROP: not translated, it uses Minecraft's `Block`.
        // TOMATO_CROP: not translated, it uses Minecraft's `Block`.
        // TOMATO_CROP_ON_ROPE: not translated, it uses Minecraft's `Block`.
        // RICE_CROP: not translated, it uses Minecraft's `Block`.
        // RICE_CROP_PANICLES: not translated, it uses Minecraft's `Block`.
        // ROAST_CHICKEN_BLOCK: not translated, it uses Minecraft's `Block`.
        // STUFFED_PUMPKIN_BLOCK: not translated, it uses Minecraft's `Block`.
        // HONEY_GLAZED_HAM_BLOCK: not translated, it uses Minecraft's `Block`.
        // SHEPHERDS_PIE_BLOCK: not translated, it uses Minecraft's `Block`.
        // GLEAMING_SALAD_BLOCK: not translated, it uses Minecraft's `Block`.
        // RICE_ROLL_MEDLEY_BLOCK: not translated, it uses Minecraft's `Block`.
        // STRAW_BED: not translated, it uses Minecraft's `Block`.
        #[derive(Clone, Debug)]
        pub struct ModBlocks {
        }
        impl ModBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBlocks.litBlockEmission(1): not translated, it uses Minecraft's `BlockState`.
        // ModBlocks.glowingFeastBlockEmission(0): not translated, it uses Minecraft's `BlockState`.
        // ModBlocks.lootTableKey(1): not translated, it uses Minecraft's `ResourceKey`.
        // ModBlocks.regCanvasSign(2): not translated, it uses Minecraft's `Block`.
        // ModBlocks.regWallCanvasSign(2): not translated, it uses Minecraft's `Block`.
        // ModBlocks.regHangingCanvasSign(2): not translated, it uses Minecraft's `Block`.
        // ModBlocks.regWallHangingCanvasSign(2): not translated, it uses Minecraft's `Block`.
        // ModBlocks.regBlock(3): not translated, it uses Minecraft's `Block`.
        // ModBlocks.regBlock(3): not translated, it uses Minecraft's `Block`.
        // ModBlocks.feastProperties(0): not translated, it calls Minecraft's `Properties`.of.
        pub fn touch() {
        }
    }

    /// `ModChestLootTables` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java)
    pub mod mod_chest_loot_tables {
        use super::*;
        // ABANDONED_MINESHAFT: not translated, it uses Minecraft's `ResourceKey`.
        // BASTION_HOGLIN_STABLE: not translated, it uses Minecraft's `ResourceKey`.
        // BASTION_TREASURE: not translated, it uses Minecraft's `ResourceKey`.
        // END_CITY_TREASURE: not translated, it uses Minecraft's `ResourceKey`.
        // PILLAGER_OUTPOST: not translated, it uses Minecraft's `ResourceKey`.
        // RUINED_PORTAL: not translated, it uses Minecraft's `ResourceKey`.
        // SHIPWRECK_SUPPLY: not translated, it uses Minecraft's `ResourceKey`.
        // SIMPLE_DUNGEON: not translated, it uses Minecraft's `ResourceKey`.
        // VILLAGE_BUTCHER: not translated, it uses Minecraft's `ResourceKey`.
        // VILLAGE_DESERT_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        // VILLAGE_PLAINS_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        // VILLAGE_SAVANNA_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        // VILLAGE_SNOWY_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        // VILLAGE_TAIGA_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct ModChestLootTables {
        }
        impl ModChestLootTables {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModChestLootTables.register(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `ModConsumeEffectTypes` (vectorwing/farmersdelight/common/registry/ModConsumeEffectTypes.java)
    pub mod mod_consume_effect_types {
        use super::*;
        // REMOVE_RANDOM_EFFECTS: not translated, it uses Minecraft's `Type`.
        // HEAL: not translated, it uses Minecraft's `Type`.
        // EXTINGUISH: not translated, it uses Minecraft's `Type`.
        #[derive(Clone, Debug)]
        pub struct ModConsumeEffectTypes {
        }
        impl ModConsumeEffectTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModCreativeTabs` (vectorwing/farmersdelight/common/registry/ModCreativeTabs.java)
    pub mod mod_creative_tabs {
        use super::*;
        // TAB_FARMERS_DELIGHT: not translated, it uses Minecraft's `CreativeModeTab`.
        #[derive(Clone, Debug)]
        pub struct ModCreativeTabs {
        }
        impl ModCreativeTabs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModDamageTypes` (vectorwing/farmersdelight/common/registry/ModDamageTypes.java)
    pub mod mod_damage_types {
        use super::*;
        // STOVE_BURN: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct ModDamageTypes {
        }
        impl ModDamageTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModDamageTypes.getSimpleDamageSource(2): not translated, it uses Minecraft's `DamageSource`.
        // ModDamageTypes.bootstrapDamageTypes(1): not translated, it uses Minecraft's `BootstrapContext`.
    }

    /// `ModDataComponents` (vectorwing/farmersdelight/common/registry/ModDataComponents.java)
    pub mod mod_data_components {
        use super::*;
        // MEAL: not translated, it uses Minecraft's `DataComponentType`.
        // CONTAINER: not translated, it uses Minecraft's `DataComponentType`.
        // COOKING_TIME_LENGTH: not translated, it uses Minecraft's `DataComponentType`.
        // SKILLET_INGREDIENT: not translated, it uses Minecraft's `DataComponentType`.
        // SKILLET_FLIP_TIMESTAMP: not translated, it uses Minecraft's `DataComponentType`.
        // SKILLET_FLIPPED: not translated, it uses Minecraft's `DataComponentType`.
        // BACKSTABBING: not translated, it uses Minecraft's `DataComponentType`.
        #[derive(Clone, Debug)]
        pub struct ModDataComponents {
        }
        impl ModDataComponents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModEffects` (vectorwing/farmersdelight/common/registry/ModEffects.java)
    pub mod mod_effects {
        use super::*;
        // NOURISHMENT: not translated, it uses Minecraft's `Holder`.
        // COMFORT: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct ModEffects {
        }
        impl ModEffects {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
        }
        pub fn touch() {
        }
    }

    /// `ModEntityTypes` (vectorwing/farmersdelight/common/registry/ModEntityTypes.java)
    pub mod mod_entity_types {
        use super::*;
        // ROTTEN_TOMATO: not translated, it uses Minecraft's `EntityType`.
        #[derive(Clone, Debug)]
        pub struct ModEntityTypes {
        }
        impl ModEntityTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModIngredientTypes` (vectorwing/farmersdelight/common/registry/ModIngredientTypes.java)
    pub mod mod_ingredient_types {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModIngredientTypes {
        }
        impl ModIngredientTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ModItems` (vectorwing/farmersdelight/common/registry/ModItems.java)
    pub mod mod_items {
        use super::*;
        // CREATIVE_TAB_ITEMS: not translated, it uses Minecraft's `Item`.
        // STOVE: not translated, it uses Minecraft's `Item`.
        // COOKING_POT: not translated, it uses Minecraft's `Item`.
        // SKILLET: not translated, it uses Minecraft's `Item`.
        // CUTTING_BOARD: not translated, it uses Minecraft's `Item`.
        // WOODEN_BASKET: not translated, it uses Minecraft's `Item`.
        // BAMBOO_BASKET: not translated, it uses Minecraft's `Item`.
        // BASKET: not translated, it uses Minecraft's `Item`.
        // CARROT_CRATE: not translated, it uses Minecraft's `Item`.
        // POTATO_CRATE: not translated, it uses Minecraft's `Item`.
        // BEETROOT_CRATE: not translated, it uses Minecraft's `Item`.
        // CABBAGE_CRATE: not translated, it uses Minecraft's `Item`.
        // TOMATO_CRATE: not translated, it uses Minecraft's `Item`.
        // ONION_CRATE: not translated, it uses Minecraft's `Item`.
        // RICE_BALE: not translated, it uses Minecraft's `Item`.
        // RICE_BAG: not translated, it uses Minecraft's `Item`.
        // STRAW_BALE: not translated, it uses Minecraft's `Item`.
        // SAFETY_NET: not translated, it uses Minecraft's `Item`.
        // OAK_CABINET: not translated, it uses Minecraft's `Item`.
        // SPRUCE_CABINET: not translated, it uses Minecraft's `Item`.
        // BIRCH_CABINET: not translated, it uses Minecraft's `Item`.
        // JUNGLE_CABINET: not translated, it uses Minecraft's `Item`.
        // ACACIA_CABINET: not translated, it uses Minecraft's `Item`.
        // DARK_OAK_CABINET: not translated, it uses Minecraft's `Item`.
        // MANGROVE_CABINET: not translated, it uses Minecraft's `Item`.
        // CHERRY_CABINET: not translated, it uses Minecraft's `Item`.
        // BAMBOO_CABINET: not translated, it uses Minecraft's `Item`.
        // PALE_OAK_CABINET: not translated, it uses Minecraft's `Item`.
        // POPLAR_CABINET: not translated, it uses Minecraft's `Item`.
        // CRIMSON_CABINET: not translated, it uses Minecraft's `Item`.
        // WARPED_CABINET: not translated, it uses Minecraft's `Item`.
        // TATAMI: not translated, it uses Minecraft's `Item`.
        // FULL_TATAMI_MAT: not translated, it uses Minecraft's `Item`.
        // HALF_TATAMI_MAT: not translated, it uses Minecraft's `Item`.
        // CANVAS_RUG: not translated, it uses Minecraft's `Item`.
        // ROPE_FENCE: not translated, it uses Minecraft's `Item`.
        // ROPE_FENCE_GATE: not translated, it uses Minecraft's `Item`.
        // ORGANIC_COMPOST: not translated, it uses Minecraft's `Item`.
        // RICH_SOIL: not translated, it uses Minecraft's `Item`.
        // RICH_SOIL_FARMLAND: not translated, it uses Minecraft's `Item`.
        // ROPE: not translated, it uses Minecraft's `Item`.
        // CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // WHITE_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // WHITE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // LIGHT_GRAY_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // LIGHT_GRAY_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // GRAY_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // GRAY_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // BLACK_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // BLACK_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // BROWN_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // BROWN_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // RED_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // RED_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // ORANGE_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // ORANGE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // YELLOW_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // YELLOW_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // LIME_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // LIME_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // GREEN_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // GREEN_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // CYAN_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // CYAN_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // LIGHT_BLUE_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // LIGHT_BLUE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // BLUE_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // BLUE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // PURPLE_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // PURPLE_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // MAGENTA_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // MAGENTA_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // PINK_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // PINK_HANGING_CANVAS_SIGN: not translated, it uses Minecraft's `Item`.
        // FLINT_KNIFE: not translated, it uses Minecraft's `Item`.
        // COPPER_KNIFE: not translated, it uses Minecraft's `Item`.
        // IRON_KNIFE: not translated, it uses Minecraft's `Item`.
        // DIAMOND_KNIFE: not translated, it uses Minecraft's `Item`.
        // NETHERITE_KNIFE: not translated, it uses Minecraft's `Item`.
        // GOLDEN_KNIFE: not translated, it uses Minecraft's `Item`.
        // STRAW: not translated, it uses Minecraft's `Item`.
        // CANVAS: not translated, it uses Minecraft's `Item`.
        // TREE_BARK: not translated, it uses Minecraft's `Item`.
        // SANDY_SHRUB: not translated, it uses Minecraft's `Item`.
        // WILD_CABBAGES: not translated, it uses Minecraft's `Item`.
        // WILD_ONIONS: not translated, it uses Minecraft's `Item`.
        // WILD_TOMATOES: not translated, it uses Minecraft's `Item`.
        // WILD_CARROTS: not translated, it uses Minecraft's `Item`.
        // WILD_POTATOES: not translated, it uses Minecraft's `Item`.
        // WILD_BEETROOTS: not translated, it uses Minecraft's `Item`.
        // WILD_RICE: not translated, it uses Minecraft's `Item`.
        // BROWN_MUSHROOM_COLONY: not translated, it uses Minecraft's `Item`.
        // RED_MUSHROOM_COLONY: not translated, it uses Minecraft's `Item`.
        // CABBAGE: not translated, it uses Minecraft's `Item`.
        // TOMATO: not translated, it uses Minecraft's `Item`.
        // ONION: not translated, it uses Minecraft's `Item`.
        // RICE_PANICLE: not translated, it uses Minecraft's `Item`.
        // RICE: not translated, it uses Minecraft's `Item`.
        // CABBAGE_SEEDS: not translated, it uses Minecraft's `Item`.
        // TOMATO_SEEDS: not translated, its value couldn't be read: an anonymous class.
        // ROTTEN_TOMATO: not translated, it uses Minecraft's `Item`.
        // FRIED_EGG: not translated, it uses Minecraft's `Item`.
        // MILK_BOTTLE: not translated, it uses Minecraft's `Item`.
        // HOT_COCOA: not translated, it uses Minecraft's `Item`.
        // APPLE_CIDER: not translated, it uses Minecraft's `Item`.
        // MELON_JUICE: not translated, it uses Minecraft's `Item`.
        // TOMATO_SAUCE: not translated, it uses Minecraft's `Item`.
        // WHEAT_DOUGH: not translated, it uses Minecraft's `Item`.
        // RAW_PASTA: not translated, it uses Minecraft's `Item`.
        // PUMPKIN_SLICE: not translated, it uses Minecraft's `Item`.
        // CABBAGE_LEAF: not translated, it uses Minecraft's `Item`.
        // MINCED_BEEF: not translated, it uses Minecraft's `Item`.
        // BEEF_PATTY: not translated, it uses Minecraft's `Item`.
        // CHICKEN_CUTS: not translated, it uses Minecraft's `Item`.
        // COOKED_CHICKEN_CUTS: not translated, it uses Minecraft's `Item`.
        // BACON: not translated, it uses Minecraft's `Item`.
        // COOKED_BACON: not translated, it uses Minecraft's `Item`.
        // COD_SLICE: not translated, it uses Minecraft's `Item`.
        // COOKED_COD_SLICE: not translated, it uses Minecraft's `Item`.
        // SALMON_SLICE: not translated, it uses Minecraft's `Item`.
        // COOKED_SALMON_SLICE: not translated, it uses Minecraft's `Item`.
        // MUTTON_CHOPS: not translated, it uses Minecraft's `Item`.
        // COOKED_MUTTON_CHOPS: not translated, it uses Minecraft's `Item`.
        // HAM: not translated, it uses Minecraft's `Item`.
        // SMOKED_HAM: not translated, it uses Minecraft's `Item`.
        // PIE_CRUST: not translated, it uses Minecraft's `Item`.
        // APPLE_PIE: not translated, it uses Minecraft's `Item`.
        // SWEET_BERRY_CHEESECAKE: not translated, it uses Minecraft's `Item`.
        // CHOCOLATE_PIE: not translated, it uses Minecraft's `Item`.
        // CAKE_SLICE: not translated, it uses Minecraft's `Item`.
        // APPLE_PIE_SLICE: not translated, it uses Minecraft's `Item`.
        // SWEET_BERRY_CHEESECAKE_SLICE: not translated, it uses Minecraft's `Item`.
        // CHOCOLATE_PIE_SLICE: not translated, it uses Minecraft's `Item`.
        // PUMPKIN_PIE_SLICE: not translated, it uses Minecraft's `Item`.
        // SWEET_BERRY_COOKIE: not translated, it uses Minecraft's `Item`.
        // HONEY_COOKIE: not translated, it uses Minecraft's `Item`.
        // MELON_POPSICLE: not translated, it uses Minecraft's `Item`.
        // GLOW_BERRY_CUSTARD: not translated, it uses Minecraft's `Item`.
        // FRUIT_SALAD: not translated, it uses Minecraft's `Item`.
        // MIXED_SALAD: not translated, it uses Minecraft's `Item`.
        // NETHER_SALAD: not translated, it uses Minecraft's `Item`.
        // BARBECUE_STICK: not translated, it uses Minecraft's `Item`.
        // EGG_SANDWICH: not translated, it uses Minecraft's `Item`.
        // CHICKEN_SANDWICH: not translated, it uses Minecraft's `Item`.
        // HAMBURGER: not translated, it uses Minecraft's `Item`.
        // BACON_SANDWICH: not translated, it uses Minecraft's `Item`.
        // MUTTON_WRAP: not translated, it uses Minecraft's `Item`.
        // DUMPLINGS: not translated, it uses Minecraft's `Item`.
        // STUFFED_POTATO: not translated, it uses Minecraft's `Item`.
        // CABBAGE_ROLLS: not translated, it uses Minecraft's `Item`.
        // SALMON_ROLL: not translated, it uses Minecraft's `Item`.
        // COD_ROLL: not translated, it uses Minecraft's `Item`.
        // KELP_ROLL: not translated, it uses Minecraft's `Item`.
        // KELP_ROLL_SLICE: not translated, it uses Minecraft's `Item`.
        // COOKED_RICE: not translated, it uses Minecraft's `Item`.
        // BONE_BROTH: not translated, it uses Minecraft's `Item`.
        // BEEF_STEW: not translated, it uses Minecraft's `Item`.
        // CHICKEN_SOUP: not translated, it uses Minecraft's `Item`.
        // VEGETABLE_SOUP: not translated, it uses Minecraft's `Item`.
        // FISH_STEW: not translated, it uses Minecraft's `Item`.
        // FRIED_RICE: not translated, it uses Minecraft's `Item`.
        // PUMPKIN_SOUP: not translated, it uses Minecraft's `Item`.
        // BAKED_COD_STEW: not translated, it uses Minecraft's `Item`.
        // NOODLE_SOUP: not translated, it uses Minecraft's `Item`.
        // ONION_SOUP: not translated, it uses Minecraft's `Item`.
        // BACON_AND_EGGS: not translated, it uses Minecraft's `Item`.
        // PASTA_WITH_MEATBALLS: not translated, it uses Minecraft's `Item`.
        // PASTA_WITH_MUTTON_CHOP: not translated, it uses Minecraft's `Item`.
        // MUSHROOM_RICE: not translated, it uses Minecraft's `Item`.
        // ROASTED_MUTTON_CHOPS: not translated, it uses Minecraft's `Item`.
        // VEGETABLE_NOODLES: not translated, it uses Minecraft's `Item`.
        // STEAK_AND_POTATOES: not translated, it uses Minecraft's `Item`.
        // RATATOUILLE: not translated, it uses Minecraft's `Item`.
        // SQUID_INK_PASTA: not translated, it uses Minecraft's `Item`.
        // GRILLED_SALMON: not translated, it uses Minecraft's `Item`.
        // ROAST_CHICKEN_BLOCK: not translated, it uses Minecraft's `Item`.
        // ROAST_CHICKEN: not translated, it uses Minecraft's `Item`.
        // STUFFED_PUMPKIN_BLOCK: not translated, it uses Minecraft's `Item`.
        // STUFFED_PUMPKIN: not translated, it uses Minecraft's `Item`.
        // HONEY_GLAZED_HAM_BLOCK: not translated, it uses Minecraft's `Item`.
        // HONEY_GLAZED_HAM: not translated, it uses Minecraft's `Item`.
        // SHEPHERDS_PIE_BLOCK: not translated, it uses Minecraft's `Item`.
        // SHEPHERDS_PIE: not translated, it uses Minecraft's `Item`.
        // GLEAMING_SALAD_BLOCK: not translated, it uses Minecraft's `Item`.
        // GLEAMING_SALAD: not translated, it uses Minecraft's `Item`.
        // RICE_ROLL_MEDLEY_BLOCK: not translated, it uses Minecraft's `Item`.
        // DOG_FOOD: not translated, it uses Minecraft's `Item`.
        // HORSE_FEED: not translated, it uses Minecraft's `Item`.
        // STRAW_BED: not translated, it uses Minecraft's `Item`.
        // DEBUG_PUMPKIN_PIE: not translated, its value couldn't be read: an anonymous class.
        #[derive(Clone, Debug)]
        pub struct ModItems {
        }
        impl ModItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModItems.registerWithTab(3): not translated, it uses Minecraft's `Item`.
        // ModItems.registerHidden(3): not translated, it uses Minecraft's `Item`.
        // ModItems.registerFuelWithTab(3): not translated, it uses Minecraft's `Item`.
        // ModItems.registerBlockWithTab(4): not translated, it uses Minecraft's `Item`.
        // ModItems.registerItemNameBlockWithTab(4): not translated, it uses Minecraft's `Item`.
        // ModItems.registerFuelBlockWithTab(4): not translated, it uses Minecraft's `Item`.
        pub fn basic_item() -> JProps {
            return JProps::new();
        }
        // ModItems.knifeItem(1): not translated, it uses Minecraft's `ToolMaterial`.
        // ModItems.foodItem(1): not translated, it uses Minecraft's `FoodProperties`.
        // ModItems.foodItem(2): not translated, it uses Minecraft's `FoodProperties`.
        // ModItems.bowlFoodItem(1): not translated, it uses Minecraft's `FoodProperties`.
        // ModItems.bowlFoodItem(2): not translated, it uses Minecraft's `FoodProperties`.
        // ModItems.drinkItem(0): not translated, it uses Minecraft's `Consumable`.
        // ModItems.drinkItem(1): not translated, it uses Minecraft's `Consumable`.
        pub fn touch() {
        }
    }

    /// `ModLootFunctions` (vectorwing/farmersdelight/common/registry/ModLootFunctions.java)
    pub mod mod_loot_functions {
        use super::*;
        // COPY_SKILLET: not translated, it uses Minecraft's `MapCodec`.
        // SMOKER_COOK: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug)]
        pub struct ModLootFunctions {
        }
        impl ModLootFunctions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModMenuTypes` (vectorwing/farmersdelight/common/registry/ModMenuTypes.java)
    pub mod mod_menu_types {
        use super::*;
        // COOKING_POT: not translated, it uses Minecraft's `MenuType`.
        #[derive(Clone, Debug)]
        pub struct ModMenuTypes {
        }
        impl ModMenuTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModParticleTypes` (vectorwing/farmersdelight/common/registry/ModParticleTypes.java)
    pub mod mod_particle_types {
        use super::*;
        // STAR: not translated, it uses Minecraft's `SimpleParticleType`.
        // STEAM: not translated, it uses Minecraft's `SimpleParticleType`.
        // SPARKLE: not translated, it uses Minecraft's `SimpleParticleType`.
        #[derive(Clone, Debug)]
        pub struct ModParticleTypes {
        }
        impl ModParticleTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModPlacementModifiers` (vectorwing/farmersdelight/common/registry/ModPlacementModifiers.java)
    pub mod mod_placement_modifiers {
        use super::*;
        // BIOME_TAG: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug)]
        pub struct ModPlacementModifiers {
        }
        impl ModPlacementModifiers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModRecipeBookCategories` (vectorwing/farmersdelight/common/registry/ModRecipeBookCategories.java)
    pub mod mod_recipe_book_categories {
        use super::*;
        // COOKING_MEALS: not translated, it uses Minecraft's `RecipeBookCategory`.
        // COOKING_DRINKS: not translated, it uses Minecraft's `RecipeBookCategory`.
        // COOKING_MISC: not translated, it uses Minecraft's `RecipeBookCategory`.
        // CUTTING: not translated, it uses Minecraft's `RecipeBookCategory`.
        #[derive(Clone, Debug)]
        pub struct ModRecipeBookCategories {
        }
        impl ModRecipeBookCategories {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModRecipeDisplays` (vectorwing/farmersdelight/common/registry/ModRecipeDisplays.java)
    pub mod mod_recipe_displays {
        use super::*;
        // COOKING: not translated, it uses Minecraft's `Type`.
        #[derive(Clone, Debug)]
        pub struct ModRecipeDisplays {
        }
        impl ModRecipeDisplays {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModRecipePropertySets` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java)
    pub mod mod_recipe_property_sets {
        use super::*;
        // COOKING_POT_INPUT_ONE: not translated, it uses Minecraft's `ResourceKey`.
        // COOKING_POT_INPUT_TWO: not translated, it uses Minecraft's `ResourceKey`.
        // COOKING_POT_INPUT_THREE: not translated, it uses Minecraft's `ResourceKey`.
        // COOKING_POT_INPUT_FOUR: not translated, it uses Minecraft's `ResourceKey`.
        // COOKING_POT_INPUT_FIVE: not translated, it uses Minecraft's `ResourceKey`.
        // COOKING_POT_INPUT_SIX: not translated, it uses Minecraft's `ResourceKey`.
        // COOKING_POT_CONTAINER: not translated, it uses Minecraft's `ResourceKey`.
        // CUTTING_BOARD_INPUT: not translated, it uses Minecraft's `ResourceKey`.
        // CUTTING_BOARD_TOOL: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct ModRecipePropertySets {
        }
        impl ModRecipePropertySets {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ModRecipeSerializers` (vectorwing/farmersdelight/common/registry/ModRecipeSerializers.java)
    pub mod mod_recipe_serializers {
        use super::*;
        // COOKING: not translated, it uses Minecraft's `RecipeSerializer`.
        // CUTTING: not translated, it uses Minecraft's `RecipeSerializer`.
        // FOOD_SERVING: not translated, it uses Minecraft's `RecipeSerializer`.
        // DOUGH: not translated, it uses Minecraft's `RecipeSerializer`.
        #[derive(Clone, Debug)]
        pub struct ModRecipeSerializers {
        }
        impl ModRecipeSerializers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModRecipeTypes` (vectorwing/farmersdelight/common/registry/ModRecipeTypes.java)
    pub mod mod_recipe_types {
        use super::*;
        // COOKING: not translated, it uses Minecraft's `RecipeType`.
        // CUTTING: not translated, it uses Minecraft's `RecipeType`.
        #[derive(Clone, Debug)]
        pub struct ModRecipeTypes {
        }
        impl ModRecipeTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModRecipeTypes.registerRecipeType(1): not translated, its body couldn't be read: an anonymous class.
        pub fn touch() {
        }
    }

    /// `ModSounds` (vectorwing/farmersdelight/common/registry/ModSounds.java)
    pub mod mod_sounds {
        use super::*;
        // BLOCK_STOVE_CRACKLE: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_COOKING_POT_BOIL: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_COOKING_POT_BOIL_SOUP: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_CUTTING_BOARD_PLACE: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_CUTTING_BOARD_REMOVE: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_CUTTING_BOARD_CARVE: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_CUTTING_BOARD_KNIFE: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_ROPE_FENCE_GATE_OPEN: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_ROPE_FENCE_GATE_CLOSE: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_CABINET_OPEN: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_CABINET_CLOSE: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_SKILLET_SIZZLE: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_SKILLET_ADD_FOOD: not translated, it uses Minecraft's `SoundEvent`.
        // ITEM_SKILLET_ATTACK_STRONG: not translated, it uses Minecraft's `SoundEvent`.
        // ITEM_SKILLET_ATTACK_WEAK: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_TOMATOES_PICK_TOMATOES: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_FOOD_TAKE_PORTION: not translated, it uses Minecraft's `SoundEvent`.
        // BLOCK_FOOD_SLICE: not translated, it uses Minecraft's `SoundEvent`.
        // ENTITY_ROTTEN_TOMATO_THROW: not translated, it uses Minecraft's `SoundEvent`.
        // ENTITY_ROTTEN_TOMATO_HIT: not translated, it uses Minecraft's `SoundEvent`.
        #[derive(Clone, Debug)]
        pub struct ModSounds {
        }
        impl ModSounds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn touch() {
        }
    }

    /// `ModToolMaterials` (vectorwing/farmersdelight/common/registry/ModToolMaterials.java)
    pub mod mod_tool_materials {
        use super::*;
        // FLINT: not translated, it uses Minecraft's `ToolMaterial`.
        #[derive(Clone, Debug)]
        pub struct ModToolMaterials {
        }
        impl ModToolMaterials {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `RegistryAliases` (vectorwing/farmersdelight/common/registry/RegistryAliases.java)
    pub mod registry_aliases {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RegistryAliases {
        }
        impl RegistryAliases {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RegistryAliases.addRegistryAliases(0): not translated, it calls RegistryAliases.addBlockAlias, which isn't translated.
        // RegistryAliases.addBlockEntityAlias(2): not translated, it uses Minecraft's `BuiltInRegistries`.BLOCK_ENTITY_TYPE.
        // RegistryAliases.addBlockAlias(2): not translated, it uses Minecraft's `BuiltInRegistries`.BLOCK.
        // RegistryAliases.addItemAlias(2): not translated, it uses Minecraft's `BuiltInRegistries`.ITEM.
    }

    /// `CommonTags` (vectorwing/farmersdelight/common/tag/CommonTags.java)
    pub mod common_tags {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonTags {
        }
        impl CommonTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonTags.commonBlockTag(1): not translated, it uses Minecraft's `TagKey`.
        // CommonTags.commonItemTag(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `CommonTags.Blocks` (vectorwing/farmersdelight/common/tag/CommonTags.java)
    pub mod common_tags_blocks {
        use super::*;
        // MINEABLE_WITH_KNIFE: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_CARROT: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_POTATO: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_BEETROOT: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_TOMATO: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_ONION: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_RICE: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_RICE_PANICLE: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_STRAW: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct Blocks {
        }
        impl Blocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `CommonTags.Items` (vectorwing/farmersdelight/common/tag/CommonTags.java)
    pub mod common_tags_items {
        use super::*;
        // CROPS_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // CROPS_TOMATO: not translated, it uses Minecraft's `TagKey`.
        // CROPS_ONION: not translated, it uses Minecraft's `TagKey`.
        // CROPS_RICE: not translated, it uses Minecraft's `TagKey`.
        // CROPS_GRAIN: not translated, it uses Minecraft's `TagKey`.
        // FOODS_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // FOODS_TOMATO: not translated, it uses Minecraft's `TagKey`.
        // FOODS_ONION: not translated, it uses Minecraft's `TagKey`.
        // FOODS_LEAFY_GREEN: not translated, it uses Minecraft's `TagKey`.
        // FOODS_DOUGH: not translated, it uses Minecraft's `TagKey`.
        // FOODS_DOUGH_WHEAT: not translated, it uses Minecraft's `TagKey`.
        // FOODS_PASTA: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_BACON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_BEEF: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_CHICKEN: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_PORK: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_MUTTON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_SAFE_RAW_FISH: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_COD: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_SALMON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_BACON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_BEEF: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_CHICKEN: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_PORK: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_MUTTON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_EGG: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_COD: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_SALMON: not translated, it uses Minecraft's `TagKey`.
        // TOOLS_KNIFE: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_CARROT: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_POTATO: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_BEETROOT: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_TOMATO: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_ONION: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_RICE: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_RICE_PANICLE: not translated, it uses Minecraft's `TagKey`.
        // STORAGE_BLOCKS_STRAW: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct Items {
        }
        impl Items {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `CompatibilityTags` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java)
    pub mod compatibility_tags {
        use super::*;
        pub const CREATE: &str = "create";
        // CREATE_BRITTLE: not translated, it uses Minecraft's `TagKey`.
        // CREATE_FAN_TRANSPARENT: not translated, it uses Minecraft's `TagKey`.
        // CREATE_PASSIVE_BOILER_HEATERS: not translated, it uses Minecraft's `TagKey`.
        // CREATE_UPRIGHT_ON_BELT: not translated, it uses Minecraft's `TagKey`.
        pub const CREATE_CA: &str = "createaddition";
        // CREATE_CA_PLANT_FOODS: not translated, it uses Minecraft's `TagKey`.
        // CREATE_CA_PLANTS: not translated, it uses Minecraft's `TagKey`.
        pub const SABLE: &str = "sable";
        // SABLE_SUPER_LIGHT: not translated, it uses Minecraft's `TagKey`.
        // SABLE_LIGHT: not translated, it uses Minecraft's `TagKey`.
        pub const ORIGINS: &str = "origins";
        // ORIGINS_MEAT: not translated, it uses Minecraft's `TagKey`.
        pub const SERENE_SEASONS: &str = "sereneseasons";
        // SERENE_SEASONS_AUTUMN_CROPS_BLOCK: not translated, it uses Minecraft's `TagKey`.
        // SERENE_SEASONS_SPRING_CROPS_BLOCK: not translated, it uses Minecraft's `TagKey`.
        // SERENE_SEASONS_SUMMER_CROPS_BLOCK: not translated, it uses Minecraft's `TagKey`.
        // SERENE_SEASONS_WINTER_CROPS_BLOCK: not translated, it uses Minecraft's `TagKey`.
        // SERENE_SEASONS_UNBREAKABLE_FERTILE_CROPS: not translated, it uses Minecraft's `TagKey`.
        // SERENE_SEASONS_AUTUMN_CROPS: not translated, it uses Minecraft's `TagKey`.
        // SERENE_SEASONS_SPRING_CROPS: not translated, it uses Minecraft's `TagKey`.
        // SERENE_SEASONS_SUMMER_CROPS: not translated, it uses Minecraft's `TagKey`.
        // SERENE_SEASONS_WINTER_CROPS: not translated, it uses Minecraft's `TagKey`.
        pub const TINKERS_CONSTRUCT: &str = "tconstruct";
        // TINKERS_CONSTRUCT_SEEDS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct CompatibilityTags {
        }
        impl CompatibilityTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CompatibilityTags.externalItemTag(2): not translated, it uses Minecraft's `TagKey`.
        // CompatibilityTags.externalBlockTag(2): not translated, it uses Minecraft's `TagKey`.
    }

    /// `ConventionalTags` (vectorwing/farmersdelight/common/tag/ConventionalTags.java)
    pub mod conventional_tags {
        use super::*;
        // MINEABLE_WITH_KNIFE: not translated, it uses Minecraft's `TagKey`.
        // CROPS: not translated, it uses Minecraft's `TagKey`.
        // CROPS_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // CROPS_ONION: not translated, it uses Minecraft's `TagKey`.
        // CROPS_RICE: not translated, it uses Minecraft's `TagKey`.
        // CROPS_TOMATO: not translated, it uses Minecraft's `TagKey`.
        // FOODS_BERRIES: not translated, it uses Minecraft's `TagKey`.
        // FOODS_BREADS: not translated, it uses Minecraft's `TagKey`.
        // FOODS_BREADS_WHEAT: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_FISHES: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_FISHES_COD: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_FISHES_SALMON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_MEATS: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_MEATS_COOKED_BACON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_MEATS_COOKED_BEEF: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_MEATS_COOKED_CHICKEN: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_MEATS_COOKED_PORK: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_MEATS_COOKED_MUTTON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_COOKED_MEATS_COOKED_EGGS: not translated, it uses Minecraft's `TagKey`.
        // FOODS_DOUGHS: not translated, it uses Minecraft's `TagKey`.
        // FOODS_DOUGHS_WHEAT: not translated, it uses Minecraft's `TagKey`.
        // FOODS_PASTAS: not translated, it uses Minecraft's `TagKey`.
        // FOODS_PASTA_RAW_PASTAS: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_FISHES: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_FISHES_COD: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_FISHES_SALMON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_FISHES_TROPICAL: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_MEATS: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_MEATS_RAW_BACON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_MEATS_RAW_BEEF: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_MEATS_RAW_CHICKEN: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_MEATS_RAW_PORK: not translated, it uses Minecraft's `TagKey`.
        // FOODS_RAW_MEATS_RAW_MUTTON: not translated, it uses Minecraft's `TagKey`.
        // FOODS_VEGETABLES: not translated, it uses Minecraft's `TagKey`.
        // FOODS_VEGETABLES_BEETROOT: not translated, it uses Minecraft's `TagKey`.
        // FOODS_VEGETABLES_CARROT: not translated, it uses Minecraft's `TagKey`.
        // FOODS_VEGETABLES_ONION: not translated, it uses Minecraft's `TagKey`.
        // FOODS_VEGETABLES_POTATO: not translated, it uses Minecraft's `TagKey`.
        // FOODS_VEGETABLES_TOMATO: not translated, it uses Minecraft's `TagKey`.
        // GRAINS: not translated, it uses Minecraft's `TagKey`.
        // GRAINS_WHEATS: not translated, it uses Minecraft's `TagKey`.
        // GRAINS_RICES: not translated, it uses Minecraft's `TagKey`.
        // MILKS: not translated, it uses Minecraft's `TagKey`.
        // MILK_BUCKET: not translated, it uses Minecraft's `TagKey`.
        // MILK_BOTTLE: not translated, it uses Minecraft's `TagKey`.
        // SALAD_INGREDIENTS: not translated, it uses Minecraft's `TagKey`.
        // SALAD_INGREDIENTS_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // SEEDS: not translated, it uses Minecraft's `TagKey`.
        // SEEDS_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // SEEDS_RICE: not translated, it uses Minecraft's `TagKey`.
        // SEEDS_TOMATO: not translated, it uses Minecraft's `TagKey`.
        // TOOLS_KNIVES: not translated, it uses Minecraft's `TagKey`.
        // DRINKS_MILK: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct ConventionalTags {
        }
        impl ConventionalTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConventionalTags.cBlockTag(1): not translated, it uses Minecraft's `TagKey`.
        // ConventionalTags.cItemTag(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `ModTags` (vectorwing/farmersdelight/common/tag/ModTags.java)
    pub mod mod_tags {
        use super::*;
        // CAMPFIRE_SIGNAL_SMOKE: not translated, it uses Minecraft's `TagKey`.
        // COMPOST_ACTIVATORS: not translated, it uses Minecraft's `TagKey`.
        // DROPS_CAKE_SLICE: not translated, it uses Minecraft's `TagKey`.
        // HEAT_CONDUCTORS: not translated, it uses Minecraft's `TagKey`.
        // HEAT_SOURCES: not translated, it uses Minecraft's `TagKey`.
        // TRAY_HEAT_SOURCES: not translated, it uses Minecraft's `TagKey`.
        // MUSHROOM_COLONY_GROWABLE_ON: not translated, it uses Minecraft's `TagKey`.
        // MINEABLE_WITH_KNIFE: not translated, it uses Minecraft's `TagKey`.
        // TERRAIN: not translated, it uses Minecraft's `TagKey`.
        // STRAW_BLOCKS: not translated, it uses Minecraft's `TagKey`.
        // WILD_CROPS: not translated, it uses Minecraft's `TagKey`.
        // CABINETS: not translated, it uses Minecraft's `TagKey`.
        // WOODEN_CABINETS: not translated, it uses Minecraft's `TagKey`.
        // MUSHROOM_COLONIES: not translated, it uses Minecraft's `TagKey`.
        // ROPES: not translated, it uses Minecraft's `TagKey`.
        // UNAFFECTED_BY_RICH_SOIL: not translated, it uses Minecraft's `TagKey`.
        // MEALS: not translated, it uses Minecraft's `TagKey`.
        // DRINKS: not translated, it uses Minecraft's `TagKey`.
        // FEASTS: not translated, it uses Minecraft's `TagKey`.
        // WILD_CROPS_ITEM: not translated, it uses Minecraft's `TagKey`.
        // STRAW_HARVESTERS: not translated, it uses Minecraft's `TagKey`.
        // KNIVES: not translated, it uses Minecraft's `TagKey`.
        // CANVAS_SIGNS: not translated, it uses Minecraft's `TagKey`.
        // HANGING_CANVAS_SIGNS: not translated, it uses Minecraft's `TagKey`.
        // WOODEN_CABINET_ITEMS: not translated, it uses Minecraft's `TagKey`.
        // CABINET_ITEMS: not translated, it uses Minecraft's `TagKey`.
        // MUSHROOM_COLONY_ITEMS: not translated, it uses Minecraft's `TagKey`.
        // SERVING_CONTAINERS: not translated, it uses Minecraft's `TagKey`.
        // FLAT_ON_CUTTING_BOARD: not translated, it uses Minecraft's `TagKey`.
        // DOG_FOOD_USERS: not translated, it uses Minecraft's `TagKey`.
        // HORSE_FEED_USERS: not translated, it uses Minecraft's `TagKey`.
        // HORSE_FEED_TEMPTED: not translated, it uses Minecraft's `TagKey`.
        // HAS_BROWN_MUSHROOM_COLONY: not translated, it uses Minecraft's `TagKey`.
        // HAS_RED_MUSHROOM_COLONY: not translated, it uses Minecraft's `TagKey`.
        // HAS_WILD_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // HAS_WILD_BEETROOTS: not translated, it uses Minecraft's `TagKey`.
        // WILD_CARROTS_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_CARROTS_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_ONIONS_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_ONIONS_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_POTATOES_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_POTATOES_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_RICE_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_RICE_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_TOMATOES_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_TOMATOES_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // DROPS_LEATHER: not translated, it uses Minecraft's `TagKey`.
        // HOT_COCOA_IGNORED: not translated, it uses Minecraft's `TagKey`.
        // MILK_BOTTLE_IGNORED: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct ModTags {
        }
        impl ModTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModTags.modItemTag(1): not translated, it uses Minecraft's `TagKey`.
        // ModTags.modBlockTag(1): not translated, it uses Minecraft's `TagKey`.
        // ModTags.modEntityTag(1): not translated, it uses Minecraft's `TagKey`.
        // ModTags.modBiomeTag(1): not translated, it uses Minecraft's `TagKey`.
        // ModTags.modEffectTag(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `ModTags.Blocks` (vectorwing/farmersdelight/common/tag/ModTags.java)
    pub mod mod_tags_blocks {
        use super::*;
        // CAMPFIRE_SIGNAL_SMOKE: not translated, it uses Minecraft's `TagKey`.
        // COMPOST_ACTIVATORS: not translated, it uses Minecraft's `TagKey`.
        // DROPS_CAKE_SLICE: not translated, it uses Minecraft's `TagKey`.
        // HEAT_CONDUCTORS: not translated, it uses Minecraft's `TagKey`.
        // HEAT_SOURCES: not translated, it uses Minecraft's `TagKey`.
        // TRAY_HEAT_SOURCES: not translated, it uses Minecraft's `TagKey`.
        // MUSHROOM_COLONY_GROWABLE_ON: not translated, it uses Minecraft's `TagKey`.
        // PLANTED_FROM_BELOW: not translated, it uses Minecraft's `TagKey`.
        // FEASTS: not translated, it uses Minecraft's `TagKey`.
        // PIES: not translated, it uses Minecraft's `TagKey`.
        // STRAW_BLOCKS: not translated, it uses Minecraft's `TagKey`.
        // TERRAIN: not translated, it uses Minecraft's `TagKey`.
        // UNAFFECTED_BY_RICH_SOIL: not translated, it uses Minecraft's `TagKey`.
        // CABINETS: not translated, it uses Minecraft's `TagKey`.
        // CABINETS_WOODEN: not translated, it uses Minecraft's `TagKey`.
        // MUSHROOM_COLONIES: not translated, it uses Minecraft's `TagKey`.
        // MINEABLE_WITH_KNIFE: not translated, it uses Minecraft's `TagKey`.
        // ROPES: not translated, it uses Minecraft's `TagKey`.
        // WILD_CROPS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct Blocks {
        }
        impl Blocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ModTags.EntityTypes` (vectorwing/farmersdelight/common/tag/ModTags.java)
    pub mod mod_tags_entity_types {
        use super::*;
        // DOG_FOOD_USERS: not translated, it uses Minecraft's `TagKey`.
        // HORSE_FEED_USERS: not translated, it uses Minecraft's `TagKey`.
        // HORSE_FEED_TEMPTED: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct EntityTypes {
        }
        impl EntityTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ModTags.Items` (vectorwing/farmersdelight/common/tag/ModTags.java)
    pub mod mod_tags_items {
        use super::*;
        // KNIFE_ENCHANTABLE: not translated, it uses Minecraft's `TagKey`.
        // SNACKS: not translated, it uses Minecraft's `TagKey`.
        // MEALS: not translated, it uses Minecraft's `TagKey`.
        // DRINKS: not translated, it uses Minecraft's `TagKey`.
        // SWEETS: not translated, it uses Minecraft's `TagKey`.
        // FEASTS: not translated, it uses Minecraft's `TagKey`.
        // PIES: not translated, it uses Minecraft's `TagKey`.
        // FLAT_ON_CUTTING_BOARD: not translated, it uses Minecraft's `TagKey`.
        // SERVING_CONTAINERS: not translated, it uses Minecraft's `TagKey`.
        // STRAW_HARVESTERS: not translated, it uses Minecraft's `TagKey`.
        // CABINETS: not translated, it uses Minecraft's `TagKey`.
        // CABINETS_WOODEN: not translated, it uses Minecraft's `TagKey`.
        // CANVAS_SIGNS: not translated, it uses Minecraft's `TagKey`.
        // HANGING_CANVAS_SIGNS: not translated, it uses Minecraft's `TagKey`.
        // KNIVES: not translated, it uses Minecraft's `TagKey`.
        // MUSHROOM_COLONIES: not translated, it uses Minecraft's `TagKey`.
        // WILD_CROPS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct Items {
        }
        impl Items {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ClientRenderUtils` (vectorwing/farmersdelight/common/utility/ClientRenderUtils.java)
    pub mod client_render_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientRenderUtils {
        }
        impl ClientRenderUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn is_cursor_inside_bounds(icon_x: i32, icon_y: i32, icon_width: i32, icon_height: i32, cursor_x: f64, cursor_y: f64) -> bool {
            return ((((icon_x as f64) <= cursor_x) && (cursor_x < ((icon_x + icon_width) as f64))) && ((icon_y as f64) <= cursor_y)) && (cursor_y < ((icon_y + icon_height) as f64));
        }
        // ClientRenderUtils.getClientPlayerHack(0): needs the game, so it's in game_side.rs.
    }

    /// `ItemUtils` (vectorwing/farmersdelight/common/utility/ItemUtils.java)
    pub mod item_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ItemUtils {
        }
        impl ItemUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ItemUtils.isValidTool(3): not translated, it uses Minecraft's `ItemStack`.
        // ItemUtils.isKnife(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemUtils.dropItems(3): not translated, it uses Minecraft's `Level`.
        // ItemUtils.clearItems(1): not translated, it uses ItemStackHandler, which isn't translated (it extends `FabricWrappedInventory`).
        // ItemUtils.doesInventoryHaveItems(1): not translated, it uses ItemHandler, which isn't translated (it's an interface).
        // ItemUtils.spawnItemEntity(8): not translated, it uses Minecraft's `Level`.
        // ItemUtils.getValidatedEnchantmentLevel(3): not translated, it uses Minecraft's `ResourceKey`.
        // ItemUtils.getEnchantmentLevel(2): not translated, it uses Minecraft's `DataComponentGetter`.
    }

    /// `ListUtils` (vectorwing/farmersdelight/common/utility/ListUtils.java)
    pub mod list_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ListUtils {
        }
        impl ListUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ListUtils.mapArrayIndexSet(3): not translated, it's generic.
    }

    /// `MathUtils` (vectorwing/farmersdelight/common/utility/MathUtils.java)
    pub mod math_utils {
        use super::*;
        // RAND: not translated, it creates `Random` (java.util).
        #[derive(Clone, Debug)]
        pub struct MathUtils {
        }
        impl MathUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MathUtils.calcRedstoneFromItemHandler(1): not translated, it uses ItemStackHandler, which isn't translated (it extends `FabricWrappedInventory`).
    }

    /// `RecipeUtils` (vectorwing/farmersdelight/common/utility/RecipeUtils.java)
    pub mod recipe_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RecipeUtils {
        }
        impl RecipeUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn fd_location(name: &str) -> JIdentifier {
            return JIdentifier::new("farmersdelight", &name);
        }
    }

    /// `ShapeUtils` (vectorwing/farmersdelight/common/utility/ShapeUtils.java)
    pub mod shape_utils {
        use super::*;
        // PLATED_SHAPE_CACHE: not translated, it uses Minecraft's `VoxelShape`.
        // ROTATED_SHAPE_CACHE: not translated, it uses Minecraft's `VoxelShape`.
        #[derive(Clone, Debug)]
        pub struct ShapeUtils {
        }
        impl ShapeUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ShapeUtils.rotateY(2): not translated, it uses Minecraft's `VoxelShape`.
        // ShapeUtils.blockBox(6): not translated, it uses Minecraft's `VoxelShape`.
        // ShapeUtils.getShapesRotatedFromNorth(1): not translated, it uses Minecraft's `Direction`.
        // ShapeUtils.buildPlatedFoodShapes(2): not translated, it uses Minecraft's `VoxelShape`.
        // ShapeUtils.buildRotatedFoodShapes(1): not translated, it uses Minecraft's `VoxelShape`.
        // ShapeUtils.mergeShapes(1): not translated, it uses Minecraft's `VoxelShape`.
    }

    /// `ShapeUtils.RotationAmount` (vectorwing/farmersdelight/common/utility/ShapeUtils.java)
    pub mod shape_utils_rotation_amount {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum RotationAmount {
            Ninety,
            HundredEighty,
            TwoHundredSeventy,
        }
        impl RotationAmount {
            pub const VALUES: [RotationAmount; 3] = [RotationAmount::Ninety, RotationAmount::HundredEighty, RotationAmount::TwoHundredSeventy, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    RotationAmount::Ninety => "NINETY",
                    RotationAmount::HundredEighty => "HUNDRED_EIGHTY",
                    RotationAmount::TwoHundredSeventy => "TWO_HUNDRED_SEVENTY",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NINETY" => RotationAmount::Ninety,
                    "HUNDRED_EIGHTY" => RotationAmount::HundredEighty,
                    "TWO_HUNDRED_SEVENTY" => RotationAmount::TwoHundredSeventy,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `TextUtils` (vectorwing/farmersdelight/common/utility/TextUtils.java)
    pub mod text_utils {
        use super::*;
        // PLACEABLE: not translated, it uses Minecraft's `MutableComponent`.
        // PLACEABLE_SNEAKING: not translated, it uses Minecraft's `MutableComponent`.
        // DEBUG_ITEM: not translated, it uses Minecraft's `MutableComponent`.
        #[derive(Clone, Debug)]
        pub struct TextUtils {
        }
        impl TextUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TextUtils.getTranslation(2): not translated, it uses Minecraft's `MutableComponent`.
        // TextUtils.getTextWithType(3): not translated, it uses Minecraft's `MutableComponent`.
        // TextUtils.block(2): not translated, it uses Minecraft's `MutableComponent`.
        // TextUtils.item(2): not translated, it uses Minecraft's `MutableComponent`.
        // TextUtils.advancement(2): not translated, it uses Minecraft's `MutableComponent`.
        // TextUtils.container(2): not translated, it uses Minecraft's `MutableComponent`.
        // TextUtils.JEI(2): not translated, it uses Minecraft's `MutableComponent`.
        // TextUtils.tooltip(2): not translated, it uses Minecraft's `MutableComponent`.
        // TextUtils.subtitleKey(2): not translated, it uses `Object`.
        // TextUtils.addFoodEffectTooltip(4): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `InOrderFeature` (vectorwing/farmersdelight/common/world/feature/InOrderFeature.java)
    pub mod in_order_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // InOrderFeature.InOrderFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // InOrderFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // InOrderFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // InOrderFeature.features(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `WildRiceFeature` (vectorwing/farmersdelight/common/world/feature/WildRiceFeature.java)
    pub mod wild_rice_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WildRiceFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // WildRiceFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `BiomeTagFilter` (vectorwing/farmersdelight/common/world/filter/BiomeTagFilter.java)
    pub mod biome_tag_filter {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PlacementFilter`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BiomeTagFilter.BiomeTagFilter(1): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
        // BiomeTagFilter.biomeIsInTag(1): not translated, it uses BiomeTagFilter, which isn't translated (it implements Minecraft's `PlacementFilter`).
        // BiomeTagFilter.modify(4): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
        // BiomeTagFilter.shouldPlace(3): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
        // BiomeTagFilter.codec(0): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
    }

    /// `AddFeaturesByFilterBiomeModifier` (vectorwing/farmersdelight/common/world/modifier/AddFeaturesByFilterBiomeModifier.java)
    pub mod add_features_by_filter_biome_modifier {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `HolderSet`. Its static parts are below.
        // AddFeaturesByFilterBiomeModifier.AddFeaturesByFilterBiomeModifier(6): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // AddFeaturesByFilterBiomeModifier.allowedBiomes(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // AddFeaturesByFilterBiomeModifier.deniedBiomes(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // AddFeaturesByFilterBiomeModifier.minimumTemperature(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // AddFeaturesByFilterBiomeModifier.maximumTemperature(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // AddFeaturesByFilterBiomeModifier.features(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // AddFeaturesByFilterBiomeModifier.step(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
    }

    /// `VillageStructures` (vectorwing/farmersdelight/common/world/VillageStructures.java)
    pub mod village_structures {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VillageStructures {
        }
        impl VillageStructures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VillageStructures.init(0): not translated, it uses Fabric's `ServerLifecycleEvents`.SERVER_STARTING.
        // VillageStructures.addNewVillageBuilding(1): not translated, it uses Minecraft's `MinecraftServer`.
        // VillageStructures.addBuildingToPool(5): not translated, it uses Minecraft's `Registry`.
        // VillageStructures.addNewRuleToProcessorList(3): not translated, it uses Minecraft's `StructureProcessor`.
    }

    /// `WildCropGeneration` (vectorwing/farmersdelight/common/world/WildCropGeneration.java)
    pub mod wild_crop_generation {
        use super::*;
        // SANDY_SHRUB: not translated, it uses Minecraft's `ResourceKey`.
        // WILD_CABBAGES: not translated, it uses Minecraft's `ResourceKey`.
        // WILD_ONIONS: not translated, it uses Minecraft's `ResourceKey`.
        // WILD_TOMATOES: not translated, it uses Minecraft's `ResourceKey`.
        // WILD_CARROTS: not translated, it uses Minecraft's `ResourceKey`.
        // WILD_POTATOES: not translated, it uses Minecraft's `ResourceKey`.
        // WILD_BEETROOTS: not translated, it uses Minecraft's `ResourceKey`.
        // WILD_RICE: not translated, it uses Minecraft's `ResourceKey`.
        // BROWN_MUSHROOM_COLONIES: not translated, it uses Minecraft's `ResourceKey`.
        // RED_MUSHROOM_COLONIES: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_SANDY_SHRUB: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_WILD_CABBAGES: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_WILD_ONIONS: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_WILD_TOMATOES: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_WILD_CARROTS: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_WILD_POTATOES: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_WILD_BEETROOTS: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_WILD_RICE: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_BROWN_MUSHROOM_COLONIES: not translated, it uses Minecraft's `ResourceKey`.
        // PATCH_RED_MUSHROOM_COLONIES: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct WildCropGeneration {
        }
        impl WildCropGeneration {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn load() {
        }
        // WildCropGeneration.bootstrapConfiguredFeatures(1): not translated, it uses Minecraft's `BootstrapContext`.
        // WildCropGeneration.bootstrapPlacedFeatures(1): not translated, it uses Minecraft's `BootstrapContext`.
    }

    /// `FDAdvancementGenerator` (vectorwing/farmersdelight/data/advancement/FDAdvancementGenerator.java)
    pub mod fd_advancement_generator {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricAdvancementProvider`. Its static parts are below.
        // FDAdvancementGenerator.FDAdvancementGenerator(2): not translated, its class isn't translated (it extends Fabric's `FabricAdvancementProvider`).
        // FDAdvancementGenerator.generateAdvancement(2): not translated, its class isn't translated (it extends Fabric's `FabricAdvancementProvider`).
        // FDAdvancementGenerator.placedBlock(1): not translated, its class isn't translated (it extends Fabric's `FabricAdvancementProvider`).
        // FDAdvancementGenerator.usedItem(2): not translated, it uses Minecraft's `Criterion`.
        // FDAdvancementGenerator.getAdvancement(7): not translated, it uses Minecraft's `Builder`.
        // FDAdvancementGenerator.getNameId(1): not translated, its class isn't translated (it extends Fabric's `FabricAdvancementProvider`).
    }

    /// `BlockStates` (vectorwing/farmersdelight/data/BlockStates.java)
    pub mod block_states {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricModelProvider`. Its static parts are below.
        pub const DEFAULT_ANGLE_OFFSET: i32 = 180;
        // BlockStates.BlockStates(1): not translated, its class isn't translated (it extends Fabric's `FabricModelProvider`).
    }

    /// `BlockTags` (vectorwing/farmersdelight/data/BlockTags.java)
    pub mod block_tags {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `BlockTagsProvider`. Its static parts are below.
        // BlockTags.BlockTags(2): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.addTags(1): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.registerBlockMineables(0): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.registerMinecraftTags(0): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.registerNeoForgeTags(0): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.registerCommonTags(0): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.registerModTags(0): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.registerRefabricatedTags(0): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.registerCompatibilityTags(0): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
        // BlockTags.tagBuilder(1): not translated, its class isn't translated (it extends Fabric's `BlockTagsProvider`).
    }

    /// `BlockTags.RefabricatedTagBuilder` (vectorwing/farmersdelight/data/BlockTags.java)
    pub mod block_tags_refabricated_tag_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockItemTagAppender`. Its static parts are below.
        // BlockTags.RefabricatedTagBuilder.RefabricatedTagBuilder(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // BlockTags.RefabricatedTagBuilder.addOptionalTag(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // BlockTags.RefabricatedTagBuilder.add(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // BlockTags.RefabricatedTagBuilder.add(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // BlockTags.RefabricatedTagBuilder.addColorKey(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // BlockTags.RefabricatedTagBuilder.addColorId(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // BlockTags.RefabricatedTagBuilder.addOptional(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // BlockTags.RefabricatedTagBuilder.addTags(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // BlockTags.RefabricatedTagBuilder.addTag(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
    }

    /// `CookingPotRecipeBuilder` (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java)
    pub mod cooking_pot_recipe_builder {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RecipeBuilder`. Its static parts are below.
        // CookingPotRecipeBuilder.CookingPotRecipeBuilder(6): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.CookingPotRecipeBuilder(5): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.cookingPotRecipe(5): not translated, it uses CookingPotRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.cookingPotRecipe(6): not translated, it uses CookingPotRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.addIngredient(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.addIngredient(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.addIngredient(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.addIngredient(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.addIngredient(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.group(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.defaultId(0): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.setRecipeBookCategory(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.getResult(0): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.unlockedBy(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.unlockedByItems(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.unlockedByAnyIngredient(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.setNamespace(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.getDefaultRecipeId(1): not translated, it uses Minecraft's `ItemLike`.
        // CookingPotRecipeBuilder.saveToFD(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.save(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.build(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.build(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.save(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CookingPotRecipeBuilder.save(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
    }

    /// `CuttingBoardRecipeBuilder` (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java)
    pub mod cutting_board_recipe_builder {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RecipeBuilder`. Its static parts are below.
        // CuttingBoardRecipeBuilder.CuttingBoardRecipeBuilder(5): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.cuttingRecipe(4): not translated, it uses CuttingBoardRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.cuttingRecipe(5): not translated, it uses CuttingBoardRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.cuttingRecipe(3): not translated, it uses CuttingBoardRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.addResult(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.addResult(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.addResultWithChance(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.addResultWithChance(3): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.addSound(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.addSound(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.unlockedBy(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.setNamespace(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.salvaging(0): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.group(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.defaultId(0): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.getResult(0): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.getDefaultRecipeId(1): not translated, it uses Minecraft's `ItemLike`.
        // CuttingBoardRecipeBuilder.saveToFD(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.save(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.build(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.build(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.save(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // CuttingBoardRecipeBuilder.save(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
    }

    /// `CuttingRecipeFolder` (vectorwing/farmersdelight/data/builder/CuttingRecipeFolder.java)
    pub mod cutting_recipe_folder {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CuttingRecipeFolder.CuttingRecipeFolder(1): not translated, its class isn't translated (its constants carry values).
        // CuttingRecipeFolder.toString(0): not translated, its class isn't translated (its constants carry values).
        // CuttingRecipeFolder.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `DamageTypeTags` (vectorwing/farmersdelight/data/DamageTypeTags.java)
    pub mod damage_type_tags {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricTagsProvider`. Its static parts are below.
        // DamageTypeTags.DamageTypeTags(2): not translated, its class isn't translated (it extends Fabric's `FabricTagsProvider`).
        // DamageTypeTags.addTags(1): not translated, its class isn't translated (it extends Fabric's `FabricTagsProvider`).
    }

    /// `DataGenerators` (vectorwing/farmersdelight/data/DataGenerators.java)
    pub mod data_generators {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `DataGeneratorEntrypoint`. Its static parts are below.
        // DataGenerators.onInitializeDataGenerator(1): not translated, its class isn't translated (it implements Fabric's `DataGeneratorEntrypoint`).
        // DataGenerators.buildRegistry(1): not translated, its class isn't translated (it implements Fabric's `DataGeneratorEntrypoint`).
    }

    /// `DataGenerators.DynamicRegistryProvider` (vectorwing/farmersdelight/data/DataGenerators.java)
    pub mod data_generators_dynamic_registry_provider {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricDynamicRegistryProvider`. Its static parts are below.
        // DataGenerators.DynamicRegistryProvider.DynamicRegistryProvider(2): not translated, its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`).
        // DataGenerators.DynamicRegistryProvider.configure(2): not translated, its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`).
        // DataGenerators.DynamicRegistryProvider.createContext(2): not translated, its body couldn't be read: an anonymous class.
        // DataGenerators.DynamicRegistryProvider.getName(0): not translated, its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`).
    }

    /// `DataMaps` (vectorwing/farmersdelight/data/DataMaps.java)
    pub mod data_maps {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DataMaps {
        }
        impl DataMaps {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EnchantmentTags` (vectorwing/farmersdelight/data/EnchantmentTags.java)
    pub mod enchantment_tags {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricTagsProvider`. Its static parts are below.
        // EnchantmentTags.EnchantmentTags(2): not translated, its class isn't translated (it extends Fabric's `FabricTagsProvider`).
        // EnchantmentTags.addTags(1): not translated, its class isn't translated (it extends Fabric's `FabricTagsProvider`).
    }

    /// `EntityTags` (vectorwing/farmersdelight/data/EntityTags.java)
    pub mod entity_tags {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `EntityTypeTagsProvider`. Its static parts are below.
        // EntityTags.EntityTags(2): not translated, its class isn't translated (it extends Fabric's `EntityTypeTagsProvider`).
        // EntityTags.addTags(1): not translated, its class isn't translated (it extends Fabric's `EntityTypeTagsProvider`).
    }

    /// `ItemModels` (vectorwing/farmersdelight/data/ItemModels.java)
    pub mod item_models {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricModelProvider`. Its static parts are below.
        // ItemModels.ItemModels(1): not translated, its class isn't translated (it extends Fabric's `FabricModelProvider`).
        // ItemModels.generateBlockStateModels(1): not translated, its class isn't translated (it extends Fabric's `FabricModelProvider`).
        // ItemModels.generateItemModels(1): not translated, its class isn't translated (it extends Fabric's `FabricModelProvider`).
        // ItemModels.sign(3): not translated, it uses Minecraft's `BlockModelGenerators`.
        // ItemModels.hangingSign(3): not translated, it uses Minecraft's `BlockModelGenerators`.
    }

    /// `ItemTags` (vectorwing/farmersdelight/data/ItemTags.java)
    pub mod item_tags {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `ItemTagsProvider`. Its static parts are below.
        // ItemTags.ItemTags(3): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
        // ItemTags.addTags(1): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
        // ItemTags.registerMinecraftTags(0): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
        // ItemTags.registerModTags(0): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
        // ItemTags.registerRefabricatedTags(0): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
        // ItemTags.registerConventionalTags(0): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
        // ItemTags.registerCommonTags(0): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
        // ItemTags.registerCompatibilityTags(0): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
        // ItemTags.tagBuilder(1): not translated, its class isn't translated (it extends Fabric's `ItemTagsProvider`).
    }

    /// `ItemTags.RefabricatedTagBuilder` (vectorwing/farmersdelight/data/ItemTags.java)
    pub mod item_tags_refabricated_tag_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockItemTagAppender`. Its static parts are below.
        // ItemTags.RefabricatedTagBuilder.RefabricatedTagBuilder(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // ItemTags.RefabricatedTagBuilder.addOptionalTag(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // ItemTags.RefabricatedTagBuilder.add(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // ItemTags.RefabricatedTagBuilder.add(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // ItemTags.RefabricatedTagBuilder.addColorKey(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // ItemTags.RefabricatedTagBuilder.addColorId(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // ItemTags.RefabricatedTagBuilder.addOptional(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // ItemTags.RefabricatedTagBuilder.addTags(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
        // ItemTags.RefabricatedTagBuilder.addTag(1): not translated, its class isn't translated (it uses Minecraft's `BlockItemTagAppender`).
    }

    /// `FDBlockLoot` (vectorwing/farmersdelight/data/loot/FDBlockLoot.java)
    pub mod fd_block_loot {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricBlockLootSubProvider`. Its static parts are below.
        // FDBlockLoot.FDBlockLoot(2): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.generate(0): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.hasBlockStateProperties(2): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.dropNamedContainer(1): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.mushroomColony(2): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.createCropDrops(4): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.createSeedlessCropDrops(3): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.wildCrop(4): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.wildCropNoSeeds(3): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.platedFood(2): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // FDBlockLoot.platedFoodExtraDrop(3): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
    }

    /// `FDChestLoot` (vectorwing/farmersdelight/data/loot/FDChestLoot.java)
    pub mod fd_chest_loot {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `SimpleFabricLootTableSubProvider`. Its static parts are below.
        // FDChestLoot.FDChestLoot(2): not translated, its class isn't translated (it extends Fabric's `SimpleFabricLootTableSubProvider`).
        // FDChestLoot.generate(1): not translated, its class isn't translated (it extends Fabric's `SimpleFabricLootTableSubProvider`).
        // FDChestLoot.getName(0): not translated, its class isn't translated (it extends Fabric's `SimpleFabricLootTableSubProvider`).
        // FDChestLoot.run(0): not translated, its class isn't translated (it extends Fabric's `SimpleFabricLootTableSubProvider`).
    }

    /// `ModEnchantments` (vectorwing/farmersdelight/data/ModEnchantments.java)
    pub mod mod_enchantments {
        use super::*;
        // BACKSTABBING: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct ModEnchantments {
        }
        impl ModEnchantments {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModEnchantments.bootstrap(1): not translated, it uses Minecraft's `BootstrapContext`.
        // ModEnchantments.register(3): not translated, it uses Minecraft's `BootstrapContext`.
        // ModEnchantments.key(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `Recipes` (vectorwing/farmersdelight/data/Recipes.java)
    pub mod recipes {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricRecipeProvider`. Its static parts are below.
        // recipeContext: not translated, it uses Minecraft's `BootstrapContext`.
        // Recipes.Recipes(2): not translated, its class isn't translated (it extends Fabric's `FabricRecipeProvider`).
        // Recipes.createRecipeProvider(3): not translated, its body couldn't be read: an anonymous class.
        // Recipes.getName(0): not translated, its class isn't translated (it extends Fabric's `FabricRecipeProvider`).
    }

    /// `CookingRecipes` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java)
    pub mod cooking_recipes {
        use super::*;
        pub const FAST_COOKING: i32 = 100;
        pub const NORMAL_COOKING: i32 = 200;
        pub const SLOW_COOKING: i32 = 400;
        pub const SMALL_EXP: f32 = 0.35f32;
        pub const MEDIUM_EXP: f32 = 1.0f32;
        pub const LARGE_EXP: f32 = 2.0f32;
        #[derive(Clone, Debug)]
        pub struct CookingRecipes {
        }
        impl CookingRecipes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CookingRecipes.register(2): not translated, it uses Minecraft's `Provider`.
        // CookingRecipes.cookMiscellaneous(2): not translated, it uses Minecraft's `HolderGetter`.
        // CookingRecipes.cookMinecraftSoups(2): not translated, it uses Minecraft's `HolderGetter`.
        // CookingRecipes.cookMeals(2): not translated, it uses Minecraft's `HolderGetter`.
    }

    /// `CraftingRecipes` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java)
    pub mod crafting_recipes {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CraftingRecipes {
        }
        impl CraftingRecipes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CraftingRecipes.register(3): not translated, it uses Minecraft's `Provider`.
        // CraftingRecipes.canvasSignDyeing(5): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.recipesVanillaAlternatives(2): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.recipesCanvasSigns(2): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.recipesBlocks(3): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.recipesTools(2): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.recipesMaterials(2): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.recipesFoodstuffs(2): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.recipesFoodBlocks(2): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.recipesCraftedMeals(2): not translated, it uses Minecraft's `HolderGetter`.
        // CraftingRecipes.fdLocation(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `CuttingRecipes` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java)
    pub mod cutting_recipes {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CuttingRecipes {
        }
        impl CuttingRecipes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CuttingRecipes.register(2): not translated, it uses Minecraft's `Provider`.
        // CuttingRecipes.cuttingAnimalItems(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.cuttingVegetables(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.cuttingFoods(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.cuttingFlowers(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.salvagingMinerals(1): not translated, it uses Minecraft's `RecipeOutput`.
        // CuttingRecipes.strippingWood(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.salvagingWoodenFurniture(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.diggingSediments(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.salvagingUsingShears(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.salvagingBlockFromVehicle(2): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.salvagePlankFromFurniture(5): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.stripLogForBark(4): not translated, it uses Minecraft's `HolderGetter`.
        // CuttingRecipes.matchesTool(3): not translated, it uses Minecraft's `Ingredient`.
        pub fn salvaging_recipe(name: &str) -> JIdentifier {
            return JIdentifier::new("farmersdelight", &(format!("salvaging/{}", name)));
        }
    }

    /// `SmeltingRecipes` (vectorwing/farmersdelight/data/recipe/SmeltingRecipes.java)
    pub mod smelting_recipes {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SmeltingRecipes {
        }
        impl SmeltingRecipes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SmeltingRecipes.register(2): not translated, it uses Minecraft's `Provider`.
        // SmeltingRecipes.foodSmeltingRecipes(5): not translated, it uses Minecraft's `ItemLike`.
        // SmeltingRecipes.foodSmeltingRecipes(6): not translated, it uses Minecraft's `HolderGetter`.
    }

    /// `SoundDefinitions` (vectorwing/farmersdelight/data/SoundDefinitions.java)
    pub mod sound_definitions {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricSoundsProvider`. Its static parts are below.
        // SoundDefinitions.SoundDefinitions(2): not translated, its class isn't translated (it extends Fabric's `FabricSoundsProvider`).
        // SoundDefinitions.configure(2): not translated, its class isn't translated (it extends Fabric's `FabricSoundsProvider`).
        // SoundDefinitions.generateNewSoundWithSubtitle(4): not translated, its class isn't translated (it extends Fabric's `FabricSoundsProvider`).
        // SoundDefinitions.generateNewSound(5): not translated, its class isn't translated (it extends Fabric's `FabricSoundsProvider`).
        // SoundDefinitions.generateNewSoundCustomSubtitle(5): not translated, its class isn't translated (it extends Fabric's `FabricSoundsProvider`).
        // SoundDefinitions.generateExistingSoundWithSubtitle(3): not translated, its class isn't translated (it extends Fabric's `FabricSoundsProvider`).
        // SoundDefinitions.generateExistingSound(4): not translated, its class isn't translated (it extends Fabric's `FabricSoundsProvider`).
        // SoundDefinitions.getName(0): not translated, its class isn't translated (it extends Fabric's `FabricSoundsProvider`).
    }

    /// `StructureUpdater` (vectorwing/farmersdelight/data/tools/StructureUpdater.java)
    pub mod structure_updater {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DataProvider`. Its static parts are below.
        // StructureUpdater.StructureUpdater(3): not translated, its class isn't translated (it implements Minecraft's `DataProvider`).
        // StructureUpdater.run(1): not translated, its class isn't translated (it implements Minecraft's `DataProvider`).
        // StructureUpdater.process(3): not translated, its class isn't translated (it implements Minecraft's `DataProvider`).
        // StructureUpdater.writeNBTTo(3): not translated, its class isn't translated (it implements Minecraft's `DataProvider`).
        // StructureUpdater.updateNBT(1): not translated, it uses Minecraft's `CompoundTag`.
        // StructureUpdater.getName(0): not translated, its class isn't translated (it implements Minecraft's `DataProvider`).
    }

    /// `VillagerTags` (vectorwing/farmersdelight/data/VillagerTags.java)
    pub mod villager_tags {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricTagsProvider`. Its static parts are below.
        // VillagerTags.VillagerTags(2): not translated, its class isn't translated (it extends Fabric's `FabricTagsProvider`).
        // VillagerTags.addTags(1): not translated, its class isn't translated (it extends Fabric's `FabricTagsProvider`).
    }

    /// `VillagerTrades` (vectorwing/farmersdelight/data/VillagerTrades.java)
    pub mod villager_trades {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricDynamicRegistryProvider`. Its static parts are below.
        // FARMER_1_ONION_EMERALD: not translated, it uses Minecraft's `ResourceKey`.
        // FARMER_1_TOMATO_EMERALD: not translated, it uses Minecraft's `ResourceKey`.
        // FARMER_2_CABBAGE_EMERALD: not translated, it uses Minecraft's `ResourceKey`.
        // FARMER_2_RICE_EMERALD: not translated, it uses Minecraft's `ResourceKey`.
        // WANDERING_TRADER_CABBAGE_EMERALD: not translated, it uses Minecraft's `ResourceKey`.
        // WANDERING_TRADER_TOMATO_EMERALD: not translated, it uses Minecraft's `ResourceKey`.
        // WANDERING_TRADER_RICE_EMERALD: not translated, it uses Minecraft's `ResourceKey`.
        // WANDERING_TRADER_ONION_EMERALD: not translated, it uses Minecraft's `ResourceKey`.
        // VillagerTrades.VillagerTrades(2): not translated, its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`).
        // VillagerTrades.init(1): not translated, it uses Minecraft's `BootstrapContext`.
        // VillagerTrades.onVillagerTrades(1): not translated, it uses Minecraft's `BootstrapContext`.
        // VillagerTrades.onWandererTrades(1): not translated, it uses Minecraft's `BootstrapContext`.
        // VillagerTrades.emeraldsForItemsTrade(4): not translated, it uses Minecraft's `VillagerTrade`.
        // VillagerTrades.itemForEmeraldTrade(3): not translated, it uses Minecraft's `VillagerTrade`.
        // VillagerTrades.resourceKey(1): not translated, it uses Minecraft's `ResourceKey`.
        // VillagerTrades.configure(2): not translated, its body couldn't be read: an anonymous class.
        // VillagerTrades.getName(0): not translated, its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`).
    }

    /// `FarmersDelight` (vectorwing/farmersdelight/FarmersDelight.java)
    pub mod farmers_delight {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        pub const MODID: &str = "farmersdelight";
        // LOGGER: the logger. Log calls become comments.
        pub fn id(name: &str) -> JIdentifier {
            return JIdentifier::new("farmersdelight", &name);
        }
        // FarmersDelight.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
    }

    /// `CookingRecipeCategory` (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java)
    pub mod cooking_recipe_category {
        use super::*;
        // The class itself isn't translated: it implements `IRecipeCategory` (mezz.jei.api.recipe.category). Its static parts are below.
        // CookingRecipeCategory.CookingRecipeCategory(1): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CookingRecipeCategory.getRecipeType(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CookingRecipeCategory.getTitle(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CookingRecipeCategory.getWidth(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CookingRecipeCategory.getHeight(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CookingRecipeCategory.getIcon(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CookingRecipeCategory.setRecipe(3): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CookingRecipeCategory.draw(5): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CookingRecipeCategory.getTooltip(5): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
    }

    /// `CuttingRecipeCategory` (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java)
    pub mod cutting_recipe_category {
        use super::*;
        // The class itself isn't translated: it implements `IRecipeCategory` (mezz.jei.api.recipe.category). Its static parts are below.
        pub const OUTPUT_GRID_X: i32 = 76;
        pub const OUTPUT_GRID_Y: i32 = 10;
        // CuttingRecipeCategory.CuttingRecipeCategory(1): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CuttingRecipeCategory.getRecipeType(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CuttingRecipeCategory.getTitle(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CuttingRecipeCategory.getWidth(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CuttingRecipeCategory.getHeight(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CuttingRecipeCategory.getIcon(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CuttingRecipeCategory.setRecipe(3): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // CuttingRecipeCategory.draw(5): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
    }

    /// `DecompositionRecipeCategory` (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java)
    pub mod decomposition_recipe_category {
        use super::*;
        // The class itself isn't translated: it implements `IRecipeCategory` (mezz.jei.api.recipe.category). Its static parts are below.
        pub static UID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "decomposition"));
        pub const SLOT_SIZE: i32 = 22;
        // DecompositionRecipeCategory.DecompositionRecipeCategory(1): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.getRecipeType(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.getTitle(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.getWidth(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.getHeight(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.getIcon(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.setRecipe(3): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.draw(5): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.getTooltip(5): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // DecompositionRecipeCategory.translateKey(1): not translated, it uses Minecraft's `MutableComponent`.
    }

    /// `FDRecipes` (vectorwing/farmersdelight/integration/jei/FDRecipes.java)
    pub mod fd_recipes {
        use super::*;
        // The class itself isn't translated: it uses Fabric's `SynchronizedRecipes`. Its static parts are below.
        // FDRecipes.FDRecipes(0): not translated, its class isn't translated (it uses Fabric's `SynchronizedRecipes`).
        // FDRecipes.getCookingPotRecipes(0): not translated, its class isn't translated (it uses Fabric's `SynchronizedRecipes`).
        // FDRecipes.getCuttingBoardRecipes(0): not translated, its class isn't translated (it uses Fabric's `SynchronizedRecipes`).
    }

    /// `FDRecipeTypes` (vectorwing/farmersdelight/integration/jei/FDRecipeTypes.java)
    pub mod fd_recipe_types {
        use super::*;
        // COOKING: not translated, it uses `IRecipeType` (mezz.jei.api.recipe.types).
        // CUTTING: not translated, it uses `IRecipeType` (mezz.jei.api.recipe.types).
        // DECOMPOSITION: not translated, it uses `IRecipeType` (mezz.jei.api.recipe.types).
        #[derive(Clone, Debug)]
        pub struct FDRecipeTypes {
        }
        impl FDRecipeTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `JEIPlugin` (vectorwing/farmersdelight/integration/jei/JEIPlugin.java)
    pub mod jei_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IModPlugin` (mezz.jei.api). Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "jei_plugin"));
        // JEIPlugin.registerCategories(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JEIPlugin.registerRecipes(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JEIPlugin.registerRecipeCatalysts(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JEIPlugin.registerGuiHandlers(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JEIPlugin.registerRecipeTransferHandlers(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JEIPlugin.getPluginUid(0): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
    }

    /// `DecompositionDummy` (vectorwing/farmersdelight/integration/jei/resource/DecompositionDummy.java)
    pub mod decomposition_dummy {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DecompositionDummy {
        }
        impl DecompositionDummy {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `DoughRecipeMaker` (vectorwing/farmersdelight/integration/jei/resource/DoughRecipeMaker.java)
    pub mod dough_recipe_maker {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DoughRecipeMaker {
        }
        impl DoughRecipeMaker {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DoughRecipeMaker.createRecipe(0): not translated, it uses Minecraft's `RecipeHolder`.
    }

    /// `LaunchpadClientEvents` (vectorwing/farmersdelight/integration/launchpad/LaunchpadClientEvents.java)
    pub mod launchpad_client_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LaunchpadClientEvents {
        }
        impl LaunchpadClientEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LaunchpadClientEvents.register(1): not translated, it uses `RegisterGuiLayersEvent` (net.neoforged.neoforge.client.event).
    }

    /// `LaunchpadEvents` (vectorwing/farmersdelight/integration/launchpad/LaunchpadEvents.java)
    pub mod launchpad_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LaunchpadEvents {
        }
        impl LaunchpadEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LaunchpadEvents.getGrowthSpeed(3): not translated, it uses Minecraft's `BlockState`.
    }

    /// `ModMenuCompat` (vectorwing/farmersdelight/integration/modmenu/ModMenuCompat.java)
    pub mod mod_menu_compat {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // ModMenuCompat.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `CookingPotCategory` (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java)
    pub mod cooking_pot_category {
        use super::*;
        // The class itself isn't translated: it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display). Its static parts are below.
        // TITLE: a game value, so it's in game_side.rs.
        pub static BACKGROUND: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "textures/gui/cooking_pot.png"));
        // CookingPotCategory.getCategoryIdentifier(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CookingPotCategory.getTitle(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CookingPotCategory.getIcon(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CookingPotCategory.getDisplayHeight(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CookingPotCategory.getDisplayWidth(1): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CookingPotCategory.getDisplayRenderer(1): not translated, its body couldn't be read: an anonymous class.
        // CookingPotCategory.setupDisplay(2): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CookingPotCategory.getTooltipStrings(1): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
    }

    /// `CuttingCategory` (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java)
    pub mod cutting_category {
        use super::*;
        // The class itself isn't translated: it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display). Its static parts are below.
        pub static BACKGROUND: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "textures/gui/jei/cutting_board.png"));
        pub const OUTPUT_GRID_X: i32 = 76;
        pub const OUTPUT_GRID_Y: i32 = 10;
        // CuttingCategory.getCategoryIdentifier(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CuttingCategory.getTitle(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CuttingCategory.getIcon(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CuttingCategory.getDisplayHeight(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CuttingCategory.getDisplayWidth(1): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // CuttingCategory.setupDisplay(2): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
    }

    /// `DecompositionCategory` (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java)
    pub mod decomposition_category {
        use super::*;
        // The class itself isn't translated: it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display). Its static parts are below.
        pub static BACKGROUND: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("farmersdelight", "textures/gui/jei/decomposition.png"));
        pub const OUTPUT_GRID_X: i32 = 76;
        pub const OUTPUT_GRID_Y: i32 = 10;
        // DecompositionCategory.getCategoryIdentifier(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // DecompositionCategory.getTitle(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // DecompositionCategory.getIcon(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // DecompositionCategory.getDisplayHeight(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // DecompositionCategory.getDisplayWidth(1): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // DecompositionCategory.setupDisplay(2): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
        // DecompositionCategory.translateKey(1): not translated, it uses Minecraft's `MutableComponent`.
        // DecompositionCategory.getFixedDisplaysPerPage(0): not translated, its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)).
    }

    /// `ClientREIPlugin` (vectorwing/farmersdelight/integration/rei/client/ClientREIPlugin.java)
    pub mod client_rei_plugin {
        use super::*;
        // The class itself isn't translated: it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins). Its static parts are below.
        // ClientREIPlugin.registerDisplays(1): not translated, its class isn't translated (it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)).
        // ClientREIPlugin.registerScreens(1): not translated, its class isn't translated (it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)).
        // ClientREIPlugin.registerCategories(1): not translated, its class isn't translated (it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)).
        // ClientREIPlugin.registerTransferHandlers(1): not translated, its class isn't translated (it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)).
    }

    /// `ClientsidedCookingPotDisplay` (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java)
    pub mod clientsided_cooking_pot_display {
        use super::*;
        // The class itself isn't translated: it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic). Its static parts are below.
        // SERIALIZER: not translated, it uses `DisplaySerializer` (me.shedaniel.rei.api.common.display).
        // ClientsidedCookingPotDisplay.ClientsidedCookingPotDisplay(2): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.ClientsidedCookingPotDisplay(6): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.getOutputContainer(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.getCookTime(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.getExperience(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.isShapeless(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.getWidth(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.getHeight(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.getCategoryIdentifier(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.getSerializer(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // ClientsidedCookingPotDisplay.recipeDisplayId(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
    }

    /// `CommonREIPlugin` (vectorwing/farmersdelight/integration/rei/CommonREIPlugin.java)
    pub mod common_rei_plugin {
        use super::*;
        // The class itself isn't translated: it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins). Its static parts are below.
        // CommonREIPlugin.registerDisplaySerializer(1): not translated, its class isn't translated (it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins)).
        // CommonREIPlugin.registerDisplays(1): not translated, its class isn't translated (it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins)).
    }

    /// `CookingPotDisplay` (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java)
    pub mod cooking_pot_display {
        use super::*;
        // The class itself isn't translated: it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic). Its static parts are below.
        // SERIALIZER: not translated, it uses `DisplaySerializer` (me.shedaniel.rei.api.common.display).
        // CookingPotDisplay.CookingPotDisplay(1): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.CookingPotDisplay(6): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.getOutputContainer(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.getCookTime(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.getExperience(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.getCategoryIdentifier(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.getSerializer(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.isShapeless(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.getWidth(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CookingPotDisplay.getHeight(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
    }

    /// `CuttingDisplay` (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java)
    pub mod cutting_display {
        use super::*;
        // The class itself isn't translated: it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic). Its static parts are below.
        // CHANCE_RESULTS_STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SERIALIZER: not translated, it uses `DisplaySerializer` (me.shedaniel.rei.api.common.display).
        // CuttingDisplay.CuttingDisplay(1): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CuttingDisplay.CuttingDisplay(5): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CuttingDisplay.getTool(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CuttingDisplay.getRollableOutputs(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CuttingDisplay.getCategoryIdentifier(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CuttingDisplay.getSerializer(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CuttingDisplay.isShapeless(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CuttingDisplay.getWidth(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // CuttingDisplay.getHeight(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
    }

    /// `DecompositionDisplay` (vectorwing/farmersdelight/integration/rei/display/DecompositionDisplay.java)
    pub mod decomposition_display {
        use super::*;
        // The class itself isn't translated: it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic). Its static parts are below.
        // SERIALIZER: not translated, it uses `DisplaySerializer` (me.shedaniel.rei.api.common.display).
        // DecompositionDisplay.DecompositionDisplay(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // DecompositionDisplay.getCategoryIdentifier(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
        // DecompositionDisplay.getSerializer(0): not translated, its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)).
    }

    /// `REICategoryIdentifiers` (vectorwing/farmersdelight/integration/rei/REICategoryIdentifiers.java)
    pub mod rei_category_identifiers {
        use super::*;
        // COOKING: not translated, it uses `CategoryIdentifier` (me.shedaniel.rei.api.common.category).
        // CUTTING: not translated, it uses `CategoryIdentifier` (me.shedaniel.rei.api.common.category).
        // DECOMPOSITION: not translated, it uses `CategoryIdentifier` (me.shedaniel.rei.api.common.category).
        #[derive(Clone, Debug)]
        pub struct REICategoryIdentifiers {
        }
        impl REICategoryIdentifiers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ServerREIPlugin` (vectorwing/farmersdelight/integration/rei/ServerREIPlugin.java)
    pub mod server_rei_plugin {
        use super::*;
        // The class itself isn't translated: it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins). Its static parts are below.
        // ServerREIPlugin.registerDisplaySerializer(1): not translated, its class isn't translated (it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins)).
    }

    /// `CookingPotClientRecipe` (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java)
    pub mod cooking_pot_client_recipe {
        use super::*;
        // The class itself isn't translated: it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe). Its static parts are below.
        // CookingPotClientRecipe.CookingPotClientRecipe(6): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.getType(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.getId(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.bindSlots(1): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.renderRecipe(6): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.getIngredients(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.getResults(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.supportsItemTransfer(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.mapRecipeItems(2): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.getTransferClasses(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipe.canTransferToScreen(1): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
    }

    /// `CookingPotClientRecipeType` (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java)
    pub mod cooking_pot_client_recipe_type {
        use super::*;
        // The class itself isn't translated: it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe). Its static parts are below.
        // INSTANCE: not translated, it uses CookingPotClientRecipeType, which isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.getDisplayName(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.getId(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.getIcon(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.getDisplayWidth(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.getDisplayHeight(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.getGuiTexture(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.getSlotCount(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.placeSlots(1): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CookingPotClientRecipeType.getCraftReferences(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
    }

    /// `CuttingClientRecipe` (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java)
    pub mod cutting_client_recipe {
        use super::*;
        // The class itself isn't translated: it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe). Its static parts are below.
        // CuttingClientRecipe.CuttingClientRecipe(5): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipe.getType(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipe.getId(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipe.bindSlots(1): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipe.getIngredients(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipe.getResults(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
    }

    /// `CuttingClientRecipe.CuttingSlotRenderer` (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java)
    pub mod cutting_client_recipe_cutting_slot_renderer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CuttingClientRecipe.CuttingSlotRenderer.render(4): not translated, it has no body.
    }

    /// `CuttingClientRecipeType` (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java)
    pub mod cutting_client_recipe_type {
        use super::*;
        // The class itself isn't translated: it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe). Its static parts are below.
        // INSTANCE: not translated, it uses CuttingClientRecipeType, which isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.getDisplayName(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.getId(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.getIcon(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.getDisplayWidth(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.getDisplayHeight(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.getGuiTexture(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.getSlotCount(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.placeSlots(1): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // CuttingClientRecipeType.getCraftReferences(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
    }

    /// `DecompositionClientRecipe` (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java)
    pub mod decomposition_client_recipe {
        use super::*;
        // The class itself isn't translated: it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe). Its static parts are below.
        // activators: not translated, it uses `SlotContent` (cc.cassian.rrv.common.recipe.inventory).
        // LIGHT_TOOLTIP: not translated, it uses Minecraft's `MutableComponent`.
        // FLUID_TOOLTIP: not translated, it uses Minecraft's `MutableComponent`.
        // ACCELERATORS_TOOLTIP: not translated, it uses Minecraft's `MutableComponent`.
        // DecompositionClientRecipe.DecompositionClientRecipe(2): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipe.getId(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipe.getType(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipe.bindSlots(1): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipe.renderRecipe(6): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipe.createTooltip(1): not translated, it uses Minecraft's `MutableComponent`.
        // DecompositionClientRecipe.getIngredients(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipe.getResults(0): not translated, its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)).
    }

    /// `DecompositionClientRecipeType` (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java)
    pub mod decomposition_client_recipe_type {
        use super::*;
        // The class itself isn't translated: it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe). Its static parts are below.
        // INSTANCE: not translated, it uses DecompositionClientRecipeType, which isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipeType.getDisplayName(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipeType.getDisplayWidth(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipeType.getDisplayHeight(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipeType.getGuiTexture(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipeType.getSlotCount(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipeType.placeSlots(1): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipeType.getId(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
        // DecompositionClientRecipeType.getIcon(0): not translated, its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)).
    }

    /// `RRVPlugin` (vectorwing/farmersdelight/integration/rrv/RRVPlugin.java)
    pub mod rrv_plugin {
        use super::*;
        // The class itself isn't translated: it implements `ReliableRecipeViewerClientPlugin` (cc.cassian.rrv.api). Its static parts are below.
        // RRVPlugin.onIntegrationInitialize(0): not translated, its class isn't translated (it implements `ReliableRecipeViewerClientPlugin` (cc.cassian.rrv.api)).
    }

    /// `ReplaceOnRemoval` (vectorwing/farmersdelight/refabricated/block/ReplaceOnRemoval.java)
    pub mod replace_on_removal {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ReplaceOnRemoval.onRemove(5): not translated, it has no body.
    }

    /// `CanItemPerformAbility` (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java)
    pub mod can_item_perform_ability {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `LootItemCondition`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CanItemPerformAbility.CanItemPerformAbility(1): not translated, its class isn't translated (it implements Minecraft's `LootItemCondition`).
        // CanItemPerformAbility.init(0): not translated, it uses Minecraft's `MapCodec`.
        // CanItemPerformAbility.canItemPerformAbility(1): not translated, it uses Minecraft's `Builder`.
        // CanItemPerformAbility.test(1): not translated, its class isn't translated (it implements Minecraft's `LootItemCondition`).
        // CanItemPerformAbility.getReferencedContextParams(0): not translated, its class isn't translated (it implements Minecraft's `LootItemCondition`).
        // CanItemPerformAbility.codec(0): not translated, its class isn't translated (it implements Minecraft's `LootItemCondition`).
        // CanItemPerformAbility.ability(0): not translated, its class isn't translated (it implements Minecraft's `LootItemCondition`).
    }

    /// `CompostableHelper` (vectorwing/farmersdelight/refabricated/CompostableHelper.java)
    pub mod compostable_helper {
        use super::*;
        pub const COMPOSTABLES: &str = "\n{\n  \"values\": {\n    \"farmersdelight:apple_pie\": {\n      \"chance\": 1.0\n    },\n    \"farmersdelight:apple_pie_slice\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:brown_mushroom_colony\": {\n      \"chance\": 1.0\n    },\n    \"farmersdelight:cabbage\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:cabbage_leaf\": {\n      \"chance\": 0.5\n    },\n    \"farmersdelight:cabbage_seeds\": {\n      \"chance\": 0.3\n    },\n    \"farmersdelight:cake_slice\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:chocolate_pie\": {\n      \"chance\": 1.0\n    },\n    \"farmersdelight:chocolate_pie_slice\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:dumplings\": {\n      \"chance\": 1.0\n    },\n    \"farmersdelight:honey_cookie\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:kelp_roll\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:kelp_roll_slice\": {\n      \"chance\": 0.5\n    },\n    \"farmersdelight:onion\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:pie_crust\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:pumpkin_slice\": {\n      \"chance\": 0.5\n    },\n    \"farmersdelight:raw_pasta\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:red_mushroom_colony\": {\n      \"chance\": 1.0\n    },\n    \"farmersdelight:rice\": {\n      \"chance\": 0.3\n    },\n    \"farmersdelight:rice_bale\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:rice_panicle\": {\n      \"chance\": 0.3\n    },\n    \"farmersdelight:rotten_tomato\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:sandy_shrub\": {\n      \"chance\": 0.3\n    },\n    \"farmersdelight:straw\": {\n      \"chance\": 0.3\n    },\n    \"farmersdelight:stuffed_pumpkin_block\": {\n      \"chance\": 1.0\n    },\n    \"farmersdelight:sweet_berry_cheesecake\": {\n      \"chance\": 1.0\n    },\n    \"farmersdelight:sweet_berry_cheesecake_slice\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:sweet_berry_cookie\": {\n      \"chance\": 0.85\n    },\n    \"farmersdelight:tomato\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:tomato_seeds\": {\n      \"chance\": 0.3\n    },\n    \"farmersdelight:tree_bark\": {\n      \"chance\": 0.3\n    },\n    \"farmersdelight:wild_beetroots\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:wild_cabbages\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:wild_carrots\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:wild_onions\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:wild_potatoes\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:wild_rice\": {\n      \"chance\": 0.65\n    },\n    \"farmersdelight:wild_tomatoes\": {\n      \"chance\": 0.65\n    }\n  }\n}\n";
        #[derive(Clone, Debug)]
        pub struct CompostableHelper {
        }
        impl CompostableHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CompostableHelper.apply(0): not translated, JSON's asMap.
    }

    /// `RefabricatedMobEffectTags` (vectorwing/farmersdelight/refabricated/data/RefabricatedMobEffectTags.java)
    pub mod refabricated_mob_effect_tags {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricTagsProvider`. Its static parts are below.
        // RefabricatedMobEffectTags.RefabricatedMobEffectTags(2): not translated, its class isn't translated (it extends Fabric's `FabricTagsProvider`).
        // RefabricatedMobEffectTags.addTags(1): not translated, its class isn't translated (it extends Fabric's `FabricTagsProvider`).
    }

    /// `SkilletEntityRenderState` (vectorwing/farmersdelight/refabricated/duck/SkilletEntityRenderState.java)
    pub mod skillet_entity_render_state {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SkilletEntityRenderState.fdrf$getSkilletFlipTimestamp(0): not translated, it has no body.
        // SkilletEntityRenderState.fdrf$setSkilletFlipTimestamp(1): not translated, it has no body.
    }

    /// `FDRecipeBookTypes` (vectorwing/farmersdelight/refabricated/FDRecipeBookTypes.java)
    pub mod fd_recipe_book_types {
        use super::*;
        // COOKING: not translated, it uses Minecraft's `RecipeBookType`.
        #[derive(Clone, Debug)]
        pub struct FDRecipeBookTypes {
        }
        impl FDRecipeBookTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FDRefabricatedTags` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java)
    pub mod fd_refabricated_tags {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FDRefabricatedTags {
        }
        impl FDRefabricatedTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FDRefabricatedTags.modBiomeTag(1): not translated, it uses Minecraft's `TagKey`.
        // FDRefabricatedTags.modBlockTag(1): not translated, it uses Minecraft's `TagKey`.
        // FDRefabricatedTags.modEntityTag(1): not translated, it uses Minecraft's `TagKey`.
        // FDRefabricatedTags.modItemTag(1): not translated, it uses Minecraft's `TagKey`.
        // FDRefabricatedTags.modEffectTag(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `FDRefabricatedTags.Biomes` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java)
    pub mod fd_refabricated_tags_biomes {
        use super::*;
        // HAS_BROWN_MUSHROOM_COLONY: not translated, it uses Minecraft's `TagKey`.
        // HAS_RED_MUSHROOM_COLONY: not translated, it uses Minecraft's `TagKey`.
        // HAS_WILD_CABBAGE: not translated, it uses Minecraft's `TagKey`.
        // HAS_WILD_BEETROOTS: not translated, it uses Minecraft's `TagKey`.
        // WILD_CARROTS_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_CARROTS_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_ONIONS_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_ONIONS_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_POTATOES_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_POTATOES_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_RICE_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_RICE_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_TOMATOES_WHITELIST: not translated, it uses Minecraft's `TagKey`.
        // WILD_TOMATOES_BLACKLIST: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct Biomes {
        }
        impl Biomes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FDRefabricatedTags.Blocks` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java)
    pub mod fd_refabricated_tags_blocks {
        use super::*;
        // KNIFE_INSTANTLY_MINES: not translated, it uses Minecraft's `TagKey`.
        // GROWS_MUSHROOM_COLONIES: not translated, it uses Minecraft's `TagKey`.
        // GROWS_WILD_CROPS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct Blocks {
        }
        impl Blocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FDRefabricatedTags.EntityTypes` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java)
    pub mod fd_refabricated_tags_entity_types {
        use super::*;
        // DROPS_LEATHER: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct EntityTypes {
        }
        impl EntityTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FDRefabricatedTags.Items` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java)
    pub mod fd_refabricated_tags_items {
        use super::*;
        // FLINT_TOOL_MATERIALS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct Items {
        }
        impl Items {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FDRefabricatedTags.MobEffects` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java)
    pub mod fd_refabricated_tags_mob_effects {
        use super::*;
        // MILK_BOTTLE_IGNORED: not translated, it uses Minecraft's `TagKey`.
        // HOT_COCOA_IGNORED: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct MobEffects {
        }
        impl MobEffects {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FabricWrappedInventory` (vectorwing/farmersdelight/refabricated/inventory/FabricWrappedInventory.java)
    pub mod fabric_wrapped_inventory {
        use super::*;
        // The class itself isn't translated: it implements `ItemHandler`. Its static parts are below.
        // FabricWrappedInventory.FabricWrappedInventory(1): not translated, its class isn't translated (it implements `ItemHandler`).
        // FabricWrappedInventory.getSlot(1): not translated, its class isn't translated (it implements `ItemHandler`).
        // FabricWrappedInventory.insert(3): not translated, its class isn't translated (it implements `ItemHandler`).
        // FabricWrappedInventory.extract(3): not translated, its class isn't translated (it implements `ItemHandler`).
        // FabricWrappedInventory.iterator(0): not translated, its class isn't translated (it implements `ItemHandler`).
    }

    /// `InvWrapper` (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java)
    pub mod inv_wrapper {
        use super::*;
        // The class itself isn't translated: it extends `FabricWrappedInventory`. Its static parts are below.
        // InvWrapper.InvWrapper(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.equals(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.getInv(0): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.getSlotCount(0): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.getSlotLimit(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.getStackInSlot(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.insertItem(3): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.extractItem(3): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.setStackInSlot(2): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // InvWrapper.isItemValid(2): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
    }

    /// `ItemHandler` (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java)
    pub mod item_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ItemHandler.getSlotCount(0): not translated, it has no body.
        // ItemHandler.getSlotLimit(1): not translated, it has no body.
        // ItemHandler.getStackInSlot(1): not translated, it has no body.
        // ItemHandler.insertItem(3): not translated, it has no body.
        // ItemHandler.extractItem(3): not translated, it has no body.
        // ItemHandler.setStackInSlot(2): not translated, it has no body.
        // ItemHandler.isItemValid(2): not translated, it has no body.
        // ItemHandler.getInputSlotIndexes(0): not translated, its class isn't translated (it's an interface).
        // ItemHandler.commitModifiedStacks(0): not translated, its class isn't translated (it's an interface).
        // ItemHandler.removeItem(2): not translated, its class isn't translated (it's an interface).
        // ItemHandler.insertSlot(4): not translated, its class isn't translated (it's an interface).
        // ItemHandler.extractSlot(4): not translated, its class isn't translated (it's an interface).
    }

    /// `ItemHandlerSlot` (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java)
    pub mod item_handler_slot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // EMPTY_INVENTORY: not translated, it uses Minecraft's `Container`.
        // ItemHandlerSlot.ItemHandlerSlot(4): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // ItemHandlerSlot.mayPlace(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // ItemHandlerSlot.getItem(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // ItemHandlerSlot.set(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // ItemHandlerSlot.remove(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // ItemHandlerSlot.getMaxStackSize(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // ItemHandlerSlot.getItemHandler(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `ItemHandlerStackWrapper` (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerStackWrapper.java)
    pub mod item_handler_stack_wrapper {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `SingleStackStorage`. Its static parts are below.
        // ItemHandlerStackWrapper.ItemHandlerStackWrapper(2): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // ItemHandlerStackWrapper.getStack(0): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // ItemHandlerStackWrapper.setStack(1): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // ItemHandlerStackWrapper.getCapacity(1): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
    }

    /// `ItemStackHandler` (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java)
    pub mod item_stack_handler {
        use super::*;
        // The class itself isn't translated: it extends `FabricWrappedInventory`. Its static parts are below.
        // ItemStackHandler.ItemStackHandler(0): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.ItemStackHandler(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.getSlotCount(0): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.getSlotLimit(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.getStackLimit(2): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.isItemValid(2): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.getStackInSlot(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.insertItem(3): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.extractItem(3): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.setStackInSlot(2): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.serialize(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.deserialize(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.onContentsChanged(1): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
        // ItemStackHandler.removeItem(3): not translated, its class isn't translated (it extends `FabricWrappedInventory`).
    }

    /// `RecipeWrapper` (vectorwing/farmersdelight/refabricated/inventory/RecipeWrapper.java)
    pub mod recipe_wrapper {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RecipeInput`. Its static parts are below.
        // RecipeWrapper.RecipeWrapper(1): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // RecipeWrapper.stackedContents(0): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // RecipeWrapper.ingredientAmount(0): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // RecipeWrapper.getItem(1): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // RecipeWrapper.size(0): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
    }

    /// `ItemAbility` (vectorwing/farmersdelight/refabricated/ItemAbility.java)
    pub mod item_ability {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StringRepresentable`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ItemAbility.getSerializedName(0): not translated, its class isn't translated (it implements Minecraft's `StringRepresentable`).
        // ItemAbility.canPerformAction(1): not translated, its class isn't translated (it implements Minecraft's `StringRepresentable`).
        // ItemAbility.canPerformAction(1): not translated, its class isn't translated (it implements Minecraft's `StringRepresentable`).
    }

    /// `LootModificationEvents` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java)
    pub mod loot_modification_events {
        use super::*;
        // BLOCKS_CAKE: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_PUMPKIN: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_SHORT_GRASS: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_TALL_GRASS: not translated, it uses Minecraft's `ResourceKey`.
        // ENTITIES_CAVE_SPIDER: not translated, it uses Minecraft's `ResourceKey`.
        // ENTITIES_CHICKEN: not translated, it uses Minecraft's `ResourceKey`.
        // ENTITIES_HOGLIN: not translated, it uses Minecraft's `ResourceKey`.
        // ENTITIES_PIG: not translated, it uses Minecraft's `ResourceKey`.
        // ENTITIES_RABBIT: not translated, it uses Minecraft's `ResourceKey`.
        // ENTITIES_SHULKER: not translated, it uses Minecraft's `ResourceKey`.
        // ENTITIES_SPIDER: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_WHEAT: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_APPLE_PIE: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_CHOCOLATE_PIE: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_PUMPKIN_PIE: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_RICE_PANICLES: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_SANDY_SHRUB: not translated, it uses Minecraft's `ResourceKey`.
        // BLOCKS_SWEET_BERRY_CHEESECAKE: not translated, it uses Minecraft's `ResourceKey`.
        // FD_ABANDONED_MINESHAFT: not translated, it uses Minecraft's `ResourceKey`.
        // FD_BASTION_HOGLIN_STABLE: not translated, it uses Minecraft's `ResourceKey`.
        // FD_BASTION_TREASURE: not translated, it uses Minecraft's `ResourceKey`.
        // FD_END_CITY_TREASURE: not translated, it uses Minecraft's `ResourceKey`.
        // FD_PILLAGER_OUTPOST: not translated, it uses Minecraft's `ResourceKey`.
        // FD_RUINED_PORTAL: not translated, it uses Minecraft's `ResourceKey`.
        // FD_SHIPWRECK_SUPPLY: not translated, it uses Minecraft's `ResourceKey`.
        // FD_SIMPLE_DUNGEON: not translated, it uses Minecraft's `ResourceKey`.
        // FD_VILLAGE_BUTCHER: not translated, it uses Minecraft's `ResourceKey`.
        // FD_VILLAGE_DESERT_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        // FD_VILLAGE_PLAINS_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        // FD_VILLAGE_SAVANNA_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        // FD_VILLAGE_SNOWY_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        // FD_VILLAGE_TAIGA_HOUSE: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct LootModificationEvents {
        }
        impl LootModificationEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LootModificationEvents.init(0): not translated, it uses Fabric's `LootTableEvents`.MODIFY.
        // LootModificationEvents.modifyDrops(3): not translated, it uses Minecraft's `Holder`.
        // LootModificationEvents.modifyTable(4): not translated, it uses Minecraft's `ResourceKey`.
        // LootModificationEvents.chestLoot(4): not translated, it uses Minecraft's `ResourceKey`.
        // LootModificationEvents.lootTableReference(2): not translated, it uses Minecraft's `LootPoolEntryContainer`.
        // LootModificationEvents.scavengingLoot(4): not translated, it uses Minecraft's `ResourceKey`.
        // LootModificationEvents.slicingLoot(4): not translated, it uses Minecraft's `ResourceKey`.
        // LootModificationEvents.pastrySlicing(6): not translated, it uses Minecraft's `Builder`.
        // LootModificationEvents.straw(4): not translated, it uses Minecraft's `ResourceKey`.
        // LootModificationEvents.strawChance02(2): not translated, it uses Minecraft's `Builder`.
        // LootModificationEvents.strawChance03(2): not translated, it uses Minecraft's `Builder`.
        // LootModificationEvents.vanillaKey(1): not translated, it uses Minecraft's `ResourceKey`.
        // LootModificationEvents.key(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `LootModifier` (vectorwing/farmersdelight/refabricated/LootModifier.java)
    pub mod loot_modifier {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LootModifier {
        }
        impl LootModifier {
            // LootModifier.LootModifier(1): not translated, it uses Minecraft's `LootItemCondition`.
            // LootModifier.LootModifier(2): not translated, it uses Minecraft's `LootItemCondition`.
            // LootModifier.doApply(2): not translated, it has no body.
        }
    }

    /// `ConfigBuilder` (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java)
    pub mod config_builder {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        pub static YACL: LazyLock<bool> = LazyLock::new(|| false);
        pub static CLOTH_CONFIG: LazyLock<bool> = LazyLock::new(|| false);
        // STRING_CHECK: not translated, it uses `Object`.
        // LIST_STRING_CHECK: not translated, it uses `Object`.
        // ConfigBuilder.create(2): not translated, it uses ConfigBuilder, which isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.create(2): not translated, it uses ConfigBuilder, which isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.ConfigBuilder(2): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.build(0): not translated, it has no body.
        // ConfigBuilder.getName(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.push(1): not translated, it has no body.
        // ConfigBuilder.pop(0): not translated, it has no body.
        // ConfigBuilder.setWriteJsons(0): not translated, it's generic.
        // ConfigBuilder.define(2): not translated, it has no body.
        // ConfigBuilder.define(4): not translated, it has no body.
        // ConfigBuilder.define(4): not translated, it has no body.
        // ConfigBuilder.define(4): not translated, it has no body.
        // ConfigBuilder.defineColor(2): not translated, it has no body.
        // ConfigBuilder.define(3): not translated, it has no body.
        // ConfigBuilder.define(2): not translated, it has no body.
        // ConfigBuilder.define(2): not translated, it has no body.
        // ConfigBuilder.currentCategory(0): not translated, it has no body.
        // ConfigBuilder.define(3): not translated, it has no body.
        // ConfigBuilder.define(2): not translated, it has no body.
        // ConfigBuilder.defineObject(3): not translated, it has no body.
        // ConfigBuilder.defineObjectList(3): not translated, it's generic.
        // ConfigBuilder.defineMap(2): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.defineIDMap(2): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.defineJson(2): not translated, it has no body.
        // ConfigBuilder.defineJson(2): not translated, it has no body.
        // ConfigBuilder.define(2): not translated, it has no body.
        // ConfigBuilder.description(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.tooltip(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.tooltipKey(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.translationKey(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.comment(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.onChange(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilder.worldReload(0): not translated, it has no body.
        // ConfigBuilder.gameRestart(0): not translated, it has no body.
        // ConfigBuilder.maybeAddTranslationString(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `ConfigBuilder.IdentifierConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java)
    pub mod config_builder_identifier_config_value {
        use super::*;
        // The class itself isn't translated: it implements `Supplier` (com.google.common.base). Its static parts are below.
        // ConfigBuilder.IdentifierConfigValue.IdentifierConfigValue(3): not translated, its class isn't translated (it implements `Supplier` (com.google.common.base)).
        // ConfigBuilder.IdentifierConfigValue.get(0): not translated, its class isn't translated (it implements `Supplier` (com.google.common.base)).
    }

    /// `ConfigType` (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigType.java)
    pub mod config_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ConfigType {
            Common,
            CommonSynced,
            Client,
        }
        impl ConfigType {
            pub const VALUES: [ConfigType; 3] = [ConfigType::Common, ConfigType::CommonSynced, ConfigType::Client, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ConfigType::Common => "COMMON",
                    ConfigType::CommonSynced => "COMMON_SYNCED",
                    ConfigType::Client => "CLIENT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "COMMON" => ConfigType::Common,
                    "COMMON_SYNCED" => ConfigType::CommonSynced,
                    "CLIENT" => ConfigType::Client,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl ConfigType {
            // ConfigType.isSynced(0): not translated, `this` used as a value.
            // ConfigType.getDefaultName(0): not translated, `this` used as a value.
        }
    }

    /// `ConfigBuilderImpl` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java)
    pub mod config_builder_impl {
        use super::*;
        // The class itself isn't translated: it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs). Its static parts are below.
        // ConfigBuilderImpl.create(2): not translated, it uses ConfigBuilder, which isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigBuilderImpl.ConfigBuilderImpl(2): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.build(0): not translated, its body couldn't be read: an assert.
        // ConfigBuilderImpl.currentCategory(0): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.push(1): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.pop(0): not translated, its body couldn't be read: an assert.
        // ConfigBuilderImpl.doAddConfig(2): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.define(2): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.define(4): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.define(4): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.define(4): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.defineColor(2): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.define(3): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.define(3): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.define(2): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.defineJson(2): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.defineJson(2): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.defineObject(3): not translated, it's generic.
        // ConfigBuilderImpl.maybeAddTranslationString(1): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.gameRestart(0): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.worldReload(0): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // ConfigBuilderImpl.comment(1): not translated, its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
    }

    /// `ConfigEntry` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigEntry.java)
    pub mod config_entry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigEntry {
            pub name: String,
        }
        impl ConfigEntry {
            pub fn new(name: &str) -> Self {
                let mut s = Self { name: String::new() };
                s.name = name.to_string();
                s
            }
            // ConfigEntry.loadFromJson(1): not translated, it has no body.
            // ConfigEntry.saveToJson(1): not translated, it has no body.
            pub fn get_name(&self) -> String {
                return self.name.to_string();
            }
        }
    }

    /// `ConfigSubCategory` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigSubCategory.java)
    pub mod config_sub_category {
        use super::*;
        // The class itself isn't translated: it extends `ConfigEntry`. Its static parts are below.
        // ConfigSubCategory.ConfigSubCategory(1): not translated, its class isn't translated (it extends `ConfigEntry`).
        // ConfigSubCategory.addEntry(1): not translated, its class isn't translated (it extends `ConfigEntry`).
        // ConfigSubCategory.getEntries(0): not translated, its class isn't translated (it extends `ConfigEntry`).
        // ConfigSubCategory.loadFromJson(1): not translated, its class isn't translated (it extends `ConfigEntry`).
        // ConfigSubCategory.saveToJson(1): not translated, its class isn't translated (it extends `ConfigEntry`).
    }

    /// `FabricConfigHolder` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java)
    pub mod fabric_config_holder {
        use super::*;
        // The class itself isn't translated: it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs). Its static parts are below.
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson { pretty: true, ..JGson::default() });
        // FabricConfigHolder.loadAllConfigs(0): not translated, it calls FabricConfigHolder.getTrackedSpecs, which isn't in the mod's source.
        // FabricConfigHolder.FabricConfigHolder(4): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // FabricConfigHolder.getMainEntry(0): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // FabricConfigHolder.isLoaded(0): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // FabricConfigHolder.forceLoad(0): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // FabricConfigHolder.saveConfig(0): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // FabricConfigHolder.makeScreen(2): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // FabricConfigHolder.hasConfigScreen(0): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // FabricConfigHolder.loadFromBytes(1): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
        // FabricConfigHolder.onPlayerLoggedIn(3): not translated, its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)).
    }

    /// `FabricConfigListScreen` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java)
    pub mod fabric_config_list_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // FabricConfigListScreen.FabricConfigListScreen(6): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // FabricConfigListScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // FabricConfigListScreen.addExtraButtons(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // FabricConfigListScreen.removed(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // FabricConfigListScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // FabricConfigListScreen.isMouseWithin(6): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // FabricConfigListScreen.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // FabricConfigListScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `FabricConfigListScreen.ConfigButton` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java)
    pub mod fabric_config_list_screen_config_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Entry`. Its static parts are below.
        // FabricConfigListScreen.ConfigButton.ConfigButton(1): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // FabricConfigListScreen.ConfigButton.ConfigButton(3): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // FabricConfigListScreen.ConfigButton.extractContent(5): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // FabricConfigListScreen.ConfigButton.children(0): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // FabricConfigListScreen.ConfigButton.narratables(0): not translated, its class isn't translated (it extends Minecraft's `Entry`).
    }

    /// `FabricConfigListScreen.ConfigList` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java)
    pub mod fabric_config_list_screen_config_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ContainerObjectSelectionList`. Its static parts are below.
        // FabricConfigListScreen.ConfigList.ConfigList(6): not translated, its body couldn't be read: an inner class creation.
        // FabricConfigListScreen.ConfigList.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // FabricConfigListScreen.ConfigList.scrollBarX(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
    }

    /// `BoolConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/BoolConfigValue.java)
    pub mod bool_config_value {
        use super::*;
        // The class itself isn't translated: it extends `ConfigValue`. Its static parts are below.
        // BoolConfigValue.BoolConfigValue(2): not translated, its class isn't translated (it extends `ConfigValue`).
        // BoolConfigValue.isValid(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // BoolConfigValue.loadFromJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // BoolConfigValue.saveToJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
    }

    /// `ColorConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ColorConfigValue.java)
    pub mod color_config_value {
        use super::*;
        // The class itself isn't translated: it extends `IntConfigValue`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ColorConfigValue.ColorConfigValue(2): not translated, its class isn't translated (it extends `IntConfigValue`).
        // ColorConfigValue.loadFromJson(1): not translated, its class isn't translated (it extends `IntConfigValue`).
        // ColorConfigValue.saveToJson(1): not translated, its class isn't translated (it extends `IntConfigValue`).
        pub fn format_string(s: &str) -> String {
            return format!("#{}", s.to_uppercase());
        }
        // ColorConfigValue.isValidStringOrError(1): not translated, it uses Minecraft's `DataResult`.
    }

    /// `ConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java)
    pub mod config_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfigValue.ConfigValue(2): not translated, its class isn't translated (it's generic).
        // ConfigValue.getDefaultValue(0): not translated, its class isn't translated (it's generic).
        // ConfigValue.isValid(1): not translated, it has no body.
        // ConfigValue.set(1): not translated, its class isn't translated (it's generic).
        // ConfigValue.get(0): not translated, its class isn't translated (it's generic).
        // ConfigValue.setDescriptionKey(1): not translated, its class isn't translated (it's generic).
        // ConfigValue.setTranslationKey(1): not translated, its class isn't translated (it's generic).
        // ConfigValue.getTranslation(0): not translated, its class isn't translated (it's generic).
        // ConfigValue.getDescription(0): not translated, its class isn't translated (it's generic).
    }

    /// `DoubleConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/DoubleConfigValue.java)
    pub mod double_config_value {
        use super::*;
        // The class itself isn't translated: it extends `ConfigValue`. Its static parts are below.
        // DoubleConfigValue.DoubleConfigValue(4): not translated, its class isn't translated (it extends `ConfigValue`).
        // DoubleConfigValue.isValid(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // DoubleConfigValue.loadFromJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // DoubleConfigValue.saveToJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // DoubleConfigValue.getMax(0): not translated, its class isn't translated (it extends `ConfigValue`).
        // DoubleConfigValue.getMin(0): not translated, its class isn't translated (it extends `ConfigValue`).
    }

    /// `EnumConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/EnumConfigValue.java)
    pub mod enum_config_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EnumConfigValue.EnumConfigValue(2): not translated, its class isn't translated (it's generic).
        // EnumConfigValue.isValid(1): not translated, its class isn't translated (it's generic).
        // EnumConfigValue.getEnumClass(0): not translated, its class isn't translated (it's generic).
        // EnumConfigValue.loadFromJson(1): not translated, its class isn't translated (it's generic).
        // EnumConfigValue.saveToJson(1): not translated, its class isn't translated (it's generic).
    }

    /// `FloatConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/FloatConfigValue.java)
    pub mod float_config_value {
        use super::*;
        // The class itself isn't translated: it extends `ConfigValue`. Its static parts are below.
        // FloatConfigValue.FloatConfigValue(4): not translated, its class isn't translated (it extends `ConfigValue`).
        // FloatConfigValue.isValid(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // FloatConfigValue.loadFromJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // FloatConfigValue.saveToJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // FloatConfigValue.getMax(0): not translated, its class isn't translated (it extends `ConfigValue`).
        // FloatConfigValue.getMin(0): not translated, its class isn't translated (it extends `ConfigValue`).
    }

    /// `IntConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/IntConfigValue.java)
    pub mod int_config_value {
        use super::*;
        // The class itself isn't translated: it extends `ConfigValue`. Its static parts are below.
        // IntConfigValue.IntConfigValue(4): not translated, its class isn't translated (it extends `ConfigValue`).
        // IntConfigValue.isValid(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // IntConfigValue.loadFromJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // IntConfigValue.saveToJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // IntConfigValue.getMax(0): not translated, its class isn't translated (it extends `ConfigValue`).
        // IntConfigValue.getMin(0): not translated, its class isn't translated (it extends `ConfigValue`).
    }

    /// `JsonConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/JsonConfigValue.java)
    pub mod json_config_value {
        use super::*;
        // The class itself isn't translated: it extends `ConfigValue`. Its static parts are below.
        // JsonConfigValue.JsonConfigValue(2): not translated, its class isn't translated (it extends `ConfigValue`).
        // JsonConfigValue.isValid(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // JsonConfigValue.loadFromJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // JsonConfigValue.getDefaultValue(0): not translated, its class isn't translated (it extends `ConfigValue`).
        // JsonConfigValue.saveToJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
    }

    /// `ListStringConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ListStringConfigValue.java)
    pub mod list_string_config_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ListStringConfigValue.ListStringConfigValue(3): not translated, its class isn't translated (it's generic).
        // ListStringConfigValue.isValid(1): not translated, its class isn't translated (it's generic).
        // ListStringConfigValue.loadFromJson(1): not translated, its class isn't translated (it's generic).
        // ListStringConfigValue.saveToJson(1): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ObjectConfigValue.java)
    pub mod object_config_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectConfigValue.ObjectConfigValue(3): not translated, its class isn't translated (it's generic).
        // ObjectConfigValue.isValid(1): not translated, its class isn't translated (it's generic).
        // ObjectConfigValue.loadFromJson(1): not translated, its class isn't translated (it's generic).
        // ObjectConfigValue.getDefaultValue(0): not translated, its class isn't translated (it's generic).
        // ObjectConfigValue.saveToJson(1): not translated, its class isn't translated (it's generic).
    }

    /// `StringConfigValue` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/StringConfigValue.java)
    pub mod string_config_value {
        use super::*;
        // The class itself isn't translated: it extends `ConfigValue`. Its static parts are below.
        // StringConfigValue.StringConfigValue(3): not translated, its class isn't translated (it extends `ConfigValue`).
        // StringConfigValue.isValid(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // StringConfigValue.loadFromJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
        // StringConfigValue.saveToJson(1): not translated, its class isn't translated (it extends `ConfigValue`).
    }

    /// `ClothConfigCompat` (vectorwing/farmersdelight/refabricated/mlconfigs/integration/cloth_config/ClothConfigCompat.java)
    pub mod cloth_config_compat {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClothConfigCompat {
        }
        impl ClothConfigCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClothConfigCompat.makeScreen(2): not translated, it uses Minecraft's `Screen`.
        // ClothConfigCompat.makeScreen(3): not translated, it uses Minecraft's `Screen`.
        // ClothConfigCompat.addEntriesRecursive(3): not translated, it uses ConfigBuilder, which isn't translated (a function kept in a variable or field (only parameters can be)).
        // ClothConfigCompat.buildEntry(2): not translated, it uses `AbstractConfigListEntry` (me.shedaniel.clothconfig2.api).
        // ClothConfigCompat.addEnum(2): not translated, it's generic.
    }

    /// `YACLCompat` (vectorwing/farmersdelight/refabricated/mlconfigs/integration/yacl/YACLCompat.java)
    pub mod yacl_compat {
        use super::*;
        // DOUBLE_FORMATTER: not translated, it uses `ValueFormatter` (dev.isxander.yacl3.api.controller).
        // FLOAT_FORMATTER: not translated, it uses `ValueFormatter` (dev.isxander.yacl3.api.controller).
        #[derive(Clone, Debug)]
        pub struct YACLCompat {
        }
        impl YACLCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // YACLCompat.makeScreen(2): not translated, it uses Minecraft's `Screen`.
        // YACLCompat.makeScreen(3): not translated, it uses Minecraft's `Screen`.
        // YACLCompat.addEntriesRecursive(3): not translated, it uses `Builder` (dev.isxander.yacl3.api.Option).
        // YACLCompat.buildEntry(1): not translated, it uses `Option` (dev.isxander.yacl3.api).
        // YACLCompat.addEnum(1): not translated, it's generic.
    }

    /// `ModConfigHolder` (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java)
    pub mod mod_config_holder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // CONFIG_STORAGE: not translated, it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.addTrackedSpec(1): not translated, it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.getTrackedSpecs(0): not translated, it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.getConfigSpec(1): not translated, it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.ModConfigHolder(5): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.getReadableName(1): not translated, String.capitalize with 0 values.
        // ModConfigHolder.getReadableName(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.onRefresh(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.isLoaded(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.forceLoad(0): not translated, it has no body.
        // ModConfigHolder.getConfigType(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.getModId(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.getId(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.isSynced(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.getFileName(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.getFullPath(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.loadFromBytes(1): not translated, it has no body.
        // ModConfigHolder.makeScreen(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.makeScreen(2): not translated, it has no body.
        // ModConfigHolder.hasConfigScreen(0): not translated, it has no body.
        // ModConfigHolder.syncConfigsToPlayer(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ModConfigHolder.sendSyncedConfigsToAllPlayers(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `ModConfigHolder.ConfigLoadingException` (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java)
    pub mod mod_config_holder_config_loading_exception {
        use super::*;
        // The class itself isn't translated: it extends `RuntimeException`. Its static parts are below.
        // ModConfigHolder.ConfigLoadingException.ConfigLoadingException(2): not translated, its class isn't translated (it extends `RuntimeException`).
    }

    /// `RegUtils` (vectorwing/farmersdelight/refabricated/RegUtils.java)
    pub mod reg_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RegUtils {
        }
        impl RegUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RegUtils.register(3): not translated, it's generic.
        // RegUtils.regEntity(2): not translated, it's generic.
        // RegUtils.regMenu(2): not translated, it's generic.
        // RegUtils.regPlacementMod(2): not translated, it's generic.
        // RegUtils.regRecipeSerializer(2): not translated, it's generic.
        // RegUtils.regRecipe(2): not translated, it's generic.
        // RegUtils.regParticle(2): not translated, it's generic.
        // RegUtils.regSound(2): not translated, it's generic.
        // RegUtils.regFeature(2): not translated, it's generic.
        // RegUtils.regBlockEntity(2): not translated, it's generic.
        // RegUtils.regTab(2): not translated, it's generic.
        // RegUtils.regComponent(2): not translated, it's generic.
        // RegUtils.regComponent(2): not translated, it's generic.
        // RegUtils.regEnchComponent(2): not translated, it's generic.
        // RegUtils.regEffect(2): not translated, it's generic.
        // RegUtils.regLootFunc(2): not translated, it's generic.
        // RegUtils.regItem(2): not translated, it's generic.
        // RegUtils.regBlock(2): not translated, it's generic.
        // RegUtils.regTrigger(2): not translated, it's generic.
        // RegUtils.regLootCond(2): not translated, it's generic.
        // RegUtils.regRecipeBookCategory(2): not translated, it's generic.
        // RegUtils.regConsumeEffectType(2): not translated, it's generic.
        // RegUtils.regRecipeDisplay(2): not translated, it's generic.
    }

    
    // ---- Gson (com.google.gson): JSON values, a reader and Gson's writer ----
    // JsonElement, JsonObject, JsonArray and JsonPrimitive are all one Rust type. Numbers keep their text, as
    // Gson's LazilyParsedNumber does. Objects keep their insertion order, as Gson's LinkedTreeMap does.
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum JJson {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<JJson>),
        Obj(Vec<(String, JJson)>),
    }
    
    /// A float or double as Java prints it (Float.toString / Double.toString).
    pub fn j_java_num(x: f64, float: bool) -> String {
        if x.is_nan() {
            return "NaN".into();
        }
        if x.is_infinite() {
            return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
        }
        let a = x.abs();
        if x == 0.0 {
            return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
        }
        let digits = if float { format!("{:e}", x as f32) } else { format!("{:e}", x) };
        let (mant, exp) = digits.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        if (1e-3..1e7).contains(&a) {
            let plain = if float { format!("{}", x as f32) } else { format!("{}", x) };
            if plain.contains('.') { plain } else { plain + ".0" }
        } else {
            let mant = if mant.contains('.') { mant.to_string() } else { format!("{mant}.0") };
            format!("{mant}E{exp}")
        }
    }
    
    struct JJsonReader<'a> {
        s: &'a [char],
        i: usize,
    }
    
    impl JJsonReader<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && self.s[self.i].is_whitespace() {
                self.i += 1;
            }
        }
        fn fail(&self, what: &str) -> ! {
            panic!("JsonSyntaxException: {what} at {}", self.i)
        }
        fn value(&mut self) -> JJson {
            self.ws();
            match self.s.get(self.i) {
                Some('{') => {
                    self.i += 1;
                    let mut members = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&'}') {
                        self.i += 1;
                        return JJson::Obj(members);
                    }
                    loop {
                        self.ws();
                        let k = match self.s.get(self.i) {
                            Some('"') | Some('\'') => self.string(),
                            _ => self.bare(),
                        };
                        self.ws();
                        if !matches!(self.s.get(self.i), Some(':') | Some('=')) {
                            self.fail("expected ':'");
                        }
                        self.i += 1;
                        let v = self.value();
                        // A repeated name: Gson keeps the last one.
                        members.retain(|(n, _): &(String, JJson)| *n != k);
                        members.push((k, v));
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some('}') => {
                                self.i += 1;
                                return JJson::Obj(members);
                            }
                            _ => self.fail("expected ',' or '}'"),
                        }
                    }
                }
                Some('[') => {
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&']') {
                        self.i += 1;
                        return JJson::Arr(items);
                    }
                    loop {
                        items.push(self.value());
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some(']') => {
                                self.i += 1;
                                return JJson::Arr(items);
                            }
                            _ => self.fail("expected ',' or ']'"),
                        }
                    }
                }
                Some('"') | Some('\'') => JJson::Str(self.string()),
                Some(_) => {
                    let word = self.bare();
                    match word.as_str() {
                        "true" => JJson::Bool(true),
                        "false" => JJson::Bool(false),
                        "null" => JJson::Null,
                        w if w.parse::<f64>().is_ok() => JJson::Num(w.to_string()),
                        // Gson is lenient: an unquoted word is a string.
                        w => JJson::Str(w.to_string()),
                    }
                }
                None => self.fail("end of input"),
            }
        }
        fn bare(&mut self) -> String {
            let start = self.i;
            while self.i < self.s.len() && !matches!(self.s[self.i], ',' | ':' | '=' | ';' | '}' | ']' | '{' | '[' | '"' | '\'') && !self.s[self.i].is_whitespace() {
                self.i += 1;
            }
            if start == self.i {
                self.fail("expected a value");
            }
            self.s[start..self.i].iter().collect()
        }
        fn string(&mut self) -> String {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            loop {
                let Some(&c) = self.s.get(self.i) else { self.fail("unterminated string") };
                self.i += 1;
                match c {
                    c if c == q => return out,
                    '\\' => {
                        let Some(&e) = self.s.get(self.i) else { self.fail("unterminated string") };
                        self.i += 1;
                        match e {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            'r' => out.push('\r'),
                            'b' => out.push('\u{8}'),
                            'f' => out.push('\u{c}'),
                            'u' => {
                                let hex: String = self.s[self.i..(self.i + 4).min(self.s.len())].iter().collect();
                                self.i += 4;
                                let unit = u16::from_str_radix(&hex, 16).unwrap_or_else(|_| self.fail("bad \\u escape"));
                                // A surrogate pair: two escapes make one char.
                                if (0xD800..0xDC00).contains(&unit) && self.s.get(self.i) == Some(&'\\') && self.s.get(self.i + 1) == Some(&'u') {
                                    let hex2: String = self.s[self.i + 2..(self.i + 6).min(self.s.len())].iter().collect();
                                    if let Ok(low) = u16::from_str_radix(&hex2, 16) {
                                        self.i += 6;
                                        out.extend(char::decode_utf16([unit, low]).map(|r| r.unwrap_or('\u{FFFD}')));
                                        continue;
                                    }
                                }
                                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                            }
                            other => out.push(other),
                        }
                    }
                    c => out.push(c),
                }
            }
        }
    }
    
    impl JJson {
        /// JsonParser.parseString: panics (Java throws JsonSyntaxException) on bad JSON.
        pub fn parse(s: &str) -> JJson {
            let chars: Vec<char> = s.chars().collect();
            let mut r = JJsonReader { s: &chars, i: 0 };
            let v = r.value();
            r.ws();
            if r.i != chars.len() {
                r.fail("more after the value");
            }
            v
        }
        pub fn object() -> JJson {
            JJson::Obj(Vec::new())
        }
        pub fn array() -> JJson {
            JJson::Arr(Vec::new())
        }
        pub fn int(v: i64) -> JJson {
            JJson::Num(v.to_string())
        }
        pub fn float(v: f32) -> JJson {
            JJson::Num(j_java_num(v as f64, true))
        }
        pub fn double(v: f64) -> JJson {
            JJson::Num(j_java_num(v, false))
        }
        pub fn str(v: &str) -> JJson {
            JJson::Str(v.to_string())
        }
        pub fn is_json_object(&self) -> bool {
            matches!(self, JJson::Obj(_))
        }
        pub fn is_json_array(&self) -> bool {
            matches!(self, JJson::Arr(_))
        }
        pub fn is_json_null(&self) -> bool {
            matches!(self, JJson::Null)
        }
        pub fn is_json_primitive(&self) -> bool {
            matches!(self, JJson::Bool(_) | JJson::Num(_) | JJson::Str(_))
        }
        pub fn is_string(&self) -> bool {
            matches!(self, JJson::Str(_))
        }
        pub fn is_number(&self) -> bool {
            matches!(self, JJson::Num(_))
        }
        pub fn is_boolean(&self) -> bool {
            matches!(self, JJson::Bool(_))
        }
        /// getAsJsonObject() / getAsJsonArray(): Java throws IllegalStateException on the wrong kind.
        pub fn as_object(&self) -> JJson {
            assert!(self.is_json_object(), "Not a JSON Object: {self}");
            self.clone()
        }
        pub fn as_array(&self) -> JJson {
            assert!(self.is_json_array(), "Not a JSON Array: {self}");
            self.clone()
        }
        fn members(&self) -> &Vec<(String, JJson)> {
            match self {
                JJson::Obj(m) => m,
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        /// The (name, value) pairs of an object, as entrySet() walks them.
        pub fn entries(&self) -> impl Iterator<Item = (&String, &JJson)> {
            self.members().iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &String> {
            self.members().iter().map(|(k, _)| k)
        }
        pub fn elements(&self) -> impl Iterator<Item = &JJson> {
            match self {
                JJson::Arr(a) => a.iter(),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn has(&self, k: &str) -> bool {
            self.members().iter().any(|(n, _)| n == k)
        }
        /// JsonObject.get: None (null) when absent.
        pub fn get(&self, k: &str) -> Option<JJson> {
            self.members().iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
        }
        /// JsonObject.getAsJsonObject(name) / getAsJsonArray(name): null when absent, a cast that throws on the wrong kind.
        pub fn member_object(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_object())
        }
        pub fn member_array(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_array())
        }
        /// JsonArray.get(i).
        pub fn at(&self, i: i32) -> JJson {
            match self {
                JJson::Arr(a) => a.get(i as usize).cloned().unwrap_or_else(|| panic!("Index {i} out of bounds for length {}", a.len())),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn size(&self) -> i32 {
            match self {
                JJson::Arr(a) => a.len() as i32,
                JJson::Obj(m) => m.len() as i32,
                _ => panic!("size() of a JSON primitive"),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        /// The single value of a one-element array, as Gson's JsonArray.getAsX does.
        fn single(&self) -> &JJson {
            match self {
                JJson::Arr(a) if a.len() == 1 => &a[0],
                JJson::Arr(_) => panic!("Array must have size 1"),
                other => other,
            }
        }
        pub fn as_string(&self) -> String {
            match self.single() {
                JJson::Str(s) => s.clone(),
                JJson::Num(n) => n.clone(),
                JJson::Bool(b) => b.to_string(),
                other => panic!("UnsupportedOperationException: {other} as a string"),
            }
        }
        fn number_text(&self) -> String {
            match self.single() {
                JJson::Num(n) | JJson::Str(n) => n.trim().to_string(),
                other => panic!("UnsupportedOperationException: {other} as a number"),
            }
        }
        pub fn as_f64(&self) -> f64 {
            let t = self.number_text();
            t.parse().unwrap_or_else(|_| panic!("NumberFormatException: {t}"))
        }
        pub fn as_float(&self) -> f32 {
            self.as_f64() as f32
        }
        pub fn as_double(&self) -> f64 {
            self.as_f64()
        }
        /// getAsInt / getAsLong: a whole number, else the number cut to one.
        pub fn as_long(&self) -> i64 {
            let t = self.number_text();
            t.parse::<i64>().unwrap_or_else(|_| t.parse::<f64>().map(|f| f as i64).unwrap_or_else(|_| panic!("NumberFormatException: {t}")))
        }
        pub fn as_int(&self) -> i32 {
            self.as_long() as i32
        }
        pub fn as_bool(&self) -> bool {
            match self.single() {
                JJson::Bool(b) => *b,
                // Gson: Boolean.parseBoolean of the text
                other => other.as_string().eq_ignore_ascii_case("true"),
            }
        }
        pub fn add(&mut self, k: &str, v: JJson) {
            match self {
                JJson::Obj(m) => {
                    m.retain(|(n, _)| n != k);
                    m.push((k.to_string(), v));
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn remove(&mut self, k: &str) -> Option<JJson> {
            match self {
                JJson::Obj(m) => {
                    let i = m.iter().position(|(n, _)| n == k)?;
                    Some(m.remove(i).1)
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn push(&mut self, v: JJson) {
            match self {
                JJson::Arr(a) => a.push(v),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        fn write(&self, out: &mut String, pretty: bool, html: bool, depth: usize) {
            let nl = |out: &mut String, d: usize| {
                if pretty {
                    out.push('\n');
                    out.push_str(&"  ".repeat(d));
                }
            };
            match self {
                JJson::Null => out.push_str("null"),
                JJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                JJson::Num(n) => out.push_str(n),
                JJson::Str(s) => j_json_quote(s, html, out),
                JJson::Arr(a) => {
                    out.push('[');
                    for (i, v) in a.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !a.is_empty() {
                        nl(out, depth);
                    }
                    out.push(']');
                }
                JJson::Obj(m) => {
                    out.push('{');
                    for (i, (k, v)) in m.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        j_json_quote(k, html, out);
                        out.push(':');
                        if pretty {
                            out.push(' ');
                        }
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !m.is_empty() {
                        nl(out, depth);
                    }
                    out.push('}');
                }
            }
        }
    }
    
    fn j_json_quote(s: &str, html: bool, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                '<' | '>' | '&' | '=' | '\'' if html => out.push_str(&format!("\\u{:04x}", c as u32)),
                '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }
    
    /// JsonElement.toString: compact, without Gson's HTML escaping.
    impl std::fmt::Display for JJson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut out = String::new();
            self.write(&mut out, false, false, 0);
            f.write_str(&out)
        }
    }
    
    /// A Gson (from `new Gson()` or a GsonBuilder): how it writes.
    #[derive(Clone, Copy, Debug)]
    pub struct JGson {
        pub pretty: bool,
        pub html: bool,
        pub nulls: bool,
    }
    
    impl Default for JGson {
        fn default() -> Self {
            JGson { pretty: false, html: true, nulls: false }
        }
    }
    
    impl JGson {
        /// Gson.toJson.
        pub fn to_json(&self, v: &JJson) -> String {
            let mut out = String::new();
            self.strip(v).write(&mut out, self.pretty, self.html, 0);
            out
        }
        /// Without serializeNulls, Gson leaves out null fields of objects.
        fn strip(&self, v: &JJson) -> JJson {
            match v {
                JJson::Obj(m) => JJson::Obj(m.iter().filter(|(_, x)| self.nulls || !x.is_json_null()).map(|(k, x)| (k.clone(), self.strip(x))).collect()),
                JJson::Arr(a) => JJson::Arr(a.iter().map(|x| self.strip(x)).collect()),
                other => other.clone(),
            }
        }
    }
    
    #[cfg(test)]
    mod jjson_tests {
        use super::*;
    
        #[test]
        fn read_and_write() {
            let j = JJson::parse("{\"a\": 1, \"b\": [1.5, \"x\", true, null], \"c\": {\"d\": -2e3}}");
            assert!(j.has("a") && !j.has("z"));
            assert_eq!(j.get("a").unwrap().as_int(), 1);
            assert_eq!(j.member_array("b").unwrap().at(0).as_float(), 1.5);
            assert_eq!(j.member_array("b").unwrap().size(), 4);
            assert_eq!(j.member_object("c").unwrap().get("d").unwrap().as_double(), -2000.0);
            assert_eq!(j.get("z"), None);
            assert_eq!(j.to_string(), "{\"a\":1,\"b\":[1.5,\"x\",true,null],\"c\":{\"d\":-2e3}}");
            let g = JGson { pretty: true, ..JGson::default() };
            assert_eq!(g.to_json(&JJson::parse("{\"a\":[1,2],\"b\":{},\"n\":null,\"s\":\"<x>\"}")),
                       "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"s\": \"\\u003cx\\u003e\"\n}");
        }
    
        #[test]
        fn java_numbers() {
            assert_eq!(j_java_num(1.0, true), "1.0");
            assert_eq!(j_java_num(0.1, true), "0.1");
            assert_eq!(j_java_num(1.0e-4, false), "1.0E-4");
            assert_eq!(j_java_num(12345678.0, false), "1.2345678E7");
            assert_eq!(j_java_num(-0.75, false), "-0.75");
        }
    }

    
    // ---- java.util.Properties: `key=value` text files ----
    
    #[derive(Clone, Debug, Default)]
    pub struct JProps(pub std::collections::BTreeMap<String, String>);
    
    fn j_props_unescape(s: &str) -> String {
        let mut out = String::new();
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            match it.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('f') => out.push('\u{c}'),
                Some('u') => {
                    let hex: String = (0..4).filter_map(|_| it.next()).collect();
                    out.push(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32).unwrap_or('\u{FFFD}'));
                }
                Some(o) => out.push(o),
                None => {}
            }
        }
        out
    }
    
    fn j_props_escape(s: &str, key: bool) -> String {
        let mut out = String::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '\\' => out.push_str("\\\\"),
                '\t' => out.push_str("\\t"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\u{c}' => out.push_str("\\f"),
                '=' | ':' | '#' | '!' => {
                    out.push('\\');
                    out.push(c);
                }
                ' ' if key || i == 0 => out.push_str("\\ "),
                c if (c as u32) < 0x20 || (c as u32) > 0x7e => out.push_str(&format!("\\u{:04X}", c as u32)),
                c => out.push(c),
            }
        }
        out
    }
    
    impl JProps {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn get_property(&self, k: &str) -> Option<String> {
            self.0.get(k).cloned()
        }
        pub fn get_property_or(&self, k: &str, default: &str) -> String {
            self.0.get(k).cloned().unwrap_or_else(|| default.to_string())
        }
        pub fn set_property(&mut self, k: &str, v: &str) -> Option<String> {
            self.0.insert(k.to_string(), v.to_string())
        }
        pub fn contains_key(&self, k: &str) -> bool {
            self.0.contains_key(k)
        }
        pub fn remove(&mut self, k: &str) -> Option<String> {
            self.0.remove(k)
        }
        pub fn names(&self) -> JSet<String> {
            JSet::from_iter(self.0.keys().cloned())
        }
        /// Properties.load: `key=value`, `key: value` or `key value`, `#`/`!` comments, `\` continuations.
        pub fn load(&mut self, text: &str) {
            let mut logical = String::new();
            for raw in text.lines() {
                let line = raw.trim_start();
                if logical.is_empty() && (line.is_empty() || line.starts_with('#') || line.starts_with('!')) {
                    continue;
                }
                let trailing = line.chars().rev().take_while(|&c| c == '\\').count();
                if trailing % 2 == 1 {
                    logical.push_str(&line[..line.len() - 1]);
                    continue;
                }
                logical.push_str(line);
                let chars: Vec<char> = logical.chars().collect();
                let mut i = 0;
                while i < chars.len() && !(matches!(chars[i], '=' | ':') || chars[i].is_whitespace()) {
                    i += if chars[i] == '\\' { 2 } else { 1 };
                }
                let key: String = chars[..i.min(chars.len())].iter().collect();
                let mut j = i;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < chars.len() && matches!(chars[j], '=' | ':') {
                    j += 1;
                }
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                let value: String = chars[j.min(chars.len())..].iter().collect();
                self.0.insert(j_props_unescape(&key), j_props_unescape(&value));
                logical.clear();
            }
        }
        /// Properties.store: the comment, then `key=value` lines (Java also writes the date, left out here).
        pub fn store_text(&self, comment: Option<&str>) -> String {
            let mut out = String::new();
            if let Some(c) = comment {
                for l in c.lines() {
                    out.push('#');
                    out.push_str(l);
                    out.push('\n');
                }
            }
            for (k, v) in &self.0 {
                out.push_str(&j_props_escape(k, true));
                out.push('=');
                out.push_str(&j_props_escape(v, false));
                out.push('\n');
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "farmersdelight");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
