// AmbientSounds, ported to Horizon by horizon-port from AmbientSounds_FABRIC_v6.3.11_mc26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_ambientsounds). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "ambientsounds";
pub const NAME: &str = "AmbientSounds";
pub const VERSION: &str = "6.3.11";
pub const DESCRIPTION: &str = "Expands minecraft's ambient sounds.";

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

    /// `AmbientSounds` (team/creative/ambientsounds/AmbientSounds.java)
    pub mod ambient_sounds {
        use super::*;
        // The class itself isn't translated: it implements `ClientLoader` (team.creative.creativecore.client). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        pub const MODID: &str = "ambientsounds";
        // CONFIG: not translated, it uses AmbientSoundsConfig, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // TICK_HANDLER: not translated, it uses AmbientTickHandler, which isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))).
        // AmbientSounds.AmbientSounds(0): not translated, its body couldn't be read: an anonymous class.
        // AmbientSounds.scheduleReload(0): not translated, a nullable object kept in a changing static.
        // AmbientSounds.reloadAsync(0): not translated, a method reference.
        // AmbientSounds.reload(0): not translated, a nullable object kept in a changing static.
        // AmbientSounds.onInitializeClient(0): not translated, its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)).
        // AmbientSounds.registerClientCommands(1): not translated, it's generic.
    }

    /// `AmbientSoundsConfig` (team/creative/ambientsounds/AmbientSoundsConfig.java)
    pub mod ambient_sounds_config {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // AmbientSoundsConfig.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `AmbientBlock` (team/creative/ambientsounds/block/AmbientBlock.java)
    pub mod ambient_block {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AmbientBlock {
        }
        impl AmbientBlock {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // AmbientBlock.is(1): not translated, it has no body.
        }
        // AmbientBlock.parse(1): not translated, it uses `TupleList` (team.creative.creativecore.common.util.type.list).
        // AmbientBlock.parseFirst(1): not translated, it creates a AmbientBlockTag, which isn't translated (it extends `AmbientBlock`).
    }

    /// `AmbientBlock.AmbientBlockBlock` (team/creative/ambientsounds/block/AmbientBlock.java)
    pub mod ambient_block_ambient_block_block {
        use super::*;
        // The class itself isn't translated: it extends `AmbientBlock`. Its static parts are below.
        // AmbientBlock.AmbientBlockBlock.AmbientBlockBlock(1): not translated, its class isn't translated (it extends `AmbientBlock`).
        // AmbientBlock.AmbientBlockBlock.is(1): not translated, its class isn't translated (it extends `AmbientBlock`).
    }

    /// `AmbientBlock.AmbientBlockProperty` (team/creative/ambientsounds/block/AmbientBlock.java)
    pub mod ambient_block_ambient_block_property {
        use super::*;
        // The class itself isn't translated: it extends `AmbientBlock`. Its static parts are below.
        // AmbientBlock.AmbientBlockProperty.AmbientBlockProperty(2): not translated, its class isn't translated (it extends `AmbientBlock`).
        // AmbientBlock.AmbientBlockProperty.is(1): not translated, its class isn't translated (it extends `AmbientBlock`).
    }

    /// `AmbientBlock.AmbientBlockTag` (team/creative/ambientsounds/block/AmbientBlock.java)
    pub mod ambient_block_ambient_block_tag {
        use super::*;
        // The class itself isn't translated: it extends `AmbientBlock`. Its static parts are below.
        // AmbientBlock.AmbientBlockTag.AmbientBlockTag(1): not translated, its class isn't translated (it extends `AmbientBlock`).
        // AmbientBlock.AmbientBlockTag.is(1): not translated, its class isn't translated (it extends `AmbientBlock`).
    }

    /// `AmbientBlockGroup` (team/creative/ambientsounds/block/AmbientBlockGroup.java)
    pub mod ambient_block_group {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AmbientBlockGroup {
            pub data: Vec<String>,
            pub filters: Option<Vec<ambient_block::AmbientBlock>>,
        }
        impl AmbientBlockGroup {
            pub fn new() -> Self {
                let mut s = Self { data: Vec::new(), filters: None };
                s
            }
            // AmbientBlockGroup.onClientLoad(0): not translated, a try/catch (AmbientEngineLoadException).
            pub fn add(&mut self, data: &[String]) {
                self.data.extend(data.to_vec().iter().cloned().collect::<Vec<_>>());
            }
            pub fn is_empty(&self) -> bool {
                return (self.filters.as_ref().expect("null")).is_empty();
            }
            // AmbientBlockGroup.is(1): not translated, it uses Minecraft's `BlockState`.
        }
    }

    /// `AmbientCondition` (team/creative/ambientsounds/condition/AmbientCondition.java)
    pub mod ambient_condition {
        use super::*;
        // The class itself isn't translated: it extends `AmbientSoundProperties` (team.creative.ambientsounds.sound). Its static parts are below.
        // AmbientCondition.regionName(0): not translated, its class isn't translated (it extends `AmbientSoundProperties` (team.creative.ambientsounds.sound)).
        // AmbientCondition.init(1): not translated, its class isn't translated (it extends `AmbientSoundProperties` (team.creative.ambientsounds.sound)).
        // AmbientCondition.value(1): not translated, its class isn't translated (it extends `AmbientSoundProperties` (team.creative.ambientsounds.sound)).
    }

    /// `AmbientCondition.AmbientMinMaxCondition` (team/creative/ambientsounds/condition/AmbientCondition.java)
    pub mod ambient_condition_ambient_min_max_condition {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AmbientMinMaxCondition {
            pub min: Option<f64>,
            pub max: Option<f64>,
        }
        impl AmbientMinMaxCondition {
            pub fn new() -> Self {
                let mut s = Self { min: None, max: None };
                s
            }
            pub fn is(&self, value: f64) -> bool {
                return if self.min.is_some() && (value < (self.min.expect("null"))) { false } else { self.max.is_none() || (!(value > (self.max.expect("null")))) };
            }
            // AmbientCondition.AmbientMinMaxCondition.randomValue(0): not translated, Math.random (no random numbers without a crate).
        }
    }

    /// `AmbientCondition.AmbientMinMaxFadeCondition` (team/creative/ambientsounds/condition/AmbientCondition.java)
    pub mod ambient_condition_ambient_min_max_fade_condition {
        use super::*;
        // The class itself isn't translated: it extends `AmbientMinMaxCondition`. Its static parts are below.
        // AmbientCondition.AmbientMinMaxFadeCondition.volume(1): not translated, its class isn't translated (it extends `AmbientMinMaxCondition`).
    }

    /// `AmbientCondition.AmbientMinMaxFadeSpecialCondition` (team/creative/ambientsounds/condition/AmbientCondition.java)
    pub mod ambient_condition_ambient_min_max_fade_special_condition {
        use super::*;
        // The class itself isn't translated: it extends `AmbientMinMaxFadeCondition`. Its static parts are below.
        // AmbientCondition.AmbientMinMaxFadeSpecialCondition.volume(3): not translated, its class isn't translated (it extends `AmbientMinMaxFadeCondition`).
    }

    /// `AmbientSelection` (team/creative/ambientsounds/condition/AmbientSelection.java)
    pub mod ambient_selection {
        use super::*;
        // The class itself isn't translated: it extends `AmbientVolume`. Its static parts are below.
        // AmbientSelection.AmbientSelection(1): not translated, its class isn't translated (it extends `AmbientVolume`).
        // AmbientSelection.assign(1): not translated, its class isn't translated (it extends `AmbientVolume`).
        // AmbientSelection.conditionVolume(0): not translated, its class isn't translated (it extends `AmbientVolume`).
        // AmbientSelection.settingVolume(0): not translated, its class isn't translated (it extends `AmbientVolume`).
        // AmbientSelection.volume(0): not translated, its class isn't translated (it extends `AmbientVolume`).
        // AmbientSelection.last(0): not translated, its class isn't translated (it extends `AmbientVolume`).
        // AmbientSelection.getProperties(0): not translated, its class isn't translated (it extends `AmbientVolume`).
        // AmbientSelection.assignProperties(1): not translated, its class isn't translated (it extends `AmbientVolume`).
    }

    /// `AmbientSelectionMulti` (team/creative/ambientsounds/condition/AmbientSelectionMulti.java)
    pub mod ambient_selection_multi {
        use super::*;
        // The class itself isn't translated: it extends `AmbientSelection`. Its static parts are below.
        // AmbientSelectionMulti.AmbientSelectionMulti(2): not translated, its class isn't translated (it extends `AmbientSelection`).
        // AmbientSelectionMulti.conditionVolume(0): not translated, its class isn't translated (it extends `AmbientSelection`).
        // AmbientSelectionMulti.settingVolume(0): not translated, its class isn't translated (it extends `AmbientSelection`).
        // AmbientSelectionMulti.volume(0): not translated, its class isn't translated (it extends `AmbientSelection`).
        // AmbientSelectionMulti.assignProperties(1): not translated, its class isn't translated (it extends `AmbientSelection`).
    }

    /// `AmbientTime` (team/creative/ambientsounds/condition/AmbientTime.java)
    pub mod ambient_time {
        use super::*;
        // PREMADE: not translated, it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`).
        pub const ANGLE_TO_TIME: f64 = 0.06666666666666667f64;
        pub const TIME_TO_ANGLE: f64 = 15.0f64;
        pub const FADE: f64 = 1.0f64;
        // NONE: not translated, its value couldn't be read: an anonymous class.
        // DAY: not translated, it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`).
        // NIGHT: not translated, it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`).
        // SUNRISE: not translated, it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`).
        // SUNSET: not translated, it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`).
        // NOON: not translated, it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`).
        // MIDNIGHT: not translated, it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`).
        #[derive(Clone, Debug)]
        pub struct AmbientTime {
        }
        impl AmbientTime {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // AmbientTime.value(1): not translated, it has no body.
            // AmbientTime.toJson(0): not translated, it has no body.
        }
    }

    /// `AmbientTime.AmbientTimeHour` (team/creative/ambientsounds/condition/AmbientTime.java)
    pub mod ambient_time_ambient_time_hour {
        use super::*;
        // The class itself isn't translated: it extends `AmbientTime`. Its static parts are below.
        // AmbientTime.AmbientTimeHour.AmbientTimeHour(1): not translated, its class isn't translated (it extends `AmbientTime`).
        // AmbientTime.AmbientTimeHour.value(1): not translated, its class isn't translated (it extends `AmbientTime`).
        // AmbientTime.AmbientTimeHour.toJson(0): not translated, its class isn't translated (it extends `AmbientTime`).
    }

    /// `AmbientTime.AmbientTimeMulti` (team/creative/ambientsounds/condition/AmbientTime.java)
    pub mod ambient_time_ambient_time_multi {
        use super::*;
        // The class itself isn't translated: it extends `AmbientTime`. Its static parts are below.
        // AmbientTime.AmbientTimeMulti.AmbientTimeMulti(1): not translated, its class isn't translated (it extends `AmbientTime`).
        // AmbientTime.AmbientTimeMulti.value(1): not translated, its class isn't translated (it extends `AmbientTime`).
        // AmbientTime.AmbientTimeMulti.toJson(0): not translated, its class isn't translated (it extends `AmbientTime`).
    }

    /// `AmbientTime.AmbientTimePremade` (team/creative/ambientsounds/condition/AmbientTime.java)
    pub mod ambient_time_ambient_time_premade {
        use super::*;
        // The class itself isn't translated: it extends `AmbientTimeHour`. Its static parts are below.
        // AmbientTime.AmbientTimePremade.AmbientTimePremade(4): not translated, its class isn't translated (it extends `AmbientTimeHour`).
        // AmbientTime.AmbientTimePremade.AmbientTimePremade(2): not translated, its class isn't translated (it extends `AmbientTimeHour`).
        // AmbientTime.AmbientTimePremade.toJson(0): not translated, its class isn't translated (it extends `AmbientTimeHour`).
    }

    /// `AmbientTime.AmbientTimeSunAngle` (team/creative/ambientsounds/condition/AmbientTime.java)
    pub mod ambient_time_ambient_time_sun_angle {
        use super::*;
        // The class itself isn't translated: it extends `AmbientTime`. Its static parts are below.
        // AmbientTime.AmbientTimeSunAngle.AmbientTimeSunAngle(1): not translated, its class isn't translated (it extends `AmbientTime`).
        // AmbientTime.AmbientTimeSunAngle.value(1): not translated, its class isn't translated (it extends `AmbientTime`).
        // AmbientTime.AmbientTimeSunAngle.toJson(0): not translated, its class isn't translated (it extends `AmbientTime`).
    }

    /// `AmbientTime.Serializer` (team/creative/ambientsounds/condition/AmbientTime.java)
    pub mod ambient_time_serializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer` (com.google.gson). Its static parts are below.
        // AmbientTime.Serializer.serialize(3): not translated, its class isn't translated (it implements `JsonDeserializer` (com.google.gson)).
        // AmbientTime.Serializer.deserialize(3): not translated, its class isn't translated (it implements `JsonDeserializer` (com.google.gson)).
    }

    /// `AmbientVolume` (team/creative/ambientsounds/condition/AmbientVolume.java)
    pub mod ambient_volume {
        use super::*;
        // SILENT: not translated, its value couldn't be read: an anonymous class.
        // MAX: not translated, its value couldn't be read: an anonymous class.
        #[derive(Clone, Debug)]
        pub struct AmbientVolume {
            pub condition_volume: f64,
            pub setting_volume: f64,
        }
        impl AmbientVolume {
            pub fn new_2(condition_volume: f64, setting_volume: f64) -> Self {
                let mut s = Self { condition_volume: condition_volume, setting_volume: setting_volume };
                s.condition_volume = condition_volume;
                s.setting_volume = setting_volume;
                s
            }
            pub fn new() -> Self {
                let mut s = Self::new_2(1.0f64, 1.0f64);
                s
            }
            pub fn assign(&mut self, volume: &ambient_volume::AmbientVolume) {
                self.condition_volume = volume.condition_volume;
                self.setting_volume = volume.setting_volume;
            }
            pub fn set_condition_volume_direct(&mut self, volume: f64) {
                self.condition_volume = volume;
            }
            pub fn mul_volume(&mut self, selection: &ambient_volume::AmbientVolume) {
                self.mul_condition(selection.condition_volume());
                self.mul_setting(selection.setting_volume());
            }
            pub fn mul_condition(&mut self, volume: f64) {
                self.condition_volume *= volume;
            }
            pub fn mul_setting(&mut self, volume: f64) {
                self.setting_volume *= volume;
            }
            pub fn condition_volume(&self) -> f64 {
                return self.condition_volume;
            }
            pub fn setting_volume(&self) -> f64 {
                return self.setting_volume;
            }
            pub fn volume(&self) -> f64 {
                return self.setting_volume * self.condition_volume;
            }
            // AmbientVolume.toString(0): not translated, it uses `DebugTextRenderer` (team.creative.creativecore.client.render.text).DECIMAL_FORMAT.
            pub fn copy(&self) -> ambient_volume::AmbientVolume {
                return ambient_volume::AmbientVolume::new_2(self.condition_volume, self.setting_volume);
            }
        }
    }

    /// `BiomeCondition` (team/creative/ambientsounds/condition/BiomeCondition.java)
    pub mod biome_condition {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BiomeCondition {
            pub tag: bool,
            pub pattern: JRegex,
        }
        impl BiomeCondition {
            // BiomeCondition.BiomeCondition(2): not translated, it stores a second reference to an object (Rust would need it copied or shared).
            // BiomeCondition.matches(1): not translated, it calls BiomeMatchCache.matches, which isn't translated.
            pub fn tag(&self) -> bool {
                return self.tag;
            }
            // BiomeCondition.pattern(0): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        }
        // BiomeCondition.of(1): not translated, it's overloaded with the same number of parameters.
        // BiomeCondition.of(1): not translated, it's overloaded with the same number of parameters.
    }

    /// `BiomeMatchCache` (team/creative/ambientsounds/condition/BiomeMatchCache.java)
    pub mod biome_match_cache {
        use super::*;
        pub const MAX_LOCATIONS_PER_PATTERN: i32 = 256;
        // CACHE: not translated, it uses `ThreadLocal`.
        #[derive(Clone, Debug)]
        pub struct BiomeMatchCache {
        }
        impl BiomeMatchCache {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BiomeMatchCache.matches(2): not translated, a Map or Set keyed by a Pattern.
    }

    /// `AmbientDimension` (team/creative/ambientsounds/dimension/AmbientDimension.java)
    pub mod ambient_dimension {
        use super::*;
        // The class itself isn't translated: it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)). Its static parts are below.
        // AmbientDimension.load(4): not translated, its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))).
        // AmbientDimension.init(1): not translated, its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))).
        // AmbientDimension.is(1): not translated, its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))).
        // AmbientDimension.manipulateEnviroment(1): not translated, its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))).
        // AmbientDimension.toString(0): not translated, its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))).
    }

    /// `AmbientEngine` (team/creative/ambientsounds/engine/AmbientEngine.java)
    pub mod ambient_engine {
        use super::*;
        // The class itself isn't translated: it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))). Its static parts are below.
        pub static CONFIG_LOCATION: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::try_build("ambientsounds", "config.json").expect("null"));
        pub const ENGINE_LOCATION: &str = "engine.json";
        pub const DIMENSIONS_LOCATION: &str = "dimensions";
        pub const REGIONS_LOCATION: &str = "regions";
        pub const SOUNDCOLLECTIONS_LOCATION: &str = "sound_collections";
        pub const SOUNDCATEGORIES_LOCATION: &str = "sound_categories";
        pub const BLOCKGROUPS_LOCATION: &str = "blockgroups";
        pub const FEATURES_LOCATION: &str = "features";
        // GSON: not translated, Gson.registerTypeAdapter.
        pub static LOADED_ENGINE: LazyLock<JCell<Option<String>>> = LazyLock::new(|| JCell::new(None));
        pub fn has_loaded_at_least_once() -> bool {
            return ambient_engine::LOADED_ENGINE.get().is_some();
        }
        pub fn has_engine_changed(new_engine: &str) -> bool {
            return (ambient_engine::LOADED_ENGINE.get().is_none()) || (!(j_eq(&(ambient_engine::LOADED_ENGINE.get().expect("null")), &new_engine)));
        }
        // AmbientEngine.attemptToLoadEngine(3): not translated, it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.applyStackType(3): not translated, it's generic.
        // AmbientEngine.loadMultiple(5): not translated, it's generic.
        // AmbientEngine.loadAmbientEngine(1): not translated, it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.getRegion(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.checkRegion(3): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.addRegion(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.getDimension(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.stopEngine(0): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.consumeSoundCollections(2): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.airPocketVolume(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.init(0): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.onClientLoad(0): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.airWeightFactor(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.tick(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.fastTick(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.changeDimension(2): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.collectDetails(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEngine.getSoundCategory(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
    }

    /// `AmbientEngine.Serializer` (team/creative/ambientsounds/engine/AmbientEngine.java)
    pub mod ambient_engine_serializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer` (com.google.gson). Its static parts are below.
        // AmbientEngine.Serializer.deserialize(3): not translated, its class isn't translated (it implements `JsonDeserializer` (com.google.gson)).
        // AmbientEngine.Serializer.serialize(3): not translated, its class isn't translated (it implements `JsonDeserializer` (com.google.gson)).
    }

    /// `AmbientEngineConfig` (team/creative/ambientsounds/engine/AmbientEngineConfig.java)
    pub mod ambient_engine_config {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AmbientEngineConfig {
            pub default_engine: String,
            pub engines: Option<Vec<String>>,
        }
        impl AmbientEngineConfig {
            pub fn new() -> Self {
                let mut s = Self { default_engine: "basic".to_string(), engines: None };
                s
            }
        }
    }

    /// `AmbientEngineLoadException` (team/creative/ambientsounds/engine/AmbientEngineLoadException.java)
    pub mod ambient_engine_load_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        // AmbientEngineLoadException.AmbientEngineLoadException(1): not translated, its class isn't translated (it extends `Exception`).
    }

    /// `AmbientLoader` (team/creative/ambientsounds/engine/AmbientLoader.java)
    pub mod ambient_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AmbientLoader.setNameAndLoad(3): not translated, it has no body.
    }

    /// `AmbientStackType` (team/creative/ambientsounds/engine/AmbientStackType.java)
    pub mod ambient_stack_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // AmbientStackType.apply(3): not translated, it has no body.
    }

    /// `AmbientTickHandler` (team/creative/ambientsounds/engine/AmbientTickHandler.java)
    pub mod ambient_tick_handler {
        use super::*;
        // The class itself isn't translated: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))). Its static parts are below.
        // mc: not translated, it uses Minecraft's `Minecraft`.
        // AmbientTickHandler.scheduleReload(0): not translated, its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))).
        // AmbientTickHandler.setEngine(1): not translated, its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))).
        // AmbientTickHandler.initConfiguration(0): not translated, its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))).
        // AmbientTickHandler.createSoundCategoryConfiguration(3): not translated, its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))).
        // AmbientTickHandler.onRender(1): not translated, its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))).
        // AmbientTickHandler.loadLevel(1): not translated, its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))).
        // AmbientTickHandler.onTick(0): not translated, its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))).
    }

    /// `AmbientEntityCondition` (team/creative/ambientsounds/entity/AmbientEntityCondition.java)
    pub mod ambient_entity_condition {
        use super::*;
        // The class itself isn't translated: it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`). Its static parts are below.
        // AmbientEntityCondition.init(1): not translated, its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)).
        // AmbientEntityCondition.value(1): not translated, its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)).
        // AmbientEntityCondition.hasNbt(2): not translated, its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)).
        // AmbientEntityCondition.isTeam(2): not translated, its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)).
        // AmbientEntityCondition.containsTag(2): not translated, its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)).
        // AmbientEntityCondition.checkEntityName(2): not translated, its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)).
    }

    /// `AmbientEntityCondition.StringJson` (team/creative/ambientsounds/entity/AmbientEntityCondition.java)
    pub mod ambient_entity_condition_string_json {
        use super::*;
        // The class itself isn't translated: it extends `TypeAdapter` (com.google.gson). Its static parts are below.
        // AmbientEntityCondition.StringJson.write(2): not translated, its class isn't translated (it extends `TypeAdapter` (com.google.gson)).
        // AmbientEntityCondition.StringJson.read(1): not translated, its class isn't translated (it extends `TypeAdapter` (com.google.gson)).
    }

    /// `AmbientEnvironment` (team/creative/ambientsounds/environment/AmbientEnvironment.java)
    pub mod ambient_environment {
        use super::*;
        // The class itself isn't translated: it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))). Its static parts are below.
        // AmbientEnvironment.isRainAudibleAtSurface(0): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.analyzeFast(5): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.analyzeTime(3): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.isEyeInFluid(2): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.isWater(2): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.analyzeUnderwater(3): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.analyzeSlow(5): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.collectLevelDetails(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.collectPlayerDetails(2): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.collectTerrainDetails(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.collectBiomeDetails(1): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        // AmbientEnvironment.reload(0): not translated, its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
    }

    /// `BiomeEnvironment` (team/creative/ambientsounds/environment/BiomeEnvironment.java)
    pub mod biome_environment {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        // BiomeEnvironment.BiomeEnvironment(0): not translated, its class isn't translated (it implements `Iterable`).
        // BiomeEnvironment.BiomeEnvironment(4): not translated, its class isn't translated (it implements `Iterable`).
        // BiomeEnvironment.iterator(0): not translated, its class isn't translated (it implements `Iterable`).
        // BiomeEnvironment.rainVolume(0): not translated, its class isn't translated (it implements `Iterable`).
        // BiomeEnvironment.collectDetails(1): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `BiomeEnvironment.BiomeArea` (team/creative/ambientsounds/environment/BiomeEnvironment.java)
    pub mod biome_environment_biome_area {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Holder`. Its static parts are below.
        // BiomeEnvironment.BiomeArea.BiomeArea(2): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BiomeEnvironment.BiomeArea.checkBiome(1): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BiomeEnvironment.BiomeArea.equals(1): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BiomeEnvironment.BiomeArea.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
    }

    /// `EntityEnvironment` (team/creative/ambientsounds/environment/EntityEnvironment.java)
    pub mod entity_environment {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Entity`. Its static parts are below.
        // EntityEnvironment.analyzeFast(4): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityEnvironment.squaredDistance(1): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityEnvironment.x(0): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityEnvironment.y(0): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityEnvironment.z(0): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityEnvironment.all(0): not translated, its class isn't translated (it uses Minecraft's `Entity`).
    }

    /// `AmbientFeature` (team/creative/ambientsounds/environment/feature/AmbientFeature.java)
    pub mod ambient_feature {
        use super::*;
        // The class itself isn't translated: it extends `AmbientFeatureSelection`. Its static parts are below.
    }

    /// `AmbientFeatureSelection` (team/creative/ambientsounds/environment/feature/AmbientFeatureSelection.java)
    pub mod ambient_feature_selection {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AmbientFeatureSelection {
            pub group: Vec<ambient_block_group::AmbientBlockGroup>,
            pub groups: Option<Vec<String>>,
            pub and: Option<Vec<ambient_feature_selection::AmbientFeatureSelection>>,
            pub or: Option<Vec<ambient_feature_selection::AmbientFeatureSelection>>,
            pub not: Option<Vec<ambient_feature_selection::AmbientFeatureSelection>>,
            pub low_weight: Option<f64>,
            pub high_weight: Option<f64>,
            pub low_count: Option<f64>,
            pub high_count: Option<f64>,
        }
        impl AmbientFeatureSelection {
            pub fn new() -> Self {
                let mut s = Self { group: Vec::new(), groups: None, and: None, or: None, not: None, low_weight: None, high_weight: None, low_count: None, high_count: None };
                s
            }
            // AmbientFeatureSelection.collectGroups(1): not translated, Set.addAll of a List<String[]>.
            // AmbientFeatureSelection.volume(1): not translated, it's overloaded with the same number of parameters.
        }
    }

    /// `AirPocket` (team/creative/ambientsounds/environment/pocket/AirPocket.java)
    pub mod air_pocket {
        use super::*;
        // The class itself isn't translated: it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map). Its static parts are below.
        // AirPocket.AirPocket(0): not translated, its class isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map)).
        // AirPocket.AirPocket(7): not translated, its class isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map)).
        // AirPocket.volume(1): not translated, its class isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map)).
    }

    /// `AirPocketGroup` (team/creative/ambientsounds/environment/pocket/AirPocketGroup.java)
    pub mod air_pocket_group {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AirPocketGroup {
            pub distance: i32,
            pub weight: f64,
        }
        impl AirPocketGroup {
            pub fn new() -> Self {
                let mut s = Self { distance: 0i32, weight: 0.0f64 };
                s
            }
        }
    }

    /// `AirPocketScanner` (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java)
    pub mod air_pocket_scanner {
        use super::*;
        // The class itself isn't translated: it extends `Thread`. Its static parts are below.
        // AirPocketScanner.AirPocketScanner(4): not translated, its class isn't translated (it extends `Thread`).
        // AirPocketScanner.run(0): not translated, its class isn't translated (it extends `Thread`).
        // AirPocketScanner.getOrCreate(1): not translated, its class isn't translated (it extends `Thread`).
        // AirPocketScanner.findState(2): not translated, its class isn't translated (it extends `Thread`).
        // AirPocketScanner.hearThrough(3): not translated, its class isn't translated (it extends `Thread`).
        // AirPocketScanner.scan(3): not translated, its class isn't translated (it extends `Thread`).
        // AirPocketScanner.canSeeSkyConsiderSolids(2): not translated, its class isn't translated (it extends `Thread`).
    }

    /// `AirPocketScanner.BlockPosInspection` (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java)
    pub mod air_pocket_scanner_block_pos_inspection {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockPos`. Its static parts are below.
        // AirPocketScanner.BlockPosInspection.BlockPosInspection(1): not translated, its class isn't translated (it extends Minecraft's `BlockPos`).
        // AirPocketScanner.BlockPosInspection.BlockPosInspection(2): not translated, its class isn't translated (it extends Minecraft's `BlockPos`).
        // AirPocketScanner.BlockPosInspection.add(1): not translated, its class isn't translated (it extends Minecraft's `BlockPos`).
        // AirPocketScanner.BlockPosInspection.isUp(0): not translated, its class isn't translated (it extends Minecraft's `BlockPos`).
        // AirPocketScanner.BlockPosInspection.is(1): not translated, its class isn't translated (it extends Minecraft's `BlockPos`).
        // AirPocketScanner.BlockPosInspection.iterator(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `BlockDistribution` (team/creative/ambientsounds/environment/pocket/BlockDistribution.java)
    pub mod block_distribution {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BlockDistribution {
            pub percentage: f64,
            pub count: f64,
        }
        impl BlockDistribution {
            pub fn new() -> Self {
                let mut s = Self { percentage: 0.0f64, count: 0.0f64 };
                s
            }
            // BlockDistribution.add(1): not translated, it's overloaded with the same number of parameters.
            pub fn calculate_percentage(&mut self, total: f64) {
                self.percentage = self.count / total;
            }
            // BlockDistribution.add(1): not translated, it's overloaded with the same number of parameters.
            pub fn to_string(&self) -> String {
                return format!("c:{} ({})", j_float(self.count), j_float(self.percentage));
            }
        }
    }

    /// `TerrainEnvironment` (team/creative/ambientsounds/environment/TerrainEnvironment.java)
    pub mod terrain_environment {
        use super::*;
        // The class itself isn't translated: it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map)). Its static parts are below.
        // POSSIBLE_HEIGHT_BLOCK: not translated, it uses Minecraft's `BlockState`.
        // TerrainEnvironment.getHeightBlock(2): not translated, it uses Minecraft's `Level`.
        // TerrainEnvironment.getHeightScanStart(2): not translated, it uses Minecraft's `Level`.
        // TerrainEnvironment.TerrainEnvironment(0): not translated, its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))).
        // TerrainEnvironment.analyze(4): not translated, its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))).
        // TerrainEnvironment.analyzeHeight(4): not translated, its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))).
        // TerrainEnvironment.analyzeAirPocket(3): not translated, its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))).
        // TerrainEnvironment.collectDetails(1): not translated, its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))).
    }

    /// `SereneSeasonsCompat` (team/creative/ambientsounds/mod/SereneSeasonsCompat.java)
    pub mod serene_seasons_compat {
        use super::*;
        // getBiomeTemperature: not translated, it uses `Method` (java.lang.reflect).
        #[derive(Clone, Debug)]
        pub struct SereneSeasonsCompat {
        }
        impl SereneSeasonsCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SereneSeasonsCompat.getTemperature(1): not translated, it uses Minecraft's `Player`.
    }

    /// `AmbientRegion` (team/creative/ambientsounds/region/AmbientRegion.java)
    pub mod ambient_region {
        use super::*;
        // The class itself isn't translated: it extends `AmbientCondition` (team.creative.ambientsounds.condition). Its static parts are below.
        // AmbientRegion.load(3): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.regionName(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.init(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.value(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.fastTick(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.tick(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.isActive(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.activate(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.deactivate(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientRegion.toString(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
    }

    /// `AmbientSound` (team/creative/ambientsounds/sound/AmbientSound.java)
    pub mod ambient_sound {
        use super::*;
        // The class itself isn't translated: it extends `AmbientCondition` (team.creative.ambientsounds.condition). Its static parts are below.
        // RANDOM: not translated, it creates `Random` (java.util).
        // COPYFIELDS: not translated, it uses `Field` (java.lang.reflect).
        // AmbientSound.AmbientSound(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.getSoundSource(1): not translated, it uses Minecraft's `SoundSource`.
        // AmbientSound.init(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.getRandomFile(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.getRandomFileExcept(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.fastTick(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.value(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.isAudible(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.tick(2): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.play(2): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.playTransition(2): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.isPlaying(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.inTransition(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.isActive(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.activate(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.deactivate(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.onSoundFinished(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.loop(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.toString(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSound.copy(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
    }

    /// `AmbientSound.SoundStream` (team/creative/ambientsounds/sound/AmbientSound.java)
    pub mod ambient_sound_sound_stream {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `TickableSoundInstance`. Its static parts are below.
        // rand: not translated, it calls Minecraft's `RandomSource`.createThreadLocalInstance.
        // AmbientSound.SoundStream.SoundStream(1): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.loop(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.remaining(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.conditionVolume(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.combinedVolume(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.mute(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.mutePriority(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.onStart(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.onFinished(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.hasPlayedOnce(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.setPlayedOnce(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.hasFinished(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.toString(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.isLooping(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getSoundEvent(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getOrResolve(1): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getAttenuation(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getSource(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getPitch(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getDelay(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getSound(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getIdentifier(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getVolume(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getX(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getY(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getZ(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.isStopped(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.tick(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.isRelative(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.canStartSilent(0): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
        // AmbientSound.SoundStream.getAudioStream(3): not translated, its body couldn't be read: an anonymous class.
        // AmbientSound.SoundStream.collectDetails(1): not translated, its class isn't translated (it implements Minecraft's `TickableSoundInstance`).
    }

    /// `AmbientSoundCategory` (team/creative/ambientsounds/sound/AmbientSoundCategory.java)
    pub mod ambient_sound_category {
        use super::*;
        // The class itself isn't translated: it extends `AmbientCondition` (team.creative.ambientsounds.condition). Its static parts are below.
        // AmbientSoundCategory.init(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSoundCategory.hashCode(0): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSoundCategory.postInit(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSoundCategory.tick(2): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
        // AmbientSoundCategory.collectDetails(1): not translated, its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)).
    }

    /// `AmbientSoundCollection` (team/creative/ambientsounds/sound/AmbientSoundCollection.java)
    pub mod ambient_sound_collection {
        use super::*;
        // The class itself isn't translated: it uses AmbientStackType, which isn't translated (its constants carry values). Its static parts are below.
    }

    /// `AmbientSoundEngine` (team/creative/ambientsounds/sound/AmbientSoundEngine.java)
    pub mod ambient_sound_engine {
        use super::*;
        // The class itself isn't translated: it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`). Its static parts are below.
        // mc: not translated, it uses Minecraft's `Minecraft`.
        // AmbientSoundEngine.playingCount(0): not translated, its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)).
        // AmbientSoundEngine.getManager(0): not translated, its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)).
        // AmbientSoundEngine.tick(1): not translated, its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)).
        // AmbientSoundEngine.stop(1): not translated, its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)).
        // AmbientSoundEngine.play(1): not translated, its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)).
        // AmbientSoundEngine.stopAll(0): not translated, its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)).
    }

    /// `AmbientSoundProperties` (team/creative/ambientsounds/sound/AmbientSoundProperties.java)
    pub mod ambient_sound_properties {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AmbientSoundProperties {
            pub transition: Option<i32>,
            pub pitch: Option<f64>,
            pub fade_volume: Option<f64>,
            pub fade_in_volume: Option<f64>,
            pub fade_out_volume: Option<f64>,
            pub fade_pitch: Option<f64>,
            pub fade_in_pitch: Option<f64>,
            pub fade_out_pitch: Option<f64>,
            pub mute: Option<f64>,
            pub mute_priority: Option<f64>,
            pub random_offset: bool,
            pub pause: Option<ambient_condition_ambient_min_max_condition::AmbientMinMaxCondition>,
            pub length: Option<ambient_condition_ambient_min_max_condition::AmbientMinMaxCondition>,
            pub underwater_pitch: Option<ambient_sound_properties_ambient_min_max_climbing_property::AmbientMinMaxClimbingProperty>,
            pub channel: Option<String>,
        }
        impl AmbientSoundProperties {
            pub fn new() -> Self {
                let mut s = Self { transition: None, pitch: Some(1.0f64), fade_volume: None, fade_in_volume: None, fade_out_volume: None, fade_pitch: None, fade_in_pitch: None, fade_out_pitch: None, mute: None, mute_priority: None, random_offset: true, pause: None, length: None, underwater_pitch: None, channel: None };
                s
            }
            // AmbientSoundProperties.init(1): not translated, it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
            // AmbientSoundProperties.getFadeInVolume(1): not translated, it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
            // AmbientSoundProperties.getFadeOutVolume(1): not translated, it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
            // AmbientSoundProperties.getFadeInPitch(1): not translated, it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
            // AmbientSoundProperties.getFadeOutPitch(1): not translated, it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
            // AmbientSoundProperties.getPitch(1): not translated, it uses AmbientEnvironment, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))).
        }
    }

    /// `AmbientSoundProperties.AmbientMinMaxClimbingProperty` (team/creative/ambientsounds/sound/AmbientSoundProperties.java)
    pub mod ambient_sound_properties_ambient_min_max_climbing_property {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AmbientMinMaxClimbingProperty {
            pub min: f64,
            pub max: f64,
            pub distance_factor: f64,
        }
        impl AmbientMinMaxClimbingProperty {
            pub fn new() -> Self {
                let mut s = Self { min: 0.0f64, max: 0.0f64, distance_factor: 1.0f64 };
                s
            }
            pub fn get_value(&mut self, value: f64) -> f64 {
                if self.max <= self.min {
                    self.max = self.min + 1.0f64;
                }
                let distance: f64 = self.max - self.min;
                return (value / distance) * self.distance_factor;
            }
        }
    }

    /// `OggAudioStreamExtended` (team/creative/ambientsounds/sound/OggAudioStreamExtended.java)
    pub mod ogg_audio_stream_extended {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // OggAudioStreamExtended.setPositionRandomly(2): not translated, it has no body.
    }

    
    // ---- Java regular expressions (java.util.regex), a small backtracking engine ----
    // Bundles can't add the regex crate, so this covers what mods use: literals, `.`, classes with ranges and
    // \d \s \w, groups (capturing, `(?:`, `(?<name>`), lookahead, alternation, greedy/lazy quantifiers, ^ $ \b.
    // Positions are in chars, like the translated String methods.
    
    #[derive(Clone, Debug)]
    enum JClassItem {
        Range(char, char),
        Digit(bool),
        Space(bool),
        Word(bool),
    }
    
    #[derive(Clone, Debug)]
    enum JNode {
        Char(char),
        Any,
        Class(Vec<JClassItem>, bool),
        Start,
        End,
        WordB(bool),
        Group(Box<JNode>, Option<usize>),
        Look(Box<JNode>, bool),
        Alt(Vec<JNode>),
        Concat(Vec<JNode>),
        Repeat(Box<JNode>, u32, u32, bool),
        Backref(usize),
    }
    
    type JCaps = Vec<Option<(usize, usize)>>;
    
    /// A compiled `Pattern`.
    #[derive(Clone, Debug)]
    pub struct JRegex {
        node: JNode,
        groups: usize,
        names: Vec<(String, usize)>,
        ci: bool,
    }
    
    fn j_is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
    
    fn j_is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
    }
    
    struct JRegexParser<'a> {
        p: &'a [char],
        i: usize,
        groups: usize,
        names: Vec<(String, usize)>,
    }
    
    impl JRegexParser<'_> {
        fn peek(&self) -> Option<char> {
            self.p.get(self.i).copied()
        }
        fn eat(&mut self, c: char) -> bool {
            if self.peek() == Some(c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn alt(&mut self) -> JNode {
            let mut arms = vec![self.concat()];
            while self.eat('|') {
                arms.push(self.concat());
            }
            if arms.len() == 1 { arms.pop().unwrap() } else { JNode::Alt(arms) }
        }
        fn concat(&mut self) -> JNode {
            let mut items = Vec::new();
            while let Some(c) = self.peek() {
                if c == '|' || c == ')' {
                    break;
                }
                let atom = self.atom();
                items.push(self.quantified(atom));
            }
            JNode::Concat(items)
        }
        fn number(&mut self) -> Option<u32> {
            let start = self.i;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.i += 1;
            }
            if start == self.i { None } else { self.p[start..self.i].iter().collect::<String>().parse().ok() }
        }
        fn quantified(&mut self, atom: JNode) -> JNode {
            let (min, max) = match self.peek() {
                Some('*') => (0, u32::MAX),
                Some('+') => (1, u32::MAX),
                Some('?') => (0, 1),
                Some('{') => {
                    let save = self.i;
                    self.i += 1;
                    let Some(lo) = self.number() else {
                        self.i = save;
                        return atom;
                    };
                    let hi = if self.eat(',') { self.number().unwrap_or(u32::MAX) } else { lo };
                    if !self.eat('}') {
                        panic!("bad repetition in regex");
                    }
                    self.i -= 1;
                    (lo, hi)
                }
                _ => return atom,
            };
            self.i += 1;
            let greedy = !self.eat('?');
            if greedy {
                self.eat('+');
            }
            self.quantified(JNode::Repeat(Box::new(atom), min, max, greedy))
        }
        fn escape(&mut self, in_class: bool) -> JClassItem {
            let c = self.peek().expect("regex ends with a backslash");
            self.i += 1;
            let lit = |c: char| JClassItem::Range(c, c);
            match c {
                'd' => JClassItem::Digit(true),
                'D' => JClassItem::Digit(false),
                's' => JClassItem::Space(true),
                'S' => JClassItem::Space(false),
                'w' => JClassItem::Word(true),
                'W' => JClassItem::Word(false),
                't' => lit('\t'),
                'n' => lit('\n'),
                'r' => lit('\r'),
                'f' => lit('\u{0C}'),
                'e' => lit('\u{1B}'),
                '0' => lit('\0'),
                'u' => {
                    let hex: String = self.p[self.i..self.i + 4].iter().collect();
                    self.i += 4;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\u escape")).unwrap_or('\u{FFFD}'))
                }
                'x' => {
                    let hex: String = self.p[self.i..self.i + 2].iter().collect();
                    self.i += 2;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\x escape")).unwrap_or('\u{FFFD}'))
                }
                c if !in_class && c.is_ascii_alphabetic() => panic!("unsupported regex escape \\{c}"),
                c => lit(c),
            }
        }
        fn class(&mut self) -> JNode {
            let negated = self.eat('^');
            let mut items = Vec::new();
            let mut first = true;
            loop {
                let c = self.peek().expect("unclosed [ in regex");
                if c == ']' && !first {
                    self.i += 1;
                    break;
                }
                first = false;
                if c == '[' {
                    panic!("nested character classes aren't supported");
                }
                self.i += 1;
                let item = if c == '\\' { self.escape(true) } else { JClassItem::Range(c, c) };
                // a-z
                if let JClassItem::Range(lo, _) = item
                    && self.peek() == Some('-')
                    && self.p.get(self.i + 1).is_some_and(|&n| n != ']')
                {
                    self.i += 1;
                    let hc = self.peek().unwrap();
                    self.i += 1;
                    let hi = if hc == '\\' {
                        match self.escape(true) {
                            JClassItem::Range(h, _) => h,
                            _ => panic!("bad range in regex"),
                        }
                    } else {
                        hc
                    };
                    items.push(JClassItem::Range(lo, hi));
                    continue;
                }
                items.push(item);
            }
            JNode::Class(items, negated)
        }
        fn atom(&mut self) -> JNode {
            let c = self.peek().unwrap();
            self.i += 1;
            match c {
                '.' => JNode::Any,
                '^' => JNode::Start,
                '$' => JNode::End,
                '[' => self.class(),
                '(' => {
                    let mut index = None;
                    let mut look = None;
                    if self.eat('?') {
                        if self.eat(':') {
                        } else if self.eat('=') {
                            look = Some(false);
                        } else if self.eat('!') {
                            look = Some(true);
                        } else if self.eat('<') {
                            let start = self.i;
                            while self.peek().is_some_and(|c| c != '>') {
                                self.i += 1;
                            }
                            let name: String = self.p[start..self.i].iter().collect();
                            self.i += 1;
                            self.groups += 1;
                            self.names.push((name, self.groups));
                            index = Some(self.groups);
                        } else {
                            panic!("unsupported regex group (?{:?}", self.peek());
                        }
                    } else {
                        self.groups += 1;
                        index = Some(self.groups);
                    }
                    let inner = self.alt();
                    if !self.eat(')') {
                        panic!("unclosed ( in regex");
                    }
                    match look {
                        Some(neg) => JNode::Look(Box::new(inner), neg),
                        None => JNode::Group(Box::new(inner), index),
                    }
                }
                '\\' => match self.peek() {
                    Some('b') => {
                        self.i += 1;
                        JNode::WordB(true)
                    }
                    Some('B') => {
                        self.i += 1;
                        JNode::WordB(false)
                    }
                    Some('A') => {
                        self.i += 1;
                        JNode::Start
                    }
                    Some('z') | Some('Z') => {
                        self.i += 1;
                        JNode::End
                    }
                    Some(d) if d.is_ascii_digit() && d != '0' => {
                        self.i += 1;
                        JNode::Backref(d as usize - '0' as usize)
                    }
                    Some('Q') => {
                        self.i += 1;
                        let mut lits = Vec::new();
                        while self.i < self.p.len() && !(self.p[self.i] == '\\' && self.p.get(self.i + 1) == Some(&'E')) {
                            lits.push(JNode::Char(self.p[self.i]));
                            self.i += 1;
                        }
                        self.i = (self.i + 2).min(self.p.len());
                        JNode::Concat(lits)
                    }
                    _ => match self.escape(false) {
                        JClassItem::Range(c, _) => JNode::Char(c),
                        other => JNode::Class(vec![other], false),
                    },
                },
                c => JNode::Char(c),
            }
        }
    }
    
    impl JRegex {
        /// `Pattern.compile(p)`. Java throws PatternSyntaxException on a bad pattern; this panics.
        pub fn new(p: &str) -> Self {
            Self::with_flags(p, 0)
        }
        /// `Pattern.compile(p, flags)`: CASE_INSENSITIVE (2) is honoured.
        pub fn with_flags(p: &str, flags: i32) -> Self {
            let chars: Vec<char> = p.chars().collect();
            let (chars, ci) = match chars.strip_prefix(&['(', '?', 'i', ')']) {
                Some(rest) => (rest.to_vec(), true),
                None => (chars, flags & 2 != 0),
            };
            let mut parser = JRegexParser { p: &chars, i: 0, groups: 0, names: Vec::new() };
            let node = parser.alt();
            if parser.i != chars.len() {
                panic!("unmatched ) in regex {p:?}");
            }
            JRegex { node, groups: parser.groups, names: parser.names, ci }
        }
    
        fn same(&self, a: char, b: char) -> bool {
            a == b || self.ci && (j_lower_char(a) == j_lower_char(b) || j_upper_char(a) == j_upper_char(b))
        }
    
        fn in_class(&self, items: &[JClassItem], c: char) -> bool {
            items.iter().any(|it| match *it {
                JClassItem::Range(lo, hi) => {
                    (lo..=hi).contains(&c) || self.ci && ((lo..=hi).contains(&j_lower_char(c)) || (lo..=hi).contains(&j_upper_char(c)))
                }
                JClassItem::Digit(yes) => c.is_ascii_digit() == yes,
                JClassItem::Space(yes) => j_is_space(c) == yes,
                JClassItem::Word(yes) => j_is_word(c) == yes,
            })
        }
    
        fn m(&self, n: &JNode, s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match n {
                JNode::Char(c) => i < s.len() && self.same(s[i], *c) && k(i + 1, caps),
                JNode::Any => i < s.len() && !matches!(s[i], '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}') && k(i + 1, caps),
                JNode::Class(items, neg) => i < s.len() && self.in_class(items, s[i]) != *neg && k(i + 1, caps),
                JNode::Start => i == 0 && k(i, caps),
                JNode::End => (i == s.len() || i + 1 == s.len() && s[i] == '\n') && k(i, caps),
                JNode::WordB(yes) => {
                    let before = i > 0 && j_is_word(s[i - 1]);
                    let after = i < s.len() && j_is_word(s[i]);
                    (before != after) == *yes && k(i, caps)
                }
                JNode::Group(inner, index) => match *index {
                    None => self.m(inner, s, i, caps, k),
                    Some(g) => self.m(inner, s, i, caps, &mut |j, caps: &mut JCaps| {
                        let prev = caps[g];
                        caps[g] = Some((i, j));
                        if k(j, caps) {
                            return true;
                        }
                        caps[g] = prev;
                        false
                    }),
                },
                JNode::Look(inner, neg) => {
                    let mut probe = caps.clone();
                    let found = self.m(inner, s, i, &mut probe, &mut |_, _| true);
                    if found == *neg {
                        return false;
                    }
                    if !neg {
                        *caps = probe;
                    }
                    k(i, caps)
                }
                JNode::Alt(arms) => arms.iter().any(|a| self.m(a, s, i, caps, k)),
                JNode::Concat(items) => self.seq(items, s, i, caps, k),
                JNode::Repeat(inner, min, max, greedy) => self.rep(inner, *min, *max, *greedy, 0, s, i, caps, k),
                JNode::Backref(g) => match caps.get(*g).copied().flatten() {
                    Some((a, b)) => {
                        let len = b - a;
                        i + len <= s.len() && (0..len).all(|d| self.same(s[a + d], s[i + d])) && k(i + len, caps)
                    }
                    None => false,
                },
            }
        }
    
        fn seq(&self, items: &[JNode], s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match items.split_first() {
                None => k(i, caps),
                Some((first, rest)) => self.m(first, s, i, caps, &mut |j, caps: &mut JCaps| self.seq(rest, s, j, caps, k)),
            }
        }
    
        #[allow(clippy::too_many_arguments)]
        fn rep(&self, n: &JNode, min: u32, max: u32, greedy: bool, count: u32, s: &[char], i: usize, caps: &mut JCaps,
               k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            let more = |caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool| {
                count < max
                    && self.m(n, s, i, caps, &mut |j, caps: &mut JCaps| {
                        // An empty pass can't make progress: stop repeating once the minimum is met.
                        (j != i || count < min) && self.rep(n, min, max, greedy, count + 1, s, j, caps, k)
                    })
            };
            if count < min {
                return more(caps, k);
            }
            if greedy { more(caps, k) || k(i, caps) } else { k(i, caps) || more(caps, k) }
        }
    
        /// A match starting exactly at `i`; `whole`: it must also end at the end of the text.
        fn at(&self, s: &[char], i: usize, whole: bool) -> Option<JCaps> {
            let mut caps: JCaps = vec![None; self.groups + 1];
            let mut end = None;
            let ok = self.m(&self.node, s, i, &mut caps, &mut |j, _| {
                if whole && j != s.len() {
                    return false;
                }
                end = Some(j);
                true
            });
            if !ok {
                return None;
            }
            caps[0] = Some((i, end.unwrap()));
            Some(caps)
        }
    
        fn find_from(&self, s: &[char], from: usize) -> Option<JCaps> {
            (from..=s.len()).find_map(|i| self.at(s, i, false))
        }
    
        /// `String.matches` / `Pattern.matches`: the whole text.
        pub fn matches(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.at(&cs, 0, true).is_some()
        }
    
        /// `Pattern.matcher(s).find()` as a test.
        pub fn is_found(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.find_from(&cs, 0).is_some()
        }
    
        fn expand(&self, s: &[char], caps: &JCaps, rep: &str, out: &mut String) {
            let r: Vec<char> = rep.chars().collect();
            let mut i = 0;
            while i < r.len() {
                match r[i] {
                    '\\' if i + 1 < r.len() => {
                        out.push(r[i + 1]);
                        i += 2;
                    }
                    '$' if i + 1 < r.len() && r[i + 1].is_ascii_digit() => {
                        let g = r[i + 1] as usize - '0' as usize;
                        if let Some((a, b)) = caps.get(g).copied().flatten() {
                            out.extend(&s[a..b]);
                        } else if g > self.groups {
                            panic!("No group {g}");
                        }
                        i += 2;
                    }
                    '$' if r.get(i + 1) == Some(&'{') => {
                        let close = r[i..].iter().position(|&c| c == '}').expect("bad ${name} in replacement") + i;
                        let name: String = r[i + 2..close].iter().collect();
                        let g = self.names.iter().find(|(n, _)| *n == name).map(|(_, g)| *g).expect("no group with that name");
                        if let Some((a, b)) = caps[g] {
                            out.extend(&s[a..b]);
                        }
                        i = close + 1;
                    }
                    c => {
                        out.push(c);
                        i += 1;
                    }
                }
            }
        }
    
        fn replace(&self, s: &str, rep: &str, all: bool) -> String {
            let cs: Vec<char> = s.chars().collect();
            let mut out = String::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                out.extend(&cs[pos..a]);
                self.expand(&cs, &caps, rep, &mut out);
                pos = b;
                from = if b == a { b + 1 } else { b };
                if b == a && a < cs.len() {
                    out.push(cs[a]);
                    pos = a + 1;
                }
                if !all {
                    break;
                }
            }
            if pos <= cs.len() {
                out.extend(&cs[pos..]);
            }
            out
        }
    
        /// `String.replaceAll` / `Matcher.replaceAll`, with Java's `$1` and `\$` in the replacement.
        pub fn replace_all(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, true)
        }
    
        pub fn replace_first(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, false)
        }
    
        /// `String.split(regex, limit)` / `Pattern.split`, with Java's rules for empty parts.
        pub fn split(&self, s: &str, limit: i32) -> Vec<String> {
            let cs: Vec<char> = s.chars().collect();
            let mut parts: Vec<String> = Vec::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                if limit > 0 && parts.len() as i32 == limit - 1 {
                    break;
                }
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                if b == 0 {
                    // A zero-width match at the start makes no leading empty part.
                    from = 1;
                    continue;
                }
                if a == b && a >= cs.len() {
                    break;
                }
                parts.push(cs[pos..a].iter().collect());
                pos = b;
                from = if a == b { b + 1 } else { b };
            }
            parts.push(cs[pos.min(cs.len())..].iter().collect());
            if limit == 0 {
                while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
                    parts.pop();
                }
                if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
                    parts.clear();
                }
            }
            parts
        }
    }
    
    /// A `Matcher`: a regex over one text, with the last match.
    #[derive(Clone, Debug)]
    pub struct JMatcher {
        re: JRegex,
        text: Vec<char>,
        from: usize,
        caps: Option<JCaps>,
    }
    
    impl JMatcher {
        pub fn new(re: &JRegex, s: &str) -> Self {
            JMatcher { re: re.clone(), text: s.chars().collect(), from: 0, caps: None }
        }
        pub fn find(&mut self) -> bool {
            if self.from > self.text.len() {
                self.caps = None;
                return false;
            }
            self.caps = self.re.find_from(&self.text, self.from);
            if let Some((a, b)) = self.caps.as_ref().and_then(|c| c[0]) {
                self.from = if a == b { b + 1 } else { b };
                true
            } else {
                self.from = self.text.len() + 1;
                false
            }
        }
        pub fn find_at(&mut self, start: i32) -> bool {
            self.from = start as usize;
            self.find()
        }
        pub fn matches(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, true);
            self.caps.is_some()
        }
        pub fn looking_at(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, false);
            self.caps.is_some()
        }
        fn span(&self, g: usize) -> Option<(usize, usize)> {
            let caps = self.caps.as_ref().expect("No match found");
            assert!(g < caps.len(), "No group {g}");
            caps[g]
        }
        /// `group(n)`: null (None) when that group took no part in the match.
        pub fn group(&self, g: i32) -> Option<String> {
            self.span(g as usize).map(|(a, b)| self.text[a..b].iter().collect())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let g = self.re.names.iter().find(|(n, _)| n == name).map(|(_, g)| *g).expect("No group with that name");
            self.group(g as i32)
        }
        pub fn start(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(a, _)| a as i32)
        }
        pub fn end(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(_, b)| b as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.re.groups as i32
        }
        pub fn replace_all(&self, rep: &str) -> String {
            self.re.replace_all(&self.text.iter().collect::<String>(), rep)
        }
    }
    
    #[cfg(test)]
    mod jregex_tests {
        use super::*;
    
        #[test]
        fn groups_and_lazy() {
            let re = JRegex::new("^(?:\\S )?(?:VERY SPECIAL|COMMON|RARE|EPIC)(?: DUNGEON)?(?: (.+?))?(?: \\S)?$");
            let mut m = JMatcher::new(&re, "EPIC DUNGEON SWORD");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("SWORD"));
            let mut m = JMatcher::new(&re, "a RARE FISHING ROD b");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("FISHING ROD"));
            let mut m = JMatcher::new(&re, "COMMON");
            assert!(m.matches());
            assert_eq!(m.group(1), None);
        }
    
        #[test]
        fn classes_counts_find() {
            let re = JRegex::new("server/[0-9A-Fa-f]{8}/(.+)");
            let mut m = JMatcher::new(&re, "server/0aF3b2c1/pack.zip");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("pack.zip"));
            assert!(!JRegex::new("server/[0-9A-Fa-f]{8}/(.+)").matches("server/xyz/a"));
            let re = JRegex::new("\"id\"\\s*:\\s*\"([a-z0-9_.-]+:[a-z0-9_./-]+)\"");
            let mut m = JMatcher::new(&re, "{\"id\": \"minecraft:item/a\", \"id\":\"x:y\"}");
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("minecraft:item/a"));
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("x:y"));
            assert!(!m.find());
        }
    
        #[test]
        fn split_and_replace() {
            assert_eq!(JRegex::new("\\s*,\\s*").split("a , b,c,,", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new(",").split("a,b,c", 2), vec!["a", "b,c"]);
            assert_eq!(JRegex::new("").split("abc", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new("(\\w+)@(\\w+)").replace_all("x a@b y c@d", "$2.$1"), "x b.a y d.c");
            assert_eq!(JRegex::new("a*").replace_all("baaac", "-"), "-b--c-");
            assert!(JRegex::with_flags("abc", 2).matches("AbC"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "ambientsounds");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
