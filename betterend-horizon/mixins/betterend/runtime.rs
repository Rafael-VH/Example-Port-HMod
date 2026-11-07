// Better End, ported to Horizon by horizon-port from better-end-26.302.1.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_betterend). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "betterend";
pub const NAME: &str = "Better End";
pub const VERSION: &str = "26.302.1";
pub const DESCRIPTION: &str = "More content for The End dimension, including new biomes, plants, mobs, mechanics, terrain generation and more. Made by: - paulevs (code & art) - Bulldog83 (code & art) - Edos (building) - Yuki (art) - Seaward (art) - Firel (music) ";

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

    /// `BECriteria` (org/betterx/betterend/advancements/BECriteria.java)
    pub mod be_criteria {
        use super::*;
        // PORTAL_ON_ID: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // PORTAL_TRAVEL_ID: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // INFUSION_FINISHED_ID: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // PORTAL_ON: not translated, it uses Minecraft's `PlayerTrigger`.
        // PORTAL_TRAVEL: not translated, it uses Minecraft's `PlayerTrigger`.
        // INFUSION_FINISHED: not translated, it uses Minecraft's `PlayerTrigger`.
        // PORTAL_ON_TRIGGER: not translated, it uses Minecraft's `Criterion`.
        // PORTAL_TRAVEL_TRIGGER: not translated, it uses Minecraft's `Criterion`.
        // INFUSION_FINISHED_TRIGGER: not translated, it uses Minecraft's `Criterion`.
        #[derive(Clone, Debug)]
        pub struct BECriteria {
        }
        impl BECriteria {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BECriteria.register(0): not translated, it uses BECriteria.PORTAL_ON, which isn't translated (it uses Minecraft's `PlayerTrigger`).
        // BECriteria.register(2): not translated, it's generic.
    }

    /// `BetterEndPlugin` (org/betterx/betterend/api/BetterEndPlugin.java)
    pub mod better_end_plugin {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BetterEndPlugin.registerAlloyingRecipes(0): not translated, its class isn't translated (it's an interface).
        // BetterEndPlugin.registerSmithingRecipes(0): not translated, its class isn't translated (it's an interface).
        // BetterEndPlugin.registerEndBiomes(0): not translated, its class isn't translated (it's an interface).
        // BetterEndPlugin.registerOthers(0): not translated, its class isn't translated (it's an interface).
        // BetterEndPlugin.register(1): not translated, it uses BetterEndPlugin, which isn't translated (it's an interface).
    }

    /// `BetterEnd` (org/betterx/betterend/BetterEnd.java)
    pub mod better_end {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // TRINKETS_CORE: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // MOD_ID: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // LOGGER: the logger. Log calls become comments.
        // NOURISH: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // FLAMBOYANT: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // PATCHOULI: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // GLIMMERWILD: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // HYDROGEN: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // NOURISH_ADDITIONS_PACK: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // FLAMBOYANT_ADDITIONS_PACK: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // PATCHOULI_ADDITIONS_PACK: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // GLIMMERWILD_ADDITIONS_PACK: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // BetterEnd.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
    }

    /// `AeterniumAnvil` (org/betterx/betterend/blocks/AeterniumAnvil.java)
    pub mod aeternium_anvil {
        use super::*;
        // The class itself isn't translated: it extends `LeveledAnvilBlock` (org.betterx.bclib.blocks). Its static parts are below.
        pub const AETERNIUM_ANVIL_LEVEL: i32 = 5;
        // AeterniumAnvil.AeterniumAnvil(1): not translated, its class isn't translated (it extends `LeveledAnvilBlock` (org.betterx.bclib.blocks)).
        // AeterniumAnvil.getMaxDurability(0): not translated, its class isn't translated (it extends `LeveledAnvilBlock` (org.betterx.bclib.blocks)).
    }

    /// `AeterniumBlock` (org/betterx/betterend/blocks/AeterniumBlock.java)
    pub mod aeternium_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // AeterniumBlock.AeterniumBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `AmaranitaCapBlock` (org/betterx/betterend/blocks/AmaranitaCapBlock.java)
    pub mod amaranita_cap_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // AmaranitaCapBlock.AmaranitaCapBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `AmaranitaHymenophoreBlock` (org/betterx/betterend/blocks/AmaranitaHymenophoreBlock.java)
    pub mod amaranita_hymenophore_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // AmaranitaHymenophoreBlock.AmaranitaHymenophoreBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `AmaranitaStemBlock` (org/betterx/betterend/blocks/AmaranitaStemBlock.java)
    pub mod amaranita_stem_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        // AmaranitaStemBlock.AmaranitaStemBlock(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
    }

    /// `AmberBlock` (org/betterx/betterend/blocks/AmberBlock.java)
    pub mod amber_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // AmberBlock.AmberBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `AncientEmeraldIceBlock` (org/betterx/betterend/blocks/AncientEmeraldIceBlock.java)
    pub mod ancient_emerald_ice_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // AncientEmeraldIceBlock.AncientEmeraldIceBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // AncientEmeraldIceBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // AncientEmeraldIceBlock.makeParticles(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // AncientEmeraldIceBlock.stepOn(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `AuroraCrystalBlock` (org/betterx/betterend/blocks/AuroraCrystalBlock.java)
    pub mod aurora_crystal_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TransparentBlock`. Its static parts are below.
        // COLORS: not translated, it uses Minecraft's `Vec3i`.
        // AuroraCrystalBlock.AuroraCrystalBlock(1): not translated, its class isn't translated (it extends Minecraft's `TransparentBlock`).
        // AuroraCrystalBlock.getVisualShape(4): not translated, its class isn't translated (it extends Minecraft's `TransparentBlock`).
    }

    /// `EndLanternBlock` (org/betterx/betterend/blocks/basis/EndLanternBlock.java)
    pub mod end_lantern_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // IS_FLOOR: not translated, it uses Minecraft's `BooleanProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // EndLanternBlock.EndLanternBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLanternBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLanternBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLanternBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLanternBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLanternBlock.canPlaceLiquid(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLanternBlock.placeLiquid(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLanternBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `EndUnderwaterWallPlantBlock` (org/betterx/betterend/blocks/basis/EndUnderwaterWallPlantBlock.java)
    pub mod end_underwater_wall_plant_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // EndUnderwaterWallPlantBlock.EndUnderwaterWallPlantBlock(1): not translated, its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)).
        // EndUnderwaterWallPlantBlock.canPlaceLiquid(5): not translated, its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)).
        // EndUnderwaterWallPlantBlock.placeLiquid(4): not translated, its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)).
        // EndUnderwaterWallPlantBlock.getFluidState(1): not translated, its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)).
        // EndUnderwaterWallPlantBlock.canSurvive(3): not translated, its class isn't translated (it extends `BaseWallPlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `FurBlock` (org/betterx/betterend/blocks/basis/FurBlock.java)
    pub mod fur_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseAttachedBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // FurBlock.FurBlock(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // FurBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // FurBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // FurBlock.getFluidState(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // FurBlock.updateShape(8): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // FurBlock.getShape(4): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
    }

    /// `PedestalBlock` (org/betterx/betterend/blocks/basis/PedestalBlock.java)
    pub mod pedestal_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // STATE: not translated, it uses Minecraft's `EnumProperty`.
        // HAS_ITEM: not translated, it uses Minecraft's `BooleanProperty`.
        // HAS_LIGHT: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE_DEFAULT: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_COLUMN: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_PILLAR: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_PEDESTAL_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_COLUMN_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // PedestalBlock.PedestalBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.getHeight(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.destroy(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.checkRitual(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.getUpdatedState(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.getDrops(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.moveStoredStack(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.moveStoredStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.dropStoredStack(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.getDropPos(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.defaultShape(0): not translated, it uses Minecraft's `VoxelShape`.
        // PedestalBlock.isPlaceable(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.hasUniqueEntity(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PedestalBlock.provideBlockModel(3): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
        // PedestalBlock.provideBlockModel(4): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
        // PedestalBlock.createTextureMapping(1): not translated, it uses Minecraft's `TextureMapping`.
    }

    /// `PedestalBlock.ClientHooks` (org/betterx/betterend/blocks/basis/PedestalBlock.java)
    pub mod pedestal_block_client_hooks {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientHooks {
        }
        impl ClientHooks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PedestalBlock.ClientHooks.onSocketFilled(3): not translated, it uses Minecraft's `Level`.
    }

    /// `PedestalBlock.ClientModelBuilder` (org/betterx/betterend/blocks/basis/PedestalBlock.java)
    pub mod pedestal_block_client_model_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModelBuilder {
        }
        impl ClientModelBuilder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PedestalBlock.ClientModelBuilder.buildBlockModel(4): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
    }

    /// `PedestalBlock.Models` (org/betterx/betterend/blocks/basis/PedestalBlock.java)
    pub mod pedestal_block_models {
        use super::*;
        // PEDESTAL_MODELS: not translated, it uses PedestalState, which isn't translated (its constants carry values).
        #[derive(Clone, Debug)]
        pub struct Models {
        }
        impl Models {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `StoneLanternBlock` (org/betterx/betterend/blocks/basis/StoneLanternBlock.java)
    pub mod stone_lantern_block {
        use super::*;
        // The class itself isn't translated: it extends `EndLanternBlock`. Its static parts are below.
        // SHAPE_CEIL: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_FLOOR: not translated, it uses Minecraft's `VoxelShape`.
        // MODEL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // StoneLanternBlock.StoneLanternBlock(1): not translated, its class isn't translated (it extends `EndLanternBlock`).
        // StoneLanternBlock.getShape(4): not translated, its class isn't translated (it extends `EndLanternBlock`).
        // StoneLanternBlock.buildModel(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // StoneLanternBlock.provideBlockModel(3): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
    }

    /// `StoneLanternBlock.ClientModel` (org/betterx/betterend/blocks/basis/StoneLanternBlock.java)
    pub mod stone_lantern_block_client_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModel {
        }
        impl ClientModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StoneLanternBlock.ClientModel.buildBlockModel(3): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
        // StoneLanternBlock.ClientModel.build(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `BlueVineBlock` (org/betterx/betterend/blocks/BlueVineBlock.java)
    pub mod blue_vine_block {
        use super::*;
        // The class itself isn't translated: it extends `UpDownPlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlueVineBlock.BlueVineBlock(1): not translated, its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)).
        // BlueVineBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)).
        // BlueVineBlock.isTerrain(1): not translated, its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `BlueVineLanternBlock` (org/betterx/betterend/blocks/BlueVineLanternBlock.java)
    pub mod blue_vine_lantern_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // NATURAL: not translated, it uses Minecraft's `BooleanProperty`.
        // BlueVineLanternBlock.BlueVineLanternBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlueVineLanternBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlueVineLanternBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlueVineLanternBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlueVineLanternBlock.provideBlockModel(2): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
    }

    /// `BlueVineSeedBlock` (org/betterx/betterend/blocks/BlueVineSeedBlock.java)
    pub mod blue_vine_seed_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BlueVineSeedBlock.BlueVineSeedBlock(1): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // BlueVineSeedBlock.growAdult(3): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // BlueVineSeedBlock.placeLantern(2): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `BoluxMushroomBlock` (org/betterx/betterend/blocks/BoluxMushroomBlock.java)
    pub mod bolux_mushroom_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BoluxMushroomBlock.BoluxMushroomBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // BoluxMushroomBlock.getShape(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // BoluxMushroomBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // BoluxMushroomBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `BrimstoneBlock` (org/betterx/betterend/blocks/BrimstoneBlock.java)
    pub mod brimstone_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // ACTIVATED: not translated, it uses Minecraft's `BooleanProperty`.
        // BrimstoneBlock.BrimstoneBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrimstoneBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrimstoneBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrimstoneBlock.destroy(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrimstoneBlock.updateChunks(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrimstoneBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BubbleCoralBlock` (org/betterx/betterend/blocks/BubbleCoralBlock.java)
    pub mod bubble_coral_block {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BubbleCoralBlock.BubbleCoralBlock(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // BubbleCoralBlock.animateTick(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // BubbleCoralBlock.getShape(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // BubbleCoralBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // BubbleCoralBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
    }

    /// `BuddingSmaragdantCrystalBlock` (org/betterx/betterend/blocks/BuddingSmaragdantCrystalBlock.java)
    pub mod budding_smaragdant_crystal_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        // BuddingSmaragdantCrystalBlock.BuddingSmaragdantCrystalBlock(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // BuddingSmaragdantCrystalBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // BuddingSmaragdantCrystalBlock.canShardGrowAtState(1): not translated, it uses Minecraft's `BlockState`.
    }

    /// `BulbVineBlock` (org/betterx/betterend/blocks/BulbVineBlock.java)
    pub mod bulb_vine_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseVineBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BulbVineBlock.BulbVineBlock(1): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // BulbVineBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // BulbVineBlock.canSurvive(3): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // BulbVineBlock.buildLoot(2): not translated, it uses Builder, which isn't translated (it's an interface).
    }

    /// `BulbVineLanternBlock` (org/betterx/betterend/blocks/BulbVineLanternBlock.java)
    pub mod bulb_vine_lantern_block {
        use super::*;
        // The class itself isn't translated: it extends `EndLanternBlock` (org.betterx.betterend.blocks.basis). Its static parts are below.
        // SHAPE_CEIL: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_FLOOR: not translated, it uses Minecraft's `VoxelShape`.
        // MODEL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // BulbVineLanternBlock.BulbVineLanternBlock(1): not translated, its class isn't translated (it extends `EndLanternBlock` (org.betterx.betterend.blocks.basis)).
        // BulbVineLanternBlock.getShape(4): not translated, its class isn't translated (it extends `EndLanternBlock` (org.betterx.betterend.blocks.basis)).
        pub fn get_metal_texture(block_id: &JIdentifier) -> String {
            let mut name: String = block_id.path.clone();
            name = j_substring(&name, 0, Some(j_index_of_char(&name, char::from_u32(95 as u32).unwrap_or('\0'))));
            return format!("{}_bulb_vine_lantern_metal", name);
        }
        // BulbVineLanternBlock.getGlowTexture(0): not translated, its class isn't translated (it extends `EndLanternBlock` (org.betterx.betterend.blocks.basis)).
        // BulbVineLanternBlock.buildModel(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
    }

    /// `BulbVineLanternBlock.ClientModel` (org/betterx/betterend/blocks/BulbVineLanternBlock.java)
    pub mod bulb_vine_lantern_block_client_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModel {
        }
        impl ClientModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BulbVineLanternBlock.ClientModel.build(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `BulbVineLanternColoredBlock` (org/betterx/betterend/blocks/BulbVineLanternColoredBlock.java)
    pub mod bulb_vine_lantern_colored_block {
        use super::*;
        // The class itself isn't translated: it extends `BulbVineLanternBlock`. Its static parts are below.
        // BulbVineLanternColoredBlock.boostLanternColor(1): not translated, it calls `ColorUtilARGB32` (de.ambertation.wunderlib.utils).RGBtoHSB.
        // BulbVineLanternColoredBlock.BulbVineLanternColoredBlock(1): not translated, its class isn't translated (it extends `BulbVineLanternBlock`).
        // BulbVineLanternColoredBlock.getGlowTexture(0): not translated, its class isn't translated (it extends `BulbVineLanternBlock`).
    }

    /// `BulbVineSeedBlock` (org/betterx/betterend/blocks/BulbVineSeedBlock.java)
    pub mod bulb_vine_seed_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BulbVineSeedBlock.BulbVineSeedBlock(1): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // BulbVineSeedBlock.canSurvive(3): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // BulbVineSeedBlock.growAdult(3): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `CaveMossBlock` (org/betterx/betterend/blocks/CaveMossBlock.java)
    pub mod cave_moss_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // CaveMossBlock.CaveMossBlock(2): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // CaveMossBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // CaveMossBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // CaveMossBlock.useWithoutItem(5): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // CaveMossBlock.randomTick(4): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // CaveMossBlock.canStayBottom(2): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // CaveMossBlock.isMiddle(1): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
    }

    /// `CavePumpkinBlock` (org/betterx/betterend/blocks/CavePumpkinBlock.java)
    pub mod cave_pumpkin_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SMALL: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE_SMALL: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BIG: not translated, it uses Minecraft's `VoxelShape`.
        // CavePumpkinBlock.CavePumpkinBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CavePumpkinBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CavePumpkinBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CavePumpkinBlock.buildLoot(2): not translated, it uses Minecraft's `Builder`.
    }

    /// `CavePumpkinVineBlock` (org/betterx/betterend/blocks/CavePumpkinVineBlock.java)
    pub mod cave_pumpkin_vine_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // CavePumpkinVineBlock.CavePumpkinVineBlock(1): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // CavePumpkinVineBlock.canSurvive(3): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // CavePumpkinVineBlock.performBonemeal(5): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // CavePumpkinVineBlock.growAdult(3): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // CavePumpkinVineBlock.updateShape(8): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // CavePumpkinVineBlock.getShape(4): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `ChandelierBlock` (org/betterx/betterend/blocks/ChandelierBlock.java)
    pub mod chandelier_block {
        use super::*;
        // The class itself isn't translated: it extends `Metal` (org.betterx.bclib.blocks.BaseAttachedBlock). Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        // MODEL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // ChandelierBlock.ChandelierBlock(1): not translated, its class isn't translated (it extends `Metal` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // ChandelierBlock.ChandelierBlock(1): not translated, its class isn't translated (it extends `Metal` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // ChandelierBlock.getShape(4): not translated, its class isn't translated (it extends `Metal` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // ChandelierBlock.buildModel(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
    }

    /// `ChandelierBlock.ClientModel` (org/betterx/betterend/blocks/ChandelierBlock.java)
    pub mod chandelier_block_client_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModel {
        }
        impl ClientModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChandelierBlock.ClientModel.build(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `CharcoalBlock` (org/betterx/betterend/blocks/CharcoalBlock.java)
    pub mod charcoal_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // CharcoalBlock.CharcoalBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CharcoalBlock.getFuelTime(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `CharniaBlock` (org/betterx/betterend/blocks/CharniaBlock.java)
    pub mod charnia_block {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants). Its static parts are below.
        // CharniaBlock.CharniaBlock(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // CharniaBlock.isTerrain(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
    }

    /// `CrystalMossCoverBlock` (org/betterx/betterend/blocks/CrystalMossCoverBlock.java)
    pub mod crystal_moss_cover_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MultifaceSpreadeableBlock`. Its static parts are below.
        // CrystalMossCoverBlock.CrystalMossCoverBlock(1): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
        // CrystalMossCoverBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
        // CrystalMossCoverBlock.canBeReplaced(2): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
        // CrystalMossCoverBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
        // CrystalMossCoverBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
        // CrystalMossCoverBlock.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
        // CrystalMossCoverBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
        // CrystalMossCoverBlock.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
        // CrystalMossCoverBlock.getSpreader(0): not translated, its class isn't translated (it extends Minecraft's `MultifaceSpreadeableBlock`).
    }

    /// `DragonTreeSaplingBlock` (org/betterx/betterend/blocks/DragonTreeSaplingBlock.java)
    pub mod dragon_tree_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // DragonTreeSaplingBlock.DragonTreeSaplingBlock(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `EmeraldIceBlock` (org/betterx/betterend/blocks/EmeraldIceBlock.java)
    pub mod emerald_ice_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HalfTransparentBlock`. Its static parts are below.
        // EmeraldIceBlock.EmeraldIceBlock(1): not translated, its class isn't translated (it extends Minecraft's `HalfTransparentBlock`).
        // EmeraldIceBlock.playerDestroy(6): not translated, its class isn't translated (it extends Minecraft's `HalfTransparentBlock`).
        // EmeraldIceBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `HalfTransparentBlock`).
        // EmeraldIceBlock.melt(2): not translated, its class isn't translated (it extends Minecraft's `HalfTransparentBlock`).
    }

    /// `EndBlockProperties` (org/betterx/betterend/blocks/EndBlockProperties.java)
    pub mod end_block_properties {
        use super::*;
        // The class itself isn't translated: it extends `BlockProperties` (de.ambertation.wover.block.api). Its static parts are below.
        // HYDRALUX_SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // PEDESTAL_STATE: not translated, it uses Minecraft's `EnumProperty`.
        // CACTUS_BOTTOM: not translated, it uses Minecraft's `EnumProperty`.
        // PORTAL: not translated, it uses Minecraft's `IntegerProperty`.
        // PLANT_ID: not translated, it uses Minecraft's `IntegerProperty`.
        // SOIL_ID: not translated, it uses Minecraft's `IntegerProperty`.
        // POT_LIGHT: not translated, it uses Minecraft's `IntegerProperty`.
        // HAS_ITEM: not translated, it uses Minecraft's `BooleanProperty`.
    }

    /// `EndBlockProperties.CactusBottom` (org/betterx/betterend/blocks/EndBlockProperties.java)
    pub mod end_block_properties_cactus_bottom {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EndBlockProperties.CactusBottom.CactusBottom(1): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.CactusBottom.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.CactusBottom.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EndBlockProperties.HydraluxShape` (org/betterx/betterend/blocks/EndBlockProperties.java)
    pub mod end_block_properties_hydralux_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EndBlockProperties.HydraluxShape.HydraluxShape(2): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.HydraluxShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.HydraluxShape.toString(0): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.HydraluxShape.hasGlow(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EndBlockProperties.LumecornShape` (org/betterx/betterend/blocks/EndBlockProperties.java)
    pub mod end_block_properties_lumecorn_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EndBlockProperties.LumecornShape.LumecornShape(2): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.LumecornShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.LumecornShape.toString(0): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.LumecornShape.getLight(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EndBlockProperties.PedestalState` (org/betterx/betterend/blocks/EndBlockProperties.java)
    pub mod end_block_properties_pedestal_state {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EndBlockProperties.PedestalState.PedestalState(1): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.PedestalState.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // EndBlockProperties.PedestalState.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EnderBlock` (org/betterx/betterend/blocks/EnderBlock.java)
    pub mod ender_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // EnderBlock.EnderBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EnderBlock.getColor(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `EndLilyBlock` (org/betterx/betterend/blocks/EndLilyBlock.java)
    pub mod end_lily_block {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // EndLilyBlock.EndLilyBlock(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // EndLilyBlock.updateShape(8): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // EndLilyBlock.getShape(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // EndLilyBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // EndLilyBlock.getFluidState(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // EndLilyBlock.canSurvive(3): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // EndLilyBlock.buildLoot(2): not translated, it uses Minecraft's `Builder`.
        // EndLilyBlock.isTop(2): not translated, it uses Minecraft's `Builder`.
        // EndLilyBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // EndLilyBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // EndLilyBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
    }

    /// `EndLilySeedBlock` (org/betterx/betterend/blocks/EndLilySeedBlock.java)
    pub mod end_lily_seed_block {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // EndLilySeedBlock.EndLilySeedBlock(1): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // EndLilySeedBlock.grow(3): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // EndLilySeedBlock.canGrow(2): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // EndLilySeedBlock.isTerrain(1): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `EndLotusFlowerBlock` (org/betterx/betterend/blocks/EndLotusFlowerBlock.java)
    pub mod end_lotus_flower_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE_OUTLINE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_COLLISION: not translated, it uses Minecraft's `VoxelShape`.
        // EndLotusFlowerBlock.EndLotusFlowerBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // EndLotusFlowerBlock.getShape(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // EndLotusFlowerBlock.getCollisionShape(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // EndLotusFlowerBlock.buildLoot(2): not translated, it uses Builder, which isn't translated (it's an interface).
        // EndLotusFlowerBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `EndLotusLeafBlock` (org/betterx/betterend/blocks/EndLotusLeafBlock.java)
    pub mod end_lotus_leaf_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // HORIZONTAL_FACING: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // VSHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // EndLotusLeafBlock.EndLotusLeafBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusLeafBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusLeafBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusLeafBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusLeafBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusLeafBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusLeafBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `EndLotusSeedBlock` (org/betterx/betterend/blocks/EndLotusSeedBlock.java)
    pub mod end_lotus_seed_block {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // EndLotusSeedBlock.EndLotusSeedBlock(1): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // EndLotusSeedBlock.grow(3): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // EndLotusSeedBlock.canGrow(2): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // EndLotusSeedBlock.generateLeaf(2): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // EndLotusSeedBlock.hasLeaf(2): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // EndLotusSeedBlock.isTerrain(1): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `EndLotusStemBlock` (org/betterx/betterend/blocks/EndLotusStemBlock.java)
    pub mod end_lotus_stem_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // LEAF: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPES: not translated, it uses Minecraft's `Axis`.
        // EndLotusStemBlock.EndLotusStemBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusStemBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusStemBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusStemBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusStemBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusStemBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusStemBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // EndLotusStemBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `EndPathBlock` (org/betterx/betterend/blocks/EndPathBlock.java)
    pub mod end_path_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `PathBlock`. Its static parts are below.
        // EndPathBlock.EndPathBlock(2): not translated, its class isn't translated (it extends Minecraft's `PathBlock`).
        // EndPathBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `PathBlock`).
    }

    /// `EndPedestal` (org/betterx/betterend/blocks/EndPedestal.java)
    pub mod end_pedestal {
        use super::*;
        // The class itself isn't translated: it extends `PedestalBlock` (org.betterx.betterend.blocks.basis). Its static parts are below.
        // MODEL: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // EndPedestal.EndPedestal(1): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EndPedestal.createTextureMapping(1): not translated, it uses Minecraft's `TextureMapping`.
        // EndPedestal.buildModel(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
    }

    /// `EndPedestal.ClientModel` (org/betterx/betterend/blocks/EndPedestal.java)
    pub mod end_pedestal_client_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModel {
        }
        impl ClientModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndPedestal.ClientModel.buildFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `EndPedestal.ModelParams` (org/betterx/betterend/blocks/EndPedestal.java)
    pub mod end_pedestal_model_params {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // EndPedestal.ModelParams.ModelParams(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndPedestal.ModelParams.base(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndPedestal.Pedestal` (org/betterx/betterend/blocks/EndPedestal.java)
    pub mod end_pedestal_pedestal {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndPedestal.Pedestal.Pedestal(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndPedestal.Pedestal.base(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndPortalBlock` (org/betterx/betterend/blocks/EndPortalBlock.java)
    pub mod end_portal_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `NetherPortalBlock`. Its static parts are below.
        // PORTAL: not translated, it uses Minecraft's `IntegerProperty`.
        // EndPortalBlock.EndPortalBlock(1): not translated, its class isn't translated (it extends Minecraft's `NetherPortalBlock`).
        // EndPortalBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `NetherPortalBlock`).
        // EndPortalBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `NetherPortalBlock`).
        // EndPortalBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `NetherPortalBlock`).
        // EndPortalBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `NetherPortalBlock`).
        // EndPortalBlock.entityInside(6): not translated, its class isn't translated (it extends Minecraft's `NetherPortalBlock`).
        // EndPortalBlock.validate(1): not translated, its class isn't translated (it extends Minecraft's `NetherPortalBlock`).
        // EndPortalBlock.getPortalDestination(3): not translated, its class isn't translated (it extends Minecraft's `NetherPortalBlock`).
        // EndPortalBlock.getDestinationLevels(2): not translated, it uses Minecraft's `ServerLevel`.
        // EndPortalBlock.getStartPos(3): not translated, it uses Minecraft's `BlockPos`.
        // EndPortalBlock.getTeleportTransition(4): not translated, it uses Minecraft's `TeleportTransition`.
        // EndPortalBlock.getTeleportTransitionFromExit(5): not translated, it uses Minecraft's `TeleportTransition`.
        // EndPortalBlock.createTeleportTransition(9): not translated, it uses Minecraft's `TeleportTransition`.
    }

    /// `EndstoneDustBlock` (org/betterx/betterend/blocks/EndstoneDustBlock.java)
    pub mod endstone_dust_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FallingBlock`. Its static parts are below.
        // COLOR: not translated, it calls `ColorUtilARGB32` (de.ambertation.wunderlib.utils).color.
        // EndstoneDustBlock.EndstoneDustBlock(1): not translated, its class isn't translated (it extends Minecraft's `FallingBlock`).
        // EndstoneDustBlock.getDustColor(3): not translated, its class isn't translated (it extends Minecraft's `FallingBlock`).
    }

    /// `EndStoneSmelter` (org/betterx/betterend/blocks/EndStoneSmelter.java)
    pub mod end_stone_smelter {
        use super::*;
        // The class itself isn't translated: it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity). Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // LIT: not translated, it uses Minecraft's `BooleanProperty`.
        pub const ID: &str = "end_stone_smelter";
        // EndStoneSmelter.EndStoneSmelter(1): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.useWithoutItem(5): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.openScreen(3): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.getStateForPlacement(1): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.newBlockEntity(2): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.getDrops(2): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.getRenderShape(1): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.rotate(2): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.mirror(2): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.animateTick(4): not translated, its class isn't translated (it extends `Stone` (org.betterx.bclib.blocks.BaseBlockWithEntity)).
        // EndStoneSmelter.getTicker(3): not translated, it's generic.
    }

    /// `AlloyingRecipeInfo` (org/betterx/betterend/blocks/entities/AlloyingRecipeInfo.java)
    pub mod alloying_recipe_info {
        use super::*;
        // The class itself isn't translated: it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes). Its static parts are below.
        // AlloyingRecipeInfo.AlloyingRecipeInfo(2): not translated, its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)).
        // AlloyingRecipeInfo.burnTimeMultiplicator(0): not translated, its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)).
        // AlloyingRecipeInfo.cookTimeIncrement(0): not translated, its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)).
        // AlloyingRecipeInfo.input(0): not translated, its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)).
        // AlloyingRecipeInfo.holder(0): not translated, its class isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)).
    }

    /// `BlastingRecipeInfo` (org/betterx/betterend/blocks/entities/BlastingRecipeInfo.java)
    pub mod blasting_recipe_info {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `SingleRecipeInput`. Its static parts are below.
        // BlastingRecipeInfo.BlastingRecipeInfo(2): not translated, its class isn't translated (it uses Minecraft's `SingleRecipeInput`).
        // BlastingRecipeInfo.burnTimeMultiplicator(0): not translated, its class isn't translated (it uses Minecraft's `SingleRecipeInput`).
        // BlastingRecipeInfo.cookTimeIncrement(0): not translated, its class isn't translated (it uses Minecraft's `SingleRecipeInput`).
        // BlastingRecipeInfo.input(0): not translated, its class isn't translated (it uses Minecraft's `SingleRecipeInput`).
        // BlastingRecipeInfo.holder(0): not translated, its class isn't translated (it uses Minecraft's `SingleRecipeInput`).
    }

    /// `BlockEntityHydrothermalVent` (org/betterx/betterend/blocks/entities/BlockEntityHydrothermalVent.java)
    pub mod block_entity_hydrothermal_vent {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // POSITIVE_Y: not translated, it uses Minecraft's `Vec3`.
        // BlockEntityHydrothermalVent.BlockEntityHydrothermalVent(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BlockEntityHydrothermalVent.tick(4): not translated, it's generic.
        // BlockEntityHydrothermalVent.clientTick(4): not translated, it uses Minecraft's `Level`.
        // BlockEntityHydrothermalVent.serverTick(4): not translated, it uses Minecraft's `Level`.
        // BlockEntityHydrothermalVent.hasElytra(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `EndStoneSmelterBlockEntity` (org/betterx/betterend/blocks/entities/EndStoneSmelterBlockEntity.java)
    pub mod end_stone_smelter_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseContainerBlockEntity`. Its static parts are below.
        // RECIPES_USED_CODEC: not translated, it uses Minecraft's `Codec`.
        pub static TOP_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1, 0]);
        pub static BOTTOM_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![2, 3]);
        pub static SIDE_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![2]);
        // AVAILABLE_FUELS: not translated, it uses Minecraft's `Item`.
        // LONG_BURNING_FUELS: not translated, it uses Minecraft's `ResourceKey`.
        // EndStoneSmelterBlockEntity.EndStoneSmelterBlockEntity(2): not translated, its body couldn't be read: an anonymous class.
        // EndStoneSmelterBlockEntity.isLit(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.isEmpty(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getItem(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.removeItem(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.removeItemNoUpdate(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getTotalCookTime(1): not translated, it uses AlloyingRecipeInfo, which isn't translated (it uses `AlloyingRecipeInput` (org.betterx.bclib.recipes)).
        // EndStoneSmelterBlockEntity.setLitState(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.setItem(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getSmeltTime(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getRecipesToAwardAndPopExperience(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.dropExperience(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.dropExperience(4): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.preRemoveSideEffects(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.clearContent(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getItems(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.setItems(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.serverTick(4): not translated, it uses Minecraft's `ServerLevel`.
        // EndStoneSmelterBlockEntity.burn(3): not translated, it uses Minecraft's `RecipeHolder`.
        // EndStoneSmelterBlockEntity.burn(4): not translated, it uses Minecraft's `RecipeHolder`.
        // EndStoneSmelterBlockEntity.burn(4): not translated, it uses Minecraft's `RecipeHolder`.
        // EndStoneSmelterBlockEntity.createResultItem(2): not translated, it uses Minecraft's `NonNullList`.
        // EndStoneSmelterBlockEntity.canBurn(4): not translated, it uses Minecraft's `RegistryAccess`.
        // EndStoneSmelterBlockEntity.canBurn(5): not translated, it's overloaded with the same number of parameters.
        // EndStoneSmelterBlockEntity.canBurn(5): not translated, it's overloaded with the same number of parameters.
        // EndStoneSmelterBlockEntity.canStore(3): not translated, it uses Minecraft's `NonNullList`.
        // EndStoneSmelterBlockEntity.fillStackedContents(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.setRecipeUsed(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getRecipeUsed(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.getSlotsForFace(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.canPlaceItemThroughFace(3): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.canTakeItemThroughFace(3): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.canPlaceItem(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.canUseAsFuel(1): not translated, it uses Minecraft's `ItemStack`.
        // EndStoneSmelterBlockEntity.longBurningTicks(1): not translated, it uses Minecraft's `ItemStack`.
        // EndStoneSmelterBlockEntity.smelterFuels(0): not translated, it uses Minecraft's `Item`.
        // EndStoneSmelterBlockEntity.getBurnDuration(3): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.resolveVanillaBurnDuration(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.lootContext(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // EndStoneSmelterBlockEntity.registerFuel(2): not translated, it uses Minecraft's `ItemLike`.
        // EndStoneSmelterBlockEntity.availableFuels(0): not translated, it uses Minecraft's `Item`.
    }

    /// `EternalPedestalEntity` (org/betterx/betterend/blocks/entities/EternalPedestalEntity.java)
    pub mod eternal_pedestal_entity {
        use super::*;
        // The class itself isn't translated: it extends `PedestalBlockEntity`. Its static parts are below.
        // EternalPedestalEntity.EternalPedestalEntity(2): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // EternalPedestalEntity.hasRitual(0): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // EternalPedestalEntity.linkRitual(1): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // EternalPedestalEntity.getRitual(0): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // EternalPedestalEntity.setLevel(1): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // EternalPedestalEntity.saveAdditional(1): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // EternalPedestalEntity.loadAdditional(1): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
    }

    /// `FlowerPotBlockEntity` (org/betterx/betterend/blocks/entities/FlowerPotBlockEntity.java)
    pub mod flower_pot_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // FlowerPotBlockEntity.FlowerPotBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.getPlant(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.getSoil(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.getPlantBlock(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.getSoilBlock(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.setPlant(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.setSoil(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.setChanged(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // FlowerPotBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `InfusionPedestalEntity` (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java)
    pub mod infusion_pedestal_entity {
        use super::*;
        // The class itself isn't translated: it extends `PedestalBlockEntity`. Its static parts are below.
        // InfusionPedestalEntity.InfusionPedestalEntity(2): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // InfusionPedestalEntity.setLevel(1): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // InfusionPedestalEntity.linkRitual(3): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // InfusionPedestalEntity.getRitual(0): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // InfusionPedestalEntity.hasRitual(0): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // InfusionPedestalEntity.saveAdditional(1): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // InfusionPedestalEntity.loadAdditional(1): not translated, its class isn't translated (it extends `PedestalBlockEntity`).
        // InfusionPedestalEntity.tickEntity(4): not translated, it's generic.
    }

    /// `InfusionPedestalEntity.ClientHooks` (org/betterx/betterend/blocks/entities/InfusionPedestalEntity.java)
    pub mod infusion_pedestal_entity_client_hooks {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientHooks {
        }
        impl ClientHooks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // InfusionPedestalEntity.ClientHooks.spawnHintMist(2): not translated, it uses Minecraft's `Level`.
    }

    /// `InputState` (org/betterx/betterend/blocks/entities/InputState.java)
    pub mod input_state {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // InputState.InputState(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InputState.singleInput(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InputState.hasOneInput(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InputState.hasBothInputs(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InputState.hasInput(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InputState.inputA(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InputState.inputB(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `MultiRecipeInfo` (org/betterx/betterend/blocks/entities/MultiRecipeInfo.java)
    pub mod multi_recipe_info {
        use super::*;
        // The class itself isn't translated: it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`). Its static parts are below.
        // MultiRecipeInfo.MultiRecipeInfo(3): not translated, its class isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)).
        // MultiRecipeInfo.isBlasting(0): not translated, its class isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)).
        // MultiRecipeInfo.burnTimeMultiplicator(0): not translated, its class isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)).
        // MultiRecipeInfo.cookTimeIncrement(0): not translated, its class isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)).
        // MultiRecipeInfo.of(3): not translated, it uses MultiRecipeInfo, which isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)).
        // MultiRecipeInfo.of(3): not translated, it uses MultiRecipeInfo, which isn't translated (it uses BlastingRecipeInfo, which isn't translated (it uses Minecraft's `SingleRecipeInput`)).
    }

    /// `PedestalBlockEntity` (org/betterx/betterend/blocks/entities/PedestalBlockEntity.java)
    pub mod pedestal_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // PedestalBlockEntity.PedestalBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.PedestalBlockEntity(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.isEmpty(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.getItem(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.removeItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.canPlaceItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.clearContent(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.removeItemNoUpdate(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.setItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.setChanged(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // PedestalBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `EternalPedestal` (org/betterx/betterend/blocks/EternalPedestal.java)
    pub mod eternal_pedestal {
        use super::*;
        // The class itself isn't translated: it extends `PedestalBlock` (org.betterx.betterend.blocks.basis). Its static parts are below.
        // ACTIVATED: not translated, it uses Minecraft's `BooleanProperty`.
        // EternalPedestal.EternalPedestal(1): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.checkRitual(3): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.updateShape(8): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.getDestroyProgress(4): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.getExplosionResistance(0): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.dropFromExplosion(1): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.getDrops(2): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.newBlockEntity(2): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.hasUniqueEntity(0): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.useItemOn(7): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.dispatchParticles(3): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.animateTick(4): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // EternalPedestal.createVariants(6): not translated, it uses Minecraft's `MultiVariant`.
        // EternalPedestal.provideBlockModelsInstance(1): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
    }

    /// `EternalPedestal.ClientHooks` (org/betterx/betterend/blocks/EternalPedestal.java)
    pub mod eternal_pedestal_client_hooks {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientHooks {
        }
        impl ClientHooks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EternalPedestal.ClientHooks.showVision(2): not translated, it uses Minecraft's `Level`.
        // EternalPedestal.ClientHooks.tickVision(3): not translated, it uses Minecraft's `Level`.
    }

    /// `FilaluxBlock` (org/betterx/betterend/blocks/FilaluxBlock.java)
    pub mod filalux_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseVineBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // FilaluxBlock.FilaluxBlock(1): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
    }

    /// `FilaluxLanternBlock` (org/betterx/betterend/blocks/FilaluxLanternBlock.java)
    pub mod filalux_lantern_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FilaluxLanternBlock.FilaluxLanternBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `FilaluxWingsBlock` (org/betterx/betterend/blocks/FilaluxWingsBlock.java)
    pub mod filalux_wings_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseAttachedBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        // FilaluxWingsBlock.FilaluxWingsBlock(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // FilaluxWingsBlock.getShape(4): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
    }

    /// `FlamaeaBlock` (org/betterx/betterend/blocks/FlamaeaBlock.java)
    pub mod flamaea_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // FlamaeaBlock.FlamaeaBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // FlamaeaBlock.getShape(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `FlammalixBlock` (org/betterx/betterend/blocks/FlammalixBlock.java)
    pub mod flammalix_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // FlammalixBlock.FlammalixBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // FlammalixBlock.getShape(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `FlowerPotBlock` (org/betterx/betterend/blocks/FlowerPotBlock.java)
    pub mod flower_pot_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // POT_LIGHT: not translated, it uses Minecraft's `IntegerProperty`.
        // SHAPE_EMPTY: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_FULL: not translated, it uses Minecraft's `VoxelShape`.
        // MODEL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // FlowerPotBlock.FlowerPotBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FlowerPotBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FlowerPotBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FlowerPotBlock.getDrops(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FlowerPotBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FlowerPotBlock.findByBlock(3): not translated, it's generic.
        // FlowerPotBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FlowerPotBlock.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // FlowerPotBlock.buildModel(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
    }

    /// `FlowerPotBlock.ClientModel` (org/betterx/betterend/blocks/FlowerPotBlock.java)
    pub mod flower_pot_block_client_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModel {
        }
        impl ClientModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FlowerPotBlock.ClientModel.build(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `GlowingHymenophoreBlock` (org/betterx/betterend/blocks/GlowingHymenophoreBlock.java)
    pub mod glowing_hymenophore_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // GlowingHymenophoreBlock.GlowingHymenophoreBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // GlowingHymenophoreBlock.provideUnshadedCubeModel(2): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
    }

    /// `GlowingMossBlock` (org/betterx/betterend/blocks/GlowingMossBlock.java)
    pub mod glowing_moss_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // GlowingMossBlock.GlowingMossBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // GlowingMossBlock.hasEmissiveLighting(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // GlowingMossBlock.getAmbientOcclusionLightLevel(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `GlowingPillarLuminophorBlock` (org/betterx/betterend/blocks/GlowingPillarLuminophorBlock.java)
    pub mod glowing_pillar_luminophor_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // NATURAL: not translated, it uses Minecraft's `BooleanProperty`.
        // GlowingPillarLuminophorBlock.GlowingPillarLuminophorBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // GlowingPillarLuminophorBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // GlowingPillarLuminophorBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // GlowingPillarLuminophorBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `GlowingPillarRootsBlock` (org/betterx/betterend/blocks/GlowingPillarRootsBlock.java)
    pub mod glowing_pillar_roots_block {
        use super::*;
        // The class itself isn't translated: it extends `UpDownPlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // GlowingPillarRootsBlock.GlowingPillarRootsBlock(1): not translated, its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)).
        // GlowingPillarRootsBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)).
        // GlowingPillarRootsBlock.isTerrain(1): not translated, its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)).
        // GlowingPillarRootsBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends `UpDownPlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `GlowingPillarSeedBlock` (org/betterx/betterend/blocks/GlowingPillarSeedBlock.java)
    pub mod glowing_pillar_seed_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // GlowingPillarSeedBlock.GlowingPillarSeedBlock(1): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // GlowingPillarSeedBlock.growAdult(3): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `HelixTreeLeavesBlock` (org/betterx/betterend/blocks/HelixTreeLeavesBlock.java)
    pub mod helix_tree_leaves_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // COLOR: not translated, it uses Minecraft's `IntegerProperty`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // HelixTreeLeavesBlock.HelixTreeLeavesBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HelixTreeLeavesBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HelixTreeLeavesBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `HelixTreeSaplingBlock` (org/betterx/betterend/blocks/HelixTreeSaplingBlock.java)
    pub mod helix_tree_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // HelixTreeSaplingBlock.HelixTreeSaplingBlock(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `HydraluxBlock` (org/betterx/betterend/blocks/HydraluxBlock.java)
    pub mod hydralux_block {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // HydraluxBlock.HydraluxBlock(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // HydraluxBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // HydraluxBlock.canSurvive(3): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // HydraluxBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // HydraluxBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // HydraluxBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // HydraluxBlock.buildLoot(2): not translated, it uses Minecraft's `Builder`.
        // HydraluxBlock.shapeCond(3): not translated, it uses Minecraft's `Builder`.
    }

    /// `HydraluxPetalBlock` (org/betterx/betterend/blocks/HydraluxPetalBlock.java)
    pub mod hydralux_petal_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // HydraluxPetalBlock.HydraluxPetalBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydraluxPetalBlock.fallOn(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `HydraluxPetalColoredBlock` (org/betterx/betterend/blocks/HydraluxPetalColoredBlock.java)
    pub mod hydralux_petal_colored_block {
        use super::*;
        // The class itself isn't translated: it extends `HydraluxPetalBlock`. Its static parts are below.
        pub static PETAL_MODEL: LazyLock<JCell<Option<JIdentifier>>> = LazyLock::new(|| JCell::new(None));
        // HydraluxPetalColoredBlock.HydraluxPetalColoredBlock(1): not translated, its class isn't translated (it extends `HydraluxPetalBlock`).
        // HydraluxPetalColoredBlock.provideBlockModel(2): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
    }

    /// `HydraluxSaplingBlock` (org/betterx/betterend/blocks/HydraluxSaplingBlock.java)
    pub mod hydralux_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // HydraluxSaplingBlock.HydraluxSaplingBlock(1): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // HydraluxSaplingBlock.grow(3): not translated, its class isn't translated (it extends `UnderwaterPlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `HydrothermalVentBlock` (org/betterx/betterend/blocks/HydrothermalVentBlock.java)
    pub mod hydrothermal_vent_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // ACTIVATED: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // HydrothermalVentBlock.HydrothermalVentBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.canPlaceLiquid(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.placeLiquid(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.tick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // HydrothermalVentBlock.getTicker(3): not translated, it's generic.
    }

    /// `InfusionPedestal` (org/betterx/betterend/blocks/InfusionPedestal.java)
    pub mod infusion_pedestal {
        use super::*;
        // The class itself isn't translated: it extends `PedestalBlock` (org.betterx.betterend.blocks.basis). Its static parts are below.
        // SHAPE_DEFAULT: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_PEDESTAL_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // InfusionPedestal.InfusionPedestal(1): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // InfusionPedestal.checkRitual(3): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // InfusionPedestal.setPlacedBy(5): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // InfusionPedestal.useItemOn(7): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // InfusionPedestal.newBlockEntity(2): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // InfusionPedestal.hasUniqueEntity(0): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // InfusionPedestal.getShape(4): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // InfusionPedestal.getTicker(3): not translated, it's generic.
        // InfusionPedestal.createTextureMapping(0): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
        // InfusionPedestal.provideBlockModelsInstance(1): not translated, its class isn't translated (it extends `PedestalBlock` (org.betterx.betterend.blocks.basis)).
    }

    /// `InfusionPedestal.ClientHooks` (org/betterx/betterend/blocks/InfusionPedestal.java)
    pub mod infusion_pedestal_client_hooks {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientHooks {
        }
        impl ClientHooks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // InfusionPedestal.ClientHooks.showHint(2): not translated, it uses Minecraft's `Level`.
        // InfusionPedestal.ClientHooks.showRecipes(0): needs the game, so it's in game_side.rs.
    }

    /// `InfusionPedestal.Models` (org/betterx/betterend/blocks/InfusionPedestal.java)
    pub mod infusion_pedestal_models {
        use super::*;
        // PEDESTAL_MODELS: not translated, it uses PedestalState, which isn't translated (its constants carry values).
        #[derive(Clone, Debug)]
        pub struct Models {
        }
        impl Models {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `JellyshroomCapBlock` (org/betterx/betterend/blocks/JellyshroomCapBlock.java)
    pub mod jellyshroom_cap_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SlimeBlock`. Its static parts are below.
        // COLOR: not translated, it uses Minecraft's `IntegerProperty`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // JellyshroomCapBlock.JellyshroomCapBlock(1): not translated, its class isn't translated (it extends Minecraft's `SlimeBlock`).
        // JellyshroomCapBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `SlimeBlock`).
        // JellyshroomCapBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `SlimeBlock`).
        // JellyshroomCapBlock.provideBlockModel(2): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
    }

    /// `LacugroveSaplingBlock` (org/betterx/betterend/blocks/LacugroveSaplingBlock.java)
    pub mod lacugrove_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // LacugroveSaplingBlock.LacugroveSaplingBlock(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `LanceleafBlock` (org/betterx/betterend/blocks/LanceleafBlock.java)
    pub mod lanceleaf_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // ROTATION: not translated, it uses Minecraft's `IntegerProperty`.
        // LanceleafBlock.LanceleafBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LanceleafBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LanceleafBlock.canSurvive(3): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LanceleafBlock.updateShape(8): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LanceleafBlock.isBottom(2): not translated, it uses Minecraft's `Builder`.
        // LanceleafBlock.buildLoot(2): not translated, it uses Minecraft's `Builder`.
    }

    /// `LanceleafSeedBlock` (org/betterx/betterend/blocks/LanceleafSeedBlock.java)
    pub mod lanceleaf_seed_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // LanceleafSeedBlock.LanceleafSeedBlock(1): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // LanceleafSeedBlock.growAdult(3): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `LargeAmaranitaBlock` (org/betterx/betterend/blocks/LargeAmaranitaBlock.java)
    pub mod large_amaranita_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // LargeAmaranitaBlock.LargeAmaranitaBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LargeAmaranitaBlock.getShape(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LargeAmaranitaBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LargeAmaranitaBlock.canSurvive(3): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LargeAmaranitaBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LargeAmaranitaBlock.updateShape(8): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LargeAmaranitaBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // LargeAmaranitaBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `LucerniaSaplingBlock` (org/betterx/betterend/blocks/LucerniaSaplingBlock.java)
    pub mod lucernia_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // LucerniaSaplingBlock.LucerniaSaplingBlock(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `LumecornBlock` (org/betterx/betterend/blocks/LumecornBlock.java)
    pub mod lumecorn_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // LumecornBlock.LumecornBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LumecornBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LumecornBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LumecornBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LumecornBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LumecornBlock.buildLoot(2): not translated, it uses Minecraft's `Builder`.
        // LumecornBlock.shapeIs(3): not translated, it uses Minecraft's `Builder`.
        // LumecornBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `LumecornSeedBlock` (org/betterx/betterend/blocks/LumecornSeedBlock.java)
    pub mod lumecorn_seed_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // LumecornSeedBlock.LumecornSeedBlock(1): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
        // LumecornSeedBlock.growAdult(3): not translated, its class isn't translated (it extends `BasePlantWithAgeBlock` (org.betterx.bclib.blocks)).
    }

    /// `MengerSpongeBlock` (org/betterx/betterend/blocks/MengerSpongeBlock.java)
    pub mod menger_sponge_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // MengerSpongeBlock.MengerSpongeBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeBlock.onPlace(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeBlock.absorbWater(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `MengerSpongeWetBlock` (org/betterx/betterend/blocks/MengerSpongeWetBlock.java)
    pub mod menger_sponge_wet_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // MengerSpongeWetBlock.MengerSpongeWetBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeWetBlock.onPlace(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeWetBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeWetBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeWetBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MengerSpongeWetBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `MossyDragonBoneBlock` (org/betterx/betterend/blocks/MossyDragonBoneBlock.java)
    pub mod mossy_dragon_bone_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        // MossyDragonBoneBlock.MossyDragonBoneBlock(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // MossyDragonBoneBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // MossyDragonBoneBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
    }

    /// `MossyGlowshroomCapBlock` (org/betterx/betterend/blocks/MossyGlowshroomCapBlock.java)
    pub mod mossy_glowshroom_cap_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TRANSITION: not translated, it uses Minecraft's `BooleanProperty`.
        // MossyGlowshroomCapBlock.MossyGlowshroomCapBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MossyGlowshroomCapBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MossyGlowshroomCapBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `MossyGlowshroomSaplingBlock` (org/betterx/betterend/blocks/MossyGlowshroomSaplingBlock.java)
    pub mod mossy_glowshroom_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // MossyGlowshroomSaplingBlock.MossyGlowshroomSaplingBlock(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `MossyObsidian` (org/betterx/betterend/blocks/MossyObsidian.java)
    pub mod mossy_obsidian {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // MossyObsidian.MossyObsidian(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MossyObsidian.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // MossyObsidian.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `MurkweedBlock` (org/betterx/betterend/blocks/MurkweedBlock.java)
    pub mod murkweed_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // MurkweedBlock.MurkweedBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // MurkweedBlock.animateTick(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // MurkweedBlock.entityInside(6): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // MurkweedBlock.isPathfindable(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `NeedlegrassBlock` (org/betterx/betterend/blocks/NeedlegrassBlock.java)
    pub mod needlegrass_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // NeedlegrassBlock.NeedlegrassBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // NeedlegrassBlock.entityInside(6): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // NeedlegrassBlock.buildLoot(2): not translated, it uses Builder, which isn't translated (it's an interface).
        // NeedlegrassBlock.isPathfindable(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `NeonCactusPlantBlock` (org/betterx/betterend/blocks/NeonCactusPlantBlock.java)
    pub mod neon_cactus_plant_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // CACTUS_BOTTOM: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // BIG_SHAPES_OPEN: not translated, it uses `EnumMap` (java.util).
        // MEDIUM_SHAPES_OPEN: not translated, it uses `EnumMap` (java.util).
        // SMALL_SHAPES_OPEN: not translated, it uses `EnumMap` (java.util).
        // BIG_SHAPES: not translated, it uses `EnumMap` (java.util).
        // MEDIUM_SHAPES: not translated, it uses `EnumMap` (java.util).
        // SMALL_SHAPES: not translated, it uses `EnumMap` (java.util).
        pub const MAX_LENGTH: i32 = 12;
        // NeonCactusPlantBlock.NeonCactusPlantBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.tick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.growPlant(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.growPlant(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.growIteration(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.getSideDirection(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.entityInside(6): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.getLength(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.getHorizontal(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // NeonCactusPlantBlock.mutateStem(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `PallidiumBlock` (org/betterx/betterend/blocks/PallidiumBlock.java)
    pub mod pallidium_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain). Its static parts are below.
        // PallidiumBlock.PallidiumBlock(3): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // PallidiumBlock.getBaseBlock(0): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // PallidiumBlock.useItemOn(7): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
    }

    /// `PondAnemoneBlock` (org/betterx/betterend/blocks/PondAnemoneBlock.java)
    pub mod pond_anemone_block {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // PondAnemoneBlock.PondAnemoneBlock(1): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // PondAnemoneBlock.animateTick(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // PondAnemoneBlock.getShape(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // PondAnemoneBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
        // PondAnemoneBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `UnderwaterPlantBlock` (de.ambertation.wover.block.api.plants)).
    }

    /// `PythadendronSaplingBlock` (org/betterx/betterend/blocks/PythadendronSaplingBlock.java)
    pub mod pythadendron_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // PythadendronSaplingBlock.PythadendronSaplingBlock(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `RespawnObeliskBlock` (org/betterx/betterend/blocks/RespawnObeliskBlock.java)
    pub mod respawn_obelisk_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // VOXEL_SHAPE_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // VOXEL_SHAPE_MIDDLE_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // RespawnObeliskBlock.RespawnObeliskBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RespawnObeliskBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RespawnObeliskBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RespawnObeliskBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RespawnObeliskBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RespawnObeliskBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RespawnObeliskBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RespawnObeliskBlock.buildLoot(2): not translated, it uses Minecraft's `Builder`.
        // RespawnObeliskBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `RunedFlavolite` (org/betterx/betterend/blocks/RunedFlavolite.java)
    pub mod runed_flavolite {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // ACTIVATED: not translated, it uses Minecraft's `BooleanProperty`.
        // RunedFlavolite.RunedFlavolite(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RunedFlavolite.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // RunedFlavolite.dropFromExplosion(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `ShadowBerryBlock` (org/betterx/betterend/blocks/ShadowBerryBlock.java)
    pub mod shadow_berry_block {
        use super::*;
        // The class itself isn't translated: it extends `PottableCropBlock` (de.ambertation.wover.block.api.plants). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // ShadowBerryBlock.ShadowBerryBlock(1): not translated, its class isn't translated (it extends `PottableCropBlock` (de.ambertation.wover.block.api.plants)).
        // ShadowBerryBlock.getShape(4): not translated, its class isn't translated (it extends `PottableCropBlock` (de.ambertation.wover.block.api.plants)).
    }

    /// `ShadowGrassBlock` (org/betterx/betterend/blocks/ShadowGrassBlock.java)
    pub mod shadow_grass_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain). Its static parts are below.
        // ShadowGrassBlock.ShadowGrassBlock(1): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
        // ShadowGrassBlock.animateTick(4): not translated, its class isn't translated (it extends `BaseTerrainBlock` (de.ambertation.wover.block.api.terrain)).
    }

    /// `SilkMothHiveBlock` (org/betterx/betterend/blocks/SilkMothHiveBlock.java)
    pub mod silk_moth_hive_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // FULLNESS: not translated, it uses Minecraft's `IntegerProperty`.
        // SilkMothHiveBlock.SilkMothHiveBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothHiveBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothHiveBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothHiveBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothHiveBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothHiveBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothHiveBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `SilkMothNestBlock` (org/betterx/betterend/blocks/SilkMothNestBlock.java)
    pub mod silk_moth_nest_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // ACTIVE: not translated, it uses Minecraft's `BooleanProperty`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // FULLNESS: not translated, it uses Minecraft's `IntegerProperty`.
        // TOP: not translated, it uses Minecraft's `VoxelShape`.
        // BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // SilkMothNestBlock.SilkMothNestBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.buildLoot(2): not translated, it uses Minecraft's `Builder`.
        // SilkMothNestBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SilkMothNestBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `SmallAmaranitaBlock` (org/betterx/betterend/blocks/SmallAmaranitaBlock.java)
    pub mod small_amaranita_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SmallAmaranitaBlock.SmallAmaranitaBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // SmallAmaranitaBlock.performBonemeal(5): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // SmallAmaranitaBlock.getShape(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // SmallAmaranitaBlock.growBig(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // SmallAmaranitaBlock.checkFrame(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // SmallAmaranitaBlock.replaceMushroom(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // SmallAmaranitaBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // SmallAmaranitaBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `SmallJellyshroomBlock` (org/betterx/betterend/blocks/SmallJellyshroomBlock.java)
    pub mod small_jellyshroom_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseAttachedBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        // SmallJellyshroomBlock.SmallJellyshroomBlock(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmallJellyshroomBlock.getShape(4): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmallJellyshroomBlock.canSurvive(3): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmallJellyshroomBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmallJellyshroomBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmallJellyshroomBlock.performBonemeal(5): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
    }

    /// `SmaragdantCrystalShardBlock` (org/betterx/betterend/blocks/SmaragdantCrystalShardBlock.java)
    pub mod smaragdant_crystal_shard_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseAttachedBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // SmaragdantCrystalShardBlock.SmaragdantCrystalShardBlock(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmaragdantCrystalShardBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmaragdantCrystalShardBlock.canPlaceLiquid(5): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmaragdantCrystalShardBlock.placeLiquid(4): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmaragdantCrystalShardBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmaragdantCrystalShardBlock.getFluidState(1): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmaragdantCrystalShardBlock.getShape(4): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
        // SmaragdantCrystalShardBlock.canSurvive(3): not translated, its class isn't translated (it extends `BaseAttachedBlock` (org.betterx.bclib.blocks)).
    }

    /// `SulphurCrystalBlock` (org/betterx/betterend/blocks/SulphurCrystalBlock.java)
    pub mod sulphur_crystal_block {
        use super::*;
        // The class itself isn't translated: it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock). Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        pub const MAX_AGE: i32 = 2;
        // AGE: not translated, it uses Minecraft's `IntegerProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // SulphurCrystalBlock.SulphurCrystalBlock(1): not translated, its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // SulphurCrystalBlock.naturalState(4): not translated, it uses Minecraft's `BlockState`.
        // SulphurCrystalBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // SulphurCrystalBlock.ageCondition(3): not translated, it uses Builder, which isn't translated (it's an interface).
        // SulphurCrystalBlock.buildLoot(2): not translated, it uses Minecraft's `Builder`.
        // SulphurCrystalBlock.canPlaceLiquid(5): not translated, its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // SulphurCrystalBlock.placeLiquid(4): not translated, its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // SulphurCrystalBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // SulphurCrystalBlock.getFluidState(1): not translated, its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // SulphurCrystalBlock.getShape(4): not translated, its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)).
        // SulphurCrystalBlock.canSurvive(3): not translated, its class isn't translated (it extends `Glass` (org.betterx.bclib.blocks.BaseAttachedBlock)).
    }

    /// `TenaneaFlowersBlock` (org/betterx/betterend/blocks/TenaneaFlowersBlock.java)
    pub mod tenanea_flowers_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseVineBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // COLORS: not translated, it uses Minecraft's `Vec3i`.
        // TenaneaFlowersBlock.TenaneaFlowersBlock(1): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // TenaneaFlowersBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // TenaneaFlowersBlock.animateTick(4): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
    }

    /// `TenaneaSaplingBlock` (org/betterx/betterend/blocks/TenaneaSaplingBlock.java)
    pub mod tenanea_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // TenaneaSaplingBlock.TenaneaSaplingBlock(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `TwistedUmbrellaMossBlock` (org/betterx/betterend/blocks/TwistedUmbrellaMossBlock.java)
    pub mod twisted_umbrella_moss_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // TwistedUmbrellaMossBlock.TwistedUmbrellaMossBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // TwistedUmbrellaMossBlock.hasEmissiveLighting(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // TwistedUmbrellaMossBlock.getAmbientOcclusionLightLevel(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // TwistedUmbrellaMossBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // TwistedUmbrellaMossBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // TwistedUmbrellaMossBlock.performBonemeal(5): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `TwistedUmbrellaMossTallBlock` (org/betterx/betterend/blocks/TwistedUmbrellaMossTallBlock.java)
    pub mod twisted_umbrella_moss_tall_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants). Its static parts are below.
        // TwistedUmbrellaMossTallBlock.TwistedUmbrellaMossTallBlock(1): not translated, its class isn't translated (it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)).
        // TwistedUmbrellaMossTallBlock.performBonemeal(5): not translated, its class isn't translated (it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)).
    }

    /// `UmbrellaMossBlock` (org/betterx/betterend/blocks/UmbrellaMossBlock.java)
    pub mod umbrella_moss_block {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // UmbrellaMossBlock.UmbrellaMossBlock(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // UmbrellaMossBlock.hasEmissiveLighting(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // UmbrellaMossBlock.getAmbientOcclusionLightLevel(2): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // UmbrellaMossBlock.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // UmbrellaMossBlock.isBonemealSuccess(5): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // UmbrellaMossBlock.performBonemeal(5): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `UmbrellaMossTallBlock` (org/betterx/betterend/blocks/UmbrellaMossTallBlock.java)
    pub mod umbrella_moss_tall_block {
        use super::*;
        // The class itself isn't translated: it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants). Its static parts are below.
        // UmbrellaMossTallBlock.UmbrellaMossTallBlock(1): not translated, its class isn't translated (it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)).
        // UmbrellaMossTallBlock.performBonemeal(5): not translated, its class isn't translated (it extends `BaseDoublePlantBlock` (de.ambertation.wover.block.api.plants)).
    }

    /// `UmbrellaTreeClusterBlock` (org/betterx/betterend/blocks/UmbrellaTreeClusterBlock.java)
    pub mod umbrella_tree_cluster_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // NATURAL: not translated, it uses Minecraft's `BooleanProperty`.
        // UmbrellaTreeClusterBlock.UmbrellaTreeClusterBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // UmbrellaTreeClusterBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // UmbrellaTreeClusterBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // UmbrellaTreeClusterBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `UmbrellaTreeClusterEmptyBlock` (org/betterx/betterend/blocks/UmbrellaTreeClusterEmptyBlock.java)
    pub mod umbrella_tree_cluster_empty_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // NATURAL: not translated, it uses Minecraft's `BooleanProperty`.
        // UmbrellaTreeClusterEmptyBlock.UmbrellaTreeClusterEmptyBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // UmbrellaTreeClusterEmptyBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // UmbrellaTreeClusterEmptyBlock.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `UmbrellaTreeMembraneBlock` (org/betterx/betterend/blocks/UmbrellaTreeMembraneBlock.java)
    pub mod umbrella_tree_membrane_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SlimeBlock`. Its static parts are below.
        // COLOR: not translated, it uses Minecraft's `IntegerProperty`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // UmbrellaTreeMembraneBlock.UmbrellaTreeMembraneBlock(1): not translated, its class isn't translated (it extends Minecraft's `SlimeBlock`).
        // UmbrellaTreeMembraneBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `SlimeBlock`).
        // UmbrellaTreeMembraneBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `SlimeBlock`).
        // UmbrellaTreeMembraneBlock.provideBlockModel(2): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
        // UmbrellaTreeMembraneBlock.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `SlimeBlock`).
        // UmbrellaTreeMembraneBlock.skipRendering(3): not translated, its class isn't translated (it extends Minecraft's `SlimeBlock`).
    }

    /// `UmbrellaTreeSaplingBlock` (org/betterx/betterend/blocks/UmbrellaTreeSaplingBlock.java)
    pub mod umbrella_tree_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // UmbrellaTreeSaplingBlock.UmbrellaTreeSaplingBlock(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `VentBubbleColumnBlock` (org/betterx/betterend/blocks/VentBubbleColumnBlock.java)
    pub mod vent_bubble_column_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // VentBubbleColumnBlock.VentBubbleColumnBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.pickupBlock(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.entityInside(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.canPlaceLiquid(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.placeLiquid(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VentBubbleColumnBlock.getPickupSound(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BetterEndClient` (org/betterx/betterend/client/BetterEndClient.java)
    pub mod better_end_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // BetterEndClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // BetterEndClient.registerTooltips(0): not translated, unknown static ItemTooltipCallback.EVENT.
    }

    /// `EternalHint` (org/betterx/betterend/client/effects/EternalHint.java)
    pub mod eternal_hint {
        use super::*;
        pub const DURATION: i32 = 220;
        pub const FADE_IN: i32 = 8;
        pub const FADE_OUT: i32 = 40;
        pub const MAX_DISTANCE_SQ: f64 = 4096.0f64;
        pub const CRYSTAL_HEIGHT: f64 = 1.35f64;
        pub const STREAK_SPEED: f64 = 0.55f64;
        pub const STREAKS_PER_TICK: i32 = 2;
        pub const DRIP_CHANCE: f32 = 0.35f32;
        // center: not translated, it uses Minecraft's `BlockPos`.
        // source: not translated, it uses Minecraft's `BlockPos`.
        // axis: not translated, it uses Minecraft's `Axis`.
        pub static START_TIME: JCell<i64> = JCell::new(0i64);
        #[derive(Clone, Debug)]
        pub struct EternalHint {
        }
        impl EternalHint {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EternalHint.trigger(2): not translated, it uses Minecraft's `Level`.
        // EternalHint.clear(0): not translated, it uses EternalHint.center, which isn't translated (it uses Minecraft's `BlockPos`).
        // EternalHint.anchorPedestal(1): not translated, it uses Minecraft's `BlockPos`.
        // EternalHint.center(0): not translated, it uses Minecraft's `BlockPos`.
        // EternalHint.age(1): not translated, it uses EternalHint.center, which isn't translated (it uses Minecraft's `BlockPos`).
        // EternalHint.intensity(1): not translated, it calls EternalHint.age, which isn't translated.
        // EternalHint.wobble(1): not translated, it calls EternalHint.age, which isn't translated.
        // EternalHint.pedestalPositions(0): not translated, it uses Minecraft's `BlockPos`.
        // EternalHint.missingFrame(1): not translated, it uses Minecraft's `BlockPos`.
        // EternalHint.tickParticles(3): not translated, it uses Minecraft's `Level`.
        // EternalHint.streakToCentre(4): not translated, it uses `ClientLevelAccess` (org.betterx.bclib.interfaces).
        // EternalHint.dripFromGaps(4): not translated, it uses `ClientLevelAccess` (org.betterx.bclib.interfaces).
    }

    /// `InfusionHint` (org/betterx/betterend/client/effects/InfusionHint.java)
    pub mod infusion_hint {
        use super::*;
        pub const DURATION: i32 = 160;
        pub const FADE_IN: i32 = 8;
        pub const FADE_OUT: i32 = 30;
        pub const MAX_DISTANCE_SQ: f64 = 2304.0f64;
        pub const MIST_SPAWN_HEIGHT: f64 = 1.0f64;
        pub const MIST_SPAWN_JITTER: f64 = 0.35f64;
        pub const MIST_SPREAD: f64 = 0.008f64;
        pub const BURST_COUNT: i32 = 14;
        pub const BURST_RADIUS: f64 = 0.34f64;
        pub const BURST_HEIGHT: f64 = 0.08f64;
        pub const SUBTLE_ALPHA: f32 = 0.45f32;
        pub const FLASH_PER_SPOKE: i32 = 3;
        pub const FLASH_PULSES: i32 = 4;
        pub const FLASH_PULSE_INTERVAL: i32 = 10;
        pub const FLASH_HEIGHT: f64 = 1.6f64;
        // pedestalPos: not translated, it uses Minecraft's `BlockPos`.
        pub static START_TIME: JCell<i64> = JCell::new(0i64);
        pub static SUBTLE: JCell<bool> = JCell::new(false);
        // flashPos: not translated, it uses Minecraft's `BlockPos`.
        pub static FLASH_START: JCell<i64> = JCell::new(0i64);
        #[derive(Clone, Debug)]
        pub struct InfusionHint {
        }
        impl InfusionHint {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // InfusionHint.trigger(2): not translated, it uses Minecraft's `Level`.
        // InfusionHint.triggerSubtle(2): not translated, it uses Minecraft's `Level`.
        // InfusionHint.start(3): not translated, it uses Minecraft's `Level`.
        // InfusionHint.clear(0): not translated, it uses InfusionHint.pedestalPos, which isn't translated (it uses Minecraft's `BlockPos`).
        // InfusionHint.dismiss(1): not translated, it uses Minecraft's `Level`.
        // InfusionHint.activePedestal(0): not translated, it uses Minecraft's `BlockPos`.
        // InfusionHint.isActiveAt(1): not translated, it uses Minecraft's `BlockPos`.
        // InfusionHint.age(1): not translated, it uses InfusionHint.pedestalPos, which isn't translated (it uses Minecraft's `BlockPos`).
        // InfusionHint.spawnMist(2): not translated, it uses Minecraft's `Level`.
        // InfusionHint.outline(1): not translated, it calls InfusionHint.age, which isn't translated.
        // InfusionHint.mistFor(1): not translated, it uses Minecraft's `SimpleParticleType`.
        // InfusionHint.burstAtSocket(3): not translated, it uses Minecraft's `Level`.
        // InfusionHint.beginFlash(2): not translated, it uses Minecraft's `Level`.
        // InfusionHint.tickFlash(2): not translated, it uses Minecraft's `Level`.
        // InfusionHint.wobble(1): not translated, it calls InfusionHint.age, which isn't translated.
    }

    /// `ScreenWobble` (org/betterx/betterend/client/effects/ScreenWobble.java)
    pub mod screen_wobble {
        use super::*;
        pub const DECAY: f32 = 14.0f32;
        pub const MAX_ROLL: f32 = 4.5f32;
        pub const MAX_PITCH: f32 = 2.5f32;
        #[derive(Clone, Debug)]
        pub struct ScreenWobble {
        }
        impl ScreenWobble {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ScreenWobble.at(1): not translated, it calls Minecraft's `Minecraft`.getInstance.
    }

    /// `EndStoneSmelterMenu` (org/betterx/betterend/client/gui/EndStoneSmelterMenu.java)
    pub mod end_stone_smelter_menu {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RecipeBookMenu`. Its static parts are below.
        pub const INGREDIENT_SLOT_A: i32 = 0;
        pub const INGREDIENT_SLOT_B: i32 = 1;
        pub const FUEL_SLOT: i32 = 2;
        pub const RESULT_SLOT: i32 = 3;
        pub const SLOT_COUNT: i32 = 4;
        pub const INV_SLOT_START: i32 = 4;
        pub const INV_SLOT_END: i32 = 31;
        pub const USE_ROW_SLOT_START: i32 = 31;
        pub const USE_ROW_SLOT_END: i32 = 40;
        // EndStoneSmelterMenu.EndStoneSmelterMenu(2): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.EndStoneSmelterMenu(4): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.getType(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.fillCraftSlotsStackedContents(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.clearCraftingContent(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.recipeMatches(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.handlePlacement(5): not translated, its body couldn't be read: an anonymous class.
        // EndStoneSmelterMenu.getRecipeBookType(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.isSmeltable(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.isFuel(1): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.quickMoveStack(2): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.getSmeltProgress(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.getFuelProgress(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
        // EndStoneSmelterMenu.isBurning(0): not translated, its class isn't translated (it extends Minecraft's `RecipeBookMenu`).
    }

    /// `EndStoneSmelterScreen` (org/betterx/betterend/client/gui/EndStoneSmelterScreen.java)
    pub mod end_stone_smelter_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerScreen`. Its static parts are below.
        // BACKGROUND_TEXTURE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EndStoneSmelterScreen.EndStoneSmelterScreen(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // EndStoneSmelterScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // EndStoneSmelterScreen.extractContents(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
    }

    /// `InfusionRecipeScreen` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java)
    pub mod infusion_recipe_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const PAGE_COLOR: i32 = -1556;
        pub const PAGE_SHADE_COLOR: i32 = -1056060;
        pub const FRAME_COLOR: i32 = -8438744;
        pub const FRAME_DARK_COLOR: i32 = -10868196;
        pub const TEXT_COLOR: i32 = -12566464;
        pub const TEXT_MUTED_COLOR: i32 = -7701924;
        pub static SLOT_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/slot"));
        pub static ARROW_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/villager/trade_arrow"));
        // COMPASS: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        pub const COMPASS_SIZE: i32 = 84;
        pub const SLOT_SIZE: i32 = 18;
        pub const ARROW_WIDTH: i32 = 20;
        pub const ARROW_HEIGHT: i32 = 18;
        pub const RING_RADIUS: i32 = 52;
        pub const PANEL_WIDTH: i32 = 330;
        pub const PANEL_HEIGHT: i32 = 194;
        pub const MARGIN: i32 = 8;
        pub const HEADER_HEIGHT: i32 = 20;
        pub const LIST_WIDTH: i32 = 118;
        pub const ROW_HEIGHT: i32 = 20;
        pub const INGREDIENT_CYCLE_TICKS: i32 = 20;
        // ORIENTATIONS: a game value, so it's in game_side.rs.
        // InfusionRecipeScreen.InfusionRecipeScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.loadRecipes(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.outputOf(1): not translated, it uses Minecraft's `ItemStack`.
        // InfusionRecipeScreen.displayName(1): not translated, it uses Minecraft's `Component`.
        // InfusionRecipeScreen.enchantmentName(2): not translated, it uses Minecraft's `MutableComponent`.
        // InfusionRecipeScreen.displayStack(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.ringCenterX(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.ringCenterY(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        pub fn catalyst_x(center_x: i32, index: i32) -> i32 {
            return center_x + ((j_round_d(52.0f64 * ((((index as f64) * std::f64::consts::PI) / 4.0f64).sin()))) as i32);
        }
        pub fn catalyst_y(center_y: i32, index: i32) -> i32 {
            return center_y - ((j_round_d(52.0f64 * ((((index as f64) * std::f64::consts::PI) / 4.0f64).cos()))) as i32);
        }
        // InfusionRecipeScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.extractPages(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.extractRecipe(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.extractSlot(7): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // InfusionRecipeScreen.isPauseScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `InfusionRecipeScreen.RecipeList` (org/betterx/betterend/client/gui/InfusionRecipeScreen.java)
    pub mod infusion_recipe_screen_recipe_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractScrollArea`. Its static parts are below.
        // InfusionRecipeScreen.RecipeList.RecipeList(4): not translated, its class isn't translated (it extends Minecraft's `AbstractScrollArea`).
        // InfusionRecipeScreen.RecipeList.contentHeight(0): not translated, its class isn't translated (it extends Minecraft's `AbstractScrollArea`).
        // InfusionRecipeScreen.RecipeList.rowWidth(0): not translated, its class isn't translated (it extends Minecraft's `AbstractScrollArea`).
        // InfusionRecipeScreen.RecipeList.indexAt(2): not translated, its class isn't translated (it extends Minecraft's `AbstractScrollArea`).
        // InfusionRecipeScreen.RecipeList.onClick(2): not translated, its class isn't translated (it extends Minecraft's `AbstractScrollArea`).
        // InfusionRecipeScreen.RecipeList.extractWidgetRenderState(4): not translated, its class isn't translated (it extends Minecraft's `AbstractScrollArea`).
        // InfusionRecipeScreen.RecipeList.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `AbstractScrollArea`).
    }

    /// `SmelterFuelSlot` (org/betterx/betterend/client/gui/slot/SmelterFuelSlot.java)
    pub mod smelter_fuel_slot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // SmelterFuelSlot.SmelterFuelSlot(5): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SmelterFuelSlot.mayPlace(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SmelterFuelSlot.getMaxStackSize(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `SmelterOutputSlot` (org/betterx/betterend/client/gui/slot/SmelterOutputSlot.java)
    pub mod smelter_output_slot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // SmelterOutputSlot.SmelterOutputSlot(5): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SmelterOutputSlot.mayPlace(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SmelterOutputSlot.remove(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SmelterOutputSlot.onTake(2): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SmelterOutputSlot.onQuickCraft(2): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SmelterOutputSlot.checkTakeAchievements(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `EndModels` (org/betterx/betterend/client/models/EndModels.java)
    pub mod end_models {
        use super::*;
        // ROOTS: not translated, it uses Minecraft's `TextureSlot`.
        // BASE: not translated, it uses Minecraft's `TextureSlot`.
        // PILLAR: not translated, it uses Minecraft's `TextureSlot`.
        // FLOOR: not translated, it uses Minecraft's `TextureSlot`.
        // CEIL: not translated, it uses Minecraft's `TextureSlot`.
        // WALL: not translated, it uses Minecraft's `TextureSlot`.
        // TWISTED_VINE: not translated, it uses Minecraft's `ModelTemplate`.
        // CROSS_NO_DISTORTION: not translated, it uses Minecraft's `ModelTemplate`.
        // CROSS_NO_DISTORTION_INVERTED: not translated, it uses Minecraft's `ModelTemplate`.
        // PATH: not translated, it uses Minecraft's `ModelTemplate`.
        // CUBE_NO_SHADE: not translated, it uses Minecraft's `ModelTemplate`.
        // FLOWER_POT: not translated, it uses Minecraft's `ModelTemplate`.
        // PEDESTAL_PILLAR: not translated, it uses Minecraft's `ModelTemplate`.
        // PEDESTAL_TOP: not translated, it uses Minecraft's `ModelTemplate`.
        // PEDESTAL_DEFAULT: not translated, it uses Minecraft's `ModelTemplate`.
        // PEDESTAL_COLUMN_TOP: not translated, it uses Minecraft's `ModelTemplate`.
        // PEDESTAL_COLUMN: not translated, it uses Minecraft's `ModelTemplate`.
        // PEDESTAL_BOTTOM: not translated, it uses Minecraft's `ModelTemplate`.
        // INFUSION_PEDESTAL_TOP: not translated, it uses Minecraft's `ModelTemplate`.
        // INFUSION_PEDESTAL_DEFAULT: not translated, it uses Minecraft's `ModelTemplate`.
        // PETAL_COLORED: not translated, it uses Minecraft's `ModelTemplate`.
        // LIT_STAIRS_INNER: not translated, it uses Minecraft's `ModelTemplate`.
        // LIT_STAIRS: not translated, it uses Minecraft's `ModelTemplate`.
        // LIT_STAIRS_OUTER: not translated, it uses Minecraft's `ModelTemplate`.
        // CHANDELIER_FLOOR: not translated, it uses Minecraft's `ModelTemplate`.
        // CHANDELIER_WALL: not translated, it uses Minecraft's `ModelTemplate`.
        // CHANDELIER_CEIL: not translated, it uses Minecraft's `ModelTemplate`.
        #[derive(Clone, Debug)]
        pub struct EndModels {
        }
        impl EndModels {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndModelTraits` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits {
        use super::*;
        // IDENTIFIER_SUPPLIER: not translated, it uses Minecraft's `Codec`.
        // TWISTED_VINE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // AMBER_MOSS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // CAVE_MOSS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // UNSHADED_CUBE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // HYDRALUX_PETAL_COLORED: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // ETERNAL_PEDESTAL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // INFUSION_PEDESTAL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // BLUE_VINE_LANTERN: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // UMBRELLA_TREE_MEMBRANE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // JELLYSHROOM_CAP: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // AMBER_MOSS_PATH: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // ROTATED_PILLAR: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // LIT_STAIRS: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // SLAB_FROM: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        #[derive(Clone, Debug)]
        pub struct EndModelTraits {
        }
        impl EndModelTraits {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
        // EndModelTraits.twistedVine(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.amberMoss(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.caveMoss(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.amberMossPath(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.rotatedPillar(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.unshadedCube(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.litStairs(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.slabFrom(2): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.flavoliteRunedVariants(3): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.flavoliteRuned(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.bulbMossMeshMembers(2): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.glowingPillarLeaves(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.ruscus(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.bulbMossTwoTexMembers(3): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.twistedMoss(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.bulbMossTemplateMembers(1): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.bulbMoss(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.jungleFernMembers(1): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.jungleFern(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.creepingMoss(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.upLeafChild(1): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.upLeaf(2): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.lucerniaHorizontal(1): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.lucerniaUp(1): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.lucerniaOuterLeaves(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.umbrellaMossSmallLayers(0): not translated, it uses `Layer` (de.ambertation.wover.block.api.model.WeightedTemplateModels).
        // EndModelTraits.umbrellaMoss(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.twistedUmbrellaMoss(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.hydraluxPetalColored(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.eternalPedestal(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.infusionPedestal(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.blueVineLantern(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.umbrellaTreeMembrane(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // EndModelTraits.jellyshroomCap(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
    }

    /// `EndModelTraits.AmberMossPath` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_amber_moss_path {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndModelTraits.AmberMossPath.AmberMossPath(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndModelTraits.AmberMossPath.base(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndModelTraits.AmberMossPathParams` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_amber_moss_path_params {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // EndModelTraits.AmberMossPathParams.AmberMossPathParams(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndModelTraits.AmberMossPathParams.base(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndModelTraits.Impl` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_impl {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Impl {
        }
        impl Impl {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndModelTraits.Impl.amberMossPathFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.rotatedPillarFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.litStairsFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.slabFromFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.hydraluxPetalColored(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.eternalPedestal(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.infusionPedestal(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.blueVineLantern(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.umbrellaTreeMembrane(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.jellyshroomCap(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.buildRotated(3): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.caveMoss(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.amberMoss(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.amberMossPath(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.rotatedPillar(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.unshadedCube(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.litStairs(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.slabFrom(2): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // EndModelTraits.Impl.twistedVine(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `EndModelTraits.LitStairs` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_lit_stairs {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndModelTraits.LitStairs.LitStairs(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // EndModelTraits.LitStairs.texture(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `EndModelTraits.LitStairsParams` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_lit_stairs_params {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // EndModelTraits.LitStairsParams.LitStairsParams(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // EndModelTraits.LitStairsParams.texture(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `EndModelTraits.RotatedPillar` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_rotated_pillar {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndModelTraits.RotatedPillar.RotatedPillar(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // EndModelTraits.RotatedPillar.texture(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `EndModelTraits.RotatedPillarParams` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_rotated_pillar_params {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // EndModelTraits.RotatedPillarParams.RotatedPillarParams(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // EndModelTraits.RotatedPillarParams.texture(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `EndModelTraits.SlabFrom` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_slab_from {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndModelTraits.SlabFrom.SlabFrom(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndModelTraits.SlabFrom.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndModelTraits.SlabFrom.texture(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndModelTraits.SlabFromParams` (org/betterx/betterend/client/models/EndModelTraits.java)
    pub mod end_model_traits_slab_from_params {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // EndModelTraits.SlabFromParams.SlabFromParams(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndModelTraits.SlabFromParams.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndModelTraits.SlabFromParams.texture(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `BeamRenderer` (org/betterx/betterend/client/render/BeamRenderer.java)
    pub mod beam_renderer {
        use super::*;
        pub static BEAM_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/entity/end_portal/end_gateway_beam.png"));
        #[derive(Clone, Debug)]
        pub struct BeamRenderer {
        }
        impl BeamRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BeamRenderer.renderLightBeam(10): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // BeamRenderer.renderBeam(20): not translated, it uses Minecraft's `Pose`.
        // BeamRenderer.renderBeam(17): not translated, it uses Minecraft's `Pose`.
        // BeamRenderer.addVertex(12): not translated, it uses Minecraft's `Pose`.
    }

    /// `BetterEndSkyRenderer` (org/betterx/betterend/client/render/BetterEndSkyRenderer.java)
    pub mod better_end_sky_renderer {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `StartMain`. Its static parts are below.
        // NEBULA_1: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // NEBULA_2: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // HORIZON: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // STARS: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // FOG: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // activeSolidRenderPass: not translated, it uses Minecraft's `RenderPass`.
        pub const HORIZON_RED: f32 = 0.38f32;
        pub const HORIZON_GREEN: f32 = 0.13f32;
        pub const HORIZON_BLUE: f32 = 0.4f32;
        // BetterEndSkyRenderer.setActiveSolidRenderPass(1): not translated, it uses Minecraft's `RenderPass`.
        // BetterEndSkyRenderer.initialise(0): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.register(0): not translated, it uses Fabric's `LevelRenderEvents`.START_MAIN.
        // BetterEndSkyRenderer.startMain(1): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.renderTexturedMesh(7): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.renderStarMesh(6): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.initStars(0): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.buildStarMesh(4): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.buildTexturedMesh(5): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.uploadMesh(2): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.makeStars(5): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.makeUVStars(5): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.makeFarFog(5): not translated, its class isn't translated (it implements Fabric's `StartMain`).
        // BetterEndSkyRenderer.makeCylinder(4): not translated, its class isn't translated (it implements Fabric's `StartMain`).
    }

    /// `BetterEndSkyRenderer.BufferFunction` (org/betterx/betterend/client/render/BetterEndSkyRenderer.java)
    pub mod better_end_sky_renderer_buffer_function {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BetterEndSkyRenderer.BufferFunction.make(5): not translated, it has no body.
    }

    /// `BetterEndSkyRenderer.SkyMesh` (org/betterx/betterend/client/render/BetterEndSkyRenderer.java)
    pub mod better_end_sky_renderer_sky_mesh {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GpuBuffer`. Its static parts are below.
        // BetterEndSkyRenderer.SkyMesh.SkyMesh(2): not translated, its class isn't translated (it uses Minecraft's `GpuBuffer`).
        // BetterEndSkyRenderer.SkyMesh.buffer(0): not translated, its class isn't translated (it uses Minecraft's `GpuBuffer`).
        // BetterEndSkyRenderer.SkyMesh.indexCount(0): not translated, its class isn't translated (it uses Minecraft's `GpuBuffer`).
    }

    /// `EndCrystalRenderer` (org/betterx/betterend/client/render/EndCrystalRenderer.java)
    pub mod end_crystal_renderer {
        use super::*;
        pub static CRYSTAL_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/entity/end_crystal/end_crystal.png"));
        // END_CRYSTAL: not translated, it uses Minecraft's `RenderType`.
        // CORE: not translated, it uses Minecraft's `ModelPart`.
        // FRAME: not translated, it uses Minecraft's `ModelPart`.
        pub const AGE_CYCLE: i32 = 240;
        pub static SINE_45_DEGREES: LazyLock<f32> = LazyLock::new(|| (((std::f64::consts::PI / 4.0f64).sin()) as f32));
        // ROTATOR: not translated, JOML's Quaternionf.setAngleAxis.
        #[derive(Clone, Debug)]
        pub struct EndCrystalRenderer {
        }
        impl EndCrystalRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndCrystalRenderer.render(6): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // EndCrystalRenderer.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
    }

    /// `EndFlowerPotModels` (org/betterx/betterend/client/render/EndFlowerPotModels.java)
    pub mod end_flower_pot_models {
        use super::*;
        pub const MODELS_PREFIX: &str = "models/";
        pub const MODELS_DIR: &str = "models/block";
        pub const POTTED_SUFFIX: &str = "_potted";
        pub const JSON_SUFFIX: &str = ".json";
        pub const MINECRAFT_NAMESPACE: &str = "minecraft";
        pub const VANILLA_POTTED_PREFIX: &str = "potted_";
        pub static VANILLA_POTTED_OVERRIDES: LazyLock<JMap<String, String>> = LazyLock::new(|| JMap::from_iter([("azalea".to_string(), "potted_azalea_bush".to_string()), ("flowering_azalea".to_string(), "potted_flowering_azalea_bush".to_string())]));
        // POTTED_KEYS: not translated, it uses Fabric's `ExtraModelKey`.
        #[derive(Clone, Debug)]
        pub struct EndFlowerPotModels {
        }
        impl EndFlowerPotModels {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndFlowerPotModels.register(0): not translated, a method reference.
        // EndFlowerPotModels.scan(2): not translated, it uses Minecraft's `SharedState`.
        // EndFlowerPotModels.initialize(2): not translated, it uses Fabric's `Context`.
        // EndFlowerPotModels.getPottedModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // EndFlowerPotModels.getVanillaPottedModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // EndFlowerPotModels.hasVanillaPottedModel(1): not translated, it uses Minecraft's `Block`.
    }

    /// `EndTinterKeys` (org/betterx/betterend/client/render/EndTinterKeys.java)
    pub mod end_tinter_keys {
        use super::*;
        // END_PORTAL: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        #[derive(Clone, Debug)]
        pub struct EndTinterKeys {
        }
        impl EndTinterKeys {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EternalCrystalRenderer` (org/betterx/betterend/client/render/EternalCrystalRenderer.java)
    pub mod eternal_crystal_renderer {
        use super::*;
        // RENDER_LAYER: not translated, it uses Minecraft's `RenderType`.
        // SHARDS: not translated, it uses Minecraft's `ModelPart`.
        // CORE: not translated, it uses Minecraft's `ModelPart`.
        #[derive(Clone, Debug)]
        pub struct EternalCrystalRenderer {
        }
        impl EternalCrystalRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EternalCrystalRenderer.render(5): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // EternalCrystalRenderer.render(6): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // EternalCrystalRenderer.colors(1): not translated, it calls `MHelper` (org.betterx.bclib.util).floor.
        // EternalCrystalRenderer.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
    }

    /// `FlowerPotItemRenderer` (org/betterx/betterend/client/render/FlowerPotItemRenderer.java)
    pub mod flower_pot_item_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        pub const INNER_MIN: f32 = 0.3125f32;
        pub const INNER_MAX: f32 = 0.6875f32;
        pub const SOIL_Y: f32 = 0.4375f32;
        pub const PLANT_SCALE: f32 = 0.85f32;
        pub const FALLBACK_PLANT_SCALE: f32 = 0.7f32;
        // FlowerPotItemRenderer.FlowerPotItemRenderer(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // FlowerPotItemRenderer.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // FlowerPotItemRenderer.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // FlowerPotItemRenderer.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // FlowerPotItemRenderer.renderSoilPlane(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // FlowerPotItemRenderer.emitVertex(10): not translated, it uses Minecraft's `VertexConsumer`.
        // FlowerPotItemRenderer.renderPlantModel(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // FlowerPotItemRenderer.renderBakedModel(6): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // FlowerPotItemRenderer.renderFallbackModel(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // FlowerPotItemRenderer.collectModelParts(2): not translated, it uses Minecraft's `BlockStateModelPart`.
        // FlowerPotItemRenderer.topSprite(1): not translated, it uses Minecraft's `TextureAtlasSprite`.
        // FlowerPotItemRenderer.firstRenderType(1): not translated, it uses Minecraft's `RenderType`.
    }

    /// `PedestalItemRenderer` (org/betterx/betterend/client/render/PedestalItemRenderer.java)
    pub mod pedestal_item_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        pub const SOCKET_MISSING_RGB: i32 = 16771488;
        pub const SOCKET_PLACED_RGB: i32 = 13216078;
        pub const SOCKET_NORTH_MISSING_RGB: i32 = 12946431;
        pub const SOCKET_NORTH_PLACED_RGB: i32 = 9068489;
        pub const FRAME_MISSING_RGB: i32 = 12159976;
        pub const CRYSTAL_GHOST_ALPHA: f32 = 0.55f32;
        // PedestalItemRenderer.PedestalItemRenderer(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // PedestalItemRenderer.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // PedestalItemRenderer.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // PedestalItemRenderer.extractInfusionHint(4): not translated, it uses Minecraft's `Level`.
        // PedestalItemRenderer.extractEternalVision(4): not translated, it uses Minecraft's `Level`.
        // PedestalItemRenderer.submitEternalVision(3): not translated, it uses PedestalRenderState, which isn't translated (it extends Minecraft's `BlockEntityRenderState`).
        // PedestalItemRenderer.submitInfusionHint(3): not translated, it uses PedestalRenderState, which isn't translated (it extends Minecraft's `BlockEntityRenderState`).
        // PedestalItemRenderer.submitOutline(5): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // PedestalItemRenderer.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // PedestalItemRenderer.getGemAge(0): not translated, it calls Minecraft's `Minecraft`.getInstance.
    }

    /// `FlowerPotRenderState` (org/betterx/betterend/client/render/state/FlowerPotRenderState.java)
    pub mod flower_pot_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `PedestalRenderState` (org/betterx/betterend/client/render/state/PedestalRenderState.java)
    pub mod pedestal_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `CommandRegistry` (org/betterx/betterend/commands/CommandRegistry.java)
    pub mod command_registry {
        use super::*;
        pub static BIOME_INDEX: JCell<i32> = JCell::new(0);
        pub const MAX_SEARCH_RADIUS: i32 = 12800;
        pub const SAMPLE_RESOLUTION_HORIZONTAL: i32 = 32;
        pub const SAMPLE_RESOLUTION_VERTICAL: i32 = 64;
        // ERROR_BIOME_NOT_FOUND: not translated, it uses Minecraft's `DynamicCommandExceptionType`.
        #[derive(Clone, Debug)]
        pub struct CommandRegistry {
        }
        impl CommandRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommandRegistry.register(0): not translated, it uses Fabric's `CommandRegistrationCallback`.EVENT.
        // CommandRegistry.register(3): not translated, it uses Minecraft's `CommandDispatcher`.
        // CommandRegistry.find_poi(2): not translated, it uses Minecraft's `CommandContext`.
        // CommandRegistry.teleportToNextBiome(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `ColoredMaterial` (org/betterx/betterend/complexmaterials/ColoredMaterial.java)
    pub mod colored_material {
        use super::*;
        // The class itself isn't translated: it uses `BlockKey` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // DYES: not translated, it uses Minecraft's `ItemLike`.
        // COLORS: not translated, it calls `Maps` (com.google.common.collect).newLinkedHashMap.
        // VANILLA_COLORS: not translated, it uses Minecraft's `DyeColor`.
        // ColoredMaterial.ColoredMaterial(8): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredMaterial.ColoredMaterial(10): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredMaterial.register(1): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredMaterial.keys(0): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredMaterial.dye(1): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredMaterial.source(0): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredMaterial.craftsEight(0): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredMaterial.getByColor(1): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredMaterial.getByColor(1): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
    }

    /// `CrystalSubblocksMaterial` (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java)
    pub mod crystal_subblocks_material {
        use super::*;
        // The class itself isn't translated: it uses `BlockKey` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // BRICK_SLAB_MODEL: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // CrystalSubblocksMaterial.CrystalSubblocksMaterial(2): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // CrystalSubblocksMaterial.key(1): not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
    }

    /// `CrystalSubblocksMaterial.BuildPedestalParams` (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java)
    pub mod crystal_subblocks_material_build_pedestal_params {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CrystalSubblocksMaterial.BuildPedestalParams.BuildPedestalParams(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CrystalSubblocksMaterial.BuildPedestalParams.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `CrystalSubblocksMaterial.BuildSlabParams` (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java)
    pub mod crystal_subblocks_material_build_slab_params {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CrystalSubblocksMaterial.BuildSlabParams.BuildSlabParams(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CrystalSubblocksMaterial.BuildSlabParams.baseBlock(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `CrystalSubblocksMaterial.ClientModel` (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java)
    pub mod crystal_subblocks_material_client_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModel {
        }
        impl ClientModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CrystalSubblocksMaterial.ClientModel.buildSlabFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // CrystalSubblocksMaterial.ClientModel.buildPedestalFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // CrystalSubblocksMaterial.ClientModel.buildSlab(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // CrystalSubblocksMaterial.ClientModel.buildPedestal(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `CrystalSubblocksMaterial.CrystalBrickSlab` (org/betterx/betterend/complexmaterials/CrystalSubblocksMaterial.java)
    pub mod crystal_subblocks_material_crystal_brick_slab {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CrystalSubblocksMaterial.CrystalBrickSlab.CrystalBrickSlab(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CrystalSubblocksMaterial.CrystalBrickSlab.baseBlock(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndWoodenComplexMaterial` (org/betterx/betterend/complexmaterials/EndWoodenComplexMaterial.java)
    pub mod end_wooden_complex_material {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `MapColor`. Its static parts are below.
        // EndWoodenComplexMaterial.EndWoodenComplexMaterial(3): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.setFurnitureCloth(1): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.setLogVariants(1): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.setStrippedVariants(1): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.useRaft(0): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.build(0): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.register(1): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.defineBlocks(1): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.addFurniture(3): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.furniture(3): not translated, it uses `EntryConfig` (de.ambertation.wover.sets.api.classify.BlockSet).
        // EndWoodenComplexMaterial.weightedLogAndBark(3): not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // EndWoodenComplexMaterial.isLogKind(1): not translated, it uses `Kind` (de.ambertation.wover.block.api.classify).
        // EndWoodenComplexMaterial.isFurniture(1): not translated, it uses `Kind` (de.ambertation.wover.block.api.classify).
        // EndWoodenComplexMaterial.blocks(0): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.woodType(0): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.isTreeLog(1): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.isTreeLog(1): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.getLog(0): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
        // EndWoodenComplexMaterial.getBark(0): not translated, its class isn't translated (it uses Minecraft's `MapColor`).
    }

    /// `JellyLucerniaWoodMaterial` (org/betterx/betterend/complexmaterials/JellyLucerniaWoodMaterial.java)
    pub mod jelly_lucernia_wood_material {
        use super::*;
        // The class itself isn't translated: it extends `EndWoodenComplexMaterial`. Its static parts are below.
        // JellyLucerniaWoodMaterial.JellyLucerniaWoodMaterial(0): not translated, its class isn't translated (it extends `EndWoodenComplexMaterial`).
        // JellyLucerniaWoodMaterial.defineBlocks(1): not translated, its class isn't translated (it extends `EndWoodenComplexMaterial`).
    }

    /// `MetalMaterial` (org/betterx/betterend/complexmaterials/MetalMaterial.java)
    pub mod metal_material {
        use super::*;
        // The class itself isn't translated: it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)). Its static parts are below.
        // MetalMaterial.makeNormal(6): not translated, it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // MetalMaterial.makeNormal(7): not translated, it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // MetalMaterial.makeOreless(8): not translated, it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // MetalMaterial.makeOreless(9): not translated, it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // MetalMaterial.MetalMaterial(11): not translated, its class isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // MetalMaterial.oreInstance(0): not translated, its class isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // MetalMaterial.harvestTierTag(0): not translated, its class isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // MetalMaterial.registerRecipes(1): not translated, its class isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
    }

    /// `StoneMaterial` (org/betterx/betterend/complexmaterials/StoneMaterial.java)
    pub mod stone_material {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StoneMaterial {
        }
        impl StoneMaterial {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StoneMaterial.declare(2): not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
    }

    /// `VanillaMetalSet` (org/betterx/betterend/complexmaterials/VanillaMetalSet.java)
    pub mod vanilla_metal_set {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VanillaMetalSet {
        }
        impl VanillaMetalSet {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VanillaMetalSet.declare(5): not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
    }

    /// `VanillaStoneSet` (org/betterx/betterend/complexmaterials/VanillaStoneSet.java)
    pub mod vanilla_stone_set {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VanillaStoneSet {
        }
        impl VanillaStoneSet {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VanillaStoneSet.declare(3): not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
    }

    /// `VanillaVariantStoneMaterial` (org/betterx/betterend/complexmaterials/VanillaVariantStoneMaterial.java)
    pub mod vanilla_variant_stone_material {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VanillaVariantStoneMaterial {
        }
        impl VanillaVariantStoneMaterial {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VanillaVariantStoneMaterial.declare(3): not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
    }

    /// `ClientConfig` (org/betterx/betterend/config/ClientConfig.java)
    pub mod client_config {
        use super::*;
        // The class itself isn't translated: it extends `ConfigFile` (de.ambertation.wunderlib.configs). Its static parts are below.
        // ClientConfig.ClientConfig(0): not translated, its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
    }

    /// `Configs` (org/betterx/betterend/config/Configs.java)
    pub mod configs {
        use super::*;
        // GENERATOR_CONFIG: not translated, it uses GeneratorConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
        // CLIENT_CONFIG: not translated, it uses ClientConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
        #[derive(Clone, Debug)]
        pub struct Configs {
        }
        impl Configs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Configs.saveConfigs(0): not translated, unknown name `de`.
    }

    /// `GeneratorConfig` (org/betterx/betterend/config/GeneratorConfig.java)
    pub mod generator_config {
        use super::*;
        // The class itself isn't translated: it extends `ConfigFile` (de.ambertation.wunderlib.configs). Its static parts are below.
        // GENERATOR: not translated, it uses `Group` (de.ambertation.wunderlib.configs.AbstractConfig).
        // GeneratorConfig.GeneratorConfig(0): not translated, its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
    }

    /// `GeneratorConfig.LayerOptionsValue` (org/betterx/betterend/config/GeneratorConfig.java)
    pub mod generator_config_layer_options_value {
        use super::*;
        // The class itself isn't translated: it extends `Value`. Its static parts are below.
        // GeneratorConfig.LayerOptionsValue.LayerOptionsValue(3): not translated, its class isn't translated (it extends `Value`).
        // GeneratorConfig.LayerOptionsValue.LayerOptionsValue(4): not translated, its class isn't translated (it extends `Value`).
        // GeneratorConfig.LayerOptionsValue.LayerOptionsValue(1): not translated, its class isn't translated (it extends `Value`).
        // GeneratorConfig.LayerOptionsValue.LayerOptionsValue(2): not translated, its class isn't translated (it extends `Value`).
        // GeneratorConfig.LayerOptionsValue.convert(1): not translated, its class isn't translated (it extends `Value`).
        // GeneratorConfig.LayerOptionsValue.convert(1): not translated, its class isn't translated (it extends `Value`).
        // GeneratorConfig.LayerOptionsValue.parseString(1): not translated, its class isn't translated (it extends `Value`).
    }

    /// `ConfigScreen` (org/betterx/betterend/config/screen/ConfigScreen.java)
    pub mod config_screen {
        use super::*;
        // The class itself isn't translated: it extends `ConfigScreen`. Its static parts are below.
        // BE_LOGO_LOCATION: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // ConfigScreen.ConfigScreen(1): not translated, its class isn't translated (it extends `ConfigScreen`).
        // ConfigScreen.onClose(0): not translated, its class isn't translated (it extends `ConfigScreen`).
    }

    /// `EndPotions` (org/betterx/betterend/effects/EndPotions.java)
    pub mod end_potions {
        use super::*;
        // END_VEIL: not translated, it uses Minecraft's `Holder`.
        // LONG_END_VEIL: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct EndPotions {
        }
        impl EndPotions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndPotions.register(0): not translated, it uses `PotionManager` (de.ambertation.wover.potions.api).BOOTSTRAP_POTIONS.
        // EndPotions.bootstrap(1): not translated, it uses `BrewingBuilder` (de.ambertation.wover.potions.api).
    }

    /// `EndStatusEffects` (org/betterx/betterend/effects/EndStatusEffects.java)
    pub mod end_status_effects {
        use super::*;
        // CRYSTALITE_HEALTH_REGEN: not translated, it uses Minecraft's `MobEffectInstance`.
        // CRYSTALITE_DIG_SPEED: not translated, it uses Minecraft's `MobEffectInstance`.
        // CRYSTALITE_MOVE_SPEED: not translated, it uses Minecraft's `MobEffectInstance`.
        // END_VEIL: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct EndStatusEffects {
        }
        impl EndStatusEffects {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndStatusEffects.registerEffect(2): not translated, it's generic.
        pub fn ensure_statically_loaded() {
        }
    }

    /// `EndVeilEffect` (org/betterx/betterend/effects/status/EndVeilEffect.java)
    pub mod end_veil_effect {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobEffect`. Its static parts are below.
        // EndVeilEffect.EndVeilEffect(0): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // EndVeilEffect.shouldApplyEffectTickThisTick(2): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
    }

    /// `CubozoaEntity` (org/betterx/betterend/entity/CubozoaEntity.java)
    pub mod cubozoa_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSchoolingFish`. Its static parts are below.
        // VARIANT: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SCALE: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CubozoaEntity.CubozoaEntity(2): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.finalizeSpawn(4): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.saveToBucketTag(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.getBucketItemStack(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.createMobAttributes(0): not translated, it uses Builder, which isn't translated (it's an interface).
        // CubozoaEntity.getVariant(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.updateScaleAttribute(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.getStandingEyeHeight(2): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.dropFromLootTable(3): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.getFlopSound(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // CubozoaEntity.playerTouch(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
    }

    /// `DragonflyEntity` (org/betterx/betterend/entity/DragonflyEntity.java)
    pub mod dragonfly_entity {
        use super::*;
        // The class itself isn't translated: it extends `DespawnableAnimal` (org.betterx.bclib.entity). Its static parts are below.
        // DragonflyEntity.DragonflyEntity(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.createMobAttributes(0): not translated, it uses Builder, which isn't translated (it's an interface).
        // DragonflyEntity.canBeLeashed(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.createNavigation(1): not translated, its body couldn't be read: an anonymous class.
        // DragonflyEntity.getWalkTargetValue(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.isFood(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.registerGoals(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.isPushable(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.causeFallDamage(3): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.getMovementEmission(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.omnidirectionalAirMover(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.isNoGravity(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.getAmbientSound(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.getSoundVolume(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // DragonflyEntity.getBreedOffspring(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
    }

    /// `DragonflyEntity.DragonflyLookControl` (org/betterx/betterend/entity/DragonflyEntity.java)
    pub mod dragonfly_entity_dragonfly_look_control {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LookControl`. Its static parts are below.
        // DragonflyEntity.DragonflyLookControl.DragonflyLookControl(1): not translated, its class isn't translated (it extends Minecraft's `LookControl`).
        // DragonflyEntity.DragonflyLookControl.resetXRotOnTick(0): not translated, its class isn't translated (it extends Minecraft's `LookControl`).
    }

    /// `DragonflyEntity.WanderAroundGoal` (org/betterx/betterend/entity/DragonflyEntity.java)
    pub mod dragonfly_entity_wander_around_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // DragonflyEntity.WanderAroundGoal.WanderAroundGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // DragonflyEntity.WanderAroundGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // DragonflyEntity.WanderAroundGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // DragonflyEntity.WanderAroundGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // DragonflyEntity.WanderAroundGoal.getRandomLocation(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // DragonflyEntity.WanderAroundGoal.isInVoid(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EndFishEntity` (org/betterx/betterend/entity/EndFishEntity.java)
    pub mod end_fish_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSchoolingFish`. Its static parts are below.
        pub const VARIANTS_NORMAL: i32 = 5;
        pub const VARIANTS_SULPHUR: i32 = 3;
        pub const VARIANTS: i32 = 8;
        // VARIANT: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SCALE: not translated, it uses Minecraft's `EntityDataAccessor`.
        // EndFishEntity.EndFishEntity(2): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.finalizeSpawn(4): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.saveToBucketTag(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.getBucketItemStack(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.getFlopSound(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.getAmbientSound(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.getDeathSound(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.getHurtSound(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.createMobAttributes(0): not translated, it uses Builder, which isn't translated (it's an interface).
        // EndFishEntity.getVariant(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.updateScaleAttribute(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
        // EndFishEntity.dropFromLootTable(3): not translated, its class isn't translated (it extends Minecraft's `AbstractSchoolingFish`).
    }

    /// `EndSlimeEntity` (org/betterx/betterend/entity/EndSlimeEntity.java)
    pub mod end_slime_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slime`. Its static parts are below.
        // VARIANT: not translated, it uses Minecraft's `EntityDataAccessor`.
        // EndSlimeEntity.EndSlimeEntity(2): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.registerGoals(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.createMobAttributes(0): not translated, it uses Builder, which isn't translated (it's an interface).
        // EndSlimeEntity.finalizeSpawn(4): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.getParticleType(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.setUpSplitCube(4): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.dropFromLootTable(3): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.getSlimeType(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.setSlimeType(1): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.setMossy(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.isMossy(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.setLake(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.isLake(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.setAmber(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.isAmber(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.isChorus(0): not translated, its class isn't translated (it extends Minecraft's `Slime`).
        // EndSlimeEntity.canSpawn(5): not translated, it uses Minecraft's `EntityType`.
        // EndSlimeEntity.isWaterNear(2): not translated, it uses Minecraft's `LevelAccessor`.
    }

    /// `EndSlimeEntity.EndSlimeMoveControl` (org/betterx/betterend/entity/EndSlimeEntity.java)
    pub mod end_slime_entity_end_slime_move_control {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MoveControl`. Its static parts are below.
        // EndSlimeEntity.EndSlimeMoveControl.EndSlimeMoveControl(1): not translated, its class isn't translated (it extends Minecraft's `MoveControl`).
        // EndSlimeEntity.EndSlimeMoveControl.look(2): not translated, its class isn't translated (it extends Minecraft's `MoveControl`).
        // EndSlimeEntity.EndSlimeMoveControl.move(1): not translated, its class isn't translated (it extends Minecraft's `MoveControl`).
        // EndSlimeEntity.EndSlimeMoveControl.tick(0): not translated, its class isn't translated (it extends Minecraft's `MoveControl`).
        // EndSlimeEntity.EndSlimeMoveControl.getJumpSoundPitch(0): not translated, its class isn't translated (it extends Minecraft's `MoveControl`).
    }

    /// `EndSlimeEntity.FaceTowardTargetGoal` (org/betterx/betterend/entity/EndSlimeEntity.java)
    pub mod end_slime_entity_face_toward_target_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EndSlimeEntity.FaceTowardTargetGoal.FaceTowardTargetGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.FaceTowardTargetGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.FaceTowardTargetGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.FaceTowardTargetGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.FaceTowardTargetGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EndSlimeEntity.MoveGoal` (org/betterx/betterend/entity/EndSlimeEntity.java)
    pub mod end_slime_entity_move_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EndSlimeEntity.MoveGoal.MoveGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.MoveGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.MoveGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EndSlimeEntity.RandomLookGoal` (org/betterx/betterend/entity/EndSlimeEntity.java)
    pub mod end_slime_entity_random_look_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EndSlimeEntity.RandomLookGoal.RandomLookGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.RandomLookGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.RandomLookGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EndSlimeEntity.SwimmingGoal` (org/betterx/betterend/entity/EndSlimeEntity.java)
    pub mod end_slime_entity_swimming_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EndSlimeEntity.SwimmingGoal.SwimmingGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.SwimmingGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EndSlimeEntity.SwimmingGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `CubozoaEntityModel` (org/betterx/betterend/entity/model/CubozoaEntityModel.java)
    pub mod cubozoa_entity_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        pub const TENTACLE_COUNT: i32 = 4;
        // CubozoaEntityModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // CubozoaEntityModel.CubozoaEntityModel(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // CubozoaEntityModel.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `DragonflyEntityModel` (org/betterx/betterend/entity/model/DragonflyEntityModel.java)
    pub mod dragonfly_entity_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        // DragonflyEntityModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // DragonflyEntityModel.DragonflyEntityModel(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // DragonflyEntityModel.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `EndFishEntityModel` (org/betterx/betterend/entity/model/EndFishEntityModel.java)
    pub mod end_fish_entity_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        // EndFishEntityModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // EndFishEntityModel.EndFishEntityModel(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // EndFishEntityModel.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `EndSlimeEntityModel` (org/betterx/betterend/entity/model/EndSlimeEntityModel.java)
    pub mod end_slime_entity_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        // EndSlimeEntityModel.getShellOnlyTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // EndSlimeEntityModel.getCompleteTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // EndSlimeEntityModel.getTexturedModelData(1): not translated, it uses Minecraft's `LayerDefinition`.
        // EndSlimeEntityModel.EndSlimeEntityModel(2): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // EndSlimeEntityModel.EndSlimeEntityModel(2): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // EndSlimeEntityModel.buildRenderRoot(2): not translated, it uses Minecraft's `ModelPart`.
        // EndSlimeEntityModel.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // EndSlimeEntityModel.renderFlower(4): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // EndSlimeEntityModel.renderCrop(4): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `SilkMothEntityModel` (org/betterx/betterend/entity/model/SilkMothEntityModel.java)
    pub mod silk_moth_entity_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        // SilkMothEntityModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // SilkMothEntityModel.SilkMothEntityModel(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // SilkMothEntityModel.buildRenderRoot(1): not translated, it uses Minecraft's `ModelPart`.
        // SilkMothEntityModel.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `RendererEntityCubozoa` (org/betterx/betterend/entity/render/RendererEntityCubozoa.java)
    pub mod renderer_entity_cubozoa {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        pub static TEXTURE: LazyLock<Vec<JIdentifier>> = LazyLock::new(|| vec![JIdentifier::new("", ""); 2]);
        // GLOW: not translated, it uses Minecraft's `RenderType`.
        // RendererEntityCubozoa.RendererEntityCubozoa(1): not translated, its body couldn't be read: an anonymous class.
        // RendererEntityCubozoa.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityCubozoa.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityCubozoa.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RendererEntityDragonfly` (org/betterx/betterend/entity/render/RendererEntityDragonfly.java)
    pub mod renderer_entity_dragonfly {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // GLOW: not translated, it uses Minecraft's `RenderType`.
        // RendererEntityDragonfly.RendererEntityDragonfly(1): not translated, its body couldn't be read: an anonymous class.
        // RendererEntityDragonfly.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityDragonfly.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RendererEntityEndFish` (org/betterx/betterend/entity/render/RendererEntityEndFish.java)
    pub mod renderer_entity_end_fish {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        pub static TEXTURE: LazyLock<Vec<JIdentifier>> = LazyLock::new(|| vec![JIdentifier::new("", ""); 8]);
        // GLOW: not translated, it uses Minecraft's `RenderType`.
        // RendererEntityEndFish.RendererEntityEndFish(1): not translated, its body couldn't be read: an anonymous class.
        // RendererEntityEndFish.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityEndFish.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityEndFish.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RendererEntityEndSlime` (org/betterx/betterend/entity/render/RendererEntityEndSlime.java)
    pub mod renderer_entity_end_slime {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        pub static TEXTURE: LazyLock<Vec<JIdentifier>> = LazyLock::new(|| vec![JIdentifier::new("", ""); 4]);
        // GLOW: not translated, it uses Minecraft's `RenderType`.
        // RendererEntityEndSlime.RendererEntityEndSlime(1): not translated, its body couldn't be read: an anonymous class.
        // RendererEntityEndSlime.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityEndSlime.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityEndSlime.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityEndSlime.getShadowRadius(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RendererEntityEndSlime.scale(2): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RendererEntityEndSlime.OverlayFeatureRenderer` (org/betterx/betterend/entity/render/RendererEntityEndSlime.java)
    pub mod renderer_entity_end_slime_overlay_feature_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RenderLayer`. Its static parts are below.
        // RendererEntityEndSlime.OverlayFeatureRenderer.OverlayFeatureRenderer(2): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // RendererEntityEndSlime.OverlayFeatureRenderer.submit(6): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // RendererEntityEndSlime.OverlayFeatureRenderer.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
    }

    /// `RendererEntityShadowWalker` (org/betterx/betterend/entity/render/RendererEntityShadowWalker.java)
    pub mod renderer_entity_shadow_walker {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HumanoidMobRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // RendererEntityShadowWalker.RendererEntityShadowWalker(1): not translated, its class isn't translated (it extends Minecraft's `HumanoidMobRenderer`).
        // RendererEntityShadowWalker.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `HumanoidMobRenderer`).
        // RendererEntityShadowWalker.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `HumanoidMobRenderer`).
    }

    /// `SilkMothEntityRenderer` (org/betterx/betterend/entity/render/SilkMothEntityRenderer.java)
    pub mod silk_moth_entity_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // SilkMothEntityRenderer.SilkMothEntityRenderer(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // SilkMothEntityRenderer.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // SilkMothEntityRenderer.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `CubozoaRenderState` (org/betterx/betterend/entity/render/state/CubozoaRenderState.java)
    pub mod cubozoa_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `DragonflyRenderState` (org/betterx/betterend/entity/render/state/DragonflyRenderState.java)
    pub mod dragonfly_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `EndFishRenderState` (org/betterx/betterend/entity/render/state/EndFishRenderState.java)
    pub mod end_fish_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `EndSlimeRenderState` (org/betterx/betterend/entity/render/state/EndSlimeRenderState.java)
    pub mod end_slime_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SlimeRenderState`. Its static parts are below.
    }

    /// `ShadowWalkerRenderState` (org/betterx/betterend/entity/render/state/ShadowWalkerRenderState.java)
    pub mod shadow_walker_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HumanoidRenderState`. Its static parts are below.
    }

    /// `SilkMothRenderState` (org/betterx/betterend/entity/render/state/SilkMothRenderState.java)
    pub mod silk_moth_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `ShadowWalkerEntity` (org/betterx/betterend/entity/ShadowWalkerEntity.java)
    pub mod shadow_walker_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Monster`. Its static parts are below.
        // ShadowWalkerEntity.ShadowWalkerEntity(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.registerGoals(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.createMobAttributes(0): not translated, it uses Builder, which isn't translated (it's an interface).
        // ShadowWalkerEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.getAmbientSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.getHurtSound(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.getDeathSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.playStepSound(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.getSoundVolume(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.getVoicePitch(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // ShadowWalkerEntity.doHurtTarget(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
    }

    /// `ShadowWalkerEntity.AttackGoal` (org/betterx/betterend/entity/ShadowWalkerEntity.java)
    pub mod shadow_walker_entity_attack_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MeleeAttackGoal`. Its static parts are below.
        // ShadowWalkerEntity.AttackGoal.AttackGoal(3): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
        // ShadowWalkerEntity.AttackGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
        // ShadowWalkerEntity.AttackGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
        // ShadowWalkerEntity.AttackGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
    }

    /// `SilkMothEntity` (org/betterx/betterend/entity/SilkMothEntity.java)
    pub mod silk_moth_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Animal`. Its static parts are below.
        // SilkMothEntity.SilkMothEntity(2): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.createMobAttributes(0): not translated, it uses Builder, which isn't translated (it's an interface).
        // SilkMothEntity.setHive(2): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.canBeLeashed(0): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.registerGoals(0): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.createNavigation(1): not translated, its body couldn't be read: an anonymous class.
        // SilkMothEntity.isPushable(0): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.causeFallDamage(3): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.getMovementEmission(0): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.omnidirectionalAirMover(0): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.isNoGravity(0): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.getBreedOffspring(2): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.dropFromLootTable(3): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.isFood(1): not translated, its class isn't translated (it extends Minecraft's `Animal`).
        // SilkMothEntity.mobInteract(2): not translated, its class isn't translated (it extends Minecraft's `Animal`).
    }

    /// `SilkMothEntity.MothLookControl` (org/betterx/betterend/entity/SilkMothEntity.java)
    pub mod silk_moth_entity_moth_look_control {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LookControl`. Its static parts are below.
        // SilkMothEntity.MothLookControl.MothLookControl(1): not translated, its class isn't translated (it extends Minecraft's `LookControl`).
        // SilkMothEntity.MothLookControl.resetXRotOnTick(0): not translated, its class isn't translated (it extends Minecraft's `LookControl`).
    }

    /// `SilkMothEntity.ReturnToHiveGoal` (org/betterx/betterend/entity/SilkMothEntity.java)
    pub mod silk_moth_entity_return_to_hive_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // SilkMothEntity.ReturnToHiveGoal.ReturnToHiveGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SilkMothEntity.ReturnToHiveGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SilkMothEntity.ReturnToHiveGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SilkMothEntity.ReturnToHiveGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SilkMothEntity.ReturnToHiveGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `SilkMothEntity.WanderAroundGoal` (org/betterx/betterend/entity/SilkMothEntity.java)
    pub mod silk_moth_entity_wander_around_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // SilkMothEntity.WanderAroundGoal.WanderAroundGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SilkMothEntity.WanderAroundGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SilkMothEntity.WanderAroundGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SilkMothEntity.WanderAroundGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SilkMothEntity.WanderAroundGoal.getRandomLocation(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `ItemTooltipCallback` (org/betterx/betterend/events/ItemTooltipCallback.java)
    pub mod item_tooltip_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ItemTooltipCallback.getTooltip(4): not translated, it has no body.
    }

    /// `PlayerAdvancementsCallback` (org/betterx/betterend/events/PlayerAdvancementsCallback.java)
    pub mod player_advancements_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PlayerAdvancementsCallback.onAdvancementComplete(3): not translated, it has no body.
    }

    /// `FlowerPotMigration` (org/betterx/betterend/FlowerPotMigration.java)
    pub mod flower_pot_migration {
        use super::*;
        pub const NAMESPACE: &str = "betterend:";
        pub const BLOCK_SUFFIX: &str = "_flower_pot";
        pub const BLOCK_ENTITY: &str = "betterend:flower_pot";
        pub const PLANT_ID: &str = "plant_id";
        pub const SOIL_ID: &str = "soil_id";
        pub static DEFAULT_PLANTS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["mossy_glowshroom_sapling".to_string(), "pythadendron_sapling".to_string(), "pythadendron_leaves".to_string(), "end_lotus_flower".to_string(), "lacugrove_sapling".to_string(), "lacugrove_leaves".to_string(), "dragon_tree_sapling".to_string(), "dragon_tree_leaves".to_string(), "tenanea_sapling".to_string(), "tenanea_leaves".to_string(), "helix_tree_sapling".to_string(), "umbrella_tree_sapling".to_string(), "lucernia_sapling".to_string(), "lucernia_leaves".to_string(), "umbrella_moss".to_string(), "creeping_moss".to_string(), "chorus_grass".to_string(), "cave_grass".to_string(), "crystal_grass".to_string(), "shadow_plant".to_string(), "bushy_grass".to_string(), "amber_grass".to_string(), "twisted_umbrella_moss".to_string(), "jungle_grass".to_string(), "blooming_cooksonia".to_string(), "salteago".to_string(), "vaiolush_fern".to_string(), "fracturn".to_string(), "clawfern".to_string(), "globulagus".to_string(), "orango".to_string(), "aeridium".to_string(), "lutebus".to_string(), "lamellarium".to_string(), "inflexia".to_string(), "flammalix".to_string(), "small_jellyshroom".to_string(), "bolux_mushroom".to_string(), "small_amaranita_mushroom".to_string(), "neon_cactus".to_string(), "shadow_berry".to_string(), "blossom_berry_seed".to_string(), "amber_root_seed".to_string(), "chorus_mushroom_seed".to_string(), "murkweed".to_string(), "needlegrass".to_string()]);
        pub static DEFAULT_SOILS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["end_mycelium".to_string(), "end_moss".to_string(), "chorus_nylium".to_string(), "cave_moss".to_string(), "crystal_moss".to_string(), "shadow_grass".to_string(), "pink_moss".to_string(), "amber_moss".to_string(), "jungle_moss".to_string(), "sangnum".to_string(), "rutiscus".to_string(), "pallidium_full".to_string()]);
        pub static PLANTS: LazyLock<JCell<Option<JMap<i32, String>>>> = LazyLock::new(|| JCell::new(None));
        pub static SOILS: LazyLock<JCell<Option<JMap<i32, String>>>> = LazyLock::new(|| JCell::new(None));
        #[derive(Clone, Debug)]
        pub struct FlowerPotMigration {
        }
        impl FlowerPotMigration {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FlowerPotMigration.loadTables(0): not translated, new HashMap<>() without its types.
        // FlowerPotMigration.readTable(3): not translated, it changes an object through a second name.
        // FlowerPotMigration.patchChunk(2): not translated, it uses Minecraft's `CompoundTag`.
        // FlowerPotMigration.lookup(2): not translated, it calls `Integer`.parseInt.
        // FlowerPotMigration.hasEntityAt(2): not translated, it uses Minecraft's `ListTag`.
        // FlowerPotMigration.unpack(2): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `EndBiomeIntegration` (org/betterx/betterend/integration/EndBiomeIntegration.java)
    pub mod end_biome_integration {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EndBiomeIntegration.addBiomes(0): not translated, it has no body.
    }

    /// `FlamboyantColors` (org/betterx/betterend/integration/FlamboyantColors.java)
    pub mod flamboyant_colors {
        use super::*;
        pub static ALL: LazyLock<Vec<flamboyant_colors_color::Color>> = LazyLock::new(|| vec![flamboyant_colors_color::Color::new("fead1d", "amber"), flamboyant_colors_color::Color::new("bd9a5f", "beige"), flamboyant_colors_color::Color::new("edeada", "cream"), flamboyant_colors_color::Color::new("33430e", "dark_green"), flamboyant_colors_color::Color::new("639920", "forest_green"), flamboyant_colors_color::Color::new("f0618c", "hot_pink"), flamboyant_colors_color::Color::new("491c7b", "indigo"), flamboyant_colors_color::Color::new("65291b", "maroon"), flamboyant_colors_color::Color::new("2c3969", "navy"), flamboyant_colors_color::Color::new("827c17", "olive"), flamboyant_colors_color::Color::new("7bc618", "pale_green"), flamboyant_colors_color::Color::new("f4a4bd", "pale_pink"), flamboyant_colors_color::Color::new("f8d45a", "pale_yellow"), flamboyant_colors_color::Color::new("6bb1cf", "sky_blue"), flamboyant_colors_color::Color::new("6e8c9c", "slate_gray"), flamboyant_colors_color::Color::new("b02454", "violet")]);
        #[derive(Clone, Debug)]
        pub struct FlamboyantColors {
        }
        impl FlamboyantColors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FlamboyantColors.Color` (org/betterx/betterend/integration/FlamboyantColors.java)
    pub mod flamboyant_colors_color {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Color {
            pub hex: String,
            pub name: String,
        }
        impl Color {
            pub fn new(hex: &str, name: &str) -> Self {
                let mut s = Self { hex: String::new(), name: String::new() };
                s.hex = hex.to_string();
                s.name = name.to_string();
                s
            }
            // FlamboyantColors.Color.argb(0): not translated, it calls `ColorUtilARGB32` (de.ambertation.wunderlib.utils).color.
            // FlamboyantColors.Color.dye(0): not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
            pub fn hex(&self) -> String {
                return self.hex.to_string();
            }
            pub fn name(&self) -> String {
                return self.name.to_string();
            }
        }
    }

    /// `FlamboyantModels` (org/betterx/betterend/integration/FlamboyantModels.java)
    pub mod flamboyant_models {
        use super::*;
        // PETAL_MODEL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        #[derive(Clone, Debug)]
        pub struct FlamboyantModels {
        }
        impl FlamboyantModels {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `FlamboyantModels.ClientModel` (org/betterx/betterend/integration/FlamboyantModels.java)
    pub mod flamboyant_models_client_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModel {
        }
        impl ClientModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FlamboyantModels.ClientModel.build(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `Integrations` (org/betterx/betterend/integration/Integrations.java)
    pub mod integrations {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Integrations {
        }
        impl Integrations {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Integrations.init(0): not translated, it uses BetterEnd.PATCHOULI, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
    }

    /// `AlloyingFuelCategory` (org/betterx/betterend/integration/jei/AlloyingFuelCategory.java)
    pub mod alloying_fuel_category {
        use super::*;
        // The class itself isn't translated: it implements `IRecipeCategory` (mezz.jei.api.recipe.category). Its static parts are below.
        // TYPE: not translated, it uses `IRecipeType` (mezz.jei.api.recipe.types).
        pub const TEXT_X: i32 = 20;
        pub const HEIGHT: i32 = 34;
        // AlloyingFuelCategory.AlloyingFuelCategory(1): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // AlloyingFuelCategory.burnTimeText(1): not translated, it uses Minecraft's `Component`.
        // AlloyingFuelCategory.getRecipeType(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // AlloyingFuelCategory.getTitle(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // AlloyingFuelCategory.getWidth(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // AlloyingFuelCategory.getHeight(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // AlloyingFuelCategory.getIcon(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // AlloyingFuelCategory.setRecipe(3): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // AlloyingFuelCategory.createRecipeExtras(3): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
    }

    /// `FuelEntry` (org/betterx/betterend/integration/jei/FuelEntry.java)
    pub mod fuel_entry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Item`. Its static parts are below.
        // FuelEntry.FuelEntry(2): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // FuelEntry.item(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // FuelEntry.burnTimeTicks(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
    }

    /// `InfusionCategory` (org/betterx/betterend/integration/jei/InfusionCategory.java)
    pub mod infusion_category {
        use super::*;
        // The class itself isn't translated: it implements `IRecipeCategory` (mezz.jei.api.recipe.category). Its static parts are below.
        // TYPE: not translated, it uses `IRecipeHolderType` (mezz.jei.api.recipe.types).
        // BACKGROUND: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // ORIENTATIONS: a game value, so it's in game_side.rs.
        pub const LEFT: i32 = 30;
        pub const TOP: i32 = 24;
        pub const BOX_SIZE: i32 = 64;
        pub const RADIUS: i32 = 42;
        pub const HALF_SLOT: i32 = 9;
        pub const CENTER_X: i32 = 62;
        pub const CENTER_Y: i32 = 56;
        pub const RIGHT: i32 = 94;
        pub const RING_ITEM_X_OFFSET: i32 = 2;
        pub const OUTPUT_X: i32 = 158;
        pub const WIDTH: i32 = 178;
        pub static HEIGHT: LazyLock<i32> = LazyLock::new(|| (92i32).max(123));
        // InfusionCategory.InfusionCategory(1): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // InfusionCategory.getRecipeType(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // InfusionCategory.getTitle(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // InfusionCategory.getWidth(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // InfusionCategory.getHeight(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // InfusionCategory.getIcon(0): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        pub fn catalyst_x(index: i32) -> i32 {
            return (64 + ((j_round_d(42.0f64 * ((((index as f64) * std::f64::consts::PI) / 4.0f64).sin()))) as i32)) - 9;
        }
        pub fn catalyst_y(index: i32) -> i32 {
            return (56 - ((j_round_d(42.0f64 * ((((index as f64) * std::f64::consts::PI) / 4.0f64).cos()))) as i32)) - 9;
        }
        // InfusionCategory.setRecipe(3): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // InfusionCategory.draw(5): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
        // InfusionCategory.createRecipeExtras(3): not translated, its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)).
    }

    /// `JeiPlugin` (org/betterx/betterend/integration/jei/JeiPlugin.java)
    pub mod jei_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IModPlugin` (mezz.jei.api). Its static parts are below.
        // UID: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // JeiPlugin.getPluginUid(0): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JeiPlugin.registerCategories(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JeiPlugin.registerRecipes(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JeiPlugin.registerRecipeCatalysts(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
    }

    /// `BETargetChecker` (org/betterx/betterend/interfaces/BETargetChecker.java)
    pub mod be_target_checker {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BETargetChecker.be_isTarget(0): not translated, it has no body.
        // BETargetChecker.be_setTarget(1): not translated, it has no body.
    }

    /// `BetterEndElytra` (org/betterx/betterend/interfaces/BetterEndElytra.java)
    pub mod better_end_elytra {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BetterEndElytra.getModelTexture(0): not translated, it has no body.
    }

    /// `ISlime` (org/betterx/betterend/interfaces/ISlime.java)
    pub mod i_slime {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ISlime.be_setSlimeSize(2): not translated, it has no body.
        // ISlime.entityRemove(1): not translated, it has no body.
    }

    /// `MobEffectApplier` (org/betterx/betterend/interfaces/MobEffectApplier.java)
    pub mod mob_effect_applier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MobEffectApplier.applyEffect(1): not translated, it has no body.
    }

    /// `MultiModelItem` (org/betterx/betterend/interfaces/MultiModelItem.java)
    pub mod multi_model_item {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MultiModelItem.registerModelPredicate(0): not translated, it has no body.
        // MultiModelItem.register(0): not translated, it uses Minecraft's `Item`.
    }

    /// `ArmoredElytra` (org/betterx/betterend/item/ArmoredElytra.java)
    pub mod armored_elytra {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // ArmoredElytra.ArmoredElytra(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ArmoredElytra.getMovementFactor(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ArmoredElytra.getModelTexture(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `CrystaliteArmor` (org/betterx/betterend/item/CrystaliteArmor.java)
    pub mod crystalite_armor {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // CHEST_DESC: a game value, so it's in game_side.rs.
        // ELYTRA_DESC: a game value, so it's in game_side.rs.
        // BOOTS_DESC: a game value, so it's in game_side.rs.
        // HELMET_DESC: a game value, so it's in game_side.rs.
        // LEGGINGS_DESC: a game value, so it's in game_side.rs.
        // SET_SLOTS: not translated, it uses Minecraft's `EquipmentSlot`.
        // CrystaliteArmor.CrystaliteArmor(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // CrystaliteArmor.hasFullSet(1): not translated, it uses Minecraft's `LivingEntity`.
        // CrystaliteArmor.applySetEffect(1): not translated, it uses Minecraft's `LivingEntity`.
    }

    /// `CrystaliteBoots` (org/betterx/betterend/item/CrystaliteBoots.java)
    pub mod crystalite_boots {
        use super::*;
        // The class itself isn't translated: it extends `CrystaliteArmor`. Its static parts are below.
        // CrystaliteBoots.CrystaliteBoots(1): not translated, its class isn't translated (it extends `CrystaliteArmor`).
        // CrystaliteBoots.applyEffect(1): not translated, its class isn't translated (it extends `CrystaliteArmor`).
        // CrystaliteBoots.appendHoverText(5): not translated, its class isn't translated (it extends `CrystaliteArmor`).
    }

    /// `CrystaliteChestplate` (org/betterx/betterend/item/CrystaliteChestplate.java)
    pub mod crystalite_chestplate {
        use super::*;
        // The class itself isn't translated: it extends `CrystaliteArmor`. Its static parts are below.
        // CrystaliteChestplate.CrystaliteChestplate(1): not translated, its class isn't translated (it extends `CrystaliteArmor`).
        // CrystaliteChestplate.applyEffect(1): not translated, its class isn't translated (it extends `CrystaliteArmor`).
        // CrystaliteChestplate.appendHoverText(5): not translated, its class isn't translated (it extends `CrystaliteArmor`).
    }

    /// `CrystaliteElytra` (org/betterx/betterend/item/CrystaliteElytra.java)
    pub mod crystalite_elytra {
        use super::*;
        // The class itself isn't translated: it extends `ArmoredElytra`. Its static parts are below.
        // CrystaliteElytra.CrystaliteElytra(3): not translated, its class isn't translated (it extends `ArmoredElytra`).
        // CrystaliteElytra.applyEffect(1): not translated, its class isn't translated (it extends `ArmoredElytra`).
        // CrystaliteElytra.appendHoverText(5): not translated, its class isn't translated (it extends `ArmoredElytra`).
    }

    /// `CrystaliteHelmet` (org/betterx/betterend/item/CrystaliteHelmet.java)
    pub mod crystalite_helmet {
        use super::*;
        // The class itself isn't translated: it extends `CrystaliteArmor`. Its static parts are below.
        // CrystaliteHelmet.CrystaliteHelmet(1): not translated, its class isn't translated (it extends `CrystaliteArmor`).
        // CrystaliteHelmet.appendHoverText(5): not translated, its class isn't translated (it extends `CrystaliteArmor`).
    }

    /// `CrystaliteLeggings` (org/betterx/betterend/item/CrystaliteLeggings.java)
    pub mod crystalite_leggings {
        use super::*;
        // The class itself isn't translated: it extends `CrystaliteArmor`. Its static parts are below.
        // CrystaliteLeggings.CrystaliteLeggings(1): not translated, its class isn't translated (it extends `CrystaliteArmor`).
        // CrystaliteLeggings.appendHoverText(5): not translated, its class isn't translated (it extends `CrystaliteArmor`).
    }

    /// `EndArmorItem` (org/betterx/betterend/item/EndArmorItem.java)
    pub mod end_armor_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // BASE_BLINDNESS_RESISTANCE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // BASE_KNOCKBACK_RESISTANCE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // MAX_HEALTH_BOOST: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // TOUGHNESS_BOOST: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // ARMOR_BOOST: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EndArmorItem.armorBoostId(1): not translated, it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment).
        // EndArmorItem.toughnessBoostId(1): not translated, it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment).
        // EndArmorItem.knockbackBoostId(1): not translated, it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment).
        // EndArmorItem.createDefaultEndArmorSettings(2): not translated, it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment).
        // EndArmorItem.createDefaultEndArmorSettings(3): not translated, it uses `ArmorSlot` (de.ambertation.wover.complex.api.equipment).
        // EndArmorItem.EndArmorItem(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `EndAttribute` (org/betterx/betterend/item/EndAttribute.java)
    pub mod end_attribute {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Attribute`. Its static parts are below.
        // EndAttribute.EndAttribute(2): not translated, its class isn't translated (it extends Minecraft's `Attribute`).
    }

    /// `EndBucketItem` (org/betterx/betterend/item/EndBucketItem.java)
    pub mod end_bucket_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobBucketItem`. Its static parts are below.
        // EndBucketItem.EndBucketItem(2): not translated, its class isn't translated (it extends Minecraft's `MobBucketItem`).
    }

    /// `GuideBookItem` (org/betterx/betterend/item/GuideBookItem.java)
    pub mod guide_book_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // GUIDE_BOOK: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        pub fn register() {
        }
        // GuideBookItem.GuideBookItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // GuideBookItem.use(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // GuideBookItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        pub fn ensure_statically_loaded() {
        }
    }

    /// `AeterniumSet` (org/betterx/betterend/item/material/AeterniumSet.java)
    pub mod aeternium_set {
        use super::*;
        // The class itself isn't translated: it extends `ToolsWithHeadsSet`. Its static parts are below.
        // AeterniumSet.AeterniumSet(0): not translated, its class isn't translated (it extends `ToolsWithHeadsSet`).
        // AeterniumSet.commonToolProperties(1): not translated, its class isn't translated (it extends `ToolsWithHeadsSet`).
        // AeterniumSet.hasAutoHeadRecipes(0): not translated, its class isn't translated (it extends `ToolsWithHeadsSet`).
        // AeterniumSet.hasAutoSwordHandleRecipe(0): not translated, its class isn't translated (it extends `ToolsWithHeadsSet`).
        // AeterniumSet.pickaxe(0): not translated, its class isn't translated (it extends `ToolsWithHeadsSet`).
        // AeterniumSet.axe(0): not translated, its class isn't translated (it extends `ToolsWithHeadsSet`).
    }

    /// `EndArmorMaterial` (org/betterx/betterend/item/material/EndArmorMaterial.java)
    pub mod end_armor_material {
        use super::*;
        // THALLASIUM: not translated, it uses Minecraft's `Holder`.
        // TERMINITE: not translated, it uses Minecraft's `Holder`.
        // AETERNIUM: not translated, it uses Minecraft's `Holder`.
        // CRYSTALITE: not translated, it uses Minecraft's `Holder`.
        // AETERNIUM_ELYTRA: not translated, it uses Minecraft's `Holder`.
        // CRYSTALITE_ELYTRA: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct EndArmorMaterial {
        }
        impl EndArmorMaterial {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndArmorTier` (org/betterx/betterend/item/material/EndArmorTier.java)
    pub mod end_armor_tier {
        use super::*;
        // THALLASIUM: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        // TERMINITE: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        // CRYSTALITE: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        // AETERNIUM: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        // AETERNIUM_ELYTRA: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        // CRYSTALITE_ELYTRA: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        #[derive(Clone, Debug)]
        pub struct EndArmorTier {
        }
        impl EndArmorTier {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndToolMaterial` (org/betterx/betterend/item/material/EndToolMaterial.java)
    pub mod end_tool_material {
        use super::*;
        // THALLASIUM: not translated, it uses Minecraft's `ToolMaterial`.
        // TERMINITE: not translated, it uses Minecraft's `ToolMaterial`.
        // AETERNIUM: not translated, it uses Minecraft's `ToolMaterial`.
        #[derive(Clone, Debug)]
        pub struct EndToolMaterial {
        }
        impl EndToolMaterial {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndToolTier` (org/betterx/betterend/item/material/EndToolTier.java)
    pub mod end_tool_tier {
        use super::*;
        // THALLASIUM: not translated, it uses `ToolTier` (de.ambertation.wover.complex.api.equipment).
        // TERMINITE: not translated, it uses `ToolTier` (de.ambertation.wover.complex.api.equipment).
        // AETERNIUM: not translated, it uses `ToolTier` (de.ambertation.wover.complex.api.equipment).
        #[derive(Clone, Debug)]
        pub struct EndToolTier {
        }
        impl EndToolTier {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ToolsWithHeadsSet` (org/betterx/betterend/item/material/ToolsWithHeadsSet.java)
    pub mod tools_with_heads_set {
        use super::*;
        // The class itself isn't translated: it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment). Its static parts are below.
        // ToolsWithHeadsSet.ToolsWithHeadsSet(9): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // ToolsWithHeadsSet.buildArmor(0): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // ToolsWithHeadsSet.registerRecipes(1): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // ToolsWithHeadsSet.hasAutoHeadRecipes(0): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // ToolsWithHeadsSet.hasAutoSwordHandleRecipe(0): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
    }

    /// `CrystaliteArmorParts` (org/betterx/betterend/item/model/CrystaliteArmorParts.java)
    pub mod crystalite_armor_parts {
        use super::*;
        // The class itself isn't translated: it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`). Its static parts are below.
        // HELMET: not translated, it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)).
        // CHESTPLATE: not translated, it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)).
        // CHESTPLATE_SLIM: not translated, it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)).
        // LEGGINGS: not translated, it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)).
        // BOOTS: not translated, it uses CrystaliteArmorParts, which isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)).
        // CrystaliteArmorParts.CrystaliteArmorParts(1): not translated, its class isn't translated (it uses Attachment, which isn't translated (it uses Minecraft's `HumanoidModel`)).
        // CrystaliteArmorParts.forSlot(2): not translated, it uses Minecraft's `HumanoidRenderState`.
        // CrystaliteArmorParts.forEachVisible(3): not translated, it uses Minecraft's `HumanoidModel`.
        // CrystaliteArmorParts.attach(2): not translated, it uses Minecraft's `HumanoidModel`.
        // CrystaliteArmorParts.detach(1): not translated, it uses Minecraft's `ModelPart`.
        // CrystaliteArmorParts.helmet(1): not translated, it uses CrystaliteHelmetModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteArmorParts.chestplate(1): not translated, it uses CrystaliteChestplateModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteArmorParts.leggings(1): not translated, it uses CrystaliteLeggingsModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteArmorParts.boots(1): not translated, it uses CrystaliteBootsModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
    }

    /// `CrystaliteArmorParts.Attachment` (org/betterx/betterend/item/model/CrystaliteArmorParts.java)
    pub mod crystalite_armor_parts_attachment {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `HumanoidModel`. Its static parts are below.
        // CrystaliteArmorParts.Attachment.Attachment(3): not translated, its class isn't translated (it uses Minecraft's `HumanoidModel`).
        // CrystaliteArmorParts.Attachment.bone(0): not translated, its class isn't translated (it uses Minecraft's `HumanoidModel`).
        // CrystaliteArmorParts.Attachment.insideHat(0): not translated, its class isn't translated (it uses Minecraft's `HumanoidModel`).
        // CrystaliteArmorParts.Attachment.crystal(0): not translated, its class isn't translated (it uses Minecraft's `HumanoidModel`).
    }

    /// `CrystaliteArmorRenderer` (org/betterx/betterend/item/model/CrystaliteArmorRenderer.java)
    pub mod crystalite_armor_renderer {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ArmorRenderer`. Its static parts are below.
        // FIRST_LAYER: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // SECOND_LAYER: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // INSTANCE: not translated, it uses CrystaliteArmorRenderer, which isn't translated (it implements Fabric's `ArmorRenderer`).
        // CrystaliteArmorRenderer.register(0): not translated, it uses CrystaliteArmorRenderer.INSTANCE, which isn't translated (it uses CrystaliteArmorRenderer, which isn't translated (it implements Fabric's `ArmorRenderer`)).
        // CrystaliteArmorRenderer.getTextureForSlot(1): not translated, its class isn't translated (it implements Fabric's `ArmorRenderer`).
        // CrystaliteArmorRenderer.render(7): not translated, its class isn't translated (it implements Fabric's `ArmorRenderer`).
        // CrystaliteArmorRenderer.submitLayer(6): not translated, it uses Minecraft's `SubmitNodeCollector`.
    }

    /// `CrystaliteBootsModel` (org/betterx/betterend/item/model/CrystaliteBootsModel.java)
    pub mod crystalite_boots_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HumanoidModel`. Its static parts are below.
        // CrystaliteBootsModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // CrystaliteBootsModel.createModel(1): not translated, it uses CrystaliteBootsModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteBootsModel.CrystaliteBootsModel(1): not translated, its class isn't translated (it extends Minecraft's `HumanoidModel`).
    }

    /// `CrystaliteChestplateModel` (org/betterx/betterend/item/model/CrystaliteChestplateModel.java)
    pub mod crystalite_chestplate_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HumanoidModel`. Its static parts are below.
        // CrystaliteChestplateModel.getRegularTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // CrystaliteChestplateModel.getThinTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // CrystaliteChestplateModel.getTexturedModelData(2): not translated, it uses Minecraft's `LayerDefinition`.
        // CrystaliteChestplateModel.createRegularModel(1): not translated, it uses CrystaliteChestplateModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteChestplateModel.createThinModel(1): not translated, it uses CrystaliteChestplateModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteChestplateModel.CrystaliteChestplateModel(2): not translated, its class isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteChestplateModel.translateToHand(3): not translated, its class isn't translated (it extends Minecraft's `HumanoidModel`).
    }

    /// `CrystaliteHelmetModel` (org/betterx/betterend/item/model/CrystaliteHelmetModel.java)
    pub mod crystalite_helmet_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HumanoidModel`. Its static parts are below.
        // CrystaliteHelmetModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // CrystaliteHelmetModel.createModel(1): not translated, it uses CrystaliteHelmetModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteHelmetModel.CrystaliteHelmetModel(1): not translated, its class isn't translated (it extends Minecraft's `HumanoidModel`).
    }

    /// `CrystaliteLeggingsModel` (org/betterx/betterend/item/model/CrystaliteLeggingsModel.java)
    pub mod crystalite_leggings_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HumanoidModel`. Its static parts are below.
        // CrystaliteLeggingsModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // CrystaliteLeggingsModel.createModel(1): not translated, it uses CrystaliteLeggingsModel, which isn't translated (it extends Minecraft's `HumanoidModel`).
        // CrystaliteLeggingsModel.CrystaliteLeggingsModel(1): not translated, its class isn't translated (it extends Minecraft's `HumanoidModel`).
    }

    /// `RitualUpdate` (org/betterx/betterend/network/RitualUpdate.java)
    pub mod ritual_update {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        pub const ACTIVE_FLAG: i8 = 1i8;
        pub const WILL_ACTIVATE_FLAG: i8 = 2i8;
        // AXIS_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // KEY: not translated, it uses `ClientBoundMessage` (de.ambertation.wunderlib.network).
        // RitualUpdate.RitualUpdate(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RitualUpdate.of(1): not translated, it uses RitualUpdate, which isn't translated (it uses Minecraft's `BlockPos`).
        // RitualUpdate.isActive(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RitualUpdate.willActivate(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RitualUpdate.send(2): not translated, it uses Minecraft's `ServerLevel`.
        // RitualUpdate.center(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RitualUpdate.axis(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RitualUpdate.flags(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `OpenSimplexNoise` (org/betterx/betterend/noise/OpenSimplexNoise.java)
    pub mod open_simplex_noise {
        use super::*;
        pub const STRETCH_CONSTANT_2D: f64 = -0.211324865405187f64;
        pub const SQUISH_CONSTANT_2D: f64 = 0.366025403784439f64;
        pub const STRETCH_CONSTANT_3D: f64 = -0.16666666666666666f64;
        pub const SQUISH_CONSTANT_3D: f64 = 0.3333333333333333f64;
        pub const STRETCH_CONSTANT_4D: f64 = -0.138196601125011f64;
        pub const SQUISH_CONSTANT_4D: f64 = 0.309016994374947f64;
        pub const NORM_CONSTANT_2D: f64 = 47.0f64;
        pub const NORM_CONSTANT_3D: f64 = 103.0f64;
        pub const NORM_CONSTANT_4D: f64 = 30.0f64;
        pub const DEFAULT_SEED: i64 = 0i64;
        pub static GRADIENTS2_D: LazyLock<Vec<i8>> = LazyLock::new(|| vec![5i8, 2i8, 2i8, 5i8, -5i8, 2i8, -2i8, 5i8, 5i8, -2i8, 2i8, -5i8, -5i8, -2i8, -2i8, -5i8]);
        pub static GRADIENTS3_D: LazyLock<Vec<i8>> = LazyLock::new(|| vec![-11i8, 4i8, 4i8, -4i8, 11i8, 4i8, -4i8, 4i8, 11i8, 11i8, 4i8, 4i8, 4i8, 11i8, 4i8, 4i8, 4i8, 11i8, -11i8, -4i8, 4i8, -4i8, -11i8, 4i8, -4i8, -4i8, 11i8, 11i8, -4i8, 4i8, 4i8, -11i8, 4i8, 4i8, -4i8, 11i8, -11i8, 4i8, -4i8, -4i8, 11i8, -4i8, -4i8, 4i8, -11i8, 11i8, 4i8, -4i8, 4i8, 11i8, -4i8, 4i8, 4i8, -11i8, -11i8, -4i8, -4i8, -4i8, -11i8, -4i8, -4i8, -4i8, -11i8, 11i8, -4i8, -4i8, 4i8, -11i8, -4i8, 4i8, -4i8, -11i8]);
        pub static GRADIENTS4_D: LazyLock<Vec<i8>> = LazyLock::new(|| vec![3i8, 1i8, 1i8, 1i8, 1i8, 3i8, 1i8, 1i8, 1i8, 1i8, 3i8, 1i8, 1i8, 1i8, 1i8, 3i8, -3i8, 1i8, 1i8, 1i8, -1i8, 3i8, 1i8, 1i8, -1i8, 1i8, 3i8, 1i8, -1i8, 1i8, 1i8, 3i8, 3i8, -1i8, 1i8, 1i8, 1i8, -3i8, 1i8, 1i8, 1i8, -1i8, 3i8, 1i8, 1i8, -1i8, 1i8, 3i8, -3i8, -1i8, 1i8, 1i8, -1i8, -3i8, 1i8, 1i8, -1i8, -1i8, 3i8, 1i8, -1i8, -1i8, 1i8, 3i8, 3i8, 1i8, -1i8, 1i8, 1i8, 3i8, -1i8, 1i8, 1i8, 1i8, -3i8, 1i8, 1i8, 1i8, -1i8, 3i8, -3i8, 1i8, -1i8, 1i8, -1i8, 3i8, -1i8, 1i8, -1i8, 1i8, -3i8, 1i8, -1i8, 1i8, -1i8, 3i8, 3i8, -1i8, -1i8, 1i8, 1i8, -3i8, -1i8, 1i8, 1i8, -1i8, -3i8, 1i8, 1i8, -1i8, -1i8, 3i8, -3i8, -1i8, -1i8, 1i8, -1i8, -3i8, -1i8, 1i8, -1i8, -1i8, -3i8, 1i8, -1i8, -1i8, -1i8, 3i8, 3i8, 1i8, 1i8, -1i8, 1i8, 3i8, 1i8, -1i8, 1i8, 1i8, 3i8, -1i8, 1i8, 1i8, 1i8, -3i8, -3i8, 1i8, 1i8, -1i8, -1i8, 3i8, 1i8, -1i8, -1i8, 1i8, 3i8, -1i8, -1i8, 1i8, 1i8, -3i8, 3i8, -1i8, 1i8, -1i8, 1i8, -3i8, 1i8, -1i8, 1i8, -1i8, 3i8, -1i8, 1i8, -1i8, 1i8, -3i8, -3i8, -1i8, 1i8, -1i8, -1i8, -3i8, 1i8, -1i8, -1i8, -1i8, 3i8, -1i8, -1i8, -1i8, 1i8, -3i8, 3i8, 1i8, -1i8, -1i8, 1i8, 3i8, -1i8, -1i8, 1i8, 1i8, -3i8, -1i8, 1i8, 1i8, -1i8, -3i8, -3i8, 1i8, -1i8, -1i8, -1i8, 3i8, -1i8, -1i8, -1i8, 1i8, -3i8, -1i8, -1i8, 1i8, -1i8, -3i8, 3i8, -1i8, -1i8, -1i8, 1i8, -3i8, -1i8, -1i8, 1i8, -1i8, -3i8, -1i8, 1i8, -1i8, -1i8, -3i8, -3i8, -1i8, -1i8, -1i8, -1i8, -3i8, -1i8, -1i8, -1i8, -1i8, -3i8, -1i8, -1i8, -1i8, -1i8, -3i8]);
        #[derive(Clone, Debug)]
        pub struct OpenSimplexNoise {
            pub perm: Option<Vec<i16>>,
            pub perm_grad_index3_d: Option<Vec<i16>>,
        }
        impl OpenSimplexNoise {
            // OpenSimplexNoise.OpenSimplexNoise(0): not translated, its constructors are overloaded with the same number of parameters.
            // OpenSimplexNoise.OpenSimplexNoise(1): not translated, its constructors are overloaded with the same number of parameters.
            // OpenSimplexNoise.OpenSimplexNoise(1): not translated, its constructors are overloaded with the same number of parameters.
            pub fn eval_2(&self, x: f64, y: f64) -> f64 {
                let stretch_offset: f64 = (x + y) * (-0.211324865405187f64);
                let xs: f64 = x + stretch_offset;
                let ys: f64 = y + stretch_offset;
                let mut xsb: i32 = open_simplex_noise::fast_floor(xs);
                let mut ysb: i32 = open_simplex_noise::fast_floor(ys);
                let squish_offset: f64 = ((xsb + ysb) as f64) * 0.366025403784439f64;
                let xb: f64 = (xsb as f64) + squish_offset;
                let yb: f64 = (ysb as f64) + squish_offset;
                let xins: f64 = xs - (xsb as f64);
                let yins: f64 = ys - (ysb as f64);
                let in_sum: f64 = xins + yins;
                let mut dx0: f64 = x - xb;
                let mut dy0: f64 = y - yb;
                let mut value: f64 = 0.0f64;
                let dx1: f64 = (dx0 - 1.0f64) - 0.366025403784439f64;
                let dy1: f64 = (dy0 - 0.0f64) - 0.366025403784439f64;
                let mut attn1: f64 = (2.0f64 - (dx1 * dx1)) - (dy1 * dy1);
                if attn1 > 0.0f64 {
                    attn1 *= attn1;
                    value += (attn1 * attn1) * (self.extrapolate_4(xsb + 1, ysb + 0, dx1, dy1));
                }
                let dx2: f64 = (dx0 - 0.0f64) - 0.366025403784439f64;
                let dy2: f64 = (dy0 - 1.0f64) - 0.366025403784439f64;
                let mut attn2: f64 = (2.0f64 - (dx2 * dx2)) - (dy2 * dy2);
                if attn2 > 0.0f64 {
                    attn2 *= attn2;
                    value += (attn2 * attn2) * (self.extrapolate_4(xsb + 0, ysb + 1, dx2, dy2));
                }
                let mut dx_ext: f64;
                let mut dy_ext: f64;
                let mut xsv_ext: i32;
                let mut ysv_ext: i32;
                if in_sum <= 1.0f64 {
                    let zins: f64 = 1.0f64 - in_sum;
                    if (!(zins > xins)) && (!(zins > yins)) {
                        xsv_ext = xsb + 1;
                        ysv_ext = ysb + 1;
                        dx_ext = (dx0 - 1.0f64) - 0.732050807568878f64;
                        dy_ext = (dy0 - 1.0f64) - 0.732050807568878f64;
                    } else if xins > yins {
                        xsv_ext = xsb + 1;
                        ysv_ext = ysb - 1;
                        dx_ext = dx0 - 1.0f64;
                        dy_ext = dy0 + 1.0f64;
                    } else {
                        xsv_ext = xsb - 1;
                        ysv_ext = ysb + 1;
                        dx_ext = dx0 + 1.0f64;
                        dy_ext = dy0 - 1.0f64;
                    }
                } else {
                    let zins: f64 = 2.0f64 - in_sum;
                    if (!(zins < xins)) && (!(zins < yins)) {
                        dx_ext = dx0;
                        dy_ext = dy0;
                        xsv_ext = xsb;
                        ysv_ext = ysb;
                    } else if xins > yins {
                        xsv_ext = xsb + 2;
                        ysv_ext = ysb + 0;
                        dx_ext = (dx0 - 2.0f64) - 0.732050807568878f64;
                        dy_ext = (dy0 + 0.0f64) - 0.732050807568878f64;
                    } else {
                        xsv_ext = xsb + 0;
                        ysv_ext = ysb + 2;
                        dx_ext = (dx0 + 0.0f64) - 0.732050807568878f64;
                        dy_ext = (dy0 - 2.0f64) - 0.732050807568878f64;
                    }
                    xsb += 1;
                    ysb += 1;
                    dx0 = (dx0 - 1.0f64) - 0.732050807568878f64;
                    dy0 = (dy0 - 1.0f64) - 0.732050807568878f64;
                }
                let mut attn0: f64 = (2.0f64 - (dx0 * dx0)) - (dy0 * dy0);
                if attn0 > 0.0f64 {
                    attn0 *= attn0;
                    value += (attn0 * attn0) * (self.extrapolate_4(xsb, ysb, dx0, dy0));
                }
                let mut attn_ext: f64 = (2.0f64 - (dx_ext * dx_ext)) - (dy_ext * dy_ext);
                if attn_ext > 0.0f64 {
                    attn_ext *= attn_ext;
                    value += (attn_ext * attn_ext) * (self.extrapolate_4(xsv_ext, ysv_ext, dx_ext, dy_ext));
                }
                return value / 47.0f64;
            }
            pub fn eval_3(&self, x: f64, y: f64, z: f64) -> f64 {
                let stretch_offset: f64 = ((x + y) + z) * (-0.16666666666666666f64);
                let xs: f64 = x + stretch_offset;
                let ys: f64 = y + stretch_offset;
                let zs: f64 = z + stretch_offset;
                let xsb: i32 = open_simplex_noise::fast_floor(xs);
                let ysb: i32 = open_simplex_noise::fast_floor(ys);
                let zsb: i32 = open_simplex_noise::fast_floor(zs);
                let squish_offset: f64 = (((xsb + ysb) + zsb) as f64) * 0.3333333333333333f64;
                let xb: f64 = (xsb as f64) + squish_offset;
                let yb: f64 = (ysb as f64) + squish_offset;
                let zb: f64 = (zsb as f64) + squish_offset;
                let xins: f64 = xs - (xsb as f64);
                let yins: f64 = ys - (ysb as f64);
                let zins: f64 = zs - (zsb as f64);
                let in_sum: f64 = (xins + yins) + zins;
                let mut dx0: f64 = x - xb;
                let mut dy0: f64 = y - yb;
                let mut dz0: f64 = z - zb;
                let mut value: f64 = 0.0f64;
                let mut dx_ext0: f64;
                let mut dy_ext0: f64;
                let mut dz_ext0: f64;
                let mut dx_ext1: f64;
                let mut dy_ext1: f64;
                let mut dz_ext1: f64;
                let mut xsv_ext0: i32;
                let mut ysv_ext0: i32;
                let mut zsv_ext0: i32;
                let mut xsv_ext1: i32;
                let mut ysv_ext1: i32;
                let mut zsv_ext1: i32;
                if in_sum <= 1.0f64 {
                    let mut a_point: i8 = 1i8;
                    let mut a_score: f64 = xins;
                    let mut b_point: i8 = 2i8;
                    let mut b_score: f64 = yins;
                    if (a_score >= b_score) && (zins > b_score) {
                        b_score = zins;
                        b_point = 4i8;
                    } else if (a_score < b_score) && (zins > a_score) {
                        a_score = zins;
                        a_point = 4i8;
                    }
                    let wins: f64 = 1.0f64 - in_sum;
                    if (!(wins > a_score)) && (!(wins > b_score)) {
                        let c: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                        if ((c as i32) & 1) == 0 {
                            xsv_ext0 = xsb;
                            xsv_ext1 = xsb - 1;
                            dx_ext0 = dx0 - 0.6666666666666666f64;
                            dx_ext1 = (dx0 + 1.0f64) - 0.3333333333333333f64;
                        } else {
                            let chain_0 = xsb + 1;
                            xsv_ext1 = chain_0;
                            xsv_ext0 = chain_0;
                            dx_ext0 = (dx0 - 1.0f64) - 0.6666666666666666f64;
                            dx_ext1 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                        }
                        if ((c as i32) & 2) == 0 {
                            ysv_ext0 = ysb;
                            ysv_ext1 = ysb - 1;
                            dy_ext0 = dy0 - 0.6666666666666666f64;
                            dy_ext1 = (dy0 + 1.0f64) - 0.3333333333333333f64;
                        } else {
                            let chain_1 = ysb + 1;
                            ysv_ext1 = chain_1;
                            ysv_ext0 = chain_1;
                            dy_ext0 = (dy0 - 1.0f64) - 0.6666666666666666f64;
                            dy_ext1 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                        }
                        if ((c as i32) & 4) == 0 {
                            zsv_ext0 = zsb;
                            zsv_ext1 = zsb - 1;
                            dz_ext0 = dz0 - 0.6666666666666666f64;
                            dz_ext1 = (dz0 + 1.0f64) - 0.3333333333333333f64;
                        } else {
                            let chain_2 = zsb + 1;
                            zsv_ext1 = chain_2;
                            zsv_ext0 = chain_2;
                            dz_ext0 = (dz0 - 1.0f64) - 0.6666666666666666f64;
                            dz_ext1 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                        }
                    } else {
                        let c: i8 = ((if b_score > a_score { (b_point as i32) } else { (a_point as i32) }) as i8);
                        if ((c as i32) & 1) == 0 {
                            xsv_ext0 = xsb - 1;
                            xsv_ext1 = xsb;
                            dx_ext0 = dx0 + 1.0f64;
                            dx_ext1 = dx0;
                        } else {
                            let chain_3 = xsb + 1;
                            xsv_ext1 = chain_3;
                            xsv_ext0 = chain_3;
                            let chain_4 = dx0 - 1.0f64;
                            dx_ext1 = chain_4;
                            dx_ext0 = chain_4;
                        }
                        if ((c as i32) & 2) == 0 {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext1 = dy0;
                            dy_ext0 = dy0;
                            if ((c as i32) & 1) == 0 {
                                ysv_ext1 -= 1;
                                dy_ext1 += 1.0f64;
                            } else {
                                ysv_ext0 -= 1;
                                dy_ext0 += 1.0f64;
                            }
                        } else {
                            let chain_5 = ysb + 1;
                            ysv_ext1 = chain_5;
                            ysv_ext0 = chain_5;
                            let chain_6 = dy0 - 1.0f64;
                            dy_ext1 = chain_6;
                            dy_ext0 = chain_6;
                        }
                        if ((c as i32) & 4) == 0 {
                            zsv_ext0 = zsb;
                            zsv_ext1 = zsb - 1;
                            dz_ext0 = dz0;
                            dz_ext1 = dz0 + 1.0f64;
                        } else {
                            let chain_7 = zsb + 1;
                            zsv_ext1 = chain_7;
                            zsv_ext0 = chain_7;
                            let chain_8 = dz0 - 1.0f64;
                            dz_ext1 = chain_8;
                            dz_ext0 = chain_8;
                        }
                    }
                    let mut attn0: f64 = ((2.0f64 - (dx0 * dx0)) - (dy0 * dy0)) - (dz0 * dz0);
                    if attn0 > 0.0f64 {
                        attn0 *= attn0;
                        value += (attn0 * attn0) * (self.extrapolate_6(xsb + 0, ysb + 0, zsb + 0, dx0, dy0, dz0));
                    }
                    let dx1: f64 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                    let dy1: f64 = (dy0 - 0.0f64) - 0.3333333333333333f64;
                    let dz1: f64 = (dz0 - 0.0f64) - 0.3333333333333333f64;
                    let mut attn1: f64 = ((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_6(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1));
                    }
                    let dx2: f64 = (dx0 - 0.0f64) - 0.3333333333333333f64;
                    let dy2: f64 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                    let dz2: f64 = dz1;
                    let mut attn2: f64 = ((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_6(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2));
                    }
                    let dx3: f64 = dx2;
                    let dy3: f64 = dy1;
                    let dz3: f64 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                    let mut attn3: f64 = ((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_6(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3));
                    }
                } else if in_sum >= 2.0f64 {
                    let mut a_point: i8 = 6i8;
                    let mut a_score: f64 = xins;
                    let mut b_point: i8 = 5i8;
                    let mut b_score: f64 = yins;
                    if (a_score <= b_score) && (zins < b_score) {
                        b_score = zins;
                        b_point = 3i8;
                    } else if (a_score > b_score) && (zins < a_score) {
                        a_score = zins;
                        a_point = 3i8;
                    }
                    let wins: f64 = 3.0f64 - in_sum;
                    if (!(wins < a_score)) && (!(wins < b_score)) {
                        let c: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                        if ((c as i32) & 1) != 0 {
                            xsv_ext0 = xsb + 1;
                            xsv_ext1 = xsb + 2;
                            dx_ext0 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                            dx_ext1 = (dx0 - 2.0f64) - 0.6666666666666666f64;
                        } else {
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            dx_ext0 = dx0 - 0.3333333333333333f64;
                            dx_ext1 = dx0 - 0.6666666666666666f64;
                        }
                        if ((c as i32) & 2) != 0 {
                            ysv_ext0 = ysb + 1;
                            ysv_ext1 = ysb + 2;
                            dy_ext0 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                            dy_ext1 = (dy0 - 2.0f64) - 0.6666666666666666f64;
                        } else {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext0 = dy0 - 0.3333333333333333f64;
                            dy_ext1 = dy0 - 0.6666666666666666f64;
                        }
                        if ((c as i32) & 4) != 0 {
                            zsv_ext0 = zsb + 1;
                            zsv_ext1 = zsb + 2;
                            dz_ext0 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                            dz_ext1 = (dz0 - 2.0f64) - 0.6666666666666666f64;
                        } else {
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            dz_ext0 = dz0 - 0.3333333333333333f64;
                            dz_ext1 = dz0 - 0.6666666666666666f64;
                        }
                    } else {
                        let c: i8 = ((if b_score < a_score { (b_point as i32) } else { (a_point as i32) }) as i8);
                        if ((c as i32) & 1) != 0 {
                            xsv_ext0 = xsb + 2;
                            xsv_ext1 = xsb + 1;
                            dx_ext0 = (dx0 - 2.0f64) - 1.0f64;
                            dx_ext1 = (dx0 - 1.0f64) - 1.0f64;
                        } else {
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            let chain_9 = dx0 - 1.0f64;
                            dx_ext1 = chain_9;
                            dx_ext0 = chain_9;
                        }
                        if ((c as i32) & 2) != 0 {
                            let chain_10 = ysb + 1;
                            ysv_ext1 = chain_10;
                            ysv_ext0 = chain_10;
                            let chain_11 = (dy0 - 1.0f64) - 1.0f64;
                            dy_ext1 = chain_11;
                            dy_ext0 = chain_11;
                            if ((c as i32) & 1) != 0 {
                                ysv_ext1 += 1;
                                dy_ext1 -= 1.0f64;
                            } else {
                                ysv_ext0 += 1;
                                dy_ext0 -= 1.0f64;
                            }
                        } else {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            let chain_12 = dy0 - 1.0f64;
                            dy_ext1 = chain_12;
                            dy_ext0 = chain_12;
                        }
                        if ((c as i32) & 4) != 0 {
                            zsv_ext0 = zsb + 1;
                            zsv_ext1 = zsb + 2;
                            dz_ext0 = (dz0 - 1.0f64) - 1.0f64;
                            dz_ext1 = (dz0 - 2.0f64) - 1.0f64;
                        } else {
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            let chain_13 = dz0 - 1.0f64;
                            dz_ext1 = chain_13;
                            dz_ext0 = chain_13;
                        }
                    }
                    let dx3: f64 = (dx0 - 1.0f64) - 0.6666666666666666f64;
                    let dy3: f64 = (dy0 - 1.0f64) - 0.6666666666666666f64;
                    let dz3: f64 = (dz0 - 0.0f64) - 0.6666666666666666f64;
                    let mut attn3: f64 = ((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_6(xsb + 1, ysb + 1, zsb + 0, dx3, dy3, dz3));
                    }
                    let dx2: f64 = dx3;
                    let dy2: f64 = (dy0 - 0.0f64) - 0.6666666666666666f64;
                    let dz2: f64 = (dz0 - 1.0f64) - 0.6666666666666666f64;
                    let mut attn2: f64 = ((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_6(xsb + 1, ysb + 0, zsb + 1, dx2, dy2, dz2));
                    }
                    let dx1: f64 = (dx0 - 0.0f64) - 0.6666666666666666f64;
                    let dy1: f64 = dy3;
                    let dz1: f64 = dz2;
                    let mut attn1: f64 = ((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_6(xsb + 0, ysb + 1, zsb + 1, dx1, dy1, dz1));
                    }
                    dx0 = (dx0 - 1.0f64) - 1.0f64;
                    dy0 = (dy0 - 1.0f64) - 1.0f64;
                    dz0 = (dz0 - 1.0f64) - 1.0f64;
                    let mut attn0: f64 = ((2.0f64 - (dx0 * dx0)) - (dy0 * dy0)) - (dz0 * dz0);
                    if attn0 > 0.0f64 {
                        attn0 *= attn0;
                        value += (attn0 * attn0) * (self.extrapolate_6(xsb + 1, ysb + 1, zsb + 1, dx0, dy0, dz0));
                    }
                } else {
                    let p1: f64 = xins + yins;
                    let mut a_point: i8;
                    let mut a_score: f64;
                    let mut a_is_further_side: bool;
                    if p1 > 1.0f64 {
                        a_score = p1 - 1.0f64;
                        a_point = 3i8;
                        a_is_further_side = true;
                    } else {
                        a_score = 1.0f64 - p1;
                        a_point = 4i8;
                        a_is_further_side = false;
                    }
                    let p2: f64 = xins + zins;
                    let mut b_is_further_side: bool;
                    let mut b_score: f64;
                    let mut b_point: i8;
                    if p2 > 1.0f64 {
                        b_score = p2 - 1.0f64;
                        b_point = 5i8;
                        b_is_further_side = true;
                    } else {
                        b_score = 1.0f64 - p2;
                        b_point = 2i8;
                        b_is_further_side = false;
                    }
                    let p3: f64 = yins + zins;
                    if p3 > 1.0f64 {
                        let score: f64 = p3 - 1.0f64;
                        if (a_score <= b_score) && (a_score < score) {
                            a_point = 6i8;
                            a_is_further_side = true;
                        } else if (a_score > b_score) && (b_score < score) {
                            b_point = 6i8;
                            b_is_further_side = true;
                        }
                    } else {
                        let score: f64 = 1.0f64 - p3;
                        if (a_score <= b_score) && (a_score < score) {
                            a_point = 1i8;
                            a_is_further_side = false;
                        } else if (a_score > b_score) && (b_score < score) {
                            b_point = 1i8;
                            b_is_further_side = false;
                        }
                    }
                    if a_is_further_side == b_is_further_side {
                        if a_is_further_side {
                            dx_ext0 = (dx0 - 1.0f64) - 1.0f64;
                            dy_ext0 = (dy0 - 1.0f64) - 1.0f64;
                            dz_ext0 = (dz0 - 1.0f64) - 1.0f64;
                            xsv_ext0 = xsb + 1;
                            ysv_ext0 = ysb + 1;
                            zsv_ext0 = zsb + 1;
                            let c: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                            if ((c as i32) & 1) != 0 {
                                dx_ext1 = (dx0 - 2.0f64) - 0.6666666666666666f64;
                                dy_ext1 = dy0 - 0.6666666666666666f64;
                                dz_ext1 = dz0 - 0.6666666666666666f64;
                                xsv_ext1 = xsb + 2;
                                ysv_ext1 = ysb;
                                zsv_ext1 = zsb;
                            } else if ((c as i32) & 2) != 0 {
                                dx_ext1 = dx0 - 0.6666666666666666f64;
                                dy_ext1 = (dy0 - 2.0f64) - 0.6666666666666666f64;
                                dz_ext1 = dz0 - 0.6666666666666666f64;
                                xsv_ext1 = xsb;
                                ysv_ext1 = ysb + 2;
                                zsv_ext1 = zsb;
                            } else {
                                dx_ext1 = dx0 - 0.6666666666666666f64;
                                dy_ext1 = dy0 - 0.6666666666666666f64;
                                dz_ext1 = (dz0 - 2.0f64) - 0.6666666666666666f64;
                                xsv_ext1 = xsb;
                                ysv_ext1 = ysb;
                                zsv_ext1 = zsb + 2;
                            }
                        } else {
                            dx_ext0 = dx0;
                            dy_ext0 = dy0;
                            dz_ext0 = dz0;
                            xsv_ext0 = xsb;
                            ysv_ext0 = ysb;
                            zsv_ext0 = zsb;
                            let c: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                            if ((c as i32) & 1) == 0 {
                                dx_ext1 = (dx0 + 1.0f64) - 0.3333333333333333f64;
                                dy_ext1 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                                dz_ext1 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                                xsv_ext1 = xsb - 1;
                                ysv_ext1 = ysb + 1;
                                zsv_ext1 = zsb + 1;
                            } else if ((c as i32) & 2) == 0 {
                                dx_ext1 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                                dy_ext1 = (dy0 + 1.0f64) - 0.3333333333333333f64;
                                dz_ext1 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                                xsv_ext1 = xsb + 1;
                                ysv_ext1 = ysb - 1;
                                zsv_ext1 = zsb + 1;
                            } else {
                                dx_ext1 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                                dy_ext1 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                                dz_ext1 = (dz0 + 1.0f64) - 0.3333333333333333f64;
                                xsv_ext1 = xsb + 1;
                                ysv_ext1 = ysb + 1;
                                zsv_ext1 = zsb - 1;
                            }
                        }
                    } else {
                        let mut c2: i8;
                        let mut c1: i8;
                        if a_is_further_side {
                            c1 = a_point;
                            c2 = b_point;
                        } else {
                            c1 = b_point;
                            c2 = a_point;
                        }
                        if ((c1 as i32) & 1) == 0 {
                            dx_ext0 = (dx0 + 1.0f64) - 0.3333333333333333f64;
                            dy_ext0 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                            dz_ext0 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                            xsv_ext0 = xsb - 1;
                            ysv_ext0 = ysb + 1;
                            zsv_ext0 = zsb + 1;
                        } else if ((c1 as i32) & 2) == 0 {
                            dx_ext0 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                            dy_ext0 = (dy0 + 1.0f64) - 0.3333333333333333f64;
                            dz_ext0 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                            xsv_ext0 = xsb + 1;
                            ysv_ext0 = ysb - 1;
                            zsv_ext0 = zsb + 1;
                        } else {
                            dx_ext0 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                            dy_ext0 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                            dz_ext0 = (dz0 + 1.0f64) - 0.3333333333333333f64;
                            xsv_ext0 = xsb + 1;
                            ysv_ext0 = ysb + 1;
                            zsv_ext0 = zsb - 1;
                        }
                        dx_ext1 = dx0 - 0.6666666666666666f64;
                        dy_ext1 = dy0 - 0.6666666666666666f64;
                        dz_ext1 = dz0 - 0.6666666666666666f64;
                        xsv_ext1 = xsb;
                        ysv_ext1 = ysb;
                        zsv_ext1 = zsb;
                        if ((c2 as i32) & 1) != 0 {
                            dx_ext1 -= 2.0f64;
                            xsv_ext1 += 2;
                        } else if ((c2 as i32) & 2) != 0 {
                            dy_ext1 -= 2.0f64;
                            ysv_ext1 += 2;
                        } else {
                            dz_ext1 -= 2.0f64;
                            zsv_ext1 += 2;
                        }
                    }
                    let dx1: f64 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                    let dy1: f64 = (dy0 - 0.0f64) - 0.3333333333333333f64;
                    let dz1: f64 = (dz0 - 0.0f64) - 0.3333333333333333f64;
                    let mut attn1: f64 = ((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_6(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1));
                    }
                    let dx2: f64 = (dx0 - 0.0f64) - 0.3333333333333333f64;
                    let dy2: f64 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                    let dz2: f64 = dz1;
                    let mut attn2: f64 = ((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_6(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2));
                    }
                    let dx3: f64 = dx2;
                    let dy3: f64 = dy1;
                    let dz3: f64 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                    let mut attn3: f64 = ((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_6(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3));
                    }
                    let dx4: f64 = (dx0 - 1.0f64) - 0.6666666666666666f64;
                    let dy4: f64 = (dy0 - 1.0f64) - 0.6666666666666666f64;
                    let dz4: f64 = (dz0 - 0.0f64) - 0.6666666666666666f64;
                    let mut attn4: f64 = ((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_6(xsb + 1, ysb + 1, zsb + 0, dx4, dy4, dz4));
                    }
                    let dx5: f64 = dx4;
                    let dy5: f64 = (dy0 - 0.0f64) - 0.6666666666666666f64;
                    let dz5: f64 = (dz0 - 1.0f64) - 0.6666666666666666f64;
                    let mut attn5: f64 = ((2.0f64 - (dx5 * dx5)) - (dy5 * dy5)) - (dz5 * dz5);
                    if attn5 > 0.0f64 {
                        attn5 *= attn5;
                        value += (attn5 * attn5) * (self.extrapolate_6(xsb + 1, ysb + 0, zsb + 1, dx5, dy5, dz5));
                    }
                    let dx6: f64 = (dx0 - 0.0f64) - 0.6666666666666666f64;
                    let dy6: f64 = dy4;
                    let dz6: f64 = dz5;
                    let mut attn6: f64 = ((2.0f64 - (dx6 * dx6)) - (dy6 * dy6)) - (dz6 * dz6);
                    if attn6 > 0.0f64 {
                        attn6 *= attn6;
                        value += (attn6 * attn6) * (self.extrapolate_6(xsb + 0, ysb + 1, zsb + 1, dx6, dy6, dz6));
                    }
                }
                let mut attn_ext0: f64 = ((2.0f64 - (dx_ext0 * dx_ext0)) - (dy_ext0 * dy_ext0)) - (dz_ext0 * dz_ext0);
                if attn_ext0 > 0.0f64 {
                    attn_ext0 *= attn_ext0;
                    value += (attn_ext0 * attn_ext0) * (self.extrapolate_6(xsv_ext0, ysv_ext0, zsv_ext0, dx_ext0, dy_ext0, dz_ext0));
                }
                let mut attn_ext1: f64 = ((2.0f64 - (dx_ext1 * dx_ext1)) - (dy_ext1 * dy_ext1)) - (dz_ext1 * dz_ext1);
                if attn_ext1 > 0.0f64 {
                    attn_ext1 *= attn_ext1;
                    value += (attn_ext1 * attn_ext1) * (self.extrapolate_6(xsv_ext1, ysv_ext1, zsv_ext1, dx_ext1, dy_ext1, dz_ext1));
                }
                return value / 103.0f64;
            }
            pub fn eval_4(&self, x: f64, y: f64, z: f64, w: f64) -> f64 {
                let stretch_offset: f64 = (((x + y) + z) + w) * (-0.138196601125011f64);
                let xs: f64 = x + stretch_offset;
                let ys: f64 = y + stretch_offset;
                let zs: f64 = z + stretch_offset;
                let ws: f64 = w + stretch_offset;
                let xsb: i32 = open_simplex_noise::fast_floor(xs);
                let ysb: i32 = open_simplex_noise::fast_floor(ys);
                let zsb: i32 = open_simplex_noise::fast_floor(zs);
                let wsb: i32 = open_simplex_noise::fast_floor(ws);
                let squish_offset: f64 = ((((xsb + ysb) + zsb) + wsb) as f64) * 0.309016994374947f64;
                let xb: f64 = (xsb as f64) + squish_offset;
                let yb: f64 = (ysb as f64) + squish_offset;
                let zb: f64 = (zsb as f64) + squish_offset;
                let wb: f64 = (wsb as f64) + squish_offset;
                let xins: f64 = xs - (xsb as f64);
                let yins: f64 = ys - (ysb as f64);
                let zins: f64 = zs - (zsb as f64);
                let wins: f64 = ws - (wsb as f64);
                let in_sum: f64 = ((xins + yins) + zins) + wins;
                let mut dx0: f64 = x - xb;
                let mut dy0: f64 = y - yb;
                let mut dz0: f64 = z - zb;
                let mut dw0: f64 = w - wb;
                let mut value: f64 = 0.0f64;
                let mut dx_ext0: f64;
                let mut dy_ext0: f64;
                let mut dz_ext0: f64;
                let mut dw_ext0: f64;
                let mut dx_ext1: f64;
                let mut dy_ext1: f64;
                let mut dz_ext1: f64;
                let mut dw_ext1: f64;
                let mut dx_ext2: f64;
                let mut dy_ext2: f64;
                let mut dz_ext2: f64;
                let mut dw_ext2: f64;
                let mut xsv_ext0: i32;
                let mut ysv_ext0: i32;
                let mut zsv_ext0: i32;
                let mut wsv_ext0: i32;
                let mut xsv_ext1: i32;
                let mut ysv_ext1: i32;
                let mut zsv_ext1: i32;
                let mut wsv_ext1: i32;
                let mut xsv_ext2: i32;
                let mut ysv_ext2: i32;
                let mut zsv_ext2: i32;
                let mut wsv_ext2: i32;
                if in_sum <= 1.0f64 {
                    let mut a_point: i8 = 1i8;
                    let mut a_score: f64 = xins;
                    let mut b_point: i8 = 2i8;
                    let mut b_score: f64 = yins;
                    if (a_score >= b_score) && (zins > b_score) {
                        b_score = zins;
                        b_point = 4i8;
                    } else if (a_score < b_score) && (zins > a_score) {
                        a_score = zins;
                        a_point = 4i8;
                    }
                    if (a_score >= b_score) && (wins > b_score) {
                        b_score = wins;
                        b_point = 8i8;
                    } else if (a_score < b_score) && (wins > a_score) {
                        a_score = wins;
                        a_point = 8i8;
                    }
                    let uins: f64 = 1.0f64 - in_sum;
                    if (!(uins > a_score)) && (!(uins > b_score)) {
                        let c: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                        if ((c as i32) & 1) == 0 {
                            xsv_ext2 = xsb;
                            xsv_ext0 = xsb;
                            xsv_ext1 = xsb - 1;
                            dx_ext0 = dx0 - 0.618033988749894f64;
                            dx_ext1 = (dx0 + 1.0f64) - 0.309016994374947f64;
                            dx_ext2 = dx0 - 0.309016994374947f64;
                        } else {
                            let chain_0 = xsb + 1;
                            xsv_ext2 = chain_0;
                            xsv_ext1 = chain_0;
                            xsv_ext0 = chain_0;
                            dx_ext0 = (dx0 - 1.0f64) - 0.618033988749894f64;
                            let chain_1 = (dx0 - 1.0f64) - 0.309016994374947f64;
                            dx_ext2 = chain_1;
                            dx_ext1 = chain_1;
                        }
                        if ((c as i32) & 2) == 0 {
                            ysv_ext2 = ysb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext0 = dy0 - 0.618033988749894f64;
                            let chain_2 = dy0 - 0.309016994374947f64;
                            dy_ext2 = chain_2;
                            dy_ext1 = chain_2;
                            if ((c as i32) & 1) == 1 {
                                ysv_ext1 -= 1;
                                dy_ext1 += 1.0f64;
                            } else {
                                ysv_ext2 -= 1;
                                dy_ext2 += 1.0f64;
                            }
                        } else {
                            let chain_3 = ysb + 1;
                            ysv_ext2 = chain_3;
                            ysv_ext1 = chain_3;
                            ysv_ext0 = chain_3;
                            dy_ext0 = (dy0 - 1.0f64) - 0.618033988749894f64;
                            let chain_4 = (dy0 - 1.0f64) - 0.309016994374947f64;
                            dy_ext2 = chain_4;
                            dy_ext1 = chain_4;
                        }
                        if ((c as i32) & 4) == 0 {
                            zsv_ext2 = zsb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            dz_ext0 = dz0 - 0.618033988749894f64;
                            let chain_5 = dz0 - 0.309016994374947f64;
                            dz_ext2 = chain_5;
                            dz_ext1 = chain_5;
                            if ((c as i32) & 3) == 3 {
                                zsv_ext1 -= 1;
                                dz_ext1 += 1.0f64;
                            } else {
                                zsv_ext2 -= 1;
                                dz_ext2 += 1.0f64;
                            }
                        } else {
                            let chain_6 = zsb + 1;
                            zsv_ext2 = chain_6;
                            zsv_ext1 = chain_6;
                            zsv_ext0 = chain_6;
                            dz_ext0 = (dz0 - 1.0f64) - 0.618033988749894f64;
                            let chain_7 = (dz0 - 1.0f64) - 0.309016994374947f64;
                            dz_ext2 = chain_7;
                            dz_ext1 = chain_7;
                        }
                        if ((c as i32) & 8) == 0 {
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            wsv_ext2 = wsb - 1;
                            dw_ext0 = dw0 - 0.618033988749894f64;
                            dw_ext1 = dw0 - 0.309016994374947f64;
                            dw_ext2 = (dw0 + 1.0f64) - 0.309016994374947f64;
                        } else {
                            let chain_8 = wsb + 1;
                            wsv_ext2 = chain_8;
                            wsv_ext1 = chain_8;
                            wsv_ext0 = chain_8;
                            dw_ext0 = (dw0 - 1.0f64) - 0.618033988749894f64;
                            let chain_9 = (dw0 - 1.0f64) - 0.309016994374947f64;
                            dw_ext2 = chain_9;
                            dw_ext1 = chain_9;
                        }
                    } else {
                        let c: i8 = ((if b_score > a_score { (b_point as i32) } else { (a_point as i32) }) as i8);
                        if ((c as i32) & 1) == 0 {
                            xsv_ext0 = xsb - 1;
                            xsv_ext2 = xsb;
                            xsv_ext1 = xsb;
                            dx_ext0 = dx0 + 1.0f64;
                            dx_ext2 = dx0;
                            dx_ext1 = dx0;
                        } else {
                            let chain_10 = xsb + 1;
                            xsv_ext2 = chain_10;
                            xsv_ext1 = chain_10;
                            xsv_ext0 = chain_10;
                            let chain_11 = dx0 - 1.0f64;
                            dx_ext2 = chain_11;
                            dx_ext1 = chain_11;
                            dx_ext0 = chain_11;
                        }
                        if ((c as i32) & 2) == 0 {
                            ysv_ext2 = ysb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext2 = dy0;
                            dy_ext1 = dy0;
                            dy_ext0 = dy0;
                            if ((c as i32) & 1) == 1 {
                                ysv_ext0 -= 1;
                                dy_ext0 += 1.0f64;
                            } else {
                                ysv_ext1 -= 1;
                                dy_ext1 += 1.0f64;
                            }
                        } else {
                            let chain_12 = ysb + 1;
                            ysv_ext2 = chain_12;
                            ysv_ext1 = chain_12;
                            ysv_ext0 = chain_12;
                            let chain_13 = dy0 - 1.0f64;
                            dy_ext2 = chain_13;
                            dy_ext1 = chain_13;
                            dy_ext0 = chain_13;
                        }
                        if ((c as i32) & 4) == 0 {
                            zsv_ext2 = zsb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            dz_ext2 = dz0;
                            dz_ext1 = dz0;
                            dz_ext0 = dz0;
                            if ((c as i32) & 3) != 0 {
                                if ((c as i32) & 3) == 3 {
                                    zsv_ext0 -= 1;
                                    dz_ext0 += 1.0f64;
                                } else {
                                    zsv_ext1 -= 1;
                                    dz_ext1 += 1.0f64;
                                }
                            } else {
                                zsv_ext2 -= 1;
                                dz_ext2 += 1.0f64;
                            }
                        } else {
                            let chain_14 = zsb + 1;
                            zsv_ext2 = chain_14;
                            zsv_ext1 = chain_14;
                            zsv_ext0 = chain_14;
                            let chain_15 = dz0 - 1.0f64;
                            dz_ext2 = chain_15;
                            dz_ext1 = chain_15;
                            dz_ext0 = chain_15;
                        }
                        if ((c as i32) & 8) == 0 {
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            wsv_ext2 = wsb - 1;
                            dw_ext1 = dw0;
                            dw_ext0 = dw0;
                            dw_ext2 = dw0 + 1.0f64;
                        } else {
                            let chain_16 = wsb + 1;
                            wsv_ext2 = chain_16;
                            wsv_ext1 = chain_16;
                            wsv_ext0 = chain_16;
                            let chain_17 = dw0 - 1.0f64;
                            dw_ext2 = chain_17;
                            dw_ext1 = chain_17;
                            dw_ext0 = chain_17;
                        }
                    }
                    let mut attn0: f64 = (((2.0f64 - (dx0 * dx0)) - (dy0 * dy0)) - (dz0 * dz0)) - (dw0 * dw0);
                    if attn0 > 0.0f64 {
                        attn0 *= attn0;
                        value += (attn0 * attn0) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 0, wsb + 0, dx0, dy0, dz0, dw0));
                    }
                    let dx1: f64 = (dx0 - 1.0f64) - 0.309016994374947f64;
                    let dy1: f64 = (dy0 - 0.0f64) - 0.309016994374947f64;
                    let dz1: f64 = (dz0 - 0.0f64) - 0.309016994374947f64;
                    let dw1: f64 = (dw0 - 0.0f64) - 0.309016994374947f64;
                    let mut attn1: f64 = (((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1)) - (dw1 * dw1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 0, wsb + 0, dx1, dy1, dz1, dw1));
                    }
                    let dx2: f64 = (dx0 - 0.0f64) - 0.309016994374947f64;
                    let dy2: f64 = (dy0 - 1.0f64) - 0.309016994374947f64;
                    let dz2: f64 = dz1;
                    let dw2: f64 = dw1;
                    let mut attn2: f64 = (((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2)) - (dw2 * dw2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 0, wsb + 0, dx2, dy2, dz2, dw2));
                    }
                    let dx3: f64 = dx2;
                    let dy3: f64 = dy1;
                    let dz3: f64 = (dz0 - 1.0f64) - 0.309016994374947f64;
                    let dw3: f64 = dw1;
                    let mut attn3: f64 = (((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3)) - (dw3 * dw3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 1, wsb + 0, dx3, dy3, dz3, dw3));
                    }
                    let dx4: f64 = dx2;
                    let dy4: f64 = dy1;
                    let dz4: f64 = dz1;
                    let dw4: f64 = (dw0 - 1.0f64) - 0.309016994374947f64;
                    let mut attn4: f64 = (((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4)) - (dw4 * dw4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 0, wsb + 1, dx4, dy4, dz4, dw4));
                    }
                } else if in_sum >= 3.0f64 {
                    let mut a_point: i8 = 14i8;
                    let mut a_score: f64 = xins;
                    let mut b_point: i8 = 13i8;
                    let mut b_score: f64 = yins;
                    if (a_score <= b_score) && (zins < b_score) {
                        b_score = zins;
                        b_point = 11i8;
                    } else if (a_score > b_score) && (zins < a_score) {
                        a_score = zins;
                        a_point = 11i8;
                    }
                    if (a_score <= b_score) && (wins < b_score) {
                        b_score = wins;
                        b_point = 7i8;
                    } else if (a_score > b_score) && (wins < a_score) {
                        a_score = wins;
                        a_point = 7i8;
                    }
                    let uins: f64 = 4.0f64 - in_sum;
                    if (!(uins < a_score)) && (!(uins < b_score)) {
                        let c: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                        if ((c as i32) & 1) != 0 {
                            let chain_18 = xsb + 1;
                            xsv_ext2 = chain_18;
                            xsv_ext0 = chain_18;
                            xsv_ext1 = xsb + 2;
                            dx_ext0 = (dx0 - 1.0f64) - 0.618033988749894f64;
                            dx_ext1 = (dx0 - 2.0f64) - 0.927050983124841f64;
                            dx_ext2 = (dx0 - 1.0f64) - 0.927050983124841f64;
                        } else {
                            xsv_ext2 = xsb;
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            dx_ext0 = dx0 - 0.618033988749894f64;
                            let chain_19 = dx0 - 0.927050983124841f64;
                            dx_ext2 = chain_19;
                            dx_ext1 = chain_19;
                        }
                        if ((c as i32) & 2) != 0 {
                            let chain_20 = ysb + 1;
                            ysv_ext2 = chain_20;
                            ysv_ext1 = chain_20;
                            ysv_ext0 = chain_20;
                            dy_ext0 = (dy0 - 1.0f64) - 0.618033988749894f64;
                            let chain_21 = (dy0 - 1.0f64) - 0.927050983124841f64;
                            dy_ext2 = chain_21;
                            dy_ext1 = chain_21;
                            if ((c as i32) & 1) != 0 {
                                ysv_ext2 += 1;
                                dy_ext2 -= 1.0f64;
                            } else {
                                ysv_ext1 += 1;
                                dy_ext1 -= 1.0f64;
                            }
                        } else {
                            ysv_ext2 = ysb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext0 = dy0 - 0.618033988749894f64;
                            let chain_22 = dy0 - 0.927050983124841f64;
                            dy_ext2 = chain_22;
                            dy_ext1 = chain_22;
                        }
                        if ((c as i32) & 4) != 0 {
                            let chain_23 = zsb + 1;
                            zsv_ext2 = chain_23;
                            zsv_ext1 = chain_23;
                            zsv_ext0 = chain_23;
                            dz_ext0 = (dz0 - 1.0f64) - 0.618033988749894f64;
                            let chain_24 = (dz0 - 1.0f64) - 0.927050983124841f64;
                            dz_ext2 = chain_24;
                            dz_ext1 = chain_24;
                            if ((c as i32) & 3) != 0 {
                                zsv_ext2 += 1;
                                dz_ext2 -= 1.0f64;
                            } else {
                                zsv_ext1 += 1;
                                dz_ext1 -= 1.0f64;
                            }
                        } else {
                            zsv_ext2 = zsb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            dz_ext0 = dz0 - 0.618033988749894f64;
                            let chain_25 = dz0 - 0.927050983124841f64;
                            dz_ext2 = chain_25;
                            dz_ext1 = chain_25;
                        }
                        if ((c as i32) & 8) != 0 {
                            let chain_26 = wsb + 1;
                            wsv_ext1 = chain_26;
                            wsv_ext0 = chain_26;
                            wsv_ext2 = wsb + 2;
                            dw_ext0 = (dw0 - 1.0f64) - 0.618033988749894f64;
                            dw_ext1 = (dw0 - 1.0f64) - 0.927050983124841f64;
                            dw_ext2 = (dw0 - 2.0f64) - 0.927050983124841f64;
                        } else {
                            wsv_ext2 = wsb;
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            dw_ext0 = dw0 - 0.618033988749894f64;
                            let chain_27 = dw0 - 0.927050983124841f64;
                            dw_ext2 = chain_27;
                            dw_ext1 = chain_27;
                        }
                    } else {
                        let c: i8 = ((if b_score < a_score { (b_point as i32) } else { (a_point as i32) }) as i8);
                        if ((c as i32) & 1) != 0 {
                            xsv_ext0 = xsb + 2;
                            let chain_28 = xsb + 1;
                            xsv_ext2 = chain_28;
                            xsv_ext1 = chain_28;
                            dx_ext0 = (dx0 - 2.0f64) - 1.236067977499788f64;
                            let chain_29 = (dx0 - 1.0f64) - 1.236067977499788f64;
                            dx_ext2 = chain_29;
                            dx_ext1 = chain_29;
                        } else {
                            xsv_ext2 = xsb;
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            let chain_30 = dx0 - 1.236067977499788f64;
                            dx_ext2 = chain_30;
                            dx_ext1 = chain_30;
                            dx_ext0 = chain_30;
                        }
                        if ((c as i32) & 2) != 0 {
                            let chain_31 = ysb + 1;
                            ysv_ext2 = chain_31;
                            ysv_ext1 = chain_31;
                            ysv_ext0 = chain_31;
                            let chain_32 = (dy0 - 1.0f64) - 1.236067977499788f64;
                            dy_ext2 = chain_32;
                            dy_ext1 = chain_32;
                            dy_ext0 = chain_32;
                            if ((c as i32) & 1) != 0 {
                                ysv_ext1 += 1;
                                dy_ext1 -= 1.0f64;
                            } else {
                                ysv_ext0 += 1;
                                dy_ext0 -= 1.0f64;
                            }
                        } else {
                            ysv_ext2 = ysb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            let chain_33 = dy0 - 1.236067977499788f64;
                            dy_ext2 = chain_33;
                            dy_ext1 = chain_33;
                            dy_ext0 = chain_33;
                        }
                        if ((c as i32) & 4) != 0 {
                            let chain_34 = zsb + 1;
                            zsv_ext2 = chain_34;
                            zsv_ext1 = chain_34;
                            zsv_ext0 = chain_34;
                            let chain_35 = (dz0 - 1.0f64) - 1.236067977499788f64;
                            dz_ext2 = chain_35;
                            dz_ext1 = chain_35;
                            dz_ext0 = chain_35;
                            if ((c as i32) & 3) != 3 {
                                if ((c as i32) & 3) == 0 {
                                    zsv_ext0 += 1;
                                    dz_ext0 -= 1.0f64;
                                } else {
                                    zsv_ext1 += 1;
                                    dz_ext1 -= 1.0f64;
                                }
                            } else {
                                zsv_ext2 += 1;
                                dz_ext2 -= 1.0f64;
                            }
                        } else {
                            zsv_ext2 = zsb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            let chain_36 = dz0 - 1.236067977499788f64;
                            dz_ext2 = chain_36;
                            dz_ext1 = chain_36;
                            dz_ext0 = chain_36;
                        }
                        if ((c as i32) & 8) != 0 {
                            let chain_37 = wsb + 1;
                            wsv_ext1 = chain_37;
                            wsv_ext0 = chain_37;
                            wsv_ext2 = wsb + 2;
                            let chain_38 = (dw0 - 1.0f64) - 1.236067977499788f64;
                            dw_ext1 = chain_38;
                            dw_ext0 = chain_38;
                            dw_ext2 = (dw0 - 2.0f64) - 1.236067977499788f64;
                        } else {
                            wsv_ext2 = wsb;
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            let chain_39 = dw0 - 1.236067977499788f64;
                            dw_ext2 = chain_39;
                            dw_ext1 = chain_39;
                            dw_ext0 = chain_39;
                        }
                    }
                    let dx4: f64 = (dx0 - 1.0f64) - 0.927050983124841f64;
                    let dy4: f64 = (dy0 - 1.0f64) - 0.927050983124841f64;
                    let dz4: f64 = (dz0 - 1.0f64) - 0.927050983124841f64;
                    let dw4: f64 = dw0 - 0.927050983124841f64;
                    let mut attn4: f64 = (((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4)) - (dw4 * dw4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 1, wsb + 0, dx4, dy4, dz4, dw4));
                    }
                    let dx3: f64 = dx4;
                    let dy3: f64 = dy4;
                    let dz3: f64 = dz0 - 0.927050983124841f64;
                    let dw3: f64 = (dw0 - 1.0f64) - 0.927050983124841f64;
                    let mut attn3: f64 = (((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3)) - (dw3 * dw3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 0, wsb + 1, dx3, dy3, dz3, dw3));
                    }
                    let dx2: f64 = dx4;
                    let dy2: f64 = dy0 - 0.927050983124841f64;
                    let dz2: f64 = dz4;
                    let dw2: f64 = dw3;
                    let mut attn2: f64 = (((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2)) - (dw2 * dw2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 1, wsb + 1, dx2, dy2, dz2, dw2));
                    }
                    let dx1: f64 = dx0 - 0.927050983124841f64;
                    let dz1: f64 = dz4;
                    let dy1: f64 = dy4;
                    let dw1: f64 = dw3;
                    let mut attn1: f64 = (((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1)) - (dw1 * dw1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 1, wsb + 1, dx1, dy1, dz1, dw1));
                    }
                    dx0 = (dx0 - 1.0f64) - 1.236067977499788f64;
                    dy0 = (dy0 - 1.0f64) - 1.236067977499788f64;
                    dz0 = (dz0 - 1.0f64) - 1.236067977499788f64;
                    dw0 = (dw0 - 1.0f64) - 1.236067977499788f64;
                    let mut attn0: f64 = (((2.0f64 - (dx0 * dx0)) - (dy0 * dy0)) - (dz0 * dz0)) - (dw0 * dw0);
                    if attn0 > 0.0f64 {
                        attn0 *= attn0;
                        value += (attn0 * attn0) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 1, wsb + 1, dx0, dy0, dz0, dw0));
                    }
                } else if in_sum <= 2.0f64 {
                    let mut a_is_bigger_side: bool = true;
                    let mut b_is_bigger_side: bool = true;
                    let mut a_point: i8;
                    let mut a_score: f64;
                    if (xins + yins) > (zins + wins) {
                        a_score = xins + yins;
                        a_point = 3i8;
                    } else {
                        a_score = zins + wins;
                        a_point = 12i8;
                    }
                    let mut b_score: f64;
                    let mut b_point: i8;
                    if (xins + zins) > (yins + wins) {
                        b_score = xins + zins;
                        b_point = 5i8;
                    } else {
                        b_score = yins + wins;
                        b_point = 10i8;
                    }
                    if (xins + wins) > (yins + zins) {
                        let score: f64 = xins + wins;
                        if (a_score >= b_score) && (score > b_score) {
                            b_score = score;
                            b_point = 9i8;
                        } else if (a_score < b_score) && (score > a_score) {
                            a_score = score;
                            a_point = 9i8;
                        }
                    } else {
                        let score: f64 = yins + zins;
                        if (a_score >= b_score) && (score > b_score) {
                            b_score = score;
                            b_point = 6i8;
                        } else if (a_score < b_score) && (score > a_score) {
                            a_score = score;
                            a_point = 6i8;
                        }
                    }
                    let p1: f64 = (2.0f64 - in_sum) + xins;
                    if (a_score >= b_score) && (p1 > b_score) {
                        b_score = p1;
                        b_point = 1i8;
                        b_is_bigger_side = false;
                    } else if (a_score < b_score) && (p1 > a_score) {
                        a_score = p1;
                        a_point = 1i8;
                        a_is_bigger_side = false;
                    }
                    let p2: f64 = (2.0f64 - in_sum) + yins;
                    if (a_score >= b_score) && (p2 > b_score) {
                        b_score = p2;
                        b_point = 2i8;
                        b_is_bigger_side = false;
                    } else if (a_score < b_score) && (p2 > a_score) {
                        a_score = p2;
                        a_point = 2i8;
                        a_is_bigger_side = false;
                    }
                    let p3: f64 = (2.0f64 - in_sum) + zins;
                    if (a_score >= b_score) && (p3 > b_score) {
                        b_score = p3;
                        b_point = 4i8;
                        b_is_bigger_side = false;
                    } else if (a_score < b_score) && (p3 > a_score) {
                        a_score = p3;
                        a_point = 4i8;
                        a_is_bigger_side = false;
                    }
                    let p4: f64 = (2.0f64 - in_sum) + wins;
                    if (a_score >= b_score) && (p4 > b_score) {
                        b_point = 8i8;
                        b_is_bigger_side = false;
                    } else if (a_score < b_score) && (p4 > a_score) {
                        a_point = 8i8;
                        a_is_bigger_side = false;
                    }
                    if a_is_bigger_side == b_is_bigger_side {
                        if a_is_bigger_side {
                            let mut c1: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                            let mut c2: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                            if ((c1 as i32) & 1) == 0 {
                                xsv_ext0 = xsb;
                                xsv_ext1 = xsb - 1;
                                dx_ext0 = dx0 - 0.927050983124841f64;
                                dx_ext1 = (dx0 + 1.0f64) - 0.618033988749894f64;
                            } else {
                                let chain_40 = xsb + 1;
                                xsv_ext1 = chain_40;
                                xsv_ext0 = chain_40;
                                dx_ext0 = (dx0 - 1.0f64) - 0.927050983124841f64;
                                dx_ext1 = (dx0 - 1.0f64) - 0.618033988749894f64;
                            }
                            if ((c1 as i32) & 2) == 0 {
                                ysv_ext0 = ysb;
                                ysv_ext1 = ysb - 1;
                                dy_ext0 = dy0 - 0.927050983124841f64;
                                dy_ext1 = (dy0 + 1.0f64) - 0.618033988749894f64;
                            } else {
                                let chain_41 = ysb + 1;
                                ysv_ext1 = chain_41;
                                ysv_ext0 = chain_41;
                                dy_ext0 = (dy0 - 1.0f64) - 0.927050983124841f64;
                                dy_ext1 = (dy0 - 1.0f64) - 0.618033988749894f64;
                            }
                            if ((c1 as i32) & 4) == 0 {
                                zsv_ext0 = zsb;
                                zsv_ext1 = zsb - 1;
                                dz_ext0 = dz0 - 0.927050983124841f64;
                                dz_ext1 = (dz0 + 1.0f64) - 0.618033988749894f64;
                            } else {
                                let chain_42 = zsb + 1;
                                zsv_ext1 = chain_42;
                                zsv_ext0 = chain_42;
                                dz_ext0 = (dz0 - 1.0f64) - 0.927050983124841f64;
                                dz_ext1 = (dz0 - 1.0f64) - 0.618033988749894f64;
                            }
                            if ((c1 as i32) & 8) == 0 {
                                wsv_ext0 = wsb;
                                wsv_ext1 = wsb - 1;
                                dw_ext0 = dw0 - 0.927050983124841f64;
                                dw_ext1 = (dw0 + 1.0f64) - 0.618033988749894f64;
                            } else {
                                let chain_43 = wsb + 1;
                                wsv_ext1 = chain_43;
                                wsv_ext0 = chain_43;
                                dw_ext0 = (dw0 - 1.0f64) - 0.927050983124841f64;
                                dw_ext1 = (dw0 - 1.0f64) - 0.618033988749894f64;
                            }
                            xsv_ext2 = xsb;
                            ysv_ext2 = ysb;
                            zsv_ext2 = zsb;
                            wsv_ext2 = wsb;
                            dx_ext2 = dx0 - 0.618033988749894f64;
                            dy_ext2 = dy0 - 0.618033988749894f64;
                            dz_ext2 = dz0 - 0.618033988749894f64;
                            dw_ext2 = dw0 - 0.618033988749894f64;
                            if ((c2 as i32) & 1) != 0 {
                                xsv_ext2 += 2;
                                dx_ext2 -= 2.0f64;
                            } else if ((c2 as i32) & 2) != 0 {
                                ysv_ext2 += 2;
                                dy_ext2 -= 2.0f64;
                            } else if ((c2 as i32) & 4) != 0 {
                                zsv_ext2 += 2;
                                dz_ext2 -= 2.0f64;
                            } else {
                                wsv_ext2 += 2;
                                dw_ext2 -= 2.0f64;
                            }
                        } else {
                            xsv_ext2 = xsb;
                            ysv_ext2 = ysb;
                            zsv_ext2 = zsb;
                            wsv_ext2 = wsb;
                            dx_ext2 = dx0;
                            dy_ext2 = dy0;
                            dz_ext2 = dz0;
                            dw_ext2 = dw0;
                            let c: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                            if ((c as i32) & 1) == 0 {
                                xsv_ext0 = xsb - 1;
                                xsv_ext1 = xsb;
                                dx_ext0 = (dx0 + 1.0f64) - 0.309016994374947f64;
                                dx_ext1 = dx0 - 0.309016994374947f64;
                            } else {
                                let chain_44 = xsb + 1;
                                xsv_ext1 = chain_44;
                                xsv_ext0 = chain_44;
                                let chain_45 = (dx0 - 1.0f64) - 0.309016994374947f64;
                                dx_ext1 = chain_45;
                                dx_ext0 = chain_45;
                            }
                            if ((c as i32) & 2) == 0 {
                                ysv_ext1 = ysb;
                                ysv_ext0 = ysb;
                                let chain_46 = dy0 - 0.309016994374947f64;
                                dy_ext1 = chain_46;
                                dy_ext0 = chain_46;
                                if ((c as i32) & 1) == 1 {
                                    ysv_ext0 -= 1;
                                    dy_ext0 += 1.0f64;
                                } else {
                                    ysv_ext1 -= 1;
                                    dy_ext1 += 1.0f64;
                                }
                            } else {
                                let chain_47 = ysb + 1;
                                ysv_ext1 = chain_47;
                                ysv_ext0 = chain_47;
                                let chain_48 = (dy0 - 1.0f64) - 0.309016994374947f64;
                                dy_ext1 = chain_48;
                                dy_ext0 = chain_48;
                            }
                            if ((c as i32) & 4) == 0 {
                                zsv_ext1 = zsb;
                                zsv_ext0 = zsb;
                                let chain_49 = dz0 - 0.309016994374947f64;
                                dz_ext1 = chain_49;
                                dz_ext0 = chain_49;
                                if ((c as i32) & 3) == 3 {
                                    zsv_ext0 -= 1;
                                    dz_ext0 += 1.0f64;
                                } else {
                                    zsv_ext1 -= 1;
                                    dz_ext1 += 1.0f64;
                                }
                            } else {
                                let chain_50 = zsb + 1;
                                zsv_ext1 = chain_50;
                                zsv_ext0 = chain_50;
                                let chain_51 = (dz0 - 1.0f64) - 0.309016994374947f64;
                                dz_ext1 = chain_51;
                                dz_ext0 = chain_51;
                            }
                            if ((c as i32) & 8) == 0 {
                                wsv_ext0 = wsb;
                                wsv_ext1 = wsb - 1;
                                dw_ext0 = dw0 - 0.309016994374947f64;
                                dw_ext1 = (dw0 + 1.0f64) - 0.309016994374947f64;
                            } else {
                                let chain_52 = wsb + 1;
                                wsv_ext1 = chain_52;
                                wsv_ext0 = chain_52;
                                let chain_53 = (dw0 - 1.0f64) - 0.309016994374947f64;
                                dw_ext1 = chain_53;
                                dw_ext0 = chain_53;
                            }
                        }
                    } else {
                        let mut c1: i8;
                        let mut c2: i8;
                        if a_is_bigger_side {
                            c1 = a_point;
                            c2 = b_point;
                        } else {
                            c1 = b_point;
                            c2 = a_point;
                        }
                        if ((c1 as i32) & 1) == 0 {
                            xsv_ext0 = xsb - 1;
                            xsv_ext1 = xsb;
                            dx_ext0 = (dx0 + 1.0f64) - 0.309016994374947f64;
                            dx_ext1 = dx0 - 0.309016994374947f64;
                        } else {
                            let chain_54 = xsb + 1;
                            xsv_ext1 = chain_54;
                            xsv_ext0 = chain_54;
                            let chain_55 = (dx0 - 1.0f64) - 0.309016994374947f64;
                            dx_ext1 = chain_55;
                            dx_ext0 = chain_55;
                        }
                        if ((c1 as i32) & 2) == 0 {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            let chain_56 = dy0 - 0.309016994374947f64;
                            dy_ext1 = chain_56;
                            dy_ext0 = chain_56;
                            if ((c1 as i32) & 1) == 1 {
                                ysv_ext0 -= 1;
                                dy_ext0 += 1.0f64;
                            } else {
                                ysv_ext1 -= 1;
                                dy_ext1 += 1.0f64;
                            }
                        } else {
                            let chain_57 = ysb + 1;
                            ysv_ext1 = chain_57;
                            ysv_ext0 = chain_57;
                            let chain_58 = (dy0 - 1.0f64) - 0.309016994374947f64;
                            dy_ext1 = chain_58;
                            dy_ext0 = chain_58;
                        }
                        if ((c1 as i32) & 4) == 0 {
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            let chain_59 = dz0 - 0.309016994374947f64;
                            dz_ext1 = chain_59;
                            dz_ext0 = chain_59;
                            if ((c1 as i32) & 3) == 3 {
                                zsv_ext0 -= 1;
                                dz_ext0 += 1.0f64;
                            } else {
                                zsv_ext1 -= 1;
                                dz_ext1 += 1.0f64;
                            }
                        } else {
                            let chain_60 = zsb + 1;
                            zsv_ext1 = chain_60;
                            zsv_ext0 = chain_60;
                            let chain_61 = (dz0 - 1.0f64) - 0.309016994374947f64;
                            dz_ext1 = chain_61;
                            dz_ext0 = chain_61;
                        }
                        if ((c1 as i32) & 8) == 0 {
                            wsv_ext0 = wsb;
                            wsv_ext1 = wsb - 1;
                            dw_ext0 = dw0 - 0.309016994374947f64;
                            dw_ext1 = (dw0 + 1.0f64) - 0.309016994374947f64;
                        } else {
                            let chain_62 = wsb + 1;
                            wsv_ext1 = chain_62;
                            wsv_ext0 = chain_62;
                            let chain_63 = (dw0 - 1.0f64) - 0.309016994374947f64;
                            dw_ext1 = chain_63;
                            dw_ext0 = chain_63;
                        }
                        xsv_ext2 = xsb;
                        ysv_ext2 = ysb;
                        zsv_ext2 = zsb;
                        wsv_ext2 = wsb;
                        dx_ext2 = dx0 - 0.618033988749894f64;
                        dy_ext2 = dy0 - 0.618033988749894f64;
                        dz_ext2 = dz0 - 0.618033988749894f64;
                        dw_ext2 = dw0 - 0.618033988749894f64;
                        if ((c2 as i32) & 1) != 0 {
                            xsv_ext2 += 2;
                            dx_ext2 -= 2.0f64;
                        } else if ((c2 as i32) & 2) != 0 {
                            ysv_ext2 += 2;
                            dy_ext2 -= 2.0f64;
                        } else if ((c2 as i32) & 4) != 0 {
                            zsv_ext2 += 2;
                            dz_ext2 -= 2.0f64;
                        } else {
                            wsv_ext2 += 2;
                            dw_ext2 -= 2.0f64;
                        }
                    }
                    let dx1: f64 = (dx0 - 1.0f64) - 0.309016994374947f64;
                    let dy1: f64 = (dy0 - 0.0f64) - 0.309016994374947f64;
                    let dz1: f64 = (dz0 - 0.0f64) - 0.309016994374947f64;
                    let dw1: f64 = (dw0 - 0.0f64) - 0.309016994374947f64;
                    let mut attn1: f64 = (((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1)) - (dw1 * dw1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 0, wsb + 0, dx1, dy1, dz1, dw1));
                    }
                    let dx2: f64 = (dx0 - 0.0f64) - 0.309016994374947f64;
                    let dy2: f64 = (dy0 - 1.0f64) - 0.309016994374947f64;
                    let dz2: f64 = dz1;
                    let dw2: f64 = dw1;
                    let mut attn2: f64 = (((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2)) - (dw2 * dw2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 0, wsb + 0, dx2, dy2, dz2, dw2));
                    }
                    let dx3: f64 = dx2;
                    let dy3: f64 = dy1;
                    let dz3: f64 = (dz0 - 1.0f64) - 0.309016994374947f64;
                    let dw3: f64 = dw1;
                    let mut attn3: f64 = (((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3)) - (dw3 * dw3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 1, wsb + 0, dx3, dy3, dz3, dw3));
                    }
                    let dx4: f64 = dx2;
                    let dy4: f64 = dy1;
                    let dz4: f64 = dz1;
                    let dw4: f64 = (dw0 - 1.0f64) - 0.309016994374947f64;
                    let mut attn4: f64 = (((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4)) - (dw4 * dw4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 0, wsb + 1, dx4, dy4, dz4, dw4));
                    }
                    let dx5: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy5: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz5: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw5: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn5: f64 = (((2.0f64 - (dx5 * dx5)) - (dy5 * dy5)) - (dz5 * dz5)) - (dw5 * dw5);
                    if attn5 > 0.0f64 {
                        attn5 *= attn5;
                        value += (attn5 * attn5) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 0, wsb + 0, dx5, dy5, dz5, dw5));
                    }
                    let dx6: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy6: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz6: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw6: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn6: f64 = (((2.0f64 - (dx6 * dx6)) - (dy6 * dy6)) - (dz6 * dz6)) - (dw6 * dw6);
                    if attn6 > 0.0f64 {
                        attn6 *= attn6;
                        value += (attn6 * attn6) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 1, wsb + 0, dx6, dy6, dz6, dw6));
                    }
                    let dx7: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy7: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz7: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw7: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn7: f64 = (((2.0f64 - (dx7 * dx7)) - (dy7 * dy7)) - (dz7 * dz7)) - (dw7 * dw7);
                    if attn7 > 0.0f64 {
                        attn7 *= attn7;
                        value += (attn7 * attn7) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 0, wsb + 1, dx7, dy7, dz7, dw7));
                    }
                    let dx8: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy8: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz8: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw8: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn8: f64 = (((2.0f64 - (dx8 * dx8)) - (dy8 * dy8)) - (dz8 * dz8)) - (dw8 * dw8);
                    if attn8 > 0.0f64 {
                        attn8 *= attn8;
                        value += (attn8 * attn8) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 1, wsb + 0, dx8, dy8, dz8, dw8));
                    }
                    let dx9: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy9: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz9: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw9: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn9: f64 = (((2.0f64 - (dx9 * dx9)) - (dy9 * dy9)) - (dz9 * dz9)) - (dw9 * dw9);
                    if attn9 > 0.0f64 {
                        attn9 *= attn9;
                        value += (attn9 * attn9) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 0, wsb + 1, dx9, dy9, dz9, dw9));
                    }
                    let dx10: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy10: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz10: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw10: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn10: f64 = (((2.0f64 - (dx10 * dx10)) - (dy10 * dy10)) - (dz10 * dz10)) - (dw10 * dw10);
                    if attn10 > 0.0f64 {
                        attn10 *= attn10;
                        value += (attn10 * attn10) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 1, wsb + 1, dx10, dy10, dz10, dw10));
                    }
                } else {
                    let mut a_is_bigger_side: bool = true;
                    let mut b_is_bigger_side: bool = true;
                    let mut a_score: f64;
                    let mut a_point: i8;
                    if (xins + yins) < (zins + wins) {
                        a_score = xins + yins;
                        a_point = 12i8;
                    } else {
                        a_score = zins + wins;
                        a_point = 3i8;
                    }
                    let mut b_score: f64;
                    let mut b_point: i8;
                    if (xins + zins) < (yins + wins) {
                        b_score = xins + zins;
                        b_point = 10i8;
                    } else {
                        b_score = yins + wins;
                        b_point = 5i8;
                    }
                    if (xins + wins) < (yins + zins) {
                        let score: f64 = xins + wins;
                        if (a_score <= b_score) && (score < b_score) {
                            b_score = score;
                            b_point = 6i8;
                        } else if (a_score > b_score) && (score < a_score) {
                            a_score = score;
                            a_point = 6i8;
                        }
                    } else {
                        let score: f64 = yins + zins;
                        if (a_score <= b_score) && (score < b_score) {
                            b_score = score;
                            b_point = 9i8;
                        } else if (a_score > b_score) && (score < a_score) {
                            a_score = score;
                            a_point = 9i8;
                        }
                    }
                    let p1: f64 = (3.0f64 - in_sum) + xins;
                    if (a_score <= b_score) && (p1 < b_score) {
                        b_score = p1;
                        b_point = 14i8;
                        b_is_bigger_side = false;
                    } else if (a_score > b_score) && (p1 < a_score) {
                        a_score = p1;
                        a_point = 14i8;
                        a_is_bigger_side = false;
                    }
                    let p2: f64 = (3.0f64 - in_sum) + yins;
                    if (a_score <= b_score) && (p2 < b_score) {
                        b_score = p2;
                        b_point = 13i8;
                        b_is_bigger_side = false;
                    } else if (a_score > b_score) && (p2 < a_score) {
                        a_score = p2;
                        a_point = 13i8;
                        a_is_bigger_side = false;
                    }
                    let p3: f64 = (3.0f64 - in_sum) + zins;
                    if (a_score <= b_score) && (p3 < b_score) {
                        b_score = p3;
                        b_point = 11i8;
                        b_is_bigger_side = false;
                    } else if (a_score > b_score) && (p3 < a_score) {
                        a_score = p3;
                        a_point = 11i8;
                        a_is_bigger_side = false;
                    }
                    let p4: f64 = (3.0f64 - in_sum) + wins;
                    if (a_score <= b_score) && (p4 < b_score) {
                        b_point = 7i8;
                        b_is_bigger_side = false;
                    } else if (a_score > b_score) && (p4 < a_score) {
                        a_point = 7i8;
                        a_is_bigger_side = false;
                    }
                    if a_is_bigger_side == b_is_bigger_side {
                        if a_is_bigger_side {
                            let mut c1: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                            let mut c2: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            dx_ext0 = dx0 - 0.309016994374947f64;
                            dy_ext0 = dy0 - 0.309016994374947f64;
                            dz_ext0 = dz0 - 0.309016994374947f64;
                            dw_ext0 = dw0 - 0.309016994374947f64;
                            dx_ext1 = dx0 - 0.618033988749894f64;
                            dy_ext1 = dy0 - 0.618033988749894f64;
                            dz_ext1 = dz0 - 0.618033988749894f64;
                            dw_ext1 = dw0 - 0.618033988749894f64;
                            if ((c1 as i32) & 1) != 0 {
                                xsv_ext0 += 1;
                                dx_ext0 -= 1.0f64;
                                xsv_ext1 += 2;
                                dx_ext1 -= 2.0f64;
                            } else if ((c1 as i32) & 2) != 0 {
                                ysv_ext0 += 1;
                                dy_ext0 -= 1.0f64;
                                ysv_ext1 += 2;
                                dy_ext1 -= 2.0f64;
                            } else if ((c1 as i32) & 4) != 0 {
                                zsv_ext0 += 1;
                                dz_ext0 -= 1.0f64;
                                zsv_ext1 += 2;
                                dz_ext1 -= 2.0f64;
                            } else {
                                wsv_ext0 += 1;
                                dw_ext0 -= 1.0f64;
                                wsv_ext1 += 2;
                                dw_ext1 -= 2.0f64;
                            }
                            xsv_ext2 = xsb + 1;
                            ysv_ext2 = ysb + 1;
                            zsv_ext2 = zsb + 1;
                            wsv_ext2 = wsb + 1;
                            dx_ext2 = (dx0 - 1.0f64) - 0.618033988749894f64;
                            dy_ext2 = (dy0 - 1.0f64) - 0.618033988749894f64;
                            dz_ext2 = (dz0 - 1.0f64) - 0.618033988749894f64;
                            dw_ext2 = (dw0 - 1.0f64) - 0.618033988749894f64;
                            if ((c2 as i32) & 1) == 0 {
                                xsv_ext2 -= 2;
                                dx_ext2 += 2.0f64;
                            } else if ((c2 as i32) & 2) == 0 {
                                ysv_ext2 -= 2;
                                dy_ext2 += 2.0f64;
                            } else if ((c2 as i32) & 4) == 0 {
                                zsv_ext2 -= 2;
                                dz_ext2 += 2.0f64;
                            } else {
                                wsv_ext2 -= 2;
                                dw_ext2 += 2.0f64;
                            }
                        } else {
                            xsv_ext2 = xsb + 1;
                            ysv_ext2 = ysb + 1;
                            zsv_ext2 = zsb + 1;
                            wsv_ext2 = wsb + 1;
                            dx_ext2 = (dx0 - 1.0f64) - 1.236067977499788f64;
                            dy_ext2 = (dy0 - 1.0f64) - 1.236067977499788f64;
                            dz_ext2 = (dz0 - 1.0f64) - 1.236067977499788f64;
                            dw_ext2 = (dw0 - 1.0f64) - 1.236067977499788f64;
                            let c: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                            if ((c as i32) & 1) != 0 {
                                xsv_ext0 = xsb + 2;
                                xsv_ext1 = xsb + 1;
                                dx_ext0 = (dx0 - 2.0f64) - 0.927050983124841f64;
                                dx_ext1 = (dx0 - 1.0f64) - 0.927050983124841f64;
                            } else {
                                xsv_ext1 = xsb;
                                xsv_ext0 = xsb;
                                let chain_64 = dx0 - 0.927050983124841f64;
                                dx_ext1 = chain_64;
                                dx_ext0 = chain_64;
                            }
                            if ((c as i32) & 2) != 0 {
                                let chain_65 = ysb + 1;
                                ysv_ext1 = chain_65;
                                ysv_ext0 = chain_65;
                                let chain_66 = (dy0 - 1.0f64) - 0.927050983124841f64;
                                dy_ext1 = chain_66;
                                dy_ext0 = chain_66;
                                if ((c as i32) & 1) == 0 {
                                    ysv_ext0 += 1;
                                    dy_ext0 -= 1.0f64;
                                } else {
                                    ysv_ext1 += 1;
                                    dy_ext1 -= 1.0f64;
                                }
                            } else {
                                ysv_ext1 = ysb;
                                ysv_ext0 = ysb;
                                let chain_67 = dy0 - 0.927050983124841f64;
                                dy_ext1 = chain_67;
                                dy_ext0 = chain_67;
                            }
                            if ((c as i32) & 4) != 0 {
                                let chain_68 = zsb + 1;
                                zsv_ext1 = chain_68;
                                zsv_ext0 = chain_68;
                                let chain_69 = (dz0 - 1.0f64) - 0.927050983124841f64;
                                dz_ext1 = chain_69;
                                dz_ext0 = chain_69;
                                if ((c as i32) & 3) == 0 {
                                    zsv_ext0 += 1;
                                    dz_ext0 -= 1.0f64;
                                } else {
                                    zsv_ext1 += 1;
                                    dz_ext1 -= 1.0f64;
                                }
                            } else {
                                zsv_ext1 = zsb;
                                zsv_ext0 = zsb;
                                let chain_70 = dz0 - 0.927050983124841f64;
                                dz_ext1 = chain_70;
                                dz_ext0 = chain_70;
                            }
                            if ((c as i32) & 8) != 0 {
                                wsv_ext0 = wsb + 1;
                                wsv_ext1 = wsb + 2;
                                dw_ext0 = (dw0 - 1.0f64) - 0.927050983124841f64;
                                dw_ext1 = (dw0 - 2.0f64) - 0.927050983124841f64;
                            } else {
                                wsv_ext1 = wsb;
                                wsv_ext0 = wsb;
                                let chain_71 = dw0 - 0.927050983124841f64;
                                dw_ext1 = chain_71;
                                dw_ext0 = chain_71;
                            }
                        }
                    } else {
                        let mut c1: i8;
                        let mut c2: i8;
                        if a_is_bigger_side {
                            c1 = a_point;
                            c2 = b_point;
                        } else {
                            c1 = b_point;
                            c2 = a_point;
                        }
                        if ((c1 as i32) & 1) != 0 {
                            xsv_ext0 = xsb + 2;
                            xsv_ext1 = xsb + 1;
                            dx_ext0 = (dx0 - 2.0f64) - 0.927050983124841f64;
                            dx_ext1 = (dx0 - 1.0f64) - 0.927050983124841f64;
                        } else {
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            let chain_72 = dx0 - 0.927050983124841f64;
                            dx_ext1 = chain_72;
                            dx_ext0 = chain_72;
                        }
                        if ((c1 as i32) & 2) != 0 {
                            let chain_73 = ysb + 1;
                            ysv_ext1 = chain_73;
                            ysv_ext0 = chain_73;
                            let chain_74 = (dy0 - 1.0f64) - 0.927050983124841f64;
                            dy_ext1 = chain_74;
                            dy_ext0 = chain_74;
                            if ((c1 as i32) & 1) == 0 {
                                ysv_ext0 += 1;
                                dy_ext0 -= 1.0f64;
                            } else {
                                ysv_ext1 += 1;
                                dy_ext1 -= 1.0f64;
                            }
                        } else {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            let chain_75 = dy0 - 0.927050983124841f64;
                            dy_ext1 = chain_75;
                            dy_ext0 = chain_75;
                        }
                        if ((c1 as i32) & 4) != 0 {
                            let chain_76 = zsb + 1;
                            zsv_ext1 = chain_76;
                            zsv_ext0 = chain_76;
                            let chain_77 = (dz0 - 1.0f64) - 0.927050983124841f64;
                            dz_ext1 = chain_77;
                            dz_ext0 = chain_77;
                            if ((c1 as i32) & 3) == 0 {
                                zsv_ext0 += 1;
                                dz_ext0 -= 1.0f64;
                            } else {
                                zsv_ext1 += 1;
                                dz_ext1 -= 1.0f64;
                            }
                        } else {
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            let chain_78 = dz0 - 0.927050983124841f64;
                            dz_ext1 = chain_78;
                            dz_ext0 = chain_78;
                        }
                        if ((c1 as i32) & 8) != 0 {
                            wsv_ext0 = wsb + 1;
                            wsv_ext1 = wsb + 2;
                            dw_ext0 = (dw0 - 1.0f64) - 0.927050983124841f64;
                            dw_ext1 = (dw0 - 2.0f64) - 0.927050983124841f64;
                        } else {
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            let chain_79 = dw0 - 0.927050983124841f64;
                            dw_ext1 = chain_79;
                            dw_ext0 = chain_79;
                        }
                        xsv_ext2 = xsb + 1;
                        ysv_ext2 = ysb + 1;
                        zsv_ext2 = zsb + 1;
                        wsv_ext2 = wsb + 1;
                        dx_ext2 = (dx0 - 1.0f64) - 0.618033988749894f64;
                        dy_ext2 = (dy0 - 1.0f64) - 0.618033988749894f64;
                        dz_ext2 = (dz0 - 1.0f64) - 0.618033988749894f64;
                        dw_ext2 = (dw0 - 1.0f64) - 0.618033988749894f64;
                        if ((c2 as i32) & 1) == 0 {
                            xsv_ext2 -= 2;
                            dx_ext2 += 2.0f64;
                        } else if ((c2 as i32) & 2) == 0 {
                            ysv_ext2 -= 2;
                            dy_ext2 += 2.0f64;
                        } else if ((c2 as i32) & 4) == 0 {
                            zsv_ext2 -= 2;
                            dz_ext2 += 2.0f64;
                        } else {
                            wsv_ext2 -= 2;
                            dw_ext2 += 2.0f64;
                        }
                    }
                    let dx4: f64 = (dx0 - 1.0f64) - 0.927050983124841f64;
                    let dy4: f64 = (dy0 - 1.0f64) - 0.927050983124841f64;
                    let dz4: f64 = (dz0 - 1.0f64) - 0.927050983124841f64;
                    let dw4: f64 = dw0 - 0.927050983124841f64;
                    let mut attn4: f64 = (((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4)) - (dw4 * dw4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 1, wsb + 0, dx4, dy4, dz4, dw4));
                    }
                    let dx3: f64 = dx4;
                    let dy3: f64 = dy4;
                    let dz3: f64 = dz0 - 0.927050983124841f64;
                    let dw3: f64 = (dw0 - 1.0f64) - 0.927050983124841f64;
                    let mut attn3: f64 = (((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3)) - (dw3 * dw3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 0, wsb + 1, dx3, dy3, dz3, dw3));
                    }
                    let dx2: f64 = dx4;
                    let dy2: f64 = dy0 - 0.927050983124841f64;
                    let dz2: f64 = dz4;
                    let dw2: f64 = dw3;
                    let mut attn2: f64 = (((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2)) - (dw2 * dw2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 1, wsb + 1, dx2, dy2, dz2, dw2));
                    }
                    let dx1: f64 = dx0 - 0.927050983124841f64;
                    let dz1: f64 = dz4;
                    let dy1: f64 = dy4;
                    let dw1: f64 = dw3;
                    let mut attn1: f64 = (((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1)) - (dw1 * dw1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 1, wsb + 1, dx1, dy1, dz1, dw1));
                    }
                    let dx5: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy5: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz5: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw5: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn5: f64 = (((2.0f64 - (dx5 * dx5)) - (dy5 * dy5)) - (dz5 * dz5)) - (dw5 * dw5);
                    if attn5 > 0.0f64 {
                        attn5 *= attn5;
                        value += (attn5 * attn5) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 0, wsb + 0, dx5, dy5, dz5, dw5));
                    }
                    let dx6: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy6: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz6: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw6: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn6: f64 = (((2.0f64 - (dx6 * dx6)) - (dy6 * dy6)) - (dz6 * dz6)) - (dw6 * dw6);
                    if attn6 > 0.0f64 {
                        attn6 *= attn6;
                        value += (attn6 * attn6) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 1, wsb + 0, dx6, dy6, dz6, dw6));
                    }
                    let dx7: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy7: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz7: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw7: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn7: f64 = (((2.0f64 - (dx7 * dx7)) - (dy7 * dy7)) - (dz7 * dz7)) - (dw7 * dw7);
                    if attn7 > 0.0f64 {
                        attn7 *= attn7;
                        value += (attn7 * attn7) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 0, wsb + 1, dx7, dy7, dz7, dw7));
                    }
                    let dx8: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy8: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz8: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw8: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn8: f64 = (((2.0f64 - (dx8 * dx8)) - (dy8 * dy8)) - (dz8 * dz8)) - (dw8 * dw8);
                    if attn8 > 0.0f64 {
                        attn8 *= attn8;
                        value += (attn8 * attn8) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 1, wsb + 0, dx8, dy8, dz8, dw8));
                    }
                    let dx9: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy9: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz9: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw9: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn9: f64 = (((2.0f64 - (dx9 * dx9)) - (dy9 * dy9)) - (dz9 * dz9)) - (dw9 * dw9);
                    if attn9 > 0.0f64 {
                        attn9 *= attn9;
                        value += (attn9 * attn9) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 0, wsb + 1, dx9, dy9, dz9, dw9));
                    }
                    let dx10: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy10: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz10: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw10: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn10: f64 = (((2.0f64 - (dx10 * dx10)) - (dy10 * dy10)) - (dz10 * dz10)) - (dw10 * dw10);
                    if attn10 > 0.0f64 {
                        attn10 *= attn10;
                        value += (attn10 * attn10) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 1, wsb + 1, dx10, dy10, dz10, dw10));
                    }
                }
                let mut attn_ext0: f64 = (((2.0f64 - (dx_ext0 * dx_ext0)) - (dy_ext0 * dy_ext0)) - (dz_ext0 * dz_ext0)) - (dw_ext0 * dw_ext0);
                if attn_ext0 > 0.0f64 {
                    attn_ext0 *= attn_ext0;
                    value += (attn_ext0 * attn_ext0) * (self.extrapolate_8(xsv_ext0, ysv_ext0, zsv_ext0, wsv_ext0, dx_ext0, dy_ext0, dz_ext0, dw_ext0));
                }
                let mut attn_ext1: f64 = (((2.0f64 - (dx_ext1 * dx_ext1)) - (dy_ext1 * dy_ext1)) - (dz_ext1 * dz_ext1)) - (dw_ext1 * dw_ext1);
                if attn_ext1 > 0.0f64 {
                    attn_ext1 *= attn_ext1;
                    value += (attn_ext1 * attn_ext1) * (self.extrapolate_8(xsv_ext1, ysv_ext1, zsv_ext1, wsv_ext1, dx_ext1, dy_ext1, dz_ext1, dw_ext1));
                }
                let mut attn_ext2: f64 = (((2.0f64 - (dx_ext2 * dx_ext2)) - (dy_ext2 * dy_ext2)) - (dz_ext2 * dz_ext2)) - (dw_ext2 * dw_ext2);
                if attn_ext2 > 0.0f64 {
                    attn_ext2 *= attn_ext2;
                    value += (attn_ext2 * attn_ext2) * (self.extrapolate_8(xsv_ext2, ysv_ext2, zsv_ext2, wsv_ext2, dx_ext2, dy_ext2, dz_ext2, dw_ext2));
                }
                return value / 30.0f64;
            }
            pub fn extrapolate_4(&self, xsb: i32, ysb: i32, dx: f64, dy: f64) -> f64 {
                let index: i32 = (((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(xsb & 0xFF) as usize]) as i32) + ysb) & 0xFF) as usize]) as i32) & 14;
                return ((((*open_simplex_noise::GRADIENTS2_D)[index as usize]) as f64) * dx) + ((((*open_simplex_noise::GRADIENTS2_D)[(index + 1) as usize]) as f64) * dy);
            }
            pub fn extrapolate_6(&self, xsb: i32, ysb: i32, zsb: i32, dx: f64, dy: f64, dz: f64) -> f64 {
                let index: i32 = (((self.perm_grad_index3_d.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(xsb & 0xFF) as usize]) as i32) + ysb) & 0xFF) as usize]) as i32) + zsb) & 0xFF) as usize]) as i32);
                return (((((*open_simplex_noise::GRADIENTS3_D)[index as usize]) as f64) * dx) + ((((*open_simplex_noise::GRADIENTS3_D)[(index + 1) as usize]) as f64) * dy)) + ((((*open_simplex_noise::GRADIENTS3_D)[(index + 2) as usize]) as f64) * dz);
            }
            pub fn extrapolate_8(&self, xsb: i32, ysb: i32, zsb: i32, wsb: i32, dx: f64, dy: f64, dz: f64, dw: f64) -> f64 {
                let index: i32 = (((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(xsb & 0xFF) as usize]) as i32) + ysb) & 0xFF) as usize]) as i32) + zsb) & 0xFF) as usize]) as i32) + wsb) & 0xFF) as usize]) as i32) & 252;
                return ((((((*open_simplex_noise::GRADIENTS4_D)[index as usize]) as f64) * dx) + ((((*open_simplex_noise::GRADIENTS4_D)[(index + 1) as usize]) as f64) * dy)) + ((((*open_simplex_noise::GRADIENTS4_D)[(index + 2) as usize]) as f64) * dz)) + ((((*open_simplex_noise::GRADIENTS4_D)[(index + 3) as usize]) as f64) * dw);
            }
        }
        pub fn fast_floor(x: f64) -> i32 {
            let xi: i32 = (x as i32);
            return if x < (xi as f64) { xi - 1 } else { xi };
        }
    }

    /// `FireflyParticle` (org/betterx/betterend/particle/FireflyParticle.java)
    pub mod firefly_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleAnimatedParticle`. Its static parts are below.
        // FireflyParticle.FireflyParticle(8): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
        // FireflyParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
    }

    /// `FireflyParticle.FireflyParticleFactory` (org/betterx/betterend/particle/FireflyParticle.java)
    pub mod firefly_particle_firefly_particle_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // FireflyParticle.FireflyParticleFactory.FireflyParticleFactory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // FireflyParticle.FireflyParticleFactory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `InfusionMistParticle` (org/betterx/betterend/particle/InfusionMistParticle.java)
    pub mod infusion_mist_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        pub const SINK_SPEED_MIN: f64 = 0.022f64;
        pub const SINK_SPEED_MAX: f64 = 0.048f64;
        pub const SWAY_STRENGTH: f64 = 0.0015f64;
        pub const SWAY_SPEED_MIN: f32 = 0.12f32;
        pub const SWAY_SPEED_MAX: f32 = 0.28f32;
        pub const MAX_FALL_DRIFT: f64 = 0.012f64;
        pub const GROUND_ACCELERATION: f64 = 0.0035f64;
        pub const MAX_GROUND_SPEED: f64 = 0.045f64;
        pub const GROUND_GROWTH: f32 = 1.025f32;
        pub const MAX_QUAD_SIZE: f32 = 0.42f32;
        pub const MAX_ALPHA: f32 = 0.85f32;
        pub const FADE_TICKS: i32 = 10;
        pub const DEFAULT_RGB: i32 = 16767370;
        pub const NORTH_RGB: i32 = 13081599;
        // InfusionMistParticle.InfusionMistParticle(9): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // InfusionMistParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // InfusionMistParticle.getLightCoords(1): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // InfusionMistParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `InfusionMistParticle.FactoryInfusionMist` (org/betterx/betterend/particle/InfusionMistParticle.java)
    pub mod infusion_mist_particle_factory_infusion_mist {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // InfusionMistParticle.FactoryInfusionMist.FactoryInfusionMist(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // InfusionMistParticle.FactoryInfusionMist.FactoryInfusionMist(2): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // InfusionMistParticle.FactoryInfusionMist.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `InfusionParticle` (org/betterx/betterend/particle/InfusionParticle.java)
    pub mod infusion_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        // InfusionParticle.InfusionParticle(9): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // InfusionParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // InfusionParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `InfusionParticle.InfusionFactory` (org/betterx/betterend/particle/InfusionParticle.java)
    pub mod infusion_particle_infusion_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // InfusionParticle.InfusionFactory.InfusionFactory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // InfusionParticle.InfusionFactory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `InfusionParticleType` (org/betterx/betterend/particle/InfusionParticleType.java)
    pub mod infusion_particle_type {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ParticleType`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // InfusionParticleType.InfusionParticleType(2): not translated, its class isn't translated (it extends Minecraft's `ParticleType`).
        // InfusionParticleType.InfusionParticleType(1): not translated, its class isn't translated (it extends Minecraft's `ParticleType`).
        // InfusionParticleType.getPalette(0): not translated, its class isn't translated (it extends Minecraft's `ParticleType`).
        // InfusionParticleType.getType(0): not translated, its class isn't translated (it extends Minecraft's `ParticleType`).
        // InfusionParticleType.codec(0): not translated, its class isn't translated (it extends Minecraft's `ParticleType`).
        // InfusionParticleType.streamCodec(0): not translated, its class isn't translated (it extends Minecraft's `ParticleType`).
    }

    /// `ParticleBlackSpore` (org/betterx/betterend/particle/ParticleBlackSpore.java)
    pub mod particle_black_spore {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleAnimatedParticle`. Its static parts are below.
        // ParticleBlackSpore.ParticleBlackSpore(8): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
        // ParticleBlackSpore.tick(0): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
    }

    /// `ParticleBlackSpore.FactoryBlackSpore` (org/betterx/betterend/particle/ParticleBlackSpore.java)
    pub mod particle_black_spore_factory_black_spore {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // ParticleBlackSpore.FactoryBlackSpore.FactoryBlackSpore(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // ParticleBlackSpore.FactoryBlackSpore.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `ParticleGeyser` (org/betterx/betterend/particle/ParticleGeyser.java)
    pub mod particle_geyser {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        // ParticleGeyser.ParticleGeyser(8): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleGeyser.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleGeyser.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `ParticleGeyser.FactoryGeyser` (org/betterx/betterend/particle/ParticleGeyser.java)
    pub mod particle_geyser_factory_geyser {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // ParticleGeyser.FactoryGeyser.FactoryGeyser(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // ParticleGeyser.FactoryGeyser.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `ParticleGlowingSphere` (org/betterx/betterend/particle/ParticleGlowingSphere.java)
    pub mod particle_glowing_sphere {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleAnimatedParticle`. Its static parts are below.
        // ParticleGlowingSphere.ParticleGlowingSphere(8): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
        // ParticleGlowingSphere.tick(0): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
    }

    /// `ParticleGlowingSphere.FactoryGlowingSphere` (org/betterx/betterend/particle/ParticleGlowingSphere.java)
    pub mod particle_glowing_sphere_factory_glowing_sphere {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // ParticleGlowingSphere.FactoryGlowingSphere.FactoryGlowingSphere(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // ParticleGlowingSphere.FactoryGlowingSphere.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `ParticleJungleSpore` (org/betterx/betterend/particle/ParticleJungleSpore.java)
    pub mod particle_jungle_spore {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleAnimatedParticle`. Its static parts are below.
        // ParticleJungleSpore.ParticleJungleSpore(8): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
        // ParticleJungleSpore.tick(0): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
    }

    /// `ParticleJungleSpore.FactoryJungleSpore` (org/betterx/betterend/particle/ParticleJungleSpore.java)
    pub mod particle_jungle_spore_factory_jungle_spore {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // ParticleJungleSpore.FactoryJungleSpore.FactoryJungleSpore(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // ParticleJungleSpore.FactoryJungleSpore.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `ParticleSnowflake` (org/betterx/betterend/particle/ParticleSnowflake.java)
    pub mod particle_snowflake {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        // ParticleSnowflake.ParticleSnowflake(8): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleSnowflake.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleSnowflake.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `ParticleSnowflake.FactorySnowflake` (org/betterx/betterend/particle/ParticleSnowflake.java)
    pub mod particle_snowflake_factory_snowflake {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // ParticleSnowflake.FactorySnowflake.FactorySnowflake(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // ParticleSnowflake.FactorySnowflake.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `ParticleSulphur` (org/betterx/betterend/particle/ParticleSulphur.java)
    pub mod particle_sulphur {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        // ParticleSulphur.ParticleSulphur(8): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleSulphur.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleSulphur.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `ParticleSulphur.FactorySulphur` (org/betterx/betterend/particle/ParticleSulphur.java)
    pub mod particle_sulphur_factory_sulphur {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // ParticleSulphur.FactorySulphur.FactorySulphur(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // ParticleSulphur.FactorySulphur.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `ParticleTenaneaPetal` (org/betterx/betterend/particle/ParticleTenaneaPetal.java)
    pub mod particle_tenanea_petal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        // provider: not translated, it uses Minecraft's `BlockTintSource`.
        // ParticleTenaneaPetal.ParticleTenaneaPetal(8): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleTenaneaPetal.getLightCoords(1): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleTenaneaPetal.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // ParticleTenaneaPetal.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `ParticleTenaneaPetal.FactoryTenaneaPetal` (org/betterx/betterend/particle/ParticleTenaneaPetal.java)
    pub mod particle_tenanea_petal_factory_tenanea_petal {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // ParticleTenaneaPetal.FactoryTenaneaPetal.FactoryTenaneaPetal(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // ParticleTenaneaPetal.FactoryTenaneaPetal.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `PaticlePortalSphere` (org/betterx/betterend/particle/PaticlePortalSphere.java)
    pub mod paticle_portal_sphere {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleAnimatedParticle`. Its static parts are below.
        // PaticlePortalSphere.PaticlePortalSphere(5): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
        // PaticlePortalSphere.tick(0): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
    }

    /// `PaticlePortalSphere.FactoryPortalSphere` (org/betterx/betterend/particle/PaticlePortalSphere.java)
    pub mod paticle_portal_sphere_factory_portal_sphere {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // PaticlePortalSphere.FactoryPortalSphere.FactoryPortalSphere(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // PaticlePortalSphere.FactoryPortalSphere.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `SmaragdantParticle` (org/betterx/betterend/particle/SmaragdantParticle.java)
    pub mod smaragdant_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleAnimatedParticle`. Its static parts are below.
        // SmaragdantParticle.SmaragdantParticle(8): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
        // SmaragdantParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `SimpleAnimatedParticle`).
    }

    /// `SmaragdantParticle.SmaragdantParticleFactory` (org/betterx/betterend/particle/SmaragdantParticle.java)
    pub mod smaragdant_particle_smaragdant_particle_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // SmaragdantParticle.SmaragdantParticleFactory.SmaragdantParticleFactory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // SmaragdantParticle.SmaragdantParticleFactory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `Patcher` (org/betterx/betterend/Patcher.java)
    pub mod patcher {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Patcher {
        }
        impl Patcher {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Patcher.register(0): not translated, a method reference.
    }

    /// `Patcher_001` (org/betterx/betterend/Patcher_001.java)
    pub mod patcher_001 {
        use super::*;
        // The class itself isn't translated: it extends `Patch` (org.betterx.bclib.api.v2.datafixer). Its static parts are below.
        // Patcher_001.Patcher_001(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_001.getIDReplacements(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_001.getChunkPatcher(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
    }

    /// `PortalBuilder` (org/betterx/betterend/portal/PortalBuilder.java)
    pub mod portal_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerLevel`. Its static parts are below.
        // FRAME_POSITIONS: not translated, it uses `Point` (java.awt).
        // PORTAL_POSITIONS: not translated, it uses `Point` (java.awt).
        // BASE_POSITIONS: not translated, it uses `Point` (java.awt).
        // BASE: not translated, it uses Minecraft's `Block`.
        // FRAME: not translated, it uses Minecraft's `Block`.
        // PORTAL: not translated, it uses Minecraft's `Block`.
        pub static SPIRAL_SEARCH_RADIUS: JCell<i32> = JCell::new(128);
        pub static GENERATE_SEARCH_RADIUS: JCell<i32> = JCell::new(16);
        pub const PROBE_FOOTPRINT: i32 = 4;
        // PortalBuilder.PortalBuilder(2): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // PortalBuilder.generatePortal(4): not translated, it uses Minecraft's `Level`.
        // PortalBuilder.generateBase(3): not translated, it uses Minecraft's `Level`.
        // PortalBuilder.checkIsAreaValid(3): not translated, it uses Minecraft's `Level`.
        // PortalBuilder.isBaseValid(3): not translated, it uses Minecraft's `Level`.
        // PortalBuilder.validBlock(3): not translated, it uses Minecraft's `Level`.
        // PortalBuilder.checkArea(3): not translated, it uses Minecraft's `Level`.
        // PortalBuilder.isStateInvalid(1): not translated, it uses Minecraft's `BlockState`.
        // PortalBuilder.findPortalAround(2): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // PortalBuilder.getPortalAt(1): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // PortalBuilder.getStartingPos(4): not translated, it uses Minecraft's `BlockPos`.
        // PortalBuilder.createPortal(3): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // PortalBuilder.buildPortal(3): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // PortalBuilder.getPortalID(1): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // PortalBuilder.isColumnLoaded(6): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // PortalBuilder.canPortalReplaceBlock(1): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // PortalBuilder.canHostFrame(4): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
    }

    /// `PortalBuilder.FoundPortalRect` (org/betterx/betterend/portal/PortalBuilder.java)
    pub mod portal_builder_found_portal_rect {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `FoundRectangle`. Its static parts are below.
        // PortalBuilder.FoundPortalRect.FoundPortalRect(2): not translated, its class isn't translated (it uses Minecraft's `FoundRectangle`).
        // PortalBuilder.FoundPortalRect.rect(0): not translated, its class isn't translated (it uses Minecraft's `FoundRectangle`).
        // PortalBuilder.FoundPortalRect.pos(0): not translated, its class isn't translated (it uses Minecraft's `FoundRectangle`).
    }

    /// `TravelerState` (org/betterx/betterend/portal/TravelerState.java)
    pub mod traveler_state {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TravelerState {
        }
        impl TravelerState {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `TravelingEntity` (org/betterx/betterend/portal/TravelingEntity.java)
    pub mod traveling_entity {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `InfusionRecipe` (org/betterx/betterend/recipe/builders/InfusionRecipe.java)
    pub mod infusion_recipe {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Recipe`. Its static parts are below.
        pub const GROUP: &str = "infusion";
        // INFUSION_CATEGORY: not translated, it uses Minecraft's `RecipeBookCategory`.
        // TYPE: not translated, it uses Minecraft's `RecipeType`.
        // SERIALIZER: not translated, it uses Minecraft's `RecipeSerializer`.
        // InfusionRecipe.InfusionRecipe(7): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.create(2): not translated, it uses Builder, which isn't translated (it's an interface).
        // InfusionRecipe.create(2): not translated, it uses Builder, which isn't translated (it's an interface).
        // InfusionRecipe.create(2): not translated, it uses Builder, which isn't translated (it's an interface).
        // InfusionRecipe.create(2): not translated, it uses Builder, which isn't translated (it's an interface).
        // InfusionRecipe.create(4): not translated, it uses Builder, which isn't translated (it's an interface).
        // InfusionRecipe.create(4): not translated, it uses Builder, which isn't translated (it's an interface).
        // InfusionRecipe.enchantmentPatch(3): not translated, it uses Minecraft's `DataComponentPatch`.
        // InfusionRecipe.getInfusionTime(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.getInput(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.getCatalysts(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.matches(2): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.assemble(1): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.showNotification(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.group(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.getSerializer(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.getType(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.placementInfo(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.recipeBookCategory(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        pub fn register() {
        }
    }

    /// `InfusionRecipe.Builder` (org/betterx/betterend/recipe/builders/InfusionRecipe.java)
    pub mod infusion_recipe_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // InfusionRecipe.Builder.group(1): not translated, it has no body.
        // InfusionRecipe.Builder.setPrimaryInput(1): not translated, it has no body.
        // InfusionRecipe.Builder.setPrimaryInput(1): not translated, it has no body.
        // InfusionRecipe.Builder.setPrimaryInputAndUnlock(1): not translated, it has no body.
        // InfusionRecipe.Builder.setPrimaryInputAndUnlock(1): not translated, it has no body.
        // InfusionRecipe.Builder.setTime(1): not translated, it has no body.
        // InfusionRecipe.Builder.addCatalyst(2): not translated, it has no body.
        // InfusionRecipe.Builder.addCatalyst(2): not translated, it has no body.
        // InfusionRecipe.Builder.addCatalyst(2): not translated, it has no body.
    }

    /// `InfusionRecipe.BuilderImpl` (org/betterx/betterend/recipe/builders/InfusionRecipe.java)
    pub mod infusion_recipe_builder_impl {
        use super::*;
        // The class itself isn't translated: it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes). Its static parts are below.
        // InfusionRecipe.BuilderImpl.BuilderImpl(2): not translated, its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)).
        // InfusionRecipe.BuilderImpl.BuilderImpl(2): not translated, its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)).
        // InfusionRecipe.BuilderImpl.setTime(1): not translated, its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)).
        // InfusionRecipe.BuilderImpl.addCatalyst(2): not translated, its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)).
        // InfusionRecipe.BuilderImpl.addCatalyst(2): not translated, its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)).
        // InfusionRecipe.BuilderImpl.addCatalyst(2): not translated, its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)).
        // InfusionRecipe.BuilderImpl.validate(0): not translated, its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)).
        // InfusionRecipe.BuilderImpl.createRecipe(1): not translated, its class isn't translated (it extends `BCLBaseRecipeBuilder` (org.betterx.bclib.recipes)).
    }

    /// `InfusionRecipe.CatalystSlot` (org/betterx/betterend/recipe/builders/InfusionRecipe.java)
    pub mod infusion_recipe_catalyst_slot {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // InfusionRecipe.CatalystSlot.CatalystSlot(2): not translated, its class isn't translated (its constants carry values).
        // InfusionRecipe.CatalystSlot.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `InfusionRecipe.Serializer` (org/betterx/betterend/recipe/builders/InfusionRecipe.java)
    pub mod infusion_recipe_serializer {
        use super::*;
        // CODEC_CATALYSTS: not translated, it uses Minecraft's `MapCodec`.
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
        // InfusionRecipe.Serializer.fromNetwork(1): not translated, it uses InfusionRecipe, which isn't translated (it implements Minecraft's `Recipe`).
        // InfusionRecipe.Serializer.toNetwork(2): not translated, it uses Minecraft's `RegistryFriendlyByteBuf`.
    }

    /// `InfusionRecipe.Serializer.Result` (org/betterx/betterend/recipe/builders/InfusionRecipe.java)
    pub mod infusion_recipe_serializer_result {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Item`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // InfusionRecipe.Serializer.Result.Result(3): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // InfusionRecipe.Serializer.Result.item(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // InfusionRecipe.Serializer.Result.count(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // InfusionRecipe.Serializer.Result.patch(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
    }

    /// `EndBlockItemFactories` (org/betterx/betterend/registry/block/EndBlockItemFactories.java)
    pub mod end_block_item_factories {
        use super::*;
        // FIRE_RESISTANT_ANVIL: not translated, it uses `BlockItemFactory` (de.ambertation.wover.block.api.BlockDefinition).
        // FIRE_RESISTANT: not translated, it uses `BlockItemFactory` (de.ambertation.wover.block.api.BlockDefinition).
        // PLACE_ON_WATER: not translated, it uses `BlockItemFactory` (de.ambertation.wover.block.api.BlockDefinition).
        #[derive(Clone, Debug)]
        pub struct EndBlockItemFactories {
        }
        impl EndBlockItemFactories {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndCropBlocks` (org/betterx/betterend/registry/block/EndCropBlocks.java)
    pub mod end_crop_blocks {
        use super::*;
        // SHADOW_BERRY: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLOSSOM_BERRY: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMBER_ROOT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHORUS_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CAVE_PUMPKIN_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CAVE_PUMPKIN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndCropBlocks {
        }
        impl EndCropBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndCrystalBlocks` (org/betterx/betterend/registry/block/EndCrystalBlocks.java)
    pub mod end_crystal_blocks {
        use super::*;
        // AURORA_PALETTE: not translated, it uses `PaletteCycle` (de.ambertation.wover.block.impl.classify.TintTrait).
        // AURORA_CRYSTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SMARAGDANT_CRYSTAL_SHARD: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SMARAGDANT_CRYSTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SMARAGDANT_SUBBLOCKS: not translated, it uses CrystalSubblocksMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // BUDDING_SMARAGDANT_CRYSTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndCrystalBlocks {
        }
        impl EndCrystalBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndDecorBlocks` (org/betterx/betterend/registry/block/EndDecorBlocks.java)
    pub mod end_decor_blocks {
        use super::*;
        // DENSE_SNOW: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // EMERALD_ICE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // DENSE_EMERALD_ICE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ANCIENT_EMERALD_ICE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NEON_CACTUS_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NEON_CACTUS_BLOCK_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NEON_CACTUS_BLOCK_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MENGER_SPONGE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MENGER_SPONGE_WET: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_PETAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_PETAL_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_PETAL_COLORED: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_PETAL_BLOCK_COLORED: not translated, it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        #[derive(Clone, Debug)]
        pub struct EndDecorBlocks {
        }
        impl EndDecorBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndFactories` (org/betterx/betterend/registry/block/EndFactories.java)
    pub mod end_factories {
        use super::*;
        // AETERNIUM_ANVIL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // AETERNIUM_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // AMARANITA_CAP_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // AMARANITA_HYMENOPHORE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // AMARANITA_STEM_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // AMBER_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // ANCIENT_EMERALD_ICE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // AURORA_CRYSTAL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLUE_VINE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLUE_VINE_LANTERN_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLUE_VINE_SEED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BOLUX_MUSHROOM_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BRIMSTONE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BUBBLE_CORAL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BUDDING_SMARAGDANT_CRYSTAL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BULB_VINE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BULB_VINE_LANTERN_COLORED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BULB_VINE_SEED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CAVE_PUMPKIN_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CAVE_PUMPKIN_VINE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CHARCOAL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CHARNIA_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CRYSTAL_MOSS_COVER_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // DRAGON_TREE_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // EMERALD_ICE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // ENDER_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // ENDSTONE_DUST_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_LILY_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_LILY_SEED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_LOTUS_FLOWER_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_LOTUS_LEAF_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_LOTUS_SEED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_LOTUS_STEM_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_PORTAL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_STONE_SMELTER: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_UNDERWATER_WALL_PLANT_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // ETERNAL_PEDESTAL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FILALUX_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FILALUX_LANTERN_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FILALUX_WINGS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FLAMAEA_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FLAMMALIX_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FUR_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOWING_HYMENOPHORE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOWING_MOSS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOWING_PILLAR_LUMINOPHOR_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOWING_PILLAR_ROOTS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOWING_PILLAR_SEED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // HELIX_TREE_LEAVES_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // HELIX_TREE_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_PETAL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_PETAL_COLORED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // HYDROTHERMAL_VENT_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // INFUSION_PEDESTAL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // JELLYSHROOM_CAP_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LACUGROVE_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LANCELEAF_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LANCELEAF_SEED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LARGE_AMARANITA_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LUCERNIA_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LUMECORN_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LUMECORN_SEED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MENGER_SPONGE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MENGER_SPONGE_WET_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MOSSY_DRAGON_BONE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MOSSY_GLOWSHROOM_CAP_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MOSSY_GLOWSHROOM_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MOSSY_OBSIDIAN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MURKWEED_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NEEDLEGRASS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NEON_CACTUS_PLANT_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // POND_ANEMONE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // PYTHADENDRON_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RESPAWN_OBELISK_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // ROTATED_PILLAR_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RUNED_FLAVOLITE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SHADOW_BERRY_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SHADOW_GRASS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SILK_MOTH_HIVE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SILK_MOTH_NEST_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SMALL_AMARANITA_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SMALL_JELLYSHROOM_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SMARAGDANT_CRYSTAL_SHARD_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SULPHUR_CRYSTAL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // TENANEA_FLOWERS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // TENANEA_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // TWISTED_UMBRELLA_MOSS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // TWISTED_UMBRELLA_MOSS_TALL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_MOSS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_MOSS_TALL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_TREE_CLUSTER_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_TREE_CLUSTER_EMPTY_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_TREE_MEMBRANE_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_TREE_SAPLING_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // VENT_BUBBLE_COLUMN_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_TERRAIN_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CAVE_MOSS_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CHORUS_MUSHROOM_CROP_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLOSSOM_BERRY_CROP_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // AMBER_ROOT_CROP_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM_TINY_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM_THIN_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM_HEAVY_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM_FULL_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // END_PATH_BLOCK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndFactories {
        }
        impl EndFactories {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndFunctionalBlocks` (org/betterx/betterend/registry/block/EndFunctionalBlocks.java)
    pub mod end_functional_blocks {
        use super::*;
        // SILK_MOTH_NEST: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SILK_MOTH_HIVE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // RESPAWN_OBELISK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_STONE_SMELTER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ETERNAL_PEDESTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // INFUSION_PEDESTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AETERNIUM_ANVIL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_PORTAL_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndFunctionalBlocks {
        }
        impl EndFunctionalBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndKinds` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds {
        use super::*;
        // OWN_PEDESTAL_MODEL: not translated, a raw Supplier.
        // PEDESTAL: not translated, it uses PedestalKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // STONE_LANTERN: not translated, it uses StoneLanternKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // FURNACE: not translated, it uses FurnaceKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // FLOWER_POT: not translated, it uses FlowerPotKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // METAL_BLOCK: not translated, it uses MetalBlockKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // TILE: not translated, it uses TileKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // ORE: not translated, it uses OreKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // CHANDELIER: not translated, it uses ChandelierKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // BULB_LANTERN: not translated, it uses BulbLanternKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // ANVIL: not translated, it uses AnvilKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        #[derive(Clone, Debug)]
        pub struct EndKinds {
        }
        impl EndKinds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndKinds.optionalBlock(1): not translated, it uses `InputKey` (de.ambertation.wover.block.api.classify).
        // EndKinds.optionalItem(1): not translated, it uses `InputKey` (de.ambertation.wover.block.api.classify).
        // EndKinds.physical(1): not translated, it uses Builder, which isn't translated (it's an interface).
        // EndKinds.light(2): not translated, it uses `ResolveContext` (de.ambertation.wover.block.api.classify).
    }

    /// `EndKinds.AnvilKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_anvil_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.AnvilKind.AnvilKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.AnvilKind.AnvilKind(7): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.AnvilKind.createItem(3): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.AnvilKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.BulbLanternKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_bulb_lantern_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.BulbLanternKind.BulbLanternKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.BulbLanternKind.BulbLanternKind(7): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.BulbLanternKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.ChandelierKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_chandelier_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.ChandelierKind.ChandelierKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.ChandelierKind.ChandelierKind(4): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.ChandelierKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.FlowerPotKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_flower_pot_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.FlowerPotKind.FlowerPotKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.FlowerPotKind.FlowerPotKind(4): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.FlowerPotKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.FurnaceKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_furnace_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.FurnaceKind.FurnaceKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.FurnaceKind.FurnaceKind(4): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.FurnaceKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.MetalBlockKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_metal_block_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.MetalBlockKind.MetalBlockKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.MetalBlockKind.MetalBlockKind(4): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.MetalBlockKind.slotName(2): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.MetalBlockKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.OreKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_ore_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.OreKind.OreKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.OreKind.OreKind(8): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.OreKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.PedestalKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_pedestal_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.PedestalKind.PedestalKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.PedestalKind.PedestalKind(7): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.PedestalKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.StoneLanternKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_stone_lantern_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.StoneLanternKind.StoneLanternKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.StoneLanternKind.StoneLanternKind(4): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.StoneLanternKind.slotName(2): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.StoneLanternKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndKinds.TileKind` (org/betterx/betterend/registry/block/EndKinds.java)
    pub mod end_kinds_tile_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndKinds.TileKind.TileKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.TileKind.TileKind(4): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // EndKinds.TileKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndLightBlocks` (org/betterx/betterend/registry/block/EndLightBlocks.java)
    pub mod end_light_blocks {
        use super::*;
        // BLUE_VINE_LANTERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOWING_PILLAR_LUMINOPHOR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMARANITA_LANTERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FILALUX_LANTERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // IRON_BULB_LANTERN_COLORED: not translated, it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        #[derive(Clone, Debug)]
        pub struct EndLightBlocks {
        }
        impl EndLightBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndLoot` (org/betterx/betterend/registry/block/EndLoot.java)
    pub mod end_loot {
        use super::*;
        // BULB_VINE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // CAVE_PUMPKIN: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // CRYSTAL_SHARDS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // EMPTY: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // END_LILY: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // END_LOTUS_FLOWER: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // HYDRALUX: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // LANCELEAF: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // LUMECORN: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // NEEDLEGRASS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // NONE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // POTTABLE_CROP: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // RESPAWN_OBELISK: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // SILK_MOTH_NEST: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // SULPHUR_CRYSTAL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        #[derive(Clone, Debug)]
        pub struct EndLoot {
        }
        impl EndLoot {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndMaterials` (org/betterx/betterend/registry/block/EndMaterials.java)
    pub mod end_materials {
        use super::*;
        // WOOD: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // FUR: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NONFLAMMABLE_PLANT: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // SHEARED_PLANT: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // TERRAIN_PATH: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CRYSTAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // ENDSTONE_DUST: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CAVE_PUMPKIN: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // DENSE_SNOW: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // EMERALD_ICE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // DENSE_EMERALD_ICE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // ANCIENT_EMERALD_ICE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // MENGER_SPONGE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // MENGER_SPONGE_WET: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NEON_CACTUS_BLOCK: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NEON_CACTUS: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // END_PORTAL_BLOCK: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // VENT_BUBBLE_COLUMN: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // ETERNAL_PEDESTAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // INFUSION_PEDESTAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // END_STONE_STALACTITE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // END_STONE_STALACTITE_CAVEMOSS: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndMaterials {
        }
        impl EndMaterials {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndMaterials.ice(2): not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // EndMaterials.sponge(2): not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
    }

    /// `EndMetalBlocks` (org/betterx/betterend/registry/block/EndMetalBlocks.java)
    pub mod end_metal_blocks {
        use super::*;
        // IRON_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // GOLD_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // TERMINITE: not translated, it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // THALLASIUM: not translated, it uses MetalMaterial, which isn't translated (it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify))).
        // AETERNIUM_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHARCOAL_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ENDER_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMBER_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndMetalBlocks {
        }
        impl EndMetalBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndMushroomBlocks` (org/betterx/betterend/registry/block/EndMushroomBlocks.java)
    pub mod end_mushroom_blocks {
        use super::*;
        // GLOWING_PILLAR_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOWING_PILLAR_ROOTS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOWING_PILLAR_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SMALL_JELLYSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BOLUX_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUMECORN_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUMECORN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SMALL_AMARANITA_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LARGE_AMARANITA_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMARANITA_STEM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMARANITA_HYPHAE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMARANITA_HYMENOPHORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMARANITA_FUR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMARANITA_CAP: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndMushroomBlocks {
        }
        impl EndMushroomBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndOreBlocks` (org/betterx/betterend/registry/block/EndOreBlocks.java)
    pub mod end_ore_blocks {
        use super::*;
        // ENDER_ORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMBER_ORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndOreBlocks {
        }
        impl EndOreBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndPlantBlocks` (org/betterx/betterend/registry/block/EndPlantBlocks.java)
    pub mod end_plant_blocks {
        use super::*;
        // UMBRELLA_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_MOSS_TALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CREEPING_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHORUS_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CAVE_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CRYSTAL_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SHADOW_PLANT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BUSHY_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMBER_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TWISTED_UMBRELLA_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TWISTED_UMBRELLA_MOSS_TALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JUNGLE_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLOOMING_COOKSONIA: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SALTEAGO: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // VAIOLUSH_FERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FRACTURN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CLAWFERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOBULAGUS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ORANGO: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AERIDIUM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUTEBUS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LAMELLARIUM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // INFLEXIA: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FLAMMALIX: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CRYSTAL_MOSS_COVER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LANCELEAF_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LANCELEAF: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NEON_CACTUS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CAVE_BUSH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MURKWEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NEEDLEGRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndPlantBlocks {
        }
        impl EndPlantBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndRecipes` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes {
        use super::*;
        // PEDESTAL: not translated, it uses `Template3` (de.ambertation.wover.core.api.template).
        // STONE_LANTERN: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // FURNACE: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // FLOWER_POT: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // METAL_BLOCK: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // TILE: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // CHANDELIER: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // BULB_LANTERN: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        // ANVIL: not translated, it uses `Template3` (de.ambertation.wover.core.api.template).
        // COLORED_WITH_DYE: not translated, it uses `Template3` (de.ambertation.wover.core.api.template).
        // CRYSTAL_SUBBLOCK: not translated, it uses `Template4` (de.ambertation.wover.core.api.template).
        // JELLYSHROOM_HANGING_SIGN: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        #[derive(Clone, Debug)]
        pub struct EndRecipes {
        }
        impl EndRecipes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
        // EndRecipes.pedestal(3): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.stoneLantern(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.furnace(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.flowerPot(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.metalBlock(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.tile(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.chandelier(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.bulbLantern(2): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.anvil(3): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.coloredWithDye(3): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.crystalSubblock(4): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // EndRecipes.jellyshroomHangingSign(2): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
    }

    /// `EndRecipes.Anvil` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_anvil {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.Anvil.Anvil(3): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.Anvil.base(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.Anvil.tile(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.Anvil.ingot(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.BulbLantern` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_bulb_lantern {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.BulbLantern.BulbLantern(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.BulbLantern.chain(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.BulbLantern.ingot(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.Chandelier` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_chandelier {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemLike`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.Chandelier.Chandelier(1): not translated, its class isn't translated (it uses Minecraft's `ItemLike`).
        // EndRecipes.Chandelier.ingot(0): not translated, its class isn't translated (it uses Minecraft's `ItemLike`).
    }

    /// `EndRecipes.ColoredWithDye` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_colored_with_dye {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.ColoredWithDye.ColoredWithDye(3): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.ColoredWithDye.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.ColoredWithDye.dye(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.ColoredWithDye.craftEight(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.CrystalSubblock` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_crystal_subblock {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.CrystalSubblock.CrystalSubblock(4): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.CrystalSubblock.ingredient(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.CrystalSubblock.count(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.CrystalSubblock.group(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.CrystalSubblock.shape(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.FlowerPot` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_flower_pot {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.FlowerPot.FlowerPot(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.FlowerPot.brick(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.Furnace` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_furnace {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.Furnace.Furnace(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.Furnace.base(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.JellyshroomHangingSign` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_jellyshroom_hanging_sign {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.JellyshroomHangingSign.JellyshroomHangingSign(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.JellyshroomHangingSign.strippedLog(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.JellyshroomHangingSign.log(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.MetalBlock` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_metal_block {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemLike`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.MetalBlock.MetalBlock(1): not translated, its class isn't translated (it uses Minecraft's `ItemLike`).
        // EndRecipes.MetalBlock.ingot(0): not translated, its class isn't translated (it uses Minecraft's `ItemLike`).
    }

    /// `EndRecipes.Pedestal` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_pedestal {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.Pedestal.Pedestal(3): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.Pedestal.slab(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.Pedestal.pillar(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.Pedestal.group(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.StoneLantern` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_stone_lantern {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.StoneLantern.StoneLantern(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.StoneLantern.slab(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndRecipes.Tile` (org/betterx/betterend/registry/block/EndRecipes.java)
    pub mod end_recipes_tile {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndRecipes.Tile.Tile(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // EndRecipes.Tile.base(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `EndSaplingBlocks` (org/betterx/betterend/registry/block/EndSaplingBlocks.java)
    pub mod end_sapling_blocks {
        use super::*;
        // MOSSY_GLOWSHROOM_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PYTHADENDRON_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LACUGROVE_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // DRAGON_TREE_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TENANEA_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // HELIX_TREE_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_TREE_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUCERNIA_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // HYDRALUX_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndSaplingBlocks {
        }
        impl EndSaplingBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndStoneBlocks` (org/betterx/betterend/registry/block/EndStoneBlocks.java)
    pub mod end_stone_blocks {
        use super::*;
        // MOSSY_OBSIDIAN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // DRAGON_BONE_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // DRAGON_BONE_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // DRAGON_BONE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MOSSY_DRAGON_BONE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FLAVOLITE: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // VIOLECITE: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // SULPHURIC_ROCK: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // VIRID_JADESTONE: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // AZURE_JADESTONE: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // SANDY_JADESTONE: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // UMBRALITH: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // BRIMSTONE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SULPHUR_CRYSTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MISSING_TILE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_STONE_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // ANDESITE_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // DIORITE_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // GRANITE_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // QUARTZ_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // PURPUR_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // BLACKSTONE_SET: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // FLAVOLITE_RUNED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FLAVOLITE_RUNED_ETERNAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // HYDROTHERMAL_VENT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // VENT_BUBBLE_COLUMN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_STONE_STALACTITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_STONE_STALACTITE_CAVEMOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_STONE_BRICK_VARIATIONS: not translated, it uses `BlockSet` (de.ambertation.wover.sets.api.classify).
        // END_STONE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_STONE_STAIR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_STONE_WALLS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndStoneBlocks {
        }
        impl EndStoneBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndTerrainBlocks` (org/betterx/betterend/registry/block/EndTerrainBlocks.java)
    pub mod end_terrain_blocks {
        use super::*;
        // ENDSTONE_DUST: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_MYCELIUM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHORUS_NYLIUM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CAVE_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CRYSTAL_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SHADOW_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PINK_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMBER_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JUNGLE_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SANGNUM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // RUTISCUS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM_FULL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM_HEAVY: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM_THIN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PALLIDIUM_TINY: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_MYCELIUM_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_MOSS_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHORUS_NYLIUM_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CAVE_MOSS_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CRYSTAL_MOSS_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SHADOW_GRASS_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PINK_MOSS_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AMBER_MOSS_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JUNGLE_MOSS_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SANGNUM_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // RUTISCUS_PATH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndTerrainBlocks {
        }
        impl EndTerrainBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndTints` (org/betterx/betterend/registry/block/EndTints.java)
    pub mod end_tints {
        use super::*;
        // AURORA_PALETTE: not translated, it uses `PaletteCycle` (de.ambertation.wover.block.impl.classify.TintTrait).
        #[derive(Clone, Debug)]
        pub struct EndTints {
        }
        impl EndTints {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndVineBlocks` (org/betterx/betterend/registry/block/EndVineBlocks.java)
    pub mod end_vine_blocks {
        use super::*;
        // BLUE_VINE_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_VINE_FUR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // DENSE_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TWISTED_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BULB_VINE_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BULB_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JUNGLE_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // RUBINEA: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MAGNULA: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FILALUX: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FILALUX_WINGS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndVineBlocks {
        }
        impl EndVineBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndWallPlantBlocks` (org/betterx/betterend/registry/block/EndWallPlantBlocks.java)
    pub mod end_wall_plant_blocks {
        use super::*;
        // PURPLE_POLYPORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AURANT_POLYPORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TAIL_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CYAN_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TWISTED_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TUBE_WORM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BULB_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JUNGLE_FERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // RUSCUS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndWallPlantBlocks {
        }
        impl EndWallPlantBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndWaterPlantBlocks` (org/betterx/betterend/registry/block/EndWaterPlantBlocks.java)
    pub mod end_water_plant_blocks {
        use super::*;
        // END_LOTUS_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_LOTUS_LEAF: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_LOTUS_FLOWER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BUBBLE_CORAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHARNIA_RED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHARNIA_PURPLE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHARNIA_ORANGE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHARNIA_LIGHT_BLUE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHARNIA_CYAN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHARNIA_GREEN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_LILY: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_LILY_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // HYDRALUX: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // POND_ANEMONE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FLAMAEA: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndWaterPlantBlocks {
        }
        impl EndWaterPlantBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndWoodBlocks` (org/betterx/betterend/registry/block/EndWoodBlocks.java)
    pub mod end_wood_blocks {
        use super::*;
        // MOSSY_GLOWSHROOM_CAP: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MOSSY_GLOWSHROOM_HYMENOPHORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MOSSY_GLOWSHROOM_FUR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MOSSY_GLOWSHROOM: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // PYTHADENDRON_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PYTHADENDRON: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // END_LOTUS_STEM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // END_LOTUS: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // LACUGROVE_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LACUGROVE: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // DRAGON_TREE_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // DRAGON_TREE: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // TENANEA_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TENANEA_FLOWERS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TENANEA_OUTER_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TENANEA: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // HELIX_TREE_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // HELIX_TREE: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // UMBRELLA_TREE_MEMBRANE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_TREE_CLUSTER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_TREE_CLUSTER_EMPTY: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // UMBRELLA_TREE: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // JELLYSHROOM_CAP_PURPLE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JELLYSHROOM: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // LUCERNIA_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUCERNIA_OUTER_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUCERNIA: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        // LUCERNIA_JELLY: not translated, it uses EndWoodenComplexMaterial, which isn't translated (it uses Minecraft's `MapColor`).
        #[derive(Clone, Debug)]
        pub struct EndWoodBlocks {
        }
        impl EndWoodBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `ExtraTagsTrait` (org/betterx/betterend/registry/block/ExtraTagsTrait.java)
    pub mod extra_tags_trait {
        use super::*;
        // The class itself isn't translated: it extends `BlockTrait` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // TYPE: not translated, it uses `TraitType` (de.ambertation.wover.block.api.classify).
        // ExtraTagsTrait.ExtraTagsTrait(2): not translated, its class isn't translated (it extends `BlockTrait` (de.ambertation.wover.block.api.classify)).
        // ExtraTagsTrait.type(0): not translated, its class isn't translated (it extends `BlockTrait` (de.ambertation.wover.block.api.classify)).
        // ExtraTagsTrait.datagen(1): not translated, its class isn't translated (it extends `BlockTrait` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndAttributes` (org/betterx/betterend/registry/EndAttributes.java)
    pub mod end_attributes {
        use super::*;
        // BLINDNESS_RESISTANCE: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct EndAttributes {
        }
        impl EndAttributes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndAttributes.registerAttribute(3): not translated, it uses Minecraft's `Holder`.
        // EndAttributes.addLivingEntityAttributes(1): not translated, it uses Builder, which isn't translated (it's an interface).
    }

    /// `EndBiomeBuildings` (org/betterx/betterend/registry/EndBiomeBuildings.java)
    pub mod end_biome_buildings {
        use super::*;
        // ALL: not translated, it uses Building, which isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        #[derive(Clone, Debug)]
        pub struct EndBiomeBuildings {
        }
        impl EndBiomeBuildings {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndBiomeBuildings.variants(6): not translated, it uses Builder, which isn't translated (it's an interface).
        // EndBiomeBuildings.single(5): not translated, it uses Builder, which isn't translated (it's an interface).
        // EndBiomeBuildings.add(7): not translated, it uses Builder, which isn't translated (it's an interface).
        // EndBiomeBuildings.templateCount(1): not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // EndBiomeBuildings.all(0): not translated, it uses Building, which isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.of(1): not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        pub fn register() {
        }
    }

    /// `EndBiomeBuildings.Building` (org/betterx/betterend/registry/EndBiomeBuildings.java)
    pub mod end_biome_buildings_building {
        use super::*;
        // The class itself isn't translated: it uses EndBiomeKey, which isn't translated (it's generic). Its static parts are below.
        // EndBiomeBuildings.Building.Building(8): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.templateIds(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.biome(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.name(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.templates(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.offsetY(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.adjustment(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.mound(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.pool(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
        // EndBiomeBuildings.Building.structure(0): not translated, its class isn't translated (it uses EndBiomeKey, which isn't translated (it's generic)).
    }

    /// `EndBiomes` (org/betterx/betterend/registry/EndBiomes.java)
    pub mod end_biomes {
        use super::*;
        // AMBER_LAND: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // BLOSSOMING_SPIRES: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // CHORUS_FOREST: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // CRYSTAL_MOUNTAINS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // DRAGON_GRAVEYARDS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // DRY_SHRUBLAND: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // DUST_WASTELANDS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // FOGGY_MUSHROOMLAND: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // GLOWING_GRASSLANDS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // ICE_STARFIELD: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // FLOWER_ISLETS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // WATERFALL_PONDS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // LANTERN_WOODS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // MEGALAKE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // SHADOW_FOREST: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // SULPHUR_SPRINGS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // UMBRELLA_JUNGLE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // UMBRA_VALLEY: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // MEGALAKE_GROVE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // NEON_OASIS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // PAINTED_MOUNTAINS: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // EMPTY_END_CAVE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // EMPTY_SMARAGDANT_CAVE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // LUSH_SMARAGDANT_CAVE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // EMPTY_AURORA_CAVE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // LUSH_AURORA_CAVE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // JADE_CAVE: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct EndBiomes {
        }
        impl EndBiomes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndBiomes.register(0): not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
    }

    /// `EndBlockEntities` (org/betterx/betterend/registry/EndBlockEntities.java)
    pub mod end_block_entities {
        use super::*;
        // END_STONE_SMELTER: not translated, it uses Minecraft's `BlockEntityType`.
        // PEDESTAL: not translated, it uses Minecraft's `BlockEntityType`.
        // ETERNAL_PEDESTAL: not translated, it uses Minecraft's `BlockEntityType`.
        // INFUSION_PEDESTAL: not translated, it uses Minecraft's `BlockEntityType`.
        // HYDROTHERMAL_VENT: not translated, it uses Minecraft's `BlockEntityType`.
        // FLOWER_POT: not translated, it uses Minecraft's `BlockEntityType`.
        #[derive(Clone, Debug)]
        pub struct EndBlockEntities {
        }
        impl EndBlockEntities {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndBlockEntities.registerBlockEntity(2): not translated, it's generic.
        pub fn register() {
        }
        // EndBlockEntities.addLateRegisteredBlocks(0): not translated, it uses Minecraft's `Block`.
        // EndBlockEntities.getPedestals(0): not translated, it uses Minecraft's `Block`.
        // EndBlockEntities.getFlowerPots(0): not translated, it uses Minecraft's `Block`.
    }

    /// `EndBlockEntityRenders` (org/betterx/betterend/registry/EndBlockEntityRenders.java)
    pub mod end_block_entity_renders {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndBlockEntityRenders {
        }
        impl EndBlockEntityRenders {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndBlockEntityRenders.register(0): not translated, it uses Minecraft's `BlockEntityRendererProvider`.
    }

    /// `EndBlocks` (org/betterx/betterend/registry/EndBlocks.java)
    pub mod end_blocks {
        use super::*;
        // BLOCKS_REGISTRY: not translated, it uses `BlockRegistry` (de.ambertation.wover.block.api).
        #[derive(Clone, Debug)]
        pub struct EndBlocks {
        }
        impl EndBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndBlocks.getModBlocks(0): not translated, it uses Minecraft's `Block`.
        // EndBlocks.key(2): not translated, it's generic.
        // EndBlocks.getBlockRegistry(0): not translated, it uses `BlockRegistry` (de.ambertation.wover.block.api).
        pub fn ensure_statically_loaded() {
            end_terrain_blocks::ensure_loaded();
            end_stone_blocks::ensure_loaded();
            end_metal_blocks::ensure_loaded();
            end_ore_blocks::ensure_loaded();
            end_sapling_blocks::ensure_loaded();
            end_wood_blocks::ensure_loaded();
            end_plant_blocks::ensure_loaded();
            end_water_plant_blocks::ensure_loaded();
            end_wall_plant_blocks::ensure_loaded();
            end_crop_blocks::ensure_loaded();
            end_vine_blocks::ensure_loaded();
            end_mushroom_blocks::ensure_loaded();
            end_light_blocks::ensure_loaded();
            end_crystal_blocks::ensure_loaded();
            end_functional_blocks::ensure_loaded();
            end_decor_blocks::ensure_loaded();
        }
    }

    /// `EndCarvers` (org/betterx/betterend/registry/EndCarvers.java)
    pub mod end_carvers {
        use super::*;
        // END_ROUND_CAVE: not translated, it uses Minecraft's `MapCodec`.
        // END_TUNNEL_CAVE: not translated, it uses Minecraft's `MapCodec`.
        // ROUND_CAVE: not translated, it uses Minecraft's `ResourceKey`.
        // TUNNEL_CAVE: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct EndCarvers {
        }
        impl EndCarvers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndCarvers.registerType(2): not translated, it's generic.
        // EndCarvers.carverKey(1): not translated, it uses Minecraft's `ResourceKey`.
        pub fn ensure_statically_loaded() {
        }
    }

    /// `EndClassification` (org/betterx/betterend/registry/EndClassification.java)
    pub mod end_classification {
        use super::*;
        // The class itself isn't translated: it implements `WoverBlocksEntrypoint` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndClassification.registerClassification(0): not translated, its class isn't translated (it implements `WoverBlocksEntrypoint` (de.ambertation.wover.block.api.classify)).
        // EndClassification.init(1): not translated, it uses `Class`.
    }

    /// `EndEnchantments` (org/betterx/betterend/registry/EndEnchantments.java)
    pub mod end_enchantments {
        use super::*;
        // END_VEIL: not translated, it uses `EnchantmentKey` (de.ambertation.wover.enchantment.api).
        // RESONANCE: not translated, it uses `EnchantmentKey` (de.ambertation.wover.enchantment.api).
        // END_VEIL_STATE: not translated, it uses Minecraft's `DataComponentType`.
        #[derive(Clone, Debug)]
        pub struct EndEnchantments {
        }
        impl EndEnchantments {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_statically_loaded() {
            end_status_effects::ensure_statically_loaded();
        }
    }

    /// `EndEntities` (org/betterx/betterend/registry/EndEntities.java)
    pub mod end_entities {
        use super::*;
        // SPAWN_EGGS: not translated, it uses SpawnEgg, which isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)).
        // DRAGONFLY: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // END_SLIME: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // END_FISH: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // SHADOW_WALKER: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // CUBOZOA: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // SILK_MOTH: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        #[derive(Clone, Debug)]
        pub struct EndEntities {
        }
        impl EndEntities {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndEntities.spawnEggs(0): not translated, it uses SpawnEgg, which isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)).
        // EndEntities.register(0): not translated, it uses EndEntities.DRAGONFLY, which isn't translated (it uses `BCLEntityWrapper` (org.betterx.bclib.entity)).
        // EndEntities.register(5): not translated, it's generic.
        // EndEntities.register(9): not translated, it's generic.
    }

    /// `EndEntities.SpawnEgg` (org/betterx/betterend/registry/EndEntities.java)
    pub mod end_entities_spawn_egg {
        use super::*;
        // The class itself isn't translated: it uses `ItemKey` (de.ambertation.wover.item.api.classify). Its static parts are below.
        // EndEntities.SpawnEgg.SpawnEgg(4): not translated, its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)).
        // EndEntities.SpawnEgg.key(0): not translated, its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)).
        // EndEntities.SpawnEgg.type(0): not translated, its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)).
        // EndEntities.SpawnEgg.background(0): not translated, its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)).
        // EndEntities.SpawnEgg.dots(0): not translated, its class isn't translated (it uses `ItemKey` (de.ambertation.wover.item.api.classify)).
    }

    /// `EndEntitiesRenders` (org/betterx/betterend/registry/EndEntitiesRenders.java)
    pub mod end_entities_renders {
        use super::*;
        // DRAGONFLY_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // END_SLIME_SHELL_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // END_SLIME_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // END_FISH_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // CUBOZOA_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // SILK_MOTH_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // TEST_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // CRYSTALITE_CHESTPLATE: not translated, it uses Minecraft's `ModelLayerLocation`.
        // CRYSTALITE_CHESTPLATE_THIN: not translated, it uses Minecraft's `ModelLayerLocation`.
        // CRYSTALITE_HELMET: not translated, it uses Minecraft's `ModelLayerLocation`.
        // CRYSTALITE_LEGGINGS: not translated, it uses Minecraft's `ModelLayerLocation`.
        // CRYSTALITE_BOOTS: not translated, it uses Minecraft's `ModelLayerLocation`.
        #[derive(Clone, Debug)]
        pub struct EndEntitiesRenders {
        }
        impl EndEntitiesRenders {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndEntitiesRenders.register(0): not translated, it uses Minecraft's `EntityType`.
        // EndEntitiesRenders.register(2): not translated, it uses Minecraft's `EntityType`.
        // EndEntitiesRenders.registerMain(1): not translated, it uses Minecraft's `ModelLayerLocation`.
    }

    /// `EndFeatures` (org/betterx/betterend/registry/EndFeatures.java)
    pub mod end_features {
        use super::*;
        // STALACTITE_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // VINE_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // WALL_PLANT_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // WALL_PLANT_ON_LOG_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // GLOW_PILLAR_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // HYDRALUX_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // LANCELEAF_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // MENGER_SPONGE_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // SINGLE_PLANT_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // SINGLE_INVERTED_SCATTER_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // DOUBLE_PLANT_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // UNDERWATER_PLANT_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // ARCH_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // THIN_ARCH_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // CHARNIA_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // BLUE_VINE_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // FILALUX_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // END_LILY_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // END_LOTUS_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // END_LOTUS_LEAF_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // BUSH_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // SINGLE_BLOCK_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // BUSH_WITH_OUTER_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // STALACTITE_CLUSTER: not translated, it uses Minecraft's `MapCodec`.
        // CAVE_SURFACE_COAT: not translated, it uses Minecraft's `MapCodec`.
        // CAVE_VEGETATION_PATCH: not translated, it uses Minecraft's `MapCodec`.
        // LAYERED_ORE_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // ICE_STAR_FEATURE: not translated, it uses Minecraft's `MapCodec`.
        // MOSSY_GLOWSHROOM_FEATURE: not translated, it uses MossyGlowshroomFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PYTHADENDRON_TREE_FEATURE: not translated, it uses PythadendronTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LACUGROVE_FEATURE: not translated, it uses LacugroveFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DRAGON_TREE_FEATURE: not translated, it uses DragonTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // TENANEA_FEATURE: not translated, it uses TenaneaFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // HELIX_TREE_FEATURE: not translated, it uses HelixTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DRAGON_HELIX_TREE_FEATURE: not translated, it uses DragonHelixTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // UMBRELLA_TREE_FEATURE: not translated, it uses UmbrellaTreeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // JELLYSHROOM_FEATURE: not translated, it uses JellyshroomFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // GIGANTIC_AMARANITA_FEATURE: not translated, it uses GiganticAmaranitaFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LUCERNIA_FEATURE: not translated, it uses LucerniaFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // TENANEA_BUSH_FEATURE: not translated, it uses TenaneaBushFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LUMECORN_FEATURE: not translated, it uses Lumecorn, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LARGE_AMARANITA_FEATURE: not translated, it uses LargeAmaranitaFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // AMARANITA_PATCH_FEATURE: not translated, it uses AmaranitaPatchFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // NEON_CACTUS_FEATURE: not translated, it uses NeonCactusFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DESERT_LAKE_FEATURE: not translated, it uses DesertLakeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SULPHURIC_LAKE_FEATURE: not translated, it uses SulphuricLakeFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SURFACE_VENT_FEATURE: not translated, it uses SurfaceVentFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SULPHUR_HILL_FEATURE: not translated, it uses SulphurHillFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // OBSIDIAN_PILLAR_FEATURE: not translated, it uses ObsidianPillarBasementFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // OBSIDIAN_BOULDER_FEATURE: not translated, it uses ObsidianBoulderFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // FALLEN_PILLAR_FEATURE: not translated, it uses FallenPillarFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SILK_MOTH_NEST_FEATURE: not translated, it uses SilkMothNestFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SPIRE_FEATURE: not translated, it uses SpireFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // FLOATING_SPIRE_FEATURE: not translated, it uses FloatingSpireFeature, which isn't translated (it extends `SpireFeature`).
        // GEYSER_FEATURE: not translated, it uses GeyserFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // POND_WITH_WATERFALL_FEATURE: not translated, it uses PondWithWaterfallFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // OVERWORLD_ISLAND: not translated, it uses BiomeIslandFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SMARAGDANT_CRYSTAL_FEATURE: not translated, it uses SmaragdantCrystalFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // BIG_AURORA_CRYSTAL_FEATURE: not translated, it uses BigAuroraCrystalFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // CAVE_PUMPKIN_FEATURE: not translated, it uses CavePumpkinFeature, which isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        #[derive(Clone, Debug)]
        pub struct EndFeatures {
        }
        impl EndFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndFeatures.registerType(2): not translated, it's generic.
        // EndFeatures.register(3): not translated, it's generic.
        // EndFeatures.addDefaultFeatures(2): not translated, it uses EndBiomeBuilder, which isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        pub fn register_0() {
        }
    }

    /// `EndItems` (org/betterx/betterend/registry/EndItems.java)
    pub mod end_items {
        use super::*;
        // ITEMS_REGISTRY: not translated, it uses `ItemRegistry` (de.ambertation.wover.item.api).
        #[derive(Clone, Debug)]
        pub struct EndItems {
        }
        impl EndItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndItems.getModItems(0): not translated, it uses Minecraft's `Item`.
        // EndItems.key(2): not translated, it's generic.
        pub fn make_end_item_settings() -> JProps {
            return JProps::new();
        }
        // EndItems.getItemRegistry(0): not translated, it uses `ItemRegistry` (de.ambertation.wover.item.api).
        pub fn ensure_statically_loaded() {
            end_resource_items::ensure_loaded();
            end_food_items::ensure_loaded();
            end_disc_items::ensure_loaded();
            end_equipment_items::ensure_loaded();
            guide_book_item::ensure_statically_loaded();
        }
        pub fn default_settings() -> JProps {
            return JProps::new();
        }
    }

    /// `EndItems.ExternalItemModel` (org/betterx/betterend/registry/EndItems.java)
    pub mod end_items_external_item_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ExternalItemModel {
        }
        impl ExternalItemModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndItems.ExternalItemModel.build(0): not translated, it uses `ItemModel` (de.ambertation.wover.item.api.model).
        // EndItems.ExternalItemModel.factory(0): not translated, it uses `ItemModelFactory` (de.ambertation.wover.item.api.model).
    }

    /// `EndMenuTypes` (org/betterx/betterend/registry/EndMenuTypes.java)
    pub mod end_menu_types {
        use super::*;
        // END_STONE_SMELTER: not translated, it uses Minecraft's `MenuType`.
        #[derive(Clone, Debug)]
        pub struct EndMenuTypes {
        }
        impl EndMenuTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndMenuTypes.registerSimple(2): not translated, it's generic.
        pub fn ensure_statically_loaded() {
        }
    }

    /// `EndModelProviders` (org/betterx/betterend/registry/EndModelProviders.java)
    pub mod end_model_providers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndModelProviders {
        }
        impl EndModelProviders {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndModelProviders.register(0): not translated, it calls CrystaliteArmorRenderer.register, which isn't translated.
    }

    /// `EndNumericProviders` (org/betterx/betterend/registry/EndNumericProviders.java)
    pub mod end_numeric_providers {
        use super::*;
        // SULPHURIC_SURF: not translated, it uses Minecraft's `ResourceKey`.
        // VERTICAL_BAND: not translated, it uses Minecraft's `ResourceKey`.
        // SPLIT_NOISE: not translated, it uses Minecraft's `ResourceKey`.
        // UMBRA_SRF: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct EndNumericProviders {
        }
        impl EndNumericProviders {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndNumericProviders.register(0): not translated, it uses EndNumericProviders.SULPHURIC_SURF, which isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `EndParticleProviders` (org/betterx/betterend/registry/EndParticleProviders.java)
    pub mod end_particle_providers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndParticleProviders {
        }
        impl EndParticleProviders {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndParticleProviders.register(0): not translated, it calls Fabric's `ParticleProviderRegistry`.getInstance.
    }

    /// `EndParticles` (org/betterx/betterend/registry/EndParticles.java)
    pub mod end_particles {
        use super::*;
        // GLOWING_SPHERE: not translated, it uses Minecraft's `SimpleParticleType`.
        // PORTAL_SPHERE: not translated, it uses Minecraft's `SimpleParticleType`.
        // INFUSION: not translated, it uses Minecraft's `ParticleType`.
        // SULPHUR_PARTICLE: not translated, it uses Minecraft's `SimpleParticleType`.
        // GEYSER_PARTICLE: not translated, it uses Minecraft's `SimpleParticleType`.
        // SNOWFLAKE: not translated, it uses Minecraft's `SimpleParticleType`.
        // AMBER_SPHERE: not translated, it uses Minecraft's `SimpleParticleType`.
        // BLACK_SPORE: not translated, it uses Minecraft's `SimpleParticleType`.
        // TENANEA_PETAL: not translated, it uses Minecraft's `SimpleParticleType`.
        // JUNGLE_SPORE: not translated, it uses Minecraft's `SimpleParticleType`.
        // FIREFLY: not translated, it uses Minecraft's `SimpleParticleType`.
        // SMARAGDANT: not translated, it uses Minecraft's `SimpleParticleType`.
        // INFUSION_MIST: not translated, it uses Minecraft's `SimpleParticleType`.
        // INFUSION_MIST_NORTH: not translated, it uses Minecraft's `SimpleParticleType`.
        #[derive(Clone, Debug)]
        pub struct EndParticles {
        }
        impl EndParticles {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndParticles.register(1): not translated, it uses Minecraft's `SimpleParticleType`.
        // EndParticles.registerFar(1): not translated, it uses Minecraft's `SimpleParticleType`.
        pub fn ensure_statically_loaded_serverside() {
        }
    }

    /// `EndPoiTypes` (org/betterx/betterend/registry/EndPoiTypes.java)
    pub mod end_poi_types {
        use super::*;
        // ETERNAL_PORTAL: not translated, it uses `WoverPoiType` (de.ambertation.wover.poi.api).
        // ETERNAL_PORTAL_FRAME: not translated, it uses `WoverPoiType` (de.ambertation.wover.poi.api).
        #[derive(Clone, Debug)]
        pub struct EndPoiTypes {
        }
        impl EndPoiTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
        }
    }

    /// `EndPortalLooks` (org/betterx/betterend/registry/EndPortalLooks.java)
    pub mod end_portal_looks {
        use super::*;
        // ETERNAL: not translated, it uses `PortalLook` (de.ambertation.wover.portal.api).
        #[derive(Clone, Debug)]
        pub struct EndPortalLooks {
        }
        impl EndPortalLooks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndPortals` (org/betterx/betterend/registry/EndPortals.java)
    pub mod end_portals {
        use super::*;
        // OVERWORLD_ID: not translated, it uses Minecraft's `Level`.OVERWORLD.
        // portals: not translated, it uses PortalInfo, which isn't translated (it uses Minecraft's `ServerLevel`).
        #[derive(Clone, Debug)]
        pub struct EndPortals {
        }
        impl EndPortals {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndPortals.loadPortals(0): not translated, it uses `File` (java.io).
        // EndPortals.getCount(0): not translated, it calls EndPortals.loadPortals, which isn't translated.
        // EndPortals.getWorld(2): not translated, it uses Minecraft's `ServerLevel`.
        // EndPortals.getWorldId(1): not translated, a nullable object kept in a changing static.
        // EndPortals.getPortalIdByItem(1): not translated, a nullable object kept in a changing static.
        // EndPortals.getPortalIdByWorld(1): not translated, a nullable object kept in a changing static.
        // EndPortals.getColor(1): not translated, a nullable object kept in a changing static.
        // EndPortals.isAvailableItem(1): not translated, a nullable object kept in a changing static.
        // EndPortals.makeDefault(1): not translated, it uses `File` (java.io).
        // EndPortals.makeDefault(0): not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
    }

    /// `EndPortals.PortalInfo` (org/betterx/betterend/registry/EndPortals.java)
    pub mod end_portals_portal_info {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerLevel`. Its static parts are below.
        // EndPortals.PortalInfo.PortalInfo(1): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // EndPortals.PortalInfo.PortalInfo(5): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // EndPortals.PortalInfo.getWorld(1): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // EndPortals.PortalInfo.toJson(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
    }

    /// `EndProcessors` (org/betterx/betterend/registry/EndProcessors.java)
    pub mod end_processors {
        use super::*;
        // WEATHERED_10_PERCENT: not translated, it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors).
        // CRACK_20_PERCENT: not translated, it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors).
        // CRACK_AND_WEATHER: not translated, it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors).
        // END_STREET: not translated, it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors).
        // BIOME_BUILDING: not translated, it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors).
        // BIOME_BUILDING_MOUND: not translated, it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors).
        // CRASHED_SHIP: not translated, it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors).
        // CRYING_10_PERCENT: not translated, it uses `StructureProcessorKey` (de.ambertation.wover.structure.api.processors).
        #[derive(Clone, Debug)]
        pub struct EndProcessors {
        }
        impl EndProcessors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndScreens` (org/betterx/betterend/registry/EndScreens.java)
    pub mod end_screens {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndScreens {
        }
        impl EndScreens {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndScreens.register(0): not translated, it uses EndMenuTypes.END_STONE_SMELTER, which isn't translated (it uses Minecraft's `MenuType`).
    }

    /// `EndSounds` (org/betterx/betterend/registry/EndSounds.java)
    pub mod end_sounds {
        use super::*;
        // MUSIC_FOREST: not translated, it uses Minecraft's `Holder`.
        // MUSIC_WATER: not translated, it uses Minecraft's `Holder`.
        // MUSIC_DARK: not translated, it uses Minecraft's `Holder`.
        // MUSIC_OPENSPACE: not translated, it uses Minecraft's `Holder`.
        // MUSIC_CAVES: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_FOGGY_MUSHROOMLAND: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_CHORUS_FOREST: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_MEGALAKE: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_DUST_WASTELANDS: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_MEGALAKE_GROVE: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_BLOSSOMING_SPIRES: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_SULPHUR_SPRINGS: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_UMBRELLA_JUNGLE: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_GLOWING_GRASSLANDS: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_CAVES: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_AMBER_LAND: not translated, it uses Minecraft's `Holder`.
        // UMBRA_VALLEY: not translated, it uses Minecraft's `Holder`.
        // ENTITY_DRAGONFLY: not translated, it uses Minecraft's `Holder`.
        // ENTITY_SHADOW_WALKER: not translated, it uses Minecraft's `Holder`.
        // ENTITY_SHADOW_WALKER_DAMAGE: not translated, it uses Minecraft's `Holder`.
        // ENTITY_SHADOW_WALKER_DEATH: not translated, it uses Minecraft's `Holder`.
        // RECORD_STRANGE_AND_ALIEN_SOUND: not translated, it uses Minecraft's `Reference`.
        // RECORD_GRASPING_AT_STARS_SOUND: not translated, it uses Minecraft's `Reference`.
        // RECORD_ENDSEEKER_SOUND: not translated, it uses Minecraft's `Reference`.
        // RECORD_EO_DRACONA_SOUND: not translated, it uses Minecraft's `Reference`.
        // RECORD_STRANGE_AND_ALIEN: not translated, it uses Minecraft's `ResourceKey`.
        // RECORD_GRASPING_AT_STARS: not translated, it uses Minecraft's `ResourceKey`.
        // RECORD_ENDSEEKER: not translated, it uses Minecraft's `ResourceKey`.
        // RECORD_EO_DRACONA: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct EndSounds {
        }
        impl EndSounds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register_0() {
        }
        // EndSounds.createSongKey(1): not translated, it uses Minecraft's `ResourceKey`.
        // EndSounds.register(2): not translated, it uses Minecraft's `Reference`.
    }

    /// `EndStructures` (org/betterx/betterend/registry/EndStructures.java)
    pub mod end_structures {
        use super::*;
        // VOXEL_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // MOUNTAIN_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // CAVE_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // LAKE_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // END_LAKE_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // PAINTED_MOUNTAIN_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // NBT_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // END_BRIDGE_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // SULPHURIC_CAVE_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // GIANT_MOSSY_GLOWSHROOM: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // MEGALAKE: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // MEGALAKE_SMALL: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // END_LAKE: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // END_LAKE_NORMAL: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // END_LAKE_RARE: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // END_BRIDGE: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // MOUNTAIN: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // PAINTED_MOUNTAIN: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // ETERNAL_PORTAL: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // GIANT_ICE_STAR: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // SMALL_ISLAND: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // SULPHURIC_CAVE: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // END_VILLAGE: not translated, it uses `Jigsaw` (de.ambertation.wover.structure.api.StructureKey).
        // BLOSSOMING_SPIRES_HOUSE_POOL: not translated, it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools).
        // BLOSSOMING_SPIRES_HOUSE: not translated, it uses `Jigsaw` (de.ambertation.wover.structure.api.StructureKey).
        // CRASHED_SHIP_POOL: not translated, it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools).
        // CRASHED_SHIP: not translated, it uses `Jigsaw` (de.ambertation.wover.structure.api.StructureKey).
        #[derive(Clone, Debug)]
        pub struct EndStructures {
        }
        impl EndStructures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
        }
    }

    /// `EndTags` (org/betterx/betterend/registry/EndTags.java)
    pub mod end_tags {
        use super::*;
        // IS_END_CAVE: not translated, it uses Minecraft's `TagKey`.
        // IS_END_HIGH_OR_MIDLAND: not translated, it uses Minecraft's `TagKey`.
        // PEDESTALS: not translated, it uses Minecraft's `TagKey`.
        // ALLOYING_IRON: not translated, it uses Minecraft's `TagKey`.
        // ALLOYING_GOLD: not translated, it uses Minecraft's `TagKey`.
        // ALLOYING_COPPER: not translated, it uses Minecraft's `TagKey`.
        // ANVIL_AETERNIUM_TOOL: not translated, it uses Minecraft's `TagKey`.
        // ANVIL_IRON_TOOL: not translated, it uses Minecraft's `TagKey`.
        // ANVIL_NETHERITE_TOOL: not translated, it uses Minecraft's `TagKey`.
        // ANVIL_DIAMOND_TOOL: not translated, it uses Minecraft's `TagKey`.
        // CRYSTALITE_SET: not translated, it uses Minecraft's `TagKey`.
        // CRASHED_SHIP_ROTS: not translated, it uses Minecraft's `TagKey`.
        // BONEMEAL_SOURCE_DRAGON_BONE: not translated, it uses Minecraft's `TagKey`.
        // BONEMEAL_TARGET_DRAGON_BONE: not translated, it uses Minecraft's `TagKey`.
        // BONEMEAL_SOURCE_WATER_GRASS: not translated, it uses Minecraft's `TagKey`.
        // BONEMEAL_TARGET_WATER_GRASS: not translated, it uses Minecraft's `TagKey`.
        // INCORRECT_FOR_AETERNIUM_TOOL: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_PALLIDIUM: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_AMBER_MOSS: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_BRIMSTONE: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_CHORUS_NYLIUM: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_END_BONE: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_END_MOSS: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_END_STONE: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_END_STONE_OR_TREES: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_JUNGLE_MOSS: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_JUNGLE_MOSS_OR_MYCELIUM: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_MOSS_OR_DUST: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_MOSS_OR_MYCELIUM: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_PINK_MOSS: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_RUTISCUS: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_SHADOW_GRASS: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_SULPHURIC_ROCK: not translated, it uses Minecraft's `TagKey`.
        // SURVIVES_ON_DIRT: not translated, it uses Minecraft's `TagKey`.
        // SCATTER_FEATURE_EXTRA_GROUND: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct EndTags {
        }
        impl EndTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
        }
    }

    /// `EndTemplates` (org/betterx/betterend/registry/EndTemplates.java)
    pub mod end_templates {
        use super::*;
        // LEATHER_HANDLE_ATTACHMENT: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // TOOL_ASSEMBLY: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // HANDLE_ATTACHMENT: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // PLATE_UPGRADE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // THALLASIUM_UPGRADE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // TERMINITE_UPGRADE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // AETERNIUM_UPGRADE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // NETHERITE_UPGRADE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndTemplates {
        }
        impl EndTemplates {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_statically_loaded() {
        }
    }

    /// `EndVillagerProfessions` (org/betterx/betterend/registry/EndVillagerProfessions.java)
    pub mod end_villager_professions {
        use super::*;
        // END_TRADER: not translated, it uses `VillagerProfessionKey` (de.ambertation.wover.villager.api).
        #[derive(Clone, Debug)]
        pub struct EndVillagerProfessions {
        }
        impl EndVillagerProfessions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndVillagerTypes` (org/betterx/betterend/registry/EndVillagerTypes.java)
    pub mod end_villager_types {
        use super::*;
        // END: not translated, it uses `VillagerTypeKey` (de.ambertation.wover.villager.api).
        #[derive(Clone, Debug)]
        pub struct EndVillagerTypes {
        }
        impl EndVillagerTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndBonemealFeature` (org/betterx/betterend/registry/features/EndBonemealFeature.java)
    pub mod end_bonemeal_feature {
        use super::*;
        // BONEMEAL_END_MOSS_NOT_GLOWING_GRASSLANDS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BONEMEAL_END_MOSS_GLOWING_GRASSLANDS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BONEMEAL_RUTISCUS_NOT_LANTERN_WOODS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BONEMEAL_RUTISCUS_LANTERN_WOODS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct EndBonemealFeature {
        }
        impl EndBonemealFeature {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndConfiguredBonemealFeature` (org/betterx/betterend/registry/features/EndConfiguredBonemealFeature.java)
    pub mod end_configured_bonemeal_feature {
        use super::*;
        // BONEMEAL_END_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_RUTISCUS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_END_MYCELIUM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_JUNGLE_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_SANGNUM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_MOSSY_DRAGON_BONE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_MOSSY_OBSIDIAN: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_CAVE_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_CHORUS_NYLIUM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_CRYSTAL_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_SHADOW_GRASS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_PINK_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_AMBER_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct EndConfiguredBonemealFeature {
        }
        impl EndConfiguredBonemealFeature {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndConfiguredCaveFeatures` (org/betterx/betterend/registry/features/EndConfiguredCaveFeatures.java)
    pub mod end_configured_cave_features {
        use super::*;
        // SMARAGDANT_CRYSTAL: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // SMARAGDANT_CRYSTAL_SHARD: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BIG_AURORA_CRYSTAL: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // CAVE_BUSH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // CAVE_GRASS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // RUBINEA: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // MAGNULA: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // END_STONE_STALACTITE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // END_STONE_STALAGMITE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // END_STONE_STALACTITE_CAVEMOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // END_STONE_STALAGMITE_CAVEMOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // END_STONE_WITH_CAVEMOSS_STALACTITE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // END_STONE_WITH_CAVEMOSS_STALAGMITE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // CAVE_PUMPKIN: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // STALACTITE_CLUSTER_PLAIN: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // STALACTITE_CLUSTER_CAVEMOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // CAVE_SURFACE_COAT: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // CAVE_LUSH_FLOOR_PATCH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // CAVE_LUSH_CEILING_PATCH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct EndConfiguredCaveFeatures {
        }
        impl EndConfiguredCaveFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndConfiguredCaveFeatures.lateBound(1): not translated, it's generic.
    }

    /// `EndConfiguredLakeFeature` (org/betterx/betterend/registry/features/EndConfiguredLakeFeature.java)
    pub mod end_configured_lake_feature {
        use super::*;
        // SULPHURIC_LAKE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct EndConfiguredLakeFeature {
        }
        impl EndConfiguredLakeFeature {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndConfiguredVegetation` (org/betterx/betterend/registry/features/EndConfiguredVegetation.java)
    pub mod end_configured_vegetation {
        use super::*;
        // DRAGON_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // GIGANTIC_AMARANITA: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // HELIX_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // DRAGON_HELIX_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // JELLYSHROOM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // LACUGROVE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // LUCERNIA: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // MOSSY_GLOWSHROOM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PYTHADENDRON_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // TENANEA: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // UMBRELLA_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // LARGE_AMARANITA: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // LUMECORN: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // TENANEA_BUSH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct EndConfiguredVegetation {
        }
        impl EndConfiguredVegetation {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndLakeFeatures` (org/betterx/betterend/registry/features/EndLakeFeatures.java)
    pub mod end_lake_features {
        use super::*;
        // DESERT_LAKE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SULPHURIC_LAKE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct EndLakeFeatures {
        }
        impl EndLakeFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndOreFeatures` (org/betterx/betterend/registry/features/EndOreFeatures.java)
    pub mod end_ore_features {
        use super::*;
        // THALLASIUM_ORE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ENDER_ORE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // AMBER_ORE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // DRAGON_BONE_BLOCK_ORE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VIOLECITE_LAYER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FLAVOLITE_LAYER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct EndOreFeatures {
        }
        impl EndOreFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndPlacedCaveFeatures` (org/betterx/betterend/registry/features/EndPlacedCaveFeatures.java)
    pub mod end_placed_cave_features {
        use super::*;
        // STALACTITE_CLUSTER_PLAIN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // STALACTITE_CLUSTER_CAVEMOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // STALAGMITE_SCATTER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // STALACTITE_SCATTER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BIG_AURORA_CRYSTAL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMARAGDANT_CRYSTAL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMARAGDANT_SHARD_SCATTER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CAVE_SURFACE_COAT: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CAVE_LUSH_FLOOR_PATCH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CAVE_LUSH_CEILING_PATCH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CAVE_GRASS_SCATTER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CAVE_BUSH_SCATTER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CAVE_PUMPKIN_PLACED: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct EndPlacedCaveFeatures {
        }
        impl EndPlacedCaveFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndPlacedCaveFeatures.key(1): not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
    }

    /// `EndTerrainFeatures` (org/betterx/betterend/registry/features/EndTerrainFeatures.java)
    pub mod end_terrain_features {
        use super::*;
        // SURFACE_VENT: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SULPHUR_HILL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SULPHUR_SPIKE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SULPHUR_SPIKE_HANGING: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // OBSIDIAN_PILLAR_BASEMENT: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // OBSIDIAN_BOULDER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FALLEN_PILLAR: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // UMBRALITH_ARCH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // THIN_UMBRALITH_ARCH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SILK_MOTH_NEST: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SPIRE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FLOATING_SPIRE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GEYSER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ICE_STAR: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ICE_STAR_SMALL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // POND_WITH_WATERFALL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BIOME_ISLAND: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct EndTerrainFeatures {
        }
        impl EndTerrainFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EndVegetationFeatures` (org/betterx/betterend/registry/features/EndVegetationFeatures.java)
    pub mod end_vegetation_features {
        use super::*;
        // MOSSY_GLOWSHROOM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PYTHADENDRON_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LACUGROVE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // DRAGON_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TENANEA: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // HELIX_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // DRAGON_HELIX_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // UMBRELLA_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // JELLYSHROOM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GIGANTIC_AMARANITA: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LUCERNIA: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TENANEA_BUSH: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LUMECORN: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LARGE_AMARANITA: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NEON_CACTUS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PYTHADENDRON_BUSH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // DRAGON_TREE_BUSH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LUCERNIA_BUSH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LUCERNIA_BUSH_RARE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // DENSE_VINE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TWISTED_VINE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BULB_VINE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // JUNGLE_VINE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLUE_VINE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMALL_JELLYSHROOM_CEIL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PURPLE_POLYPORE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // AURANT_POLYPORE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TAIL_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CYAN_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TAIL_MOSS_WOOD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CYAN_MOSS_WOOD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TWISTED_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TWISTED_MOSS_WOOD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BULB_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BULB_MOSS_WOOD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMALL_JELLYSHROOM_WALL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMALL_JELLYSHROOM_WOOD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // JUNGLE_FERN_WOOD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // RUSCUS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // RUSCUS_WOOD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FILALUX: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BUBBLE_CORAL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BUBBLE_CORAL_RARE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // END_LILY: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // END_LILY_RARE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // END_LOTUS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // END_LOTUS_LEAF: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // HYDRALUX: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // POND_ANEMONE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // MENGER_SPONGE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FLAMAEA: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHARNIA_RED: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHARNIA_PURPLE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHARNIA_CYAN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHARNIA_LIGHT_BLUE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHARNIA_ORANGE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHARNIA_GREEN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHARNIA_RED_RARE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // UMBRELLA_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CREEPING_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHORUS_GRASS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CRYSTAL_GRASS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CRYSTAL_MOSS_COVER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SHADOW_PLANT: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // MURKWEED: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NEEDLEGRASS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SHADOW_BERRY: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BUSHY_GRASS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BUSHY_GRASS_WG: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // AMBER_GRASS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TWISTED_UMBRELLA_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // JUNGLE_GRASS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMALL_JELLYSHROOM_FLOOR: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLOSSOM_BERRY: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLOOMING_COOKSONIA: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SALTEAGO: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VAIOLUSH_FERN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FRACTURN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // UMBRELLA_MOSS_RARE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CREEPING_MOSS_RARE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TWISTED_UMBRELLA_MOSS_RARE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ORANGO: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // AERIDIUM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LUTEBUS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LAMELLARIUM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMALL_AMARANITA: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // AMARANITA_PATCH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOBULAGUS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CLAWFERN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BOLUX_MUSHROOM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CHORUS_MUSHROOM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // AMBER_ROOT: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // INFLEXIA: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FLAMMALIX: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LANCELEAF: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOW_PILLAR: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct EndVegetationFeatures {
        }
        impl EndVegetationFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndVegetationFeatures.createVegetationKey(1): not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
    }

    /// `GlimmerwildVegetationFeatures` (org/betterx/betterend/registry/features/GlimmerwildVegetationFeatures.java)
    pub mod glimmerwild_vegetation_features {
        use super::*;
        // WOOL_FLOWER_MEADOW: not translated, it uses Minecraft's `ResourceKey`.
        // UMBRELLA_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TWISTED_UMBRELLA_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CREEPING_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLOOMING_COOKSONIA: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SALTEAGO: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VAIOLUSH_FERN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FRACTURN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // JUNGLE_GRASS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ORANGO: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // AERIDIUM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LUTEBUS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LAMELLARIUM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // AMBER_GRASS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FLAMMALIX: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ALL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct GlimmerwildVegetationFeatures {
        }
        impl GlimmerwildVegetationFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GlimmerwildVegetationFeatures.createKey(1): not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
    }

    /// `EndDiscItems` (org/betterx/betterend/registry/item/EndDiscItems.java)
    pub mod end_disc_items {
        use super::*;
        // MUSIC_DISC_STRANGE_AND_ALIEN: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // MUSIC_DISC_GRASPING_AT_STARS: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // MUSIC_DISC_ENDSEEKER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // MUSIC_DISC_EO_DRACONA: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndDiscItems {
        }
        impl EndDiscItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndEquipmentItems` (org/betterx/betterend/registry/item/EndEquipmentItems.java)
    pub mod end_equipment_items {
        use super::*;
        // ATTACK_KNOCKBACK_MODIFIER_ID: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // AETERNIUM_SET: not translated, it uses AeterniumSet, which isn't translated (it extends `ToolsWithHeadsSet`).
        // CRYSTALITE_HELMET: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_CHESTPLATE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_LEGGINGS: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_BOOTS: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // ARMORED_ELYTRA: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_ELYTRA: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // IRON_HAMMER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // GOLDEN_HAMMER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // DIAMOND_HAMMER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // NETHERITE_HAMMER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndEquipmentItems {
        }
        impl EndEquipmentItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndFoodItems` (org/betterx/betterend/registry/item/EndFoodItems.java)
    pub mod end_food_items {
        use super::*;
        // SHADOW_BERRY_RAW: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // SHADOW_BERRY_COOKED: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // END_FISH_RAW: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // END_FISH_COOKED: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // BUCKET_END_FISH: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // BUCKET_CUBOZOA: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // SWEET_BERRY_JELLY: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // SHADOW_BERRY_JELLY: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // BLOSSOM_BERRY_JELLY: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // BLOSSOM_BERRY: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // AMBER_ROOT_RAW: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CHORUS_MUSHROOM_RAW: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CHORUS_MUSHROOM_COOKED: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // BOLUX_MUSHROOM_COOKED: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CAVE_PUMPKIN_PIE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // UMBRELLA_CLUSTER_JUICE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndFoodItems {
        }
        impl EndFoodItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndItemFactories` (org/betterx/betterend/registry/item/EndItemFactories.java)
    pub mod end_item_factories {
        use super::*;
        // BUCKET_END_FISH: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // BUCKET_CUBOZOA: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_HELMET: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_CHESTPLATE: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_LEGGINGS: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_BOOTS: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // ARMORED_ELYTRA: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CRYSTALITE_ELYTRA: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // GUIDEBOOK: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndItemFactories {
        }
        impl EndItemFactories {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EndResourceItems` (org/betterx/betterend/registry/item/EndResourceItems.java)
    pub mod end_resource_items {
        use super::*;
        // ENDER_DUST: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // ENDER_SHARD: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // END_LILY_LEAF: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // END_LILY_LEAF_DRIED: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CRYSTAL_SHARDS: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // RAW_AMBER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // AMBER_GEM: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // GLOWING_BULB: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CRYSTALLINE_SULPHUR: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // HYDRALUX_PETAL: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // GELATINE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // ETERNAL_CRYSTAL: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // ENCHANTED_PETAL: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // LEATHER_STRIPE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // LEATHER_WRAPPED_STICK: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // SILK_FIBER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // LUMECORN_ROD: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // SILK_MOTH_MATRIX: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // ENCHANTED_MEMBRANE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct EndResourceItems {
        }
        impl EndResourceItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EternalRitual` (org/betterx/betterend/rituals/EternalRitual.java)
    pub mod eternal_ritual {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Level`. Its static parts are below.
        // PEDESTAL_POSITIONS: not translated, it uses `Point` (java.awt).
        // PEDESTAL: not translated, it uses Minecraft's `Block`.
        // ACTIVE: not translated, it uses Minecraft's `BooleanProperty`.
        // EternalRitual.pedestalOffsets(0): not translated, it uses `Point` (java.awt).
        // EternalRitual.EternalRitual(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.EternalRitual(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.getCenter(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.getAxis(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.setWorld(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.getWorld(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.getTargetWorldId(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.isInvalid(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.checkStructure(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.updateActiveStateOnPedestals(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.updateActiveStateOnPedestals(6): not translated, it uses Minecraft's `BlockPos`.
        // EternalRitual.checkFrame(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.isActive(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.willActivate(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.activatePortal(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.initPortal(3): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.doEffects(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.activatePortal(3): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.disablePortal(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.removePortal(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.getTargetWorld(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.configure(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.hasPedestal(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.isActive(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.toTag(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.fromTag(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.equals(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.findRitualForActivePortal(2): not translated, it uses EternalRitual, which isn't translated (it uses Minecraft's `Level`).
        // EternalRitual.isActivePortalBlock(1): not translated, it uses Minecraft's `BlockState`.
    }

    /// `InfusionRitual` (org/betterx/betterend/rituals/InfusionRitual.java)
    pub mod infusion_ritual {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Container`. Its static parts are below.
        // PEDESTALS_MAP: not translated, it uses `Point` (java.awt).
        // InfusionRitual.InfusionRitual(3): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.configure(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.checkRecipe(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.updateRecipe(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.resetTimer(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.reset(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.tick(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.canPlaceItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.isValid(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.hasRecipe(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.setLocation(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.clearContent(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.getContainerSize(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.isEmpty(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.getItem(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.removeItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.removeItemNoUpdate(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.setItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.setChanged(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.setDirty(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.stillValid(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.fromTag(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.toTag(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InfusionRitual.getMap(0): not translated, it uses `Point` (java.awt).
        // InfusionRitual.socketPos(2): not translated, it uses Minecraft's `BlockPos`.
        // InfusionRitual.socketsPresent(2): not translated, it uses Minecraft's `BlockGetter`.
        // InfusionRitual.socketAt(2): not translated, it uses Minecraft's `BlockGetter`.
        // InfusionRitual.allSocketsPresent(2): not translated, it uses Minecraft's `BlockGetter`.
    }

    /// `InfusionRitual.InfusionInput` (org/betterx/betterend/rituals/InfusionRitual.java)
    pub mod infusion_ritual_infusion_input {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RecipeInput`. Its static parts are below.
        // InfusionRitual.InfusionInput.getItem(1): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
        // InfusionRitual.InfusionInput.size(0): not translated, its class isn't translated (it implements Minecraft's `RecipeInput`).
    }

    /// `InfusionRitual.Socket` (org/betterx/betterend/rituals/InfusionRitual.java)
    pub mod infusion_ritual_socket {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // InfusionRitual.Socket.Socket(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // InfusionRitual.Socket.isNorth(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // InfusionRitual.Socket.pedestal(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // InfusionRitual.Socket.index(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `CreativeTabs` (org/betterx/betterend/tab/CreativeTabs.java)
    pub mod creative_tabs {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CreativeTabs {
        }
        impl CreativeTabs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CreativeTabs.register(0): not translated, unknown name `de`.
        // CreativeTabs.inTabOrder(1): not translated, it uses `Stream` (java.util.stream).
        // CreativeTabs.keepTogether(2): not translated, it uses Minecraft's `Item`.
        // CreativeTabs.range(3): not translated, it uses Minecraft's `Item`.
        // CreativeTabs.isPlantLike(1): not translated, it uses Minecraft's `Block`.
    }

    /// `BlockFixer` (org/betterx/betterend/util/BlockFixer.java)
    pub mod block_fixer {
        use super::*;
        // AIR: not translated, it uses Minecraft's `BlockState`.
        // WATER: not translated, it uses Minecraft's `BlockState`.
        #[derive(Clone, Debug)]
        pub struct BlockFixer {
        }
        impl BlockFixer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BlockFixer.fixBlocks(3): not translated, it uses Minecraft's `LevelAccessor`.
        // BlockFixer.fixBlocksInZone(4): not translated, it uses Minecraft's `LevelAccessor`.
        // BlockFixer.fixBlocks(4): not translated, it uses Minecraft's `LevelAccessor`.
        // BlockFixer.getAirOrFluid(1): not translated, it uses Minecraft's `BlockState`.
        // BlockFixer.setWithoutUpdate(3): not translated, it uses Minecraft's `LevelAccessor`.
    }

    /// `BonemealPlants` (org/betterx/betterend/util/BonemealPlants.java)
    pub mod bonemeal_plants {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BonemealPlants {
        }
        impl BonemealPlants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BonemealPlants.init(0): not translated, it uses `BonemealAPI` (org.betterx.bclib.api.v3.bonemeal).INSTANCE.
    }

    /// `DebugHelpers` (org/betterx/betterend/util/DebugHelpers.java)
    pub mod debug_helpers {
        use super::*;
        // ITEMS: not translated, it uses DebugItem, which isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        #[derive(Clone, Debug)]
        pub struct DebugHelpers {
        }
        impl DebugHelpers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
        // DebugHelpers.items(0): not translated, it uses DebugItem, which isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        // DebugHelpers.add(2): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `DebugHelpers.DebugItem` (org/betterx/betterend/util/DebugHelpers.java)
    pub mod debug_helpers_debug_item {
        use super::*;
        // The class itself isn't translated: it uses `ItemCreator` (de.ambertation.wover.item.api.classify). Its static parts are below.
        // DebugHelpers.DebugItem.DebugItem(2): not translated, its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        // DebugHelpers.DebugItem.path(0): not translated, its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        // DebugHelpers.DebugItem.factory(0): not translated, its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
    }

    /// `GlobalState` (org/betterx/betterend/util/GlobalState.java)
    pub mod global_state {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `MutableBlockPos`. Its static parts are below.
        // STATE: not translated, it uses `ThreadLocal`.
        // GlobalState.stateForThread(0): not translated, it uses GlobalState, which isn't translated (it uses Minecraft's `MutableBlockPos`).
    }

    /// `GrowthPlacer` (org/betterx/betterend/util/GrowthPlacer.java)
    pub mod growth_placer {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `LevelAccessor`. Its static parts are below.
        // GrowthPlacer.GrowthPlacer(1): not translated, its class isn't translated (it uses Minecraft's `LevelAccessor`).
        // GrowthPlacer.set(2): not translated, its class isn't translated (it uses Minecraft's `LevelAccessor`).
        // GrowthPlacer.notifyNeighbours(0): not translated, its class isn't translated (it uses Minecraft's `LevelAccessor`).
    }

    /// `LangUtil` (org/betterx/betterend/util/LangUtil.java)
    pub mod lang_util {
        use super::*;
        pub const CONFIG_ELEMENT: &str = "configuration";
        #[derive(Clone, Debug)]
        pub struct LangUtil {
            pub element: String,
        }
        impl LangUtil {
            pub fn new(element: &str) -> Self {
                let mut s = Self { element: String::new() };
                s.element = element.to_string();
                s
            }
            pub fn set_element(&mut self, key: &str) {
                self.element = key.to_string();
            }
            // LangUtil.getString(1): not translated, it calls LangUtil.getString, which isn't translated.
            // LangUtil.getText(1): not translated, it uses Minecraft's `MutableComponent`.
        }
        // LangUtil.translate(1): not translated, it uses `Object`.
        // LangUtil.getString(2): not translated, it calls LangUtil.translate, which isn't translated.
        // LangUtil.getText(2): not translated, it uses Minecraft's `MutableComponent`.
    }

    /// `LootTableUtil` (org/betterx/betterend/util/LootTableUtil.java)
    pub mod loot_table_util {
        use super::*;
        // VILLAGE_LOOT: not translated, it uses Minecraft's `ResourceKey`.
        // VILLAGE_TEMPLATE_LOOT: not translated, it uses Minecraft's `ResourceKey`.
        // VILLAGE_BONUS_LOOT: not translated, it uses Minecraft's `ResourceKey`.
        // COMMON: not translated, it uses Minecraft's `ResourceKey`.
        // FOGGY_MUSHROOMLAND: not translated, it uses Minecraft's `ResourceKey`.
        // CHORUS_FOREST: not translated, it uses Minecraft's `ResourceKey`.
        // SHADOW_FOREST: not translated, it uses Minecraft's `ResourceKey`.
        // LANTERN_WOODS: not translated, it uses Minecraft's `ResourceKey`.
        // UMBRELLA_JUNGLE: not translated, it uses Minecraft's `ResourceKey`.
        // BIOME_CHEST: not translated, it uses Minecraft's `ResourceKey`.
        // END_CITY_EXTRA: not translated, it uses Minecraft's `ResourceKey`.
        // FISHING_FISH: not translated, it uses Minecraft's `ResourceKey`.
        // FISHING_TREASURE: not translated, it uses Minecraft's `ResourceKey`.
        // FISHING_JUNK: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct LootTableUtil {
        }
        impl LootTableUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LootTableUtil.getTable(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `WorldgenDebug` (org/betterx/betterend/util/WorldgenDebug.java)
    pub mod worldgen_debug {
        use super::*;
        pub static ENABLED: LazyLock<bool> = LazyLock::new(|| std::env::var("betterend.debug.worldgen").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        #[derive(Clone, Debug)]
        pub struct WorldgenDebug {
        }
        impl WorldgenDebug {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldgenDebug.log(2): not translated, it uses `Object`.
    }

    /// `BiomeIceStarfield` (org/betterx/betterend/world/biome/air/BiomeIceStarfield.java)
    pub mod biome_ice_starfield {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // BiomeIceStarfield.hasCaves(0): not translated, its class isn't translated (it extends `Config`).
        // BiomeIceStarfield.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
    }

    /// `FlowerIsletsBiome` (org/betterx/betterend/world/biome/air/FlowerIsletsBiome.java)
    pub mod flower_islets_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // FlowerIsletsBiome.hasCaves(0): not translated, its class isn't translated (it extends `Config`).
        // FlowerIsletsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // FlowerIsletsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `WaterfallPondsBiome` (org/betterx/betterend/world/biome/air/WaterfallPondsBiome.java)
    pub mod waterfall_ponds_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // WaterfallPondsBiome.hasCaves(0): not translated, its class isn't translated (it extends `Config`).
        // WaterfallPondsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // WaterfallPondsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `EmptyAuroraCaveBiome` (org/betterx/betterend/world/biome/cave/EmptyAuroraCaveBiome.java)
    pub mod empty_aurora_cave_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EmptyAuroraCaveBiome.instantiateBiome(11): not translated, its class isn't translated (it extends `Config`).
        // EmptyAuroraCaveBiome.EmptyAuroraCaveBiome(1): not translated, its class isn't translated (it extends `Config`).
        // EmptyAuroraCaveBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
    }

    /// `EmptyAuroraCaveBiome.Biome` (org/betterx/betterend/world/biome/cave/EmptyAuroraCaveBiome.java)
    pub mod empty_aurora_cave_biome_biome {
        use super::*;
        // The class itself isn't translated: it extends `EndCaveBiome`. Its static parts are below.
        // EmptyAuroraCaveBiome.Biome.codec(0): not translated, its class isn't translated (it extends `EndCaveBiome`).
        // EmptyAuroraCaveBiome.Biome.Biome(11): not translated, its class isn't translated (it extends `EndCaveBiome`).
    }

    /// `EmptyEndCaveBiome` (org/betterx/betterend/world/biome/cave/EmptyEndCaveBiome.java)
    pub mod empty_end_cave_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EmptyEndCaveBiome.EmptyEndCaveBiome(1): not translated, its class isn't translated (it extends `Config`).
        // EmptyEndCaveBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // EmptyEndCaveBiome.instantiateBiome(11): not translated, its class isn't translated (it extends `Config`).
    }

    /// `EmptyEndCaveBiome.Biome` (org/betterx/betterend/world/biome/cave/EmptyEndCaveBiome.java)
    pub mod empty_end_cave_biome_biome {
        use super::*;
        // The class itself isn't translated: it extends `EndCaveBiome`. Its static parts are below.
        // EmptyEndCaveBiome.Biome.codec(0): not translated, its class isn't translated (it extends `EndCaveBiome`).
        // EmptyEndCaveBiome.Biome.Biome(11): not translated, its class isn't translated (it extends `EndCaveBiome`).
    }

    /// `EmptySmaragdantCaveBiome` (org/betterx/betterend/world/biome/cave/EmptySmaragdantCaveBiome.java)
    pub mod empty_smaragdant_cave_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EmptySmaragdantCaveBiome.EmptySmaragdantCaveBiome(1): not translated, its class isn't translated (it extends `Config`).
        // EmptySmaragdantCaveBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // EmptySmaragdantCaveBiome.instantiateBiome(11): not translated, its class isn't translated (it extends `Config`).
    }

    /// `EmptySmaragdantCaveBiome.Biome` (org/betterx/betterend/world/biome/cave/EmptySmaragdantCaveBiome.java)
    pub mod empty_smaragdant_cave_biome_biome {
        use super::*;
        // The class itself isn't translated: it extends `EndCaveBiome`. Its static parts are below.
        // EmptySmaragdantCaveBiome.Biome.codec(0): not translated, its class isn't translated (it extends `EndCaveBiome`).
        // EmptySmaragdantCaveBiome.Biome.Biome(11): not translated, its class isn't translated (it extends `EndCaveBiome`).
    }

    /// `EndCaveBiome` (org/betterx/betterend/world/biome/cave/EndCaveBiome.java)
    pub mod end_cave_biome {
        use super::*;
        // The class itself isn't translated: it extends `EndBiome` (org.betterx.betterend.world.biome). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndCaveBiome.simpleCaveBiomeCodec(1): not translated, it's generic.
        // EndCaveBiome.codec(0): not translated, its class isn't translated (it extends `EndBiome` (org.betterx.betterend.world.biome)).
        // EndCaveBiome.EndCaveBiome(11): not translated, its class isn't translated (it extends `EndBiome` (org.betterx.betterend.world.biome)).
        // EndCaveBiome.getCeil(1): not translated, its class isn't translated (it extends `EndBiome` (org.betterx.betterend.world.biome)).
        // EndCaveBiome.getWall(1): not translated, its class isn't translated (it extends `EndBiome` (org.betterx.betterend.world.biome)).
    }

    /// `EndCaveBiome.Config` (org/betterx/betterend/world/biome/cave/EndCaveBiome.java)
    pub mod end_cave_biome_config {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EndCaveBiome.Config.Config(1): not translated, its class isn't translated (it's generic).
        // EndCaveBiome.Config.addCustomBuildData(1): not translated, its class isn't translated (it's generic).
        // EndCaveBiome.Config.hasCaves(0): not translated, its class isn't translated (it's generic).
        // EndCaveBiome.Config.hasReturnGateway(0): not translated, its class isn't translated (it's generic).
        // EndCaveBiome.Config.instantiateBiome(11): not translated, its class isn't translated (it's generic).
    }

    /// `JadeCaveBiome` (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java)
    pub mod jade_cave_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // JadeCaveBiome.JadeCaveBiome(1): not translated, its class isn't translated (it extends `Config`).
        // JadeCaveBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // JadeCaveBiome.instantiateBiome(11): not translated, its class isn't translated (it extends `Config`).
    }

    /// `JadeCaveBiome.Biome` (org/betterx/betterend/world/biome/cave/JadeCaveBiome.java)
    pub mod jade_cave_biome_biome {
        use super::*;
        // The class itself isn't translated: it extends `EndCaveBiome`. Its static parts are below.
        // WALL_NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // DEPTH_NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // JADE: not translated, it uses Minecraft's `BlockState`.
        // JadeCaveBiome.Biome.codec(0): not translated, its class isn't translated (it extends `EndCaveBiome`).
        // JadeCaveBiome.Biome.Biome(11): not translated, its class isn't translated (it extends `EndCaveBiome`).
        // JadeCaveBiome.Biome.getWall(1): not translated, its class isn't translated (it extends `EndCaveBiome`).
    }

    /// `LushAuroraCaveBiome` (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java)
    pub mod lush_aurora_cave_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // LushAuroraCaveBiome.LushAuroraCaveBiome(1): not translated, its class isn't translated (it extends `Config`).
        // LushAuroraCaveBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // LushAuroraCaveBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
        // LushAuroraCaveBiome.instantiateBiome(11): not translated, its class isn't translated (it extends `Config`).
    }

    /// `LushAuroraCaveBiome.Biome` (org/betterx/betterend/world/biome/cave/LushAuroraCaveBiome.java)
    pub mod lush_aurora_cave_biome_biome {
        use super::*;
        // The class itself isn't translated: it extends `EndCaveBiome`. Its static parts are below.
        // LushAuroraCaveBiome.Biome.codec(0): not translated, its class isn't translated (it extends `EndCaveBiome`).
        // LushAuroraCaveBiome.Biome.Biome(11): not translated, its class isn't translated (it extends `EndCaveBiome`).
        // LushAuroraCaveBiome.Biome.getCeil(1): not translated, its class isn't translated (it extends `EndCaveBiome`).
    }

    /// `LushSmaragdantCaveBiome` (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java)
    pub mod lush_smaragdant_cave_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // LushSmaragdantCaveBiome.LushSmaragdantCaveBiome(1): not translated, its class isn't translated (it extends `Config`).
        // LushSmaragdantCaveBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // LushSmaragdantCaveBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
        // LushSmaragdantCaveBiome.instantiateBiome(11): not translated, its class isn't translated (it extends `Config`).
    }

    /// `LushSmaragdantCaveBiome.Biome` (org/betterx/betterend/world/biome/cave/LushSmaragdantCaveBiome.java)
    pub mod lush_smaragdant_cave_biome_biome {
        use super::*;
        // The class itself isn't translated: it extends `EndCaveBiome`. Its static parts are below.
        // LushSmaragdantCaveBiome.Biome.codec(0): not translated, its class isn't translated (it extends `EndCaveBiome`).
        // LushSmaragdantCaveBiome.Biome.Biome(11): not translated, its class isn't translated (it extends `EndCaveBiome`).
    }

    /// `EndBiome` (org/betterx/betterend/world/biome/EndBiome.java)
    pub mod end_biome {
        use super::*;
        // The class itself isn't translated: it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndBiome.EndBiome(11): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.datagenSetup(1): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.codec(0): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.setHasCaves(1): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.hasCaves(0): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.getTopMaterial(0): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.getUnderMaterial(0): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.getAltTopMaterial(0): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.generateFloorRule(0): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.surface(0): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // EndBiome.findTopMaterial(1): not translated, it uses Minecraft's `BlockState`.
        // EndBiome.findTopMaterial(2): not translated, it uses Minecraft's `BlockState`.
        // EndBiome.sampleTopMaterial(2): not translated, it uses Minecraft's `BlockState`.
        // EndBiome.findUnderMaterial(1): not translated, it uses Minecraft's `BlockState`.
        // EndBiome.findUnderMaterial(2): not translated, it uses Minecraft's `BlockState`.
    }

    /// `EndBiome.Config` (org/betterx/betterend/world/biome/EndBiome.java)
    pub mod end_biome_config {
        use super::*;
        // The class itself isn't translated: it implements `BiomeFactory`. Its static parts are below.
        // DEFAULT_MATERIAL: not translated, it uses `SurfaceMaterialProvider` (org.betterx.bclib.interfaces).
        // END_STONE: not translated, it uses Minecraft's `MaterialRule`.
        // END_MOSS: not translated, it uses Minecraft's `MaterialRule`.
        // ENDSTONE_DUST: not translated, it uses Minecraft's `MaterialRule`.
        // END_MYCELIUM: not translated, it uses Minecraft's `MaterialRule`.
        // FLAVOLITE: not translated, it uses Minecraft's `MaterialRule`.
        // SULPHURIC_ROCK: not translated, it uses Minecraft's `MaterialRule`.
        // BRIMSTONE: not translated, it uses Minecraft's `MaterialRule`.
        // PALLIDIUM_FULL: not translated, it uses Minecraft's `MaterialRule`.
        // PALLIDIUM_HEAVY: not translated, it uses Minecraft's `MaterialRule`.
        // PALLIDIUM_THIN: not translated, it uses Minecraft's `MaterialRule`.
        // PALLIDIUM_TINY: not translated, it uses Minecraft's `MaterialRule`.
        // UMBRALITH: not translated, it uses Minecraft's `MaterialRule`.
        // EndBiome.Config.Config(0): not translated, its class isn't translated (it implements `BiomeFactory`).
        // EndBiome.Config.addCustomBuildData(1): not translated, it has no body.
        // EndBiome.Config.hasCaves(0): not translated, its class isn't translated (it implements `BiomeFactory`).
        // EndBiome.Config.hasReturnGateway(0): not translated, its class isn't translated (it implements `BiomeFactory`).
        // EndBiome.Config.surfaceMaterial(0): not translated, its class isn't translated (it implements `BiomeFactory`).
        // EndBiome.Config.instantiateBiome(11): not translated, its class isn't translated (it implements `BiomeFactory`).
    }

    /// `EndBiome.DefaultSurfaceMaterialProvider` (org/betterx/betterend/world/biome/EndBiome.java)
    pub mod end_biome_default_surface_material_provider {
        use super::*;
        // The class itself isn't translated: it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces). Its static parts are below.
        // END_STONE: not translated, it uses Minecraft's `BlockState`.
        // EndBiome.DefaultSurfaceMaterialProvider.getTopMaterial(0): not translated, its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)).
        // EndBiome.DefaultSurfaceMaterialProvider.getAltTopMaterial(0): not translated, its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)).
        // EndBiome.DefaultSurfaceMaterialProvider.getUnderMaterial(0): not translated, its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)).
        // EndBiome.DefaultSurfaceMaterialProvider.generateFloorRule(0): not translated, its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)).
        // EndBiome.DefaultSurfaceMaterialProvider.surface(0): not translated, its class isn't translated (it implements `SurfaceMaterialProvider` (org.betterx.bclib.interfaces)).
    }

    /// `EndBiomeBuilder` (org/betterx/betterend/world/biome/EndBiomeBuilder.java)
    pub mod end_biome_builder {
        use super::*;
        // The class itself isn't translated: it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder). Its static parts are below.
        pub const GEN_CHANCE_FACTOR: f32 = 1.25f32;
        // ON_FLOOR: not translated, it uses Minecraft's `MaterialCondition`.
        // UNDER_FLOOR: not translated, it uses Minecraft's `MaterialCondition`.
        // ON_CEILING: not translated, it uses Minecraft's `MaterialCondition`.
        // UNDER_CEILING: not translated, it uses Minecraft's `MaterialCondition`.
        // EndBiomeBuilder.EndBiomeBuilder(3): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // EndBiomeBuilder.configure(1): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // EndBiomeBuilder.createKey(1): not translated, it's generic.
        // EndBiomeBuilder.createKey(2): not translated, it's generic.
        // EndBiomeBuilder.registerBiomeData(1): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
    }

    /// `EndBiomeBuilder.BiomeFactory` (org/betterx/betterend/world/biome/EndBiomeBuilder.java)
    pub mod end_biome_builder_biome_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EndBiomeBuilder.BiomeFactory.instantiateBiome(11): not translated, it has no body.
    }

    /// `EndBiomeKey` (org/betterx/betterend/world/biome/EndBiomeKey.java)
    pub mod end_biome_key {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EndBiomeKey.EndBiomeKey(2): not translated, its class isn't translated (it's generic).
        // EndBiomeKey.bootstrap(1): not translated, its class isn't translated (it's generic).
        // EndBiomeKey.bootstrap(3): not translated, its class isn't translated (it's generic).
    }

    /// `EndBiomeKey.ConfigSupplier` (org/betterx/betterend/world/biome/EndBiomeKey.java)
    pub mod end_biome_key_config_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EndBiomeKey.ConfigSupplier.get(1): not translated, it has no body.
    }

    /// `AmberLandBiome` (org/betterx/betterend/world/biome/land/AmberLandBiome.java)
    pub mod amber_land_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // AmberLandBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // AmberLandBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `BlossomingSpiresBiome` (org/betterx/betterend/world/biome/land/BlossomingSpiresBiome.java)
    pub mod blossoming_spires_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // BlossomingSpiresBiome.hasCaves(0): not translated, its class isn't translated (it extends `Config`).
        // BlossomingSpiresBiome.hasReturnGateway(0): not translated, its class isn't translated (it extends `Config`).
        // BlossomingSpiresBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // BlossomingSpiresBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `ChorusForestBiome` (org/betterx/betterend/world/biome/land/ChorusForestBiome.java)
    pub mod chorus_forest_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // ChorusForestBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // ChorusForestBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `CrystalMountainsBiome` (org/betterx/betterend/world/biome/land/CrystalMountainsBiome.java)
    pub mod crystal_mountains_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // CrystalMountainsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // CrystalMountainsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `DragonGraveyardsBiome` (org/betterx/betterend/world/biome/land/DragonGraveyardsBiome.java)
    pub mod dragon_graveyards_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // DragonGraveyardsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // DragonGraveyardsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `DryShrublandBiome` (org/betterx/betterend/world/biome/land/DryShrublandBiome.java)
    pub mod dry_shrubland_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // DryShrublandBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // DryShrublandBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `DustWastelandsBiome` (org/betterx/betterend/world/biome/land/DustWastelandsBiome.java)
    pub mod dust_wastelands_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // DustWastelandsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // DustWastelandsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `FoggyMushroomlandBiome` (org/betterx/betterend/world/biome/land/FoggyMushroomlandBiome.java)
    pub mod foggy_mushroomland_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // FoggyMushroomlandBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // FoggyMushroomlandBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `GlowingGrasslandsBiome` (org/betterx/betterend/world/biome/land/GlowingGrasslandsBiome.java)
    pub mod glowing_grasslands_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // GlowingGrasslandsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // GlowingGrasslandsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `LanternWoodsBiome` (org/betterx/betterend/world/biome/land/LanternWoodsBiome.java)
    pub mod lantern_woods_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // LanternWoodsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // LanternWoodsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `MegalakeBiome` (org/betterx/betterend/world/biome/land/MegalakeBiome.java)
    pub mod megalake_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // MegalakeBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // MegalakeBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `MegalakeGroveBiome` (org/betterx/betterend/world/biome/land/MegalakeGroveBiome.java)
    pub mod megalake_grove_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // MegalakeGroveBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // MegalakeGroveBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `NeonOasisBiome` (org/betterx/betterend/world/biome/land/NeonOasisBiome.java)
    pub mod neon_oasis_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // NeonOasisBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // NeonOasisBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `PaintedMountainsBiome` (org/betterx/betterend/world/biome/land/PaintedMountainsBiome.java)
    pub mod painted_mountains_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // PaintedMountainsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // PaintedMountainsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `ShadowForestBiome` (org/betterx/betterend/world/biome/land/ShadowForestBiome.java)
    pub mod shadow_forest_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // ShadowForestBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // ShadowForestBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `SulphurSpringsBiome` (org/betterx/betterend/world/biome/land/SulphurSpringsBiome.java)
    pub mod sulphur_springs_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // SulphurSpringsBiome.hasCaves(0): not translated, its class isn't translated (it extends `Config`).
        // SulphurSpringsBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // SulphurSpringsBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `UmbraValleyBiome` (org/betterx/betterend/world/biome/land/UmbraValleyBiome.java)
    pub mod umbra_valley_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // SURFACE_BLOCKS: not translated, it uses Minecraft's `Block`.
        // UmbraValleyBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // UmbraValleyBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
        // UmbraValleyBiome.getSurface(2): not translated, it uses Minecraft's `Block`.
    }

    /// `UmbrellaJungleBiome` (org/betterx/betterend/world/biome/land/UmbrellaJungleBiome.java)
    pub mod umbrella_jungle_biome {
        use super::*;
        // The class itself isn't translated: it extends `Config`. Its static parts are below.
        // UmbrellaJungleBiome.addCustomBuildData(1): not translated, its class isn't translated (it extends `Config`).
        // UmbrellaJungleBiome.surfaceMaterial(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `EndCaveCarver` (org/betterx/betterend/world/carvers/EndCaveCarver.java)
    pub mod end_cave_carver {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `WorldCarver`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const DEFAULT_BAND_CEILING: i32 = 48;
        // EndCaveCarver.EndCaveCarver(4): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndCaveCarver.codec(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndCaveCarver.isStartChunk(1): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndCaveCarver.getRange(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndCaveCarver.caveBandCeiling(0): not translated, it uses `WoverEndConfig` (de.ambertation.wover.generator.api.biomesource.end).
        // EndCaveCarver.carve(5): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndCaveCarver.probability(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndCaveCarver.y(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndCaveCarver.radius(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndCaveCarver.verticalSquash(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
    }

    /// `EndTunnelCarver` (org/betterx/betterend/world/carvers/EndTunnelCarver.java)
    pub mod end_tunnel_carver {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `WorldCarver`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const DENSITY: f32 = 0.0f32;
        pub static CACHED: LazyLock<JCell<Option<end_tunnel_carver_noises3::Noises3>>> = LazyLock::new(|| JCell::new(None));
        // EndTunnelCarver.EndTunnelCarver(2): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndTunnelCarver.codec(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndTunnelCarver.isStartChunk(1): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndTunnelCarver.getRange(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndTunnelCarver.carve(5): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndTunnelCarver.getNoises(0): not translated, it creates Minecraft's `LegacyRandomSource`.
        // EndTunnelCarver.probability(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
        // EndTunnelCarver.threshold(0): not translated, its class isn't translated (it implements Minecraft's `WorldCarver`).
    }

    /// `EndTunnelCarver.Noises3` (org/betterx/betterend/world/carvers/EndTunnelCarver.java)
    pub mod end_tunnel_carver_noises3 {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Noises3 {
            pub seed: i64,
            pub h: open_simplex_noise::OpenSimplexNoise,
            pub v: open_simplex_noise::OpenSimplexNoise,
            pub d: open_simplex_noise::OpenSimplexNoise,
        }
        impl Noises3 {
            pub fn new(seed: i64, h: &open_simplex_noise::OpenSimplexNoise, v: &open_simplex_noise::OpenSimplexNoise, d: &open_simplex_noise::OpenSimplexNoise) -> Self {
                let mut s = Self { seed: seed, h: h.clone(), v: v.clone(), d: d.clone() };
                s.seed = seed;
                s.h = h.clone();
                s.v = v.clone();
                s.d = d.clone();
                s
            }
            pub fn seed(&self) -> i64 {
                return self.seed;
            }
            pub fn h(&self) -> open_simplex_noise::OpenSimplexNoise {
                return self.h.clone();
            }
            pub fn v(&self) -> open_simplex_noise::OpenSimplexNoise {
                return self.v.clone();
            }
            pub fn d(&self) -> open_simplex_noise::OpenSimplexNoise {
                return self.d.clone();
            }
        }
    }

    /// `BiomeIslandFeature` (org/betterx/betterend/world/features/BiomeIslandFeature.java)
    pub mod biome_island_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CENTER: not translated, it uses Minecraft's `MutableBlockPos`.
        // ISLAND: not translated, it uses `SDF` (org.betterx.bclib.sdf).
        // simplexNoise: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // topBlock: not translated, it uses Minecraft's `BlockState`.
        // underBlock: not translated, it uses Minecraft's `BlockState`.
        // BiomeIslandFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // BiomeIslandFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // BiomeIslandFeature.createSDFIsland(0): not translated, it uses `SDF` (org.betterx.bclib.sdf).
    }

    /// `BlueVineFeature` (org/betterx/betterend/world/features/BlueVineFeature.java)
    pub mod blue_vine_feature {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BlueVineFeature.BlueVineFeature(1): not translated, its class isn't translated (it extends `ScatterFeature`).
        // BlueVineFeature.codec(0): not translated, its class isn't translated (it extends `ScatterFeature`).
        // BlueVineFeature.canGenerate(6): not translated, its class isn't translated (it extends `ScatterFeature`).
        // BlueVineFeature.generate(4): not translated, its class isn't translated (it extends `ScatterFeature`).
    }

    /// `AmaranitaPatchFeature` (org/betterx/betterend/world/features/bushes/AmaranitaPatchFeature.java)
    pub mod amaranita_patch_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const MIN_COUNT: i32 = 4;
        pub const MAX_COUNT: i32 = 8;
        pub const PATCH_RADIUS: i32 = 4;
        // AmaranitaPatchFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // AmaranitaPatchFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // AmaranitaPatchFeature.isGround(1): not translated, it uses Minecraft's `BlockState`.
        // AmaranitaPatchFeature.growMushroom(4): not translated, it uses Minecraft's `WorldGenLevel`.
    }

    /// `BushFeature` (org/betterx/betterend/world/features/bushes/BushFeature.java)
    pub mod bush_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // BushFeature.BushFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // BushFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // BushFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `BushFeatureConfig` (org/betterx/betterend/world/features/bushes/BushFeatureConfig.java)
    pub mod bush_feature_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockStateProvider`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BushFeatureConfig.BushFeatureConfig(2): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // BushFeatureConfig.BushFeatureConfig(2): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
    }

    /// `BushWithOuterFeature` (org/betterx/betterend/world/features/bushes/BushWithOuterFeature.java)
    pub mod bush_with_outer_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DIRECTIONS: not translated, it uses Minecraft's `Direction`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // BushWithOuterFeature.BushWithOuterFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // BushWithOuterFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // BushWithOuterFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `BushWithOuterFeatureConfig` (org/betterx/betterend/world/features/bushes/BushWithOuterFeatureConfig.java)
    pub mod bush_with_outer_feature_config {
        use super::*;
        // The class itself isn't translated: it extends `BushFeatureConfig`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BushWithOuterFeatureConfig.BushWithOuterFeatureConfig(3): not translated, its class isn't translated (it extends `BushFeatureConfig`).
        // BushWithOuterFeatureConfig.BushWithOuterFeatureConfig(3): not translated, its class isn't translated (it extends `BushFeatureConfig`).
    }

    /// `LargeAmaranitaFeature` (org/betterx/betterend/world/features/bushes/LargeAmaranitaFeature.java)
    pub mod large_amaranita_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // LargeAmaranitaFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LargeAmaranitaFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `Lumecorn` (org/betterx/betterend/world/features/bushes/Lumecorn.java)
    pub mod lumecorn {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // Lumecorn.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // Lumecorn.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `TenaneaBushFeature` (org/betterx/betterend/world/features/bushes/TenaneaBushFeature.java)
    pub mod tenanea_bush_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // DIRECTIONS: not translated, it uses Minecraft's `Direction`.
        // TenaneaBushFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // TenaneaBushFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `CavePumpkinFeature` (org/betterx/betterend/world/features/CavePumpkinFeature.java)
    pub mod cave_pumpkin_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CavePumpkinFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // CavePumpkinFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `CharniaFeature` (org/betterx/betterend/world/features/CharniaFeature.java)
    pub mod charnia_feature {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CharniaFeature.CharniaFeature(1): not translated, its class isn't translated (it extends `UnderwaterPlantFeature`).
        // CharniaFeature.codec(0): not translated, its class isn't translated (it extends `UnderwaterPlantFeature`).
        // CharniaFeature.getChance(0): not translated, its class isn't translated (it extends `UnderwaterPlantFeature`).
    }

    /// `DoublePlantFeature` (org/betterx/betterend/world/features/DoublePlantFeature.java)
    pub mod double_plant_feature {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DoublePlantFeature.DoublePlantFeature(1): not translated, its class isn't translated (it extends `ScatterFeature`).
        // DoublePlantFeature.codec(0): not translated, its class isn't translated (it extends `ScatterFeature`).
        // DoublePlantFeature.canGenerate(6): not translated, its class isn't translated (it extends `ScatterFeature`).
        // DoublePlantFeature.generate(4): not translated, its class isn't translated (it extends `ScatterFeature`).
    }

    /// `DoublePlantFeatureConfig` (org/betterx/betterend/world/features/DoublePlantFeatureConfig.java)
    pub mod double_plant_feature_config {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeatureConfig`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DoublePlantFeatureConfig.DoublePlantFeatureConfig(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // DoublePlantFeatureConfig.DoublePlantFeatureConfig(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // DoublePlantFeatureConfig.getLargePlantState(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // DoublePlantFeatureConfig.getSmallPlantState(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
    }

    /// `EndLilyFeature` (org/betterx/betterend/world/features/EndLilyFeature.java)
    pub mod end_lily_feature {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantScatter`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndLilyFeature.EndLilyFeature(1): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // EndLilyFeature.codec(0): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // EndLilyFeature.generate(4): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // EndLilyFeature.getChance(0): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
    }

    /// `EndLotusFeature` (org/betterx/betterend/world/features/EndLotusFeature.java)
    pub mod end_lotus_feature {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantScatter`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndLotusFeature.EndLotusFeature(1): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // EndLotusFeature.codec(0): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // EndLotusFeature.generate(4): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // EndLotusFeature.getChance(0): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
    }

    /// `EndLotusLeafFeature` (org/betterx/betterend/world/features/EndLotusLeafFeature.java)
    pub mod end_lotus_leaf_feature {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // EndLotusLeafFeature.EndLotusLeafFeature(1): not translated, its class isn't translated (it extends `ScatterFeature`).
        // EndLotusLeafFeature.codec(0): not translated, its class isn't translated (it extends `ScatterFeature`).
        // EndLotusLeafFeature.generate(4): not translated, its class isn't translated (it extends `ScatterFeature`).
        // EndLotusLeafFeature.getChance(0): not translated, its class isn't translated (it extends `ScatterFeature`).
        // EndLotusLeafFeature.getCenterGround(3): not translated, its class isn't translated (it extends `ScatterFeature`).
        // EndLotusLeafFeature.generateLeaf(2): not translated, its class isn't translated (it extends `ScatterFeature`).
        // EndLotusLeafFeature.canGenerate(2): not translated, its class isn't translated (it extends `ScatterFeature`).
        // EndLotusLeafFeature.canGenerate(6): not translated, its class isn't translated (it extends `ScatterFeature`).
    }

    /// `FilaluxFeature` (org/betterx/betterend/world/features/FilaluxFeature.java)
    pub mod filalux_feature {
        use super::*;
        // The class itself isn't translated: it extends `SkyScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FilaluxFeature.FilaluxFeature(1): not translated, its class isn't translated (it extends `SkyScatterFeature`).
        // FilaluxFeature.codec(0): not translated, its class isn't translated (it extends `SkyScatterFeature`).
        // FilaluxFeature.generate(4): not translated, its class isn't translated (it extends `SkyScatterFeature`).
    }

    /// `GlowPillarFeature` (org/betterx/betterend/world/features/GlowPillarFeature.java)
    pub mod glow_pillar_feature {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // GlowPillarFeature.GlowPillarFeature(1): not translated, its class isn't translated (it extends `ScatterFeature`).
        // GlowPillarFeature.codec(0): not translated, its class isn't translated (it extends `ScatterFeature`).
        // GlowPillarFeature.canGenerate(6): not translated, its class isn't translated (it extends `ScatterFeature`).
        // GlowPillarFeature.generate(4): not translated, its class isn't translated (it extends `ScatterFeature`).
        // GlowPillarFeature.getChance(0): not translated, its class isn't translated (it extends `ScatterFeature`).
    }

    /// `HydraluxFeature` (org/betterx/betterend/world/features/HydraluxFeature.java)
    pub mod hydralux_feature {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantScatter`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // HydraluxFeature.HydraluxFeature(1): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // HydraluxFeature.codec(0): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // HydraluxFeature.generate(4): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // HydraluxFeature.getChance(0): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
    }

    /// `InvertedScatterFeature` (org/betterx/betterend/world/features/InvertedScatterFeature.java)
    pub mod inverted_scatter_feature {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InvertedScatterFeature.InvertedScatterFeature(1): not translated, its class isn't translated (it's generic).
        // InvertedScatterFeature.canGenerate(6): not translated, it has no body.
        // InvertedScatterFeature.generate(4): not translated, it has no body.
        // InvertedScatterFeature.place(4): not translated, its class isn't translated (it's generic).
    }

    /// `LanceleafFeature` (org/betterx/betterend/world/features/LanceleafFeature.java)
    pub mod lanceleaf_feature {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // LanceleafFeature.LanceleafFeature(1): not translated, its class isn't translated (it extends `ScatterFeature`).
        // LanceleafFeature.codec(0): not translated, its class isn't translated (it extends `ScatterFeature`).
        // LanceleafFeature.canGenerate(6): not translated, its class isn't translated (it extends `ScatterFeature`).
        // LanceleafFeature.generate(4): not translated, its class isn't translated (it extends `ScatterFeature`).
        // LanceleafFeature.getChance(0): not translated, its class isn't translated (it extends `ScatterFeature`).
    }

    /// `MengerSpongeFeature` (org/betterx/betterend/world/features/MengerSpongeFeature.java)
    pub mod menger_sponge_feature {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantScatter`. Its static parts are below.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MengerSpongeFeature.MengerSpongeFeature(1): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // MengerSpongeFeature.codec(0): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // MengerSpongeFeature.generate(4): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
    }

    /// `NeonCactusFeature` (org/betterx/betterend/world/features/NeonCactusFeature.java)
    pub mod neon_cactus_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NeonCactusFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // NeonCactusFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `ScatterFeature` (org/betterx/betterend/world/features/ScatterFeature.java)
    pub mod scatter_feature {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ScatterFeature.ScatterFeature(1): not translated, its class isn't translated (it's generic).
        // ScatterFeature.canGenerate(6): not translated, it has no body.
        // ScatterFeature.generate(4): not translated, it has no body.
        // ScatterFeature.getCenterGround(3): not translated, its class isn't translated (it's generic).
        // ScatterFeature.canSpawn(3): not translated, its class isn't translated (it's generic).
        // ScatterFeature.getGroundPlant(3): not translated, its class isn't translated (it's generic).
        // ScatterFeature.getYOffset(0): not translated, its class isn't translated (it's generic).
        // ScatterFeature.getChance(0): not translated, its class isn't translated (it's generic).
        // ScatterFeature.place(4): not translated, its class isn't translated (it's generic).
    }

    /// `ScatterFeatureConfig` (org/betterx/betterend/world/features/ScatterFeatureConfig.java)
    pub mod scatter_feature_config {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug)]
        pub struct ScatterFeatureConfig {
            pub radius: i32,
        }
        impl ScatterFeatureConfig {
            pub fn new(radius: i32) -> Self {
                let mut s = Self { radius: radius };
                s.radius = radius;
                s
            }
        }
        pub fn blue_vine() -> scatter_feature_config::ScatterFeatureConfig {
            return scatter_feature_config::ScatterFeatureConfig::new(5);
        }
        pub fn filalux() -> scatter_feature_config::ScatterFeatureConfig {
            return scatter_feature_config::ScatterFeatureConfig::new(10);
        }
        pub fn glow_pillar() -> scatter_feature_config::ScatterFeatureConfig {
            return scatter_feature_config::ScatterFeatureConfig::new(9);
        }
        pub fn lanceleaf() -> scatter_feature_config::ScatterFeatureConfig {
            return scatter_feature_config::ScatterFeatureConfig::new(7);
        }
    }

    /// `SilkMothNestFeature` (org/betterx/betterend/world/features/SilkMothNestFeature.java)
    pub mod silk_moth_nest_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SilkMothNestFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SilkMothNestFeature.canGenerate(2): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SilkMothNestFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `SingleInvertedScatterFeature` (org/betterx/betterend/world/features/SingleInvertedScatterFeature.java)
    pub mod single_inverted_scatter_feature {
        use super::*;
        // The class itself isn't translated: it extends `InvertedScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SingleInvertedScatterFeature.SingleInvertedScatterFeature(1): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // SingleInvertedScatterFeature.codec(0): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // SingleInvertedScatterFeature.canGenerate(6): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // SingleInvertedScatterFeature.generate(4): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
    }

    /// `SinglePlantFeature` (org/betterx/betterend/world/features/SinglePlantFeature.java)
    pub mod single_plant_feature {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SinglePlantFeature.SinglePlantFeature(1): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SinglePlantFeature.codec(0): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SinglePlantFeature.getCenterGround(3): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SinglePlantFeature.canGenerate(6): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SinglePlantFeature.generate(4): not translated, its class isn't translated (it extends `ScatterFeature`).
    }

    /// `SinglePlantFeatureConfig` (org/betterx/betterend/world/features/SinglePlantFeatureConfig.java)
    pub mod single_plant_feature_config {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeatureConfig`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SinglePlantFeatureConfig.SinglePlantFeatureConfig(2): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // SinglePlantFeatureConfig.SinglePlantFeatureConfig(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // SinglePlantFeatureConfig.SinglePlantFeatureConfig(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // SinglePlantFeatureConfig.SinglePlantFeatureConfig(4): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // SinglePlantFeatureConfig.SinglePlantFeatureConfig(4): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // SinglePlantFeatureConfig.getPlantState(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // SinglePlantFeatureConfig.charnia(1): not translated, it uses SinglePlantFeatureConfig, which isn't translated (it extends `ScatterFeatureConfig`).
    }

    /// `SkyScatterFeature` (org/betterx/betterend/world/features/SkyScatterFeature.java)
    pub mod sky_scatter_feature {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeature`. Its static parts are below.
        // SkyScatterFeature.SkyScatterFeature(1): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SkyScatterFeature.getChance(0): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SkyScatterFeature.canGenerate(6): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SkyScatterFeature.canSpawn(3): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SkyScatterFeature.getCenterGround(3): not translated, its class isn't translated (it extends `ScatterFeature`).
        // SkyScatterFeature.getGroundPlant(3): not translated, its class isn't translated (it extends `ScatterFeature`).
    }

    /// `ArchFeature` (org/betterx/betterend/world/features/terrain/ArchFeature.java)
    pub mod arch_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ArchFeature.ArchFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // ArchFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // ArchFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `ArchFeatureConfig` (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java)
    pub mod arch_feature_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockStateProvider`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ArchFeatureConfig.ArchFeatureConfig(2): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // ArchFeatureConfig.ArchFeatureConfig(2): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
    }

    /// `ArchFeatureConfig.SurfaceFunction` (org/betterx/betterend/world/features/terrain/ArchFeatureConfig.java)
    pub mod arch_feature_config_surface_function {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ArchFeatureConfig.SurfaceFunction.SurfaceFunction(2): not translated, its class isn't translated (its constants carry values).
        // ArchFeatureConfig.SurfaceFunction.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // ArchFeatureConfig.SurfaceFunction.apply(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `BigAuroraCrystalFeature` (org/betterx/betterend/world/features/terrain/BigAuroraCrystalFeature.java)
    pub mod big_aurora_crystal_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const RARITY_DIVISOR: i32 = 7;
        // BigAuroraCrystalFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // BigAuroraCrystalFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `CaveSurfaceCoatConfig` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatConfig.java)
    pub mod cave_surface_coat_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `HolderSet`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CaveSurfaceCoatConfig.CaveSurfaceCoatConfig(5): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // CaveSurfaceCoatConfig.replaceable(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // CaveSurfaceCoatConfig.protect(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // CaveSurfaceCoatConfig.minY(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // CaveSurfaceCoatConfig.maxY(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
        // CaveSurfaceCoatConfig.shellDepth(0): not translated, its class isn't translated (it uses Minecraft's `HolderSet`).
    }

    /// `CaveSurfaceCoatFeature` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java)
    pub mod cave_surface_coat_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const SOLID: i8 = 0i8;
        pub const ROCK: i8 = 1i8;
        pub const CAVE_AIR: i8 = 2i8;
        pub const OPEN_AIR: i8 = 3i8;
        pub const MARGIN: i32 = 1;
        pub const SPAN: i32 = 18;
        pub const NO_RUN: i32 = -1;
        pub const OPEN_RUN: i32 = -2;
        // CaveSurfaceCoatFeature.CaveSurfaceCoatFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CaveSurfaceCoatFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CaveSurfaceCoatFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CaveSurfaceCoatFeature.readColumn(9): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CaveSurfaceCoatFeature.expand(7): not translated, it uses `IntArrayList` (it.unimi.dsi.fastutil.ints).
        pub fn is_cave_air(cells: &[i8], dx: i32, dy: i32, dz: i32, height: i32) -> bool {
            return cave_surface_coat_feature::in_window(dx, dy, dz, height) && ((cells[cave_surface_coat_feature::index(dx, dy, dz, height) as usize] as i32) == 2);
        }
        pub fn touches_open_air(cells: &[i8], dx: i32, dy: i32, dz: i32, height: i32) -> bool {
            return ((((cave_surface_coat_feature::is_open_air(cells, dx + 1, dy, dz, height) || cave_surface_coat_feature::is_open_air(cells, dx - 1, dy, dz, height)) || cave_surface_coat_feature::is_open_air(cells, dx, dy + 1, dz, height)) || cave_surface_coat_feature::is_open_air(cells, dx, dy - 1, dz, height)) || cave_surface_coat_feature::is_open_air(cells, dx, dy, dz + 1, height)) || cave_surface_coat_feature::is_open_air(cells, dx, dy, dz - 1, height);
        }
        pub fn is_open_air(cells: &[i8], dx: i32, dy: i32, dz: i32, height: i32) -> bool {
            return cave_surface_coat_feature::in_window(dx, dy, dz, height) && ((cells[cave_surface_coat_feature::index(dx, dy, dz, height) as usize] as i32) == 3);
        }
        pub fn in_window(dx: i32, dy: i32, dz: i32, height: i32) -> bool {
            return (((((dx >= (-1)) && (dx < 17)) && (dz >= (-1))) && (dz < 17)) && (dy >= 0)) && (dy < height);
        }
        pub fn index(dx: i32, dy: i32, dz: i32, height: i32) -> i32 {
            return (((((dx + 1) * 18) + dz) + 1) * height) + dy;
        }
        // CaveSurfaceCoatFeature.toPos(5): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CaveSurfaceCoatFeature.coatable(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CaveSurfaceCoatFeature.isDecoration(3): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CaveSurfaceCoatFeature.paintsAnything(2): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `CaveSurfaceCoatFeature.Columns` (org/betterx/betterend/world/features/terrain/caves/CaveSurfaceCoatFeature.java)
    pub mod cave_surface_coat_feature_columns {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `WorldGenLevel`. Its static parts are below.
        // CaveSurfaceCoatFeature.Columns.Columns(2): not translated, its class isn't translated (it uses Minecraft's `WorldGenLevel`).
        pub fn key(x: i32, z: i32) -> i64 {
            return ((x as i64) << 32) ^ ((z as i64) & 4294967295i64);
        }
        // CaveSurfaceCoatFeature.Columns.chunkAt(2): not translated, its class isn't translated (it uses Minecraft's `WorldGenLevel`).
        // CaveSurfaceCoatFeature.Columns.biomeAt(2): not translated, its class isn't translated (it uses Minecraft's `WorldGenLevel`).
    }

    /// `CaveVegetationPatchFeature` (org/betterx/betterend/world/features/terrain/caves/CaveVegetationPatchFeature.java)
    pub mod cave_vegetation_patch_feature {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `VegetationPatchFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CaveVegetationPatchFeature.CaveVegetationPatchFeature(10): not translated, its class isn't translated (it extends Minecraft's `VegetationPatchFeature`).
        // CaveVegetationPatchFeature.codec(0): not translated, its class isn't translated (it extends Minecraft's `VegetationPatchFeature`).
        // CaveVegetationPatchFeature.placeGroundPatch(6): not translated, its class isn't translated (it extends Minecraft's `VegetationPatchFeature`).
        // CaveVegetationPatchFeature.placeGround(5): not translated, its class isn't translated (it extends Minecraft's `VegetationPatchFeature`).
        // CaveVegetationPatchFeature.isExposed(4): not translated, it uses Minecraft's `WorldGenLevel`.
    }

    /// `StalactiteClusterConfig` (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterConfig.java)
    pub mod stalactite_cluster_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `IntProvider`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // StalactiteClusterConfig.StalactiteClusterConfig(5): not translated, its class isn't translated (it uses Minecraft's `IntProvider`).
        // StalactiteClusterConfig.radius(0): not translated, its class isn't translated (it uses Minecraft's `IntProvider`).
        // StalactiteClusterConfig.density(0): not translated, its class isn't translated (it uses Minecraft's `IntProvider`).
        // StalactiteClusterConfig.pairChance(0): not translated, its class isn't translated (it uses Minecraft's `IntProvider`).
        // StalactiteClusterConfig.floorFeature(0): not translated, its class isn't translated (it uses Minecraft's `IntProvider`).
        // StalactiteClusterConfig.ceilFeature(0): not translated, its class isn't translated (it uses Minecraft's `IntProvider`).
    }

    /// `StalactiteClusterFeature` (org/betterx/betterend/world/features/terrain/caves/StalactiteClusterFeature.java)
    pub mod stalactite_cluster_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const SEARCH_RANGE: i32 = 32;
        // StalactiteClusterFeature.StalactiteClusterFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StalactiteClusterFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StalactiteClusterFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StalactiteClusterFeature.findFloor(5): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StalactiteClusterFeature.findCeiling(5): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `DesertLakeFeature` (org/betterx/betterend/world/features/terrain/DesertLakeFeature.java)
    pub mod desert_lake_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // END_STONE: not translated, it uses Minecraft's `BlockState`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // DesertLakeFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DesertLakeFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DesertLakeFeature.canReplace(1): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `FallenPillarFeature` (org/betterx/betterend/world/features/terrain/FallenPillarFeature.java)
    pub mod fallen_pillar_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FallenPillarFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // FallenPillarFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `FloatingSpireFeature` (org/betterx/betterend/world/features/terrain/FloatingSpireFeature.java)
    pub mod floating_spire_feature {
        use super::*;
        // The class itself isn't translated: it extends `SpireFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FloatingSpireFeature.codec(0): not translated, its class isn't translated (it extends `SpireFeature`).
        // FloatingSpireFeature.place(4): not translated, its class isn't translated (it extends `SpireFeature`).
    }

    /// `GeyserFeature` (org/betterx/betterend/world/features/terrain/GeyserFeature.java)
    pub mod geyser_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE1: not translated, it uses Minecraft's `BlockState`.
        // REPLACE2: not translated, it uses Minecraft's `BlockState`.
        // IGNORE: not translated, it uses Minecraft's `BlockState`.
        // HORIZONTAL: not translated, it uses Minecraft's `Direction`.
        // GeyserFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // GeyserFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `IceStarFeature` (org/betterx/betterend/world/features/terrain/IceStarFeature.java)
    pub mod ice_star_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // IceStarFeature.IceStarFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // IceStarFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // IceStarFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // IceStarFeature.getFibonacciPoints(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `IceStarFeatureConfig` (org/betterx/betterend/world/features/terrain/IceStarFeatureConfig.java)
    pub mod ice_star_feature_config {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug)]
        pub struct IceStarFeatureConfig {
            pub min_size: f32,
            pub max_size: f32,
            pub min_count: i32,
            pub max_count: i32,
        }
        impl IceStarFeatureConfig {
            pub fn new(min_size: f32, max_size: f32, min_count: i32, max_count: i32) -> Self {
                let mut s = Self { min_size: min_size, max_size: max_size, min_count: min_count, max_count: max_count };
                s.min_size = min_size;
                s.max_size = max_size;
                s.min_count = min_count;
                s.max_count = max_count;
                s
            }
        }
    }

    /// `ObsidianBoulderFeature` (org/betterx/betterend/world/features/terrain/ObsidianBoulderFeature.java)
    pub mod obsidian_boulder_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ObsidianBoulderFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // ObsidianBoulderFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // ObsidianBoulderFeature.makeBoulder(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `ObsidianPillarBasementFeature` (org/betterx/betterend/world/features/terrain/ObsidianPillarBasementFeature.java)
    pub mod obsidian_pillar_basement_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ObsidianPillarBasementFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // ObsidianPillarBasementFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `OreLayerFeature` (org/betterx/betterend/world/features/terrain/OreLayerFeature.java)
    pub mod ore_layer_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SPHERE: not translated, it uses `SDFSphere` (org.betterx.bclib.sdf.primitive).
        // NOISE: not translated, it uses `SDFCoordModify` (org.betterx.bclib.sdf.operator).
        // FUNCTION: not translated, it uses `SDF` (org.betterx.bclib.sdf).
        // OreLayerFeature.OreLayerFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // OreLayerFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // OreLayerFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `OreLayerFeatureConfig` (org/betterx/betterend/world/features/terrain/OreLayerFeatureConfig.java)
    pub mod ore_layer_feature_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // OreLayerFeatureConfig.OreLayerFeatureConfig(4): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OreLayerFeatureConfig.getNoise(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `PondWithWaterfallFeature` (org/betterx/betterend/world/features/terrain/PondWithWaterfallFeature.java)
    pub mod pond_with_waterfall_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // END_STONE: not translated, it uses Minecraft's `BlockState`.
        // END_MOSS: not translated, it uses Minecraft's `BlockState`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        pub const MIN_POND: i32 = 3;
        pub const MAX_POND: i32 = 10;
        pub const DRY_RIM: i32 = 3;
        pub const MAX_PLATEAU_PROBE: i32 = 16;
        pub const PLATEAU_FLAT_TOL: i32 = 2;
        pub const JITTER: f64 = 1.5f64;
        pub const MAX_FLATNESS_VARIANCE: i32 = 3;
        pub const MIN_TERRAIN_ABOVE_FLOOR: i32 = 5;
        pub const MIN_DEPTH: i32 = 4;
        pub const MAX_DEPTH: i32 = 7;
        pub const FALLBACK_DEPTH: i32 = 2;
        pub const MIN_SUPPORT_BELOW_BOWL: i32 = 2;
        pub const MAX_THICKNESS_PROBE: i32 = 24;
        // PondWithWaterfallFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PondWithWaterfallFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PondWithWaterfallFeature.probeFlatPlateau(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PondWithWaterfallFeature.probeSolidThickness(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PondWithWaterfallFeature.spillWaterfall(7): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PondWithWaterfallFeature.shuffledHorizontals(1): not translated, it uses Minecraft's `Direction`.
    }

    /// `SingleBlockFeature` (org/betterx/betterend/world/features/terrain/SingleBlockFeature.java)
    pub mod single_block_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SingleBlockFeature.SingleBlockFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // SingleBlockFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // SingleBlockFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `SmaragdantCrystalFeature` (org/betterx/betterend/world/features/terrain/SmaragdantCrystalFeature.java)
    pub mod smaragdant_crystal_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const SURFACE_REJECT_CHANCE: i32 = 80;
        // SmaragdantCrystalFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SmaragdantCrystalFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SmaragdantCrystalFeature.touchesOpenSky(2): not translated, it uses Minecraft's `WorldGenLevel`.
    }

    /// `SpireFeature` (org/betterx/betterend/world/features/terrain/SpireFeature.java)
    pub mod spire_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // SpireFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SpireFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SpireFeature.addSegment(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `StalactiteFeature` (org/betterx/betterend/world/features/terrain/StalactiteFeature.java)
    pub mod stalactite_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // StalactiteFeature.StalactiteFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StalactiteFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StalactiteFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `StalactiteFeatureConfig` (org/betterx/betterend/world/features/terrain/StalactiteFeatureConfig.java)
    pub mod stalactite_feature_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockStateProvider`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // StalactiteFeatureConfig.StalactiteFeatureConfig(3): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // StalactiteFeatureConfig.StalactiteFeatureConfig(6): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // StalactiteFeatureConfig.StalactiteFeatureConfig(3): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
    }

    /// `SulphurFloorMix` (org/betterx/betterend/world/features/terrain/SulphurFloorMix.java)
    pub mod sulphur_floor_mix {
        use super::*;
        // TYPE_NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // CINNABAR_NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        pub const MIN_DEPTH: i32 = 3;
        pub const TYPE_SCALE: f64 = 0.11f64;
        pub const CINNABAR_SCALE: f64 = 0.14f64;
        pub const CINNABAR_THRESHOLD: f64 = 0.45f64;
        #[derive(Clone, Debug)]
        pub struct SulphurFloorMix {
        }
        impl SulphurFloorMix {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SulphurFloorMix.pick(3): not translated, it uses Minecraft's `BlockState`.
        // SulphurFloorMix.pickByNoise(3): not translated, it uses Minecraft's `BlockState`.
        // SulphurFloorMix.apply(3): not translated, it uses Minecraft's `WorldGenLevel`.
        // SulphurFloorMix.isBuried(2): not translated, it uses Minecraft's `WorldGenLevel`.
        // SulphurFloorMix.isDepositBlock(1): not translated, it uses Minecraft's `BlockState`.
        // SulphurFloorMix.facesOpenSpace(3): not translated, it uses Minecraft's `WorldGenLevel`.
    }

    /// `SulphurHillFeature` (org/betterx/betterend/world/features/terrain/SulphurHillFeature.java)
    pub mod sulphur_hill_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SulphurHillFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphurHillFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphurHillFeature.makeCircle(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `SulphuricLakeFeature` (org/betterx/betterend/world/features/terrain/SulphuricLakeFeature.java)
    pub mod sulphuric_lake_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // SulphuricLakeFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphuricLakeFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphuricLakeFeature.clampedRelative(3): not translated, it uses Minecraft's `BlockPos`.
        // SulphuricLakeFeature.isBorder(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphuricLakeFeature.isAbsoluteBorder(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphuricLakeFeature.isDeepWater(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphuricLakeFeature.placeBrimstone(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphuricLakeFeature.getBrimstone(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SulphuricLakeFeature.makeShards(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `SurfaceVentFeature` (org/betterx/betterend/world/features/terrain/SurfaceVentFeature.java)
    pub mod surface_vent_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SurfaceVentFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // SurfaceVentFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `ThinArchFeature` (org/betterx/betterend/world/features/terrain/ThinArchFeature.java)
    pub mod thin_arch_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ThinArchFeature.ThinArchFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // ThinArchFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // ThinArchFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `ThinArchFeatureConfig` (org/betterx/betterend/world/features/terrain/ThinArchFeatureConfig.java)
    pub mod thin_arch_feature_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockStateProvider`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ThinArchFeatureConfig.ThinArchFeatureConfig(1): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // ThinArchFeatureConfig.ThinArchFeatureConfig(1): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
    }

    /// `DragonHelixTreeFeature` (org/betterx/betterend/world/features/trees/DragonHelixTreeFeature.java)
    pub mod dragon_helix_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const LIGHT_CHANCE: f32 = 0.6f32;
        pub const HELIX_TURN: f64 = 1.15f64;
        pub const HELIX_REACH: f64 = 1.8f64;
        // DragonHelixTreeFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DragonHelixTreeFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DragonHelixTreeFeature.placeLeafRing(7): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DragonHelixTreeFeature.hangBulbVine(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `DragonTreeFeature` (org/betterx/betterend/world/features/trees/DragonTreeFeature.java)
    pub mod dragon_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // IGNORE: not translated, it uses Minecraft's `BlockState`.
        // POST: not translated, it uses `PosInfo` (org.betterx.bclib.sdf).
        // BRANCH: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        // SIDE1: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        // SIDE2: not translated, it uses DragonTreeFeature.SIDE1, which isn't translated (it calls `Lists` (com.google.common.collect).newArrayList).
        // ROOT: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        pub const LEAF_BALL_BULGE: f32 = 1.5f32;
        // DragonTreeFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DragonTreeFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DragonTreeFeature.makeCap(6): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DragonTreeFeature.makeRoots(5): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // DragonTreeFeature.leavesBall(6): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `EndTreeHelper` (org/betterx/betterend/world/features/trees/EndTreeHelper.java)
    pub mod end_tree_helper {
        use super::*;
        pub const SCAN_DOWN: i32 = 16;
        pub const SCAN_UP: i32 = 8;
        pub const GATE_DOWN: i32 = 3;
        pub const GATE_UP: i32 = 2;
        #[derive(Clone, Debug)]
        pub struct EndTreeHelper {
        }
        impl EndTreeHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndTreeHelper.fitBallRadius(5): not translated, it uses `WriteZone` (de.ambertation.wover.feature.api).
        // EndTreeHelper.pruneUnsupportedFur(2): not translated, it uses Minecraft's `WorldGenLevel`.
        // EndTreeHelper.dropIfUnsupported(3): not translated, it uses Minecraft's `WorldGenLevel`.
        // EndTreeHelper.waterlogSubmerged(3): not translated, it uses Minecraft's `WorldGenLevel`.
        // EndTreeHelper.hasWater(8): not translated, it uses Minecraft's `WorldGenLevel`.
        // EndTreeHelper.isLakeWater(1): not translated, it uses Minecraft's `BlockState`.
    }

    /// `GiganticAmaranitaFeature` (org/betterx/betterend/world/features/trees/GiganticAmaranitaFeature.java)
    pub mod gigantic_amaranita_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // IGNORE: not translated, it uses Minecraft's `BlockState`.
        // POST: not translated, it uses `PosInfo` (org.betterx.bclib.sdf).
        // GiganticAmaranitaFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // GiganticAmaranitaFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // GiganticAmaranitaFeature.makeHead(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `HelixTreeFeature` (org/betterx/betterend/world/features/trees/HelixTreeFeature.java)
    pub mod helix_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // POST: not translated, it uses `PosInfo` (org.betterx.bclib.sdf).
        // HelixTreeFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // HelixTreeFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // HelixTreeFeature.fillLine(6): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `JellyshroomFeature` (org/betterx/betterend/world/features/trees/JellyshroomFeature.java)
    pub mod jellyshroom_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // ROOT: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        // JellyshroomFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // JellyshroomFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // JellyshroomFeature.makeRoots(6): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // JellyshroomFeature.makeCap(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `LacugroveFeature` (org/betterx/betterend/world/features/trees/LacugroveFeature.java)
    pub mod lacugrove_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // IGNORE: not translated, it uses Minecraft's `BlockState`.
        // POST: not translated, it uses `PosInfo` (org.betterx.bclib.sdf).
        pub const LEAF_BALL_BULGE: f32 = 4.5f32;
        // LacugroveFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LacugroveFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LacugroveFeature.grow(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LacugroveFeature.generate(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LacugroveFeature.leavesBall(6): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `LucerniaFeature` (org/betterx/betterend/world/features/trees/LucerniaFeature.java)
    pub mod lucernia_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DIRECTIONS: not translated, it uses Minecraft's `Direction`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // IGNORE: not translated, it uses Minecraft's `BlockState`.
        // SPLINE: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        // ROOT: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        pub const LEAF_BALL_BULGE: f32 = 3.5f32;
        // LucerniaFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LucerniaFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LucerniaFeature.leavesBall(8): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // LucerniaFeature.makeRoots(5): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `MossyGlowshroomFeature` (org/betterx/betterend/world/features/trees/MossyGlowshroomFeature.java)
    pub mod mossy_glowshroom_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        pub static CENTER: LazyLock<JVec3> = LazyLock::new(|| JVec3::new(0.0, 0.0, 0.0));
        // FUNCTION: not translated, it uses `SDFBinary` (org.betterx.bclib.sdf.operator).
        // HEAD_POS: not translated, it uses `SDFTranslate` (org.betterx.bclib.sdf.operator).
        // ROOTS_ROT: not translated, it uses `SDFFlatWave` (org.betterx.bclib.sdf.operator).
        // CONE1: not translated, it uses `SDFPrimitive` (org.betterx.bclib.sdf.primitive).
        // CONE2: not translated, it uses `SDFPrimitive` (org.betterx.bclib.sdf.primitive).
        // CONE_GLOW: not translated, it uses `SDFPrimitive` (org.betterx.bclib.sdf.primitive).
        // ROOTS: not translated, it uses `SDFPrimitive` (org.betterx.bclib.sdf.primitive).
        pub const CAP_RADIUS: f32 = 13.0f32;
        pub const GLOW_RADIUS: f32 = 12.5f32;
        pub const INNER_CONE_SCALE: f32 = 1.2f32;
        pub const CAP_BULGE: f32 = 2.3f32;
        pub const CAP_REACH: f32 = 17.9f32;
        pub const MIN_SCALE: f32 = 0.5f32;
        // MossyGlowshroomFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // MossyGlowshroomFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // MossyGlowshroomFeature.grow(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // MossyGlowshroomFeature.generate(3): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `PythadendronTreeFeature` (org/betterx/betterend/world/features/trees/PythadendronTreeFeature.java)
    pub mod pythadendron_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // IGNORE: not translated, it uses Minecraft's `BlockState`.
        // POST: not translated, it uses `PosInfo` (org.betterx.bclib.sdf).
        pub const LEAF_BALL_BULGE: f32 = 4.5f32;
        // PythadendronTreeFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PythadendronTreeFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PythadendronTreeFeature.branch(10): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PythadendronTreeFeature.leavesBall(5): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `TenaneaFeature` (org/betterx/betterend/world/features/trees/TenaneaFeature.java)
    pub mod tenanea_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DIRECTIONS: not translated, it uses Minecraft's `Direction`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // IGNORE: not translated, it uses Minecraft's `BlockState`.
        // SPLINE: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        pub const LEAF_BALL_BULGE: f32 = 3.5f32;
        // TenaneaFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // TenaneaFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // TenaneaFeature.leavesBall(7): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `UmbrellaTreeFeature` (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java)
    pub mod umbrella_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // REPLACE: not translated, it uses Minecraft's `BlockState`.
        // SPLINE: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        // ROOT: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        pub const MEMBRANE_BULGE: f32 = 1.1f32;
        // UmbrellaTreeFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // UmbrellaTreeFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // UmbrellaTreeFeature.fitBranch(5): not translated, it uses Minecraft's `BlockPos`.
        // UmbrellaTreeFeature.makeRoots(6): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // UmbrellaTreeFeature.makeMembrane(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // UmbrellaTreeFeature.makeFruits(5): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
    }

    /// `UmbrellaTreeFeature.Center` (org/betterx/betterend/world/features/trees/UmbrellaTreeFeature.java)
    pub mod umbrella_tree_feature_center {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Center {
            pub px: f64,
            pub py: f64,
            pub pz: f64,
            pub radius: f32,
        }
        impl Center {
            pub fn new(x: f64, y: f64, z: f64, radius: f32) -> Self {
                let mut s = Self { px: x, py: y, pz: z, radius: radius };
                s.px = x;
                s.py = y;
                s.pz = z;
                s.radius = radius;
                s
            }
            // UmbrellaTreeFeature.Center.distance(2): not translated, it calls `MHelper` (org.betterx.bclib.util).length.
        }
    }

    /// `UnderwaterPlantFeature` (org/betterx/betterend/world/features/UnderwaterPlantFeature.java)
    pub mod underwater_plant_feature {
        use super::*;
        // The class itself isn't translated: it extends `UnderwaterPlantScatter`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // UnderwaterPlantFeature.UnderwaterPlantFeature(1): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // UnderwaterPlantFeature.codec(0): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // UnderwaterPlantFeature.canGenerate(6): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
        // UnderwaterPlantFeature.generate(4): not translated, its class isn't translated (it extends `UnderwaterPlantScatter`).
    }

    /// `UnderwaterPlantScatter` (org/betterx/betterend/world/features/UnderwaterPlantScatter.java)
    pub mod underwater_plant_scatter {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // UnderwaterPlantScatter.UnderwaterPlantScatter(1): not translated, its class isn't translated (it's generic).
        // UnderwaterPlantScatter.getCenterGround(3): not translated, its class isn't translated (it's generic).
        // UnderwaterPlantScatter.canGenerate(6): not translated, its class isn't translated (it's generic).
        // UnderwaterPlantScatter.canSpawn(3): not translated, its class isn't translated (it's generic).
        // UnderwaterPlantScatter.getGroundPlant(3): not translated, its class isn't translated (it's generic).
        // UnderwaterPlantScatter.getYOffset(0): not translated, its class isn't translated (it's generic).
        // UnderwaterPlantScatter.getChance(0): not translated, its class isn't translated (it's generic).
        // UnderwaterPlantScatter.getGround(2): not translated, its class isn't translated (it's generic).
    }

    /// `VineFeature` (org/betterx/betterend/world/features/VineFeature.java)
    pub mod vine_feature {
        use super::*;
        // The class itself isn't translated: it extends `InvertedScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // VineFeature.VineFeature(1): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // VineFeature.codec(0): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // VineFeature.canGenerate(6): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // VineFeature.generate(4): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // VineFeature.canPlaceBlock(3): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // VineFeature.getTopState(0): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // VineFeature.getMiddleState(0): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
        // VineFeature.getBottomState(0): not translated, its class isn't translated (it extends `InvertedScatterFeature`).
    }

    /// `VineFeatureConfig` (org/betterx/betterend/world/features/VineFeatureConfig.java)
    pub mod vine_feature_config {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeatureConfig`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // VineFeatureConfig.VineFeatureConfig(2): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // VineFeatureConfig.VineFeatureConfig(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // VineFeatureConfig.getPlantState(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
    }

    /// `WallPlantFeature` (org/betterx/betterend/world/features/WallPlantFeature.java)
    pub mod wall_plant_feature {
        use super::*;
        // The class itself isn't translated: it extends `WallScatterFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WallPlantFeature.WallPlantFeature(1): not translated, its class isn't translated (it extends `WallScatterFeature`).
        // WallPlantFeature.codec(0): not translated, its class isn't translated (it extends `WallScatterFeature`).
        // WallPlantFeature.canGenerate(5): not translated, its class isn't translated (it extends `WallScatterFeature`).
        // WallPlantFeature.generate(5): not translated, its class isn't translated (it extends `WallScatterFeature`).
    }

    /// `WallPlantFeatureConfig` (org/betterx/betterend/world/features/WallPlantFeatureConfig.java)
    pub mod wall_plant_feature_config {
        use super::*;
        // The class itself isn't translated: it extends `ScatterFeatureConfig`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WallPlantFeatureConfig.WallPlantFeatureConfig(2): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // WallPlantFeatureConfig.WallPlantFeatureConfig(2): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
        // WallPlantFeatureConfig.getPlantState(3): not translated, its class isn't translated (it extends `ScatterFeatureConfig`).
    }

    /// `WallPlantOnLogFeature` (org/betterx/betterend/world/features/WallPlantOnLogFeature.java)
    pub mod wall_plant_on_log_feature {
        use super::*;
        // The class itself isn't translated: it extends `WallPlantFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WallPlantOnLogFeature.WallPlantOnLogFeature(1): not translated, its class isn't translated (it extends `WallPlantFeature`).
        // WallPlantOnLogFeature.codec(0): not translated, its class isn't translated (it extends `WallPlantFeature`).
        // WallPlantOnLogFeature.canGenerate(5): not translated, its class isn't translated (it extends `WallPlantFeature`).
    }

    /// `WallScatterFeature` (org/betterx/betterend/world/features/WallScatterFeature.java)
    pub mod wall_scatter_feature {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // DIR: not translated, it uses Minecraft's `Direction`.
        // WallScatterFeature.WallScatterFeature(1): not translated, its class isn't translated (it's generic).
        // WallScatterFeature.canGenerate(5): not translated, it has no body.
        // WallScatterFeature.generate(5): not translated, it has no body.
        // WallScatterFeature.place(4): not translated, its class isn't translated (it's generic).
        // WallScatterFeature.overLakeWater(2): not translated, it uses Minecraft's `WorldGenLevel`.
        // WallScatterFeature.shuffle(2): not translated, it uses Minecraft's `Direction`.
    }

    /// `EndCaveBiomeDecider` (org/betterx/betterend/world/generator/EndCaveBiomeDecider.java)
    pub mod end_cave_biome_decider {
        use super::*;
        // The class itself isn't translated: it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end). Its static parts are below.
        pub const CAVE_REGION_FREQ: f64 = 0.004f64;
        pub const CAVE_REGION_THRESHOLD: f64 = 0.1f64;
        // EndCaveBiomeDecider.EndCaveBiomeDecider(0): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.EndCaveBiomeDecider(1): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.canProvideFor(1): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.createInstance(1): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.pickerTag(0): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.createMap(2): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.createMap(1): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.suggestType(10): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.inCaveRegion(2): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.isLandSuggestion(1): not translated, it uses Minecraft's `TagKey`.
        // EndCaveBiomeDecider.caveTopY(2): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.columnHasCaves(2): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.canProvideBiome(1): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndCaveBiomeDecider.provideBiome(4): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
    }

    /// `EndLandBiomeDecider` (org/betterx/betterend/world/generator/EndLandBiomeDecider.java)
    pub mod end_land_biome_decider {
        use super::*;
        // The class itself isn't translated: it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end). Its static parts are below.
        // EndLandBiomeDecider.EndLandBiomeDecider(0): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndLandBiomeDecider.canProvideFor(1): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndLandBiomeDecider.createInstance(1): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndLandBiomeDecider.suggestType(10): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
        // EndLandBiomeDecider.canProvideBiome(1): not translated, its class isn't translated (it extends `BiomeDecider` (de.ambertation.wover.generator.api.biomesource.end)).
    }

    /// `GeneratorOptions` (org/betterx/betterend/world/generator/GeneratorOptions.java)
    pub mod generator_options {
        use super::*;
        pub static BIOME_SIZE_CAVES: JCell<i32> = JCell::new(0i32);
        pub static HAS_PORTAL: JCell<bool> = JCell::new(false);
        pub static HAS_PILLARS: JCell<bool> = JCell::new(false);
        pub static HAS_DRAGON_FIGHTS: JCell<bool> = JCell::new(false);
        pub static CHANGE_CHORUS_PLANT: JCell<bool> = JCell::new(false);
        pub static NEW_GENERATOR: JCell<bool> = JCell::new(false);
        pub static GENERATE_CENTRAL_ISLAND: JCell<bool> = JCell::new(false);
        pub static GENERATE_OBSIDIAN_PLATFORM: JCell<bool> = JCell::new(false);
        pub static END_CITY_FAIL_CHANCE: JCell<i32> = JCell::new(0i32);
        pub static BIG_OPTIONS: LazyLock<JCell<Option<layer_options::LayerOptions>>> = LazyLock::new(|| JCell::new(None));
        pub static MEDIUM_OPTIONS: LazyLock<JCell<Option<layer_options::LayerOptions>>> = LazyLock::new(|| JCell::new(None));
        pub static SMALL_OPTIONS: LazyLock<JCell<Option<layer_options::LayerOptions>>> = LazyLock::new(|| JCell::new(None));
        pub static CHANGE_SPAWN: JCell<bool> = JCell::new(false);
        // spawn: not translated, it uses Minecraft's `BlockPos`.
        pub static REPLACE_PORTAL: JCell<bool> = JCell::new(false);
        pub static REPLACE_PILLARS: JCell<bool> = JCell::new(false);
        pub static DIRECT_SPIKE_HEIGHT: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct GeneratorOptions {
        }
        impl GeneratorOptions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GeneratorOptions.init(0): not translated, it uses `Object`.
        // GeneratorOptions.updatedValue(1): not translated, it uses `Object`.
        pub fn get_biome_size_caves() -> i32 {
            return generator_options::BIOME_SIZE_CAVES.get();
        }
        pub fn has_portal() -> bool {
            return generator_options::HAS_PORTAL.get();
        }
        pub fn has_pillars() -> bool {
            return generator_options::HAS_PILLARS.get();
        }
        pub fn has_dragon_fights() -> bool {
            return generator_options::HAS_DRAGON_FIGHTS.get();
        }
        pub fn change_chorus_plant() -> bool {
            return generator_options::CHANGE_CHORUS_PLANT.get();
        }
        pub fn use_new_generator() -> bool {
            return generator_options::NEW_GENERATOR.get();
        }
        pub fn generate_obsidian_platform() -> bool {
            return generator_options::GENERATE_OBSIDIAN_PLATFORM.get();
        }
        pub fn get_end_city_fail_chance() -> i32 {
            return generator_options::END_CITY_FAIL_CHANCE.get();
        }
        pub fn change_spawn() -> bool {
            return generator_options::CHANGE_SPAWN.get();
        }
        // GeneratorOptions.getSpawn(0): not translated, it uses Minecraft's `BlockPos`.
        pub fn replace_portal() -> bool {
            return generator_options::REPLACE_PORTAL.get();
        }
        pub fn replace_pillars() -> bool {
            return generator_options::REPLACE_PILLARS.get();
        }
        pub fn set_direct_spike_height() {
            generator_options::DIRECT_SPIKE_HEIGHT.set(true);
        }
        pub fn is_direct_spike_height() -> bool {
            let height: bool = generator_options::DIRECT_SPIKE_HEIGHT.get();
            generator_options::DIRECT_SPIKE_HEIGHT.set(false);
            return height;
        }
        pub fn has_central_island() -> bool {
            return generator_options::GENERATE_CENTRAL_ISLAND.get();
        }
    }

    /// `IslandLayer` (org/betterx/betterend/world/generator/IslandLayer.java)
    pub mod island_layer {
        use super::*;
        // The class itself isn't translated: it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator). Its static parts are below.
        // IslandLayer.IslandLayer(2): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.getSeed(2): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.updatePositions(3): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.getIsland(1): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.getRelativeDistance(5): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.calculateSDF(3): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.getDensity(3): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.getDensity(4): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.clearCache(0): not translated, its class isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // IslandLayer.makeCone(4): not translated, it uses `SDF` (org.betterx.bclib.sdf).
    }

    /// `IslandTerrainDensityFunction` (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java)
    pub mod island_terrain_density_function {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // IslandTerrainDensityFunction.IslandTerrainDensityFunction(4): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.cellSizeXz(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.cellSizeY(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.noiseMinY(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // IslandTerrainDensityFunction.noiseHeight(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `IslandTerrainDensityFunction.Sampler` (org/betterx/betterend/world/generator/IslandTerrainDensityFunction.java)
    pub mod island_terrain_density_function_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // IslandTerrainDensityFunction.Sampler.Sampler(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.newColumn(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.fillColumn(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.valueAt(2): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.cellSizeXz(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.cellSizeY(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.noiseMinY(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // IslandTerrainDensityFunction.Sampler.noiseHeight(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `LayerOptions` (org/betterx/betterend/world/generator/LayerOptions.java)
    pub mod layer_options {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LayerOptions {
            pub distance: f32,
            pub scale: f32,
            pub raw_coverage: f32,
            pub coverage: f32,
            pub average_height: f32,
            pub height_variation: f32,
            pub min_y: f32,
            pub max_y: f32,
            pub center_dist: i64,
            pub has_central_island: bool,
        }
        impl LayerOptions {
            pub fn new_5(distance: f32, scale: f32, average_height: f32, height_variation: f32, has_central: bool) -> Self {
                let mut s = Self { distance: distance, scale: scale, raw_coverage: 0.0f32, coverage: 0.0f32, average_height: average_height, height_variation: height_variation, min_y: 0.0f32, max_y: 0.0f32, center_dist: 0i64, has_central_island: has_central };
                s.distance = distance;
                s.scale = scale;
                s.average_height = average_height;
                s.height_variation = height_variation;
                s.raw_coverage = 0.5f32;
                s.coverage = s.clamp_coverage(0.5f32);
                s.has_central_island = has_central;
                s.min_y = s.average_height - s.height_variation;
                s.max_y = s.average_height + s.height_variation;
                s.center_dist = ((((1000.0f32 / s.distance) as f64).floor() as i32) as i64);
                s
            }
            pub fn new_1(config: &JJson) -> Self {
                let mut s = Self { distance: 0.0f32, scale: 0.0f32, raw_coverage: 0.0f32, coverage: 0.0f32, average_height: 0.0f32, height_variation: 0.0f32, min_y: 0.0f32, max_y: 0.0f32, center_dist: 0i64, has_central_island: false };
                s.distance = s.clamp_distance(((config.get("distance")).expect("null")).as_float());
                s.scale = s.clamp_scale(((config.get("scale")).expect("null")).as_float());
                s.average_height = s.clamp_average_height(((config.get("average_height")).expect("null")).as_float());
                s.height_variation = s.clamp_variation(((config.get("height_variation")).expect("null")).as_float());
                s.raw_coverage = ((config.get("coverage")).expect("null")).as_float();
                s.coverage = s.clamp_coverage(s.raw_coverage);
                s.has_central_island = ((config.get("contains_central_island")).expect("null")).as_bool();
                s.min_y = s.average_height - s.height_variation;
                s.max_y = s.average_height + s.height_variation;
                s.center_dist = ((((1000.0f32 / s.distance) as f64).floor() as i32) as i64);
                s
            }
            pub fn clamp_distance(&self, value: f32) -> f32 {
                return j_clamp(value, 1.0f32, 8192.0f32);
            }
            pub fn clamp_scale(&self, value: f32) -> f32 {
                return j_clamp(value, 0.1f32, 1024.0f32);
            }
            pub fn clamp_coverage(&self, value: f32) -> f32 {
                return 0.9999f32 - (j_clamp(value, 0.0f32, 1.0f32) * 2.0f32);
            }
            pub fn clamp_average_height(&self, value: f32) -> f32 {
                return j_clamp(value, 0.0f32, 1.0f32);
            }
            pub fn clamp_variation(&self, value: f32) -> f32 {
                return j_clamp(value, 0.0f32, 1.0f32);
            }
            pub fn to_json(&self) -> JJson {
                let mut obj: JJson = JJson::object();
                obj.add("distance", JJson::float(self.clamp_distance(self.distance)));
                obj.add("scale", JJson::float(self.clamp_scale(self.scale)));
                obj.add("average_height", JJson::float(self.clamp_average_height(self.average_height)));
                obj.add("height_variation", JJson::float(self.clamp_variation(self.height_variation)));
                obj.add("coverage", JJson::float(self.raw_coverage));
                obj.add("contains_central_island", JJson::Bool(self.has_central_island));
                return obj.clone();
            }
        }
    }

    /// `TerrainBoolCache` (org/betterx/betterend/world/generator/TerrainBoolCache.java)
    pub mod terrain_bool_cache {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TerrainBoolCache {
            pub data: Vec<i8>,
        }
        impl TerrainBoolCache {
            pub fn new() -> Self {
                let mut s = Self { data: vec![0i8; 16384] };
                s
            }
            pub fn get_index(&self, x: i32, z: i32) -> i32 {
                return (x << 7) | z;
            }
            pub fn set_data(&mut self, x: i32, z: i32, value: i8) {
                self.data[(self.get_index(x & 127, z & 127)) as usize] = value;
            }
            pub fn get_data(&self, x: i32, z: i32) -> i8 {
                return self.data[(self.get_index(x & 127, z & 127)) as usize];
            }
        }
        pub fn scale_coordinate(value: i32) -> i32 {
            return value >> 7;
        }
    }

    /// `TerrainGenerator` (org/betterx/betterend/world/generator/TerrainGenerator.java)
    pub mod terrain_generator {
        use super::*;
        // TERRAIN_BOOL_CACHE_MAP: not translated, it uses `Point` (java.awt).
        // LOCKER: not translated, it uses `ReentrantLock` (java.util.concurrent.locks).
        // POS: not translated, it uses `Point` (java.awt).
        pub const SCALE_XZ: f64 = 8.0f64;
        pub const SCALE_Y: f64 = 4.0f64;
        pub static COEF: LazyLock<Option<Vec<f32>>> = LazyLock::new(|| None);
        // OFFS: not translated, it uses `Point` (java.awt).
        // largeIslands: not translated, it uses IslandLayer, which isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // mediumIslands: not translated, it uses IslandLayer, which isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        // smallIslands: not translated, it uses IslandLayer, which isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator)).
        pub static NOISE1: LazyLock<JCell<Option<open_simplex_noise::OpenSimplexNoise>>> = LazyLock::new(|| JCell::new(None));
        pub static NOISE2: LazyLock<JCell<Option<open_simplex_noise::OpenSimplexNoise>>> = LazyLock::new(|| JCell::new(None));
        // biomeSource: not translated, it uses Minecraft's `BiomeSource`.
        // config: not translated, it uses `WoverEndConfig` (de.ambertation.wover.generator.api.biomesource.end).
        // biomeResolver: not translated, it uses Minecraft's `BiomeResolver`.
        pub static NOISE_SEED: JCell<i64> = JCell::new(0i64);
        pub const DEFAULT_CELL_SIZE_XZ: i32 = 8;
        pub const DEFAULT_CELL_SIZE_Y: i32 = 4;
        // islandTerrain: not translated, it uses IslandTerrain, which isn't translated (it uses Minecraft's `DensityFunction`).
        #[derive(Clone, Debug)]
        pub struct TerrainGenerator {
        }
        impl TerrainGenerator {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TerrainGenerator.endConfigOf(1): not translated, it uses `WoverEndConfig` (de.ambertation.wover.generator.api.biomesource.end).
        // TerrainGenerator.disableTerrainGenerator(0): not translated, it uses TerrainGenerator.LOCKER, which isn't translated (it uses `ReentrantLock` (java.util.concurrent.locks)).
        // TerrainGenerator.initNoise(3): not translated, it uses Minecraft's `BiomeSource`.
        // TerrainGenerator.fillTerrainDensity(6): not translated, it uses TerrainGenerator.LOCKER, which isn't translated (it uses `ReentrantLock` (java.util.concurrent.locks)).
        // TerrainGenerator.computeColumnDensity(6): not translated, it uses TerrainGenerator.largeIslands, which isn't translated (it uses IslandLayer, which isn't translated (it uses `SDFRadialNoiseMap` (org.betterx.bclib.sdf.operator))).
        // TerrainGenerator.getSurfaceHeight(6): not translated, it uses TerrainGenerator.LOCKER, which isn't translated (it uses `ReentrantLock` (java.util.concurrent.locks)).
        // TerrainGenerator.getAverageDepth(2): not translated, it uses TerrainGenerator.biomeSource, which isn't translated (it uses Minecraft's `BiomeSource`).
        // TerrainGenerator.getBiomeData(3): not translated, it uses `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource).
        // TerrainGenerator.isLand(3): not translated, it uses TerrainGenerator.LOCKER, which isn't translated (it uses `ReentrantLock` (java.util.concurrent.locks)).
        // TerrainGenerator.onServerLevelInit(3): not translated, it uses Minecraft's `ServerLevel`.
        // TerrainGenerator.makeObsidianPlatform(2): not translated, it uses Minecraft's `ServerLevelAccessor`.
        // TerrainGenerator.withIslandTerrain(2): not translated, it uses Minecraft's `DensityFunction`.
        // TerrainGenerator.islandCellSize(1): not translated, it uses Minecraft's `NoiseGeneratorSettings`.
        // TerrainGenerator.islandTerrainFor(2): not translated, its body couldn't be read: an anonymous class.
    }

    /// `TerrainGenerator.IslandTerrain` (org/betterx/betterend/world/generator/TerrainGenerator.java)
    pub mod terrain_generator_island_terrain {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `DensityFunction`. Its static parts are below.
        // TerrainGenerator.IslandTerrain.IslandTerrain(5): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // TerrainGenerator.IslandTerrain.source(0): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // TerrainGenerator.IslandTerrain.noiseSettings(0): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // TerrainGenerator.IslandTerrain.withIslands(0): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // TerrainGenerator.IslandTerrain.cellSizeXz(0): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // TerrainGenerator.IslandTerrain.cellSizeY(0): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
    }

    /// `EndBridgeStructure` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java)
    pub mod end_bridge_structure {
        use super::*;
        // The class itself isn't translated: it extends `FeatureBaseStructure`. Its static parts are below.
        pub const SCAN_DIRECTIONS: i32 = 16;
        pub const SCAN_MIN_RADIUS: i32 = 16;
        pub const SCAN_MAX_RADIUS: i32 = 96;
        pub const SCAN_RADIUS_STEP: i32 = 8;
        pub const TERRAIN_MIN_Y: i32 = 30;
        pub const TERRAIN_MAX_Y: i32 = 90;
        pub const VOID_MARGIN: i32 = 5;
        pub const MIN_SPAN: i32 = 24;
        pub const MAX_SPAN: i32 = 96;
        pub const MAX_HEIGHT_DELTA: i32 = 12;
        pub static MIN_DIRECTION_SEPARATION: LazyLock<f64> = LazyLock::new(|| std::f64::consts::PI / 2.0f64);
        // EndBridgeStructure.EndBridgeStructure(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // EndBridgeStructure.type(0): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // EndBridgeStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // EndBridgeStructure.generatePieces(2): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        pub fn has_terrain(base_height: i32, min_y: i32) -> bool {
            return ((base_height > (min_y + 5)) && (base_height >= 30)) && (base_height <= 90);
        }
        pub fn angular_separation(a: f64, b: f64) -> f64 {
            let mut diff: f64 = ((a - b).abs()) % (std::f64::consts::PI * 2.0f64);
            if diff > std::f64::consts::PI {
                diff = (std::f64::consts::PI * 2.0f64) - diff;
            }
            return diff;
        }
    }

    /// `EndBridgeStructure.Anchor` (org/betterx/betterend/world/structures/features/EndBridgeStructure.java)
    pub mod end_bridge_structure_anchor {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Anchor {
            pub x: i32,
            pub y: i32,
            pub z: i32,
            pub angle: f64,
            pub island: bool,
        }
        impl Anchor {
            pub fn new(x: i32, y: i32, z: i32, angle: f64, island: bool) -> Self {
                let mut s = Self { x: x, y: y, z: z, angle: angle, island: island };
                s.x = x;
                s.y = y;
                s.z = z;
                s.angle = angle;
                s.island = island;
                s
            }
            pub fn dist_sq_to(&self, o: &end_bridge_structure_anchor::Anchor) -> i32 {
                let dx: i32 = self.x - o.x;
                let dz: i32 = self.z - o.z;
                return (dx * dx) + (dz * dz);
            }
            pub fn x(&self) -> i32 {
                return self.x;
            }
            pub fn y(&self) -> i32 {
                return self.y;
            }
            pub fn z(&self) -> i32 {
                return self.z;
            }
            pub fn angle(&self) -> f64 {
                return self.angle;
            }
            pub fn island(&self) -> bool {
                return self.island;
            }
        }
    }

    /// `EndLakeNormalStructure` (org/betterx/betterend/world/structures/features/EndLakeNormalStructure.java)
    pub mod end_lake_normal_structure {
        use super::*;
        // The class itself isn't translated: it extends `EndLakeStructure`. Its static parts are below.
        // EndLakeNormalStructure.EndLakeNormalStructure(1): not translated, its class isn't translated (it extends `EndLakeStructure`).
        // EndLakeNormalStructure.type(0): not translated, its class isn't translated (it extends `EndLakeStructure`).
    }

    /// `EndLakeRareStructure` (org/betterx/betterend/world/structures/features/EndLakeRareStructure.java)
    pub mod end_lake_rare_structure {
        use super::*;
        // The class itself isn't translated: it extends `EndLakeStructure`. Its static parts are below.
        // EndLakeRareStructure.EndLakeRareStructure(1): not translated, its class isn't translated (it extends `EndLakeStructure`).
        // EndLakeRareStructure.type(0): not translated, its class isn't translated (it extends `EndLakeStructure`).
    }

    /// `EndLakeStructure` (org/betterx/betterend/world/structures/features/EndLakeStructure.java)
    pub mod end_lake_structure {
        use super::*;
        // The class itself isn't translated: it extends `FeatureBaseStructure`. Its static parts are below.
        // EndLakeStructure.EndLakeStructure(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // EndLakeStructure.type(0): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // EndLakeStructure.generatePieces(2): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
    }

    /// `EndSulphuricCaveStructure` (org/betterx/betterend/world/structures/features/EndSulphuricCaveStructure.java)
    pub mod end_sulphuric_cave_structure {
        use super::*;
        // The class itself isn't translated: it extends `FeatureBaseStructure`. Its static parts are below.
        // EndSulphuricCaveStructure.EndSulphuricCaveStructure(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // EndSulphuricCaveStructure.type(0): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // EndSulphuricCaveStructure.generatePieces(2): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
    }

    /// `EternalPortalStructure` (org/betterx/betterend/world/structures/features/EternalPortalStructure.java)
    pub mod eternal_portal_structure {
        use super::*;
        // The class itself isn't translated: it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures). Its static parts are below.
        // EternalPortalStructure.EternalPortalStructure(1): not translated, its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)).
        // EternalPortalStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)).
        // EternalPortalStructure.type(0): not translated, its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)).
        // EternalPortalStructure.erosion(1): not translated, its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)).
        // EternalPortalStructure.cover(1): not translated, its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)).
        // EternalPortalStructure.alwaysBroken(0): not translated, its class isn't translated (it extends `TemplateStructure` (org.betterx.bclib.api.v2.levelgen.structures)).
        // EternalPortalStructure.cfg(4): not translated, it uses Config, which isn't translated (it's generic).
    }

    /// `FeatureBaseStructure` (org/betterx/betterend/world/structures/features/FeatureBaseStructure.java)
    pub mod feature_base_structure {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Structure`. Its static parts are below.
        // AIR: not translated, it uses Minecraft's `BlockState`.
        pub const MIN_TERRAIN_Y: i32 = 10;
        pub const HEIGHT_CACHE_SIZE: i32 = 16384;
        // HEIGHT_CACHE: not translated, it uses Minecraft's `RandomState`.
        // FeatureBaseStructure.getCachedFirstOccupiedHeight(5): not translated, its body couldn't be read: an anonymous class.
        // FeatureBaseStructure.FeatureBaseStructure(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // FeatureBaseStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // FeatureBaseStructure.findVoidGenerationPoint(2): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // FeatureBaseStructure.getNoiseBiome(5): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // FeatureBaseStructure.generatePieces(2): not translated, it has no body.
        // FeatureBaseStructure.anchorColumn(1): not translated, it uses Minecraft's `BlockPos`.
        // FeatureBaseStructure.mayStartInBiome(2): not translated, it uses Minecraft's `GenerationContext`.
        // FeatureBaseStructure.getGenerationHeight(4): not translated, it uses Minecraft's `BlockPos`.
    }

    /// `GiantIceStarStructure` (org/betterx/betterend/world/structures/features/GiantIceStarStructure.java)
    pub mod giant_ice_star_structure {
        use super::*;
        // The class itself isn't translated: it extends `SDFStructureFeature`. Its static parts are below.
        pub const MIN_SIZE: f32 = 20.0f32;
        pub const MAX_SIZE: f32 = 35.0f32;
        pub const MIN_COUNT: i32 = 25;
        pub const MAX_COUNT: i32 = 40;
        // GiantIceStarStructure.GiantIceStarStructure(1): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // GiantIceStarStructure.type(0): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // GiantIceStarStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // GiantIceStarStructure.getSDF(2): not translated, it uses `SDF` (org.betterx.bclib.sdf).
        // GiantIceStarStructure.getFibonacciPoints(1): not translated, it uses `MHelper` (org.betterx.bclib.util).PHI.
        // GiantIceStarStructure.generatePieces(2): not translated, its class isn't translated (it extends `SDFStructureFeature`).
    }

    /// `GiantMossyGlowshroomStructure` (org/betterx/betterend/world/structures/features/GiantMossyGlowshroomStructure.java)
    pub mod giant_mossy_glowshroom_structure {
        use super::*;
        // The class itself isn't translated: it extends `SDFStructureFeature`. Its static parts are below.
        // GiantMossyGlowshroomStructure.GiantMossyGlowshroomStructure(1): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // GiantMossyGlowshroomStructure.generatePieces(2): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // GiantMossyGlowshroomStructure.getSDF(2): not translated, it uses `SDF` (org.betterx.bclib.sdf).
        // GiantMossyGlowshroomStructure.type(0): not translated, its class isn't translated (it extends `SDFStructureFeature`).
    }

    /// `MegaLakeSmallStructure` (org/betterx/betterend/world/structures/features/MegaLakeSmallStructure.java)
    pub mod mega_lake_small_structure {
        use super::*;
        // The class itself isn't translated: it extends `FeatureBaseStructure`. Its static parts are below.
        // MegaLakeSmallStructure.MegaLakeSmallStructure(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // MegaLakeSmallStructure.type(0): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // MegaLakeSmallStructure.generatePieces(2): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
    }

    /// `MegaLakeStructure` (org/betterx/betterend/world/structures/features/MegaLakeStructure.java)
    pub mod mega_lake_structure {
        use super::*;
        // The class itself isn't translated: it extends `FeatureBaseStructure`. Its static parts are below.
        // MegaLakeStructure.MegaLakeStructure(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // MegaLakeStructure.type(0): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // MegaLakeStructure.generatePieces(2): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
    }

    /// `MountainStructure` (org/betterx/betterend/world/structures/features/MountainStructure.java)
    pub mod mountain_structure {
        use super::*;
        // The class itself isn't translated: it extends `FeatureBaseStructure`. Its static parts are below.
        // MountainStructure.MountainStructure(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // MountainStructure.type(0): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // MountainStructure.generatePieces(2): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
    }

    /// `PaintedMountainStructure` (org/betterx/betterend/world/structures/features/PaintedMountainStructure.java)
    pub mod painted_mountain_structure {
        use super::*;
        // The class itself isn't translated: it extends `FeatureBaseStructure`. Its static parts are below.
        // VARIANTS: not translated, it uses Minecraft's `BlockState`.
        // PaintedMountainStructure.PaintedMountainStructure(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // PaintedMountainStructure.type(0): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // PaintedMountainStructure.generatePieces(2): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
    }

    /// `SDFStructureFeature` (org/betterx/betterend/world/structures/features/SDFStructureFeature.java)
    pub mod sdf_structure_feature {
        use super::*;
        // The class itself isn't translated: it extends `FeatureBaseStructure`. Its static parts are below.
        // SDFStructureFeature.SDFStructureFeature(1): not translated, its class isn't translated (it extends `FeatureBaseStructure`).
        // SDFStructureFeature.generatePieces(3): not translated, it uses Minecraft's `StructurePiecesBuilder`.
    }

    /// `SmallIslandStructure` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java)
    pub mod small_island_structure {
        use super::*;
        // The class itself isn't translated: it extends `SDFStructureFeature`. Its static parts are below.
        pub const MIN_RADIUS: i32 = 6;
        pub const MAX_RADIUS: i32 = 12;
        pub const WATERFALL_MIN_RADIUS: i32 = 12;
        pub const WATERFALL_MAX_RADIUS: i32 = 17;
        // END_STONE: not translated, it uses Minecraft's `BlockState`.
        // END_MOSS: not translated, it uses Minecraft's `BlockState`.
        // SANGNUM: not translated, it uses Minecraft's `BlockState`.
        // PALLIDIUM_FULL: not translated, it uses Minecraft's `BlockState`.
        // PALLIDIUM_HEAVY: not translated, it uses Minecraft's `BlockState`.
        // PALLIDIUM_THIN: not translated, it uses Minecraft's `BlockState`.
        // PALLIDIUM_TINY: not translated, it uses Minecraft's `BlockState`.
        // UMBRALITH: not translated, it uses Minecraft's `BlockState`.
        // UNDERSIDE_PATCH: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        pub const PATCH_FREQ: f64 = 0.14f64;
        pub const PATCH_THRESHOLD: f64 = 0.05f64;
        // SmallIslandStructure.SmallIslandStructure(1): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // SmallIslandStructure.type(0): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // SmallIslandStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // SmallIslandStructure.drawGeometry(2): not translated, it calls `MHelper` (org.betterx.bclib.util).randRange.
        // SmallIslandStructure.getSDF(3): not translated, it uses `SDF` (org.betterx.bclib.sdf).
        // SmallIslandStructure.waterfallCoat(2): not translated, it uses Minecraft's `BlockState`.
        // SmallIslandStructure.inUndersidePatch(4): not translated, it uses Minecraft's `BlockPos`.
        // SmallIslandStructure.hangStalactite(2): not translated, it uses `PosInfo` (org.betterx.bclib.sdf).
        // SmallIslandStructure.flowerCoat(2): not translated, it uses Minecraft's `BlockState`.
        // SmallIslandStructure.hangVineStrand(2): not translated, it uses `PosInfo` (org.betterx.bclib.sdf).
        // SmallIslandStructure.pickVine(1): not translated, it uses Minecraft's `Block`.
        // SmallIslandStructure.makeCone(4): not translated, it uses `SDF` (org.betterx.bclib.sdf).
        // SmallIslandStructure.generatePieces(2): not translated, its class isn't translated (it extends `SDFStructureFeature`).
        // SmallIslandStructure.overlapsNativeLand(4): not translated, it calls TerrainGenerator.isLand, which isn't translated.
        // SmallIslandStructure.terrainMaxHeight(1): not translated, it uses Minecraft's `GenerationContext`.
        // SmallIslandStructure.isFlowerIslets(4): not translated, its class isn't translated (it extends `SDFStructureFeature`).
    }

    /// `SmallIslandStructure.IslandGeometry` (org/betterx/betterend/world/structures/features/SmallIslandStructure.java)
    pub mod small_island_structure_island_geometry {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct IslandGeometry {
            pub radius: f32,
            pub scale_x: f32,
            pub scale: f32,
            pub scale_z: f32,
            pub noise_intensity: f32,
            pub noise_seed: i32,
        }
        impl IslandGeometry {
            pub fn new(radius: f32, scale_x: f32, scale: f32, scale_z: f32, noise_intensity: f32, noise_seed: i32) -> Self {
                let mut s = Self { radius: radius, scale_x: scale_x, scale: scale, scale_z: scale_z, noise_intensity: noise_intensity, noise_seed: noise_seed };
                s.radius = radius;
                s.scale_x = scale_x;
                s.scale = scale;
                s.scale_z = scale_z;
                s.noise_intensity = noise_intensity;
                s.noise_seed = noise_seed;
                s
            }
            pub fn radius(&self) -> f32 {
                return self.radius;
            }
            pub fn scale_x(&self) -> f32 {
                return self.scale_x;
            }
            pub fn scale(&self) -> f32 {
                return self.scale;
            }
            pub fn scale_z(&self) -> f32 {
                return self.scale_z;
            }
            pub fn noise_intensity(&self) -> f32 {
                return self.noise_intensity;
            }
            pub fn noise_seed(&self) -> i32 {
                return self.noise_seed;
            }
        }
    }

    /// `BasePiece` (org/betterx/betterend/world/structures/piece/BasePiece.java)
    pub mod base_piece {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StructurePiece`. Its static parts are below.
        // BasePiece.BasePiece(3): not translated, its class isn't translated (it extends Minecraft's `StructurePiece`).
        // BasePiece.BasePiece(2): not translated, its class isn't translated (it extends Minecraft's `StructurePiece`).
        // BasePiece.fromNbt(1): not translated, it has no body.
        // BasePiece.worldgenSurfaceHeight(3): not translated, its class isn't translated (it extends Minecraft's `StructurePiece`).
        // BasePiece.setChunkBlock(3): not translated, it uses Minecraft's `ChunkAccess`.
        // BasePiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `StructurePiece`).
        // BasePiece.addAdditionalSaveData(2): not translated, its class isn't translated (it extends Minecraft's `StructurePiece`).
    }

    /// `CavePiece` (org/betterx/betterend/world/structures/piece/CavePiece.java)
    pub mod cave_piece {
        use super::*;
        // The class itself isn't translated: it extends `BasePiece`. Its static parts are below.
        // CavePiece.CavePiece(3): not translated, its class isn't translated (it extends `BasePiece`).
        // CavePiece.CavePiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // CavePiece.postProcess(7): not translated, its class isn't translated (it extends `BasePiece`).
        // CavePiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `BasePiece`).
        // CavePiece.fromNbt(1): not translated, its class isn't translated (it extends `BasePiece`).
        // CavePiece.makeBoundingBox(0): not translated, its class isn't translated (it extends `BasePiece`).
    }

    /// `CrystalMountainPiece` (org/betterx/betterend/world/structures/piece/CrystalMountainPiece.java)
    pub mod crystal_mountain_piece {
        use super::*;
        // The class itself isn't translated: it extends `MountainPiece`. Its static parts are below.
        pub const CRYSTAL_END_STONE_CRYSTAL_CHANCE: f32 = 0.03f32;
        pub const CRYSTAL_MOSS_CRYSTAL_CHANCE: f32 = 0.02f32;
        pub const CRYSTAL_BIG_MAX_COUNT: i32 = 6;
        pub const CROWN_MIN_COUNT: i32 = 3;
        pub const CROWN_MAX_COUNT: i32 = 5;
        pub const CROWN_SEARCH_RADIUS: i32 = 8;
        pub const CROWN_SPREAD: i32 = 6;
        pub const CROWN_MIN_PEAK_HEIGHT: i32 = 12;
        // CrystalMountainPiece.CrystalMountainPiece(5): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.CrystalMountainPiece(2): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.fromNbt(1): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.postProcess(7): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.crystalHeight(3): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.placeCrown(3): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.findPeak(0): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.placeMountain(4): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.crystal(7): not translated, its class isn't translated (it extends `MountainPiece`).
        // CrystalMountainPiece.crystal(8): not translated, its class isn't translated (it extends `MountainPiece`).
    }

    /// `EndBridgeGeometry` (org/betterx/betterend/world/structures/piece/EndBridgeGeometry.java)
    pub mod end_bridge_geometry {
        use super::*;
        pub const LANDING_LEN: i32 = 4;
        pub const DECK_HALF_WIDTH: f64 = 1.5f64;
        pub const LANDING_HALF_WIDTH: f64 = 2.5f64;
        #[derive(Clone, Debug)]
        pub struct EndBridgeGeometry {
            pub ax: f64,
            pub az: f64,
            pub ay: f64,
            pub by: f64,
            pub dx: f64,
            pub dz: f64,
            pub len_sq: f64,
            pub len: f64,
            pub rise: f64,
        }
        impl EndBridgeGeometry {
            // EndBridgeGeometry.EndBridgeGeometry(2): not translated, it uses Minecraft's `BlockPos`.
            pub fn length_sq(&self) -> f64 {
                return self.len_sq;
            }
            pub fn t(&self, x: i32, z: i32) -> f64 {
                return j_clamp((((((x as f64) + 0.5f64) - self.ax) * self.dx) + ((((z as f64) + 0.5f64) - self.az) * self.dz)) / self.len_sq, 0.0f64, 1.0f64);
            }
            pub fn perp(&self, x: i32, z: i32, t: f64) -> f64 {
                let px: f64 = ((x as f64) + 0.5f64) - (self.ax + (t * self.dx));
                let pz: f64 = ((z as f64) + 0.5f64) - (self.az + (t * self.dz));
                return ((px * px) + (pz * pz)).sqrt();
            }
            pub fn along(&self, t: f64) -> f64 {
                return t * self.len;
            }
            pub fn is_landing(&self, t: f64) -> bool {
                let along: f64 = self.along(t);
                return (along.min(self.len - along)) < 4.0f64;
            }
            pub fn half_width(&self, t: f64) -> f64 {
                return if self.is_landing(t) { 2.5f64 } else { 1.5f64 };
            }
            pub fn is_deck(&self, x: i32, z: i32) -> bool {
                let t: f64 = self.t(x, z);
                return (self.perp(x, z, t)) <= ((self.half_width(t)) + 0.5f64);
            }
            pub fn is_edge(&self, x: i32, z: i32) -> bool {
                return (((!(self.is_deck(x - 1, z))) || (!(self.is_deck(x + 1, z)))) || (!(self.is_deck(x, z - 1)))) || (!(self.is_deck(x, z + 1)));
            }
            pub fn deck_y(&self, t: f64) -> i32 {
                return ((j_lerp(t, self.ay, self.by) + (((self.rise * 4.0f64) * t) * (1.0f64 - t))) + 0.5f64).floor() as i32;
            }
        }
    }

    /// `EndBridgePiece` (org/betterx/betterend/world/structures/piece/EndBridgePiece.java)
    pub mod end_bridge_piece {
        use super::*;
        // The class itself isn't translated: it extends `BasePiece`. Its static parts are below.
        // DECK: not translated, it uses Minecraft's `BlockState`.
        // CRACKED: not translated, it uses Minecraft's `BlockState`.
        // WEATHERED: not translated, it uses Minecraft's `BlockState`.
        // RAILING: not translated, it uses Minecraft's `BlockState`.
        pub const PILLAR_SPACING: i32 = 8;
        pub const PILLAR_MAX_DROP: i32 = 12;
        pub const WEATHERED_CHANCE: f32 = 0.1f32;
        pub const CRACKED_CHANCE: f32 = 0.15f32;
        pub const RAILING_GAP_CHANCE: f32 = 0.2f32;
        pub const EROSION_CHANCE: f32 = 0.05f32;
        // EndBridgePiece.EndBridgePiece(3): not translated, its class isn't translated (it extends `BasePiece`).
        // EndBridgePiece.EndBridgePiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // EndBridgePiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `BasePiece`).
        // EndBridgePiece.fromNbt(1): not translated, its class isn't translated (it extends `BasePiece`).
        // EndBridgePiece.makeBoundingBox(0): not translated, its class isn't translated (it extends `BasePiece`).
        // EndBridgePiece.isReplaceable(1): not translated, it uses Minecraft's `BlockState`.
        // EndBridgePiece.isTerrain(1): not translated, it uses Minecraft's `BlockState`.
        // EndBridgePiece.deckMaterial(1): not translated, its class isn't translated (it extends `BasePiece`).
        // EndBridgePiece.postProcess(7): not translated, its class isn't translated (it extends `BasePiece`).
    }

    /// `EndLakePiece` (org/betterx/betterend/world/structures/piece/EndLakePiece.java)
    pub mod end_lake_piece {
        use super::*;
        // The class itself isn't translated: it extends `BasePiece`. Its static parts are below.
        // END_STONE: not translated, it uses Minecraft's `BlockState`.
        // AIR: not translated, it uses Minecraft's `BlockState`.
        // WATER: not translated, it uses Minecraft's `BlockState`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        pub const SHORE_DUST_CHANCE: f32 = 0.05f32;
        pub const SHORE_WIDTH: i32 = 8;
        pub const OUTER_WATER_WEIGHT_MIN: f64 = 0.1f64;
        pub const OUTER_WATER_WEIGHT_MAX: f64 = 0.3f64;
        pub const MAX_AIR_FILL: i32 = 4;
        pub const GROUND_SEARCH_DEPTH: i32 = 24;
        pub const BRIDGE_DEPTH: i32 = 8;
        pub const SHELL_DEPTH: i32 = 4;
        pub static NO_GROUND: LazyLock<i32> = LazyLock::new(|| i32::MIN);
        pub const FLUID_BELOW: i32 = -2147483647;
        // EndLakePiece.EndLakePiece(6): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.EndLakePiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.fromNbt(1): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.makeBoundingBox(0): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.borderMaterial(1): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.sampleSurface(1): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.shoreMaterial(2): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.canReplace(1): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.groundColumn(7): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.findGroundBelow(3): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.fillDownToGround(3): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.fillBelowWater(3): not translated, its class isn't translated (it extends `BasePiece`).
        // EndLakePiece.postProcess(7): not translated, its class isn't translated (it extends `BasePiece`).
    }

    /// `LakeFloorLife` (org/betterx/betterend/world/structures/piece/LakeFloorLife.java)
    pub mod lake_floor_life {
        use super::*;
        // CLUSTER: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        pub const LIFE_PER_COLUMN: f64 = 0.7f64;
        pub const BIOME_TOPUP: f64 = 0.2f64;
        // plants: not translated, it uses Minecraft's `BlockState`.
        pub static WEIGHTS: LazyLock<JCell<Option<Vec<i32>>>> = LazyLock::new(|| JCell::new(None));
        pub const LOTUS: i32 = -1;
        #[derive(Clone, Debug)]
        pub struct LakeFloorLife {
        }
        impl LakeFloorLife {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LakeFloorLife.init(0): not translated, it uses LakeFloorLife.plants, which isn't translated (it uses Minecraft's `BlockState`).
        // LakeFloorLife.isLakeBiome(1): not translated, it uses Minecraft's `Holder`.
        // LakeFloorLife.place(7): not translated, it uses Minecraft's `WorldGenLevel`.
        pub fn hash64(wx: i32, wz: i32, seed: i64, salt: i64) -> i64 {
            let mut h: i64 = (((wx as i64) * (-7046029254386353131i64)) ^ ((wz as i64) * (-4417276706812531889i64))) ^ ((seed + salt) * 1609587929392839161i64);
            h ^= ((h as u64) >> 31) as i64;
            h *= -4658895280553007687i64;
            return h ^ (((h as u64) >> 29) as i64);
        }
        pub fn hash01(wx: i32, wz: i32, seed: i64, salt: i64) -> f64 {
            return ((((((lake_floor_life::hash64(wx, wz, seed, salt) as u64) >> 11) as i64) as f32) * (1.110223e-16f32)) as f64);
        }
    }

    /// `LakePiece` (org/betterx/betterend/world/structures/piece/LakePiece.java)
    pub mod lake_piece {
        use super::*;
        // The class itself isn't translated: it extends `BasePiece`. Its static parts are below.
        // ENDSTONE: not translated, it uses Minecraft's `BlockState`.
        // WATER: not translated, it uses Minecraft's `BlockState`.
        pub const SHORE_WIDTH: i32 = 16;
        pub const GROUND_SEARCH_DEPTH: i32 = 32;
        pub const BRIDGE_DEPTH: i32 = 8;
        pub const SHELL_DEPTH: i32 = 4;
        pub const SKIRT_FLUSH: f64 = 3.0f64;
        pub const MAX_BANK_DISTANCE: i32 = 72;
        pub const BANK_MAX_RAISE: i32 = 5;
        pub const DIST_RELAX: i32 = 6;
        pub const DIKE_KEEP: i32 = 3;
        // rimPlants: not translated, it uses Minecraft's `BlockState`.
        pub static RIM_WEIGHTS: LazyLock<JCell<Option<Vec<i32>>>> = LazyLock::new(|| JCell::new(None));
        pub static RIM_SOIL_ONLY: LazyLock<JCell<Option<Vec<bool>>>> = LazyLock::new(|| JCell::new(None));
        pub const RIM_FUR_CHANCE: f64 = 0.35f64;
        pub static NO_GROUND: LazyLock<i32> = LazyLock::new(|| i32::MIN);
        pub const FLUID_BELOW: i32 = -2147483647;
        pub const MAX_AIR_FILL: i32 = 4;
        pub const SHORE_PLANTS_DENSE: f64 = 4.0f64;
        pub const SHORE_PLANTS_END: f64 = 7.0f64;
        pub const SHORE_PLANTS_TAIL: f64 = 0.04f64;
        pub const SHORE_PLANTS_TAIL_FULL: f64 = 12.0f64;
        pub const SHORE_PLANTS_TAIL_END: f64 = 16.0f64;
        pub const SHORE_MOSS_PATCH: f64 = 0.6f64;
        // LakePiece.rimPlants(0): not translated, it uses Minecraft's `BlockState`.
        // LakePiece.LakePiece(6): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.LakePiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.fromNbt(1): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.biomeMaterial(1): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.skirt(0): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.shorePatch(4): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.postProcess(7): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.placeShorePlants(7): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.groundColumn(2): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.cutBack(4): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.nearFluid(3): not translated, it uses Minecraft's `ChunkAccess`.
        // LakePiece.edgeFunction(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.columnMaterials(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.shoreDistances(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.fillDownToGround(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.findGroundBelow(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.fillBelow(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.placeRimPlant(4): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.fixWater(7): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.shorePlantHere(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.isBareGround(1): not translated, it uses Minecraft's `BlockState`.
        // LakePiece.shorePatchHere(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.hash01(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.staysSupported(3): not translated, it uses Minecraft's `WorldGenLevel`.
        // LakePiece.isLoosePlant(1): not translated, it uses Minecraft's `BlockState`.
        // LakePiece.markForPostProcessing(2): not translated, it uses Minecraft's `ChunkAccess`.
        // LakePiece.makeEndstonePillar(3): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.getHeight(2): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.getHeightClamp(4): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.computeHeightClamp(4): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.carveMinY(0): not translated, its class isn't translated (it extends `BasePiece`).
        // LakePiece.makeBoundingBox(0): not translated, its class isn't translated (it extends `BasePiece`).
    }

    /// `MountainPiece` (org/betterx/betterend/world/structures/piece/MountainPiece.java)
    pub mod mountain_piece {
        use super::*;
        // The class itself isn't translated: it extends `BasePiece`. Its static parts are below.
        // MountainPiece.MountainPiece(6): not translated, its class isn't translated (it extends `BasePiece`).
        // MountainPiece.MountainPiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // MountainPiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `BasePiece`).
        // MountainPiece.fromNbt(1): not translated, its class isn't translated (it extends `BasePiece`).
        // MountainPiece.getWorldgenSurface(3): not translated, its class isn't translated (it extends `BasePiece`).
        // MountainPiece.getHeight(2): not translated, its class isn't translated (it extends `BasePiece`).
        // MountainPiece.getHeightClamp(4): not translated, its class isn't translated (it extends `BasePiece`).
        // MountainPiece.makeBoundingBox(0): not translated, its class isn't translated (it extends `BasePiece`).
    }

    /// `NBTPiece` (org/betterx/betterend/world/structures/piece/NBTPiece.java)
    pub mod nbt_piece {
        use super::*;
        // The class itself isn't translated: it extends `BasePiece`. Its static parts are below.
        // NBTPiece.NBTPiece(6): not translated, its class isn't translated (it extends `BasePiece`).
        // NBTPiece.NBTPiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // NBTPiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `BasePiece`).
        // NBTPiece.fromNbt(1): not translated, its class isn't translated (it extends `BasePiece`).
        // NBTPiece.postProcess(7): not translated, its class isn't translated (it extends `BasePiece`).
        // NBTPiece.makeBoundingBox(0): not translated, its class isn't translated (it extends `BasePiece`).
        // NBTPiece.offsetPos(4): not translated, it uses Minecraft's `BlockPos`.
        // NBTPiece.getStructureBounds(4): not translated, it uses Minecraft's `BoundingBox`.
    }

    /// `PaintedMountainPiece` (org/betterx/betterend/world/structures/piece/PaintedMountainPiece.java)
    pub mod painted_mountain_piece {
        use super::*;
        // The class itself isn't translated: it extends `MountainPiece`. Its static parts are below.
        // PaintedMountainPiece.PaintedMountainPiece(6): not translated, its class isn't translated (it extends `MountainPiece`).
        // PaintedMountainPiece.PaintedMountainPiece(2): not translated, its class isn't translated (it extends `MountainPiece`).
        // PaintedMountainPiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `MountainPiece`).
        // PaintedMountainPiece.fromNbt(1): not translated, its class isn't translated (it extends `MountainPiece`).
        // PaintedMountainPiece.postProcess(7): not translated, its class isn't translated (it extends `MountainPiece`).
    }

    /// `SteppedSkirt` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java)
    pub mod stepped_skirt {
        use super::*;
        pub static NO_GROUND: LazyLock<i32> = LazyLock::new(|| i32::MIN);
        pub static NO_FLUID: LazyLock<i32> = LazyLock::new(|| i32::MIN);
        pub const RUN: f64 = 2.0f64;
        pub const JITTER: f64 = 1.5f64;
        #[derive(Clone, Debug)]
        pub struct SteppedSkirt {
            pub noise: Option<open_simplex_noise::OpenSimplexNoise>,
            pub flush: f64,
        }
        impl SteppedSkirt {
            pub fn new(noise: Option<open_simplex_noise::OpenSimplexNoise>, flush: f64) -> Self {
                let mut s = Self { noise: noise.clone(), flush: flush };
                s.noise = noise.clone();
                s.flush = flush;
                s
            }
            pub fn height(&self, wx: i32, wz: i32, dist_out: f64, high: i32) -> i32 {
                let jitter: f64 = ((self.noise.as_ref().expect("null")).eval_3((wx as f64) * 0.05f64, (wz as f64) * 0.05f64, 400.0f64)) * 1.5f64;
                let d: f64 = (dist_out - self.flush) + jitter;
                return if d <= 0.0f64 { high } else { high - (((d / 2.0f64).floor()) as i32) };
            }
        }
        pub fn taper(ground: i32, profile: i32, max_raise: i32) -> i32 {
            let excess: i32 = profile - ground;
            if excess <= 0 {
                return ground;
            }
            let raise: i32 = if excess <= max_raise { excess } else { (0i32).max((2 * max_raise) - excess) };
            return ground + raise;
        }
        // SteppedSkirt.probe(4): not translated, it uses Minecraft's `ChunkAccess`.
        // SteppedSkirt.fill(6): not translated, it uses Minecraft's `ChunkAccess`.
        // SteppedSkirt.passable(1): not translated, it uses Minecraft's `BlockState`.
    }

    /// `SteppedSkirt.Ground` (org/betterx/betterend/world/structures/piece/SteppedSkirt.java)
    pub mod stepped_skirt_ground {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // SteppedSkirt.Ground.Ground(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // SteppedSkirt.Ground.found(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // SteppedSkirt.Ground.overFluid(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // SteppedSkirt.Ground.reference(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // SteppedSkirt.Ground.groundY(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // SteppedSkirt.Ground.fluidTop(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // SteppedSkirt.Ground.under(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `SulphuricCavePiece` (org/betterx/betterend/world/structures/piece/SulphuricCavePiece.java)
    pub mod sulphuric_cave_piece {
        use super::*;
        // The class itself isn't translated: it extends `BasePiece`. Its static parts are below.
        // CAVE_AIR: not translated, it uses Minecraft's `BlockState`.
        // WATER: not translated, it uses Minecraft's `BlockState`.
        // HORIZONTAL: not translated, it uses Minecraft's `Direction`.
        // SulphuricCavePiece.SulphuricCavePiece(3): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.SulphuricCavePiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.resolveVentColumns(1): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.fromNbt(1): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.makeBoundingBox(0): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.isReplaceable(1): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.postProcess(7): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.placeVentColumn(9): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.placeBrimstone(3): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.placeSulfurSpikeClusters(8): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.isSpikeAnchor(1): not translated, it uses Minecraft's `BlockState`.
        // SulphuricCavePiece.placeSpike(9): not translated, it uses Minecraft's `ChunkAccess`.
        // SulphuricCavePiece.isBuriedInChunk(2): not translated, it uses Minecraft's `ChunkAccess`.
        // SulphuricCavePiece.facesOpenSpaceInChunk(2): not translated, it uses Minecraft's `ChunkAccess`.
        // SulphuricCavePiece.getBrimstone(2): not translated, its class isn't translated (it extends `BasePiece`).
        // SulphuricCavePiece.makeShards(3): not translated, its class isn't translated (it extends `BasePiece`).
    }

    /// `VoxelPiece` (org/betterx/betterend/world/structures/piece/VoxelPiece.java)
    pub mod voxel_piece {
        use super::*;
        // The class itself isn't translated: it extends `BasePiece`. Its static parts are below.
        // VoxelPiece.VoxelPiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // VoxelPiece.VoxelPiece(2): not translated, its class isn't translated (it extends `BasePiece`).
        // VoxelPiece.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `BasePiece`).
        // VoxelPiece.fromNbt(1): not translated, its class isn't translated (it extends `BasePiece`).
        // VoxelPiece.postProcess(7): not translated, its class isn't translated (it extends `BasePiece`).
    }

    /// `VillagePools` (org/betterx/betterend/world/structures/village/VillagePools.java)
    pub mod village_pools {
        use super::*;
        // CHORUS_VILLAGE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TERMINATORS_KEY: not translated, it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools).
        // START: not translated, it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools).
        // HOUSES_KEY: not translated, it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools).
        // STREET_KEY: not translated, it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools).
        // STREET_DECO_KEY: not translated, it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools).
        // DECORATIONS_KEY: not translated, it uses `StructurePoolKey` (de.ambertation.wover.structure.api.pools).
        #[derive(Clone, Debug)]
        pub struct VillagePools {
        }
        impl VillagePools {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `SplitNoiseCondition` (org/betterx/betterend/world/surface/SplitNoiseCondition.java)
    pub mod split_noise_condition {
        use super::*;
        // The class itself isn't translated: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise). Its static parts are below.
        // DEFAULT: not translated, it uses SplitNoiseCondition, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        pub const SEED: i32 = 1936747636;
        // SplitNoiseCondition.getNumber(1): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // SplitNoiseCondition.getNoise(2): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // SplitNoiseCondition.getNoise(3): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // SplitNoiseCondition.pcodec(0): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
    }

    /// `SulphuricSurfaceNoiseCondition` (org/betterx/betterend/world/surface/SulphuricSurfaceNoiseCondition.java)
    pub mod sulphuric_surface_noise_condition {
        use super::*;
        // The class itself isn't translated: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise). Its static parts are below.
        // DEFAULT: not translated, it uses SulphuricSurfaceNoiseCondition, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        pub const SEED: i32 = 1937075302;
        // SulphuricSurfaceNoiseCondition.getNumber(1): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // SulphuricSurfaceNoiseCondition.pcodec(0): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
    }

    /// `UmbraSurfaceNoiseCondition` (org/betterx/betterend/world/surface/UmbraSurfaceNoiseCondition.java)
    pub mod umbra_surface_noise_condition {
        use super::*;
        // The class itself isn't translated: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise). Its static parts are below.
        // DEFAULT: not translated, it uses UmbraSurfaceNoiseCondition, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        pub const SEED: i32 = 1970102898;
        // UmbraSurfaceNoiseCondition.getNumber(1): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // UmbraSurfaceNoiseCondition.getDepth(2): not translated, it calls UmbraSurfaceNoiseCondition.getDepth, which isn't translated.
        // UmbraSurfaceNoiseCondition.getDepth(3): not translated, it calls `MathHelper` (de.ambertation.wover.math.api).getSeed.
        // UmbraSurfaceNoiseCondition.pcodec(0): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
    }

    /// `VerticalBandNoiseCondition` (org/betterx/betterend/world/surface/VerticalBandNoiseCondition.java)
    pub mod vertical_band_noise_condition {
        use super::*;
        // The class itself isn't translated: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise). Its static parts are below.
        // DEFAULT: not translated, it uses VerticalBandNoiseCondition, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // VerticalBandNoiseCondition.VerticalBandNoiseCondition(5): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // VerticalBandNoiseCondition.getNumber(1): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // VerticalBandNoiseCondition.pcodec(0): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
    }

    /// `EndAdvancementDataProvider` (org/betterx/datagen/betterend/advancement/EndAdvancementDataProvider.java)
    pub mod end_advancement_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen). Its static parts are below.
        // EndAdvancementDataProvider.EndAdvancementDataProvider(2): not translated, its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)).
        // EndAdvancementDataProvider.bootstrap(1): not translated, its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)).
        // EndAdvancementDataProvider.addTools(1): not translated, its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)).
        // EndAdvancementDataProvider.addToolHeads(1): not translated, its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)).
        // EndAdvancementDataProvider.addArmor(1): not translated, its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)).
    }

    /// `BetterEndDatagen` (org/betterx/datagen/betterend/BetterEndDatagen.java)
    pub mod better_end_datagen {
        use super::*;
        // The class itself isn't translated: it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api). Its static parts are below.
        // BetterEndDatagen.onInitializeProviders(1): not translated, its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)).
        // BetterEndDatagen.modCore(0): not translated, its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)).
        // BetterEndDatagen.onBuildRegistry(1): not translated, its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)).
    }

    /// `CrystalSubblocksDefinitions` (org/betterx/datagen/betterend/blocks/CrystalSubblocksDefinitions.java)
    pub mod crystal_subblocks_definitions {
        use super::*;
        // LIT_STAIRS: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // LIT_STAIRS_INNER: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // LIT_STAIRS_OUTER: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // LIT_WALL_POST: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // LIT_WALL_SIDE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // LIT_WALL_SIDE_TALL: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        #[derive(Clone, Debug)]
        pub struct CrystalSubblocksDefinitions {
        }
        impl CrystalSubblocksDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CrystalSubblocksDefinitions.define(2): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // CrystalSubblocksDefinitions.block(4): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // CrystalSubblocksDefinitions.recipe(4): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
    }

    /// `EndBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndBlockDefinitions.java)
    pub mod end_block_definitions {
        use super::*;
        // The class itself isn't translated: it implements `BlockDefinitionsEntrypoint` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // EndBlockDefinitions.defineBlocks(1): not translated, its class isn't translated (it implements `BlockDefinitionsEntrypoint` (de.ambertation.wover.block.api.classify)).
    }

    /// `EndCropBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndCropBlockDefinitions.java)
    pub mod end_crop_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndCropBlockDefinitions {
        }
        impl EndCropBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndCropBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndCropBlockDefinitions.crop(7): not translated, it's generic.
        // EndCropBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndCrystalBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndCrystalBlockDefinitions.java)
    pub mod end_crystal_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndCrystalBlockDefinitions {
        }
        impl EndCrystalBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndCrystalBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndCrystalBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndDecorBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndDecorBlockDefinitions.java)
    pub mod end_decor_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndDecorBlockDefinitions {
        }
        impl EndDecorBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndDecorBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndDecorBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndFunctionalBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndFunctionalBlockDefinitions.java)
    pub mod end_functional_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndFunctionalBlockDefinitions {
        }
        impl EndFunctionalBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndFunctionalBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndFunctionalBlockDefinitions.pedestal(6): not translated, it's generic.
        // EndFunctionalBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndLightBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndLightBlockDefinitions.java)
    pub mod end_light_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndLightBlockDefinitions {
        }
        impl EndLightBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndLightBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndLightBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndMetalBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndMetalBlockDefinitions.java)
    pub mod end_metal_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndMetalBlockDefinitions {
        }
        impl EndMetalBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndMetalBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `EndMushroomBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndMushroomBlockDefinitions.java)
    pub mod end_mushroom_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndMushroomBlockDefinitions {
        }
        impl EndMushroomBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndMushroomBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndMushroomBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndOreBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndOreBlockDefinitions.java)
    pub mod end_ore_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndOreBlockDefinitions {
        }
        impl EndOreBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndOreBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndOreBlockDefinitions.ore(6): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
    }

    /// `EndPlantBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndPlantBlockDefinitions.java)
    pub mod end_plant_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndPlantBlockDefinitions {
        }
        impl EndPlantBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndPlantBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndPlantBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndSaplingBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndSaplingBlockDefinitions.java)
    pub mod end_sapling_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndSaplingBlockDefinitions {
        }
        impl EndSaplingBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndSaplingBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndSaplingBlockDefinitions.sapling(5): not translated, it's generic.
        // EndSaplingBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndStoneBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndStoneBlockDefinitions.java)
    pub mod end_stone_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        // NO_RECIPE: not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        #[derive(Clone, Debug)]
        pub struct EndStoneBlockDefinitions {
        }
        impl EndStoneBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndStoneBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndStoneBlockDefinitions.runedFlavolite(4): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // EndStoneBlockDefinitions.stalactite(3): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // EndStoneBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndTerrainBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndTerrainBlockDefinitions.java)
    pub mod end_terrain_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndTerrainBlockDefinitions {
        }
        impl EndTerrainBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndTerrainBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndTerrainBlockDefinitions.terrain(6): not translated, it's generic.
        // EndTerrainBlockDefinitions.terrain(8): not translated, it's generic.
        // EndTerrainBlockDefinitions.path(4): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // EndTerrainBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndVineBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndVineBlockDefinitions.java)
    pub mod end_vine_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndVineBlockDefinitions {
        }
        impl EndVineBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndVineBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndVineBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndWallPlantBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndWallPlantBlockDefinitions.java)
    pub mod end_wall_plant_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndWallPlantBlockDefinitions {
        }
        impl EndWallPlantBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndWallPlantBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndWallPlantBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndWaterPlantBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndWaterPlantBlockDefinitions.java)
    pub mod end_water_plant_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndWaterPlantBlockDefinitions {
        }
        impl EndWaterPlantBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndWaterPlantBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndWaterPlantBlockDefinitions.charnia(3): not translated, it uses Minecraft's `Block`.
        // EndWaterPlantBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `EndWoodBlockDefinitions` (org/betterx/datagen/betterend/blocks/EndWoodBlockDefinitions.java)
    pub mod end_wood_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        pub const CANOPY_SAPLING_CHANCE: f32 = 0.0078125f32;
        // TENANEA_PALETTE: not translated, it uses `PaletteCycle` (de.ambertation.wover.block.impl.classify.TintTrait).
        #[derive(Clone, Debug)]
        pub struct EndWoodBlockDefinitions {
        }
        impl EndWoodBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndWoodBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndWoodBlockDefinitions.leaves(5): not translated, it uses Minecraft's `Block`.
        // EndWoodBlockDefinitions.model(1): not translated, a raw Supplier.
    }

    /// `FlamboyantBlockDefinitions` (org/betterx/datagen/betterend/blocks/FlamboyantBlockDefinitions.java)
    pub mod flamboyant_block_definitions {
        use super::*;
        // petals: not translated, it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        #[derive(Clone, Debug)]
        pub struct FlamboyantBlockDefinitions {
        }
        impl FlamboyantBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FlamboyantBlockDefinitions.petals(0): not translated, it uses ColoredMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // FlamboyantBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `MetalMaterialDefinitions` (org/betterx/datagen/betterend/blocks/MetalMaterialDefinitions.java)
    pub mod metal_material_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MetalMaterialDefinitions {
        }
        impl MetalMaterialDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MetalMaterialDefinitions.define(2): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `EndModelProvider` (org/betterx/datagen/betterend/EndModelProvider.java)
    pub mod end_model_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // EndModelProvider.bootstrapItemModels(1): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndModelProvider.bootstrapBlockStateModels(1): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndModelProvider.buildModelOverrides(1): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndModelProvider.EndModelProvider(1): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `EndPottablePlantProvider` (org/betterx/datagen/betterend/EndPottablePlantProvider.java)
    pub mod end_pottable_plant_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverPottablePlantRegistryProvider` (de.ambertation.wover.pottable.api.datagen). Its static parts are below.
        // VANILLA_PLANTS: not translated, it uses Minecraft's `Block`.
        // EndPottablePlantProvider.EndPottablePlantProvider(1): not translated, its class isn't translated (it extends `WoverPottablePlantRegistryProvider` (de.ambertation.wover.pottable.api.datagen)).
        // EndPottablePlantProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverPottablePlantRegistryProvider` (de.ambertation.wover.pottable.api.datagen)).
    }

    /// `EndPottableSoilProvider` (org/betterx/datagen/betterend/EndPottableSoilProvider.java)
    pub mod end_pottable_soil_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverPottableSoilRegistryProvider` (de.ambertation.wover.pottable.api.datagen). Its static parts are below.
        // VANILLA_SOILS: not translated, it uses Minecraft's `Block`.
        // EndPottableSoilProvider.EndPottableSoilProvider(1): not translated, its class isn't translated (it extends `WoverPottableSoilRegistryProvider` (de.ambertation.wover.pottable.api.datagen)).
        // EndPottableSoilProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverPottableSoilRegistryProvider` (de.ambertation.wover.pottable.api.datagen)).
    }

    /// `EndRegistrySupplier` (org/betterx/datagen/betterend/EndRegistrySupplier.java)
    pub mod end_registry_supplier {
        use super::*;
        // The class itself isn't translated: it extends `RegistrySupplier` (org.betterx.bclib.api.v3.datagen). Its static parts are below.
        // INSTANCE: not translated, it uses EndRegistrySupplier, which isn't translated (it extends `RegistrySupplier` (org.betterx.bclib.api.v3.datagen)).
        // EndRegistrySupplier.EndRegistrySupplier(0): not translated, its class isn't translated (it extends `RegistrySupplier` (org.betterx.bclib.api.v3.datagen)).
        // EndRegistrySupplier.initializeRegistryList(1): not translated, its class isn't translated (it extends `RegistrySupplier` (org.betterx.bclib.api.v3.datagen)).
    }

    /// `GlimmerwildBiomeModificationProvider` (org/betterx/datagen/betterend/glimmerwild/GlimmerwildBiomeModificationProvider.java)
    pub mod glimmerwild_biome_modification_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // GlimmerwildBiomeModificationProvider.GlimmerwildBiomeModificationProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // GlimmerwildBiomeModificationProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `GlimmerwildBlockTagProvider` (org/betterx/datagen/betterend/glimmerwild/GlimmerwildBlockTagProvider.java)
    pub mod glimmerwild_block_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // WOOLLY_GROUND: not translated, it uses Minecraft's `TagKey`.
        // GlimmerwildBlockTagProvider.GlimmerwildBlockTagProvider(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // GlimmerwildBlockTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `GlimmerwildFeaturesProvider` (org/betterx/datagen/betterend/glimmerwild/GlimmerwildFeaturesProvider.java)
    pub mod glimmerwild_features_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // GlimmerwildFeaturesProvider.GlimmerwildFeaturesProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // GlimmerwildFeaturesProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // GlimmerwildFeaturesProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // GlimmerwildFeaturesProvider.single(5): not translated, it uses Minecraft's `BootstrapContext`.
        // GlimmerwildFeaturesProvider.double_(6): not translated, it uses Minecraft's `BootstrapContext`.
        // GlimmerwildFeaturesProvider.register(4): not translated, it uses Minecraft's `BootstrapContext`.
    }

    /// `CrystaliteDefinitions` (org/betterx/datagen/betterend/items/CrystaliteDefinitions.java)
    pub mod crystalite_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CrystaliteDefinitions {
        }
        impl CrystaliteDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CrystaliteDefinitions.helmet(2): not translated, it uses `ItemDefinition` (de.ambertation.wover.item.api.classify).
        // CrystaliteDefinitions.chestplate(2): not translated, it uses `ItemDefinition` (de.ambertation.wover.item.api.classify).
        // CrystaliteDefinitions.leggings(2): not translated, it uses `ItemDefinition` (de.ambertation.wover.item.api.classify).
        // CrystaliteDefinitions.boots(2): not translated, it uses `ItemDefinition` (de.ambertation.wover.item.api.classify).
        // CrystaliteDefinitions.armor(5): not translated, it's generic.
    }

    /// `EndArmorStats` (org/betterx/datagen/betterend/items/EndArmorStats.java)
    pub mod end_armor_stats {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndArmorStats {
        }
        impl EndArmorStats {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndArmorStats.apply(3): not translated, it's generic.
        // EndArmorStats.apply(4): not translated, it's generic.
        // EndArmorStats.apply(8): not translated, it's generic.
        // EndArmorStats.apply(9): not translated, it's generic.
    }

    /// `EndDebugItemDefinitions` (org/betterx/datagen/betterend/items/EndDebugItemDefinitions.java)
    pub mod end_debug_item_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndDebugItemDefinitions {
        }
        impl EndDebugItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndDebugItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `EndDiscItemDefinitions` (org/betterx/datagen/betterend/items/EndDiscItemDefinitions.java)
    pub mod end_disc_item_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndDiscItemDefinitions {
        }
        impl EndDiscItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndDiscItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `EndEquipmentItemDefinitions` (org/betterx/datagen/betterend/items/EndEquipmentItemDefinitions.java)
    pub mod end_equipment_item_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndEquipmentItemDefinitions {
        }
        impl EndEquipmentItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndEquipmentItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndEquipmentItemDefinitions.hammer(4): not translated, it uses `ItemDefinition` (de.ambertation.wover.item.api.classify).
    }

    /// `EndFoodItemDefinitions` (org/betterx/datagen/betterend/items/EndFoodItemDefinitions.java)
    pub mod end_food_item_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndFoodItemDefinitions {
        }
        impl EndFoodItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndFoodItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `EndItemDefinitions` (org/betterx/datagen/betterend/items/EndItemDefinitions.java)
    pub mod end_item_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndItemDefinitions {
        }
        impl EndItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndItemDefinitions.registerEndDisc(3): not translated, it uses Minecraft's `Item`.
        // EndItemDefinitions.defineEndItem(2): not translated, it uses `ItemDefinition` (de.ambertation.wover.item.api.classify).
        // EndItemDefinitions.registerEndItem(2): not translated, it uses Minecraft's `Item`.
        // EndItemDefinitions.registerEndFood(5): not translated, it uses Minecraft's `Item`.
        // EndItemDefinitions.registerEndFood(3): not translated, it uses Minecraft's `Item`.
        // EndItemDefinitions.registerEndDrink(4): not translated, it uses Minecraft's `Item`.
    }

    /// `EndResourceItemDefinitions` (org/betterx/datagen/betterend/items/EndResourceItemDefinitions.java)
    pub mod end_resource_item_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndResourceItemDefinitions {
        }
        impl EndResourceItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndResourceItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `EndSpawnEggDefinitions` (org/betterx/datagen/betterend/items/EndSpawnEggDefinitions.java)
    pub mod end_spawn_egg_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndSpawnEggDefinitions {
        }
        impl EndSpawnEggDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndSpawnEggDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `EndTemplatesDefinitions` (org/betterx/datagen/betterend/items/EndTemplatesDefinitions.java)
    pub mod end_templates_definitions {
        use super::*;
        // EMPTY_SLOT_STICK: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_HANDLE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_SWORD_HANDLE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_SWORD_BLADE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_SPEAR_TIP: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_PLATE: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_HAMMER: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_HAMMER_HEAD: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_PICKAXE_HEAD: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_AXE_HEAD: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_HOE_HEAD: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_SHOVEL_HEAD: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_ANVIL: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_ELYTRA: not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EndTemplatesDefinitions {
        }
        impl EndTemplatesDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndTemplatesDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // EndTemplatesDefinitions.smithingTemplate(2): not translated, it uses `ItemDefinition` (de.ambertation.wover.item.api.classify).
    }

    /// `GuideBookItemDefinitions` (org/betterx/datagen/betterend/items/GuideBookItemDefinitions.java)
    pub mod guide_book_item_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct GuideBookItemDefinitions {
        }
        impl GuideBookItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GuideBookItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `ToolsWithHeadsSetDefinitions` (org/betterx/datagen/betterend/items/ToolsWithHeadsSetDefinitions.java)
    pub mod tools_with_heads_set_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ToolsWithHeadsSetDefinitions {
        }
        impl ToolsWithHeadsSetDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ToolsWithHeadsSetDefinitions.define(2): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // ToolsWithHeadsSetDefinitions.head(4): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `AlloyingRecipesProvider` (org/betterx/datagen/betterend/recipes/AlloyingRecipesProvider.java)
    pub mod alloying_recipes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // AlloyingRecipesProvider.AlloyingRecipesProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // AlloyingRecipesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `AnvilRecipesProvider` (org/betterx/datagen/betterend/recipes/AnvilRecipesProvider.java)
    pub mod anvil_recipes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // AnvilRecipesProvider.AnvilRecipesProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // AnvilRecipesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `EndBlockLootTableProvider` (org/betterx/datagen/betterend/recipes/EndBlockLootTableProvider.java)
    pub mod end_block_loot_table_provider {
        use super::*;
        // The class itself isn't translated: it extends `BlockLootTableProvider` (org.betterx.bclib.api.v3.datagen). Its static parts are below.
        // EndBlockLootTableProvider.EndBlockLootTableProvider(2): not translated, its class isn't translated (it extends `BlockLootTableProvider` (org.betterx.bclib.api.v3.datagen)).
    }

    /// `EndChestLootTableProvider` (org/betterx/datagen/betterend/recipes/EndChestLootTableProvider.java)
    pub mod end_chest_loot_table_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // IN_FOGGY_MUSHROOMLAND: not translated, it uses Minecraft's `Builder`.
        // IN_CHORUS_FOREST: not translated, it uses Minecraft's `Builder`.
        // IN_AMBER_LAND: not translated, it uses Minecraft's `Builder`.
        // IN_GLOWING_GRASSLANDS: not translated, it uses Minecraft's `Builder`.
        // IN_LANTERN_WOODS: not translated, it uses Minecraft's `Builder`.
        // IN_MEGALAKE: not translated, it uses Minecraft's `Builder`.
        // IN_MEGALAKE_GROVE: not translated, it uses Minecraft's `Builder`.
        // IN_NEON_OASIS: not translated, it uses Minecraft's `Builder`.
        // IN_SHADOW_FOREST: not translated, it uses Minecraft's `Builder`.
        // IN_SULPHUR_SPRINGS: not translated, it uses Minecraft's `Builder`.
        // IN_UMBRELLA_JUNGLE: not translated, it uses Minecraft's `Builder`.
        // EndChestLootTableProvider.EndChestLootTableProvider(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.endCityGhastTear(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.endCityMusicDiscs(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.endCitySmithingTemplates(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.endCityEndVeilBook(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.resonanceBook(2): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.fishing(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.fishingJunk(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.enchantments(1): not translated, it uses Minecraft's `HolderGetter`.
        // EndChestLootTableProvider.fishingTreasure(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.foggyMushroomland(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.chorusForest(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.shadowForest(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.lanternWoods(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.umbrellaJungle(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.upgradeLootPool(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.plateUpgradeLootPool(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.leatherHandleLootPool(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.handleLootPool(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.villageBonusLoot(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.elytraLoot(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.simpleVillageLoot(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndChestLootTableProvider.addCharnia(1): not translated, it uses Minecraft's `Builder`.
        // EndChestLootTableProvider.includeCommonItems(1): not translated, it uses Builder, which isn't translated (it's an interface).
        // EndChestLootTableProvider.biomePredicate(2): not translated, it uses Minecraft's `Builder`.
        // EndChestLootTableProvider.boostrap(2): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `EndCraftingRecipesProvider` (org/betterx/datagen/betterend/recipes/EndCraftingRecipesProvider.java)
    pub mod end_crafting_recipes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // EndCraftingRecipesProvider.EndCraftingRecipesProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndCraftingRecipesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndCraftingRecipesProvider.registerLantern(4): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // EndCraftingRecipesProvider.registerPedestal(5): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // EndCraftingRecipesProvider.registerHammer(4): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
    }

    /// `EndEnchantmentProvider` (org/betterx/datagen/betterend/recipes/EndEnchantmentProvider.java)
    pub mod end_enchantment_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // EndEnchantmentProvider.EndEnchantmentProvider(1): not translated, its class isn't translated (it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndEnchantmentProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `EndFurnaceRecipeProvider` (org/betterx/datagen/betterend/recipes/EndFurnaceRecipeProvider.java)
    pub mod end_furnace_recipe_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // EndFurnaceRecipeProvider.EndFurnaceRecipeProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndFurnaceRecipeProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `EndLootAdditionProvider` (org/betterx/datagen/betterend/recipes/EndLootAdditionProvider.java)
    pub mod end_loot_addition_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // EndLootAdditionProvider.EndLootAdditionProvider(1): not translated, its class isn't translated (it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndLootAdditionProvider.bootstrap(2): not translated, its class isn't translated (it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `EndMaterialRecipesProvider` (org/betterx/datagen/betterend/recipes/EndMaterialRecipesProvider.java)
    pub mod end_material_recipes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // EndMaterialRecipesProvider.EndMaterialRecipesProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // EndMaterialRecipesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `FlamboyantRecipeProvider` (org/betterx/datagen/betterend/recipes/FlamboyantRecipeProvider.java)
    pub mod flamboyant_recipe_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // FlamboyantRecipeProvider.FlamboyantRecipeProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // FlamboyantRecipeProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `InfusionRecipesProvider` (org/betterx/datagen/betterend/recipes/InfusionRecipesProvider.java)
    pub mod infusion_recipes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // InfusionRecipesProvider.InfusionRecipesProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // InfusionRecipesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `JukeboxRegistryProvider` (org/betterx/datagen/betterend/recipes/JukeboxRegistryProvider.java)
    pub mod jukebox_registry_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // JukeboxRegistryProvider.JukeboxRegistryProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // JukeboxRegistryProvider.register(5): not translated, it uses Minecraft's `BootstrapContext`.
        // JukeboxRegistryProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `PatchouliBookProvider` (org/betterx/datagen/betterend/recipes/PatchouliBookProvider.java)
    pub mod patchouli_book_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // PatchouliBookProvider.PatchouliBookProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // PatchouliBookProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `SmithingRecipesProvider` (org/betterx/datagen/betterend/recipes/SmithingRecipesProvider.java)
    pub mod smithing_recipes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // SmithingRecipesProvider.SmithingRecipesProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // SmithingRecipesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `BiomeTagProvider` (org/betterx/datagen/betterend/tags/BiomeTagProvider.java)
    pub mod biome_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // WAYSTONES_END_STONE_WAYSTONE: not translated, it uses Minecraft's `TagKey`.
        // BiomeTagProvider.BiomeTagProvider(1): not translated, its class isn't translated (it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // BiomeTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `BlockTagProvider` (org/betterx/datagen/betterend/tags/BlockTagProvider.java)
    pub mod block_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // BlockTagProvider.BlockTagProvider(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // BlockTagProvider.addEndGround(2): not translated, it uses `TagBootstrapContext` (de.ambertation.wover.tag.api.event.context).
        // BlockTagProvider.addSurvivesOn(1): not translated, it uses `TagBootstrapContext` (de.ambertation.wover.tag.api.event.context).
        // BlockTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `EnchantmentTagProvider` (org/betterx/datagen/betterend/tags/EnchantmentTagProvider.java)
    pub mod enchantment_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // EnchantmentTagProvider.EnchantmentTagProvider(1): not translated, its class isn't translated (it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // EnchantmentTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `ItemTagProvider` (org/betterx/datagen/betterend/tags/ItemTagProvider.java)
    pub mod item_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // CAPE_SLOT: not translated, it uses Minecraft's `TagKey`.
        // ItemTagProvider.ItemTagProvider(1): not translated, its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // ItemTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `NourishItemTagProvider` (org/betterx/datagen/betterend/tags/NourishItemTagProvider.java)
    pub mod nourish_item_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // NourishItemTagProvider.NourishItemTagProvider(1): not translated, its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // NourishItemTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `EndTradeSetProvider` (org/betterx/datagen/betterend/villagers/EndTradeSetProvider.java)
    pub mod end_trade_set_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // EndTradeSetProvider.EndTradeSetProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // EndTradeSetProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `EndVillagerDefinitions` (org/betterx/datagen/betterend/villagers/EndVillagerDefinitions.java)
    pub mod end_villager_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndVillagerDefinitions {
        }
        impl EndVillagerDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EndVillagerDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `EndVillagerTradeProvider` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java)
    pub mod end_villager_trade_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        pub static EMERALD: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "emerald"));
        // TRADES: not translated, it calls EndVillagerTradeProvider.end, which isn't translated.
        // EndVillagerTradeProvider.EndVillagerTradeProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // EndVillagerTradeProvider.end(1): not translated, it uses BetterEnd.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        pub fn vanilla(path: &str) -> JIdentifier {
            return JIdentifier::new("minecraft", &path);
        }
        pub fn buy(level: i32, name: &str, wants: &JIdentifier, count: i32, max_uses: i32, xp: i32) -> end_villager_trade_provider_trade::Trade {
            return end_villager_trade_provider_trade::Trade::new(level, &(format!("{}_emerald", name)), wants, count, None, &(*end_villager_trade_provider::EMERALD), 1, max_uses, xp, 1.0f32);
        }
        pub fn sell(level: i32, name: &str, emeralds: i32, gives: &JIdentifier, count: i32, max_uses: i32, xp: i32) -> end_villager_trade_provider_trade::Trade {
            return end_villager_trade_provider_trade::Trade::new(level, &(format!("emerald_{}", name)), &(*end_villager_trade_provider::EMERALD), emeralds, None, gives, count, max_uses, xp, 1.0f32);
        }
        // EndVillagerTradeProvider.item(1): not translated, it uses Minecraft's `Item`.
        // EndVillagerTradeProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `EndVillagerTradeProvider.Trade` (org/betterx/datagen/betterend/villagers/EndVillagerTradeProvider.java)
    pub mod end_villager_trade_provider_trade {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Trade {
            pub level: i32,
            pub name: String,
            pub wants: JIdentifier,
            pub wants_count: i32,
            pub also_wants: Option<JIdentifier>,
            pub gives: JIdentifier,
            pub gives_count: i32,
            pub max_uses: i32,
            pub xp: i32,
            pub chance: f32,
        }
        impl Trade {
            pub fn new(level: i32, name: &str, wants: &JIdentifier, wants_count: i32, also_wants: Option<JIdentifier>, gives: &JIdentifier, gives_count: i32, max_uses: i32, xp: i32, chance: f32) -> Self {
                let mut s = Self { level: level, name: String::new(), wants: wants.clone(), wants_count: wants_count, also_wants: also_wants.clone(), gives: gives.clone(), gives_count: gives_count, max_uses: max_uses, xp: xp, chance: chance };
                s.level = level;
                s.name = name.to_string();
                s.wants = wants.clone();
                s.wants_count = wants_count;
                s.also_wants = also_wants.clone();
                s.gives = gives.clone();
                s.gives_count = gives_count;
                s.max_uses = max_uses;
                s.xp = xp;
                s.chance = chance;
                s
            }
            // EndVillagerTradeProvider.Trade.key(0): not translated, it uses Minecraft's `ResourceKey`.
            pub fn level(&self) -> i32 {
                return self.level;
            }
            pub fn name(&self) -> String {
                return self.name.to_string();
            }
            pub fn wants(&self) -> JIdentifier {
                return self.wants.clone();
            }
            pub fn wants_count(&self) -> i32 {
                return self.wants_count;
            }
            pub fn also_wants(&self) -> Option<JIdentifier> {
                return self.also_wants.clone();
            }
            pub fn gives(&self) -> JIdentifier {
                return self.gives.clone();
            }
            pub fn gives_count(&self) -> i32 {
                return self.gives_count;
            }
            pub fn max_uses(&self) -> i32 {
                return self.max_uses;
            }
            pub fn xp(&self) -> i32 {
                return self.xp;
            }
            pub fn chance(&self) -> f32 {
                return self.chance;
            }
        }
    }

    /// `EndVillagerTradeTagProvider` (org/betterx/datagen/betterend/villagers/EndVillagerTradeTagProvider.java)
    pub mod end_villager_trade_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverTagProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // VILLAGER_TRADES: not translated, it uses `TagRegistry` (de.ambertation.wover.tag.api).
        // EndVillagerTradeTagProvider.EndVillagerTradeTagProvider(1): not translated, its class isn't translated (it extends `WoverTagProvider` (de.ambertation.wover.datagen.api)).
        // EndVillagerTradeTagProvider.level(1): not translated, it uses Minecraft's `TagKey`.
        // EndVillagerTradeTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `WoverTagProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `CarverProvider` (org/betterx/datagen/betterend/worldgen/CarverProvider.java)
    pub mod carver_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // CarverProvider.CarverProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // CarverProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `EndBiomeModificationProvider` (org/betterx/datagen/betterend/worldgen/EndBiomeModificationProvider.java)
    pub mod end_biome_modification_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // EndBiomeModificationProvider.EndBiomeModificationProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // EndBiomeModificationProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `EndBiomesProvider` (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java)
    pub mod end_biomes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // BIOMES: not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // EndBiomesProvider.EndBiomesProvider(1): not translated, its class isn't translated (it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // EndBiomesProvider.loadAllBiomeConfigs(0): not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // EndBiomesProvider.putBiome(3): not translated, it uses EndBiomeKey, which isn't translated (it's generic).
        // EndBiomesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `EndBiomesProvider.BiomeInfo` (org/betterx/datagen/betterend/worldgen/EndBiomesProvider.java)
    pub mod end_biomes_provider_biome_info {
        use super::*;
        // The class itself isn't translated: it uses Config, which isn't translated (it implements `BiomeFactory`). Its static parts are below.
        // EndBiomesProvider.BiomeInfo.BiomeInfo(2): not translated, its class isn't translated (it uses Config, which isn't translated (it implements `BiomeFactory`)).
        // EndBiomesProvider.BiomeInfo.config(0): not translated, its class isn't translated (it uses Config, which isn't translated (it implements `BiomeFactory`)).
        // EndBiomesProvider.BiomeInfo.tags(0): not translated, its class isn't translated (it uses Config, which isn't translated (it implements `BiomeFactory`)).
    }

    /// `BonemealFeatureProvider` (org/betterx/datagen/betterend/worldgen/features/BonemealFeatureProvider.java)
    pub mod bonemeal_feature_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // BonemealFeatureProvider.BonemealFeatureProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // BonemealFeatureProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // BonemealFeatureProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `CaveFeatureProvider` (org/betterx/datagen/betterend/worldgen/features/CaveFeatureProvider.java)
    pub mod cave_feature_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        pub const LUSH_PATCH_VERTICAL_RANGE: i32 = 5;
        // CaveFeatureProvider.CaveFeatureProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // CaveFeatureProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // CaveFeatureProvider.surfaceCoat(1): not translated, it uses CaveSurfaceCoatFeature, which isn't translated (it implements Minecraft's `Feature`).
        // CaveFeatureProvider.lushPatch(3): not translated, it uses CaveVegetationPatchFeature, which isn't translated (it extends Minecraft's `VegetationPatchFeature`).
        // CaveFeatureProvider.floorVegetation(1): not translated, it uses Minecraft's `Holder`.
        // CaveFeatureProvider.ceilingVegetation(1): not translated, it uses Minecraft's `Holder`.
        // CaveFeatureProvider.inlinePlaced(2): not translated, it uses Minecraft's `Holder`.
        // CaveFeatureProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // CaveFeatureProvider.surfaceCoat(3): not translated, it uses Minecraft's `BootstrapContext`.
        // CaveFeatureProvider.cluster(4): not translated, it uses Minecraft's `BootstrapContext`.
        // CaveFeatureProvider.patch(4): not translated, it uses Minecraft's `BootstrapContext`.
        // CaveFeatureProvider.floorScatter(4): not translated, it uses Minecraft's `BootstrapContext`.
        // CaveFeatureProvider.ceilScatter(4): not translated, it uses Minecraft's `BootstrapContext`.
        // CaveFeatureProvider.caveHeight(0): not translated, it uses Minecraft's `HeightRangePlacement`.
        // CaveFeatureProvider.underCover(1): not translated, it uses Minecraft's `SurfaceRelativeThresholdFilter`.
    }

    /// `LakeFeatureProvider` (org/betterx/datagen/betterend/worldgen/features/LakeFeatureProvider.java)
    pub mod lake_feature_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // LakeFeatureProvider.LakeFeatureProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // LakeFeatureProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // LakeFeatureProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // LakeFeatureProvider.registerLake(4): not translated, it's generic.
    }

    /// `OreFeatureProvider` (org/betterx/datagen/betterend/worldgen/features/OreFeatureProvider.java)
    pub mod ore_feature_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // OreFeatureProvider.OreFeatureProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // OreFeatureProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // OreFeatureProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // OreFeatureProvider.registerOre(5): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // OreFeatureProvider.registerLayer(7): not translated, it's overloaded with the same number of parameters.
        // OreFeatureProvider.registerLayer(7): not translated, it's overloaded with the same number of parameters.
    }

    /// `TerrainFeatureProvider` (org/betterx/datagen/betterend/worldgen/features/TerrainFeatureProvider.java)
    pub mod terrain_feature_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // TerrainFeatureProvider.TerrainFeatureProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // TerrainFeatureProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // TerrainFeatureProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // TerrainFeatureProvider.surfaceSpike(0): not translated, it uses Minecraft's `SpeleothemFeature`.
        // TerrainFeatureProvider.hangingSpike(0): not translated, it uses Minecraft's `SpeleothemFeature`.
        // TerrainFeatureProvider.registerChanced(4): not translated, it's generic.
        // TerrainFeatureProvider.registerCounted(4): not translated, it's generic.
    }

    /// `VegetationFeaturesProvider` (org/betterx/datagen/betterend/worldgen/features/VegetationFeaturesProvider.java)
    pub mod vegetation_features_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // VegetationFeaturesProvider.VegetationFeaturesProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // VegetationFeaturesProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // VegetationFeaturesProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // VegetationFeaturesProvider.registerVegetation(4): not translated, it's generic.
        // VegetationFeaturesProvider.registerVegetation(3): not translated, it uses Minecraft's `BootstrapContext`.
    }

    /// `StructureDataProvider` (org/betterx/datagen/betterend/worldgen/StructureDataProvider.java)
    pub mod structure_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        pub const BUILDING_MIN_FLOOR_Y: i32 = 58;
        pub const CRASHED_SHIP_MIN_FLOOR_Y: i32 = 8;
        // StructureDataProvider.StructureDataProvider(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.bootstrapSturctures(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.bootstrapSets(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.bootstrapPools(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.bootstrapProcessors(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.prepareBiomeTags(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// JOML's Vector3f as a value (Java's methods change the vector; the translation writes the result back).
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct JVec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl JVec3 {
        pub const fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
        pub fn add(self, o: JVec3) -> Self {
            Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
        }
        pub fn add3(self, x: f32, y: f32, z: f32) -> Self {
            Self::new(self.x + x, self.y + y, self.z + z)
        }
        pub fn sub(self, o: JVec3) -> Self {
            Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
        }
        pub fn mul_s(self, s: f32) -> Self {
            Self::new(self.x * s, self.y * s, self.z * s)
        }
        pub fn mul_v(self, o: JVec3) -> Self {
            Self::new(self.x * o.x, self.y * o.y, self.z * o.z)
        }
        pub fn negate(self) -> Self {
            Self::new(-self.x, -self.y, -self.z)
        }
        pub fn dot(self, o: JVec3) -> f32 {
            self.x * o.x + self.y * o.y + self.z * o.z
        }
        pub fn length(self) -> f32 {
            self.dot(self).sqrt()
        }
        pub fn length_squared(self) -> f32 {
            self.dot(self)
        }
        /// JOML's normalize: divides by the length (a zero vector becomes NaNs, as in Java).
        pub fn normalize(self) -> Self {
            let inv = 1.0 / self.length();
            self.mul_s(inv)
        }
        pub fn cross(self, o: JVec3) -> Self {
            Self::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
        }
        /// JOML's lerp: this + (other - this) * t.
        pub fn lerp(self, o: JVec3, t: f32) -> Self {
            Self::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t, self.z + (o.z - self.z) * t)
        }
        pub fn distance(self, o: JVec3) -> f32 {
            self.sub(o).length()
        }
    }
    
    /// JOML's Quaternionf (x, y, z, w).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JQuat {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl Default for JQuat {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JQuat {
        pub const IDENTITY: JQuat = JQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self { x, y, z, w }
        }
        /// `rotationX(angle)`: a turn about +X by `angle` radians (Axis.XP.rotation).
        pub fn rotation_x(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(s, 0.0, 0.0, c)
        }
        pub fn rotation_y(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, s, 0.0, c)
        }
        pub fn rotation_z(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, 0.0, s, c)
        }
        pub fn normalize(self) -> Self {
            let inv = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
            Self::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
        }
        /// this * q.
        pub fn mul(self, q: JQuat) -> Self {
            Self::new(
                self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
                self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
                self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
                self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            )
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.mul(Self::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.mul(Self::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.mul(Self::rotation_z(a))
        }
    }
    
    /// JOML's Matrix3f, column-major, from three column vectors.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat3 {
        pub m: [f32; 9],
    }
    impl JMat3 {
        pub fn from_cols(c0: JVec3, c1: JVec3, c2: JVec3) -> Self {
            Self { m: [c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z] }
        }
    }
    
    /// JOML's Matrix4f: column-major, `m[col * 4 + row]`; `a.mul(b)` is a * b, and translate/rotate/scale
    /// multiply on the right, as JOML does.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat4 {
        pub m: [f32; 16],
    }
    impl Default for JMat4 {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JMat4 {
        pub const IDENTITY: JMat4 = JMat4 { m: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0] };
        pub fn identity() -> Self {
            Self::IDENTITY
        }
        fn at(&self, row: usize, col: usize) -> f32 {
            self.m[col * 4 + row]
        }
        pub fn mul(self, r: JMat4) -> Self {
            let mut out = [0.0f32; 16];
            for col in 0..4 {
                for row in 0..4 {
                    let mut v = 0.0;
                    for k in 0..4 {
                        v += self.at(row, k) * r.at(k, col);
                    }
                    out[col * 4 + row] = v;
                }
            }
            Self { m: out }
        }
        pub fn translation(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[12] = x;
            t.m[13] = y;
            t.m[14] = z;
            t
        }
        pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::translation(x, y, z))
        }
        pub fn translate_v(self, v: JVec3) -> Self {
            self.translate(v.x, v.y, v.z)
        }
        pub fn scaling(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[0] = x;
            t.m[5] = y;
            t.m[10] = z;
            t
        }
        pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::scaling(x, y, z))
        }
        pub fn scale_s(self, s: f32) -> Self {
            self.scale(s, s, s)
        }
        /// The rotation a unit quaternion stands for.
        pub fn rotation(q: JQuat) -> Self {
            let (x, y, z, w) = (q.x, q.y, q.z, q.w);
            let (xx, yy, zz) = (x * x, y * y, z * z);
            let (xy, xz, yz, wx, wy, wz) = (x * y, x * z, y * z, w * x, w * y, w * z);
            Self {
                m: [
                    1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0,
                    2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0,
                    2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ],
            }
        }
        pub fn rotate(self, q: JQuat) -> Self {
            self.mul(Self::rotation(q))
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_z(a))
        }
        /// JOML's rotateXYZ: about X, then Y, then Z.
        pub fn rotate_xyz(self, x: f32, y: f32, z: f32) -> Self {
            self.rotate_x(x).rotate_y(y).rotate_z(z)
        }
        /// JOML's rotateAround(q, ox, oy, oz): about the point (ox, oy, oz).
        pub fn rotate_around(self, q: JQuat, ox: f32, oy: f32, oz: f32) -> Self {
            self.translate(ox, oy, oz).rotate(q).translate(-ox, -oy, -oz)
        }
        /// The general inverse (JOML's invert); a singular matrix gives infinities, as in Java.
        pub fn invert(self) -> Self {
            let a = &self.m;
            let mut inv = [0.0f32; 16];
            inv[0] = a[5] * a[10] * a[15] - a[5] * a[11] * a[14] - a[9] * a[6] * a[15] + a[9] * a[7] * a[14] + a[13] * a[6] * a[11] - a[13] * a[7] * a[10];
            inv[4] = -a[4] * a[10] * a[15] + a[4] * a[11] * a[14] + a[8] * a[6] * a[15] - a[8] * a[7] * a[14] - a[12] * a[6] * a[11] + a[12] * a[7] * a[10];
            inv[8] = a[4] * a[9] * a[15] - a[4] * a[11] * a[13] - a[8] * a[5] * a[15] + a[8] * a[7] * a[13] + a[12] * a[5] * a[11] - a[12] * a[7] * a[9];
            inv[12] = -a[4] * a[9] * a[14] + a[4] * a[10] * a[13] + a[8] * a[5] * a[14] - a[8] * a[6] * a[13] - a[12] * a[5] * a[10] + a[12] * a[6] * a[9];
            inv[1] = -a[1] * a[10] * a[15] + a[1] * a[11] * a[14] + a[9] * a[2] * a[15] - a[9] * a[3] * a[14] - a[13] * a[2] * a[11] + a[13] * a[3] * a[10];
            inv[5] = a[0] * a[10] * a[15] - a[0] * a[11] * a[14] - a[8] * a[2] * a[15] + a[8] * a[3] * a[14] + a[12] * a[2] * a[11] - a[12] * a[3] * a[10];
            inv[9] = -a[0] * a[9] * a[15] + a[0] * a[11] * a[13] + a[8] * a[1] * a[15] - a[8] * a[3] * a[13] - a[12] * a[1] * a[11] + a[12] * a[3] * a[9];
            inv[13] = a[0] * a[9] * a[14] - a[0] * a[10] * a[13] - a[8] * a[1] * a[14] + a[8] * a[2] * a[13] + a[12] * a[1] * a[10] - a[12] * a[2] * a[9];
            inv[2] = a[1] * a[6] * a[15] - a[1] * a[7] * a[14] - a[5] * a[2] * a[15] + a[5] * a[3] * a[14] + a[13] * a[2] * a[7] - a[13] * a[3] * a[6];
            inv[6] = -a[0] * a[6] * a[15] + a[0] * a[7] * a[14] + a[4] * a[2] * a[15] - a[4] * a[3] * a[14] - a[12] * a[2] * a[7] + a[12] * a[3] * a[6];
            inv[10] = a[0] * a[5] * a[15] - a[0] * a[7] * a[13] - a[4] * a[1] * a[15] + a[4] * a[3] * a[13] + a[12] * a[1] * a[7] - a[12] * a[3] * a[5];
            inv[14] = -a[0] * a[5] * a[14] + a[0] * a[6] * a[13] + a[4] * a[1] * a[14] - a[4] * a[2] * a[13] - a[12] * a[1] * a[6] + a[12] * a[2] * a[5];
            inv[3] = -a[1] * a[6] * a[11] + a[1] * a[7] * a[10] + a[5] * a[2] * a[11] - a[5] * a[3] * a[10] - a[9] * a[2] * a[7] + a[9] * a[3] * a[6];
            inv[7] = a[0] * a[6] * a[11] - a[0] * a[7] * a[10] - a[4] * a[2] * a[11] + a[4] * a[3] * a[10] + a[8] * a[2] * a[7] - a[8] * a[3] * a[6];
            inv[11] = -a[0] * a[5] * a[11] + a[0] * a[7] * a[9] + a[4] * a[1] * a[11] - a[4] * a[3] * a[9] - a[8] * a[1] * a[7] + a[8] * a[3] * a[5];
            inv[15] = a[0] * a[5] * a[10] - a[0] * a[6] * a[9] - a[4] * a[1] * a[10] + a[4] * a[2] * a[9] + a[8] * a[1] * a[6] - a[8] * a[2] * a[5];
            let det = a[0] * inv[0] + a[1] * inv[4] + a[2] * inv[8] + a[3] * inv[12];
            let d = 1.0 / det;
            for v in inv.iter_mut() {
                *v *= d;
            }
            Self { m: inv }
        }
        /// The point v moved by this matrix (w = 1).
        pub fn transform_position(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z + self.at(0, 3),
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z + self.at(1, 3),
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z + self.at(2, 3),
            )
        }
        /// The direction v turned by this matrix (w = 0).
        pub fn transform_direction(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z,
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z,
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z,
            )
        }
        /// JOML's set3x3: the upper-left 3x3 from `m`, the rest kept.
        pub fn set3x3(self, m: JMat3) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = m.m[col * 3 + row];
                }
            }
            out
        }
        /// JOML's transpose3x3: the upper-left 3x3 transposed, the rest kept.
        pub fn transpose3x3(self) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = self.m[row * 4 + col];
                }
            }
            out
        }
        pub fn get_translation(&self) -> JVec3 {
            JVec3::new(self.m[12], self.m[13], self.m[14])
        }
    }
    
    /// Minecraft's PoseStack: a stack of matrices (normals aren't kept: Horizon's hand renderer takes one matrix).
    #[derive(Clone, Debug)]
    pub struct JPoseStack {
        pub stack: Vec<JMat4>,
    }
    impl Default for JPoseStack {
        fn default() -> Self {
            Self { stack: vec![JMat4::IDENTITY] }
        }
    }
    impl JPoseStack {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn of(m: JMat4) -> Self {
            Self { stack: vec![m] }
        }
        fn top(&mut self) -> &mut JMat4 {
            self.stack.last_mut().expect("PoseStack is empty")
        }
        pub fn push_pose(&mut self) {
            let m = *self.top();
            self.stack.push(m);
        }
        pub fn pop_pose(&mut self) {
            self.stack.pop();
        }
        /// Writes the top matrix (Java changed `pose.last().pose()` in place).
        pub fn set_last(&mut self, m: JMat4) {
            *self.top() = m;
        }
        pub fn last_pose(&self) -> JMat4 {
            *self.stack.last().expect("PoseStack is empty")
        }
        pub fn set_identity(&mut self) {
            *self.top() = JMat4::IDENTITY;
        }
        pub fn translate(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.translate(x, y, z);
        }
        pub fn scale(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.scale(x, y, z);
        }
        pub fn mul_pose(&mut self, q: JQuat) {
            let m = self.top();
            *m = m.rotate(q);
        }
        pub fn mul_pose_matrix(&mut self, r: JMat4) {
            let m = self.top();
            *m = m.mul(r);
        }
        pub fn rotate_around(&mut self, q: JQuat, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.rotate_around(q, x, y, z);
        }
    }
    
    #[cfg(test)]
    mod joml_tests {
        use super::*;
    
        fn close(a: JVec3, b: JVec3) -> bool {
            (a.x - b.x).abs() < 1e-5 && (a.y - b.y).abs() < 1e-5 && (a.z - b.z).abs() < 1e-5
        }
    
        #[test]
        fn rotations_turn_axes_as_joml_does() {
            let q = JQuat::rotation_x(std::f32::consts::FRAC_PI_2);
            // A positive turn about X takes +Y to +Z (right-handed), as JOML's rotationX.
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(0.0, 0.0, 1.0)));
            let q = JQuat::rotation_y(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 0.0, 1.0)), JVec3::new(1.0, 0.0, 0.0)));
            let q = JQuat::rotation_z(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
        }
    
        #[test]
        fn translate_then_rotate_multiplies_on_the_right() {
            // JOML: new Matrix4f().translate(1, 0, 0).rotateZ(90) moves (1, 0, 0) to (1, 1, 0).
            let m = JMat4::identity().translate(1.0, 0.0, 0.0).rotate_z(std::f32::consts::FRAC_PI_2);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(1.0, 1.0, 0.0)));
        }
    
        #[test]
        fn invert_undoes() {
            let m = JMat4::identity().translate(1.0, 2.0, 3.0).rotate_x(0.7).rotate_y(-0.3).scale(2.0, 0.5, 1.5);
            let p = JVec3::new(0.3, -1.2, 4.0);
            assert!(close(m.invert().transform_position(m.transform_position(p)), p));
            assert!(close(m.mul(m.invert()).transform_position(p), p));
        }
    
        #[test]
        fn rotate_around_keeps_its_point() {
            let m = JMat4::identity().rotate_around(JQuat::rotation_y(1.1), 0.5, 0.25, 0.0);
            assert!(close(m.transform_position(JVec3::new(0.5, 0.25, 0.0)), JVec3::new(0.5, 0.25, 0.0)));
        }
    
        #[test]
        fn pose_stack_push_pop() {
            let mut p = JPoseStack::new();
            p.translate(1.0, 0.0, 0.0);
            p.push_pose();
            p.scale(2.0, 2.0, 2.0);
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(3.0, 0.0, 0.0)));
            p.pop_pose();
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(2.0, 0.0, 0.0)));
        }
    
        #[test]
        fn quaternion_matrix_agrees_with_axis_turns() {
            let q = JQuat::rotation_x(0.4).mul(JQuat::rotation_y(-0.9));
            let a = JMat4::rotation(q);
            let b = JMat4::identity().rotate_x(0.4).rotate_y(-0.9);
            let p = JVec3::new(1.0, 2.0, 3.0);
            assert!(close(a.transform_position(p), b.transform_position(p)));
        }
    
        #[test]
        fn set3x3_and_transpose() {
            let m3 = JMat3::from_cols(JVec3::new(0.0, 1.0, 0.0), JVec3::new(-1.0, 0.0, 0.0), JVec3::new(0.0, 0.0, 1.0));
            let m = JMat4::identity().set3x3(m3);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
            assert!(close(m.transpose3x3().transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(1.0, 0.0, 0.0)));
        }
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
        assert_eq!(ID, "betterend");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
