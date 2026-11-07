// Dynamic Trees, ported to Horizon by horizon-port from dynamictrees-fabric-1.21.1-1.7.2-BETA.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_dynamictrees). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "dynamictrees";
pub const NAME: &str = "Dynamic Trees";
pub const VERSION: &str = "1.7.2";
pub const DESCRIPTION: &str = "Trees that grow, forests that spread...";

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

    /// `Cell` (com/dtteam/dynamictrees/api/cell/Cell.java)
    pub mod cell {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Cell.getValue(0): not translated, it has no body.
        // Cell.getValueFromSide(1): not translated, it has no body.
    }

    /// `CellKit` (com/dtteam/dynamictrees/api/cell/CellKit.java)
    pub mod cell_kit {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // NULL_CELL_SOLVER: not translated, it uses CellSolver, which isn't translated (it's an interface).
        // NULL_CELL_KIT: not translated, its value couldn't be read: an anonymous class.
        // REGISTRY: not translated, it uses SimpleRegistry, which isn't translated (it's generic).
        // CellKit.CellKit(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // CellKit.getCellForLeaves(1): not translated, it has no body.
        // CellKit.getCellForBranch(2): not translated, it has no body.
        // CellKit.getCellSolver(0): not translated, it has no body.
        // CellKit.getLeafCluster(0): not translated, it has no body.
        // CellKit.getDefaultHydration(0): not translated, it has no body.
        // CellKit.findCellKit(1): not translated, it uses CellKit, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // CellKit.findCellKit(1): not translated, it uses CellKit, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
    }

    /// `CellNull` (com/dtteam/dynamictrees/api/cell/CellNull.java)
    pub mod cell_null {
        use super::*;
        // The class itself isn't translated: it implements `Cell`. Its static parts are below.
        // NULL_CELL: not translated, it uses CellNull, which isn't translated (it implements `Cell`).
        // CellNull.getValue(0): not translated, its class isn't translated (it implements `Cell`).
        // CellNull.getValueFromSide(1): not translated, its class isn't translated (it implements `Cell`).
    }

    /// `CellSolver` (com/dtteam/dynamictrees/api/cell/CellSolver.java)
    pub mod cell_solver {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CellSolver.solve(1): not translated, it has no body.
    }

    /// `Configurable` (com/dtteam/dynamictrees/api/configuration/Configurable.java)
    pub mod configurable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Configurable.isPropertyRegistered(1): not translated, it has no body.
        // Configurable.getRegisteredProperties(0): not translated, it has no body.
        // Configurable.getDefaultConfiguration(0): not translated, it has no body.
    }

    /// `ConfigurableRegistry` (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistry.java)
    pub mod configurable_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfigurableRegistry.ConfigurableRegistry(3): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistry.ConfigurableRegistry(4): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistry.ConfigurableRegistry(4): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistry.ConfigurableRegistry(5): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistry.register(1): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistry.registerTemplate(1): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistry.getTemplates(0): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigurableRegistryEntry` (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java)
    pub mod configurable_registry_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfigurableRegistryEntry.ConfigurableRegistryEntry(0): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistryEntry.ConfigurableRegistryEntry(1): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistryEntry.createDefaultConfiguration(0): not translated, it has no body.
        // ConfigurableRegistryEntry.registerProperties(0): not translated, it has no body.
        // ConfigurableRegistryEntry.register(1): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistryEntry.isPropertyRegistered(1): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistryEntry.getRegisteredProperties(0): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistryEntry.getDefaultConfiguration(0): not translated, its class isn't translated (it's generic).
        // ConfigurableRegistryEntry.with(2): not translated, it's generic.
    }

    /// `Configuration` (com/dtteam/dynamictrees/api/configuration/Configuration.java)
    pub mod configuration {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Configuration.Configuration(1): not translated, its class isn't translated (it's generic).
        // Configuration.with(2): not translated, it's generic.
        // Configuration.withAll(1): not translated, its class isn't translated (it's generic).
        // Configuration.has(1): not translated, its class isn't translated (it's generic).
        // Configuration.get(1): not translated, it's generic.
        // Configuration.getAsOptional(1): not translated, it's generic.
        // Configuration.getOrInvalidDefault(3): not translated, it's generic.
        // Configuration.copy(0): not translated, it has no body.
        // Configuration.getConfigurable(0): not translated, its class isn't translated (it's generic).
        // Configuration.toString(0): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigurationProperty` (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java)
    pub mod configuration_property {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NULL: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.ConfigurationProperty(2): not translated, its class isn't translated (it's generic).
        // ConfigurationProperty.getKey(0): not translated, its class isn't translated (it's generic).
        // ConfigurationProperty.getType(0): not translated, its class isn't translated (it's generic).
        // ConfigurationProperty.deserialise(1): not translated, its class isn't translated (it's generic).
        // ConfigurationProperty.property(2): not translated, it's generic.
        // ConfigurationProperty.string(1): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.bool(1): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.integer(1): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.longProperty(1): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.doubleProperty(1): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.floatProperty(1): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.block(1): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.item(1): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConfigurationProperty.equals(1): not translated, its class isn't translated (it's generic).
        // ConfigurationProperty.hashCode(0): not translated, its class isn't translated (it's generic).
        // ConfigurationProperty.toString(0): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigurationTemplate` (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplate.java)
    pub mod configuration_template {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ConfigurationTemplate.apply(1): not translated, it has no body.
        // ConfigurationTemplate.getRegisteredProperties(0): not translated, it has no body.
    }

    /// `ConfigurationTemplateResourceLoader` (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java)
    pub mod configuration_template_resource_loader {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfigurationTemplateResourceLoader.ConfigurationTemplateResourceLoader(3): not translated, its class isn't translated (it's generic).
        // ConfigurationTemplateResourceLoader.applyOnReload(2): not translated, its class isn't translated (it's generic).
        // ConfigurationTemplateResourceLoader.register(1): not translated, its class isn't translated (it's generic).
        // ConfigurationTemplateResourceLoader.createTemplate(3): not translated, its class isn't translated (it's generic).
        // ConfigurationTemplateResourceLoader.getConfigurable(1): not translated, its class isn't translated (it's generic).
        // ConfigurationTemplateResourceLoader.getConfigurableName(1): not translated, its class isn't translated (it's generic).
        // ConfigurationTemplateResourceLoader.deserialisePropertyDefinitions(1): not translated, its class isn't translated (it's generic).
        // ConfigurationTemplateResourceLoader.assertNameNotReserved(1): not translated, its class isn't translated (it's generic).
    }

    /// `CustomConfigurationProperty` (com/dtteam/dynamictrees/api/configuration/CustomConfigurationProperty.java)
    pub mod custom_configuration_property {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CustomConfigurationProperty.CustomConfigurationProperty(2): not translated, its class isn't translated (it's generic).
    }

    /// `CustomConfigurationTemplate` (com/dtteam/dynamictrees/api/configuration/CustomConfigurationTemplate.java)
    pub mod custom_configuration_template {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CustomConfigurationTemplate.CustomConfigurationTemplate(4): not translated, its class isn't translated (it's generic).
        // CustomConfigurationTemplate.collectAllProperties(2): not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // CustomConfigurationTemplate.apply(1): not translated, its class isn't translated (it's generic).
        // CustomConfigurationTemplate.deserialiseJson(1): not translated, its class isn't translated (it's generic).
        // CustomConfigurationTemplate.processJson(1): not translated, its class isn't translated (it's generic).
        // CustomConfigurationTemplate.getRegisteredProperties(0): not translated, its class isn't translated (it's generic).
    }

    /// `DefaultConfigurationTemplate` (com/dtteam/dynamictrees/api/configuration/DefaultConfigurationTemplate.java)
    pub mod default_configuration_template {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // DefaultConfigurationTemplate.DefaultConfigurationTemplate(2): not translated, its class isn't translated (it's generic).
        // DefaultConfigurationTemplate.apply(1): not translated, its class isn't translated (it's generic).
        // DefaultConfigurationTemplate.getRegisteredProperties(0): not translated, its class isn't translated (it's generic).
    }

    /// `Properties` (com/dtteam/dynamictrees/api/configuration/Properties.java)
    pub mod properties {
        use super::*;
        // The class itself isn't translated: it implements `PropertiesAccessor`. Its static parts are below.
        pub static NONE: LazyLock<JProps> = LazyLock::new(|| JProps::new());
        // Properties.put(2): not translated, it's generic.
        // Properties.putAll(1): not translated, its class isn't translated (it implements `PropertiesAccessor`).
        // Properties.get(1): not translated, it's generic.
        // Properties.has(1): not translated, its class isn't translated (it implements `PropertiesAccessor`).
        // Properties.forEach(1): not translated, its class isn't translated (it implements `PropertiesAccessor`).
        // Properties.applyIteration(3): not translated, it's generic.
        // Properties.toString(0): not translated, its class isn't translated (it implements `PropertiesAccessor`).
    }

    /// `PropertiesAccessor` (com/dtteam/dynamictrees/api/configuration/PropertiesAccessor.java)
    pub mod properties_accessor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PropertiesAccessor.get(1): not translated, it has no body.
        // PropertiesAccessor.has(1): not translated, it has no body.
        // PropertiesAccessor.forEach(1): not translated, it has no body.
    }

    /// `PropertiesAccessor.IterationAction` (com/dtteam/dynamictrees/api/configuration/PropertiesAccessor.java)
    pub mod properties_accessor_iteration_action {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PropertiesAccessor.IterationAction.apply(2): not translated, it has no body.
    }

    /// `PropertyDefinition` (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java)
    pub mod property_definition {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const PREFIX: &str = "#";
        // NULL: not translated, it uses PropertyDefinition, which isn't translated (it's generic).
        // PropertyDefinition.PropertyDefinition(3): not translated, its class isn't translated (it's generic).
        // PropertyDefinition.PropertyDefinition(4): not translated, its class isn't translated (it's generic).
        // PropertyDefinition.getProperty(0): not translated, its class isn't translated (it's generic).
        // PropertyDefinition.process(2): not translated, its class isn't translated (it's generic).
        // PropertyDefinition.getOrDefault(1): not translated, its class isn't translated (it's generic).
        // PropertyDefinition.captureClass(0): not translated, it uses `Class`.
    }

    /// `TemplateRegistry` (com/dtteam/dynamictrees/api/configuration/TemplateRegistry.java)
    pub mod template_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TemplateRegistry.register(2): not translated, its class isn't translated (it's generic).
        // TemplateRegistry.get(1): not translated, its class isn't translated (it's generic).
    }

    /// `DynamicTreesAddonEntrypoint` (com/dtteam/dynamictrees/api/DynamicTreesAddonEntrypoint.java)
    pub mod dynamic_trees_addon_entrypoint {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // DynamicTreesAddonEntrypoint.onDynamicTreesPreSetup(0): not translated, it has no body.
        // DynamicTreesAddonEntrypoint.onAddResourceLoaders(1): not translated, its class isn't translated (it's an interface).
        // DynamicTreesAddonEntrypoint.onRegisterStagedApplier(3): not translated, it's generic.
    }

    /// `IgnoreThrowable` (com/dtteam/dynamictrees/api/function/IgnoreThrowable.java)
    pub mod ignore_throwable {
        use super::*;
        // The class itself isn't translated: it extends `Throwable`. Its static parts are below.
        // INSTANCE: not translated, it uses IgnoreThrowable, which isn't translated (it extends `Throwable`).
        // IgnoreThrowable.IgnoreThrowable(0): not translated, its class isn't translated (it extends `Throwable`).
    }

    /// `TetraFunction` (com/dtteam/dynamictrees/api/function/TetraFunction.java)
    pub mod tetra_function {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TetraFunction.apply(4): not translated, it has no body.
    }

    /// `ThrowableBiFunction` (com/dtteam/dynamictrees/api/function/ThrowableBiFunction.java)
    pub mod throwable_bi_function {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ThrowableBiFunction.apply(2): not translated, it has no body.
    }

    /// `ThrowableFunction` (com/dtteam/dynamictrees/api/function/ThrowableFunction.java)
    pub mod throwable_function {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ThrowableFunction.apply(1): not translated, it has no body.
    }

    /// `ThrowableRunnable` (com/dtteam/dynamictrees/api/function/ThrowableRunnable.java)
    pub mod throwable_runnable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ThrowableRunnable.run(0): not translated, it has no body.
    }

    /// `TriPredicate` (com/dtteam/dynamictrees/api/function/TriPredicate.java)
    pub mod tri_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TriPredicate.test(3): not translated, its class isn't translated (it's an interface).
    }

    /// `LazyValue` (com/dtteam/dynamictrees/api/lazyvalue/LazyValue.java)
    pub mod lazy_value {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LazyValue.get(0): not translated, it has no body.
        // LazyValue.of(1): not translated, it's generic.
        // LazyValue.supplied(1): not translated, it's generic.
    }

    /// `MutableLazyValue` (com/dtteam/dynamictrees/api/lazyvalue/MutableLazyValue.java)
    pub mod mutable_lazy_value {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MutableLazyValue.get(0): not translated, it has no body.
        // MutableLazyValue.reset(1): not translated, it has no body.
        // MutableLazyValue.set(1): not translated, it has no body.
        // MutableLazyValue.supplied(1): not translated, it's generic.
    }

    /// `MutableSuppliedLazyValue` (com/dtteam/dynamictrees/api/lazyvalue/MutableSuppliedLazyValue.java)
    pub mod mutable_supplied_lazy_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MutableSuppliedLazyValue.MutableSuppliedLazyValue(1): not translated, its class isn't translated (it's generic).
        // MutableSuppliedLazyValue.reset(1): not translated, its class isn't translated (it's generic).
        // MutableSuppliedLazyValue.set(1): not translated, its class isn't translated (it's generic).
    }

    /// `SuppliedLazyValue` (com/dtteam/dynamictrees/api/lazyvalue/SuppliedLazyValue.java)
    pub mod supplied_lazy_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SuppliedLazyValue.SuppliedLazyValue(1): not translated, its class isn't translated (it's generic).
        // SuppliedLazyValue.get(0): not translated, its class isn't translated (it's generic).
    }

    /// `BranchConnectionData` (com/dtteam/dynamictrees/api/network/BranchConnectionData.java)
    pub mod branch_connection_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2680`. Its static parts are below.
        // BranchConnectionData.BranchConnectionData(2): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
        // BranchConnectionData.getBlockState(0): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
        // BranchConnectionData.getConnections(0): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
    }

    /// `BranchDestructionData` (com/dtteam/dynamictrees/api/network/BranchDestructionData.java)
    pub mod branch_destruction_data {
        use super::*;
        // The class itself isn't translated: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)). Its static parts are below.
        // bounds: not translated, it uses BlockPosBounds, which isn't translated (it implements `Iterable`).
        // BranchDestructionData.BranchDestructionData(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.BranchDestructionData(12): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.merge(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.BranchDestructionData(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.writeToNBT(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.convertBranchesToIntArrays(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.encodeBranchesRadiusPos(3): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.encodeBranchesConnections(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.encodeBranchBlocks(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getNumBranches(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getBranchRelPos(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getRelativeCutPos(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getBranchRadius(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.decodeBranchRadius(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getBranchBlockState(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getConnections(2): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.convertLeavesToIntArray(2): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getNumLeaves(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getLeavesRelPos(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.decodeLeavesRelPos(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getLeavesHydro(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.decodeLeavesHydro(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getLeavesProperties(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getLeavesBlockState(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getAllLeavesWithPos(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.convertEndPointsToIntArray(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getNumEndpoints(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getEndPointRelPos(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getPositions(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BranchDestructionData.getPositions(2): not translated, its body couldn't be read: an anonymous class.
        // BranchDestructionData.encodeRelBlockPos(1): not translated, it uses Minecraft's `class_2338`.
        // BranchDestructionData.decodeRelPos(1): not translated, it uses Minecraft's `class_2338`.
    }

    /// `BranchDestructionData.PosType` (com/dtteam/dynamictrees/api/network/BranchDestructionData.java)
    pub mod branch_destruction_data_pos_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum PosType {
            Branches,
            Leaves,
            Endpoints,
        }
        impl PosType {
            pub const VALUES: [PosType; 3] = [PosType::Branches, PosType::Leaves, PosType::Endpoints, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    PosType::Branches => "BRANCHES",
                    PosType::Leaves => "LEAVES",
                    PosType::Endpoints => "ENDPOINTS",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "BRANCHES" => PosType::Branches,
                    "LEAVES" => PosType::Leaves,
                    "ENDPOINTS" => PosType::Endpoints,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `Connections` (com/dtteam/dynamictrees/api/network/Connections.java)
    pub mod connections {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Connections {
            pub radii: Vec<i32>,
        }
        impl Connections {
            pub fn new() -> Self {
                let mut s = Self { radii: Vec::new() };
                s.radii = vec![0, 0, 0, 0, 0, 0];
                s
            }
            pub fn new_1(radii: &[i32]) -> Self {
                let mut s = Self { radii: Vec::new() };
                s.radii = radii.to_vec();
                s
            }
            // Connections.setRadius(2): not translated, it uses Minecraft's `class_2350`.
            pub fn get_all_radii(&self) -> Vec<i32> {
                return self.radii.to_vec();
            }
            // Connections.setAllRadii(1): not translated, `this` used as a value.
        }
    }

    /// `MapSignal` (com/dtteam/dynamictrees/api/network/MapSignal.java)
    pub mod map_signal {
        use super::*;
        // The class itself isn't translated: it uses NodeInspector, which isn't translated (it's an interface). Its static parts are below.
        // MapSignal.MapSignal(0): not translated, its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)).
        // MapSignal.MapSignal(1): not translated, its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)).
        // MapSignal.run(4): not translated, its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)).
        // MapSignal.returnRun(4): not translated, its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)).
        // MapSignal.getInspectors(0): not translated, its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)).
        // MapSignal.doTrackingVisited(1): not translated, its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)).
    }

    /// `NodeInspector` (com/dtteam/dynamictrees/api/network/NodeInspector.java)
    pub mod node_inspector {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // NodeInspector.run(4): not translated, it has no body.
        // NodeInspector.returnRun(4): not translated, it has no body.
    }

    /// `RootConnections` (com/dtteam/dynamictrees/api/network/RootConnections.java)
    pub mod root_connections {
        use super::*;
        // The class itself isn't translated: it extends `Connections`. Its static parts are below.
        // PLACEHOLDER_CONNECTION_LEVELS: not translated, it uses ConnectionLevel, which isn't translated (its constants carry values).
        // RootConnections.RootConnections(0): not translated, its class isn't translated (it extends `Connections`).
        // RootConnections.RootConnections(1): not translated, its class isn't translated (it extends `Connections`).
        // RootConnections.getConnectionLevels(0): not translated, its class isn't translated (it extends `Connections`).
        // RootConnections.setRadius(2): not translated, its class isn't translated (it extends `Connections`).
        // RootConnections.setConnectionLevel(2): not translated, its class isn't translated (it extends `Connections`).
        // RootConnections.setConnectionLevels(1): not translated, its class isn't translated (it extends `Connections`).
    }

    /// `RootConnections.ConnectionLevel` (com/dtteam/dynamictrees/api/network/RootConnections.java)
    pub mod root_connections_connection_level {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // RootConnections.ConnectionLevel.ConnectionLevel(1): not translated, its class isn't translated (its constants carry values).
        // RootConnections.ConnectionLevel.method_15434(0): not translated, its class isn't translated (its constants carry values).
        // RootConnections.ConnectionLevel.getYOffset(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `AbstractRegistry` (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java)
    pub mod abstract_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // REGISTRY_DUMP: not translated, it uses `Marker` (org.apache.logging.log4j).
        // STRING_COMPARATOR: not translated, it uses `Comparator` (java.util).
        // AbstractRegistry.AbstractRegistry(4): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.assertValid(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getErrorMessage(3): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.registerAll(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getOptional(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.get(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.get(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getAsDataResult(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.has(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getOptional(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getRegistryNames(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getType(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getName(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.isLocked(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.lock(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.unlock(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.runOnNextLock(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.clear(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.clearAll(0): not translated, it has no body.
        // AbstractRegistry.getGetterCodec(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.generateIfValidRunnable(3): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.postRegistryEvent(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getComparator(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.dump(0): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.getAllFor(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.dataGenerationStream(1): not translated, its class isn't translated (it's generic).
        // AbstractRegistry.iterator(0): not translated, its class isn't translated (it's generic).
    }

    /// `ConcurrentRegistry` (com/dtteam/dynamictrees/api/registry/ConcurrentRegistry.java)
    pub mod concurrent_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConcurrentRegistry.ConcurrentRegistry(2): not translated, its class isn't translated (it's generic).
        // ConcurrentRegistry.ConcurrentRegistry(3): not translated, its class isn't translated (it's generic).
        // ConcurrentRegistry.ConcurrentRegistry(3): not translated, its class isn't translated (it's generic).
        // ConcurrentRegistry.ConcurrentRegistry(4): not translated, its class isn't translated (it's generic).
        // ConcurrentRegistry.register(1): not translated, its class isn't translated (it's generic).
        // ConcurrentRegistry.getAll(0): not translated, its class isn't translated (it's generic).
        // ConcurrentRegistry.clearAll(0): not translated, its class isn't translated (it's generic).
    }

    /// `Registries` (com/dtteam/dynamictrees/api/registry/Registries.java)
    pub mod registries {
        use super::*;
        // REGISTRIES: not translated, it uses Registry, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct Registries {
        }
        impl Registries {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `Registry` (com/dtteam/dynamictrees/api/registry/Registry.java)
    pub mod registry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Registry.register(1): not translated, it has no body.
        // Registry.registerAll(1): not translated, it has no body.
        // Registry.has(1): not translated, it has no body.
        // Registry.getOptional(1): not translated, it has no body.
        // Registry.getOptional(1): not translated, it has no body.
        // Registry.get(1): not translated, it has no body.
        // Registry.get(1): not translated, it has no body.
        // Registry.getAsDataResult(1): not translated, it has no body.
        // Registry.getAll(0): not translated, it has no body.
        // Registry.getRegistryNames(0): not translated, it has no body.
        // Registry.getType(0): not translated, it has no body.
        // Registry.getName(0): not translated, it has no body.
        // Registry.isLocked(0): not translated, it has no body.
        // Registry.lock(0): not translated, it has no body.
        // Registry.unlock(0): not translated, it has no body.
        // Registry.runOnNextLock(1): not translated, it has no body.
        // Registry.clear(0): not translated, it has no body.
        // Registry.getGetterCodec(0): not translated, it has no body.
        // Registry.generateIfValidRunnable(3): not translated, it has no body.
        // Registry.postRegistryEvent(0): not translated, it has no body.
        // Registry.getComparator(0): not translated, it has no body.
        // Registry.dump(0): not translated, it has no body.
        // Registry.getAllFor(1): not translated, it has no body.
        // Registry.dataGenerationStream(1): not translated, it has no body.
        // Registry.iterator(0): not translated, it has no body.
    }

    /// `RegistryEntry` (com/dtteam/dynamictrees/api/registry/RegistryEntry.java)
    pub mod registry_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegistryEntry.RegistryEntry(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.RegistryEntry(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.nullEntry(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.isValid(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.ifValid(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.ifValidElse(2): not translated, its class isn't translated (it's generic).
        // RegistryEntry.elseIfInvalid(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.ifInvalid(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.shouldGenerateData(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.setGenerateData(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.generateStateData(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.generateItemModelData(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.generateLangData(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.getRegistryName(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.getTextComponent(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.formatComponent(2): not translated, its class isn't translated (it's generic).
        // RegistryEntry.setRegistryName(1): not translated, its class isn't translated (it's generic).
        // RegistryEntry.toString(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.toLoadDataString(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.toReloadDataString(0): not translated, its class isn't translated (it's generic).
        // RegistryEntry.getString(1): not translated, its class isn't translated (it's generic).
    }

    /// `RegistryHandler` (com/dtteam/dynamictrees/api/registry/RegistryHandler.java)
    pub mod registry_handler {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry`. Its static parts are below.
        // REGISTRY: not translated, it uses ConcurrentRegistry, which isn't translated (it's generic).
        // RegistryHandler.RegistryHandler(1): not translated, its class isn't translated (it extends `RegistryEntry`).
        // RegistryHandler.RegistryHandler(0): not translated, its class isn't translated (it extends `RegistryEntry`).
        // RegistryHandler.get(1): not translated, it uses RegistryHandler, which isn't translated (it extends `RegistryEntry`).
        // RegistryHandler.getOrCorrected(1): not translated, it uses RegistryHandler, which isn't translated (it extends `RegistryEntry`).
        // RegistryHandler.correctRegistryName(1): not translated, it uses Minecraft's `class_2960`.
        // RegistryHandler.addBlock(2): not translated, it's generic.
        // RegistryHandler.addItem(2): not translated, it's generic.
        // RegistryHandler.getBlock(1): not translated, it has no body.
        // RegistryHandler.getItem(1): not translated, it has no body.
        // RegistryHandler.putBlock(2): not translated, it has no body.
        // RegistryHandler.putItem(2): not translated, it has no body.
    }

    /// `SimpleRegistry` (com/dtteam/dynamictrees/api/registry/SimpleRegistry.java)
    pub mod simple_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SimpleRegistry.SimpleRegistry(2): not translated, its class isn't translated (it's generic).
        // SimpleRegistry.SimpleRegistry(3): not translated, its class isn't translated (it's generic).
        // SimpleRegistry.SimpleRegistry(3): not translated, its class isn't translated (it's generic).
        // SimpleRegistry.SimpleRegistry(4): not translated, its class isn't translated (it's generic).
        // SimpleRegistry.register(1): not translated, its class isn't translated (it's generic).
        // SimpleRegistry.getAll(0): not translated, its class isn't translated (it's generic).
        // SimpleRegistry.clearAll(0): not translated, its class isn't translated (it's generic).
    }

    /// `TypedRegistry` (com/dtteam/dynamictrees/api/registry/TypedRegistry.java)
    pub mod typed_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RESOURCE_LOCATION: not translated, it uses Minecraft's `class_2960`.
        // TypedRegistry.TypedRegistry(3): not translated, its class isn't translated (it's generic).
        // TypedRegistry.TypedRegistry(4): not translated, its class isn't translated (it's generic).
        // TypedRegistry.registerType(2): not translated, its class isn't translated (it's generic).
        // TypedRegistry.hasType(1): not translated, its class isn't translated (it's generic).
        // TypedRegistry.getType(1): not translated, its class isn't translated (it's generic).
        // TypedRegistry.getType(2): not translated, its class isn't translated (it's generic).
        // TypedRegistry.getDefaultType(0): not translated, its class isn't translated (it's generic).
        // TypedRegistry.postRegistryEvent(0): not translated, its class isn't translated (it's generic).
        // TypedRegistry.putJsonRegistryName(2): not translated, it uses Minecraft's `class_2960`.
        // TypedRegistry.createDefaultCodec(1): not translated, it's generic.
        // TypedRegistry.newType(1): not translated, it's generic.
        // TypedRegistry.newType(1): not translated, it's generic.
    }

    /// `TypedRegistry.EntryType` (com/dtteam/dynamictrees/api/registry/TypedRegistry.java)
    pub mod typed_registry_entry_type {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TypedRegistry.EntryType.EntryType(1): not translated, its class isn't translated (it's generic).
        // TypedRegistry.EntryType.getCodec(0): not translated, its class isn't translated (it's generic).
        // TypedRegistry.EntryType.setRegistry(1): not translated, its class isn't translated (it's generic).
        // TypedRegistry.EntryType.decode(1): not translated, its class isn't translated (it's generic).
    }

    /// `DTResource` (com/dtteam/dynamictrees/api/resource/DTResource.java)
    pub mod dt_resource {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // DTResource.DTResource(2): not translated, its class isn't translated (it's generic).
        // DTResource.map(1): not translated, it's generic.
        // DTResource.location(0): not translated, its class isn't translated (it's generic).
        // DTResource.resource(0): not translated, its class isn't translated (it's generic).
    }

    /// `AbstractResourceLoader` (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java)
    pub mod abstract_resource_loader {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractResourceLoader.AbstractResourceLoader(1): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.gatherData(1): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.load(1): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.setup(1): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.prepareReload(1): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.reload(2): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.applyOnLoad(2): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.applyOnGatherData(2): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.applyOnSetup(2): not translated, its class isn't translated (it's generic).
        // AbstractResourceLoader.applyOnReload(2): not translated, its class isn't translated (it's generic).
    }

    /// `ApplicationException` (com/dtteam/dynamictrees/api/resource/loading/ApplicationException.java)
    pub mod application_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        // ApplicationException.ApplicationException(1): not translated, its class isn't translated (it extends `Exception`).
    }

    /// `ApplierResourceLoader` (com/dtteam/dynamictrees/api/resource/loading/ApplierResourceLoader.java)
    pub mod applier_resource_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ApplierResourceLoader.registerAppliers(0): not translated, it has no body.
    }

    /// `AbstractResourcePreparer` (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java)
    pub mod abstract_resource_preparer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // AbstractResourcePreparer.AbstractResourcePreparer(3): not translated, its class isn't translated (it's generic).
        // AbstractResourcePreparer.prepare(1): not translated, its class isn't translated (it's generic).
        // AbstractResourcePreparer.collectResources(1): not translated, its class isn't translated (it's generic).
        // AbstractResourcePreparer.readAndPutResources(2): not translated, its class isn't translated (it's generic).
        // AbstractResourcePreparer.tryReadAndPutResource(3): not translated, its class isn't translated (it's generic).
        // AbstractResourcePreparer.readAndPutResource(2): not translated, it has no body.
        // AbstractResourcePreparer.logError(2): not translated, its class isn't translated (it's generic).
        // AbstractResourcePreparer.getResourceName(1): not translated, its class isn't translated (it's generic).
    }

    /// `JsonRegistryResourceLoader` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java)
    pub mod json_registry_resource_loader {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const FRUITS: &str = "fruits";
        pub const PODS: &str = "pods";
        pub const SPECIES: &str = "species";
        pub const FAMILY: &str = "family";
        pub const LEAVES_PROPERTIES: &str = "leaves_properties";
        pub const SOIL_PROPERTIES: &str = "soil_properties";
        // JsonRegistryResourceLoader.JsonRegistryResourceLoader(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.JsonRegistryResourceLoader(3): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applyOnLoad(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.loadResourceOnLoad(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.postLoadOnLoad(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applyLoadAppliers(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applyOnGatherData(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.postLoadOnGatherData(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applyGatherDataAppliers(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applyOnSetup(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.loadResourceOnSetup(1): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applySetupAppliers(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applyOnReload(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.loadResourceOnReload(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.postLoadOnReload(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applyReloadAppliers(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.prepareJson(1): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.loadResource(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.loadNewResource(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.postLoad(1): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.logException(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.applyCommonAppliers(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.logError(2): not translated, its class isn't translated (it's generic).
        // JsonRegistryResourceLoader.logWarning(2): not translated, its class isn't translated (it's generic).
    }

    /// `JsonRegistryResourceLoader.LoadData` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java)
    pub mod json_registry_resource_loader_load_data {
        use super::*;
        // The class itself isn't translated: it uses `R`. Its static parts are below.
        // JsonRegistryResourceLoader.LoadData.LoadData(0): not translated, its class isn't translated (it uses `R`).
        // JsonRegistryResourceLoader.LoadData.LoadData(1): not translated, its class isn't translated (it uses `R`).
        // JsonRegistryResourceLoader.LoadData.getResourceName(0): not translated, its class isn't translated (it uses `R`).
        // JsonRegistryResourceLoader.LoadData.getResource(0): not translated, its class isn't translated (it uses `R`).
        // JsonRegistryResourceLoader.LoadData.wasAlreadyRegistered(0): not translated, its class isn't translated (it uses `R`).
    }

    /// `JsonResourcePreparer` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonResourcePreparer.java)
    pub mod json_resource_preparer {
        use super::*;
        // The class itself isn't translated: it extends `AbstractResourcePreparer`. Its static parts are below.
        pub const JSON_EXTENSION: &str = ".json";
        // JsonResourcePreparer.JsonResourcePreparer(1): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // JsonResourcePreparer.JsonResourcePreparer(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // JsonResourcePreparer.readAndPutResource(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // JsonResourcePreparer.readResource(1): not translated, it uses Minecraft's `class_3298`.
        // JsonResourcePreparer.parseJson(1): not translated, a catch block that uses its exception.
    }

    /// `MultiJsonResourcePreparer` (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java)
    pub mod multi_json_resource_preparer {
        use super::*;
        // The class itself isn't translated: it extends `AbstractResourcePreparer`. Its static parts are below.
        pub const JSON_EXTENSION: &str = ".json";
        // MultiJsonResourcePreparer.MultiJsonResourcePreparer(1): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // MultiJsonResourcePreparer.MultiJsonResourcePreparer(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // MultiJsonResourcePreparer.readAndPutResources(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // MultiJsonResourcePreparer.tryReadAndPutResource(3): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // MultiJsonResourcePreparer.readAndPutResource(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // MultiJsonResourcePreparer.readAndPutResource(3): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // MultiJsonResourcePreparer.computeResourceListIfAbsent(1): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // MultiJsonResourcePreparer.collectResources(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
    }

    /// `PreparationException` (com/dtteam/dynamictrees/api/resource/loading/preparation/PreparationException.java)
    pub mod preparation_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        // PreparationException.PreparationException(1): not translated, its class isn't translated (it extends `Exception`).
        // PreparationException.PreparationException(1): not translated, its class isn't translated (it extends `Exception`).
    }

    /// `ResourcePreparer` (com/dtteam/dynamictrees/api/resource/loading/preparation/ResourcePreparer.java)
    pub mod resource_preparer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ResourcePreparer.prepare(1): not translated, it has no body.
    }

    /// `TextResourcePreparer` (com/dtteam/dynamictrees/api/resource/loading/preparation/TextResourcePreparer.java)
    pub mod text_resource_preparer {
        use super::*;
        // The class itself isn't translated: it extends `AbstractResourcePreparer`. Its static parts are below.
        pub const EXTENSION: &str = ".txt";
        // TextResourcePreparer.TextResourcePreparer(1): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // TextResourcePreparer.TextResourcePreparer(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // TextResourcePreparer.readAndPutResource(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // TextResourcePreparer.readResource(1): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // TextResourcePreparer.readLines(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
        // TextResourcePreparer.offerLine(2): not translated, its class isn't translated (it extends `AbstractResourcePreparer`).
    }

    /// `ResourceLoader` (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java)
    pub mod resource_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ResourceLoader.load(1): not translated, it has no body.
        // ResourceLoader.gatherData(1): not translated, it has no body.
        // ResourceLoader.setup(1): not translated, it has no body.
        // ResourceLoader.prepareReload(1): not translated, it has no body.
        // ResourceLoader.reload(2): not translated, it has no body.
        // ResourceLoader.applyOnLoad(2): not translated, it has no body.
        // ResourceLoader.applyOnGatherData(2): not translated, it has no body.
        // ResourceLoader.applyOnSetup(2): not translated, it has no body.
        // ResourceLoader.applyOnReload(2): not translated, it has no body.
    }

    /// `StagedApplierResourceLoader` (com/dtteam/dynamictrees/api/resource/loading/StagedApplierResourceLoader.java)
    pub mod staged_applier_resource_loader {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // StagedApplierResourceLoader.StagedApplierResourceLoader(4): not translated, its class isn't translated (it's generic).
        // StagedApplierResourceLoader.registerAppliers(0): not translated, its class isn't translated (it's generic).
    }

    /// `StagedApplierResourceLoader.ApplierStage` (com/dtteam/dynamictrees/api/resource/loading/StagedApplierResourceLoader.java)
    pub mod staged_applier_resource_loader_applier_stage {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ApplierStage {
            Load,
            GatherData,
            Setup,
            Reload,
            Common,
        }
        impl ApplierStage {
            pub const VALUES: [ApplierStage; 5] = [ApplierStage::Load, ApplierStage::GatherData, ApplierStage::Setup, ApplierStage::Reload, ApplierStage::Common, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ApplierStage::Load => "LOAD",
                    ApplierStage::GatherData => "GATHER_DATA",
                    ApplierStage::Setup => "SETUP",
                    ApplierStage::Reload => "RELOAD",
                    ApplierStage::Common => "COMMON",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "LOAD" => ApplierStage::Load,
                    "GATHER_DATA" => ApplierStage::GatherData,
                    "SETUP" => ApplierStage::Setup,
                    "RELOAD" => ApplierStage::Reload,
                    "COMMON" => ApplierStage::Common,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ResourceAccessor` (com/dtteam/dynamictrees/api/resource/ResourceAccessor.java)
    pub mod resource_accessor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ResourceAccessor.getResource(1): not translated, it has no body.
        // ResourceAccessor.getAllResources(0): not translated, it has no body.
        // ResourceAccessor.getAllResources(1): not translated, it has no body.
        // ResourceAccessor.forEach(1): not translated, it has no body.
        // ResourceAccessor.filtered(1): not translated, it has no body.
        // ResourceAccessor.map(1): not translated, it has no body.
    }

    /// `ResourceCollector` (com/dtteam/dynamictrees/api/resource/ResourceCollector.java)
    pub mod resource_collector {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ResourceCollector.put(1): not translated, it has no body.
        // ResourceCollector.computeIfAbsent(2): not translated, it has no body.
        // ResourceCollector.createAccessor(0): not translated, it has no body.
        // ResourceCollector.clear(0): not translated, it has no body.
        // ResourceCollector.unordered(0): not translated, it's generic.
        // ResourceCollector.ordered(0): not translated, it's generic.
    }

    /// `SimpleResourceAccessor` (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java)
    pub mod simple_resource_accessor {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SimpleResourceAccessor.SimpleResourceAccessor(2): not translated, its class isn't translated (it's generic).
        // SimpleResourceAccessor.getResource(1): not translated, its class isn't translated (it's generic).
        // SimpleResourceAccessor.getAllResources(0): not translated, its class isn't translated (it's generic).
        // SimpleResourceAccessor.getAllResources(1): not translated, its class isn't translated (it's generic).
        // SimpleResourceAccessor.forEach(1): not translated, its class isn't translated (it's generic).
        // SimpleResourceAccessor.filtered(1): not translated, its class isn't translated (it's generic).
        // SimpleResourceAccessor.map(1): not translated, it's generic.
        // SimpleResourceAccessor.cloneResources(0): not translated, its class isn't translated (it's generic).
        // SimpleResourceAccessor.freshMap(0): not translated, it's generic.
        // SimpleResourceAccessor.filterMap(2): not translated, its class isn't translated (it's generic).
    }

    /// `SimpleResourceCollector` (com/dtteam/dynamictrees/api/resource/SimpleResourceCollector.java)
    pub mod simple_resource_collector {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SimpleResourceCollector.SimpleResourceCollector(1): not translated, its class isn't translated (it's generic).
        // SimpleResourceCollector.put(1): not translated, its class isn't translated (it's generic).
        // SimpleResourceCollector.computeIfAbsent(2): not translated, its class isn't translated (it's generic).
        // SimpleResourceCollector.createAccessor(0): not translated, its class isn't translated (it's generic).
        // SimpleResourceCollector.clear(0): not translated, its class isn't translated (it's generic).
        // SimpleResourceCollector.newMap(0): not translated, it's generic.
    }

    /// `TreeResourceManager` (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java)
    pub mod tree_resource_manager {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TreeResourceManager.addLoader(1): not translated, it has no body.
        // TreeResourceManager.addLoaders(1): not translated, it has no body.
        // TreeResourceManager.addLoaderBefore(2): not translated, it has no body.
        // TreeResourceManager.addLoaderAfter(2): not translated, it has no body.
        // TreeResourceManager.registerAppliers(0): not translated, it has no body.
        // TreeResourceManager.load(0): not translated, it has no body.
        // TreeResourceManager.gatherData(0): not translated, it has no body.
        // TreeResourceManager.setup(0): not translated, it has no body.
        // TreeResourceManager.prepareReload(2): not translated, it has no body.
        // TreeResourceManager.reload(1): not translated, it has no body.
        // TreeResourceManager.addPack(1): not translated, it has no body.
        // TreeResourceManager.method_29213(0): not translated, it has no body.
    }

    /// `TreeResourcePack` (com/dtteam/dynamictrees/api/resource/TreeResourcePack.java)
    pub mod tree_resource_pack {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TreeResourcePack.getResource(1): not translated, its class isn't translated (it's an interface).
        // TreeResourcePack.listResources(3): not translated, its class isn't translated (it's an interface).
        // TreeResourcePack.hasResource(1): not translated, its class isn't translated (it's an interface).
        // TreeResourcePack.getNamespaces(0): not translated, its class isn't translated (it's an interface).
    }

    /// `ClimateZoneType` (com/dtteam/dynamictrees/api/season/ClimateZoneType.java)
    pub mod climate_zone_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ClimateZoneType.ClimateZoneType(2): not translated, its class isn't translated (its constants carry values).
    }

    /// `SeasonGrowthCalculator` (com/dtteam/dynamictrees/api/season/SeasonGrowthCalculator.java)
    pub mod season_growth_calculator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SeasonGrowthCalculator.calcGrowthRate(2): not translated, it has no body.
        // SeasonGrowthCalculator.calcSeedDropRate(2): not translated, it has no body.
        // SeasonGrowthCalculator.calcFruitProductionRate(2): not translated, it has no body.
        // SeasonGrowthCalculator.getPeakFruitProductionSeasonValue(1): not translated, it has no body.
    }

    /// `SeasonManager` (com/dtteam/dynamictrees/api/season/SeasonManager.java)
    pub mod season_manager {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SeasonManager.updateTick(2): not translated, it has no body.
        // SeasonManager.flushMappings(0): not translated, it has no body.
        // SeasonManager.getGrowthFactor(3): not translated, it has no body.
        // SeasonManager.getSeedDropFactor(3): not translated, it has no body.
        // SeasonManager.getFruitProductionFactor(3): not translated, it has no body.
        // SeasonManager.getSeasonValue(2): not translated, it has no body.
        // SeasonManager.getPeakFruitProductionSeasonValue(3): not translated, it has no body.
        // SeasonManager.getClimate(2): not translated, it has no body.
        // SeasonManager.shouldSnowMelt(2): not translated, it has no body.
        // SeasonManager.clearCache(1): not translated, it has no body.
    }

    /// `SeasonProvider` (com/dtteam/dynamictrees/api/season/SeasonProvider.java)
    pub mod season_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SeasonProvider.getSeasonValue(2): not translated, it has no body.
        // SeasonProvider.updateTick(2): not translated, it has no body.
        // SeasonProvider.shouldSnowMelt(2): not translated, it has no body.
    }

    /// `SeasonType` (com/dtteam/dynamictrees/api/season/SeasonType.java)
    pub mod season_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum SeasonType {
            Temperature,
            DryWet,
        }
        impl SeasonType {
            pub const VALUES: [SeasonType; 2] = [SeasonType::Temperature, SeasonType::DryWet, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    SeasonType::Temperature => "TEMPERATURE",
                    SeasonType::DryWet => "DRY_WET",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "TEMPERATURE" => SeasonType::Temperature,
                    "DRY_WET" => SeasonType::DryWet,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `Emptiable` (com/dtteam/dynamictrees/api/substance/Emptiable.java)
    pub mod emptiable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Emptiable.getEmptyContainer(0): not translated, it has no body.
    }

    /// `SubstanceEffect` (com/dtteam/dynamictrees/api/substance/SubstanceEffect.java)
    pub mod substance_effect {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SubstanceEffect.apply(2): not translated, it has no body.
        // SubstanceEffect.update(4): not translated, its class isn't translated (it's an interface).
        // SubstanceEffect.getName(0): not translated, it has no body.
        // SubstanceEffect.isLingering(0): not translated, it has no body.
    }

    /// `SubstanceEffectProvider` (com/dtteam/dynamictrees/api/substance/SubstanceEffectProvider.java)
    pub mod substance_effect_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SubstanceEffectProvider.getSubstanceEffect(1): not translated, it has no body.
    }

    /// `BiomeSuitabilityDecider` (com/dtteam/dynamictrees/api/treedata/BiomeSuitabilityDecider.java)
    pub mod biome_suitability_decider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomeSuitabilityDecider.getSuitability(4): not translated, it has no body.
    }

    /// `BiomeSuitabilityDecider.Decision` (com/dtteam/dynamictrees/api/treedata/BiomeSuitabilityDecider.java)
    pub mod biome_suitability_decider_decision {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Decision {
            pub handled: bool,
            pub suitability: f32,
        }
        impl Decision {
            pub fn new() -> Self {
                let mut s = Self { handled: false, suitability: 0.0f32 };
                s.handled = false;
                s
            }
            // BiomeSuitabilityDecider.Decision.Decision(1): not translated, unknown name `class_3532`.
            pub fn is_handled(&self) -> bool {
                return self.handled;
            }
            pub fn get_suitability(&self) -> f32 {
                return self.suitability;
            }
        }
    }

    /// `BranchShapeState` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java)
    pub mod branch_shape_state {
        use super::*;
        pub static BLOCK_START: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0i32; 8]);
        pub static TOTAL_STATES: LazyLock<i32> = LazyLock::new(|| 0i32);
        #[derive(Clone, Debug, PartialEq)]
        pub struct BranchShapeState {
            pub down: i8,
            pub up: i8,
            pub north: i8,
            pub south: i8,
            pub west: i8,
            pub east: i8,
            pub core: i8,
        }
        impl BranchShapeState {
            pub fn new(down: i8, up: i8, north: i8, south: i8, west: i8, east: i8, core: i8) -> Self {
                let mut s = Self { down: down, up: up, north: north, south: south, west: west, east: east, core: core };
                s.down = down;
                s.up = up;
                s.north = north;
                s.south = south;
                s.west = west;
                s.east = east;
                s.core = core;
                s
            }
            pub fn to_index(&self) -> i32 {
                let base: i32 = (self.core as i32) + 1;
                let mut local: i32 = (self.down as i32);
                local = (local * base) + (self.up as i32);
                local = (local * base) + (self.north as i32);
                local = (local * base) + (self.south as i32);
                local = (local * base) + (self.west as i32);
                local = (local * base) + (self.east as i32);
                return ((*branch_shape_state::BLOCK_START)[self.core as usize]) + local;
            }
            pub fn down(&self) -> i8 {
                return self.down;
            }
            pub fn up(&self) -> i8 {
                return self.up;
            }
            pub fn north(&self) -> i8 {
                return self.north;
            }
            pub fn south(&self) -> i8 {
                return self.south;
            }
            pub fn west(&self) -> i8 {
                return self.west;
            }
            pub fn east(&self) -> i8 {
                return self.east;
            }
            pub fn core(&self) -> i8 {
                return self.core;
            }
        }
        pub fn from_array(radii: &[i8]) -> branch_shape_state::BranchShapeState {
            return branch_shape_state::BranchShapeState::new(radii[0], radii[1], radii[2], radii[3], radii[4], radii[5], radii[6]);
        }
    }

    /// `SurfaceRootShapeState` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java)
    pub mod surface_root_shape_state {
        use super::*;
        pub static BLOCK_START: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0i32; 9]);
        pub static TOTAL_STATES: LazyLock<i32> = LazyLock::new(|| 0i32);
        #[derive(Clone, Debug, PartialEq)]
        pub struct SurfaceRootShapeState {
            pub south: i8,
            pub west: i8,
            pub north: i8,
            pub east: i8,
            pub core: i8,
        }
        impl SurfaceRootShapeState {
            pub fn new(south: i8, west: i8, north: i8, east: i8, core: i8) -> Self {
                let mut s = Self { south: south, west: west, north: north, east: east, core: core };
                s.south = south;
                s.west = west;
                s.north = north;
                s.east = east;
                s.core = core;
                s
            }
            pub fn to_index(&self) -> i32 {
                let base: i32 = (self.core as i32) + 1;
                let mut local: i32 = (self.north as i32);
                local = (local * base) + (self.south as i32);
                local = (local * base) + (self.west as i32);
                local = (local * base) + (self.east as i32);
                return ((*surface_root_shape_state::BLOCK_START)[self.core as usize]) + local;
            }
            pub fn south(&self) -> i8 {
                return self.south;
            }
            pub fn west(&self) -> i8 {
                return self.west;
            }
            pub fn north(&self) -> i8 {
                return self.north;
            }
            pub fn east(&self) -> i8 {
                return self.east;
            }
            pub fn core(&self) -> i8 {
                return self.core;
            }
        }
        pub fn from_array(radii: &[i8]) -> surface_root_shape_state::SurfaceRootShapeState {
            return surface_root_shape_state::SurfaceRootShapeState::new(radii[0], radii[1], radii[2], radii[3], radii[4]);
        }
    }

    /// `TreePart` (com/dtteam/dynamictrees/api/treedata/TreePart.java)
    pub mod tree_part {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TreePart.getHydrationCell(5): not translated, it has no body.
        // TreePart.growSignal(3): not translated, it has no body.
        // TreePart.probabilityForBlock(4): not translated, it has no body.
        // TreePart.getRadiusForConnection(6): not translated, it has no body.
        // TreePart.getRadius(1): not translated, it has no body.
        // TreePart.shouldAnalyse(3): not translated, it has no body.
        // TreePart.analyse(5): not translated, it has no body.
        // TreePart.getFamily(3): not translated, it has no body.
        // TreePart.branchSupport(6): not translated, it has no body.
        // TreePart.getTreePartType(0): not translated, it has no body.
        // TreePart.isRootNode(0): not translated, its class isn't translated (it's an interface).
    }

    /// `TreePart.TreePartType` (com/dtteam/dynamictrees/api/treedata/TreePart.java)
    pub mod tree_part_tree_part_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum TreePartType {
            Null,
            Root,
            Branch,
            Leaves,
            Other,
        }
        impl TreePartType {
            pub const VALUES: [TreePartType; 5] = [TreePartType::Null, TreePartType::Root, TreePartType::Branch, TreePartType::Leaves, TreePartType::Other, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    TreePartType::Null => "NULL",
                    TreePartType::Root => "ROOT",
                    TreePartType::Branch => "BRANCH",
                    TreePartType::Leaves => "LEAVES",
                    TreePartType::Other => "OTHER",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NULL" => TreePartType::Null,
                    "ROOT" => TreePartType::Root,
                    "BRANCH" => TreePartType::Branch,
                    "LEAVES" => TreePartType::Leaves,
                    "OTHER" => TreePartType::Other,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BlockPosBounds` (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java)
    pub mod block_pos_bounds {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        // INVALID: not translated, its value couldn't be read: an anonymous class.
        // BlockPosBounds.BlockPosBounds(0): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.BlockPosBounds(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.BlockPosBounds(2): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.BlockPosBounds(6): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.BlockPosBounds(2): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.BlockPosBounds(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.BlockPosBounds(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.union(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.union(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.inBounds(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.getMin(0): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.getMax(0): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.expand(2): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.shrink(2): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.move(3): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.move(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.expand(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.shrink(1): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.iterator(0): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.getXSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.getYSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.getZSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.toAABB(0): not translated, its class isn't translated (it implements `Iterable`).
        // BlockPosBounds.toString(0): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `SimpleVoxmap` (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java)
    pub mod simple_voxmap {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2338`. Its static parts are below.
        // SimpleVoxmap.SimpleVoxmap(3): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.SimpleVoxmap(4): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.SimpleVoxmap(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.SimpleVoxmap(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.setMapAndCenter(2): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.setMap(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.setCenter(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getCenter(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getData(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getLenX(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getLenY(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getLenZ(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getBounds(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.blitOp(3): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.blitReplace(2): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.blitMax(2): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.blitClear(2): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.filter(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.crop(2): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.filter(3): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.fill(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.fill(3): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.calcPos(3): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.setVoxel(2): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.setVoxelOr(2): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.setVoxel(4): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.setVoxelOr(4): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getVoxel(2): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getVoxel(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getVoxel(3): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.testBounds(3): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.isYTouched(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.setYTouched(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getAllNonZeroCells(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getAllNonZeroCells(1): not translated, its body couldn't be read: an anonymous class.
        // SimpleVoxmap.getAllNonZero(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
        // SimpleVoxmap.getAllNonZero(1): not translated, its body couldn't be read: an anonymous class.
        // SimpleVoxmap.getTops(0): not translated, its body couldn't be read: an anonymous class.
        // SimpleVoxmap.print(0): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
    }

    /// `SimpleVoxmap.BlitOp` (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java)
    pub mod simple_voxmap_blit_op {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SimpleVoxmap.BlitOp.getOp(2): not translated, it has no body.
    }

    /// `SimpleVoxmap.FilterOp` (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java)
    pub mod simple_voxmap_filter_op {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SimpleVoxmap.FilterOp.getOp(1): not translated, it has no body.
    }

    /// `SimpleVoxmap.VoxmapCell` (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java)
    pub mod simple_voxmap_voxmap_cell {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2339`. Its static parts are below.
        // SimpleVoxmap.VoxmapCell.setValue(1): not translated, its class isn't translated (it uses Minecraft's `class_2339`).
        // SimpleVoxmap.VoxmapCell.getValue(0): not translated, its class isn't translated (it uses Minecraft's `class_2339`).
        // SimpleVoxmap.VoxmapCell.getPos(0): not translated, its class isn't translated (it uses Minecraft's `class_2339`).
    }

    /// `BiomePredicate` (com/dtteam/dynamictrees/api/worldgen/BiomePredicate.java)
    pub mod biome_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `BiomePropertySelectors` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BiomePropertySelectors {
        }
        impl BiomePropertySelectors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BiomePropertySelectors.Chance` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_chance {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Chance {
            Ok,
            Cancel,
            Unhandled,
        }
        impl Chance {
            pub const VALUES: [Chance; 3] = [Chance::Ok, Chance::Cancel, Chance::Unhandled, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Chance::Ok => "OK",
                    Chance::Cancel => "CANCEL",
                    Chance::Unhandled => "UNHANDLED",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "OK" => Chance::Ok,
                    "CANCEL" => Chance::Cancel,
                    "UNHANDLED" => Chance::Unhandled,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BiomePropertySelectors.ChanceSelector` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_chance_selector {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomePropertySelectors.ChanceSelector.getChance(1): not translated, it has no body.
    }

    /// `BiomePropertySelectors.DensitySelector` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_density_selector {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomePropertySelectors.DensitySelector.getDensity(1): not translated, it has no body.
    }

    /// `BiomePropertySelectors.FeatureCancellation` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_feature_cancellation {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomePropertySelectors.FeatureCancellation.cancelFeatures(1): not translated, it has no body.
        // BiomePropertySelectors.FeatureCancellation.cancelUsing(1): not translated, it has no body.
        // BiomePropertySelectors.FeatureCancellation.cancelWithNamespace(1): not translated, it has no body.
        // BiomePropertySelectors.FeatureCancellation.cancelDuring(1): not translated, it has no body.
    }

    /// `BiomePropertySelectors.NoFeatureCancellation` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_no_feature_cancellation {
        use super::*;
        // The class itself isn't translated: it implements `FeatureCancellation`. Its static parts are below.
        // INSTANCE: not translated, it uses NoFeatureCancellation, which isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NoFeatureCancellation.NoFeatureCancellation(0): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NoFeatureCancellation.cancelFeatures(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NoFeatureCancellation.cancelUsing(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NoFeatureCancellation.cancelWithNamespace(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NoFeatureCancellation.cancelDuring(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
    }

    /// `BiomePropertySelectors.NormalFeatureCancellation` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_normal_feature_cancellation {
        use super::*;
        // The class itself isn't translated: it implements `FeatureCancellation`. Its static parts are below.
        // BiomePropertySelectors.NormalFeatureCancellation.NormalFeatureCancellation(0): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.NormalFeatureCancellation(3): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.cancelFeatures(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.shouldRemovePlacedFeature(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.cancelUsing(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.cancelWithNamespace(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.cancelDuring(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.cancelDuringDefaultIfNoneSpecified(0): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.addFrom(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.replaceFrom(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.reset(0): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.getCancellers(0): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.getNamespaces(0): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.getDecorationSteps(0): not translated, its class isn't translated (it implements `FeatureCancellation`).
        // BiomePropertySelectors.NormalFeatureCancellation.shouldCancelNamespace(1): not translated, its class isn't translated (it implements `FeatureCancellation`).
    }

    /// `BiomePropertySelectors.RandomSpeciesSelector` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_random_species_selector {
        use super::*;
        // The class itself isn't translated: it implements `SpeciesSelector`. Its static parts are below.
        // BiomePropertySelectors.RandomSpeciesSelector.getSize(0): not translated, its class isn't translated (it implements `SpeciesSelector`).
        // BiomePropertySelectors.RandomSpeciesSelector.add(2): not translated, its class isn't translated (it implements `SpeciesSelector`).
        // BiomePropertySelectors.RandomSpeciesSelector.add(1): not translated, its class isn't translated (it implements `SpeciesSelector`).
        // BiomePropertySelectors.RandomSpeciesSelector.getSpecies(3): not translated, its class isn't translated (it implements `SpeciesSelector`).
    }

    /// `BiomePropertySelectors.RandomSpeciesSelector.Entry` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_random_species_selector_entry {
        use super::*;
        // The class itself isn't translated: it uses SpeciesSelection, which isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))). Its static parts are below.
        // BiomePropertySelectors.RandomSpeciesSelector.Entry.Entry(2): not translated, its class isn't translated (it uses SpeciesSelection, which isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)))).
    }

    /// `BiomePropertySelectors.SpeciesSelection` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_species_selection {
        use super::*;
        // The class itself isn't translated: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)). Its static parts are below.
        // BiomePropertySelectors.SpeciesSelection.SpeciesSelection(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BiomePropertySelectors.SpeciesSelection.SpeciesSelection(1): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BiomePropertySelectors.SpeciesSelection.isHandled(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
        // BiomePropertySelectors.SpeciesSelection.getSpecies(0): not translated, its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))).
    }

    /// `BiomePropertySelectors.SpeciesSelector` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_species_selector {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomePropertySelectors.SpeciesSelector.getSpecies(3): not translated, it has no body.
    }

    /// `BiomePropertySelectors.StaticSpeciesSelector` (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java)
    pub mod biome_property_selectors_static_species_selector {
        use super::*;
        // The class itself isn't translated: it implements `SpeciesSelector`. Its static parts are below.
        // BiomePropertySelectors.StaticSpeciesSelector.StaticSpeciesSelector(1): not translated, its class isn't translated (it implements `SpeciesSelector`).
        // BiomePropertySelectors.StaticSpeciesSelector.StaticSpeciesSelector(1): not translated, its class isn't translated (it implements `SpeciesSelector`).
        // BiomePropertySelectors.StaticSpeciesSelector.StaticSpeciesSelector(0): not translated, its class isn't translated (it implements `SpeciesSelector`).
        // BiomePropertySelectors.StaticSpeciesSelector.getSpecies(3): not translated, its class isn't translated (it implements `SpeciesSelector`).
    }

    /// `FeatureCanceller` (com/dtteam/dynamictrees/api/worldgen/FeatureCanceller.java)
    pub mod feature_canceller {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // NULL_CANCELLER: not translated, its value couldn't be read: an anonymous class.
        // REGISTRY: not translated, it uses SimpleRegistry, which isn't translated (it's generic).
        // FeatureCanceller.FeatureCanceller(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // FeatureCanceller.shouldCancel(2): not translated, it has no body.
    }

    /// `GroundFinder` (com/dtteam/dynamictrees/api/worldgen/GroundFinder.java)
    pub mod ground_finder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GroundFinder.findGround(3): not translated, it has no body.
        // GroundFinder.registerGroundFinder(2): not translated, it uses Minecraft's `class_5321`.
        // GroundFinder.getGroundFinder(1): not translated, it uses GroundFinder, which isn't translated (it's an interface).
    }

    /// `LevelContext` (com/dtteam/dynamictrees/api/worldgen/LevelContext.java)
    pub mod level_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_5321`. Its static parts are below.
        // LevelContext.LevelContext(4): not translated, its class isn't translated (it uses Minecraft's `class_5321`).
        // LevelContext.dimensionKey(0): not translated, its class isn't translated (it uses Minecraft's `class_5321`).
        // LevelContext.dimensionName(0): not translated, its class isn't translated (it uses Minecraft's `class_5321`).
        // LevelContext.seed(0): not translated, its class isn't translated (it uses Minecraft's `class_5321`).
        // LevelContext.accessor(0): not translated, its class isn't translated (it uses Minecraft's `class_5321`).
        // LevelContext.level(0): not translated, its class isn't translated (it uses Minecraft's `class_5321`).
        // LevelContext.create(1): not translated, it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`).
        // LevelContext.getServerLevelOrThrow(1): not translated, it uses Minecraft's `class_3218`.
    }

    /// `PoissonDebug` (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java)
    pub mod poisson_debug {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PoissonDebug.begin(2): not translated, it has no body.
        // PoissonDebug.collectSolved(1): not translated, it has no body.
        // PoissonDebug.doEdgeMasking(1): not translated, it has no body.
        // PoissonDebug.maskSolvedDiscs(1): not translated, it has no body.
        // PoissonDebug.createRootDisc(2): not translated, it has no body.
        // PoissonDebug.gatherUnsolved(2): not translated, it has no body.
        // PoissonDebug.updateCount(3): not translated, it has no body.
        // PoissonDebug.pickMasterDisc(3): not translated, it has no body.
        // PoissonDebug.getRadius(4): not translated, it has no body.
        // PoissonDebug.findSecondDisc(4): not translated, it has no body.
        // PoissonDebug.maskMasterSlave(4): not translated, it has no body.
        // PoissonDebug.intersectingList(3): not translated, it has no body.
        // PoissonDebug.findThirdDiscCandidate(5): not translated, it has no body.
        // PoissonDebug.thirdCircleCandidateIntersects(6): not translated, it has no body.
        // PoissonDebug.findThirdDiscSolved(3): not translated, it has no body.
        // PoissonDebug.solveDiscs(2): not translated, it has no body.
        // PoissonDebug.gatherUnsolved2(2): not translated, it has no body.
        // PoissonDebug.unsolvable(5): not translated, its class isn't translated (it's an interface).
    }

    /// `PoissonDiscProvider` (com/dtteam/dynamictrees/api/worldgen/PoissonDiscProvider.java)
    pub mod poisson_disc_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PoissonDiscProvider.getPoissonDiscs(3): not translated, it has no body.
        // PoissonDiscProvider.getChunkPoissonData(3): not translated, it has no body.
        // PoissonDiscProvider.setChunkPoissonData(4): not translated, it has no body.
        // PoissonDiscProvider.unloadChunkPoissonData(3): not translated, it has no body.
    }

    /// `RadiusCoordinator` (com/dtteam/dynamictrees/api/worldgen/RadiusCoordinator.java)
    pub mod radius_coordinator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RadiusCoordinator.getRadiusAtCoords(2): not translated, it has no body.
        // RadiusCoordinator.runPass(3): not translated, it has no body.
    }

    /// `RandomXOR` (com/dtteam/dynamictrees/api/worldgen/RandomXOR.java)
    pub mod random_xor {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_7384`. Its static parts are below.
        pub const SERIAL_VERSION_UID: i64 = -3477272122511092632i64;
        // RandomXOR.RandomXOR(0): not translated, its class isn't translated (it extends Minecraft's `class_7384`).
        // RandomXOR.RandomXOR(1): not translated, its class isn't translated (it extends Minecraft's `class_7384`).
        // RandomXOR.setXOR(1): not translated, its class isn't translated (it extends Minecraft's `class_7384`).
        // RandomXOR.setXOR(1): not translated, its class isn't translated (it extends Minecraft's `class_7384`).
        // RandomXOR.method_43156(1): not translated, its class isn't translated (it extends Minecraft's `class_7384`).
    }

    /// `Ageable` (com/dtteam/dynamictrees/block/Ageable.java)
    pub mod ageable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Ageable.age(5): not translated, it has no body.
    }

    /// `BlockWithDynamicHardness` (com/dtteam/dynamictrees/block/BlockWithDynamicHardness.java)
    pub mod block_with_dynamic_hardness {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2248`. Its static parts are below.
        // BlockWithDynamicHardness.BlockWithDynamicHardness(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // BlockWithDynamicHardness.getHardness(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // BlockWithDynamicHardness.hasTileEntity(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
    }

    /// `BlockWithDynamicHardness.DynamicHardnessBlockState` (com/dtteam/dynamictrees/block/BlockWithDynamicHardness.java)
    pub mod block_with_dynamic_hardness_dynamic_hardness_block_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2680`. Its static parts are below.
        // BlockWithDynamicHardness.DynamicHardnessBlockState.DynamicHardnessBlockState(3): not translated, its class isn't translated (it extends Minecraft's `class_2680`).
        // BlockWithDynamicHardness.DynamicHardnessBlockState.method_26214(2): not translated, its class isn't translated (it extends Minecraft's `class_2680`).
        // BlockWithDynamicHardness.DynamicHardnessBlockState.method_31709(0): not translated, its class isn't translated (it extends Minecraft's `class_2680`).
    }

    /// `BasicBranchBlock` (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java)
    pub mod basic_branch_block {
        use super::*;
        // The class itself isn't translated: it extends `BranchBlock`. Its static parts are below.
        // RADIUS: not translated, it uses Minecraft's `class_2758`.
        // WATERLOGGED: not translated, it uses Minecraft's `class_2746`.
        // BasicBranchBlock.BasicBranchBlock(2): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.BasicBranchBlock(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.createBranchStates(2): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.method_9515(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.branchSupport(6): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.canFall(0): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.checkForRot(8): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.method_9545(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.method_9559(6): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.method_10310(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getHardness(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getFireSpreadSpeed(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getFlammability(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.setFlammability(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.setFireSpreadSpeed(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getHydrationCell(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getRadius(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.setRadius(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getStateForRadius(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.probabilityForBlock(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.growIntoAir(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.growSignal(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.isLadder(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getRadiusForConnection(6): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getMaxSignalDepth(0): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.analyse(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicBranchBlock.getStateForDecay(3): not translated, its class isn't translated (it extends `BranchBlock`).
    }

    /// `BasicRootsBlock` (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java)
    pub mod basic_roots_block {
        use super::*;
        // The class itself isn't translated: it extends `BranchBlock`. Its static parts are below.
        pub const NAME_SUFFIX: &str = "_roots";
        // RADIUS: not translated, it uses Minecraft's `class_2758`.
        // LAYER: not translated, it uses Minecraft's `class_2754`.
        // WATERLOGGED: not translated, it uses Minecraft's `class_2746`.
        // log: the logger. Log calls become comments.
        // BasicRootsBlock.BasicRootsBlock(2): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.isFullBlock(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getFamily(0): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getFireSpreadSpeed(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getFlammability(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.setFlammability(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.setFireSpreadSpeed(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9515(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getHydrationCell(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.probabilityForBlock(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getRadiusForConnection(6): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getRadius(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getMaxSignalDepth(0): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.branchSupport(6): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.setRadius(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getStateForRadius(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9574(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9573(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9545(1): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9559(6): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_10310(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_10311(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getLootTableName(0): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getLootTable(2): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getPrimitiveLog(0): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.canPlace(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_55765(7): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.onDestroyedByPlayer(6): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.futureBreak(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.destroyBranchFromNode(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getHardness(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.getStateForDecay(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9594(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.checkForRot(8): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9530(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9549(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9571(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_26159(4): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.method_9522(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.isTransparent(1): not translated, it uses Minecraft's `class_2680`.
        // BasicRootsBlock.canGrowInto(2): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.isNextToSoil(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.growIntoAir(5): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.growSignal(3): not translated, its class isn't translated (it extends `BranchBlock`).
        // BasicRootsBlock.analyse(5): not translated, its class isn't translated (it extends `BranchBlock`).
    }

    /// `BasicRootsBlock.Layer` (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java)
    pub mod basic_roots_block_layer {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BasicRootsBlock.Layer.Layer(1): not translated, its class isn't translated (its constants carry values).
        // BasicRootsBlock.Layer.method_15434(0): not translated, its class isn't translated (its constants carry values).
        // BasicRootsBlock.Layer.getPrimitive(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `BranchBlock` (com/dtteam/dynamictrees/block/branch/BranchBlock.java)
    pub mod branch_block {
        use super::*;
        // The class itself isn't translated: it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block). Its static parts are below.
        pub const MAX_RADIUS: i32 = 8;
        pub const NAME_SUFFIX: &str = "_branch";
        // shapeCache: not translated, it uses Minecraft's `class_265`.
        pub static DESTROY_MODE: JCell<dynamic_trees_destroy_mode::DestroyMode> = JCell::new(dynamic_trees_destroy_mode::DestroyMode::Sloppy);
        // BranchBlock.BranchBlock(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.BranchBlock(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.setCanBeStripped(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.setFamily(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getFamily(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getFamily(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.isSameTree(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.isSameTree(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.isSameTree(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getPrimitiveLog(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.isStrippedBranch(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.branchSupport(6): not translated, it has no body.
        // BranchBlock.checkForRot(8): not translated, it has no body.
        pub fn set_support(branches: i32, leaves: i32) -> i32 {
            return ((branches & 15) << 4) | (leaves & 15);
        }
        pub fn get_branch_support(support: i32) -> i32 {
            return (support >> 4) & 15;
        }
        pub fn get_leaves_support(support: i32) -> i32 {
            return support & 15;
        }
        // BranchBlock.isNextToBranch(3): not translated, it uses Minecraft's `class_1937`.
        // BranchBlock.method_55765(7): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.canBeStripped(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.stripBranch(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.stripBranch(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9574(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9516(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getStateForDecay(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9651(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9650(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9652(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getConnectionData(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.connectToLeaves(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9604(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9530(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.generateNewShape(1): not translated, it uses Minecraft's `class_265`.
        // BranchBlock.makeCube(1): not translated, it uses Minecraft's `class_238`.
        // BranchBlock.getSideConnectionRadius(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getRadius(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.setRadius(5): not translated, it has no body.
        // BranchBlock.setRadius(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getStateForRadius(1): not translated, it has no body.
        // BranchBlock.getMaxRadius(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.shouldAnalyse(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.destroyBranchFromNode(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getCachedSoilState(3): not translated, it uses `Pair` (org.apache.commons.lang3.tuple).
        // BranchBlock.rot(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.destroyLeaves(7): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.canFall(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.shouldGenerateBranchDrops(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getLootTableName(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getLootTable(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.createBranchDrops(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getPrimitiveLogs(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.setPrimitiveLogDrops(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.futureBreak(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.onDestroyedByPlayer(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.sloppyBreak(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.damageAxe(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9536(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.setBlockStateIgnored(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_9576(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.breakDeliberate(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.method_55124(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // BranchBlock.getTreePartType(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
    }

    /// `BranchBlock.ItemStackPos` (com/dtteam/dynamictrees/block/branch/BranchBlock.java)
    pub mod branch_block_item_stack_pos {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_1799`. Its static parts are below.
        // BranchBlock.ItemStackPos.ItemStackPos(2): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
    }

    /// `Musable` (com/dtteam/dynamictrees/block/branch/Musable.java)
    pub mod musable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Musable.isMusable(3): not translated, it has no body.
    }

    /// `PalmBranchBlock` (com/dtteam/dynamictrees/block/branch/PalmBranchBlock.java)
    pub mod palm_branch_block {
        use super::*;
        // The class itself isn't translated: it extends `BasicBranchBlock`. Its static parts are below.
        // PalmBranchBlock.PalmBranchBlock(2): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // PalmBranchBlock.getHydrationCell(5): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // PalmBranchBlock.growIntoAir(4): not translated, its class isn't translated (it extends `BasicBranchBlock`).
    }

    /// `SurfaceRootBlock` (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java)
    pub mod surface_root_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2248`. Its static parts are below.
        pub const MAX_RADIUS: i32 = 8;
        // RADIUS: not translated, it uses Minecraft's `class_2758`.
        // GROUNDED: not translated, it uses Minecraft's `class_2746`.
        // WATERLOGGED: not translated, it uses Minecraft's `class_2746`.
        // shapeCache: not translated, it uses Minecraft's `class_265`.
        // SurfaceRootBlock.SurfaceRootBlock(2): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.getFamily(0): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.method_9574(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.method_9515(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.getRadius(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.setRadius(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.getStateForRadius(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.getMaxRadius(0): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.getRadialHeight(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.method_9545(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.method_9559(6): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.getConnectionData(2): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.method_9530(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.generateNewRootShape(2): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.isAirOrWater(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.getSideConnectionRadius(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.method_9612(6): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.method_9588(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.method_9576(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.updateDisconnectedRoot(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // SurfaceRootBlock.canBlockStay(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
    }

    /// `SurfaceRootBlock.RootConnection` (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java)
    pub mod surface_root_block_root_connection {
        use super::*;
        // The class itself isn't translated: it uses ConnectionLevel, which isn't translated (its constants carry values). Its static parts are below.
        // SurfaceRootBlock.RootConnection.RootConnection(2): not translated, its class isn't translated (it uses ConnectionLevel, which isn't translated (its constants carry values)).
        // SurfaceRootBlock.RootConnection.toString(0): not translated, its class isn't translated (it uses ConnectionLevel, which isn't translated (its constants carry values)).
    }

    /// `ThickBranchBlock` (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java)
    pub mod thick_branch_block {
        use super::*;
        // The class itself isn't translated: it extends `BasicBranchBlock`. Its static parts are below.
        pub const MAX_RADIUS_THICK: i32 = 24;
        // RADIUS_DOUBLE: not translated, it uses Minecraft's `class_2758`.
        // trunkShapes: not translated, it uses Minecraft's `class_265`.
        // ThickBranchBlock.ThickBranchBlock(2): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.precomputeTrunkShapes(0): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.getTrunkShell(0): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.method_9515(1): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.getRadius(1): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.setRadius(5): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.method_9612(6): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.updateTrunkShells(4): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.getRadiusForConnection(6): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.getSideConnectionRadius(4): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.getReplaceability(3): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.getMaxRadius(0): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.method_9530(4): not translated, its class isn't translated (it extends `BasicBranchBlock`).
        // ThickBranchBlock.isMusable(3): not translated, its class isn't translated (it extends `BasicBranchBlock`).
    }

    /// `ThickBranchBlock.ReplaceableState` (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java)
    pub mod thick_branch_block_replaceable_state {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ReplaceableState {
            Shell,
            Replaceable,
            Blocking,
            Treepart,
        }
        impl ReplaceableState {
            pub const VALUES: [ReplaceableState; 4] = [ReplaceableState::Shell, ReplaceableState::Replaceable, ReplaceableState::Blocking, ReplaceableState::Treepart, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ReplaceableState::Shell => "SHELL",
                    ReplaceableState::Replaceable => "REPLACEABLE",
                    ReplaceableState::Blocking => "BLOCKING",
                    ReplaceableState::Treepart => "TREEPART",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "SHELL" => ReplaceableState::Shell,
                    "REPLACEABLE" => ReplaceableState::Replaceable,
                    "BLOCKING" => ReplaceableState::Blocking,
                    "TREEPART" => ReplaceableState::Treepart,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `TrunkShellBlock` (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java)
    pub mod trunk_shell_block {
        use super::*;
        // The class itself isn't translated: it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block). Its static parts are below.
        // CORE_DIR: not translated, it uses Minecraft's `class_2754`.
        // WATERLOGGED: not translated, it uses Minecraft's `class_2746`.
        // TrunkShellBlock.TrunkShellBlock(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9515(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9588(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.onDestroyedByPlayer(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9594(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9556(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9576(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.getHardness(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9573(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9616(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.getMuseDir(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.museDoesNotExist(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.getMuseUnchecked(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.getMuseUnchecked(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.getMuse(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.getMuse(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.isValid(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.scheduleUpdateTick(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9612(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9530(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9574(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_55124(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.findDetachedMuse(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9585(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_55765(7): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_55766(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.getFireSpreadSpeed(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.getFlammability(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.isFullBlockShell(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.isFullBlockShell(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.isFullBlockShell(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9516(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9545(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9559(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_10310(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.isWaterLogged(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.method_9604(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TrunkShellBlock.addLandingEffects(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
    }

    /// `TrunkShellBlock.ShellMuse` (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java)
    pub mod trunk_shell_block_shell_muse {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2680`. Its static parts are below.
        // TrunkShellBlock.ShellMuse.ShellMuse(4): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
        // TrunkShellBlock.ShellMuse.getRadius(0): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
        // TrunkShellBlock.ShellMuse.state(0): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
        // TrunkShellBlock.ShellMuse.pos(0): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
        // TrunkShellBlock.ShellMuse.dir(0): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
        // TrunkShellBlock.ShellMuse.museOffset(0): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
    }

    /// `CommonVoxelShapes` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java)
    pub mod common_voxel_shapes {
        use super::*;
        // SHAPES: not translated, it uses Minecraft's `class_265`.
        // SLAB: not translated, it uses Minecraft's `class_265`.
        // TALL_SLAB: not translated, it uses Minecraft's `class_265`.
        // HALF_SLAB: not translated, it uses Minecraft's `class_265`.
        // SAPLING_TRUNK: not translated, it uses Minecraft's `class_265`.
        // FAT_SAPLING_TRUNK: not translated, it uses Minecraft's `class_265`.
        // SAPLING_LEAVES: not translated, it uses Minecraft's `class_265`.
        // FAT_SAPLING_LEAVES: not translated, it uses Minecraft's `class_265`.
        // SLIM_SAPLING_LEAVES: not translated, it uses Minecraft's `class_265`.
        // MUSHROOM_CAP_FLAT: not translated, it uses Minecraft's `class_265`.
        // FAT_MUSHROOM_CAP_FLAT: not translated, it uses Minecraft's `class_265`.
        // MUSHROOM_CAP_ROUND: not translated, it uses Minecraft's `class_265`.
        // FAT_MUSHROOM_CAP_ROUND: not translated, it uses Minecraft's `class_265`.
        // MUSHROOM_RIM: not translated, it uses Minecraft's `class_265`.
        // FAT_MUSHROOM_RIM: not translated, it uses Minecraft's `class_265`.
        // SAPLING: not translated, it uses Minecraft's `class_265`.
        // SLIM_SAPLING: not translated, it uses Minecraft's `class_265`.
        // FAT_SAPLING: not translated, it uses Minecraft's `class_265`.
        // FLAT_MUSHROOM: not translated, it uses Minecraft's `class_265`.
        // FAT_FLAT_MUSHROOM: not translated, it uses Minecraft's `class_265`.
        // ROUND_MUSHROOM: not translated, it uses Minecraft's `class_265`.
        // FAT_ROUND_MUSHROOM: not translated, it uses Minecraft's `class_265`.
        // ROUND_MUSHROOM_RIM: not translated, it uses Minecraft's `class_265`.
        // FAT_ROUND_MUSHROOM_RIM: not translated, it uses Minecraft's `class_265`.
        #[derive(Clone, Debug)]
        pub struct CommonVoxelShapes {
        }
        impl CommonVoxelShapes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `DynamicBlockProperties` (com/dtteam/dynamictrees/block/DynamicBlockProperties.java)
    pub mod dynamic_block_properties {
        use super::*;
        // AGE_PROPERTIES: not translated, it uses Minecraft's `class_2758`.
        pub static DEFAULT_AGES: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1, 2, 3, 5, 7, 15, 25]);
        // OFFSET_PROPERTIES: not translated, it uses `Pair` (oshi.util.tuples).
        #[derive(Clone, Debug)]
        pub struct DynamicBlockProperties {
        }
        impl DynamicBlockProperties {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DynamicBlockProperties.getOrCreateAge(1): not translated, it uses Minecraft's `class_2758`.
        // DynamicBlockProperties.getOrCreateOffset(2): not translated, it uses Minecraft's `class_2758`.
        // DynamicBlockProperties.getOrCreateOffset(1): not translated, it uses Minecraft's `class_2758`.
    }

    /// `Fruit` (com/dtteam/dynamictrees/block/fruit/Fruit.java)
    pub mod fruit {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // NULL: not translated, it uses Fruit, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // REGISTRY: not translated, it uses TypedRegistry, which isn't translated (it's generic).
        // Fruit.Fruit(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setSeasonalFactorGetter(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setFloweringPeriodPredicate(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getBlock(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.createBlock(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.createBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getDefaultMapColor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getDefaultBlockProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getDefaultBlockProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getAgeProperty(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getMaxAge(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setMaxAge(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setDropCount(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setMaxDropCount(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setMinDropCount(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getBlockShape(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setBlockShapes(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.canBoneMeal(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setCanBoneMeal(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getItemStack(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setItemStack(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getGrowthChance(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setGrowthChance(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getRequiredProductionFactor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setRequiredProductionFactor(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.seasonalFruitProductionFactor(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.isOutOfSeason(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.isInFlowerHoldPeriod(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.place(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.placeDuringWorldGen(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getStateForAge(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getAgeForWorldGen(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.performMatureAction(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getMatureAction(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.setMatureAction(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.shouldGenerateBlockDrops(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.getBlockDropsPath(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.createBlockDrops(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Fruit.reset(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
    }

    /// `FruitBlock` (com/dtteam/dynamictrees/block/fruit/FruitBlock.java)
    pub mod fruit_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2248`. Its static parts are below.
        // FruitBlock.FruitBlock(2): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9515(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9530(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9514(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.doTick(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.outOfSeason(2): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.tryGrow(6): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.getGrowthChance(2): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.tickMature(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.performMatureAction(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9612(6): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9558(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.isSupported(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.drop(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9574(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.harvest(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_55765(7): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_55766(5): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9651(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9650(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9652(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.getAge(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.getAgeAsPercentage(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.getMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.setAge(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // FruitBlock.method_9516(2): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
    }

    /// `FutureBreakable` (com/dtteam/dynamictrees/block/FutureBreakable.java)
    pub mod future_breakable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // FutureBreakable.futureBreak(4): not translated, it has no body.
    }

    /// `Growable` (com/dtteam/dynamictrees/block/Growable.java)
    pub mod growable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Growable.performMatureAction(3): not translated, it has no body.
        // Growable.drop(3): not translated, its class isn't translated (it's an interface).
        // Growable.rot(3): not translated, its class isn't translated (it's an interface).
        // Growable.isSupported(3): not translated, it has no body.
    }

    /// `Growable.Info` (com/dtteam/dynamictrees/block/Growable.java)
    pub mod growable_info {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_1936`. Its static parts are below.
        // Growable.Info.Info(3): not translated, its class isn't translated (it uses Minecraft's `class_1936`).
    }

    /// `Growable.MatureAction` (com/dtteam/dynamictrees/block/Growable.java)
    pub mod growable_mature_action {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // Growable.MatureAction.MatureAction(1): not translated, its class isn't translated (its constants carry values).
        // Growable.MatureAction.perform(2): not translated, its class isn't translated (its constants carry values).
    }

    /// `DynamicLeavesBlock` (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java)
    pub mod dynamic_leaves_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2397`. Its static parts are below.
        // SUPPORT_SHAPE: not translated, it uses Minecraft's `class_265`.
        // DynamicLeavesBlock.DynamicLeavesBlock(2): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.DynamicLeavesBlock(1): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9542(1): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getLeavesProperties(0): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getFamily(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getFireSpreadSpeed(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getFlammability(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9605(1): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9594(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9574(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.age(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9514(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9612(6): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9588(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.updateAllLeaves(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.updateLeaves(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.updateHydro(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.appearanceChangesWithHydro(2): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getLeavesBlockStateForPlacement(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.growLeavesIfLocationIsSuitable(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.isLocationSuitableForNewLeaves(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.hasAdequateLight(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.removeIfInvalid(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.isBottom(2): not translated, it uses Minecraft's `class_1936`.
        // DynamicLeavesBlock.getHydrationLevelFromNeighbors(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getHydrationCell(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.growSignal(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.needLeaves(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.branchOut(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.probabilityForBlock(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.addLandingEffects(6): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9530(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9549(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_25959(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.isMovementVanilla(0): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.isLeavesPassable(0): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.isEntityPassable(1): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.isEntityPassable(1): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.superFallOn(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9554(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9548(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9560(2): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.createLootParams(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getExactSpecies(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getRadiusForConnection(6): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getRadius(1): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.shouldAnalyse(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.analyse(5): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.branchSupport(6): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.getTreePartType(0): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9575(3): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
        // DynamicLeavesBlock.method_9496(4): not translated, its class isn't translated (it extends Minecraft's `class_2397`).
    }

    /// `LeavesProperties` (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java)
    pub mod leaves_properties {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // blockStateGenerators: not translated, it uses Minecraft's `class_2960`.
        // itemModelGenerators: not translated, it uses Minecraft's `class_2960`.
        // languageGenerators: not translated, it uses Minecraft's `class_2960`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // NULL: not translated, its value couldn't be read: an anonymous class.
        // REGISTRY: not translated, it uses TypedRegistry, which isn't translated (it's generic).
        pub const MAX_HYDRO: i32 = 7;
        pub const LEAVES: &str = "leaves";
        // LeavesProperties.LeavesProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.LeavesProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.LeavesProperties(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.LeavesProperties(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setSeedDropChances(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setSeedDropChances(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getBlockLootTableName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getBlockLootTable(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.shouldGenerateBlockDrops(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.createBlockDrops(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getLootTableName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getLootTable(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.shouldGenerateDrops(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.createDrops(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getDrops(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.createLootParams(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.generateStateData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.generateLangData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getPrimitiveLeaves(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getPrimitiveLeavesBlock(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setPrimitiveLeaves(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.isOnlyIfLoaded(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setOnlyIfLoaded(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getPrimitiveLeavesItemStack(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setTextureOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setModelOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setLangOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getTexturePath(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getModelPath(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getLangOverride(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getBlockRegistryName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setBlockRegistryName(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getBlockRegistryNameSuffix(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getDynamicLeavesBlock(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.createDynamicLeaves(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.generateDynamicLeaves(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setDynamicLeavesState(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getDynamicLeavesState(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getDynamicLeavesState(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getFamily(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setFamily(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getFlammability(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setFlammability(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getFireSpreadSpeed(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setFireSpreadSpeed(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getSmotherLeavesMax(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setSmotherLeavesMax(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getLightRequirement(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setLightRequirement(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.shouldAge(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setAgeingConfiguration(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setCanGrowOnGround(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.canGrowOnGround(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getCellKit(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setCellKit(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.isConnectAnyRadius(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setConnectAnyRadius(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getDefaultMapColor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getDefaultBlockProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getDefaultBlockProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.updateTick(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.getRadiusForConnection(6): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.doRequireShears(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setRequiresShears(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setHasTickParticles(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setWaterResistant(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.defaultLeavesTags(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.isCompatibleLeaves(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.maxLeavesRecursion(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setColorNumber(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.setColorString(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.treeFallColorMultiplier(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.foliageColorMultiplier(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.processColor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.postInitClient(0): not translated, it uses LeavesProperties.REGISTRY, which isn't translated (it uses TypedRegistry, which isn't translated (it's generic)).
        // LeavesProperties.toReloadDataString(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // LeavesProperties.addGeneratedBlockTags(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
    }

    /// `LeavesProperties.AgeingConfiguration` (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java)
    pub mod leaves_properties_ageing_configuration {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // LeavesProperties.AgeingConfiguration.AgeingConfiguration(2): not translated, its class isn't translated (its constants carry values).
        // LeavesProperties.AgeingConfiguration.shouldAge(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `PalmLeavesProperties` (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java)
    pub mod palm_leaves_properties {
        use super::*;
        // The class itself isn't translated: it extends `LeavesProperties`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        pub const FROND: &str = "frond";
        pub const CORE_TOP: &str = "core_top";
        pub const CORE_BOTTOM: &str = "core_bottom";
        // PalmLeavesProperties.PalmLeavesProperties(1): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.createDynamicLeaves(1): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.setFrondLoader(1): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.getFrondLoader(0): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.generateStateData(1): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.getFrondsModelName(0): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.getCoreTopModelName(0): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.getCoreBottomModelName(0): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.addFrondTextures(2): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.addCoreTextures(2): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.getCoreTopSmartModelLocation(0): not translated, its class isn't translated (it extends `LeavesProperties`).
        // PalmLeavesProperties.getCoreBottomSmartModelLocation(0): not translated, its class isn't translated (it extends `LeavesProperties`).
    }

    /// `PalmLeavesProperties.DynamicPalmLeavesBlock` (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java)
    pub mod palm_leaves_properties_dynamic_palm_leaves_block {
        use super::*;
        // The class itself isn't translated: it extends `DynamicLeavesBlock`. Its static parts are below.
        // DIRECTION: not translated, it uses Minecraft's `class_2758`.
        // hydroSurroundMap: not translated, it uses Surround, which isn't translated (its constants carry values).
        // PalmLeavesProperties.DynamicPalmLeavesBlock.method_9514(4): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // PalmLeavesProperties.DynamicPalmLeavesBlock.DynamicPalmLeavesBlock(2): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // PalmLeavesProperties.DynamicPalmLeavesBlock.method_9515(1): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // PalmLeavesProperties.DynamicPalmLeavesBlock.getDirectionState(2): not translated, it uses Minecraft's `class_2680`.
        // PalmLeavesProperties.DynamicPalmLeavesBlock.getRadiusForConnection(6): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // PalmLeavesProperties.DynamicPalmLeavesBlock.branchSupport(6): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // PalmLeavesProperties.DynamicPalmLeavesBlock.appearanceChangesWithHydro(2): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // PalmLeavesProperties.DynamicPalmLeavesBlock.getLeavesBlockStateForPlacement(5): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // PalmLeavesProperties.DynamicPalmLeavesBlock.method_9571(3): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
    }

    /// `ScruffyLeavesProperties` (com/dtteam/dynamictrees/block/leaves/ScruffyLeavesProperties.java)
    pub mod scruffy_leaves_properties {
        use super::*;
        // The class itself isn't translated: it extends `LeavesProperties`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // ScruffyLeavesProperties.ScruffyLeavesProperties(1): not translated, its class isn't translated (it extends `LeavesProperties`).
        // ScruffyLeavesProperties.setLeafChance(1): not translated, its class isn't translated (it extends `LeavesProperties`).
        // ScruffyLeavesProperties.setMaxHydro(1): not translated, its class isn't translated (it extends `LeavesProperties`).
        // ScruffyLeavesProperties.createDynamicLeaves(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `SolidDynamicLeavesBlock` (com/dtteam/dynamictrees/block/leaves/SolidDynamicLeavesBlock.java)
    pub mod solid_dynamic_leaves_block {
        use super::*;
        // The class itself isn't translated: it extends `DynamicLeavesBlock`. Its static parts are below.
        // SolidDynamicLeavesBlock.SolidDynamicLeavesBlock(2): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // SolidDynamicLeavesBlock.method_25959(3): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // SolidDynamicLeavesBlock.method_9549(4): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // SolidDynamicLeavesBlock.addLandingEffects(6): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // SolidDynamicLeavesBlock.method_9554(5): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
        // SolidDynamicLeavesBlock.method_9548(4): not translated, its class isn't translated (it extends `DynamicLeavesBlock`).
    }

    /// `SolidLeavesProperties` (com/dtteam/dynamictrees/block/leaves/SolidLeavesProperties.java)
    pub mod solid_leaves_properties {
        use super::*;
        // The class itself isn't translated: it extends `LeavesProperties`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // SolidLeavesProperties.SolidLeavesProperties(1): not translated, its class isn't translated (it extends `LeavesProperties`).
        // SolidLeavesProperties.getDefaultBlockProperties(0): not translated, its class isn't translated (it extends `LeavesProperties`).
        // SolidLeavesProperties.createDynamicLeaves(1): not translated, its class isn't translated (it extends `LeavesProperties`).
    }

    /// `WartProperties` (com/dtteam/dynamictrees/block/leaves/WartProperties.java)
    pub mod wart_properties {
        use super::*;
        // The class itself isn't translated: it extends `SolidLeavesProperties`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // WartProperties.WartProperties(1): not translated, its class isn't translated (it extends `SolidLeavesProperties`).
        // WartProperties.getBlockRegistryNameSuffix(0): not translated, its class isn't translated (it extends `SolidLeavesProperties`).
        // WartProperties.getDefaultBlockProperties(0): not translated, its class isn't translated (it extends `SolidLeavesProperties`).
        // WartProperties.defaultLeavesTags(0): not translated, its class isn't translated (it extends `SolidLeavesProperties`).
        // WartProperties.createBlockDrops(1): not translated, its class isn't translated (it extends `SolidLeavesProperties`).
        // WartProperties.createDrops(1): not translated, its class isn't translated (it extends `SolidLeavesProperties`).
    }

    /// `NullTreePart` (com/dtteam/dynamictrees/block/NullTreePart.java)
    pub mod null_tree_part {
        use super::*;
        // The class itself isn't translated: it implements `TreePart` (com.dtteam.dynamictrees.api.treedata). Its static parts are below.
        // NullTreePart.getHydrationCell(5): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.growSignal(3): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.getRadiusForConnection(6): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.probabilityForBlock(4): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.getRadius(1): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.shouldAnalyse(3): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.analyse(5): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.branchSupport(6): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.getFamily(3): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
        // NullTreePart.getTreePartType(0): not translated, its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)).
    }

    /// `OffsetablePodBlock` (com/dtteam/dynamictrees/block/pod/OffsetablePodBlock.java)
    pub mod offsetable_pod_block {
        use super::*;
        // The class itself isn't translated: it extends `PodBlock`. Its static parts are below.
        // OffsetablePodBlock.OffsetablePodBlock(2): not translated, its class isn't translated (it extends `PodBlock`).
        // OffsetablePodBlock.method_9515(1): not translated, its class isn't translated (it extends `PodBlock`).
        // OffsetablePodBlock.method_9605(1): not translated, its class isn't translated (it extends `PodBlock`).
        // OffsetablePodBlock.method_9612(6): not translated, its class isn't translated (it extends `PodBlock`).
        // OffsetablePodBlock.method_9530(4): not translated, its class isn't translated (it extends `PodBlock`).
    }

    /// `Pod` (com/dtteam/dynamictrees/block/pod/Pod.java)
    pub mod pod {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // NULL: not translated, it uses Pod, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // REGISTRY: not translated, it uses TypedRegistry, which isn't translated (it's generic).
        // Pod.Pod(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setSeasonalFactorGetter(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setFloweringPeriodPredicate(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getBlock(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.createBlock(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.createBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getDefaultMapColor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getDefaultBlockProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getDefaultBlockProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getAgeProperty(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getMaxAge(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setMaxAge(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setDropCount(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setMinDropCount(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setMaxDropCount(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getBlockShape(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setBlockShapes(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.canBoneMeal(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setCanBoneMeal(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getItemStack(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.isItem(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setItemStack(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getGrowthChance(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setGrowthChance(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getRequiredProductionFactor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setRequiredProductionFactor(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.seasonalFruitProductionFactor(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.isOutOfSeason(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.isInFlowerHoldPeriod(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.place(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.placeDuringWorldGen(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getStateFor(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getAgeForWorldGen(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.performMatureAction(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setMatureAction(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.shouldGenerateBlockDrops(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getBlockDropsPath(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.createBlockDrops(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setMaxRadius(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.setMinRadius(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.hasVariableOffset(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.isValidRadius(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.getOffsetProperty(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Pod.reset(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
    }

    /// `Pod.BlockShapeData` (com/dtteam/dynamictrees/block/pod/Pod.java)
    pub mod pod_block_shape_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2350`. Its static parts are below.
        // Pod.BlockShapeData.getShapeFor(2): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // Pod.BlockShapeData.setShapesFor(2): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
    }

    /// `PodBlock` (com/dtteam/dynamictrees/block/pod/PodBlock.java)
    pub mod pod_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2383`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // PodBlock.PodBlock(1): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.PodBlock(2): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_53969(0): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9515(1): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9530(4): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9514(4): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.doTick(4): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.outOfSeason(2): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.tryGrow(6): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.getGrowthChance(2): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.tickMature(3): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.performMatureAction(3): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9612(6): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9558(3): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.isSupported(3): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9574(3): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.harvest(3): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_55765(7): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_55766(5): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9651(3): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9650(4): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9652(4): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.getFacing(1): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.getAge(1): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.getAgeAsPercentage(1): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.getMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.setAge(4): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9605(1): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
        // PodBlock.method_9516(2): not translated, its class isn't translated (it extends Minecraft's `class_2383`).
    }

    /// `DynamicSaplingBlock` (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java)
    pub mod dynamic_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2248`. Its static parts are below.
        // SAPLING_REPLACERS: not translated, it uses Minecraft's `class_2248`.
        // DynamicSaplingBlock.DynamicSaplingBlock(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.getSpecies(0): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.getFireSpreadSpeed(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.getFlammability(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9514(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.canSaplingStay(3): not translated, it uses Minecraft's `class_4538`.
        // DynamicSaplingBlock.method_9558(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9651(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9650(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9652(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9573(1): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9612(6): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.dropBlock(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9574(3): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9560(2): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.method_9530(4): not translated, its class isn't translated (it extends Minecraft's `class_2248`).
        // DynamicSaplingBlock.registerSaplingReplacer(2): not translated, it uses Minecraft's `class_2680`.
        // DynamicSaplingBlock.shouldReplaceSaplingWhenPlaced(1): not translated, it uses Minecraft's `class_2680`.
        // DynamicSaplingBlock.shouldReplaceSaplingWhenGrown(1): not translated, it uses Minecraft's `class_2680`.
    }

    /// `PottedSaplingBlock` (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java)
    pub mod potted_sapling_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2237`. Its static parts are below.
        // FLOWER_POT_AABB: not translated, it uses Minecraft's `class_238`.
        // PottedSaplingBlock.PottedSaplingBlock(0): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.getSpecies(2): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.setSpecies(4): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.getPotState(2): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.setPotState(3): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.getTileEntityPottedSapling(2): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_10123(2): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_55765(7): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_55766(5): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.removeSaplingFromPot(5): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_9574(3): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.getCloneItemStack(5): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_9612(6): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.onDestroyedByPlayer(6): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_9556(6): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.spawnDrops(2): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_9516(2): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_9530(4): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_53969(0): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
        // PottedSaplingBlock.method_9604(1): not translated, its class isn't translated (it extends Minecraft's `class_2237`).
    }

    /// `PottedSaplingBlockEntity` (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java)
    pub mod potted_sapling_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2586`. Its static parts are below.
        pub const POT_MIMIC_TAG: &str = "pot_mimic";
        pub const SPECIES_TAG: &str = "species";
        // PottedSaplingBlockEntity.PottedSaplingBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // PottedSaplingBlockEntity.getSpecies(0): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // PottedSaplingBlockEntity.setSpecies(1): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // PottedSaplingBlockEntity.getPot(0): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // PottedSaplingBlockEntity.setPot(1): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // PottedSaplingBlockEntity.method_16887(1): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // PottedSaplingBlockEntity.method_38235(0): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // PottedSaplingBlockEntity.method_11014(2): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // PottedSaplingBlockEntity.method_11007(2): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
    }

    /// `AerialRootsSoilProperties` (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java)
    pub mod aerial_roots_soil_properties {
        use super::*;
        // The class itself isn't translated: it extends `SoilProperties`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // AerialRootsSoilProperties.AerialRootsSoilProperties(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // AerialRootsSoilProperties.setFamily(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // AerialRootsSoilProperties.getFamily(0): not translated, its class isn't translated (it extends `SoilProperties`).
        // AerialRootsSoilProperties.createBlock(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // AerialRootsSoilProperties.getSoilState(3): not translated, its class isn't translated (it extends `SoilProperties`).
        // AerialRootsSoilProperties.defaultSoilBlockTags(0): not translated, its class isn't translated (it extends `SoilProperties`).
    }

    /// `AerialRootsSoilProperties.RootSoilBlock` (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java)
    pub mod aerial_roots_soil_properties_root_soil_block {
        use super::*;
        // The class itself isn't translated: it extends `SoilBlock`. Its static parts are below.
        pub const MIN_RADIUS: i32 = 1;
        pub const MAX_RADIUS: i32 = 8;
        // RADIUS: not translated, it uses Minecraft's `class_2758`.
        // WATERLOGGED: not translated, it uses Minecraft's `class_2746`.
        // AerialRootsSoilProperties.RootSoilBlock.RootSoilBlock(2): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.GetStateFromIndex(1): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.getStateIndex(1): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.getSoilProperties(0): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.method_9515(1): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.method_9545(1): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.method_9559(6): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.getHardness(3): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.method_9530(4): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.method_9549(4): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.method_26159(4): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.method_25959(3): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.getRadius(1): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.isStructurallyUnstable(2): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.fallWithTree(4): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.method_9588(4): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.updateTree(5): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.getDecayBlockStateAir(3): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.getDecayBlockState(3): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.onDestroyedByPlayer(6): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.dropWholeTree(4): not translated, its class isn't translated (it extends `SoilBlock`).
        // AerialRootsSoilProperties.RootSoilBlock.updateRadius(5): not translated, its class isn't translated (it extends `SoilBlock`).
    }

    /// `SoilBlock` (com/dtteam/dynamictrees/block/soil/SoilBlock.java)
    pub mod soil_block {
        use super::*;
        // The class itself isn't translated: it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block). Its static parts are below.
        // soilBlockDecayer: not translated, it uses SoilBlockDecayer, which isn't translated (it's an interface).
        // FERTILITY: not translated, it uses Minecraft's `class_2758`.
        // IS_VARIANT: not translated, it uses Minecraft's `class_2746`.
        // SoilBlock.SoilBlock(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getSoilProperties(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getPrimitiveSoilBlock(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getPrimitiveSoilState(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_23349(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_23350(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9530(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9549(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_26159(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_25959(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9573(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9579(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9499(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9520(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9505(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9560(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9574(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getHardness(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9515(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.GetStateFromIndex(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getStateIndex(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_10123(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.hasTileEntity(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9514(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getTrunkDirection(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.updateTree(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getDecayBlockState(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.doDecay(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9498(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9572(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_55765(7): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.destroyTree(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.destroyTree(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.destroyTree(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.onDestroyedByPlayer(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.onBlockExploded(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9576(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.updateRadius(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9651(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9650(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9652(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getFertility(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.setFertility(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.fertilize(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getHydrationCell(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.growSignal(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getRadius(1): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getRadiusForConnection(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.probabilityForBlock(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.startAnalysis(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.shouldAnalyse(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.analyse(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9612(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.method_9588(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.branchSupport(6): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getFamily(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getMainTrunk(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getTileEntitySpecies(2): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getSpecies(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.setSpecies(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getTreePartType(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.isRootNode(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.colorMultiplier(5): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.getColorFromBark(0): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.rootColor(3): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // SoilBlock.fallWithTree(4): not translated, its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
    }

    /// `SoilBlockDecayer` (com/dtteam/dynamictrees/block/soil/SoilBlockDecayer.java)
    pub mod soil_block_decayer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SoilBlockDecayer.decay(4): not translated, it has no body.
    }

    /// `SoilHelper` (com/dtteam/dynamictrees/block/soil/SoilHelper.java)
    pub mod soil_helper {
        use super::*;
        // LOGGER: the logger. Log calls become comments.
        pub const DIRT_LIKE: &str = "dirt_like";
        pub const SAND_LIKE: &str = "sand_like";
        pub const GRAVEL_LIKE: &str = "gravel_like";
        pub const WATER_LIKE: &str = "water_like";
        pub const NETHER_LIKE: &str = "nether_like";
        pub const NETHER_SOIL_LIKE: &str = "nether_soil_like";
        pub const END_LIKE: &str = "end_like";
        pub const MUD_LIKE: &str = "mud_like";
        pub const TERRACOTTA_LIKE: &str = "terracotta_like";
        pub const SLIME_LIKE: &str = "slime_like";
        pub const FUNGUS_LIKE: &str = "fungus_like";
        pub static ADJECTIVE_MAP: LazyLock<JCell<JMap<String, i32>>> = LazyLock::new(|| JCell::new(JMap::new()));
        // dirtMap: not translated, it uses Minecraft's `class_2248`.
        #[derive(Clone, Debug)]
        pub struct SoilHelper {
        }
        impl SoilHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn create_new_adjective(adj_name: &str) {
            { let a_0 = adj_name.to_string(); let a_1 = 1 << (soil_helper::ADJECTIVE_MAP.with(|o| o.len()) as i32); soil_helper::ADJECTIVE_MAP.with(|o| o.insert(a_0, a_1)) };
        }
        pub fn get_flags(adj_name: &str) -> i32 {
            return { let a_0 = adj_name.to_string(); let a_1 = 0; soil_helper::ADJECTIVE_MAP.with(|o| o.get(&a_0).cloned().unwrap_or(a_1)) };
        }
        // SoilHelper.addSoilPropertiesToMap(1): not translated, it uses SoilProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilHelper.registerSoil(2): not translated, it uses SoilProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilHelper.registerSoil(3): not translated, it's overloaded with the same number of parameters.
        // SoilHelper.registerSoil(3): not translated, it's overloaded with the same number of parameters.
        // SoilHelper.isSoilAcceptable(2): not translated, it uses Minecraft's `class_2680`.
        // SoilHelper.isSoilRegistered(1): not translated, it uses Minecraft's `class_2248`.
        // SoilHelper.getProperties(1): not translated, it uses SoilProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        pub fn get_soil_flags(types: &[String]) -> i32 {
            let mut flags: i32 = 0;
            for t in types.iter() {
                flags |= soil_helper::get_flags(t);
            }
            return flags;
        }
        // SoilHelper.applyIfSoilIsAcceptable(3): not translated, it's generic.
    }

    /// `SoilProperties` (com/dtteam/dynamictrees/block/soil/SoilProperties.java)
    pub mod soil_properties {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // blockStateGenerators: not translated, it uses Minecraft's `class_2960`.
        // itemModelGenerators: not translated, it uses Minecraft's `class_2960`.
        // languageGenerators: not translated, it uses Minecraft's `class_2960`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // NULL_SOIL_PROPERTIES: not translated, its value couldn't be read: an anonymous class.
        // REGISTRY: not translated, it uses TypedRegistry, which isn't translated (it's generic).
        pub const ROOTS: &str = "roots";
        pub const SOIL_BLOCK: &str = "soil_block";
        // SoilProperties.SoilProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.SoilProperties(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.SoilProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.SoilProperties(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getPrimitiveSoilBlock(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getPrimitiveSoilBlockOptional(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setPrimitiveSoilBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.isValidState(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getSoilState(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getPrimitiveSoilState(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getBlockRegistryNamePrefix(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getBlockRegistryName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setBlockRegistryName(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setBlockRegistryNameIfNull(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getBlock(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.generateBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.createBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.shouldGenerateBlock(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setGenerateBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setSubstitute(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setFoliageTintIndex(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setRootsTintIndex(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getDefaultMapColor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getDefaultBlockProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getDefaultBlockProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getSoilFlags(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setSoilFlags(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.addSoilFlags(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.generateStateData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setTextureOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getTexturePath(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setModelOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getModelPath(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.getRootsOverlayModelLocation(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.defaultSoilBlockTags(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.isOnlyIfLoaded(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.setOnlyIfLoaded(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.addGeneratedBlockTags(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilProperties.toString(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
    }

    /// `SpeciesBlockEntity` (com/dtteam/dynamictrees/block/soil/SpeciesBlockEntity.java)
    pub mod species_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_2586`. Its static parts are below.
        // SpeciesBlockEntity.SpeciesBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // SpeciesBlockEntity.getSpecies(0): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // SpeciesBlockEntity.setSpecies(1): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // SpeciesBlockEntity.method_11014(2): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // SpeciesBlockEntity.method_11007(2): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // SpeciesBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
        // SpeciesBlockEntity.method_16887(1): not translated, its class isn't translated (it extends Minecraft's `class_2586`).
    }

    /// `SpreadableSoilProperties` (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java)
    pub mod spreadable_soil_properties {
        use super::*;
        // The class itself isn't translated: it extends `SoilProperties`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // SpreadableSoilProperties.setRequiredLight(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // SpreadableSoilProperties.setSpreadItem(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // SpreadableSoilProperties.SpreadableSoilProperties(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // SpreadableSoilProperties.createBlock(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // SpreadableSoilProperties.addSpreadableSoils(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // SpreadableSoilProperties.addSpreadableSoils(1): not translated, its class isn't translated (it extends `SoilProperties`).
    }

    /// `SpreadableSoilProperties.SpreadableSoilBlock` (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java)
    pub mod spreadable_soil_properties_spreadable_soil_block {
        use super::*;
        // The class itself isn't translated: it extends `SoilBlock`. Its static parts are below.
        // SpreadableSoilProperties.SpreadableSoilBlock.SpreadableSoilBlock(2): not translated, its class isn't translated (it extends `SoilBlock`).
        // SpreadableSoilProperties.SpreadableSoilBlock.getSoilProperties(0): not translated, its class isn't translated (it extends `SoilBlock`).
        // SpreadableSoilProperties.SpreadableSoilBlock.getRootyBlock(1): not translated, its class isn't translated (it extends `SoilBlock`).
        // SpreadableSoilProperties.SpreadableSoilBlock.method_55765(7): not translated, its class isn't translated (it extends `SoilBlock`).
        // SpreadableSoilProperties.SpreadableSoilBlock.method_9514(4): not translated, its class isn't translated (it extends `SoilBlock`).
    }

    /// `WaterSoilProperties` (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java)
    pub mod water_soil_properties {
        use super::*;
        // The class itself isn't translated: it extends `SoilProperties`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // WaterSoilProperties.WaterSoilProperties(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // WaterSoilProperties.createBlock(1): not translated, its class isn't translated (it extends `SoilProperties`).
        // WaterSoilProperties.getDefaultBlockProperties(0): not translated, its class isn't translated (it extends `SoilProperties`).
    }

    /// `WaterSoilProperties.SoilWaterBlock` (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java)
    pub mod water_soil_properties_soil_water_block {
        use super::*;
        // The class itself isn't translated: it extends `SoilBlock`. Its static parts are below.
        // WATER_ROOTS_AABB: not translated, it uses Minecraft's `class_238`.
        // WATERLOGGED: not translated, it uses Minecraft's `class_2746`.
        // WaterSoilProperties.SoilWaterBlock.SoilWaterBlock(2): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.method_9515(1): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.getRadiusForConnection(6): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.method_9574(3): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.getHardness(3): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.method_9530(4): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.method_25959(3): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.method_9616(2): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.method_9545(1): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.method_9559(6): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.getDecayBlockState(3): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.getColorFromBark(0): not translated, its class isn't translated (it extends `SoilBlock`).
        // WaterSoilProperties.SoilWaterBlock.fallWithTree(4): not translated, its class isn't translated (it extends `SoilBlock`).
    }

    /// `BlockColorMultipliers` (com/dtteam/dynamictrees/client/BlockColorMultipliers.java)
    pub mod block_color_multipliers {
        use super::*;
        // colorBase: not translated, it uses Minecraft's `class_2960`.
        #[derive(Clone, Debug)]
        pub struct BlockColorMultipliers {
        }
        impl BlockColorMultipliers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BlockColorMultipliers.register(2): not translated, it's overloaded with the same number of parameters.
        // BlockColorMultipliers.register(2): not translated, it's overloaded with the same number of parameters.
        // BlockColorMultipliers.find(1): not translated, it uses Minecraft's `class_322`.
        // BlockColorMultipliers.find(1): not translated, it uses Minecraft's `class_322`.
        // BlockColorMultipliers.cleanUp(0): not translated, it uses BlockColorMultipliers.colorBase, which isn't translated (it uses Minecraft's `class_2960`).
    }

    /// `ParticleHelper` (com/dtteam/dynamictrees/client/ParticleHelper.java)
    pub mod particle_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ParticleHelper {
        }
        impl ParticleHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ParticleHelper.addDustParticle(11): not translated, it uses Minecraft's `class_1937`.
        // ParticleHelper.spawnParticles(5): not translated, it uses Minecraft's `class_1937`.
        // ParticleHelper.spawnParticles(7): not translated, it uses Minecraft's `class_1936`.
        // ParticleHelper.getFoliageColor(4): not translated, it uses LeavesProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // ParticleHelper.crushLeavesBlock(4): not translated, it uses Minecraft's `class_1937`.
    }

    /// `RetexturedBakedQuad` (com/dtteam/dynamictrees/client/RetexturedBakedQuad.java)
    pub mod retextured_baked_quad {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_777`. Its static parts are below.
        // RetexturedBakedQuad.RetexturedBakedQuad(2): not translated, its class isn't translated (it extends Minecraft's `class_777`).
        // RetexturedBakedQuad.remapQuad(0): not translated, its class isn't translated (it extends Minecraft's `class_777`).
        // RetexturedBakedQuad.method_35788(0): not translated, its class isn't translated (it extends Minecraft's `class_777`).
        // RetexturedBakedQuad.getUnInterpolatedU(2): not translated, it uses Minecraft's `class_1058`.
        // RetexturedBakedQuad.getUnInterpolatedV(2): not translated, it uses Minecraft's `class_1058`.
    }

    /// `SoundInstanceHandler` (com/dtteam/dynamictrees/client/SoundInstanceHandler.java)
    pub mod sound_instance_handler {
        use super::*;
        // instances: not translated, it uses Minecraft's `class_1113`.
        #[derive(Clone, Debug)]
        pub struct SoundInstanceHandler {
        }
        impl SoundInstanceHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SoundInstanceHandler.playSoundInstance(4): not translated, it uses Minecraft's `class_3414`.
        // SoundInstanceHandler.playSoundInstance(6): not translated, it uses Minecraft's `class_3414`.
        // SoundInstanceHandler.stopSoundInstance(1): not translated, it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`).
    }

    /// `TextureHelper` (com/dtteam/dynamictrees/client/TextureHelper.java)
    pub mod texture_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TextureHelper {
        }
        impl TextureHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn compose_4(r: i32, g: i32, b: i32, a: i32) -> i32 {
            let mut rgb: i32 = a;
            rgb = (rgb << 8) + r;
            rgb = (rgb << 8) + g;
            return (rgb << 8) + b;
        }
        pub fn compose_1(c: &[i32]) -> i32 {
            return if (c.len() as i32) >= 4 { texture_helper::compose_4(c[0], c[1], c[2], c[3]) } else { 0 };
        }
        pub fn decompose(c: i32) -> Vec<i32> {
            return vec![texture_helper::red(c), texture_helper::green(c), texture_helper::blue(c), texture_helper::alpha(c)];
        }
        pub fn alpha(c: i32) -> i32 {
            return (c >> 24) & 0xFF;
        }
        pub fn blue(c: i32) -> i32 {
            return (c >> 16) & 0xFF;
        }
        pub fn green(c: i32) -> i32 {
            return (c >> 8) & 0xFF;
        }
        pub fn red(c: i32) -> i32 {
            return c & 0xFF;
        }
        pub fn avg_colors(pixels: Option<Vec<i32>>) -> i32 {
            let mut r_accum: i64 = 0i64;
            let mut g_accum: i64 = 0i64;
            let mut b_accum: i64 = 0i64;
            let mut count: i32 = 0;
            for i in 0..((pixels.as_ref().expect("null")).len() as i32) {
                let alpha: i32 = texture_helper::alpha((pixels.as_ref().expect("null"))[i as usize]);
                if alpha >= 128 {
                    r_accum += ((texture_helper::red((pixels.as_ref().expect("null"))[i as usize])) as i64);
                    g_accum += ((texture_helper::green((pixels.as_ref().expect("null"))[i as usize])) as i64);
                    b_accum += ((texture_helper::blue((pixels.as_ref().expect("null"))[i as usize])) as i64);
                    count += 1;
                }
            }
            count = (1i32).max(count);
            let r: i32 = ((r_accum / (count as i64)) as i32);
            let g: i32 = ((g_accum / (count as i64)) as i32);
            let b: i32 = ((b_accum / (count as i64)) as i32);
            return texture_helper::compose_4(r, g, b, 255);
        }
    }

    /// `TextureHelper.PixelBuffer` (com/dtteam/dynamictrees/client/TextureHelper.java)
    pub mod texture_helper_pixel_buffer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PixelBuffer {
            pub pixels: Option<Vec<i32>>,
            pub w: i32,
            pub h: i32,
        }
        impl PixelBuffer {
            // TextureHelper.PixelBuffer.PixelBuffer(2): not translated, its constructors are overloaded with the same number of parameters.
            // TextureHelper.PixelBuffer.PixelBuffer(1): not translated, its constructors are overloaded with the same number of parameters.
            // TextureHelper.PixelBuffer.PixelBuffer(1): not translated, its constructors are overloaded with the same number of parameters.
            // TextureHelper.PixelBuffer.PixelBuffer(2): not translated, its constructors are overloaded with the same number of parameters.
            // TextureHelper.PixelBuffer.PixelBuffer(1): not translated, its constructors are overloaded with the same number of parameters.
            // TextureHelper.PixelBuffer.toNativeImage(0): not translated, it uses Minecraft's `class_1011`.
            pub fn calc_pos(&self, off_x: i32, off_y: i32) -> i32 {
                return (off_y * self.w) + off_x;
            }
            pub fn get_pixel(&self, off_x: i32, off_y: i32) -> i32 {
                return if (((off_x >= 0) && (off_x < self.w)) && (off_y >= 0)) && (off_y < self.h) { (self.pixels.as_ref().expect("null"))[(self.calc_pos(off_x, off_y)) as usize] } else { 0 };
            }
            pub fn set_pixel(&mut self, off_x: i32, off_y: i32, pixel: i32) {
                if (((off_x >= 0) && (off_x < self.w)) && (off_y >= 0)) && (off_y < self.h) {
                    (self.pixels.as_mut().expect("null"))[(self.calc_pos(off_x, off_y)) as usize] = pixel;
                }
            }
            pub fn blit_3(&self, dst: &mut texture_helper_pixel_buffer::PixelBuffer, off_x: i32, off_y: i32) {
                self.blit_4(&mut *dst, off_x, off_y, 0);
            }
            pub fn blit_4(&self, dst: &mut texture_helper_pixel_buffer::PixelBuffer, off_x: i32, off_y: i32, rot_cw90: i32) {
                match rot_cw90 & 3 {
                    0 => {
                        for y in 0..self.h {
                            for x in 0..self.w {
                                dst.set_pixel(x + off_x, y + off_y, self.get_pixel(x, y));
                            }
                        }
                        return;
                    }
                    1 => {
                        for y in 0..self.h {
                            for x in 0..self.w {
                                let dest_x: i32 = (self.h - y) - 1;
                                dst.set_pixel(dest_x + off_x, x + off_y, self.get_pixel(x, y));
                            }
                        }
                        return;
                    }
                    2 => {
                        for y in 0..self.h {
                            for x in 0..self.w {
                                let dest_x: i32 = (self.w - x) - 1;
                                let dest_y: i32 = (self.h - y) - 1;
                                dst.set_pixel(dest_x + off_x, dest_y + off_y, self.get_pixel(x, y));
                            }
                        }
                        return;
                    }
                    3 => {
                        for y in 0..self.h {
                            for x in 0..self.w {
                                let dest_y: i32 = (self.w - x) - 1;
                                dst.set_pixel(y + off_x, dest_y + off_y, self.get_pixel(x, y));
                            }
                        }
                    }
                    _ => {}
                }
            }
            pub fn average_color(&self) -> i32 {
                return texture_helper::avg_colors(self.pixels.clone());
            }
            pub fn gray_scale(&mut self) {
                for i in 0..((self.pixels.as_ref().expect("null")).len() as i32) {
                    let a: i32 = texture_helper::alpha((self.pixels.as_ref().expect("null"))[i as usize]);
                    let r: i32 = texture_helper::red((self.pixels.as_ref().expect("null"))[i as usize]);
                    let g: i32 = texture_helper::green((self.pixels.as_ref().expect("null"))[i as usize]);
                    let b: i32 = texture_helper::blue((self.pixels.as_ref().expect("null"))[i as usize]);
                    let gray: i32 = (((r * 30) + (g * 59)) + (b * 11)) / 100;
                    (self.pixels.as_mut().expect("null"))[i as usize] = texture_helper::compose_4(gray, gray, gray, a);
                }
            }
            // TextureHelper.PixelBuffer.fill(1): not translated, it calls `Arrays` (java.util).fill.
        }
    }

    /// `ThickBranchRingsSource` (com/dtteam/dynamictrees/client/ThickBranchRingsSource.java)
    pub mod thick_branch_rings_source {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_7948`. Its static parts are below.
        // ID: not translated, it uses Minecraft's `class_2960`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `class_7951`.
        // ThickBranchRingsSource.ThickBranchRingsSource(1): not translated, its class isn't translated (it implements Minecraft's `class_7948`).
        // ThickBranchRingsSource.method_47673(2): not translated, its class isn't translated (it implements Minecraft's `class_7948`).
        // ThickBranchRingsSource.method_47672(0): not translated, its class isn't translated (it implements Minecraft's `class_7948`).
        // ThickBranchRingsSource.setType(1): not translated, it uses Minecraft's `class_7951`.
    }

    /// `ThickBranchRingsSprite` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java)
    pub mod thick_branch_rings_sprite {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_7764`. Its static parts are below.
        pub const RESOLUTION: i32 = 16;
        pub const LAYERS: i32 = 3;
        pub static CORNERS_XY: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![0, 1, 1, 0], vec![0, 0, 1, 1]]);
        // ThickBranchRingsSprite.ThickBranchRingsSprite(2): not translated, its class isn't translated (it extends Minecraft's `class_7764`).
        // ThickBranchRingsSprite.getFrameSize(1): not translated, it uses Minecraft's `class_7771`.
        pub fn center_corner() -> i32 {
            return 16;
        }
        // ThickBranchRingsSprite.processImage(1): not translated, it uses Minecraft's `class_1011`.
        // ThickBranchRingsSprite.createMajorTexture(1): not translated, it creates a PixelBuffer with a constructor that isn't translated.
        // ThickBranchRingsSprite.compileRingCornersAndEdges(4): not translated, it creates a PixelBuffer with a constructor that isn't translated.
        // ThickBranchRingsSprite.fillRings(3): not translated, unknown name `class_2353`.
        pub fn fill_corners(major_buffer: &mut texture_helper_pixel_buffer::PixelBuffer, corners: &[texture_helper_pixel_buffer::PixelBuffer], scale: i32) {
            for nesting in 1..=3 {
                for corner in 0..4 {
                    let corner_pixels = &corners[((corner + nesting) & 3) as usize];
                    let c_x: i32 = (2 * (((*thick_branch_rings_sprite::CORNERS_XY)[0])[corner as usize])) - 1;
                    let c_y: i32 = (2 * (((*thick_branch_rings_sprite::CORNERS_XY)[1])[corner as usize])) - 1;
                    let off_x: i32 = ((c_x * 6) * nesting) + (c_x * 5);
                    let off_y: i32 = ((c_y * 6) * nesting) + (c_y * 5);
                    let real_x: i32 = (thick_branch_rings_sprite::center_corner() + 5) + off_x;
                    let real_y: i32 = (thick_branch_rings_sprite::center_corner() + 5) + off_y;
                    corner_pixels.blit_4(&mut *major_buffer, real_x * scale, real_y * scale, corner);
                }
            }
        }
        pub fn fill_center(major_buffer: &mut texture_helper_pixel_buffer::PixelBuffer, antecedent_buffer: &texture_helper_pixel_buffer::PixelBuffer, scale: i32) {
            antecedent_buffer.blit_3(&mut *major_buffer, thick_branch_rings_sprite::center_corner() * scale, thick_branch_rings_sprite::center_corner() * scale);
        }
        // ThickBranchRingsSprite.compileBarkCornersAndEdges(4): not translated, it creates a PixelBuffer with a constructor that isn't translated.
        // ThickBranchRingsSprite.fillBarkBorder(4): not translated, `++` inside an expression.
        // ThickBranchRingsSprite.createBarklessAntecedent(1): not translated, it creates a PixelBuffer with a constructor that isn't translated.
    }

    /// `Tooltips` (com/dtteam/dynamictrees/client/Tooltips.java)
    pub mod tooltips {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Tooltips {
        }
        impl Tooltips {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Tooltips.applySeasonalTooltips(3): not translated, it uses Minecraft's `class_2561`.
    }

    /// `CommandConstants` (com/dtteam/dynamictrees/command/CommandConstants.java)
    pub mod command_constants {
        use super::*;
        pub const COMMAND: &str = "dt";
        pub const COMMAND_ALIAS: &str = "dynamictrees";
        pub const GET_TREE: &str = "gettree";
        pub const GET_ROOTS: &str = "getroots";
        pub const GROW_PULSE: &str = "growpulse";
        pub const KILL_TREE: &str = "killtree";
        pub const SPECIES_LIST: &str = "specieslist";
        pub const FERTILITY: &str = "fertility";
        pub const SET_TREE: &str = "settree";
        pub const ROTATE_JO_CODE: &str = "rotatejocode";
        pub const CREATE_STAFF: &str = "createstaff";
        pub const SET_COORD_XOR: &str = "setcoordxor";
        pub const CREATE_TRANSFORM_POTION: &str = "createtransformpotion";
        pub const TRANSFORM: &str = "transform";
        pub const CLEAR_ORPHANED: &str = "clearorphaned";
        pub const PURGE_TREES: &str = "purgetrees";
        pub const LOCATION: &str = "location";
        pub const JO_CODE: &str = "jo_code";
        pub const ROOTS_JO_CODE: &str = "roots_jo_code";
        pub const TURNS: &str = "turns";
        pub const SPECIES: &str = "species";
        pub const RAW: &str = "raw";
        pub const DEFAULT_JO_CODE: &str = "JP";
        pub const DEFAULT_ROOTS_JO_CODE: &str = "";
        pub const DEFAULT_TURNS: i32 = 0;
        // TURNS_SUGGESTIONS: not translated, it uses Minecraft's `SuggestionProvider`.
        // FERTILITY_SUGGESTIONS: not translated, it uses Minecraft's `SuggestionProvider`.
        #[derive(Clone, Debug)]
        pub struct CommandConstants {
        }
        impl CommandConstants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `CommandHelper` (com/dtteam/dynamictrees/command/CommandHelper.java)
    pub mod command_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommandHelper {
        }
        impl CommandHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommandHelper.posComponent(1): not translated, it uses Minecraft's `class_2561`.
        // CommandHelper.posComponent(2): not translated, it uses Minecraft's `class_2561`.
        // CommandHelper.colour(2): not translated, it uses Minecraft's `class_2561`.
    }

    /// `DTCommand` (com/dtteam/dynamictrees/command/DTCommand.java)
    pub mod dt_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DTCommand {
            pub sub_commands: Vec<sub_command::SubCommand>,
        }
        impl DTCommand {
            // DTCommand.DTCommand(0): not translated, it creates a GetTreeCommand, which isn't translated (it extends `SubCommand`).
            // DTCommand.registerDTCommand(1): not translated, it uses Minecraft's `CommandDispatcher`.
        }
    }

    /// `HexColorArgument` (com/dtteam/dynamictrees/command/HexColorArgument.java)
    pub mod hex_color_argument {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ArgumentType`. Its static parts are below.
        // COLOR_INVALID: not translated, it uses Minecraft's `DynamicCommandExceptionType`.
        // HexColorArgument.hex(0): not translated, it uses HexColorArgument, which isn't translated (it implements Minecraft's `ArgumentType`).
        // HexColorArgument.getHexCode(2): not translated, it uses Minecraft's `CommandContext`.
        // HexColorArgument.parse(1): not translated, its class isn't translated (it implements Minecraft's `ArgumentType`).
        // HexColorArgument.listSuggestions(2): not translated, it's generic.
    }

    /// `ChunkBasedCommand` (com/dtteam/dynamictrees/command/subcommand/ChunkBasedCommand.java)
    pub mod chunk_based_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        pub const RADIUS: &str = "radius";
        pub const DEFAULT_RADIUS: i32 = 1;
        // ChunkBasedCommand.register(0): not translated, its class isn't translated (it extends `SubCommand`).
        // ChunkBasedCommand.getChunkPos(1): not translated, its class isn't translated (it extends `SubCommand`).
        // ChunkBasedCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // ChunkBasedCommand.processChunk(4): not translated, it has no body.
    }

    /// `ClearOrphanedCommand` (com/dtteam/dynamictrees/command/subcommand/ClearOrphanedCommand.java)
    pub mod clear_orphaned_command {
        use super::*;
        // The class itself isn't translated: it extends `ChunkBasedCommand`. Its static parts are below.
        // ClearOrphanedCommand.getName(0): not translated, its class isn't translated (it extends `ChunkBasedCommand`).
        // ClearOrphanedCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `ChunkBasedCommand`).
        // ClearOrphanedCommand.processChunk(4): not translated, its class isn't translated (it extends `ChunkBasedCommand`).
    }

    /// `CreateStaffCommand` (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java)
    pub mod create_staff_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        pub const COLOR: &str = "color";
        pub const READ_ONLY: &str = "readOnly";
        pub const MAX_USES: &str = "maxUses";
        pub const DEFAULT_COLOUR: i32 = 65535;
        pub const DEFAULT_READ_ONLY: bool = false;
        pub const DEFAULT_MAX_USES: i32 = 64;
        // CreateStaffCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // CreateStaffCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // CreateStaffCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // CreateStaffCommand.spawnStaff(7): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `FertilityCommand` (com/dtteam/dynamictrees/command/subcommand/FertilityCommand.java)
    pub mod fertility_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        pub const FERTILITY: &str = "fertility";
        // FertilityCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // FertilityCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // FertilityCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // FertilityCommand.getFertility(3): not translated, its class isn't translated (it extends `SubCommand`).
        // FertilityCommand.setFertility(3): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `GetRootsCommand` (com/dtteam/dynamictrees/command/subcommand/GetRootsCommand.java)
    pub mod get_roots_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        pub const CODE_RAW: &str = "code_raw";
        // GetRootsCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GetRootsCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GetRootsCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GetRootsCommand.getRoots(3): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `GetTreeCommand` (com/dtteam/dynamictrees/command/subcommand/GetTreeCommand.java)
    pub mod get_tree_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        pub const CODE_RAW: &str = "code_raw";
        // GetTreeCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GetTreeCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GetTreeCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GetTreeCommand.getTree(3): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `GrowPulseCommand` (com/dtteam/dynamictrees/command/subcommand/GrowPulseCommand.java)
    pub mod grow_pulse_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        pub const NUMBER: &str = "number";
        pub static NUMBER_SUGGESTIONS: LazyLock<Vec<String>> = LazyLock::new(|| vec![1, 4, 8, 16, 32, 64].into_iter().map(|p_a: i32| format!("{}", p_a)).collect::<Vec<_>>());
        // GrowPulseCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GrowPulseCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GrowPulseCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // GrowPulseCommand.sendGrowPulse(3): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `KillTreeCommand` (com/dtteam/dynamictrees/command/subcommand/KillTreeCommand.java)
    pub mod kill_tree_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        // KillTreeCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // KillTreeCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // KillTreeCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // KillTreeCommand.killTree(2): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `PurgeTreesCommand` (com/dtteam/dynamictrees/command/subcommand/PurgeTreesCommand.java)
    pub mod purge_trees_command {
        use super::*;
        // The class itself isn't translated: it extends `ChunkBasedCommand`. Its static parts are below.
        // PurgeTreesCommand.getName(0): not translated, its class isn't translated (it extends `ChunkBasedCommand`).
        // PurgeTreesCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `ChunkBasedCommand`).
        // PurgeTreesCommand.processChunk(4): not translated, its class isn't translated (it extends `ChunkBasedCommand`).
    }

    /// `RegistryCommand` (com/dtteam/dynamictrees/command/subcommand/RegistryCommand.java)
    pub mod registry_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        // RegistryCommand.RegistryCommand(0): not translated, its class isn't translated (it extends `SubCommand`).
        // RegistryCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // RegistryCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // RegistryCommand.registerArguments(0): not translated, its class isn't translated (it extends `SubCommand`).
        // RegistryCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `RegistrySubCommand` (com/dtteam/dynamictrees/command/subcommand/RegistrySubCommand.java)
    pub mod registry_sub_command {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegistrySubCommand.RegistrySubCommand(1): not translated, its class isn't translated (it's generic).
        // RegistrySubCommand.getName(0): not translated, its class isn't translated (it's generic).
        // RegistrySubCommand.getPermissionLevel(0): not translated, its class isn't translated (it's generic).
        // RegistrySubCommand.registerArgument(0): not translated, its class isn't translated (it's generic).
        // RegistrySubCommand.listEntries(2): not translated, its class isn't translated (it's generic).
    }

    /// `RotateJoCodeCommand` (com/dtteam/dynamictrees/command/subcommand/RotateJoCodeCommand.java)
    pub mod rotate_jo_code_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        // RotateJoCodeCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // RotateJoCodeCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // RotateJoCodeCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // RotateJoCodeCommand.rotateJoCode(3): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `SetCoordXorCommand` (com/dtteam/dynamictrees/command/subcommand/SetCoordXorCommand.java)
    pub mod set_coord_xor_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        pub const XOR: &str = "xor";
        // SetCoordXorCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // SetCoordXorCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // SetCoordXorCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // SetCoordXorCommand.setXor(2): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `SetTreeCommand` (com/dtteam/dynamictrees/command/subcommand/SetTreeCommand.java)
    pub mod set_tree_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        pub const DEFAULT_FERTILITY: i32 = 0;
        // SetTreeCommand.getName(0): not translated, its class isn't translated (it extends `SubCommand`).
        // SetTreeCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // SetTreeCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
        // SetTreeCommand.setTree(6): not translated, its class isn't translated (it extends `SubCommand`).
        // SetTreeCommand.setTreeWithRoots(7): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `SimpleSubCommand` (com/dtteam/dynamictrees/command/subcommand/SimpleSubCommand.java)
    pub mod simple_sub_command {
        use super::*;
        // The class itself isn't translated: it extends `SubCommand`. Its static parts are below.
        // SimpleSubCommand.register(0): not translated, its class isn't translated (it extends `SubCommand`).
        // SimpleSubCommand.execute(1): not translated, it has no body.
        // SimpleSubCommand.getPermissionLevel(0): not translated, its class isn't translated (it extends `SubCommand`).
        // SimpleSubCommand.registerArgument(0): not translated, its class isn't translated (it extends `SubCommand`).
    }

    /// `SubCommand` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java)
    pub mod sub_command {
        use super::*;
        // NO_TREE_FOUND: not translated, it uses Minecraft's `DynamicCommandExceptionType`.
        // SPECIES_UNKNOWN: not translated, it uses Minecraft's `DynamicCommandExceptionType`.
        #[derive(Clone, Debug)]
        pub struct SubCommand {
        }
        impl SubCommand {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // SubCommand.getName(0): not translated, it has no body.
            // SubCommand.getPermissionLevel(0): not translated, it has no body.
            // SubCommand.register(0): not translated, it uses Minecraft's `ArgumentBuilder`.
            // SubCommand.registerArguments(0): not translated, it uses Minecraft's `ArgumentBuilder`.
            // SubCommand.registerArgument(0): not translated, it has no body.
        }
        // SubCommand.getVector3i(1): not translated, it uses Minecraft's `class_2382`.
        // SubCommand.executesSuccess(1): not translated, it uses ThrowableRunnable, which isn't translated (it's an interface).
        // SubCommand.executesSuccess(2): not translated, it uses Minecraft's `CommandContext`.
        // SubCommand.booleanArgument(1): not translated, it uses Minecraft's `RequiredArgumentBuilder`.
        // SubCommand.booleanArgument(2): needs the game, so it's in game_side.rs.
        // SubCommand.intArgument(1): not translated, it uses Minecraft's `RequiredArgumentBuilder`.
        // SubCommand.intArgument(2): needs the game, so it's in game_side.rs.
        // SubCommand.stringArgument(1): not translated, it uses Minecraft's `RequiredArgumentBuilder`.
        // SubCommand.stringArgument(2): not translated, it uses Minecraft's `RequiredArgumentBuilder`.
        // SubCommand.stringArgument(2): not translated, it uses Minecraft's `RequiredArgumentBuilder`.
        // SubCommand.blockPosArgument(0): not translated, it uses Minecraft's `RequiredArgumentBuilder`.
        // SubCommand.blockPosArgument(1): not translated, it uses Minecraft's `class_2338`.
        // SubCommand.rootPosArgument(1): not translated, it uses Minecraft's `class_2338`.
        // SubCommand.speciesArgument(0): not translated, it uses Minecraft's `RequiredArgumentBuilder`.
        // SubCommand.speciesArgument(1): not translated, it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SubCommand.resourceLocationArgument(2): not translated, it uses Minecraft's `RequiredArgumentBuilder`.
        // SubCommand.aqua(1): not translated, it uses Minecraft's `class_2561`.
        // SubCommand.darkRed(1): not translated, it uses Minecraft's `class_2561`.
        // SubCommand.sendSuccess(2): not translated, it uses Minecraft's `class_2168`.
        // SubCommand.sendSuccessAndLog(2): not translated, it uses Minecraft's `class_2168`.
        // SubCommand.sendFailure(2): not translated, it uses Minecraft's `class_2168`.
    }

    /// `ContinuityWrappedModelHandler` (com/dtteam/dynamictrees/compat/continuity/ContinuityWrappedModelHandler.java)
    pub mod continuity_wrapped_model_handler {
        use super::*;
        // The class itself isn't translated: it extends `WrappedModelHandler`. Its static parts are below.
        // ContinuityWrappedModelHandler.unwrapBranchModel(1): not translated, its class isn't translated (it extends `WrappedModelHandler`).
    }

    /// `WrappedModelHandler` (com/dtteam/dynamictrees/compat/continuity/WrappedModelHandler.java)
    pub mod wrapped_model_handler {
        use super::*;
        pub static INSTANCE: LazyLock<JCell<Option<wrapped_model_handler::WrappedModelHandler>>> = LazyLock::new(|| JCell::new(None));
        #[derive(Clone, Debug)]
        pub struct WrappedModelHandler {
        }
        impl WrappedModelHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // WrappedModelHandler.unwrapBranchModel(1): not translated, it uses BasicBranchBlockBakedModel, which isn't translated (it implements Minecraft's `class_1087`).
        }
        // WrappedModelHandler.getInstance(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `DTJeiPlugin` (com/dtteam/dynamictrees/compat/DTJeiPlugin.java)
    pub mod dt_jei_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IModPlugin` (mezz.jei.api). Its static parts are below.
        // DTJeiPlugin.getPluginUid(0): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // DTJeiPlugin.registerItemSubtypes(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // DTJeiPlugin.registerRecipes(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // DTJeiPlugin.makeJeiBrewingRecipe(4): not translated, it uses `IJeiBrewingRecipe` (mezz.jei.api.recipe.vanilla).
    }

    /// `SereneSeasonsSeasonProvider` (com/dtteam/dynamictrees/compat/SereneSeasonsSeasonProvider.java)
    pub mod serene_seasons_season_provider {
        use super::*;
        // The class itself isn't translated: it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season). Its static parts are below.
        // SereneSeasonsSeasonProvider.getSeasonValue(2): not translated, its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)).
        // SereneSeasonsSeasonProvider.updateTick(2): not translated, its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)).
        // SereneSeasonsSeasonProvider.shouldSnowMelt(2): not translated, its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)).
        // SereneSeasonsSeasonProvider.registerSereneSeasonsProvider(0): not translated, it uses SeasonManager, which isn't translated (it's an interface).
    }

    /// `WailaHelper` (com/dtteam/dynamictrees/compat/WailaHelper.java)
    pub mod waila_helper {
        use super::*;
        pub static INVALID: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct WailaHelper {
        }
        impl WailaHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn invalidate_waila_position() {
            waila_helper::INVALID.set(true);
        }
    }

    /// `WailaBranchHandler` (com/dtteam/dynamictrees/compat/waila/WailaBranchHandler.java)
    pub mod waila_branch_handler {
        use super::*;
        // The class itself isn't translated: it implements `IBlockComponentProvider` (snownee.jade.api). Its static parts are below.
        // ID: not translated, it uses Minecraft's `class_2960`.
        // WailaBranchHandler.appendTooltip(3): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // WailaBranchHandler.getTreeVolume(3): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // WailaBranchHandler.getWailaSpecies(2): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // WailaBranchHandler.getElement(1): not translated, it uses `IElement` (snownee.jade.api.ui).
        // WailaBranchHandler.getUid(0): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
    }

    /// `WailaCompat` (com/dtteam/dynamictrees/compat/waila/WailaCompat.java)
    pub mod waila_compat {
        use super::*;
        // The class itself isn't translated: it implements `IWailaPlugin` (snownee.jade.api). Its static parts are below.
        // WailaCompat.registerClient(1): not translated, its class isn't translated (it implements `IWailaPlugin` (snownee.jade.api)).
    }

    /// `WailaFruitHandler` (com/dtteam/dynamictrees/compat/waila/WailaFruitHandler.java)
    pub mod waila_fruit_handler {
        use super::*;
        // The class itself isn't translated: it implements `IBlockComponentProvider` (snownee.jade.api). Its static parts are below.
        // FRUIT_UID: not translated, it uses Minecraft's `class_2960`.
        // WailaFruitHandler.appendTooltip(3): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // WailaFruitHandler.getUid(0): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
    }

    /// `WailaPodHandler` (com/dtteam/dynamictrees/compat/waila/WailaPodHandler.java)
    pub mod waila_pod_handler {
        use super::*;
        // The class itself isn't translated: it implements `IBlockComponentProvider` (snownee.jade.api). Its static parts are below.
        // POD_UID: not translated, it uses Minecraft's `class_2960`.
        // COCOA: not translated, it uses Minecraft's `class_2960`.
        // WailaPodHandler.appendTooltip(3): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // WailaPodHandler.getUid(0): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
    }

    /// `WailaRootyHandler` (com/dtteam/dynamictrees/compat/waila/WailaRootyHandler.java)
    pub mod waila_rooty_handler {
        use super::*;
        // The class itself isn't translated: it implements `IBlockComponentProvider` (snownee.jade.api). Its static parts are below.
        // ROOTY_UID: not translated, it uses Minecraft's `class_2960`.
        // WailaRootyHandler.appendTooltip(3): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // WailaRootyHandler.getUid(0): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
    }

    /// `WailaRootyWaterHandler` (com/dtteam/dynamictrees/compat/waila/WailaRootyWaterHandler.java)
    pub mod waila_rooty_water_handler {
        use super::*;
        // The class itself isn't translated: it implements `IBlockComponentProvider` (snownee.jade.api). Its static parts are below.
        // ROOTY_WATER_UID: not translated, it uses Minecraft's `class_2960`.
        // WailaRootyWaterHandler.appendTooltip(3): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // WailaRootyWaterHandler.getUid(0): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
    }

    /// `DTConfigs` (com/dtteam/dynamictrees/config/DTConfigs.java)
    pub mod dt_configs {
        use super::*;
        // The class itself isn't translated: it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // SERVER_CONFIG: not translated, it uses `ModConfigSpec` (net.neoforged.neoforge.common).
        // SERVER: not translated, it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // COMMON_CONFIG: not translated, it uses `ModConfigSpec` (net.neoforged.neoforge.common).
        // COMMON: not translated, it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // CLIENT_CONFIG: not translated, it uses `ModConfigSpec` (net.neoforged.neoforge.common).
        // CLIENT: not translated, it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // DTConfigs.DTConfigs(0): not translated, its class isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // DTConfigs.buildServerConfig(1): not translated, it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // DTConfigs.buildCommonConfig(1): not translated, it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // DTConfigs.buildClientConfig(1): not translated, it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `DTDataProvider` (com/dtteam/dynamictrees/data/DTDataProvider.java)
    pub mod dt_data_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // DTDataProvider.block(1): not translated, its class isn't translated (it's an interface).
        // DTDataProvider.item(1): not translated, its class isn't translated (it's an interface).
    }

    /// `DTDataProvider.BlockState` (com/dtteam/dynamictrees/data/DTDataProvider.java)
    pub mod dt_data_provider_block_state {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `DTDataProvider.ItemModel` (com/dtteam/dynamictrees/data/DTDataProvider.java)
    pub mod dt_data_provider_item_model {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `DTDataProvider.Language` (com/dtteam/dynamictrees/data/DTDataProvider.java)
    pub mod dt_data_provider_language {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `DTLootTableBuilder` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java)
    pub mod dt_loot_table_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DTLootTableBuilder {
        }
        impl DTLootTableBuilder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DTLootTableBuilder.hasSilkTouch(1): not translated, it uses Minecraft's `class_210`.
        // DTLootTableBuilder.hasNoShearsOrSilkTouch(1): not translated, it uses Minecraft's `class_210`.
        // DTLootTableBuilder.hasShearsOrSilkTouch(1): not translated, it uses Minecraft's `class_210`.
        // DTLootTableBuilder.createSelfDropDispatchTable(3): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createLeavesBlockDrops(4): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createPalmLeavesBlockDrops(3): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createWartBlockDrops(2): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createLeavesDrops(3): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createLeavesDrops(4): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createPalmLeavesDrops(3): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createWartDrops(2): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createVoluntaryDrops(2): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createBranchDrops(3): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createFruitPodDrops(6): not translated, it uses Minecraft's `class_53`.
        // DTLootTableBuilder.createFruitPodDrops(7): not translated, it uses Minecraft's `class_53`.
    }

    /// `Generator` (com/dtteam/dynamictrees/data/Generator.java)
    pub mod generator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Generator.generate(2): not translated, its class isn't translated (it's an interface).
        // Generator.generate(3): not translated, it has no body.
        // Generator.gatherDependencies(1): not translated, it has no body.
        // Generator.verifyInput(1): not translated, its class isn't translated (it's an interface).
        // Generator.verifyDependencies(1): not translated, its class isn't translated (it's an interface).
    }

    /// `Generator.Dependencies` (com/dtteam/dynamictrees/data/Generator.java)
    pub mod generator_dependencies {
        use super::*;
        // The class itself isn't translated: it uses DependencyKey, which isn't translated (it's generic). Its static parts are below.
        // Generator.Dependencies.appendValue(2): not translated, it's generic.
        // Generator.Dependencies.append(2): not translated, it's generic.
        // Generator.Dependencies.get(1): not translated, it's generic.
        // Generator.Dependencies.getOptional(1): not translated, it's generic.
        // Generator.Dependencies.allRequiredPresent(0): not translated, its class isn't translated (it uses DependencyKey, which isn't translated (it's generic)).
    }

    /// `Generator.DependencyKey` (com/dtteam/dynamictrees/data/Generator.java)
    pub mod generator_dependency_key {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Generator.DependencyKey.DependencyKey(1): not translated, its class isn't translated (it's generic).
        // Generator.DependencyKey.DependencyKey(2): not translated, its class isn't translated (it's generic).
        // Generator.DependencyKey.getName(0): not translated, its class isn't translated (it's generic).
        // Generator.DependencyKey.isOptional(0): not translated, its class isn't translated (it's generic).
        // Generator.DependencyKey.equals(1): not translated, its class isn't translated (it's generic).
        // Generator.DependencyKey.hashCode(0): not translated, its class isn't translated (it's generic).
    }

    /// `DTBiomeTags` (com/dtteam/dynamictrees/data/tags/DTBiomeTags.java)
    pub mod dt_biome_tags {
        use super::*;
        // IS_BEE_NEST_COMMON: not translated, it uses Minecraft's `class_6862`.
        // IS_BEE_NEST_UNCOMMON: not translated, it uses Minecraft's `class_6862`.
        // IS_BEE_NEST_RARE: not translated, it uses Minecraft's `class_6862`.
        // IS_BEE_NEST_GUARANTEED: not translated, it uses Minecraft's `class_6862`.
        #[derive(Clone, Debug)]
        pub struct DTBiomeTags {
        }
        impl DTBiomeTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DTBiomeTags.bind(1): not translated, it uses Minecraft's `class_6862`.
    }

    /// `DTBlockTags` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java)
    pub mod dt_block_tags {
        use super::*;
        // BRANCHES: not translated, it uses Minecraft's `class_6862`.
        // STRIPPED_BRANCHES: not translated, it uses Minecraft's `class_6862`.
        // BRANCHES_THAT_BURN: not translated, it uses Minecraft's `class_6862`.
        // STRIPPED_BRANCHES_THAT_BURN: not translated, it uses Minecraft's `class_6862`.
        // FOLIAGE: not translated, it uses Minecraft's `class_6862`.
        // FUNGUS_BRANCHES: not translated, it uses Minecraft's `class_6862`.
        // STRIPPED_FUNGUS_BRANCHES: not translated, it uses Minecraft's `class_6862`.
        // FUNGUS_CAPS: not translated, it uses Minecraft's `class_6862`.
        // LEAVES: not translated, it uses Minecraft's `class_6862`.
        // SAPLINGS: not translated, it uses Minecraft's `class_6862`.
        // WART_BLOCKS: not translated, it uses Minecraft's `class_6862`.
        // ROOTY_SOIL: not translated, it uses Minecraft's `class_6862`.
        // AERIAL_ROOTS_ROOTY_SOIL: not translated, it uses Minecraft's `class_6862`.
        // ROOTS: not translated, it uses Minecraft's `class_6862`.
        #[derive(Clone, Debug)]
        pub struct DTBlockTags {
        }
        impl DTBlockTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DTBlockTags.bind(1): not translated, it uses Minecraft's `class_6862`.
    }

    /// `DTEntityTypeTags` (com/dtteam/dynamictrees/data/tags/DTEntityTypeTags.java)
    pub mod dt_entity_type_tags {
        use super::*;
        // CAN_PASS_THROUGH_LEAVES: not translated, it uses Minecraft's `class_6862`.
        // FALLING_TREE_DAMAGE_IMMUNE: not translated, it uses Minecraft's `class_6862`.
        #[derive(Clone, Debug)]
        pub struct DTEntityTypeTags {
        }
        impl DTEntityTypeTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DTEntityTypeTags.bind(1): not translated, it uses Minecraft's `class_6862`.
    }

    /// `DTItemTags` (com/dtteam/dynamictrees/data/tags/DTItemTags.java)
    pub mod dt_item_tags {
        use super::*;
        // BRANCHES: not translated, it uses Minecraft's `class_6862`.
        // BRANCHES_THAT_BURN: not translated, it uses Minecraft's `class_6862`.
        // FUNGUS_BRANCHES: not translated, it uses Minecraft's `class_6862`.
        // SEEDS: not translated, it uses Minecraft's `class_6862`.
        // FUNGUS_CAPS: not translated, it uses Minecraft's `class_6862`.
        // FERTILIZER: not translated, it uses Minecraft's `class_6862`.
        // ENHANCED_FERTILIZER: not translated, it uses Minecraft's `class_6862`.
        #[derive(Clone, Debug)]
        pub struct DTItemTags {
        }
        impl DTItemTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DTItemTags.bind(1): not translated, it uses Minecraft's `class_6862`.
    }

    /// `Applier` (com/dtteam/dynamictrees/deserialization/applier/Applier.java)
    pub mod applier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Applier.apply(2): not translated, it has no body.
    }

    /// `ArrayIteratorPropertyApplier` (com/dtteam/dynamictrees/deserialization/applier/ArrayIteratorPropertyApplier.java)
    pub mod array_iterator_property_applier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ArrayIteratorPropertyApplier.ArrayIteratorPropertyApplier(5): not translated, its class isn't translated (it's generic).
        // ArrayIteratorPropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
        // ArrayIteratorPropertyApplier.json(4): not translated, it's generic.
    }

    /// `ArrayPropertyApplier` (com/dtteam/dynamictrees/deserialization/applier/ArrayPropertyApplier.java)
    pub mod array_property_applier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ArrayPropertyApplier.ArrayPropertyApplier(5): not translated, its class isn't translated (it's generic).
        // ArrayPropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
        // ArrayPropertyApplier.json(4): not translated, it's generic.
    }

    /// `IfTrueApplier` (com/dtteam/dynamictrees/deserialization/applier/IfTrueApplier.java)
    pub mod if_true_applier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IfTrueApplier.apply(1): not translated, it has no body.
    }

    /// `JsonPropertyApplier` (com/dtteam/dynamictrees/deserialization/applier/JsonPropertyApplier.java)
    pub mod json_property_applier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // JsonPropertyApplier.JsonPropertyApplier(4): not translated, its class isn't translated (it's generic).
        // JsonPropertyApplier.JsonPropertyApplier(4): not translated, its class isn't translated (it's generic).
        // JsonPropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
    }

    /// `MapPropertyApplier` (com/dtteam/dynamictrees/deserialization/applier/MapPropertyApplier.java)
    pub mod map_property_applier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MapPropertyApplier.MapPropertyApplier(5): not translated, its class isn't translated (it's generic).
        // MapPropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
        // MapPropertyApplier.json(4): not translated, it's generic.
    }

    /// `MultiPropertyApplier` (com/dtteam/dynamictrees/deserialization/applier/MultiPropertyApplier.java)
    pub mod multi_property_applier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MultiPropertyApplier.MultiPropertyApplier(3): not translated, its class isn't translated (it's generic).
        // MultiPropertyApplier.addApplier(1): not translated, its class isn't translated (it's generic).
        // MultiPropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
        // MultiPropertyApplier.applyNext(4): not translated, its class isn't translated (it's generic).
    }

    /// `PropertyApplier` (com/dtteam/dynamictrees/deserialization/applier/PropertyApplier.java)
    pub mod property_applier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PropertyApplier.PropertyApplier(3): not translated, its class isn't translated (it's generic).
        // PropertyApplier.PropertyApplier(3): not translated, its class isn't translated (it's generic).
        // PropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
        // PropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
        // PropertyApplier.getObjectClass(0): not translated, its class isn't translated (it's generic).
    }

    /// `PropertyApplierResult` (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java)
    pub mod property_applier_result {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PropertyApplierResult {
            pub error_message: Option<String>,
            pub warnings: Option<Vec<String>>,
        }
        impl PropertyApplierResult {
            // PropertyApplierResult.PropertyApplierResult(1): not translated, it calls `Lists` (com.google.common.collect).newLinkedList.
            pub fn new_2(error_message: Option<String>, warnings: &[String]) -> Self {
                let mut s = Self { error_message: error_message.clone(), warnings: Some(warnings.to_vec()) };
                s.error_message = error_message.clone();
                s.warnings = Some(warnings.to_vec());
                s
            }
            pub fn failed(&self) -> bool {
                return self.error_message.is_some();
            }
            pub fn get_error(&self) -> Option<String> {
                return self.error_message.clone();
            }
            // PropertyApplierResult.addErrorPrefix(1): not translated, `this` used as a value.
            pub fn get_warnings(&self) -> Option<Vec<String>> {
                return self.warnings.clone();
            }
            // PropertyApplierResult.addWarnings(1): not translated, it changes an object through a second name.
            // PropertyApplierResult.addWarningsPrefix(1): not translated, List.replaceAll.
        }
        // PropertyApplierResult.success(0): not translated, it creates a PropertyApplierResult with a constructor that isn't translated.
        pub fn success_1(warnings: &[String]) -> property_applier_result::PropertyApplierResult {
            return property_applier_result::PropertyApplierResult::new_2(None, warnings);
        }
        // PropertyApplierResult.failure(1): not translated, it creates a PropertyApplierResult with a constructor that isn't translated.
        pub fn failure_2(error_message: &str, warnings: &[String]) -> property_applier_result::PropertyApplierResult {
            return property_applier_result::PropertyApplierResult::new_2(Some(error_message.to_string()), warnings);
        }
        // PropertyApplierResult.from(1): not translated, it uses Result, which isn't translated (it's an interface).
    }

    /// `VoidApplier` (com/dtteam/dynamictrees/deserialization/applier/VoidApplier.java)
    pub mod void_applier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // VoidApplier.apply(2): not translated, its class isn't translated (it's an interface).
        // VoidApplier.applySuccessful(2): not translated, it has no body.
    }

    /// `DeserializationException` (com/dtteam/dynamictrees/deserialization/DeserializationException.java)
    pub mod deserialization_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        // DeserializationException.DeserializationException(1): not translated, its class isn't translated (it extends `Exception`).
        // DeserializationException.error(1): not translated, it uses DeserializationException, which isn't translated (it extends `Exception`).
    }

    /// `AxisAlignedBBDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/AxisAlignedBBDeserializer.java)
    pub mod axis_aligned_bb_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // AxisAlignedBBDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
    }

    /// `BiomeListDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java)
    pub mod biome_list_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // DELAYED_BIOME_REGISTRY: not translated, it uses Minecraft's `class_2378`.
        // TAG_APPLIER: not translated, it uses Applier, which isn't translated (it's an interface).
        // NAME_APPLIER: not translated, it uses VoidApplier, which isn't translated (it's an interface).
        // NAMES_OR_APPLIER: not translated, it uses VoidApplier, which isn't translated (it's an interface).
        // TAGS_OR_APPLIER: not translated, it uses VoidApplier, which isn't translated (it's an interface).
        pub fn using_not_operator(category_string: &str) -> bool {
            return j_char_at(&category_string, 0) == '!';
        }
        // BiomeListDeserializer.BiomeListDeserializer(0): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // BiomeListDeserializer.registerAppliers(0): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // BiomeListDeserializer.applyAllAppliers(2): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // BiomeListDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
    }

    /// `BooleanOpDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/BooleanOpDeserializer.java)
    pub mod boolean_op_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // VALUES: not translated, it uses Minecraft's `class_247`.
        // BooleanOpDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
    }

    /// `BuiltInRegistryEntryDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/BuiltInRegistryEntryDeserializer.java)
    pub mod built_in_registry_entry_deserializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // BuiltInRegistryEntryDeserializer.BuiltInRegistryEntryDeserializer(2): not translated, its class isn't translated (it's generic).
        // BuiltInRegistryEntryDeserializer.BuiltInRegistryEntryDeserializer(3): not translated, its class isn't translated (it's generic).
        // BuiltInRegistryEntryDeserializer.deserialize(1): not translated, its class isn't translated (it's generic).
    }

    /// `ConfiguredDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/ConfiguredDeserializer.java)
    pub mod configured_deserializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfiguredDeserializer.ConfiguredDeserializer(3): not translated, its class isn't translated (it's generic).
        // ConfiguredDeserializer.deserialize(1): not translated, its class isn't translated (it's generic).
        // ConfiguredDeserializer.isConfigurationValid(1): not translated, its class isn't translated (it's generic).
        // ConfiguredDeserializer.getTemplate(1): not translated, its class isn't translated (it's generic).
        // ConfiguredDeserializer.getTemplateName(1): not translated, its class isn't translated (it's generic).
        // ConfiguredDeserializer.addProperty(4): not translated, it's generic.
    }

    /// `DeserializableClassDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/DeserializableClassDeserializer.java)
    pub mod deserializable_class_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // DeserializableClassDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
    }

    /// `Deserializer` (com/dtteam/dynamictrees/deserialization/deserializer/Deserializer.java)
    pub mod deserializer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Deserializer.deserialize(1): not translated, it has no body.
        // Deserializer.isValid(0): not translated, its class isn't translated (it's an interface).
        // Deserializer.deserializeIfValid(2): not translated, its class isn't translated (it's an interface).
    }

    /// `EnumDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/EnumDeserializer.java)
    pub mod enum_deserializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EnumDeserializer.EnumDeserializer(1): not translated, its class isn't translated (it's generic).
        // EnumDeserializer.deserialize(1): not translated, its class isn't translated (it's generic).
    }

    /// `JsonDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/JsonDeserializer.java)
    pub mod json_deserializer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // JsonDeserializer.deserializeIfValid(2): not translated, its class isn't translated (it's an interface).
    }

    /// `ListDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/ListDeserializer.java)
    pub mod list_deserializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ListDeserializer.ListDeserializer(1): not translated, its class isn't translated (it's generic).
        // ListDeserializer.ListDeserializer(2): not translated, its class isn't translated (it's generic).
        // ListDeserializer.deserialize(1): not translated, its class isn't translated (it's generic).
        // ListDeserializer.getListClass(1): not translated, it's generic.
        // ListDeserializer.getListClass(2): not translated, it's generic.
    }

    /// `MapColorDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/MapColorDeserializer.java)
    pub mod map_color_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // MATERIAL_COLORS: not translated, it uses Minecraft's `class_2960`.
        // MapColorDeserializer.registerMapColor(2): not translated, it uses Minecraft's `class_2960`.
        // MapColorDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
    }

    /// `MapDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/MapDeserializer.java)
    pub mod map_deserializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MapDeserializer.MapDeserializer(2): not translated, its class isn't translated (it's generic).
        // MapDeserializer.MapDeserializer(3): not translated, its class isn't translated (it's generic).
        // MapDeserializer.deserialize(1): not translated, its class isn't translated (it's generic).
        // MapDeserializer.getMapClass(2): not translated, it's generic.
        // MapDeserializer.getMapClass(3): not translated, it's generic.
    }

    /// `PropertyDefinitionDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/PropertyDefinitionDeserializer.java)
    pub mod property_definition_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // PropertyDefinitionDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // PropertyDefinitionDeserializer.deserializeDefinition(2): not translated, it's generic.
    }

    /// `RegistryEntryDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/RegistryEntryDeserializer.java)
    pub mod registry_entry_deserializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegistryEntryDeserializer.RegistryEntryDeserializer(1): not translated, its class isn't translated (it's generic).
        // RegistryEntryDeserializer.deserialize(1): not translated, its class isn't translated (it's generic).
    }

    /// `ResourceLocationDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/ResourceLocationDeserializer.java)
    pub mod resource_location_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // ResourceLocationDeserializer.ResourceLocationDeserializer(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // ResourceLocationDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // ResourceLocationDeserializer.isValidResourceLocation(1): not translated, it uses Minecraft's `class_2960`.
        // ResourceLocationDeserializer.decode(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // ResourceLocationDeserializer.create(0): not translated, it uses ResourceLocationDeserializer, which isn't translated (it implements `JsonDeserializer`).
        // ResourceLocationDeserializer.create(1): not translated, it uses ResourceLocationDeserializer, which isn't translated (it implements `JsonDeserializer`).
    }

    /// `SeedSaplingRecipeDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/SeedSaplingRecipeDeserializer.java)
    pub mod seed_sapling_recipe_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // SeedSaplingRecipeDeserializer.SeedSaplingRecipeDeserializer(0): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // SeedSaplingRecipeDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
    }

    /// `SoundTypeDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/SoundTypeDeserializer.java)
    pub mod sound_type_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // SOUND_TYPES: not translated, it uses Minecraft's `class_2960`.
        // SoundTypeDeserializer.registerSoundType(2): not translated, it uses Minecraft's `class_2960`.
        // SoundTypeDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
    }

    /// `StaticFieldDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/StaticFieldDeserializer.java)
    pub mod static_field_deserializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // StaticFieldDeserializer.StaticFieldDeserializer(1): not translated, its class isn't translated (it's generic).
        // StaticFieldDeserializer.deserialize(1): not translated, its class isn't translated (it's generic).
    }

    /// `VoxelShapeDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/VoxelShapeDeserializer.java)
    pub mod voxel_shape_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer`. Its static parts are below.
        // VoxelShapeDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // VoxelShapeDeserializer.deserializeArray(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // VoxelShapeDeserializer.deserializeObject(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // VoxelShapeDeserializer.getParametersJson(1): not translated, its class isn't translated (it implements `JsonDeserializer`).
        // VoxelShapeDeserializer.deserializeShapes(2): not translated, its class isn't translated (it implements `JsonDeserializer`).
    }

    /// `ChanceSelectorDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/ChanceSelectorDeserializer.java)
    pub mod chance_selector_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonBiomeDatabaseDeserializer`. Its static parts are below.
        // ChanceSelectorDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // ChanceSelectorDeserializer.createSimpleChanceSelector(1): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // ChanceSelectorDeserializer.readJsonChanceSelector(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // ChanceSelectorDeserializer.createSimpleChanceSelectorFromJson(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // ChanceSelectorDeserializer.createMathSelector(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // ChanceSelectorDeserializer.readExpressionSelector(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
    }

    /// `DensitySelectorDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/DensitySelectorDeserializer.java)
    pub mod density_selector_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonBiomeDatabaseDeserializer`. Its static parts are below.
        // DensitySelectorDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // DensitySelectorDeserializer.createStaticDensitySelector(1): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // DensitySelectorDeserializer.createScaleDensitySelector(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // DensitySelectorDeserializer.readDensitySelector(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // DensitySelectorDeserializer.readExpressionSelector(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
    }

    /// `JsonBiomeDatabaseDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/JsonBiomeDatabaseDeserializer.java)
    pub mod json_biome_database_deserializer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // JsonBiomeDatabaseDeserializer.isDefault(1): not translated, its class isn't translated (it's an interface).
    }

    /// `SpeciesSelectorDeserializer` (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/SpeciesSelectorDeserializer.java)
    pub mod species_selector_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonBiomeDatabaseDeserializer`. Its static parts are below.
        // SpeciesSelectorDeserializer.deserialize(1): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // SpeciesSelectorDeserializer.readStatic(1): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // SpeciesSelectorDeserializer.readStatic(1): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // SpeciesSelectorDeserializer.readSelector(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
        // SpeciesSelectorDeserializer.getRandomSpeciesSelector(2): not translated, its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`).
    }

    /// `JsonDeserializers` (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java)
    pub mod json_deserializers {
        use super::*;
        // DESERIALIZERS: not translated, it uses `Class`.
        // NULL: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // JSON_ELEMENT: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // JSON_PRIMITIVE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // JSON_OBJECT: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // JSON_ARRAY: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // BOOLEAN: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // NUMBER: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // STRING: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // BYTE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // SHORT: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // INTEGER: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // LONG: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // FLOAT: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // DOUBLE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // RESOURCE_LOCATION: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // DT_RESOURCE_LOCATION: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // BLOCK: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // ITEM: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // ITEM_STACK: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // AABB: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // VOXEL_SHAPE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // CELL_KIT: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // LEAVES_PROPERTIES: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // GROWTH_LOGIC_KIT: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // GEN_FEATURE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // FAMILY: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // FRUIT: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // POD: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // SPECIES: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // SOIL_PROPERTIES: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // SOIL_PROPERTIES_LIST: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // FEATURE_CANCELLER: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // RESOURCE_LOCATION_MAP: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // CONFIGURED_GEN_FEATURE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // CONFIGURED_GROWTH_LOGIC_KIT: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // SEED: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // BRANCH: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // VINE_TYPE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // DECORATION_STAGE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // OPERATION: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // CLIMATE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // BIOME_LIST: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // BIOME_PREDICATE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // SPECIES_SELECTOR: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // DENSITY_SELECTOR: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // CHANCE_SELECTOR: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // SEED_SAPLING_RECIPE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // MAP_COLOR: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // SOUND_TYPE: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // BOOLEAN_FUNCTION: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // DESERIALIZABLE_CLASS: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        // VARIABLE_DEFINITION: not translated, it uses JsonDeserializer, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct JsonDeserializers {
        }
        impl JsonDeserializers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // JsonDeserializers.get(1): not translated, it's generic.
        // JsonDeserializers.getOrThrow(1): not translated, it's generic.
        // JsonDeserializers.getOrThrow(2): not translated, it's generic.
        // JsonDeserializers.getDeserializableClasses(0): not translated, it uses `Class`.
        // JsonDeserializers.register(2): not translated, it's generic.
        // JsonDeserializers.registerRegistryEntryGetters(0): not translated, it uses JsonDeserializers.BLOCK, which isn't translated (it uses JsonDeserializer, which isn't translated (it's an interface)).
        // JsonDeserializers.postRegistryEvent(0): not translated, it uses IEventHelper, which isn't translated (it's an interface).
    }

    /// `JsonDeserializers.NullDeserializer` (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java)
    pub mod json_deserializers_null_deserializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // JsonDeserializers.NullDeserializer.isValid(0): not translated, its class isn't translated (it's generic).
        // JsonDeserializers.NullDeserializer.deserializeIfValid(2): not translated, its class isn't translated (it's generic).
        // JsonDeserializers.NullDeserializer.deserialize(1): not translated, its class isn't translated (it's generic).
    }

    /// `JsonHelper` (com/dtteam/dynamictrees/deserialization/JsonHelper.java)
    pub mod json_helper {
        use super::*;
        // LOGGER: the logger. Log calls become comments.
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson { html: false, ..(JGson { pretty: true, ..JGson::default() }) });
        #[derive(Clone, Debug)]
        pub struct JsonHelper {
        }
        impl JsonHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // JsonHelper.getGson(0): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // JsonHelper.load(1): not translated, it uses `File` (java.io).
        // JsonHelper.isComment(1): not translated, it's overloaded with the same number of parameters.
        // JsonHelper.isComment(1): not translated, it's overloaded with the same number of parameters.
        // JsonHelper.getOrDefault(4): not translated, it's generic.
        // JsonHelper.getOrDefault(6): not translated, it's generic.
        // JsonHelper.getAsOptional(3): not translated, it's generic.
        // JsonHelper.getBlockProperties(4): not translated, it uses Minecraft's `class_2251`.
        // JsonHelper.throwIfNotJsonObject(2): not translated, it's generic.
        // JsonHelper.throwIfShouldNotLoad(1): not translated, a lambda.
        // JsonHelper.isModLoaded(1): not translated, it uses JsonDeserializers.STRING, which isn't translated (it uses JsonDeserializer, which isn't translated (it's an interface)).
    }

    /// `JsonMapWrapper` (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java)
    pub mod json_map_wrapper {
        use super::*;
        // The class itself isn't translated: it implements `Map` (java.util). Its static parts are below.
        // JsonMapWrapper.JsonMapWrapper(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.size(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.isEmpty(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.containsKey(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.containsValue(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.get(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.put(2): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.remove(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.putAll(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.clear(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.keySet(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.values(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // JsonMapWrapper.entrySet(0): not translated, its class isn't translated (it implements `Map` (java.util)).
    }

    /// `JsonMath` (com/dtteam/dynamictrees/deserialization/JsonMath.java)
    pub mod json_math {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator). Its static parts are below.
        // JsonMath.JsonMath(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.getVariable(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.lazyInitNoise(0): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.processElement(2): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.apply(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
    }

    /// `JsonMath.Add` (com/dtteam/dynamictrees/deserialization/JsonMath.java)
    pub mod json_math_add {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator). Its static parts are below.
        // JsonMath.Add.Add(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.Add.apply(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
    }

    /// `JsonMath.Div` (com/dtteam/dynamictrees/deserialization/JsonMath.java)
    pub mod json_math_div {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator). Its static parts are below.
        // JsonMath.Div.Div(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.Div.apply(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
    }

    /// `JsonMath.IfGt` (com/dtteam/dynamictrees/deserialization/JsonMath.java)
    pub mod json_math_if_gt {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator). Its static parts are below.
        // JsonMath.IfGt.IfGt(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.IfGt.apply(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
    }

    /// `JsonMath.Mul` (com/dtteam/dynamictrees/deserialization/JsonMath.java)
    pub mod json_math_mul {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator). Its static parts are below.
        // JsonMath.Mul.Mul(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.Mul.apply(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
    }

    /// `JsonMath.Sub` (com/dtteam/dynamictrees/deserialization/JsonMath.java)
    pub mod json_math_sub {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator). Its static parts are below.
        // JsonMath.Sub.Sub(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
        // JsonMath.Sub.apply(1): not translated, its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)).
    }

    /// `JsonPropertyApplierLists` (com/dtteam/dynamictrees/deserialization/JsonPropertyApplierLists.java)
    pub mod json_property_applier_lists {
        use super::*;
        // PROPERTIES: not translated, it uses JsonPropertyAppliers, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct JsonPropertyApplierLists {
        }
        impl JsonPropertyApplierLists {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `JsonPropertyAppliers` (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java)
    pub mod json_property_appliers {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // JsonPropertyAppliers.JsonPropertyAppliers(1): not translated, its class isn't translated (it's generic).
        // JsonPropertyAppliers.applyAll(2): not translated, its class isn't translated (it's generic).
        // JsonPropertyAppliers.apply(3): not translated, its class isn't translated (it's generic).
        // JsonPropertyAppliers.register(1): not translated, it's generic.
        // JsonPropertyAppliers.register(3): not translated, it's generic.
        // JsonPropertyAppliers.register(3): not translated, it's generic.
        // JsonPropertyAppliers.registerArrayApplier(3): not translated, it's generic.
        // JsonPropertyAppliers.registerArrayApplier(3): not translated, it's generic.
        // JsonPropertyAppliers.registerListApplier(3): not translated, it's generic.
        // JsonPropertyAppliers.registerListApplier(3): not translated, it's generic.
        // JsonPropertyAppliers.registerMapApplier(3): not translated, it's generic.
        // JsonPropertyAppliers.registerMapApplier(3): not translated, it's generic.
        // JsonPropertyAppliers.registerIfTrueApplier(2): not translated, its class isn't translated (it's generic).
        // JsonPropertyAppliers.register(4): not translated, it's generic.
        // JsonPropertyAppliers.register(4): not translated, it's generic.
        // JsonPropertyAppliers.registerIfTrueApplier(3): not translated, it's generic.
        // JsonPropertyAppliers.registerArrayApplier(4): not translated, it's generic.
        // JsonPropertyAppliers.registerArrayApplier(4): not translated, it's generic.
        // JsonPropertyAppliers.registerListApplier(4): not translated, it's generic.
        // JsonPropertyAppliers.registerListApplier(4): not translated, it's generic.
        // JsonPropertyAppliers.registerMapApplier(4): not translated, it's generic.
        // JsonPropertyAppliers.registerMapApplier(4): not translated, it's generic.
        // JsonPropertyAppliers.getObjectType(0): not translated, its class isn't translated (it's generic).
        // JsonPropertyAppliers.getInputType(0): not translated, its class isn't translated (it's generic).
    }

    /// `MathFunction` (com/dtteam/dynamictrees/deserialization/MathFunction.java)
    pub mod math_function {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // MathFunction.getFunction(1): not translated, it uses MathFunction, which isn't translated (its constants carry values).
    }

    /// `ArithmeticType` (com/dtteam/dynamictrees/deserialization/math/ArithmeticType.java)
    pub mod arithmetic_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ArithmeticType {
            Add,
            Sub,
            Mul,
            Div,
            Mod,
        }
        impl ArithmeticType {
            pub const VALUES: [ArithmeticType; 5] = [ArithmeticType::Add, ArithmeticType::Sub, ArithmeticType::Mul, ArithmeticType::Div, ArithmeticType::Mod, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ArithmeticType::Add => "ADD",
                    ArithmeticType::Sub => "SUB",
                    ArithmeticType::Mul => "MUL",
                    ArithmeticType::Div => "DIV",
                    ArithmeticType::Mod => "MOD",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "ADD" => ArithmeticType::Add,
                    "SUB" => ArithmeticType::Sub,
                    "MUL" => ArithmeticType::Mul,
                    "DIV" => ArithmeticType::Div,
                    "MOD" => ArithmeticType::Mod,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BooleanType` (com/dtteam/dynamictrees/deserialization/math/BooleanType.java)
    pub mod boolean_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum BooleanType {
            Not,
            And,
            Or,
            Xor,
        }
        impl BooleanType {
            pub const VALUES: [BooleanType; 4] = [BooleanType::Not, BooleanType::And, BooleanType::Or, BooleanType::Xor, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    BooleanType::Not => "NOT",
                    BooleanType::And => "AND",
                    BooleanType::Or => "OR",
                    BooleanType::Xor => "XOR",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NOT" => BooleanType::Not,
                    "AND" => BooleanType::And,
                    "OR" => BooleanType::Or,
                    "XOR" => BooleanType::Xor,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ComparisonType` (com/dtteam/dynamictrees/deserialization/math/ComparisonType.java)
    pub mod comparison_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ComparisonType {
            GreaterThan,
            GreaterThanOrEqual,
            LessThan,
            LessThanOrEqual,
            Equal,
            NotEqual,
        }
        impl ComparisonType {
            pub const VALUES: [ComparisonType; 6] = [ComparisonType::GreaterThan, ComparisonType::GreaterThanOrEqual, ComparisonType::LessThan, ComparisonType::LessThanOrEqual, ComparisonType::Equal, ComparisonType::NotEqual, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ComparisonType::GreaterThan => "GREATER_THAN",
                    ComparisonType::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL",
                    ComparisonType::LessThan => "LESS_THAN",
                    ComparisonType::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
                    ComparisonType::Equal => "EQUAL",
                    ComparisonType::NotEqual => "NOT_EQUAL",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "GREATER_THAN" => ComparisonType::GreaterThan,
                    "GREATER_THAN_OR_EQUAL" => ComparisonType::GreaterThanOrEqual,
                    "LESS_THAN" => ComparisonType::LessThan,
                    "LESS_THAN_OR_EQUAL" => ComparisonType::LessThanOrEqual,
                    "EQUAL" => ComparisonType::Equal,
                    "NOT_EQUAL" => ComparisonType::NotEqual,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ExpressionParser` (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java)
    pub mod expression_parser {
        use super::*;
        // CACHE: not translated, it uses MathOperator, which isn't translated (it's an interface).
        // FUNCTIONS: not translated, it uses MathOperatorBuilder, which isn't translated (it's an interface).
        // VARIABLES: not translated, it uses MathOperator, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ExpressionParser {
            pub expression: String,
            pub cursor: i32,
        }
        impl ExpressionParser {
            pub fn new(expression: &str) -> Self {
                let mut s = Self { expression: String::new(), cursor: 0i32 };
                s.expression = expression.to_string();
                s
            }
            // ExpressionParser.parse(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseBoolean(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseComparison(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseAdditive(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseMultiplicative(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseUnary(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parsePrimary(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseNumber(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseFunction(1): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseIsSpeciesFunction(0): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseSpeciesName(0): not translated, it calls ExpressionParser.parseString, which isn't translated.
            // ExpressionParser.parseVariable(1): not translated, it uses MathOperator, which isn't translated (it's an interface).
            // ExpressionParser.parseIdentifier(0): not translated, `++` inside an expression.
            // ExpressionParser.parseString(0): not translated, `++` inside an expression.
            // ExpressionParser.expect(2): not translated, it calls ExpressionParser.consume, which isn't translated.
            // ExpressionParser.consume(1): not translated, String.startsWith with 2 values.
            pub fn is_number_start(&self) -> bool {
                return (self.peek().is_ascii_digit()) || (((self.peek() == '.') && ((self.cursor + 1) < j_len(&self.expression))) && (j_char_at(&self.expression, self.cursor + 1).is_ascii_digit()));
            }
            pub fn is_identifier_start(&self, character: char) -> bool {
                return character.is_alphabetic() || (character == '_');
            }
            pub fn is_identifier_part(&self, character: char) -> bool {
                return character.is_alphanumeric() || (character == '_');
            }
            pub fn peek(&self) -> char {
                return char::from_u32((if self.is_at_end() { ('\u{0000}' as i32) } else { (j_char_at(&self.expression, self.cursor) as i32) }) as u32).unwrap_or('\0');
            }
            pub fn is_at_end(&self) -> bool {
                return self.cursor >= j_len(&self.expression);
            }
            // ExpressionParser.error(1): not translated, it uses `JsonParseException` (com.google.gson).
        }
        // ExpressionParser.parse(1): not translated, it uses MathOperator, which isn't translated (it's an interface).
        pub fn sanitize_input(expression: &str) -> String {
            return ({ static RE: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("\\s")); &*RE }.replace_all(&expression, "")).to_lowercase();
        }
    }

    /// `MathContext` (com/dtteam/dynamictrees/deserialization/math/MathContext.java)
    pub mod math_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2382`. Its static parts are below.
        // MathContext.MathContext(4): not translated, its class isn't translated (it uses Minecraft's `class_2382`).
        // MathContext.MathContext(1): not translated, its class isn't translated (it uses Minecraft's `class_2382`).
        // MathContext.MathContext(2): not translated, its class isn't translated (it uses Minecraft's `class_2382`).
        // MathContext.pos(0): not translated, its class isn't translated (it uses Minecraft's `class_2382`).
        // MathContext.rand(0): not translated, its class isn't translated (it uses Minecraft's `class_2382`).
        // MathContext.species(0): not translated, its class isn't translated (it uses Minecraft's `class_2382`).
        // MathContext.radius(0): not translated, its class isn't translated (it uses Minecraft's `class_2382`).
    }

    /// `NoiseModel` (com/dtteam/dynamictrees/deserialization/math/noise/NoiseModel.java)
    pub mod noise_model {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // NoiseModel.sample(3): not translated, it has no body.
    }

    /// `NoiseType` (com/dtteam/dynamictrees/deserialization/math/noise/NoiseType.java)
    pub mod noise_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum NoiseType {
            Legacy,
            Perlin,
            Simplex,
        }
        impl NoiseType {
            pub const VALUES: [NoiseType; 3] = [NoiseType::Legacy, NoiseType::Perlin, NoiseType::Simplex, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    NoiseType::Legacy => "LEGACY",
                    NoiseType::Perlin => "PERLIN",
                    NoiseType::Simplex => "SIMPLEX",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "LEGACY" => NoiseType::Legacy,
                    "PERLIN" => NoiseType::Perlin,
                    "SIMPLEX" => NoiseType::Simplex,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `PerlinNoiseModel` (com/dtteam/dynamictrees/deserialization/math/noise/PerlinNoiseModel.java)
    pub mod perlin_noise_model {
        use super::*;
        // The class itself isn't translated: it implements `NoiseModel`. Its static parts are below.
        // PerlinNoiseModel.PerlinNoiseModel(2): not translated, its class isn't translated (it implements `NoiseModel`).
        // PerlinNoiseModel.sample(3): not translated, its class isn't translated (it implements `NoiseModel`).
    }

    /// `PerlinSimplexNoiseModel` (com/dtteam/dynamictrees/deserialization/math/noise/PerlinSimplexNoiseModel.java)
    pub mod perlin_simplex_noise_model {
        use super::*;
        // The class itself isn't translated: it implements `NoiseModel`. Its static parts are below.
        // PerlinSimplexNoiseModel.PerlinSimplexNoiseModel(2): not translated, its class isn't translated (it implements `NoiseModel`).
        // PerlinSimplexNoiseModel.sample(3): not translated, its class isn't translated (it implements `NoiseModel`).
    }

    /// `SimplexNoiseModel` (com/dtteam/dynamictrees/deserialization/math/noise/SimplexNoiseModel.java)
    pub mod simplex_noise_model {
        use super::*;
        // The class itself isn't translated: it implements `NoiseModel`. Its static parts are below.
        // SimplexNoiseModel.SimplexNoiseModel(1): not translated, its class isn't translated (it implements `NoiseModel`).
        // SimplexNoiseModel.sample(3): not translated, its class isn't translated (it implements `NoiseModel`).
    }

    /// `ArithmeticOperator` (com/dtteam/dynamictrees/deserialization/math/operator/ArithmeticOperator.java)
    pub mod arithmetic_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // ArithmeticOperator.ArithmeticOperator(3): not translated, its class isn't translated (it implements `MathOperator`).
        // ArithmeticOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `AverageOperator` (com/dtteam/dynamictrees/deserialization/math/operator/AverageOperator.java)
    pub mod average_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // AverageOperator.AverageOperator(1): not translated, its class isn't translated (it implements `MathOperator`).
        // AverageOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `BiFunctionOperator` (com/dtteam/dynamictrees/deserialization/math/operator/BiFunctionOperator.java)
    pub mod bi_function_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // BiFunctionOperator.BiFunctionOperator(2): not translated, its class isn't translated (it implements `MathOperator`).
        // BiFunctionOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `BooleanLogicOperator` (com/dtteam/dynamictrees/deserialization/math/operator/BooleanLogicOperator.java)
    pub mod boolean_logic_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // BooleanLogicOperator.BooleanLogicOperator(3): not translated, its class isn't translated (it implements `MathOperator`).
        // BooleanLogicOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
        pub fn is_true(value: f64) -> bool {
            return value >= 0.5f64;
        }
    }

    /// `ClampOperator` (com/dtteam/dynamictrees/deserialization/math/operator/ClampOperator.java)
    pub mod clamp_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // ClampOperator.ClampOperator(1): not translated, its class isn't translated (it implements `MathOperator`).
        // ClampOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `ComparisonOperator` (com/dtteam/dynamictrees/deserialization/math/operator/ComparisonOperator.java)
    pub mod comparison_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // ComparisonOperator.ComparisonOperator(3): not translated, its class isn't translated (it implements `MathOperator`).
        // ComparisonOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `Const` (com/dtteam/dynamictrees/deserialization/math/operator/Const.java)
    pub mod const_ {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // Const.Const(1): not translated, its class isn't translated (it implements `MathOperator`).
        // Const.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `Debug` (com/dtteam/dynamictrees/deserialization/math/operator/Debug.java)
    pub mod debug {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // Debug.Debug(1): not translated, its class isn't translated (it implements `MathOperator`).
        // Debug.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `FunctionOperator` (com/dtteam/dynamictrees/deserialization/math/operator/FunctionOperator.java)
    pub mod function_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // FunctionOperator.FunctionOperator(2): not translated, its class isn't translated (it implements `MathOperator`).
        // FunctionOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `IfOperator` (com/dtteam/dynamictrees/deserialization/math/operator/IfOperator.java)
    pub mod if_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // IfOperator.IfOperator(1): not translated, its class isn't translated (it implements `MathOperator`).
        // IfOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `IfSpecies` (com/dtteam/dynamictrees/deserialization/math/operator/IfSpecies.java)
    pub mod if_species {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // IfSpecies.IfSpecies(2): not translated, its class isn't translated (it implements `MathOperator`).
        // IfSpecies.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `IsSpecies` (com/dtteam/dynamictrees/deserialization/math/operator/IsSpecies.java)
    pub mod is_species {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // IsSpecies.IsSpecies(1): not translated, its class isn't translated (it implements `MathOperator`).
        // IsSpecies.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `LerpOperator` (com/dtteam/dynamictrees/deserialization/math/operator/LerpOperator.java)
    pub mod lerp_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // LerpOperator.LerpOperator(1): not translated, its class isn't translated (it implements `MathOperator`).
        // LerpOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `LogOperator` (com/dtteam/dynamictrees/deserialization/math/operator/LogOperator.java)
    pub mod log_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        pub static LOG2: LazyLock<f64> = LazyLock::new(|| 2.0f64.ln());
        // LogOperator.LogOperator(1): not translated, its class isn't translated (it implements `MathOperator`).
        // LogOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `MapOperator` (com/dtteam/dynamictrees/deserialization/math/operator/MapOperator.java)
    pub mod map_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // MapOperator.MapOperator(1): not translated, its class isn't translated (it implements `MathOperator`).
        // MapOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `MathOperator` (com/dtteam/dynamictrees/deserialization/math/operator/MathOperator.java)
    pub mod math_operator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MathOperator.apply(1): not translated, it has no body.
        // MathOperator.throwIfInvalidParameterLength(2): not translated, its class isn't translated (it's an interface).
        // MathOperator.throwIfInvalidParameterLengthRange(3): not translated, its class isn't translated (it's an interface).
        // MathOperator.throwIfInvalidParameterLengthMin(2): not translated, its class isn't translated (it's an interface).
    }

    /// `MathOperatorBuilder` (com/dtteam/dynamictrees/deserialization/math/operator/MathOperatorBuilder.java)
    pub mod math_operator_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MathOperatorBuilder.build(1): not translated, it has no body.
    }

    /// `MaxOperator` (com/dtteam/dynamictrees/deserialization/math/operator/MaxOperator.java)
    pub mod max_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // MaxOperator.MaxOperator(1): not translated, its class isn't translated (it implements `MathOperator`).
        // MaxOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `MinOperator` (com/dtteam/dynamictrees/deserialization/math/operator/MinOperator.java)
    pub mod min_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // MinOperator.MinOperator(1): not translated, its class isn't translated (it implements `MathOperator`).
        // MinOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `Noise` (com/dtteam/dynamictrees/deserialization/math/operator/Noise.java)
    pub mod noise {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        pub const BASE_SEED: i64 = 96i64;
        pub const DEFAULT_SCALE: f64 = 0.0078125f64;
        // Noise.Noise(4): not translated, its class isn't translated (it implements `MathOperator`).
        // Noise.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
        pub fn normalize(val: f64) -> f64 {
            return (val + 1.0f64) / 2.0f64;
        }
        // Noise.build(1): not translated, it uses Noise, which isn't translated (it implements `MathOperator`).
        // Noise.build(2): not translated, it uses Noise, which isn't translated (it implements `MathOperator`).
    }

    /// `NullOperator` (com/dtteam/dynamictrees/deserialization/math/operator/NullOperator.java)
    pub mod null_operator {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // NULL: not translated, it uses MathOperator, which isn't translated (it's an interface).
        // NullOperator.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `Scale` (com/dtteam/dynamictrees/deserialization/math/operator/Scale.java)
    pub mod scale {
        use super::*;
        // The class itself isn't translated: it implements `MathOperator`. Its static parts are below.
        // Scale.Scale(1): not translated, its class isn't translated (it implements `MathOperator`).
        // Scale.apply(1): not translated, its class isn't translated (it implements `MathOperator`).
    }

    /// `NoSuchDeserializerException` (com/dtteam/dynamictrees/deserialization/NoSuchDeserializerException.java)
    pub mod no_such_deserializer_exception {
        use super::*;
        // The class itself isn't translated: it extends `RuntimeException`. Its static parts are below.
        // NoSuchDeserializerException.NoSuchDeserializerException(1): not translated, its class isn't translated (it extends `RuntimeException`).
    }

    /// `PropertyAppliers` (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java)
    pub mod property_appliers {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PropertyAppliers.applyAll(2): not translated, it has no body.
        // PropertyAppliers.apply(3): not translated, it has no body.
        // PropertyAppliers.register(1): not translated, it has no body.
        // PropertyAppliers.register(3): not translated, it has no body.
        // PropertyAppliers.register(3): not translated, it has no body.
        // PropertyAppliers.registerArrayApplier(3): not translated, it has no body.
        // PropertyAppliers.registerArrayApplier(3): not translated, it has no body.
        // PropertyAppliers.registerListApplier(3): not translated, it has no body.
        // PropertyAppliers.registerListApplier(3): not translated, it has no body.
        // PropertyAppliers.registerMapApplier(3): not translated, it has no body.
        // PropertyAppliers.registerMapApplier(3): not translated, it has no body.
        // PropertyAppliers.registerIfTrueApplier(2): not translated, it has no body.
        // PropertyAppliers.register(4): not translated, it has no body.
        // PropertyAppliers.register(4): not translated, it has no body.
        // PropertyAppliers.registerIfTrueApplier(3): not translated, it has no body.
        // PropertyAppliers.registerArrayApplier(4): not translated, it has no body.
        // PropertyAppliers.registerArrayApplier(4): not translated, it has no body.
        // PropertyAppliers.registerListApplier(4): not translated, it has no body.
        // PropertyAppliers.registerListApplier(4): not translated, it has no body.
        // PropertyAppliers.registerMapApplier(4): not translated, it has no body.
        // PropertyAppliers.registerMapApplier(4): not translated, it has no body.
        // PropertyAppliers.getObjectType(0): not translated, it has no body.
        // PropertyAppliers.getInputType(0): not translated, it has no body.
    }

    /// `PropertyAppliers.ResultList` (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java)
    pub mod property_appliers_result_list {
        use super::*;
        // The class itself isn't translated: it extends `LinkedList` (java.util). Its static parts are below.
        // PropertyAppliers.ResultList.forEachError(1): not translated, its class isn't translated (it extends `LinkedList` (java.util)).
        // PropertyAppliers.ResultList.forEachWarning(1): not translated, its class isn't translated (it extends `LinkedList` (java.util)).
        // PropertyAppliers.ResultList.forEachErrorWarning(2): not translated, its class isn't translated (it extends `LinkedList` (java.util)).
    }

    /// `AbstractResult` (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java)
    pub mod abstract_result {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractResult.AbstractResult(3): not translated, its class isn't translated (it's generic).
        // AbstractResult.AbstractResult(4): not translated, its class isn't translated (it's generic).
        // AbstractResult.getInput(0): not translated, its class isn't translated (it's generic).
        // AbstractResult.get(0): not translated, its class isn't translated (it's generic).
        // AbstractResult.orElse(1): not translated, its class isn't translated (it's generic).
        // AbstractResult.orElseGet(1): not translated, its class isn't translated (it's generic).
        // AbstractResult.orElseThrow(0): not translated, its class isn't translated (it's generic).
        // AbstractResult.getError(0): not translated, its class isn't translated (it's generic).
        // AbstractResult.forEachWarning(1): not translated, its class isn't translated (it's generic).
        // AbstractResult.getWarnings(0): not translated, its class isn't translated (it's generic).
        // AbstractResult.success(0): not translated, its class isn't translated (it's generic).
    }

    /// `JsonResult` (com/dtteam/dynamictrees/deserialization/result/JsonResult.java)
    pub mod json_result {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // JsonResult.JsonResult(3): not translated, its class isn't translated (it's generic).
        // JsonResult.JsonResult(4): not translated, its class isn't translated (it's generic).
        // JsonResult.removeError(0): not translated, its class isn't translated (it's generic).
        // JsonResult.map(1): not translated, it's generic.
        // JsonResult.map(3): not translated, it's generic.
        // JsonResult.mapIfValid(3): not translated, it's generic.
        // JsonResult.mapIfType(2): not translated, it's generic.
        // JsonResult.mapToListOfType(1): not translated, it's generic.
        // JsonResult.mapEachIfArray(2): not translated, it's generic.
        // JsonResult.mapIfContains(3): not translated, it's generic.
        // JsonResult.mapIfContains(4): not translated, it's generic.
        // JsonResult.forInput(1): not translated, it uses JsonResult, which isn't translated (it's generic).
        // JsonResult.success(2): not translated, it's generic.
        // JsonResult.nullSuccess(2): not translated, it's generic.
        // JsonResult.failure(2): not translated, it's generic.
        // JsonResult.failure(3): not translated, it's generic.
        // JsonResult.from(2): not translated, it's generic.
    }

    /// `MappedJsonResult` (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java)
    pub mod mapped_json_result {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MappedJsonResult.MappedJsonResult(3): not translated, its class isn't translated (it's generic).
        // MappedJsonResult.MappedJsonResult(4): not translated, its class isn't translated (it's generic).
        // MappedJsonResult.elseMapIfType(2): not translated, it's generic.
        // MappedJsonResult.elseMapIfContains(3): not translated, it's generic.
        // MappedJsonResult.elseMapIfContains(4): not translated, it's generic.
        // MappedJsonResult.elseError(2): not translated, its class isn't translated (it's generic).
        // MappedJsonResult.mapErrorneous(1): not translated, it's generic.
        // MappedJsonResult.map(2): not translated, it's generic.
        // MappedJsonResult.errorneousMap(2): not translated, it's generic.
    }

    /// `MappedResult` (com/dtteam/dynamictrees/deserialization/result/MappedResult.java)
    pub mod mapped_result {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MappedResult.elseMapIfType(2): not translated, it's generic.
        // MappedResult.elseMapIfType(2): not translated, it's generic.
        // MappedResult.elseMapIfContains(3): not translated, it's generic.
        // MappedResult.elseMapIfContains(3): not translated, it's generic.
        // MappedResult.elseMapIfContains(4): not translated, it's generic.
        // MappedResult.elseMapIfContains(4): not translated, it's generic.
        // MappedResult.elseTypeError(0): not translated, its class isn't translated (it's an interface).
        // MappedResult.elseError(2): not translated, it has no body.
    }

    /// `NullJsonResult` (com/dtteam/dynamictrees/deserialization/result/NullJsonResult.java)
    pub mod null_json_result {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NullJsonResult.NullJsonResult(3): not translated, its class isn't translated (it's generic).
        // NullJsonResult.NullJsonResult(4): not translated, its class isn't translated (it's generic).
        // NullJsonResult.success(0): not translated, its class isn't translated (it's generic).
    }

    /// `Result` (com/dtteam/dynamictrees/deserialization/result/Result.java)
    pub mod result {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Result.getInput(0): not translated, it has no body.
        // Result.get(0): not translated, it has no body.
        // Result.ifSuccess(1): not translated, its class isn't translated (it's an interface).
        // Result.ifSuccessOrElse(3): not translated, its class isn't translated (it's an interface).
        // Result.ifSuccessOrElseThrow(2): not translated, its class isn't translated (it's an interface).
        // Result.orElse(1): not translated, it has no body.
        // Result.orElse(3): not translated, its class isn't translated (it's an interface).
        // Result.orElseApply(2): not translated, its class isn't translated (it's an interface).
        // Result.orElseApply(3): not translated, its class isn't translated (it's an interface).
        // Result.orElseGet(1): not translated, it has no body.
        // Result.orElseThrow(0): not translated, it has no body.
        // Result.getError(0): not translated, it has no body.
        // Result.forEachWarning(1): not translated, it has no body.
        // Result.getWarnings(0): not translated, it has no body.
        // Result.success(0): not translated, it has no body.
        // Result.removeError(0): not translated, it has no body.
        // Result.map(1): not translated, it's generic.
        // Result.map(1): not translated, it's generic.
        // Result.map(2): not translated, it's generic.
        // Result.map(2): not translated, it's generic.
        // Result.map(3): not translated, it's generic.
        // Result.map(3): not translated, it's generic.
        // Result.mapIfValid(3): not translated, it's generic.
        // Result.mapIfValid(3): not translated, it's generic.
        // Result.mapIfType(2): not translated, it's generic.
        // Result.mapIfType(2): not translated, it's generic.
        // Result.mapToListOfType(1): not translated, it has no body.
        // Result.mapEachIfArray(2): not translated, it's generic.
        // Result.mapEachIfArray(2): not translated, it's generic.
        // Result.mapIfContains(3): not translated, it's generic.
        // Result.mapIfContains(3): not translated, it's generic.
        // Result.mapIfContains(4): not translated, it's generic.
        // Result.mapIfContains(4): not translated, it's generic.
    }

    /// `Result.Mapper` (com/dtteam/dynamictrees/deserialization/result/Result.java)
    pub mod result_mapper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `Result.SimpleMapper` (com/dtteam/dynamictrees/deserialization/result/Result.java)
    pub mod result_simple_mapper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Result.SimpleMapper.fullMapper(0): not translated, its class isn't translated (it's an interface).
    }

    /// `ShapeFunctions` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java)
    pub mod shape_functions {
        use super::*;
        // SHAPE_FUNCTIONS: not translated, it uses ShapeFunction, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ShapeFunctions {
        }
        impl ShapeFunctions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ShapeFunctions.registerShapeFunction(2): not translated, it uses ShapeFunction, which isn't translated (it's an interface).
        // ShapeFunctions.calculateShape(2): not translated, it uses Minecraft's `DataResult`.
        // ShapeFunctions.calculateShape(2): not translated, it uses Minecraft's `DataResult`.
    }

    /// `ShapeFunctions.FruitShapeFunction` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java)
    pub mod shape_functions_fruit_shape_function {
        use super::*;
        // The class itself isn't translated: it implements `ShapeFunction`. Its static parts are below.
        // ShapeFunctions.FruitShapeFunction.getParameters(1): not translated, its class isn't translated (it implements `ShapeFunction`).
        // ShapeFunctions.FruitShapeFunction.calculateShape(1): not translated, its class isn't translated (it implements `ShapeFunction`).
        // ShapeFunctions.FruitShapeFunction.createFruitShape(4): not translated, it uses Minecraft's `class_238`.
    }

    /// `ShapeFunctions.FruitShapeFunction.Parameters` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java)
    pub mod shape_functions_fruit_shape_function_parameters {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct Parameters {
            pub width: f32,
            pub height: f32,
            pub stem_length: f32,
            pub fraction: f32,
        }
        impl Parameters {
            pub fn new(width: f32, height: f32, stem_length: f32, fraction: f32) -> Self {
                let mut s = Self { width: width, height: height, stem_length: stem_length, fraction: fraction };
                s.width = width;
                s.height = height;
                s.stem_length = stem_length;
                s.fraction = fraction;
                s
            }
            pub fn width(&self) -> f32 {
                return self.width;
            }
            pub fn height(&self) -> f32 {
                return self.height;
            }
            pub fn stem_length(&self) -> f32 {
                return self.stem_length;
            }
            pub fn fraction(&self) -> f32 {
                return self.fraction;
            }
        }
    }

    /// `ShapeFunctions.PodShapeFunction` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java)
    pub mod shape_functions_pod_shape_function {
        use super::*;
        // The class itself isn't translated: it implements `ShapeFunction`. Its static parts are below.
        // ShapeFunctions.PodShapeFunction.getParameters(1): not translated, its class isn't translated (it implements `ShapeFunction`).
        // ShapeFunctions.PodShapeFunction.calculateShape(1): not translated, its class isn't translated (it implements `ShapeFunction`).
        // ShapeFunctions.PodShapeFunction.offsetBoundingBox(3): not translated, it uses Minecraft's `class_238`.
    }

    /// `ShapeFunctions.PodShapeFunction.Parameters` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java)
    pub mod shape_functions_pod_shape_function_parameters {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2350`. Its static parts are below.
        // SIDE_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ShapeFunctions.PodShapeFunction.Parameters.Parameters(6): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // ShapeFunctions.PodShapeFunction.Parameters.width(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // ShapeFunctions.PodShapeFunction.Parameters.height(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // ShapeFunctions.PodShapeFunction.Parameters.stemLength(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // ShapeFunctions.PodShapeFunction.Parameters.fraction(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // ShapeFunctions.PodShapeFunction.Parameters.sideOffset(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // ShapeFunctions.PodShapeFunction.Parameters.side(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
    }

    /// `ShapeFunctions.ShapeFunction` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java)
    pub mod shape_functions_shape_function {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ShapeFunctions.ShapeFunction.getParameters(1): not translated, it has no body.
        // ShapeFunctions.ShapeFunction.calculateShape(1): not translated, it has no body.
    }

    /// `TagKeyJsonPropertyApplier` (com/dtteam/dynamictrees/deserialization/TagKeyJsonPropertyApplier.java)
    pub mod tag_key_json_property_applier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TagKeyJsonPropertyApplier.TagKeyJsonPropertyApplier(3): not translated, its class isn't translated (it's generic).
        // TagKeyJsonPropertyApplier.TagKeyJsonPropertyApplier(3): not translated, its class isn't translated (it's generic).
        // TagKeyJsonPropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
        // TagKeyJsonPropertyApplier.applyIfShould(3): not translated, its class isn't translated (it's generic).
    }

    /// `DynamicTrees` (com/dtteam/dynamictrees/DynamicTrees.java)
    pub mod dynamic_trees {
        use super::*;
        pub const MOD_ID: &str = "dynamictrees";
        pub const MOD_NAME: &str = "Dynamic Trees";
        // LOG: the logger. Log calls become comments.
        pub const MINECRAFT: &str = "minecraft";
        pub const SERENE_SEASONS: &str = "sereneseasons";
        pub const FAST_LEAF_DECAY: &str = "fastleafdecay";
        pub const PASSABLE_FOLIAGE: &str = "passablefoliage";
        pub const DYNAMIC_TREES_PLUS: &str = "dynamictreesplus";
        // NULL: not translated, it uses Minecraft's `class_2960`.
        // OAK: not translated, it uses Minecraft's `class_2960`.
        // BIRCH: not translated, it uses Minecraft's `class_2960`.
        // SPRUCE: not translated, it uses Minecraft's `class_2960`.
        // JUNGLE: not translated, it uses Minecraft's `class_2960`.
        // DARK_OAK: not translated, it uses Minecraft's `class_2960`.
        // ACACIA: not translated, it uses Minecraft's `class_2960`.
        // AZALEA: not translated, it uses Minecraft's `class_2960`.
        // CRIMSON: not translated, it uses Minecraft's `class_2960`.
        // WARPED: not translated, it uses Minecraft's `class_2960`.
        // SETUP_HANDLERS: not translated, a function kept in a variable or field (only parameters can be).
        #[derive(Clone, Debug)]
        pub struct DynamicTrees {
        }
        impl DynamicTrees {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DynamicTrees.init(0): not translated, it calls SeasonCompatibilityHandler.registerBuiltInSeasonManagers, which isn't translated.
        // DynamicTrees.runOnCommonSetup(1): not translated, it uses DynamicTrees.SETUP_HANDLERS, which isn't translated (a function kept in a variable or field (only parameters can be)).
        // DynamicTrees.commonSetup(0): not translated, it uses RegistryHandler.REGISTRY, which isn't translated (it uses ConcurrentRegistry, which isn't translated (it's generic)).
        // DynamicTrees.location(1): not translated, it uses Minecraft's `class_2960`.
    }

    /// `DynamicTrees.AxeDamage` (com/dtteam/dynamictrees/DynamicTrees.java)
    pub mod dynamic_trees_axe_damage {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum AxeDamage {
            Vanilla,
            Thickness,
            Volume,
        }
        impl AxeDamage {
            pub const VALUES: [AxeDamage; 3] = [AxeDamage::Vanilla, AxeDamage::Thickness, AxeDamage::Volume, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    AxeDamage::Vanilla => "VANILLA",
                    AxeDamage::Thickness => "THICKNESS",
                    AxeDamage::Volume => "VOLUME",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "VANILLA" => AxeDamage::Vanilla,
                    "THICKNESS" => AxeDamage::Thickness,
                    "VOLUME" => AxeDamage::Volume,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `DynamicTrees.DestroyMode` (com/dtteam/dynamictrees/DynamicTrees.java)
    pub mod dynamic_trees_destroy_mode {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum DestroyMode {
            Ignore,
            Sloppy,
            SetRadius,
            Harvest,
            Rot,
            Overflow,
        }
        impl DestroyMode {
            pub const VALUES: [DestroyMode; 6] = [DestroyMode::Ignore, DestroyMode::Sloppy, DestroyMode::SetRadius, DestroyMode::Harvest, DestroyMode::Rot, DestroyMode::Overflow, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    DestroyMode::Ignore => "IGNORE",
                    DestroyMode::Sloppy => "SLOPPY",
                    DestroyMode::SetRadius => "SET_RADIUS",
                    DestroyMode::Harvest => "HARVEST",
                    DestroyMode::Rot => "ROT",
                    DestroyMode::Overflow => "OVERFLOW",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "IGNORE" => DestroyMode::Ignore,
                    "SLOPPY" => DestroyMode::Sloppy,
                    "SET_RADIUS" => DestroyMode::SetRadius,
                    "HARVEST" => DestroyMode::Harvest,
                    "ROT" => DestroyMode::Rot,
                    "OVERFLOW" => DestroyMode::Overflow,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `DynamicTreesFabric` (com/dtteam/dynamictrees/DynamicTreesFabric.java)
    pub mod dynamic_trees_fabric {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // DynamicTreesFabric.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // DynamicTreesFabric.getEntryPointContainers(0): not translated, it uses Fabric's `EntrypointContainer`.
        // DynamicTreesFabric.runOnEntryPoints(2): not translated, it uses Fabric's `EntrypointContainer`.
    }

    /// `DynamicTreesFabricClient` (com/dtteam/dynamictrees/DynamicTreesFabricClient.java)
    pub mod dynamic_trees_fabric_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        pub static INITIALIZED: JCell<bool> = JCell::new(false);
        pub static WOOD_COLORS_DISCOVERED: JCell<bool> = JCell::new(false);
        // DynamicTreesFabricClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // DynamicTreesFabricClient.registerClientWorldLoad(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // DynamicTreesFabricClient.registerModelLoaders(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // DynamicTreesFabricClient.registerEntityRenderers(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // DynamicTreesFabricClient.registerColorHandlers(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // DynamicTreesFabricClient.registerBlockColors(0): not translated, it uses Minecraft's `class_324`.
        // DynamicTreesFabricClient.isValidPos(2): not translated, it uses Minecraft's `class_1922`.
        // DynamicTreesFabricClient.registerTooltipCallback(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // DynamicTreesFabricClient.registerClientTick(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // DynamicTreesFabricClient.discoverWoodColors(0): not translated, it uses Minecraft's `class_2960`.
        // DynamicTreesFabricClient.getFaceColor(3): not translated, it uses Minecraft's `class_2680`.
    }

    /// `AnimationConstants` (com/dtteam/dynamictrees/entity/animation/AnimationConstants.java)
    pub mod animation_constants {
        use super::*;
        pub const TREE_GRAVITY: f32 = 0.03f32;
        pub const TREE_ELASTICITY: f32 = 0.25f32;
        // TREE_DAMAGE_TYPE: not translated, it uses Minecraft's `class_5321`.
        #[derive(Clone, Debug)]
        pub struct AnimationConstants {
        }
        impl AnimationConstants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AnimationConstants.treeDamage(1): not translated, it uses Minecraft's `class_1282`.
    }

    /// `AnimationHandler` (com/dtteam/dynamictrees/entity/animation/AnimationHandler.java)
    pub mod animation_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AnimationHandler.getName(0): not translated, it has no body.
        // AnimationHandler.initMotion(1): not translated, it has no body.
        // AnimationHandler.handleMotion(1): not translated, it has no body.
        // AnimationHandler.dropPayload(1): not translated, it has no body.
        // AnimationHandler.shouldDie(1): not translated, it has no body.
        // AnimationHandler.renderTransform(4): not translated, it has no body.
        // AnimationHandler.shouldRender(4): not translated, it has no body.
    }

    /// `AnimationHandlers` (com/dtteam/dynamictrees/entity/animation/AnimationHandlers.java)
    pub mod animation_handlers {
        use super::*;
        // voidAnimationHandler: not translated, it uses AnimationHandler, which isn't translated (it's an interface).
        // defaultAnimationHandler: not translated, its value couldn't be read: an anonymous class.
        // blastAnimationHandler: not translated, its value couldn't be read: an anonymous class.
        // falloverAnimationHandler: not translated, it uses AnimationHandler, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct AnimationHandlers {
        }
        impl AnimationHandlers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `DataAnimationHandler` (com/dtteam/dynamictrees/entity/animation/DataAnimationHandler.java)
    pub mod data_animation_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DataAnimationHandler {
        }
        impl DataAnimationHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FalloverAnimationHandler` (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java)
    pub mod fallover_animation_handler {
        use super::*;
        // The class itself isn't translated: it implements `AnimationHandler`. Its static parts are below.
        pub const TICKS_BEFORE_CHECKING_COLLISION: i32 = 10;
        // FalloverAnimationHandler.getName(0): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.getData(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.playStartSound(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.playEndSound(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.playFallThroughWaterSound(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.rotateAroundAxis(3): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.flingLeavesParticles(2): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.spawnParticlesAtLeaves(7): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.getRelativeLeavesPosition(2): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.initMotion(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.handleMotion(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.testCollision(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.addRotation(2): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.testEntityCollision(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.intersects(3): not translated, it uses Minecraft's `class_238`.
        // FalloverAnimationHandler.dropPayload(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.shouldDie(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.renderTransform(4): not translated, its class isn't translated (it implements `AnimationHandler`).
        // FalloverAnimationHandler.shouldRender(4): not translated, its class isn't translated (it implements `AnimationHandler`).
    }

    /// `FalloverAnimationHandler.HandlerData` (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java)
    pub mod fallover_animation_handler_handler_data {
        use super::*;
        // The class itself isn't translated: it extends `DataAnimationHandler`. Its static parts are below.
    }

    /// `PhysicsAnimationHandler` (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java)
    pub mod physics_animation_handler {
        use super::*;
        // The class itself isn't translated: it implements `AnimationHandler`. Its static parts are below.
        // PhysicsAnimationHandler.getName(0): not translated, its class isn't translated (it implements `AnimationHandler`).
        // PhysicsAnimationHandler.getData(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // PhysicsAnimationHandler.playEndSound(2): not translated, its class isn't translated (it implements `AnimationHandler`).
        // PhysicsAnimationHandler.initMotion(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // PhysicsAnimationHandler.handleMotion(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // PhysicsAnimationHandler.dropPayload(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // PhysicsAnimationHandler.shouldDie(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // PhysicsAnimationHandler.renderTransform(4): not translated, its class isn't translated (it implements `AnimationHandler`).
        // PhysicsAnimationHandler.shouldRender(4): not translated, its class isn't translated (it implements `AnimationHandler`).
    }

    /// `PhysicsAnimationHandler.HandlerData` (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java)
    pub mod physics_animation_handler_handler_data {
        use super::*;
        // The class itself isn't translated: it extends `DataAnimationHandler`. Its static parts are below.
    }

    /// `VoidAnimationHandler` (com/dtteam/dynamictrees/entity/animation/VoidAnimationHandler.java)
    pub mod void_animation_handler {
        use super::*;
        // The class itself isn't translated: it implements `AnimationHandler`. Its static parts are below.
        // VoidAnimationHandler.getName(0): not translated, its class isn't translated (it implements `AnimationHandler`).
        // VoidAnimationHandler.shouldDie(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // VoidAnimationHandler.renderTransform(4): not translated, its class isn't translated (it implements `AnimationHandler`).
        // VoidAnimationHandler.initMotion(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // VoidAnimationHandler.handleMotion(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // VoidAnimationHandler.dropPayload(1): not translated, its class isn't translated (it implements `AnimationHandler`).
        // VoidAnimationHandler.shouldRender(4): not translated, its class isn't translated (it implements `AnimationHandler`).
    }

    /// `FallingTreeEntity` (com/dtteam/dynamictrees/entity/FallingTreeEntity.java)
    pub mod falling_tree_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_1297`. Its static parts are below.
        // voxelDataParameter: not translated, it uses Minecraft's `class_2940`.
        // AnimHandlerFall: not translated, it uses AnimationHandler, which isn't translated (it's an interface).
        // AnimHandlerDrop: not translated, it uses AnimationHandler, which isn't translated (it's an interface).
        // AnimHandlerBurn: not translated, it uses AnimationHandler, which isn't translated (it's an interface).
        // AnimHandlerFling: not translated, it uses AnimationHandler, which isn't translated (it's an interface).
        // AnimHandlerBlast: not translated, it uses AnimationHandler, which isn't translated (it's an interface).
        // FallingTreeEntity.FallingTreeEntity(2): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.isClientBuilt(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.setData(3): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.buildVoxelData(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.setupFromNBT(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.buildClient(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.cleanupShellBlocks(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.buildAABBFromDestroyData(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.method_5830(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.getDestroyData(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.getPayload(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.getGeomCenter(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.getMassCenter(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.getVolume(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.hasLeaves(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.getSpecies(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.method_5814(3): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.method_5773(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.updateNeighbors(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.selectAnimationHandler(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.defaultAnimationHandler(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.modelCleanup(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.handleMotion(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.dropPayLoad(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.shouldDie(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.method_5727(3): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.standardDropLogsPayload(1): not translated, it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.standardDropLeavesPayLoad(1): not translated, it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.spawnItemAsEntity(3): not translated, it uses Minecraft's `class_1937`.
        // FallingTreeEntity.method_5693(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.setVoxelData(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.getVoxelData(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.method_5749(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.method_5652(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntity.dropTree(4): not translated, it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`).
    }

    /// `FallingTreeEntity.DestroyType` (com/dtteam/dynamictrees/entity/FallingTreeEntity.java)
    pub mod falling_tree_entity_destroy_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum DestroyType {
            Void,
            Harvest,
            Explode,
            Fire,
            Root,
        }
        impl DestroyType {
            pub const VALUES: [DestroyType; 5] = [DestroyType::Void, DestroyType::Harvest, DestroyType::Explode, DestroyType::Fire, DestroyType::Root, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    DestroyType::Void => "VOID",
                    DestroyType::Harvest => "HARVEST",
                    DestroyType::Explode => "EXPLODE",
                    DestroyType::Fire => "FIRE",
                    DestroyType::Root => "ROOT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "VOID" => DestroyType::Void,
                    "HARVEST" => DestroyType::Harvest,
                    "EXPLODE" => DestroyType::Explode,
                    "FIRE" => DestroyType::Fire,
                    "ROOT" => DestroyType::Root,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `LingeringEffectorEntity` (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java)
    pub mod lingering_effector_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_1297`. Its static parts are below.
        // effectorDataParameter: not translated, it uses Minecraft's `class_2940`.
        // LingeringEffectorEntity.LingeringEffectorEntity(2): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.setData(3): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.treeHasEffectorForEffect(3): not translated, it uses Minecraft's `class_1936`.
        // LingeringEffectorEntity.setBlockPos(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.getBlockPos(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.getEffect(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.method_5693(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.method_5749(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.method_5652(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.buildEffectorData(2): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.setEffectorData(1): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.getEffectorData(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
        // LingeringEffectorEntity.method_5773(0): not translated, its class isn't translated (it extends Minecraft's `class_1297`).
    }

    /// `FallingTreeRenderer` (com/dtteam/dynamictrees/entity/render/FallingTreeRenderer.java)
    pub mod falling_tree_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_897`. Its static parts are below.
        // FallingTreeRenderer.FallingTreeRenderer(1): not translated, its class isn't translated (it extends Minecraft's `class_897`).
        // FallingTreeRenderer.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `class_897`).
        // FallingTreeRenderer.render(6): not translated, its class isn't translated (it extends Minecraft's `class_897`).
    }

    /// `LingeringEffectorRenderer` (com/dtteam/dynamictrees/entity/render/LingeringEffectorRenderer.java)
    pub mod lingering_effector_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_897`. Its static parts are below.
        // LingeringEffectorRenderer.LingeringEffectorRenderer(1): not translated, its class isn't translated (it extends Minecraft's `class_897`).
        // LingeringEffectorRenderer.shouldRender(5): not translated, its class isn't translated (it extends Minecraft's `class_897`).
        // LingeringEffectorRenderer.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `class_897`).
    }

    /// `CommonEventHandler` (com/dtteam/dynamictrees/event/handler/CommonEventHandler.java)
    pub mod common_event_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonEventHandler {
        }
        impl CommonEventHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonEventHandler.RegisterEvents(0): not translated, it uses Fabric's `ServerTickEvents`.START_WORLD_TICK.
    }

    /// `CommonEventHandler.FabricReloadListener` (com/dtteam/dynamictrees/event/handler/CommonEventHandler.java)
    pub mod common_event_handler_fabric_reload_listener {
        use super::*;
        // The class itself isn't translated: it extends `ReloadListener`. Its static parts are below.
        // CommonEventHandler.FabricReloadListener.FabricReloadListener(0): not translated, its class isn't translated (it extends `ReloadListener`).
        // CommonEventHandler.FabricReloadListener.method_25931(6): not translated, its class isn't translated (it extends `ReloadListener`).
        // CommonEventHandler.FabricReloadListener.getFabricId(0): not translated, its class isn't translated (it extends `ReloadListener`).
    }

    /// `ModEventHandler` (com/dtteam/dynamictrees/event/handler/ModEventHandler.java)
    pub mod mod_event_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModEventHandler {
        }
        impl ModEventHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModEventHandler.RegisterEvents(0): not translated, it uses SimpleRegistry, which isn't translated (it's generic).
        // ModEventHandler.registerLeavesPropertiesTypes(0): not translated, it uses LeavesProperties.REGISTRY, which isn't translated (it uses TypedRegistry, which isn't translated (it's generic)).
        // ModEventHandler.registerFamilyTypes(0): not translated, it uses TypedRegistry, which isn't translated (it's generic).
        // ModEventHandler.registerSpeciesTypes(0): not translated, it uses TypedRegistry, which isn't translated (it's generic).
        // ModEventHandler.registerSoilPropertiesTypes(0): not translated, it uses SoilProperties.REGISTRY, which isn't translated (it uses TypedRegistry, which isn't translated (it's generic)).
        // ModEventHandler.registerCellKits(0): not translated, it uses Registry, which isn't translated (it's an interface).
        // ModEventHandler.registerGrowthLogicKits(0): not translated, it uses Registry, which isn't translated (it's an interface).
        // ModEventHandler.registerGenFeatures(0): not translated, it uses Registry, which isn't translated (it's an interface).
        // ModEventHandler.registerFeatureCancellers(0): not translated, it uses Registry, which isn't translated (it's an interface).
    }

    /// `VanillaSaplingEventHandler` (com/dtteam/dynamictrees/event/handler/VanillaSaplingEventHandler.java)
    pub mod vanilla_sapling_event_handler {
        use super::*;
        pub static IS_ENABLED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct VanillaSaplingEventHandler {
        }
        impl VanillaSaplingEventHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VanillaSaplingEventHandler.register(0): not translated, it uses Fabric's `UseBlockCallback`.EVENT.
        // VanillaSaplingEventHandler.updateEnabled(0): not translated, it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // VanillaSaplingEventHandler.onUseBlock(4): not translated, it uses Minecraft's `class_1269`.
    }

    /// `DendroPotion` (com/dtteam/dynamictrees/item/DendroPotion.java)
    pub mod dendro_potion {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_1792`. Its static parts are below.
        // DendroPotion.DendroPotion(0): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // DendroPotion.applyIndexTag(2): not translated, it uses Minecraft's `class_1799`.
        // DendroPotion.getPotionType(1): not translated, it uses DendroPotionType, which isn't translated (its constants carry values).
        // DendroPotion.getSubstanceEffect(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // DendroPotion.method_7866(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // DendroPotion.getColor(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // DendroPotion.getEmptyContainer(0): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
    }

    /// `DendroPotion.DendroPotionType` (com/dtteam/dynamictrees/item/DendroPotion.java)
    pub mod dendro_potion_dendro_potion_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // DendroPotion.DendroPotionType.DendroPotionType(5): not translated, its class isn't translated (its constants carry values).
        // DendroPotion.DendroPotionType.DendroPotionType(6): not translated, its class isn't translated (its constants carry values).
        // DendroPotion.DendroPotionType.getIndex(0): not translated, its class isn't translated (its constants carry values).
        // DendroPotion.DendroPotionType.isActive(0): not translated, its class isn't translated (its constants carry values).
        // DendroPotion.DendroPotionType.getName(0): not translated, its class isn't translated (its constants carry values).
        // DendroPotion.DendroPotionType.getColor(0): not translated, its class isn't translated (its constants carry values).
        // DendroPotion.DendroPotionType.getIngredient(0): not translated, its class isn't translated (its constants carry values).
        // DendroPotion.DendroPotionType.getDescription(0): not translated, its class isn't translated (its constants carry values).
        // DendroPotion.DendroPotionType.getBasePotionType(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `DirtBucket` (com/dtteam/dynamictrees/item/DirtBucket.java)
    pub mod dirt_bucket {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_1792`. Its static parts are below.
        // DirtBucket.DirtBucket(0): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // DirtBucket.method_7836(3): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // DirtBucket.tryPlaceContainedDirt(3): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
    }

    /// `Seed` (com/dtteam/dynamictrees/item/Seed.java)
    pub mod seed {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_1792`. Its static parts are below.
        // BACKUP_RANDOM: not translated, it uses LazyValue, which isn't translated (it's an interface).
        pub const FORCE_PLANT_KEY: &str = "ForcePlant";
        pub const LIFESPAN_KEY: &str = "Lifespan";
        pub const CODE_KEY: &str = "Code";
        // Seed.Seed(0): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.Seed(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.Seed(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.getSpecies(0): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.onEntityItemUpdate(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.doPlanting(4): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.shouldPlant(3): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.hasForcePlant(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.getTimeToLive(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.getCode(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.getJoCodeForRadius(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.method_7884(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.method_7836(3): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.onItemUsePlantSeed(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.onItemUseFirst(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Seed.onItemUseFlowerPot(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
    }

    /// `Seed.VoluntaryPlantEventResult` (com/dtteam/dynamictrees/item/Seed.java)
    pub mod seed_voluntary_plant_event_result {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct VoluntaryPlantEventResult {
            pub cancelled: bool,
            pub will_plant: bool,
        }
        impl VoluntaryPlantEventResult {
            pub fn new(cancelled: bool, will_plant: bool) -> Self {
                let mut s = Self { cancelled: cancelled, will_plant: will_plant };
                s.cancelled = cancelled;
                s.will_plant = will_plant;
                s
            }
            pub fn cancelled(&self) -> bool {
                return self.cancelled;
            }
            pub fn will_plant(&self) -> bool {
                return self.will_plant;
            }
        }
    }

    /// `Staff` (com/dtteam/dynamictrees/item/Staff.java)
    pub mod staff {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_1792`. Its static parts are below.
        pub const REACH_DISTANCE: f32 = 512.0f32;
        // Staff.Staff(0): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.method_58404(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.method_7879(5): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.method_7836(3): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.method_7884(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.method_31567(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.method_31569(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.isReadOnly(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.setReadOnly(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.setSpecies(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.setCode(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.setRootsCode(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.getSpecies(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.getDamage(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.setDamage(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.getMaxDamage(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.setMaxDamage(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.hasMaxDamage(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.damage(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.getColor(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.setColor(2): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.getCode(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.getRootsCode(1): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
        // Staff.method_7851(4): not translated, its class isn't translated (it extends Minecraft's `class_1792`).
    }

    /// `SeasonalSeedDropChance` (com/dtteam/dynamictrees/loot/condition/SeasonalSeedDropChance.java)
    pub mod seasonal_seed_drop_chance {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_5341`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SeasonalSeedDropChance.SeasonalSeedDropChance(1): not translated, its class isn't translated (it implements Minecraft's `class_5341`).
        // SeasonalSeedDropChance.method_29325(0): not translated, its class isn't translated (it implements Minecraft's `class_5341`).
        // SeasonalSeedDropChance.test(1): not translated, its body couldn't be read: an assert.
        // SeasonalSeedDropChance.seasonalSeedDropChance(0): not translated, it uses Minecraft's `class_210`.
    }

    /// `SpeciesMatches` (com/dtteam/dynamictrees/loot/condition/SpeciesMatches.java)
    pub mod species_matches {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_5341`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SpeciesMatches.SpeciesMatches(1): not translated, its class isn't translated (it implements Minecraft's `class_5341`).
        // SpeciesMatches.method_29325(0): not translated, its class isn't translated (it implements Minecraft's `class_5341`).
        // SpeciesMatches.test(1): not translated, its body couldn't be read: an assert.
        // SpeciesMatches.speciesMatches(1): not translated, it uses Minecraft's `class_210`.
    }

    /// `VoluntarySeedDropChance` (com/dtteam/dynamictrees/loot/condition/VoluntarySeedDropChance.java)
    pub mod voluntary_seed_drop_chance {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_5341`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // VoluntarySeedDropChance.VoluntarySeedDropChance(1): not translated, its class isn't translated (it implements Minecraft's `class_5341`).
        // VoluntarySeedDropChance.method_29325(0): not translated, its class isn't translated (it implements Minecraft's `class_5341`).
        // VoluntarySeedDropChance.test(1): not translated, its body couldn't be read: an assert.
        // VoluntarySeedDropChance.voluntarySeedDropChance(0): not translated, it uses Minecraft's `class_210`.
        // VoluntarySeedDropChance.voluntarySeedDropChance(1): not translated, it uses Minecraft's `class_210`.
    }

    /// `DTLoot` (com/dtteam/dynamictrees/loot/DTLoot.java)
    pub mod dt_loot {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DTLoot {
        }
        impl DTLoot {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn load() {
            dt_loot_parameter_sets::load();
            dt_loot_context_params::load();
        }
    }

    /// `DTLootContextParams` (com/dtteam/dynamictrees/loot/DTLootContextParams.java)
    pub mod dt_loot_context_params {
        use super::*;
        // SPECIES: not translated, it uses Minecraft's `class_169`.
        // FERTILITY: not translated, it uses Minecraft's `class_169`.
        // SEASONAL_SEED_DROP_FACTOR: not translated, it uses Minecraft's `class_169`.
        // VOLUME: not translated, it uses Minecraft's `class_169`.
        #[derive(Clone, Debug)]
        pub struct DTLootContextParams {
        }
        impl DTLootContextParams {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DTLootContextParams.create(1): not translated, it's generic.
        pub fn load() {
        }
    }

    /// `DTLootParameterSets` (com/dtteam/dynamictrees/loot/DTLootParameterSets.java)
    pub mod dt_loot_parameter_sets {
        use super::*;
        // LEAVES: not translated, it uses Minecraft's `class_176`.
        // LEAVES_BLOCK: not translated, it uses Minecraft's `class_176`.
        // VOLUNTARY: not translated, it uses Minecraft's `class_176`.
        // BRANCHES: not translated, it uses Minecraft's `class_176`.
        #[derive(Clone, Debug)]
        pub struct DTLootParameterSets {
        }
        impl DTLootParameterSets {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DTLootParameterSets.register(2): not translated, it uses Minecraft's `class_176`.
        pub fn load() {
        }
    }

    /// `ItemBySpeciesLootPoolEntry` (com/dtteam/dynamictrees/loot/entry/ItemBySpeciesLootPoolEntry.java)
    pub mod item_by_species_loot_pool_entry {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_85`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ItemBySpeciesLootPoolEntry.ItemBySpeciesLootPoolEntry(5): not translated, its class isn't translated (it extends Minecraft's `class_85`).
        // ItemBySpeciesLootPoolEntry.method_29318(0): not translated, its class isn't translated (it extends Minecraft's `class_85`).
        // ItemBySpeciesLootPoolEntry.method_433(2): not translated, its body couldn't be read: an assert.
    }

    /// `SeedItemLootPoolEntry` (com/dtteam/dynamictrees/loot/entry/SeedItemLootPoolEntry.java)
    pub mod seed_item_loot_pool_entry {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_85`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SeedItemLootPoolEntry.SeedItemLootPoolEntry(4): not translated, its class isn't translated (it extends Minecraft's `class_85`).
        // SeedItemLootPoolEntry.method_29318(0): not translated, its class isn't translated (it extends Minecraft's `class_85`).
        // SeedItemLootPoolEntry.method_433(2): not translated, its body couldn't be read: an assert.
        // SeedItemLootPoolEntry.lootTableSeedItem(0): not translated, it uses Minecraft's `class_86`.
    }

    /// `WeightedItemLootPoolEntry` (com/dtteam/dynamictrees/loot/entry/WeightedItemLootPoolEntry.java)
    pub mod weighted_item_loot_pool_entry {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_85`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WeightedItemLootPoolEntry.WeightedItemLootPoolEntry(5): not translated, its class isn't translated (it extends Minecraft's `class_85`).
        // WeightedItemLootPoolEntry.method_29318(0): not translated, its class isn't translated (it extends Minecraft's `class_85`).
        // WeightedItemLootPoolEntry.method_433(2): not translated, its class isn't translated (it extends Minecraft's `class_85`).
    }

    /// `MultiplyByLogsCount` (com/dtteam/dynamictrees/loot/function/MultiplyByLogsCount.java)
    pub mod multiply_by_logs_count {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_120`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MultiplyByLogsCount.MultiplyByLogsCount(1): not translated, its class isn't translated (it extends Minecraft's `class_120`).
        // MultiplyByLogsCount.method_29321(0): not translated, its class isn't translated (it extends Minecraft's `class_120`).
        // MultiplyByLogsCount.method_522(2): not translated, its body couldn't be read: an assert.
        // MultiplyByLogsCount.multiplyByLogsCount(0): not translated, it uses Minecraft's `class_118`.
    }

    /// `MultiplyBySticksCount` (com/dtteam/dynamictrees/loot/function/MultiplyBySticksCount.java)
    pub mod multiply_by_sticks_count {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_120`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MultiplyBySticksCount.MultiplyBySticksCount(1): not translated, its class isn't translated (it extends Minecraft's `class_120`).
        // MultiplyBySticksCount.method_29321(0): not translated, its class isn't translated (it extends Minecraft's `class_120`).
        // MultiplyBySticksCount.method_522(2): not translated, its body couldn't be read: an assert.
        // MultiplyBySticksCount.multiplyBySticksCount(0): not translated, it uses Minecraft's `class_118`.
    }

    /// `MultiplyCount` (com/dtteam/dynamictrees/loot/function/MultiplyCount.java)
    pub mod multiply_count {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_120`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MultiplyCount.MultiplyCount(2): not translated, its class isn't translated (it extends Minecraft's `class_120`).
        // MultiplyCount.method_29321(0): not translated, its class isn't translated (it extends Minecraft's `class_120`).
        // MultiplyCount.method_522(2): not translated, its class isn't translated (it extends Minecraft's `class_120`).
        // MultiplyCount.multiplyCount(0): not translated, it uses Minecraft's `class_118`.
    }

    /// `LootTableSupplier` (com/dtteam/dynamictrees/loot/LootTableSupplier.java)
    pub mod loot_table_supplier {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2960`. Its static parts are below.
        // LootTableSupplier.LootTableSupplier(2): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // LootTableSupplier.get(2): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // LootTableSupplier.getName(1): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // LootTableSupplier.getName(0): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
    }

    /// `BakedModelBlockPottedSapling` (com/dtteam/dynamictrees/model/BakedModelBlockPottedSapling.java)
    pub mod baked_model_block_potted_sapling {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `ForwardingBakedModel`. Its static parts are below.
        // BakedModelBlockPottedSapling.BakedModelBlockPottedSapling(1): not translated, its class isn't translated (it extends Fabric's `ForwardingBakedModel`).
        // BakedModelBlockPottedSapling.isVanillaAdapter(0): not translated, its class isn't translated (it extends Fabric's `ForwardingBakedModel`).
        // BakedModelBlockPottedSapling.method_4707(3): not translated, its class isn't translated (it extends Fabric's `ForwardingBakedModel`).
    }

    /// `BasicBranchBlockBakedModel` (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java)
    pub mod basic_branch_block_baked_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_1087`. Its static parts are below.
        // BasicBranchBlockBakedModel.BasicBranchBlockBakedModel(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.initModels(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.generateSleevePart(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.bakeSleeve(3): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.generateCorePart(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.bakeCore(3): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.getFaceAngle(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.isVanillaAdapter(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.emitBlockQuads(5): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.collectQuads(3): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.maxBranchRadius(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.collectQuads(4): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.getRingQuads(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.getRenderMaterial(0): not translated, it uses Fabric's `RenderMaterial`.
        // BasicBranchBlockBakedModel.emitQuads(4): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.emitItemQuads(3): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.getSourceDir(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.resolveCoreDir(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.getRadius(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.method_4707(3): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.method_4708(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.method_4712(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.method_24304(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.method_4713(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.method_4711(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.method_4709(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // BasicBranchBlockBakedModel.method_4710(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
    }

    /// `BasicRootsBlockBakedModel` (com/dtteam/dynamictrees/model/baked/BasicRootsBlockBakedModel.java)
    pub mod basic_roots_block_baked_model {
        use super::*;
        // The class itself isn't translated: it extends `BasicBranchBlockBakedModel`. Its static parts are below.
        pub const Z_FIGHTING_OFFSET: f32 = 0.001f32;
        // BasicRootsBlockBakedModel.BasicRootsBlockBakedModel(2): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // BasicRootsBlockBakedModel.initRootsModels(0): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // BasicRootsBlockBakedModel.bakeSleeveFace(3): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // BasicRootsBlockBakedModel.getRadius(1): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // BasicRootsBlockBakedModel.collectQuads(4): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
    }

    /// `SurfaceRootBlockBakedModel` (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java)
    pub mod surface_root_block_baked_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_1087`. Its static parts are below.
        // SurfaceRootBlockBakedModel.SurfaceRootBlockBakedModel(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.initModels(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.getRadialHeight(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.bakeSleeve(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.bakeVert(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.bakeCore(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.getFaceAngle(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.getUVs(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.modUV(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.AABBLimits(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.isVanillaAdapter(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.emitBlockQuads(5): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.collectQuads(3): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.emitQuads(4): not translated, it uses Minecraft's `class_2350`.
        // SurfaceRootBlockBakedModel.getRenderMaterial(0): not translated, it uses Fabric's `RenderMaterial`.
        // SurfaceRootBlockBakedModel.emitItemQuads(3): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.getSourceDir(2): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.resolveCoreDir(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.getRadius(1): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.method_4707(3): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.method_4708(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.method_4712(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.method_24304(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.method_4713(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.method_4711(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.method_4709(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
        // SurfaceRootBlockBakedModel.method_4710(0): not translated, its class isn't translated (it implements Minecraft's `class_1087`).
    }

    /// `ThickBranchBlockBakedModel` (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java)
    pub mod thick_branch_block_baked_model {
        use super::*;
        // The class itself isn't translated: it extends `BasicBranchBlockBakedModel`. Its static parts are below.
        // ThickBranchBlockBakedModel.ThickBranchBlockBakedModel(3): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // ThickBranchBlockBakedModel.initThickModels(0): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // ThickBranchBlockBakedModel.bakeTrunkBark(3): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // ThickBranchBlockBakedModel.bakeTrunkRings(3): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // ThickBranchBlockBakedModel.getRingsUvs(3): not translated, it uses Minecraft's `class_2350`.
        // ThickBranchBlockBakedModel.getUVs(2): not translated, it uses Minecraft's `class_238`.
        pub fn mod_uv(uvs: &mut [f32]) -> Vec<f32> {
            uvs[0] = (((uvs[0] as i32) & 15) as f32);
            uvs[1] = (((uvs[1] as i32) & 15) as f32);
            uvs[2] = (((((uvs[2] as i32) - 1) & 15) + 1) as f32);
            uvs[3] = (((((uvs[3] as i32) - 1) & 15) + 1) as f32);
            return uvs.to_vec();
        }
        // ThickBranchBlockBakedModel.aabbLimits(1): not translated, it uses Minecraft's `class_238`.
        // ThickBranchBlockBakedModel.maxBranchRadius(0): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // ThickBranchBlockBakedModel.collectQuads(4): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
        // ThickBranchBlockBakedModel.getRingQuads(1): not translated, its class isn't translated (it extends `BasicBranchBlockBakedModel`).
    }

    /// `BranchBlockUnbakedModel` (com/dtteam/dynamictrees/model/BranchBlockUnbakedModel.java)
    pub mod branch_block_unbaked_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_1100`. Its static parts are below.
        // BranchBlockUnbakedModel.BranchBlockUnbakedModel(4): not translated, its class isn't translated (it implements Minecraft's `class_1100`).
        // BranchBlockUnbakedModel.method_4755(0): not translated, its class isn't translated (it implements Minecraft's `class_1100`).
        // BranchBlockUnbakedModel.method_45785(1): not translated, its class isn't translated (it implements Minecraft's `class_1100`).
        // BranchBlockUnbakedModel.method_4753(3): not translated, its class isn't translated (it implements Minecraft's `class_1100`).
    }

    /// `DTModelLoadingPlugin` (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java)
    pub mod dt_model_loading_plugin {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModelLoadingPlugin`. Its static parts are below.
        // POTTED_SAPLING_MODEL: not translated, it uses Minecraft's `class_2960`.
        // BRANCH_MODEL_CACHE: not translated, it uses Minecraft's `class_2960`.
        // ROOT_MODEL_CACHE: not translated, it uses Minecraft's `class_2960`.
        // UNDERGROUND_ROOTS_MODEL_CACHE: not translated, it uses Minecraft's `class_2960`.
        pub static MODELS_INITIALIZED: JCell<bool> = JCell::new(false);
        // DTModelLoadingPlugin.onInitializeModelLoader(1): not translated, its class isn't translated (it implements Fabric's `ModelLoadingPlugin`).
        // DTModelLoadingPlugin.initBranchModels(1): not translated, its class isn't translated (it implements Fabric's `ModelLoadingPlugin`).
        // DTModelLoadingPlugin.createBranchModel(4): not translated, its class isn't translated (it implements Fabric's `ModelLoadingPlugin`).
        // DTModelLoadingPlugin.createRootModel(2): not translated, its class isn't translated (it implements Fabric's `ModelLoadingPlugin`).
        // DTModelLoadingPlugin.createRootsBlockModel(3): not translated, its class isn't translated (it implements Fabric's `ModelLoadingPlugin`).
        // DTModelLoadingPlugin.createFallbackRootsModel(3): not translated, its class isn't translated (it implements Fabric's `ModelLoadingPlugin`).
        // DTModelLoadingPlugin.modifyModelAfterBake(2): not translated, its class isn't translated (it implements Fabric's `ModelLoadingPlugin`).
    }

    /// `FallingTreeEntityModel` (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java)
    pub mod falling_tree_entity_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_583`. Its static parts are below.
        // FallingTreeEntityModel.FallingTreeEntityModel(1): not translated, its class isn't translated (it extends Minecraft's `class_583`).
        // FallingTreeEntityModel.getQuads(0): not translated, its class isn't translated (it extends Minecraft's `class_583`).
        // FallingTreeEntityModel.getEntityId(0): not translated, its class isn't translated (it extends Minecraft's `class_583`).
        // FallingTreeEntityModel.getBrightness(1): not translated, it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`).
        // FallingTreeEntityModel.generateTreeQuads(1): not translated, its class isn't translated (it extends Minecraft's `class_583`).
        // FallingTreeEntityModel.setupAnim(6): not translated, its class isn't translated (it extends Minecraft's `class_583`).
        // FallingTreeEntityModel.method_2828(5): not translated, its class isn't translated (it extends Minecraft's `class_583`).
        // FallingTreeEntityModel.toTreeQuadData(2): not translated, it uses Minecraft's `class_777`.
        // FallingTreeEntityModel.toTreeQuadData(3): not translated, it uses Minecraft's `class_777`.
    }

    /// `FallingTreeEntityModel.TreeQuadData` (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java)
    pub mod falling_tree_entity_model_tree_quad_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_777`. Its static parts are below.
        // FallingTreeEntityModel.TreeQuadData.TreeQuadData(3): not translated, its class isn't translated (it uses Minecraft's `class_777`).
        // FallingTreeEntityModel.TreeQuadData.bakedQuad(0): not translated, its class isn't translated (it uses Minecraft's `class_777`).
        // FallingTreeEntityModel.TreeQuadData.color(0): not translated, its class isn't translated (it uses Minecraft's `class_777`).
        // FallingTreeEntityModel.TreeQuadData.state(0): not translated, its class isn't translated (it uses Minecraft's `class_777`).
    }

    /// `FallingTreeEntityModelFabric` (com/dtteam/dynamictrees/model/FallingTreeEntityModelFabric.java)
    pub mod falling_tree_entity_model_fabric {
        use super::*;
        // The class itself isn't translated: it extends `FallingTreeEntityModel`. Its static parts are below.
        // FallingTreeEntityModelFabric.FallingTreeEntityModelFabric(1): not translated, its class isn't translated (it extends `FallingTreeEntityModel`).
        // FallingTreeEntityModelFabric.generateTreeQuads(1): not translated, its class isn't translated (it extends `FallingTreeEntityModel`).
        // FallingTreeEntityModelFabric.getBottomRingQuads(4): not translated, its class isn't translated (it extends `FallingTreeEntityModel`).
        // FallingTreeEntityModelFabric.getBranchQuadsWithConnections(7): not translated, its class isn't translated (it extends `FallingTreeEntityModel`).
        // FallingTreeEntityModelFabric.getQuadsWithOffset(4): not translated, its class isn't translated (it extends `FallingTreeEntityModel`).
        // FallingTreeEntityModelFabric.offsetAllQuads(2): not translated, its class isn't translated (it extends `FallingTreeEntityModel`).
        // FallingTreeEntityModelFabric.offsetQuad(2): not translated, its class isn't translated (it extends `FallingTreeEntityModel`).
    }

    /// `FallingTreeEntityModelTrackerCache` (com/dtteam/dynamictrees/model/FallingTreeEntityModelTrackerCache.java)
    pub mod falling_tree_entity_model_tracker_cache {
        use super::*;
        // models: not translated, it uses FallingTreeEntityModel, which isn't translated (it extends Minecraft's `class_583`).
        #[derive(Clone, Debug)]
        pub struct FallingTreeEntityModelTrackerCache {
        }
        impl FallingTreeEntityModelTrackerCache {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FallingTreeEntityModelTrackerCache.getOrCreateModel(1): not translated, it uses FallingTreeEntityModel, which isn't translated (it extends Minecraft's `class_583`).
        // FallingTreeEntityModelTrackerCache.cleanupModels(2): not translated, it uses Minecraft's `class_1937`.
        // FallingTreeEntityModelTrackerCache.cleanupModels(1): not translated, it uses Minecraft's `class_1937`.
    }

    /// `ModelTracker` (com/dtteam/dynamictrees/model/ModelTracker.java)
    pub mod model_tracker {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ModelTracker.modelCleanup(0): not translated, its class isn't translated (it's an interface).
    }

    /// `ClientServices` (com/dtteam/dynamictrees/platform/ClientServices.java)
    pub mod client_services {
        use super::*;
        // CLIENT: not translated, it uses IClientHelper, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ClientServices {
        }
        impl ClientServices {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientServices.load(1): not translated, it's generic.
    }

    /// `FabricClientHelper` (com/dtteam/dynamictrees/platform/FabricClientHelper.java)
    pub mod fabric_client_helper {
        use super::*;
        // The class itself isn't translated: it implements `IClientHelper` (com.dtteam.dynamictrees.platform.services). Its static parts are below.
        // FabricClientHelper.getPixelRGBA(3): not translated, its class isn't translated (it implements `IClientHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricClientHelper.newFallingTreeEntityModel(1): not translated, its class isn't translated (it implements `IClientHelper` (com.dtteam.dynamictrees.platform.services)).
    }

    /// `FabricCompatHelper` (com/dtteam/dynamictrees/platform/FabricCompatHelper.java)
    pub mod fabric_compat_helper {
        use super::*;
        // The class itself isn't translated: it implements `ICompatHelper` (com.dtteam.dynamictrees.platform.services). Its static parts are below.
        // FabricCompatHelper.registerSereneSeasonsSeasonProvider(0): not translated, its class isn't translated (it implements `ICompatHelper` (com.dtteam.dynamictrees.platform.services)).
    }

    /// `FabricConfigHelper` (com/dtteam/dynamictrees/platform/FabricConfigHelper.java)
    pub mod fabric_config_helper {
        use super::*;
        // The class itself isn't translated: it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services). Its static parts are below.
        // FabricConfigHelper.getConfig(2): not translated, it's generic.
        // FabricConfigHelper.getBoolConfig(1): not translated, its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricConfigHelper.getIntConfig(1): not translated, its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricConfigHelper.getDoubleConfig(1): not translated, its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricConfigHelper.getStringConfig(1): not translated, its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricConfigHelper.getEnumConfig(2): not translated, it's generic.
        // FabricConfigHelper.getStringListConfig(1): not translated, its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricConfigHelper.isServerConfigLoaded(0): not translated, its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricConfigHelper.isCommonConfigLoaded(0): not translated, its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricConfigHelper.isClientConfigLoaded(0): not translated, its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)).
    }

    /// `FabricEventHelper` (com/dtteam/dynamictrees/platform/FabricEventHelper.java)
    pub mod fabric_event_helper {
        use super::*;
        // The class itself isn't translated: it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services). Its static parts are below.
        // FabricEventHelper.postRegistryEvent(1): not translated, it's generic.
        // FabricEventHelper.postTypedRegistryEvent(1): not translated, it's generic.
        // FabricEventHelper.postAddResourceLoadersEventPre(1): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.postAddResourceLoadersEventPost(1): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.postJsonDeserializerRegistryEvent(0): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.postApplierEvent(3): not translated, it's generic.
        // FabricEventHelper.postBiomeEntryApplierEvent(2): not translated, it's generic.
        // FabricEventHelper.postCancellationApplierEvent(2): not translated, it's generic.
        // FabricEventHelper.postSpeciesPostGenerationEvent(1): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.postTransitionSaplingToTreeEvent(3): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.canCropGrow(4): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.cropGrowPost(3): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.postBiomeSuitabilityEvent(4): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.postSeedVoluntaryPlantEvent(4): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricEventHelper.postPoissonDiscProviderCreateEvent(2): not translated, its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)).
    }

    /// `FabricInteractionHelper` (com/dtteam/dynamictrees/platform/FabricInteractionHelper.java)
    pub mod fabric_interaction_helper {
        use super::*;
        // The class itself isn't translated: it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services). Its static parts are below.
        // FabricInteractionHelper.canToolAxeStrip(1): not translated, its class isn't translated (it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricInteractionHelper.canToolAxeDig(1): not translated, its class isn't translated (it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricInteractionHelper.setSeedItemEntityLifespan(2): not translated, its class isn't translated (it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricInteractionHelper.blockDestroyByPlayer(6): not translated, its class isn't translated (it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)).
    }

    /// `FabricMiscHelper` (com/dtteam/dynamictrees/platform/FabricMiscHelper.java)
    pub mod fabric_misc_helper {
        use super::*;
        // The class itself isn't translated: it implements `IMiscHelper` (com.dtteam.dynamictrees.platform.services). Its static parts are below.
        // currentServer: not translated, it uses Minecraft's `MinecraftServer`.
        // FabricMiscHelper.debugSpeciesRegistry(0): not translated, it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // FabricMiscHelper.isLevelRestoringBlockSnapshots(1): not translated, its class isn't translated (it implements `IMiscHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricMiscHelper.getCurrentServer(0): not translated, its class isn't translated (it implements `IMiscHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricMiscHelper.newDTBiomeHolderSet(0): not translated, its class isn't translated (it implements `IMiscHelper` (com.dtteam.dynamictrees.platform.services)).
    }

    /// `FabricPlatformHelper` (com/dtteam/dynamictrees/platform/FabricPlatformHelper.java)
    pub mod fabric_platform_helper {
        use super::*;
        // The class itself isn't translated: it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services). Its static parts are below.
        // FabricPlatformHelper.getPlatformName(0): not translated, its class isn't translated (it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricPlatformHelper.isModLoaded(1): not translated, its class isn't translated (it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricPlatformHelper.isDevelopmentEnvironment(0): not translated, its class isn't translated (it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricPlatformHelper.getMods(0): not translated, its class isn't translated (it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)).
    }

    /// `FabricRegistryHelper` (com/dtteam/dynamictrees/platform/FabricRegistryHelper.java)
    pub mod fabric_registry_helper {
        use super::*;
        // The class itself isn't translated: it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services). Its static parts are below.
        pub static REGISTRIES_INSTANCE: LazyLock<JCell<Option<registry_loader::RegistryLoader>>> = LazyLock::new(|| JCell::new(None));
        // FabricRegistryHelper.getRegistryLoader(0): not translated, its class isn't translated (it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricRegistryHelper.newRegistryHandler(0): not translated, its class isn't translated (it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricRegistryHelper.getPottedSaplingBlockEntity(0): not translated, its class isn't translated (it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)).
        // FabricRegistryHelper.newPottedSaplingBlockEntity(2): not translated, its class isn't translated (it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)).
    }

    /// `Services` (com/dtteam/dynamictrees/platform/Services.java)
    pub mod services {
        use super::*;
        // PLATFORM: not translated, it uses IPlatformHelper, which isn't translated (it's an interface).
        // REGISTRY: not translated, it uses IRegistryHelper, which isn't translated (it's an interface).
        // CONFIG: not translated, it uses IConfigHelper, which isn't translated (it's an interface).
        // INTERACTION: not translated, it uses IInteractionHelper, which isn't translated (it's an interface).
        // MISC: not translated, it uses IMiscHelper, which isn't translated (it's an interface).
        // COMPAT: not translated, it uses ICompatHelper, which isn't translated (it's an interface).
        // EVENT: not translated, it uses IEventHelper, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct Services {
        }
        impl Services {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Services.load(1): not translated, it's generic.
    }

    /// `IClientHelper` (com/dtteam/dynamictrees/platform/services/IClientHelper.java)
    pub mod i_client_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IClientHelper.getPixelRGBA(3): not translated, it has no body.
        // IClientHelper.newFallingTreeEntityModel(1): not translated, it has no body.
    }

    /// `ICompatHelper` (com/dtteam/dynamictrees/platform/services/ICompatHelper.java)
    pub mod i_compat_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICompatHelper.registerSereneSeasonsSeasonProvider(0): not translated, it has no body.
    }

    /// `IConfigHelper` (com/dtteam/dynamictrees/platform/services/IConfigHelper.java)
    pub mod i_config_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IConfigHelper.getBoolConfig(1): not translated, it has no body.
        // IConfigHelper.getIntConfig(1): not translated, it has no body.
        // IConfigHelper.getDoubleConfig(1): not translated, it has no body.
        // IConfigHelper.getStringConfig(1): not translated, it has no body.
        // IConfigHelper.getEnumConfig(2): not translated, it has no body.
        // IConfigHelper.getStringListConfig(1): not translated, it has no body.
        // IConfigHelper.isServerConfigLoaded(0): not translated, it has no body.
        // IConfigHelper.isCommonConfigLoaded(0): not translated, it has no body.
        // IConfigHelper.isClientConfigLoaded(0): not translated, it has no body.
    }

    /// `IEventHelper` (com/dtteam/dynamictrees/platform/services/IEventHelper.java)
    pub mod i_event_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEventHelper.postRegistryEvent(1): not translated, it has no body.
        // IEventHelper.postTypedRegistryEvent(1): not translated, it has no body.
        // IEventHelper.postAddResourceLoadersEventPre(1): not translated, it has no body.
        // IEventHelper.postAddResourceLoadersEventPost(1): not translated, it has no body.
        // IEventHelper.postJsonDeserializerRegistryEvent(0): not translated, it has no body.
        // IEventHelper.postApplierEvent(3): not translated, it has no body.
        // IEventHelper.postBiomeEntryApplierEvent(2): not translated, it has no body.
        // IEventHelper.postCancellationApplierEvent(2): not translated, it has no body.
        // IEventHelper.postSpeciesPostGenerationEvent(1): not translated, it has no body.
        // IEventHelper.postTransitionSaplingToTreeEvent(3): not translated, it has no body.
        // IEventHelper.canCropGrow(4): not translated, it has no body.
        // IEventHelper.cropGrowPost(3): not translated, it has no body.
        // IEventHelper.postBiomeSuitabilityEvent(4): not translated, it has no body.
        // IEventHelper.postSeedVoluntaryPlantEvent(4): not translated, it has no body.
        // IEventHelper.postPoissonDiscProviderCreateEvent(2): not translated, it has no body.
    }

    /// `IInteractionHelper` (com/dtteam/dynamictrees/platform/services/IInteractionHelper.java)
    pub mod i_interaction_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IInteractionHelper.canToolAxeStrip(1): not translated, it has no body.
        // IInteractionHelper.canToolAxeDig(1): not translated, it has no body.
        // IInteractionHelper.setSeedItemEntityLifespan(2): not translated, it has no body.
        // IInteractionHelper.blockDestroyByPlayer(6): not translated, it has no body.
    }

    /// `IMiscHelper` (com/dtteam/dynamictrees/platform/services/IMiscHelper.java)
    pub mod i_misc_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IMiscHelper.isLevelRestoringBlockSnapshots(1): not translated, it has no body.
        // IMiscHelper.getCurrentServer(0): not translated, it has no body.
        // IMiscHelper.newDTBiomeHolderSet(0): not translated, it has no body.
    }

    /// `IPlatformHelper` (com/dtteam/dynamictrees/platform/services/IPlatformHelper.java)
    pub mod i_platform_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPlatformHelper.getPlatformName(0): not translated, it has no body.
        // IPlatformHelper.isModLoaded(1): not translated, it has no body.
        // IPlatformHelper.isDevelopmentEnvironment(0): not translated, it has no body.
        // IPlatformHelper.getEnvironmentName(0): not translated, its class isn't translated (it's an interface).
        // IPlatformHelper.getMods(0): not translated, it has no body.
    }

    /// `IRegistryHelper` (com/dtteam/dynamictrees/platform/services/IRegistryHelper.java)
    pub mod i_registry_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IRegistryHelper.getRegistryLoader(0): not translated, it has no body.
        // IRegistryHelper.newRegistryHandler(0): not translated, it has no body.
        // IRegistryHelper.getPottedSaplingBlockEntity(0): not translated, it has no body.
        // IRegistryHelper.newPottedSaplingBlockEntity(2): not translated, it has no body.
    }

    /// `DendroBrewingMix` (com/dtteam/dynamictrees/recipe/DendroBrewingMix.java)
    pub mod dendro_brewing_mix {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_1799`. Its static parts are below.
        // DendroBrewingMix.DendroBrewingMix(3): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
        // DendroBrewingMix.isInput(1): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
        // DendroBrewingMix.isIngredient(1): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
        // DendroBrewingMix.getOutput(2): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
        // DendroBrewingMix.input(0): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
        // DendroBrewingMix.ingredient(0): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
        // DendroBrewingMix.output(0): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
    }

    /// `DendroPotionRecipeHandler` (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java)
    pub mod dendro_potion_recipe_handler {
        use super::*;
        // brewingRecipes: not translated, it uses DendroBrewingMix, which isn't translated (it uses Minecraft's `class_1799`).
        // ingredients: not translated, it uses Minecraft's `class_1799`.
        #[derive(Clone, Debug)]
        pub struct DendroPotionRecipeHandler {
        }
        impl DendroPotionRecipeHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DendroPotionRecipeHandler.isIngredient(1): not translated, it uses Minecraft's `class_1799`.
        // DendroPotionRecipeHandler.getAllDendroRecipes(0): not translated, it uses DendroBrewingMix, which isn't translated (it uses Minecraft's `class_1799`).
        // DendroPotionRecipeHandler.setPotion(2): not translated, it uses Minecraft's `class_1799`.
        // DendroPotionRecipeHandler.getRecipe(2): not translated, it uses DendroBrewingMix, which isn't translated (it uses Minecraft's `class_1799`).
        // DendroPotionRecipeHandler.getRecipe(3): not translated, it uses DendroBrewingMix, which isn't translated (it uses Minecraft's `class_1799`).
        // DendroPotionRecipeHandler.getPotionStack(1): not translated, it uses Minecraft's `class_1799`.
    }

    /// `MegaSeedRecipe` (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java)
    pub mod mega_seed_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_1852`. Its static parts are below.
        // MegaSeedRecipe.MegaSeedRecipe(1): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
        // MegaSeedRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
        // MegaSeedRecipe.assemble(2): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
        // MegaSeedRecipe.atLeastHasSeed(1): not translated, it uses Minecraft's `class_9694`.
        // MegaSeedRecipe.recipeMatchCondition(2): not translated, it uses Minecraft's `class_9694`.
        // MegaSeedRecipe.allItemsMatchSeed(2): not translated, it uses Minecraft's `class_9694`.
        // MegaSeedRecipe.nonEmptyStacksStream(1): not translated, it uses `Stream` (java.util.stream).
        // MegaSeedRecipe.method_8113(2): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
        // MegaSeedRecipe.method_8119(0): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
    }

    /// `SeedConversionRecipe` (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java)
    pub mod seed_conversion_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_1852`. Its static parts are below.
        // SeedConversionRecipe.SeedConversionRecipe(1): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
        // SeedConversionRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
        // SeedConversionRecipe.assemble(2): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
        // SeedConversionRecipe.seedToSaplingCondition(3): not translated, it uses Minecraft's `class_9694`.
        // SeedConversionRecipe.saplingToSeedCondition(2): not translated, it uses Minecraft's `class_9694`.
        // SeedConversionRecipe.hasExactCount(2): not translated, it uses Minecraft's `class_9694`.
        // SeedConversionRecipe.containsAllIngredients(2): not translated, it uses Minecraft's `class_9694`.
        // SeedConversionRecipe.containsOneOfItem(2): not translated, it uses Minecraft's `class_9694`.
        // SeedConversionRecipe.hasDirtBucket(1): not translated, it uses Minecraft's `class_9694`.
        // SeedConversionRecipe.notEmptyInput(1): not translated, it uses `Stream` (java.util.stream).
        // SeedConversionRecipe.method_8113(2): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
        // SeedConversionRecipe.method_8119(0): not translated, its class isn't translated (it extends Minecraft's `class_1852`).
    }

    /// `DTRegistries` (com/dtteam/dynamictrees/registry/DTRegistries.java)
    pub mod dt_registries {
        use super::*;
        // CREATIVE_TAB_ITEMS: not translated, it uses `LinkedList` (java.util).
        // POTTED_SAPLING: not translated, it uses PottedSaplingBlock, which isn't translated (it extends Minecraft's `class_2237`).
        // TRUNK_SHELL: not translated, it uses TrunkShellBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // DENDRO_POTION: not translated, it uses DendroPotion, which isn't translated (it extends Minecraft's `class_1792`).
        // DIRT_BUCKET: not translated, it uses DirtBucket, which isn't translated (it extends Minecraft's `class_1792`).
        // STAFF: not translated, it uses Staff, which isn't translated (it extends Minecraft's `class_1792`).
        // DT_CREATIVE_TAB: not translated, it uses Minecraft's `class_1761`.
        // FALLING_TREE: not translated, it uses Minecraft's `class_1299`.
        // LINGERING_EFFECTOR: not translated, it uses Minecraft's `class_1299`.
        // SPECIES_BLOCK_ENTITY: not translated, it uses Minecraft's `class_2591`.
        // POTTED_SAPLING_BLOCK_ENTITY: not translated, it uses Minecraft's `class_2591`.
        // FALLING_TREE_HIT_WATER: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_BIG_START: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_BIG_END: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_MEDIUM_START: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_MEDIUM_END: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_SMALL_HIT_WATER: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_SMALL_END: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_SMALL_END_BARE: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_FUNGUS_START: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_FUNGUS_END: not translated, it uses Minecraft's `class_3414`.
        // FALLING_TREE_FUNGUS_SMALL_END: not translated, it uses Minecraft's `class_3414`.
        // STAFF_HANDLE_COLOR_DATA_COMPONENT: not translated, it uses Minecraft's `class_9331`.
        // STAFF_CRYSTAL_COLOR_DATA_COMPONENT: not translated, it uses Minecraft's `class_9331`.
        // JOCODE_DATA_COMPONENT: not translated, it uses Minecraft's `class_9331`.
        // ROOTS_JOCODE_DATA_COMPONENT: not translated, it uses Minecraft's `class_9331`.
        // READ_ONLY_DATA_COMPONENT: not translated, it uses Minecraft's `class_9331`.
        // SPECIES_DATA_COMPONENT: not translated, it uses Minecraft's `class_9331`.
        // DENDRO_POTION_INDEX_DATA_COMPONENT: not translated, it uses Minecraft's `class_9331`.
        // HEX_COLOR: not translated, it uses Minecraft's `class_2319`.
        // SPECIES_MATCHES: not translated, it uses Minecraft's `class_5342`.
        // SEASONAL_SEED_DROP_CHANCE: not translated, it uses Minecraft's `class_5342`.
        // VOLUNTARY_SEED_DROP_CHANCE: not translated, it uses Minecraft's `class_5342`.
        // ITEM_BY_SPECIES: not translated, it uses Minecraft's `class_5338`.
        // SEED_ITEM: not translated, it uses Minecraft's `class_5338`.
        // WEIGHTED_ITEM: not translated, it uses Minecraft's `class_5338`.
        // MULTIPLY_COUNT: not translated, it uses Minecraft's `class_5339`.
        // MULTIPLY_LOGS_COUNT: not translated, it uses Minecraft's `class_5339`.
        // MULTIPLY_STICKS_COUNT: not translated, it uses Minecraft's `class_5339`.
        // DYNAMIC_TREE_CONFIGURED_FEATURE: not translated, it uses Minecraft's `class_5321`.
        // CAVE_ROOTED_TREE_CONFIGURED_FEATURE: not translated, it uses Minecraft's `class_5321`.
        // DYNAMIC_TREE_PLACED_FEATURE: not translated, it uses Minecraft's `class_5321`.
        // CAVE_ROOTED_TREE_PLACED_FEATURE: not translated, it uses Minecraft's `class_5321`.
        // CAVE_ROOTED_TREE_PLACEMENT_MODIFIER_TYPE: not translated, it uses Minecraft's `class_6798`.
        // DYNAMIC_TREE_FEATURE: not translated, it uses DynamicTreeFeature, which isn't translated (it extends Minecraft's `class_3031`).
        // CAVE_ROOTED_TREE_FEATURE: not translated, it uses CaveRootedTreeFeature, which isn't translated (it extends `DynamicTreeFeature`).
        // REPLACE_NYLIUM_FUNGI_BLOCK_STATE_PROVIDER_TYPE: not translated, it uses Minecraft's `class_4652`.
        // CANCEL_VANILLA_VILLAGE_TREE_STRUCTURE_POOL_ELEMENT_TYPE: not translated, it uses Minecraft's `class_3816`.
        // TREE_STRUCTURE_POOL_ELEMENT_TYPE: not translated, it uses Minecraft's `class_3816`.
        // SEED_CONVERSION_RECIPE_TYPE: not translated, it uses Minecraft's `class_1865`.
        // MEGA_SEED_RECIPE_TYPE: not translated, it uses Minecraft's `class_1865`.
        #[derive(Clone, Debug)]
        pub struct DTRegistries {
        }
        impl DTRegistries {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DTRegistries.setup(0): not translated, it calls DTRegistries.setupConnectables, which isn't translated.
        // DTRegistries.setupConnectables(0): not translated, it uses Minecraft's `class_2248`.
        // DTRegistries.getAllRootyBlocks(0): not translated, it uses Minecraft's `class_2248`.
    }

    /// `FabricRegistryHandler` (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java)
    pub mod fabric_registry_handler {
        use super::*;
        // The class itself isn't translated: it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // FabricRegistryHandler.FabricRegistryHandler(0): not translated, its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)).
        // FabricRegistryHandler.FabricRegistryHandler(1): not translated, its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)).
        // FabricRegistryHandler.setup(1): not translated, it uses RegistryHandler, which isn't translated (it extends `RegistryEntry`).
        // FabricRegistryHandler.warnIfInvalid(2): not translated, its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)).
        // FabricRegistryHandler.getBlock(1): not translated, its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)).
        // FabricRegistryHandler.getItem(1): not translated, its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)).
        // FabricRegistryHandler.putBlock(2): not translated, it's generic.
        // FabricRegistryHandler.putItem(2): not translated, it's generic.
    }

    /// `FabricRegistryHandler.RegisterEventHandler` (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java)
    pub mod fabric_registry_handler_register_event_handler {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
    }

    /// `FabricRegistryLoader` (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java)
    pub mod fabric_registry_loader {
        use super::*;
        // The class itself isn't translated: it extends `RegistryLoader`. Its static parts are below.
        // FabricRegistryLoader.setup(0): not translated, it calls DendroPotionRecipeHandler.getAllDendroRecipes, which isn't translated.
        // FabricRegistryLoader.registerBlock(2): not translated, it's generic.
        // FabricRegistryLoader.registerItem(2): not translated, it's generic.
        // FabricRegistryLoader.registerRecipeType(2): not translated, it's generic.
        // FabricRegistryLoader.registerCreativeTab(4): not translated, its class isn't translated (it extends `RegistryLoader`).
        // FabricRegistryLoader.registerEntity(3): not translated, it's generic.
        // FabricRegistryLoader.registerBlockEntity(3): not translated, it's generic.
        // FabricRegistryLoader.registerSoundEvent(1): not translated, its class isn't translated (it extends `RegistryLoader`).
        // FabricRegistryLoader.registerDataComponentType(2): not translated, it's generic.
        // FabricRegistryLoader.registerCommandArgumentType(3): not translated, it's generic.
        // FabricRegistryLoader.registerLootConditionType(2): not translated, its class isn't translated (it extends `RegistryLoader`).
        // FabricRegistryLoader.registerLootPoolEntryType(2): not translated, its class isn't translated (it extends `RegistryLoader`).
        // FabricRegistryLoader.registerLootFunctionType(2): not translated, it's generic.
        // FabricRegistryLoader.registerPlacementModifierType(2): not translated, it's generic.
        // FabricRegistryLoader.registerFeature(2): not translated, it's generic.
        // FabricRegistryLoader.registerBlockStateProviderType(2): not translated, it's generic.
        // FabricRegistryLoader.registerStructurePoolElementType(2): not translated, it's generic.
    }

    /// `RegistryEvent` (com/dtteam/dynamictrees/registry/RegistryEvent.java)
    pub mod registry_event {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `RegistryLoader` (com/dtteam/dynamictrees/registry/RegistryLoader.java)
    pub mod registry_loader {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RegistryLoader {
        }
        impl RegistryLoader {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // RegistryLoader.registerCreativeTab(4): not translated, it has no body.
            // RegistryLoader.registerBlock(2): not translated, it has no body.
            // RegistryLoader.registerItem(2): not translated, it has no body.
            // RegistryLoader.registerRecipeType(2): not translated, it has no body.
            // RegistryLoader.registerEntity(3): not translated, it has no body.
            // RegistryLoader.registerBlockEntity(3): not translated, it has no body.
            // RegistryLoader.registerSoundEvent(1): not translated, it has no body.
            // RegistryLoader.registerDataComponentType(2): not translated, it has no body.
            // RegistryLoader.registerCommandArgumentType(3): not translated, it has no body.
            // RegistryLoader.registerLootConditionType(2): not translated, it has no body.
            // RegistryLoader.registerLootPoolEntryType(2): not translated, it has no body.
            // RegistryLoader.registerLootFunctionType(2): not translated, it has no body.
            // RegistryLoader.registerPlacementModifierType(2): not translated, it has no body.
            // RegistryLoader.registerFeature(2): not translated, it has no body.
            // RegistryLoader.registerBlockStateProviderType(2): not translated, it has no body.
            // RegistryLoader.registerStructurePoolElementType(2): not translated, it has no body.
        }
    }

    /// `BranchConnectables` (com/dtteam/dynamictrees/systems/BranchConnectables.java)
    pub mod branch_connectables {
        use super::*;
        // connectablesMap: not translated, it uses Minecraft's `class_2248`.
        #[derive(Clone, Debug)]
        pub struct BranchConnectables {
        }
        impl BranchConnectables {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BranchConnectables.makeBlockConnectable(2): not translated, it uses Minecraft's `class_2248`.
        // BranchConnectables.isBlockConnectable(1): not translated, it uses Minecraft's `class_2248`.
        // BranchConnectables.getConnectionRadiusForBlock(4): not translated, it uses Minecraft's `class_2680`.
    }

    /// `AcaciaLeafCell` (com/dtteam/dynamictrees/systems/cell/AcaciaLeafCell.java)
    pub mod acacia_leaf_cell {
        use super::*;
        // The class itself isn't translated: it extends `MatrixCell`. Its static parts are below.
        pub static VAL_MAP: LazyLock<Vec<i8>> = LazyLock::new(|| vec![0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 3i8, 3i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 4i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 4i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 4i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 4i8, 0i8, 0i8, 0i8]);
        // AcaciaLeafCell.AcaciaLeafCell(1): not translated, its class isn't translated (it extends `MatrixCell`).
    }

    /// `CellKits` (com/dtteam/dynamictrees/systems/cell/CellKits.java)
    pub mod cell_kits {
        use super::*;
        // DECIDUOUS: not translated, its value couldn't be read: an anonymous class.
        // CONIFER: not translated, its value couldn't be read: an anonymous class.
        // ACACIA: not translated, its value couldn't be read: an anonymous class.
        // DARK_OAK: not translated, its value couldn't be read: an anonymous class.
        // BARE: not translated, its value couldn't be read: an anonymous class.
        // PALM: not translated, its value couldn't be read: an anonymous class.
        // NETHER_FUNGUS: not translated, its value couldn't be read: an anonymous class.
        // AZALEA: not translated, its value couldn't be read: an anonymous class.
        #[derive(Clone, Debug)]
        pub struct CellKits {
        }
        impl CellKits {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CellKits.register(1): not translated, it uses Registry, which isn't translated (it's an interface).
        pub fn solve_cell(nv: &[i32], solution: &[i16]) -> i32 {
            for &d_v in solution.iter() {
                let d = (d_v as i32);
                if nv[((d >> 8) & 15) as usize] >= ((d >> 4) & 15) {
                    return d & 15;
                }
            }
            return 0;
        }
    }

    /// `CellKits.BasicSolver` (com/dtteam/dynamictrees/systems/cell/CellKits.java)
    pub mod cell_kits_basic_solver {
        use super::*;
        // The class itself isn't translated: it implements `CellSolver` (com.dtteam.dynamictrees.api.cell). Its static parts are below.
        // CellKits.BasicSolver.BasicSolver(1): not translated, its class isn't translated (it implements `CellSolver` (com.dtteam.dynamictrees.api.cell)).
        // CellKits.BasicSolver.solve(1): not translated, its class isn't translated (it implements `CellSolver` (com.dtteam.dynamictrees.api.cell)).
    }

    /// `ConiferBranchCell` (com/dtteam/dynamictrees/systems/cell/ConiferBranchCell.java)
    pub mod conifer_branch_cell {
        use super::*;
        // The class itself isn't translated: it implements `Cell` (com.dtteam.dynamictrees.api.cell). Its static parts are below.
        pub static MAP: LazyLock<Vec<i32>> = LazyLock::new(|| vec![2, 2, 3, 3, 3, 3]);
        // ConiferBranchCell.getValue(0): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
        // ConiferBranchCell.getValueFromSide(1): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
    }

    /// `ConiferLeafCell` (com/dtteam/dynamictrees/systems/cell/ConiferLeafCell.java)
    pub mod conifer_leaf_cell {
        use super::*;
        // The class itself isn't translated: it extends `MatrixCell`. Its static parts are below.
        pub static VAL_MAP: LazyLock<Vec<i8>> = LazyLock::new(|| vec![0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 2i8, 4i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 0i8, 2i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 0i8, 2i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 0i8, 2i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 0i8, 2i8, 0i8, 0i8, 0i8]);
        // ConiferLeafCell.ConiferLeafCell(1): not translated, its class isn't translated (it extends `MatrixCell`).
    }

    /// `ConiferTopBranchCell` (com/dtteam/dynamictrees/systems/cell/ConiferTopBranchCell.java)
    pub mod conifer_top_branch_cell {
        use super::*;
        // The class itself isn't translated: it implements `Cell` (com.dtteam.dynamictrees.api.cell). Its static parts are below.
        pub static MAP: LazyLock<Vec<i32>> = LazyLock::new(|| vec![2, 5, 3, 3, 3, 3]);
        // ConiferTopBranchCell.getValue(0): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
        // ConiferTopBranchCell.getValueFromSide(1): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
    }

    /// `DarkOakLeafCell` (com/dtteam/dynamictrees/systems/cell/DarkOakLeafCell.java)
    pub mod dark_oak_leaf_cell {
        use super::*;
        // The class itself isn't translated: it extends `MatrixCell`. Its static parts are below.
        pub static VAL_MAP: LazyLock<Vec<i8>> = LazyLock::new(|| vec![0i8, 1i8, 2i8, 3i8, 3i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 3i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 4i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 4i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 4i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 4i8, 0i8, 0i8, 0i8]);
        // DarkOakLeafCell.DarkOakLeafCell(1): not translated, its class isn't translated (it extends `MatrixCell`).
    }

    /// `LeafClusters` (com/dtteam/dynamictrees/systems/cell/LeafClusters.java)
    pub mod leaf_clusters {
        use super::*;
        // NULL_MAP: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // DECIDUOUS: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // CONIFER: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // ACACIA: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // DARK_OAK: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // BARE: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // PALM: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // BUSH: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // NETHER_FUNGUS: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // AZALEA: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        #[derive(Clone, Debug)]
        pub struct LeafClusters {
        }
        impl LeafClusters {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MatrixCell` (com/dtteam/dynamictrees/systems/cell/MatrixCell.java)
    pub mod matrix_cell {
        use super::*;
        // The class itself isn't translated: it implements `Cell` (com.dtteam.dynamictrees.api.cell). Its static parts are below.
        // MatrixCell.MatrixCell(2): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
        // MatrixCell.getValue(0): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
        // MatrixCell.getValueFromSide(1): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
    }

    /// `MetadataCell` (com/dtteam/dynamictrees/systems/cell/MetadataCell.java)
    pub mod metadata_cell {
        use super::*;
        pub const NONE: i32 = 0;
        pub const TOP_BRANCH: i32 = 1;
        #[derive(Clone, Debug)]
        pub struct MetadataCell {
        }
        impl MetadataCell {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn radius_and_meta(radius: i32, meta: i32) -> i32 {
            return (radius & 0xFF) | (meta << 8);
        }
        pub fn get_radius(radius_meta: i32) -> i32 {
            return radius_meta & 0xFF;
        }
        pub fn get_meta(radius_meta: i32) -> i32 {
            return radius_meta >> 8;
        }
    }

    /// `NetherFungusLeafCell` (com/dtteam/dynamictrees/systems/cell/NetherFungusLeafCell.java)
    pub mod nether_fungus_leaf_cell {
        use super::*;
        // The class itself isn't translated: it extends `MatrixCell`. Its static parts are below.
        pub static VAL_MAP: LazyLock<Vec<i8>> = LazyLock::new(|| vec![0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 6i8, 7i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 6i8, 7i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 6i8, 7i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 6i8, 7i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 6i8, 7i8]);
        // NetherFungusLeafCell.NetherFungusLeafCell(1): not translated, its class isn't translated (it extends `MatrixCell`).
    }

    /// `NormalCell` (com/dtteam/dynamictrees/systems/cell/NormalCell.java)
    pub mod normal_cell {
        use super::*;
        // The class itself isn't translated: it implements `Cell` (com.dtteam.dynamictrees.api.cell). Its static parts are below.
        // NormalCell.NormalCell(1): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
        // NormalCell.getValue(0): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
        // NormalCell.getValueFromSide(1): not translated, its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)).
    }

    /// `PalmFrondCell` (com/dtteam/dynamictrees/systems/cell/PalmFrondCell.java)
    pub mod palm_frond_cell {
        use super::*;
        // The class itself isn't translated: it extends `MatrixCell`. Its static parts are below.
        pub static VAL_MAP: LazyLock<Vec<i8>> = LazyLock::new(|| vec![0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 4i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 0i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 0i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 0i8, 0i8, 0i8, 0i8, 0i8, 1i8, 2i8, 3i8, 0i8, 0i8, 0i8, 0i8]);
        // PalmFrondCell.PalmFrondCell(1): not translated, its class isn't translated (it extends `MatrixCell`).
    }

    /// `FutureBreak` (com/dtteam/dynamictrees/systems/FutureBreak.java)
    pub mod future_break {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2680`. Its static parts are below.
        // FUTURE_BREAKS: not translated, it uses FutureBreak, which isn't translated (it uses Minecraft's `class_2680`).
        // FutureBreak.FutureBreak(5): not translated, its class isn't translated (it uses Minecraft's `class_2680`).
        // FutureBreak.add(1): not translated, it uses FutureBreak, which isn't translated (it uses Minecraft's `class_2680`).
        // FutureBreak.process(1): not translated, it uses Minecraft's `class_1937`.
    }

    /// `AlternativeLeavesGenFeature` (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java)
    pub mod alternative_leaves_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // ALT_LEAVES: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ALT_LEAVES_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // AlternativeLeavesGenFeature.AlternativeLeavesGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // AlternativeLeavesGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // AlternativeLeavesGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // AlternativeLeavesGenFeature.shouldApply(2): not translated, its class isn't translated (it extends `GenFeature`).
        // AlternativeLeavesGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // AlternativeLeavesGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // AlternativeLeavesGenFeature.getAltLeavesBlock(1): not translated, its class isn't translated (it extends `GenFeature`).
        // AlternativeLeavesGenFeature.getSwapBlockState(5): not translated, its class isn't translated (it extends `GenFeature`).
        // AlternativeLeavesGenFeature.setAltLeaves(5): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `BeeNestGenFeature` (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java)
    pub mod bee_nest_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // NEST_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // WORLD_GEN_CHANCE_FUNCTION: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        pub const GUARANTEED_CHANCE: f64 = 1.0f64;
        pub const COMMON_CHANCE: f64 = 0.05f64;
        pub const UNCOMMON_CHANCE: f64 = 0.02f64;
        pub const RARE_CHANCE: f64 = 2.0e-4f64;
        pub const GROW_CHANCE: f64 = 0.001f64;
        // BeeNestGenFeature.BeeNestGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.getWorldGenChanceFunction(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.getCanGrowPredicate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.placeBeeNestInValidPlace(5): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.placeBeeNestWithBees(6): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.storeBee(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.nestAlreadyPresent(4): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.getTreeHeight(3): not translated, its class isn't translated (it extends `GenFeature`).
        // BeeNestGenFeature.findBranchPits(4): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `BeeNestGenFeature.WorldGenChanceFunction` (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java)
    pub mod bee_nest_gen_feature_world_gen_chance_function {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `BiomePredicateGenFeature` (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java)
    pub mod biome_predicate_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // ONLY_WORLD_GEN: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // GEN_FEATURE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // BiomePredicateGenFeature.BiomePredicateGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // BiomePredicateGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // BiomePredicateGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // BiomePredicateGenFeature.shouldApply(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BiomePredicateGenFeature.generate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BiomePredicateGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BiomePredicateGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BiomePredicateGenFeature.postRot(2): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `BottomFlareGenFeature` (com/dtteam/dynamictrees/systems/genfeature/BottomFlareGenFeature.java)
    pub mod bottom_flare_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // MIN_RADIUS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // BottomFlareGenFeature.BottomFlareGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // BottomFlareGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // BottomFlareGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // BottomFlareGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BottomFlareGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BottomFlareGenFeature.flareBottom(4): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `BushGenFeature` (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java)
    pub mod bush_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // LOG: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // LEAVES: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // SECONDARY_LEAVES: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // SECONDARY_LEAVES_CHANCE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // BushGenFeature.BushGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.generate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.commonGen(7): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.placeLeaves(4): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.selectLeavesBlock(4): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.placeLeavesBlock(3): not translated, its class isn't translated (it extends `GenFeature`).
        // BushGenFeature.coordHashCode(1): not translated, it uses Minecraft's `class_2338`.
    }

    /// `ClearVolumeGenFeature` (com/dtteam/dynamictrees/systems/genfeature/ClearVolumeGenFeature.java)
    pub mod clear_volume_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // HEIGHT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ClearVolumeGenFeature.ClearVolumeGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // ClearVolumeGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // ClearVolumeGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // ClearVolumeGenFeature.preGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `ConiferTopperGenFeature` (com/dtteam/dynamictrees/systems/genfeature/ConiferTopperGenFeature.java)
    pub mod conifer_topper_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // LEAVES_PROPERTIES: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConiferTopperGenFeature.ConiferTopperGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // ConiferTopperGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // ConiferTopperGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // ConiferTopperGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `FullGenerationContext` (com/dtteam/dynamictrees/systems/genfeature/context/FullGenerationContext.java)
    pub mod full_generation_context {
        use super::*;
        // The class itself isn't translated: it extends `GenFeatureContext`. Its static parts are below.
        // FullGenerationContext.FullGenerationContext(6): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // FullGenerationContext.biome(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // FullGenerationContext.radius(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // FullGenerationContext.isWorldGen(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
    }

    /// `GenFeatureContext` (com/dtteam/dynamictrees/systems/genfeature/context/GenFeatureContext.java)
    pub mod gen_feature_context {
        use super::*;
        // The class itself isn't translated: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`). Its static parts are below.
        // GenFeatureContext.GenFeatureContext(3): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // GenFeatureContext.levelContext(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // GenFeatureContext.level(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // GenFeatureContext.pos(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // GenFeatureContext.species(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // GenFeatureContext.random(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
    }

    /// `PostGenerationContext` (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java)
    pub mod post_generation_context {
        use super::*;
        // The class itself isn't translated: it extends `GenFeatureContext`. Its static parts are below.
        // PostGenerationContext.PostGenerationContext(3): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGenerationContext.originPos(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGenerationContext.biome(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGenerationContext.radius(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGenerationContext.endPoints(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGenerationContext.initialDirtState(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGenerationContext.seasonValue(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGenerationContext.fruitProductionFactor(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGenerationContext.isWorldGen(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
    }

    /// `PostGrowContext` (com/dtteam/dynamictrees/systems/genfeature/context/PostGrowContext.java)
    pub mod post_grow_context {
        use super::*;
        // The class itself isn't translated: it extends `GenFeatureContext`. Its static parts are below.
        // PostGrowContext.PostGrowContext(6): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGrowContext.treePos(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGrowContext.fertility(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostGrowContext.natural(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
    }

    /// `PostRotContext` (com/dtteam/dynamictrees/systems/genfeature/context/PostRotContext.java)
    pub mod post_rot_context {
        use super::*;
        // The class itself isn't translated: it extends `GenFeatureContext`. Its static parts are below.
        // PostRotContext.PostRotContext(7): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostRotContext.radius(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostRotContext.neighbourCount(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostRotContext.fertility(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PostRotContext.rapid(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
    }

    /// `PreGenerationContext` (com/dtteam/dynamictrees/systems/genfeature/context/PreGenerationContext.java)
    pub mod pre_generation_context {
        use super::*;
        // The class itself isn't translated: it extends `GenFeatureContext`. Its static parts are below.
        // PreGenerationContext.PreGenerationContext(7): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PreGenerationContext.radius(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PreGenerationContext.facing(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PreGenerationContext.joCode(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
        // PreGenerationContext.isWorldGen(0): not translated, its class isn't translated (it extends `GenFeatureContext`).
    }

    /// `FruitGenFeature` (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java)
    pub mod fruit_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // FRUIT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // FruitGenFeature.FruitGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.shouldApply(2): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.place(6): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.shouldPlace(3): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.placeDuringWorldGen(7): not translated, its class isn't translated (it extends `GenFeature`).
        // FruitGenFeature.shouldPlaceDuringWorldGen(3): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `GenFeature` (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java)
    pub mod gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration). Its static parts are below.
        // VERTICAL_SPREAD: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // QUANTITY: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // RAY_DISTANCE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // MAX_HEIGHT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // CAN_GROW_PREDICATE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // MAX_COUNT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // FRUITING_RADIUS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // PLACE_CHANCE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // BIOME_PREDICATE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // NULL: not translated, its value couldn't be read: an anonymous class.
        // REGISTRY: not translated, it uses ConfigurableRegistry, which isn't translated (it's generic).
        // GenFeature.GenFeature(1): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeature.generate(3): not translated, it's generic.
        // GenFeature.preGenerate(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeature.postGenerate(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeature.postGrow(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeature.postRot(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeature.generate(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeature.shouldApply(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
    }

    /// `GenFeature.CanGrowPredicate` (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java)
    pub mod gen_feature_can_grow_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `GenFeature.Type` (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java)
    pub mod gen_feature_type {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PRE_GENERATION: not translated, it uses Type, which isn't translated (it's generic).
        // POST_GENERATION: not translated, it uses Type, which isn't translated (it's generic).
        // POST_GROW: not translated, it uses Type, which isn't translated (it's generic).
        // POST_ROT: not translated, it uses Type, which isn't translated (it's generic).
        // FULL: not translated, it uses Type, which isn't translated (it's generic).
        // GenFeature.Type.Type(1): not translated, its class isn't translated (it's generic).
        // GenFeature.Type.generate(2): not translated, its class isn't translated (it's generic).
    }

    /// `GenFeatureConfiguration` (com/dtteam/dynamictrees/systems/genfeature/GenFeatureConfiguration.java)
    pub mod gen_feature_configuration {
        use super::*;
        // The class itself isn't translated: it extends `Configuration` (com.dtteam.dynamictrees.api.configuration). Its static parts are below.
        // TEMPLATES: not translated, it uses TemplateRegistry, which isn't translated (it's generic).
        // GenFeatureConfiguration.GenFeatureConfiguration(1): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeatureConfiguration.getGenFeature(0): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeatureConfiguration.copy(0): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeatureConfiguration.generate(2): not translated, it's generic.
        // GenFeatureConfiguration.shouldApply(1): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GenFeatureConfiguration.getNull(0): not translated, it uses GenFeatureConfiguration, which isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
    }

    /// `GenFeatures` (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java)
    pub mod gen_features {
        use super::*;
        // FRUIT: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // POD: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // VINES: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // BEE_NEST: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // SHROOMLIGHT: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // CONIFER_TOPPER: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // MOUND: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // ROOTS: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // CLEAR_VOLUME: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // BOTTOM_FLARE: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // UNDERGROWTH: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // PODZOL: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // BUSH: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // HUGE_MUSHROOM: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // HUGE_MUSHROOMS: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // MUSHROOM_ROT: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // ROT_SOIL: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // BIOME_PREDICATE: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // RANDOM_PREDICATE: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // ALTERNATIVE_LEAVES: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // ROOT_SYSTEM: not translated, it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        #[derive(Clone, Debug)]
        pub struct GenFeatures {
        }
        impl GenFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GenFeatures.register(1): not translated, it uses Registry, which isn't translated (it's an interface).
    }

    /// `HugeMushroomGenFeature` (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java)
    pub mod huge_mushroom_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // MUSHROOM_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // STEM_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // BROWN_CAP: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // BROWN_CAP_MEDIUM: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // BROWN_CAP_SMALL: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // RED_CAP: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // RED_CAP_SHORT: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // RED_CAP_SMALL: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // HugeMushroomGenFeature.HugeMushroomGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.shouldApply(2): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.setHeight(1): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.getCapForHeight(2): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.getMushroomHeight(5): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.generate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.generateMushrooms(2): not translated, its class isn't translated (it extends `GenFeature`).
        // HugeMushroomGenFeature.getMushroomStateForValue(4): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `HugeMushroomUndergrowthGenFeature` (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomUndergrowthGenFeature.java)
    pub mod huge_mushroom_undergrowth_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `HugeMushroomGenFeature`. Its static parts are below.
        // MAX_MUSHROOMS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // MAX_ATTEMPTS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // HugeMushroomUndergrowthGenFeature.HugeMushroomUndergrowthGenFeature(1): not translated, its class isn't translated (it extends `HugeMushroomGenFeature`).
        // HugeMushroomUndergrowthGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `HugeMushroomGenFeature`).
        // HugeMushroomUndergrowthGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `HugeMushroomGenFeature`).
        // HugeMushroomUndergrowthGenFeature.generate(2): not translated, its class isn't translated (it extends `HugeMushroomGenFeature`).
        // HugeMushroomUndergrowthGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `HugeMushroomGenFeature`).
    }

    /// `MoundGenFeature` (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java)
    pub mod mound_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // moundMap: not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
        // MOUND_CUTOFF_RADIUS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // OFFSET: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // MoundGenFeature.MoundGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // MoundGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // MoundGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // MoundGenFeature.preGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // MoundGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `MushroomRotGenFeature` (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java)
    pub mod mushroom_rot_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // MUSHROOM: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ALTERNATE_MUSHROOM: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ALTERNATE_MUSHROOM_CHANCE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // MushroomRotGenFeature.MushroomRotGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // MushroomRotGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // MushroomRotGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // MushroomRotGenFeature.postRot(2): not translated, its class isn't translated (it extends `GenFeature`).
        // MushroomRotGenFeature.mayMushroomPlaceOn(2): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `PodGenFeature` (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java)
    pub mod pod_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // POD: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // BLOCKS_PER_POD: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // LOWEST_TRUNK_HEIGHT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // PodGenFeature.PodGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // PodGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // PodGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // PodGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // PodGenFeature.shouldGrow(4): not translated, its class isn't translated (it extends `GenFeature`).
        // PodGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // PodGenFeature.shouldGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // PodGenFeature.place(7): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `PodzolGenFeature` (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java)
    pub mod podzol_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // PODZOL_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // SPREAD_DISTANCE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // DARK_THRESHOLD: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // MAX_HEIGHT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // KILL_PLANTS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // PodzolGenFeature.PodzolGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // PodzolGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // PodzolGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // PodzolGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // PodzolGenFeature.shouldDestroyPlant(1): not translated, its class isn't translated (it extends `GenFeature`).
        // PodzolGenFeature.spreadPodzol(3): not translated, it uses Minecraft's `class_1936`.
    }

    /// `RandomPredicateGenFeature` (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java)
    pub mod random_predicate_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // ONLY_WORLD_GEN: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // GEN_FEATURE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // RandomPredicateGenFeature.RandomPredicateGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // RandomPredicateGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RandomPredicateGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RandomPredicateGenFeature.shouldApply(2): not translated, its class isn't translated (it extends `GenFeature`).
        // RandomPredicateGenFeature.preGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // RandomPredicateGenFeature.generate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // RandomPredicateGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // RandomPredicateGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // RandomPredicateGenFeature.postRot(2): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `RootsGenFeature` (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java)
    pub mod roots_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // MIN_TRUNK_RADIUS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // LEVEL_LIMIT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // SCALE_FACTOR: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // RootsGenFeature.RootsGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.createRootMaps(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.getMinTrunkRadius(1): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.startRoots(5): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.nextRoot(13): not translated, its class isn't translated (it extends `GenFeature`).
        // RootsGenFeature.isReplaceableWithRoots(3): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `RootSystemGenFeature` (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java)
    pub mod root_system_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // MAX_ROOT_COLUMN_HEIGHT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ROOT_RADIUS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ROOT_REPLACEABLE_TAG: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ROOT_PLACEMENT_ATTEMPTS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ROOT_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // HANGING_ROOT_RADIUS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // HANGING_ROOT_VERTICAL_SPAN: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // HANGING_ROOT_PLACEMENT_ATTEMPTS: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // HANGING_ROOT_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // RootSystemGenFeature.RootSystemGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // RootSystemGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RootSystemGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RootSystemGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // RootSystemGenFeature.placeRootedDirt(6): not translated, it uses Minecraft's `class_1936`.
        // RootSystemGenFeature.placeHangingRoots(5): not translated, it uses Minecraft's `class_1936`.
    }

    /// `RotSoilGenFeature` (com/dtteam/dynamictrees/systems/genfeature/RotSoilGenFeature.java)
    pub mod rot_soil_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // ROTTEN_SOIL: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // RotSoilGenFeature.RotSoilGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // RotSoilGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RotSoilGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // RotSoilGenFeature.postRot(2): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `ShroomlightGenFeature` (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java)
    pub mod shroomlight_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // SHROOMLIGHT_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // HORIZONTALS: not translated, it uses Minecraft's `class_2350`.
        pub const VANILLA_GROW_CHANCE: f64 = (0.005f32 as f64);
        // ShroomlightGenFeature.ShroomlightGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // ShroomlightGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // ShroomlightGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // ShroomlightGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // ShroomlightGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // ShroomlightGenFeature.placeShroomlightsInValidPlace(4): not translated, its class isn't translated (it extends `GenFeature`).
        // ShroomlightGenFeature.getTreeHeight(3): not translated, its class isn't translated (it extends `GenFeature`).
        // ShroomlightGenFeature.findBranchPits(4): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `UndergrowthGenFeature` (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java)
    pub mod undergrowth_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // SPECIES_A: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // SPECIES_B: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // PROPORTION: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // UndergrowthGenFeature.UndergrowthGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // UndergrowthGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // UndergrowthGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // UndergrowthGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // UndergrowthGenFeature.tryToPlaceTree(5): not translated, its class isn't translated (it extends `GenFeature`).
        // UndergrowthGenFeature.areTreesAround(2): not translated, its class isn't translated (it extends `GenFeature`).
        // UndergrowthGenFeature.placeTreeAtLocation(3): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `VinesGenFeature` (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java)
    pub mod vines_gen_feature {
        use super::*;
        // The class itself isn't translated: it extends `GenFeature`. Its static parts are below.
        // MAX_LENGTH: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // TIP_BLOCK: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // VINE_TYPE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // VinesGenFeature.VinesGenFeature(1): not translated, its class isn't translated (it extends `GenFeature`).
        // VinesGenFeature.registerProperties(0): not translated, its class isn't translated (it extends `GenFeature`).
        // VinesGenFeature.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GenFeature`).
        // VinesGenFeature.postGenerate(2): not translated, its class isn't translated (it extends `GenFeature`).
        // VinesGenFeature.postGrow(2): not translated, its class isn't translated (it extends `GenFeature`).
        // VinesGenFeature.addSideVines(6): not translated, its class isn't translated (it extends `GenFeature`).
        // VinesGenFeature.addVerticalVines(6): not translated, its class isn't translated (it extends `GenFeature`).
        // VinesGenFeature.findGround(2): not translated, its class isn't translated (it extends `GenFeature`).
        // VinesGenFeature.placeVines(7): not translated, its class isn't translated (it extends `GenFeature`).
    }

    /// `VinesGenFeature.VineType` (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java)
    pub mod vines_gen_feature_vine_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum VineType {
            Ceiling,
            Floor,
            Side,
        }
        impl VineType {
            pub const VALUES: [VineType; 3] = [VineType::Ceiling, VineType::Floor, VineType::Side, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    VineType::Ceiling => "CEILING",
                    VineType::Floor => "FLOOR",
                    VineType::Side => "SIDE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "CEILING" => VineType::Ceiling,
                    "FLOOR" => VineType::Floor,
                    "SIDE" => VineType::Side,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `GrowSignal` (com/dtteam/dynamictrees/systems/GrowSignal.java)
    pub mod grow_signal {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2350`. Its static parts are below.
        // GrowSignal.GrowSignal(4): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // GrowSignal.GrowSignal(5): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // GrowSignal.getSpecies(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // GrowSignal.step(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // GrowSignal.doTurn(1): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
        // GrowSignal.isInTrunk(0): not translated, its class isn't translated (it uses Minecraft's `class_2350`).
    }

    /// `AzaleaLogic` (com/dtteam/dynamictrees/systems/growthlogic/AzaleaLogic.java)
    pub mod azalea_logic {
        use super::*;
        // The class itself isn't translated: it extends `GrowthLogicKit`. Its static parts are below.
        // HORIZONTALS: not translated, it uses Minecraft's `class_2350`.
        // AzaleaLogic.AzaleaLogic(1): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // AzaleaLogic.populateDirectionProbabilityMap(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // AzaleaLogic.getDirectionBias(1): not translated, it uses Minecraft's `class_2350`.
    }

    /// `ConiferLogic` (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java)
    pub mod conifer_logic {
        use super::*;
        // The class itself isn't translated: it extends `GrowthLogicKit`. Its static parts are below.
        // ENERGY_DIVISOR: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // HORIZONTAL_LIMITER: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // VARIATE_LOWEST_BRANCH: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // ConiferLogic.ConiferLogic(1): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // ConiferLogic.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // ConiferLogic.registerProperties(0): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // ConiferLogic.selectNewDirection(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // ConiferLogic.populateDirectionProbabilityMap(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // ConiferLogic.getEnergy(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // ConiferLogic.getLowestBranchHeight(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // ConiferLogic.getHashVariation(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
    }

    /// `DirectionManipulationContext` (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionManipulationContext.java)
    pub mod direction_manipulation_context {
        use super::*;
        // The class itself isn't translated: it extends `DirectionSelectionContext`. Its static parts are below.
        // DirectionManipulationContext.DirectionManipulationContext(7): not translated, its class isn't translated (it extends `DirectionSelectionContext`).
        // DirectionManipulationContext.radius(0): not translated, its class isn't translated (it extends `DirectionSelectionContext`).
        // DirectionManipulationContext.probMap(0): not translated, its class isn't translated (it extends `DirectionSelectionContext`).
        // DirectionManipulationContext.probMap(1): not translated, its class isn't translated (it extends `DirectionSelectionContext`).
    }

    /// `DirectionSelectionContext` (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionSelectionContext.java)
    pub mod direction_selection_context {
        use super::*;
        // The class itself isn't translated: it extends `PositionalSpeciesContext`. Its static parts are below.
        // DirectionSelectionContext.DirectionSelectionContext(5): not translated, its class isn't translated (it extends `PositionalSpeciesContext`).
        // DirectionSelectionContext.branch(0): not translated, its class isn't translated (it extends `PositionalSpeciesContext`).
        // DirectionSelectionContext.signal(0): not translated, its class isn't translated (it extends `PositionalSpeciesContext`).
    }

    /// `PositionalSpeciesContext` (com/dtteam/dynamictrees/systems/growthlogic/context/PositionalSpeciesContext.java)
    pub mod positional_species_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_1937`. Its static parts are below.
        // PositionalSpeciesContext.PositionalSpeciesContext(3): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // PositionalSpeciesContext.level(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // PositionalSpeciesContext.pos(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // PositionalSpeciesContext.species(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
    }

    /// `DarkOakLogic` (com/dtteam/dynamictrees/systems/growthlogic/DarkOakLogic.java)
    pub mod dark_oak_logic {
        use super::*;
        // The class itself isn't translated: it extends `GrowthLogicKit`. Its static parts are below.
        // DarkOakLogic.DarkOakLogic(1): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // DarkOakLogic.populateDirectionProbabilityMap(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // DarkOakLogic.getEnergy(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // DarkOakLogic.getLowestBranchHeight(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
    }

    /// `GrowthLogicKit` (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java)
    pub mod growth_logic_kit {
        use super::*;
        // The class itself isn't translated: it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration). Its static parts are below.
        // HEIGHT_VARIATION: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // DEFAULT: not translated, its value couldn't be read: an anonymous class.
        // REGISTRY: not translated, it uses ConfigurableRegistry, which isn't translated (it's generic).
        // GrowthLogicKit.GrowthLogicKit(1): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKit.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKit.registerProperties(0): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKit.selectNewDirection(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKit.populateDirectionProbabilityMap(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKit.getEnergy(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKit.getLowestBranchHeight(2): not translated, its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKit.findGrowthLogicKit(1): not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKit.findGrowthLogicKit(1): not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
    }

    /// `GrowthLogicKitConfiguration` (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java)
    pub mod growth_logic_kit_configuration {
        use super::*;
        // The class itself isn't translated: it extends `Configuration` (com.dtteam.dynamictrees.api.configuration). Its static parts are below.
        // TEMPLATES: not translated, it uses TemplateRegistry, which isn't translated (it's generic).
        // GrowthLogicKitConfiguration.GrowthLogicKitConfiguration(1): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKitConfiguration.copy(0): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKitConfiguration.selectNewDirection(1): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKitConfiguration.populateDirectionProbabilityMap(1): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKitConfiguration.getEnergy(1): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKitConfiguration.getLowestBranchHeight(1): not translated, its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
        // GrowthLogicKitConfiguration.getDefault(0): not translated, it uses GrowthLogicKitConfiguration, which isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)).
    }

    /// `GrowthLogicKits` (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java)
    pub mod growth_logic_kits {
        use super::*;
        // DARK_OAK: not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // CONIFER: not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // JUNGLE: not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // AZALEA: not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // NETHER_FUNGUS: not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // PALM: not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        // MANGROVE_ROOTS: not translated, it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)).
        #[derive(Clone, Debug)]
        pub struct GrowthLogicKits {
        }
        impl GrowthLogicKits {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GrowthLogicKits.register(1): not translated, it uses Registry, which isn't translated (it's an interface).
    }

    /// `JungleLogic` (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java)
    pub mod jungle_logic {
        use super::*;
        // The class itself isn't translated: it extends `GrowthLogicKit`. Its static parts are below.
        // CANOPY_HEIGHT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // BRANCH_OUT_CHANCE: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // JungleLogic.JungleLogic(1): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // JungleLogic.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // JungleLogic.registerProperties(0): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // JungleLogic.selectNewDirection(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // JungleLogic.populateDirectionProbabilityMap(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // JungleLogic.getEnergy(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
    }

    /// `MangroveRootsLogic` (com/dtteam/dynamictrees/systems/growthlogic/MangroveRootsLogic.java)
    pub mod mangrove_roots_logic {
        use super::*;
        // The class itself isn't translated: it extends `GrowthLogicKit`. Its static parts are below.
        // MangroveRootsLogic.MangroveRootsLogic(1): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // MangroveRootsLogic.populateDirectionProbabilityMap(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // MangroveRootsLogic.selectNewDirection(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // MangroveRootsLogic.getLowestBranchHeight(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // MangroveRootsLogic.getEnergy(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
    }

    /// `NetherFungusLogic` (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java)
    pub mod nether_fungus_logic {
        use super::*;
        // The class itself isn't translated: it extends `GrowthLogicKit`. Its static parts are below.
        // MIN_CAP_HEIGHT: not translated, it uses ConfigurationProperty, which isn't translated (it's generic).
        // NetherFungusLogic.NetherFungusLogic(1): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // NetherFungusLogic.createDefaultConfiguration(0): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // NetherFungusLogic.registerProperties(0): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // NetherFungusLogic.selectNewDirection(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // NetherFungusLogic.populateDirectionProbabilityMap(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // NetherFungusLogic.getHashedVariation(3): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // NetherFungusLogic.getEnergy(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // NetherFungusLogic.getLowestBranchHeight(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
    }

    /// `PalmGrowthLogic` (com/dtteam/dynamictrees/systems/growthlogic/PalmGrowthLogic.java)
    pub mod palm_growth_logic {
        use super::*;
        // The class itself isn't translated: it extends `GrowthLogicKit`. Its static parts are below.
        // PalmGrowthLogic.PalmGrowthLogic(1): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // PalmGrowthLogic.populateDirectionProbabilityMap(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
        // PalmGrowthLogic.getEnergy(2): not translated, its class isn't translated (it extends `GrowthLogicKit`).
    }

    /// `CoderNode` (com/dtteam/dynamictrees/systems/nodemapper/CoderNode.java)
    pub mod coder_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // CoderNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // CoderNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // CoderNode.compile(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // CoderNode.nextLink(3): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `CoderNode.Link` (com/dtteam/dynamictrees/systems/nodemapper/CoderNode.java)
    pub mod coder_node_link {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2338`. Its static parts are below.
        // CoderNode.Link.Link(1): not translated, its class isn't translated (it uses Minecraft's `class_2338`).
    }

    /// `CollectorNode` (com/dtteam/dynamictrees/systems/nodemapper/CollectorNode.java)
    pub mod collector_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // CollectorNode.CollectorNode(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // CollectorNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // CollectorNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // CollectorNode.contains(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `DenuderNode` (com/dtteam/dynamictrees/systems/nodemapper/DenuderNode.java)
    pub mod denuder_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // DenuderNode.DenuderNode(2): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // DenuderNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // DenuderNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // DenuderNode.removeSurroundingLeaves(2): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `DestroyerNode` (com/dtteam/dynamictrees/systems/nodemapper/DestroyerNode.java)
    pub mod destroyer_node {
        use super::*;
        // The class itself isn't translated: it extends `FindEndsNode`. Its static parts are below.
        // DestroyerNode.DestroyerNode(1): not translated, its class isn't translated (it extends `FindEndsNode`).
        // DestroyerNode.setPlayer(1): not translated, its class isn't translated (it extends `FindEndsNode`).
        // DestroyerNode.run(4): not translated, its class isn't translated (it extends `FindEndsNode`).
        // DestroyerNode.returnRun(4): not translated, its class isn't translated (it extends `FindEndsNode`).
    }

    /// `DiseaseNode` (com/dtteam/dynamictrees/systems/nodemapper/DiseaseNode.java)
    pub mod disease_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // DiseaseNode.DiseaseNode(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // DiseaseNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // DiseaseNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `FindEndsNode` (com/dtteam/dynamictrees/systems/nodemapper/FindEndsNode.java)
    pub mod find_ends_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // FindEndsNode.FindEndsNode(0): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // FindEndsNode.FindEndsNode(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // FindEndsNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // FindEndsNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // FindEndsNode.getEnds(0): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `FreezerNode` (com/dtteam/dynamictrees/systems/nodemapper/FreezerNode.java)
    pub mod freezer_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        pub const FREEZE_RADIUS: i32 = 3;
        // FreezerNode.FreezerNode(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // FreezerNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // FreezerNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // FreezerNode.freezeSurroundingLeaves(3): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `InflatorNode` (com/dtteam/dynamictrees/systems/nodemapper/InflatorNode.java)
    pub mod inflator_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // InflatorNode.InflatorNode(2): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // InflatorNode.InflatorNode(3): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // InflatorNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // InflatorNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `NetVolumeNode` (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java)
    pub mod net_volume_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // NetVolumeNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // NetVolumeNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // NetVolumeNode.getVolume(0): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `NetVolumeNode.Volume` (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java)
    pub mod net_volume_node_volume {
        use super::*;
        pub const VOXELSPERLOG: i32 = 4096;
        #[derive(Clone, Debug)]
        pub struct Volume {
            pub branch_volumes: JMap<i32, i32>,
            pub max_branch: i32,
        }
        impl Volume {
            // NetVolumeNode.Volume.Volume(0): not translated, its constructors are overloaded with the same number of parameters.
            // NetVolumeNode.Volume.Volume(1): not translated, its constructors are overloaded with the same number of parameters.
            // NetVolumeNode.Volume.Volume(1): not translated, its constructors are overloaded with the same number of parameters.
            // NetVolumeNode.Volume.addVolume(1): not translated, it's overloaded with the same number of parameters.
            // NetVolumeNode.Volume.addVolume(2): not translated, Map.computeIfPresent.
            // NetVolumeNode.Volume.addVolume(1): not translated, it's overloaded with the same number of parameters.
            // NetVolumeNode.Volume.multiplyVolume(1): not translated, Map.replaceAll.
            pub fn get_raw_volumes_array(&self) -> Vec<i32> {
                let mut volumes: Vec<i32> = vec![0i32; (self.max_branch + 1) as usize];
                for i in 0..=self.max_branch {
                    let mut vol: i32 = 0;
                    if self.branch_volumes.contains_key(&i) {
                        vol = (self.branch_volumes.get(&i).cloned()).expect("null");
                    }
                    volumes[i as usize] = vol;
                }
                return volumes.to_vec();
            }
            pub fn get_volume_0(&self) -> f32 {
                return (self.get_raw_volume_0() as f32) / 4096.0f32;
            }
            pub fn get_volume_1(&self, branch: i32) -> f32 {
                return ((self.get_raw_volume_1(branch)) as f32) / 4096.0f32;
            }
            pub fn get_raw_volume_0(&self) -> i32 {
                let mut total_volume: i32 = 0;
                {
                    let mut i: i32 = 0;
                    while i < (self.branch_volumes.len() as i32) {
                        total_volume += self.get_raw_volume_1(i);
                        i += 1;
                    }
                }
                return total_volume;
            }
            pub fn get_raw_volume_1(&self, branch: i32) -> i32 {
                return if self.branch_volumes.contains_key(&branch) { (self.branch_volumes.get(&branch).cloned()).expect("null") } else { 0 };
            }
        }
    }

    /// `PodGenerationNode` (com/dtteam/dynamictrees/systems/nodemapper/PodGenerationNode.java)
    pub mod pod_generation_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // PodGenerationNode.PodGenerationNode(6): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // PodGenerationNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // PodGenerationNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `PodGenerationNode.PodPlacer` (com/dtteam/dynamictrees/systems/nodemapper/PodGenerationNode.java)
    pub mod pod_generation_node_pod_placer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PodGenerationNode.PodPlacer.place(5): not translated, it has no body.
    }

    /// `RootIntegrityNode` (com/dtteam/dynamictrees/systems/nodemapper/RootIntegrityNode.java)
    pub mod root_integrity_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // RootIntegrityNode.RootIntegrityNode(0): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // RootIntegrityNode.RootIntegrityNode(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // RootIntegrityNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // RootIntegrityNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // RootIntegrityNode.getStable(0): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `RootsDestroyerNode` (com/dtteam/dynamictrees/systems/nodemapper/RootsDestroyerNode.java)
    pub mod roots_destroyer_node {
        use super::*;
        // The class itself isn't translated: it extends `FindEndsNode`. Its static parts are below.
        // RootsDestroyerNode.RootsDestroyerNode(1): not translated, its class isn't translated (it extends `FindEndsNode`).
        // RootsDestroyerNode.run(4): not translated, its class isn't translated (it extends `FindEndsNode`).
        // RootsDestroyerNode.returnRun(4): not translated, its class isn't translated (it extends `FindEndsNode`).
    }

    /// `ShrinkerNode` (com/dtteam/dynamictrees/systems/nodemapper/ShrinkerNode.java)
    pub mod shrinker_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // ShrinkerNode.ShrinkerNode(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // ShrinkerNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // ShrinkerNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `SpeciesNode` (com/dtteam/dynamictrees/systems/nodemapper/SpeciesNode.java)
    pub mod species_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // SpeciesNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // SpeciesNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // SpeciesNode.getSpecies(0): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `StateNode` (com/dtteam/dynamictrees/systems/nodemapper/StateNode.java)
    pub mod state_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // StateNode.StateNode(1): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // StateNode.getBranchConnectionMap(0): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // StateNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // StateNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `TransformNode` (com/dtteam/dynamictrees/systems/nodemapper/TransformNode.java)
    pub mod transform_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        pub const TEST_LEAVES_RADIUS: i32 = 3;
        // TransformNode.TransformNode(2): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // TransformNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // TransformNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // TransformNode.transformSurroundingLeaves(2): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `TwinkleNode` (com/dtteam/dynamictrees/systems/nodemapper/TwinkleNode.java)
    pub mod twinkle_node {
        use super::*;
        // The class itself isn't translated: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network). Its static parts are below.
        // TwinkleNode.TwinkleNode(2): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // TwinkleNode.run(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
        // TwinkleNode.returnRun(4): not translated, its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)).
    }

    /// `LevelPoissonDiscProvider` (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java)
    pub mod level_poisson_disc_provider {
        use super::*;
        // The class itself isn't translated: it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen). Its static parts are below.
        // LevelPoissonDiscProvider.LevelPoissonDiscProvider(1): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.setSeed(1): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.setDebug(1): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.getPoissonDiscs(3): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.generatePoissonDiscs(3): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.doesDiscIntersectWith(2): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.getChunkDiscSet(2): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.getChunkPoissonData(3): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.setChunkPoissonData(4): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.unloadChunkPoissonData(3): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.getChunkPoissonDiscs(2): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
        // LevelPoissonDiscProvider.getChunkPoissonDiscs(3): not translated, its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)).
    }

    /// `PoissonDisc` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java)
    pub mod poisson_disc {
        use super::*;
        // The class itself isn't translated: it extends `Vec2i`. Its static parts are below.
        // cbm: not translated, a SimpleBitmap with no value (null in Java).
        // icbm: not translated, a SimpleBitmap with no value (null in Java).
        // PoissonDisc.circleBitmapGen(2): not translated, `--` inside an expression.
        pub fn bit_run(start: i32, stop: i32) -> i32 {
            return if start < stop { (((((4294967295i64 as u64) >> (32 - stop)) as i64) & (4294967295i64 << start)) as i32) } else { (((((4294967295i64 as u64) >> (32 - stop)) as i64) | (4294967295i64 << start)) as i32) };
        }
        // PoissonDisc.getCircleBitmap(1): not translated, it uses PoissonDisc.cbm, which isn't translated (a SimpleBitmap with no value (null in Java)).
        // PoissonDisc.getCircleBitmap(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.getCircleInteriorBitmap(1): not translated, it uses PoissonDisc.icbm, which isn't translated (a SimpleBitmap with no value (null in Java)).
        // PoissonDisc.getCircleInteriorBitmap(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.PoissonDisc(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.PoissonDisc(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.PoissonDisc(4): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.PoissonDisc(3): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.PoissonDisc(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.PoissonDisc(1): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.set(3): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.set(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.setRadius(1): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.isInside(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.isInterior(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.isEdge(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.doCirclesIntersect(1): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.doCirclesIntersectPadding(1): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.maskArc(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.fillArc(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.setSolved(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.clearArc(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.edgeMask(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.getFreeBitCW(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.getFreeBitCCW(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.bitToAngle(1): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.getFreeAngleCW(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.getFreeAngleCCW(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.hasFreeAngles(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.isSolved(0): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.discPenetration(1): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.isInCenterChunk(2): not translated, its class isn't translated (it extends `Vec2i`).
        // PoissonDisc.toString(0): not translated, its class isn't translated (it extends `Vec2i`).
    }

    /// `PoissonDiscChunkSet` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java)
    pub mod poisson_disc_chunk_set {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PoissonDiscChunkSet {
            pub disc_data: Vec<i8>,
            pub generated: bool,
        }
        impl PoissonDiscChunkSet {
            pub fn new() -> Self {
                let mut s = Self { disc_data: Vec::new(), generated: false };
                s.disc_data = vec![0i8; 16];
                s
            }
            // PoissonDiscChunkSet.PoissonDiscChunkSet(1): not translated, it calls `Arrays` (java.util).copyOf.
            // PoissonDiscChunkSet.getDiscs(3): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
            pub fn clear_discs(&mut self) {
                self.disc_data.fill(0i8);
            }
            // PoissonDiscChunkSet.addDiscs(1): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
            // PoissonDiscChunkSet.addDisc(1): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
            pub fn get_disc_data(&self) -> Vec<i8> {
                return self.disc_data.to_vec();
            }
            pub fn set_disc_data(&mut self, disc_data: &[i8]) {
                self.disc_data = j_copy_of(&disc_data, 16);
            }
        }
        // PoissonDiscChunkSet.unpackDiscData(4): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        pub fn get_radius_from_disc_data(disc_data: i32) -> i32 {
            return ((disc_data >> 4) & 7) + 1;
        }
        // PoissonDiscChunkSet.buildDiscData(1): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        // PoissonDiscChunkSet.calcTileNum(1): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
    }

    /// `PoissonDiscHelper` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java)
    pub mod poisson_disc_helper {
        use super::*;
        pub static SINGLE_SEARCH_ORDER: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 1, -1]);
        pub static PAIR_SEARCH_ORDER: LazyLock<Vec<i32>> = LazyLock::new(|| vec![34, 33, 35, 18, 50, 17, 17, 19, 49, 51, 32, 36, 2, 66, 1, 3, 65, 67, 16, 20, 48, 52]);
        #[derive(Clone, Debug)]
        pub struct PoissonDiscHelper {
        }
        impl PoissonDiscHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PoissonDiscHelper.findSecondDisc(4): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        // PoissonDiscHelper.findSecondDisc(4): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        // PoissonDiscHelper.findThirdDisc(3): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        // PoissonDiscHelper.maskDiscs(2): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        // PoissonDiscHelper.maskDiscs(3): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        // PoissonDiscHelper.solveDiscs(2): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        // PoissonDiscHelper.gatherUnsolved(2): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
        // PoissonDiscHelper.fastRemove(2): not translated, it uses PoissonDisc, which isn't translated (it extends `Vec2i`).
    }

    /// `PoissonDiscMathHelper` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscMathHelper.java)
    pub mod poisson_disc_math_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PoissonDiscMathHelper {
        }
        impl PoissonDiscMathHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn wrap(mut i: i32, max: i32) -> i32 {
            while i < 0 {
                i += max;
            }
            return i % max;
        }
        pub fn wrap_angle(mut angle: f64) -> f64 {
            let two_pi: f64 = std::f64::consts::PI * 2.0f64;
            angle %= std::f64::consts::PI * 2.0f64;
            return angle + (if angle < 0.0f64 { std::f64::consts::PI * 2.0f64 } else { 0.0f64 });
        }
        pub fn radians_to_turns(angle: f64) -> f32 {
            return ((poisson_disc_math_helper::wrap_angle(angle) / (std::f64::consts::PI * 2.0f64)) as f32);
        }
        pub fn delta_angle(alpha: f64, beta: f64) -> f64 {
            let phi: f64 = ((beta - alpha).abs()) % (std::f64::consts::PI * 2.0f64);
            return if phi > std::f64::consts::PI { (std::f64::consts::PI * 2.0f64) - phi } else { phi };
        }
    }

    /// `PoissonDiscPairData` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscPairData.java)
    pub mod poisson_disc_pair_data {
        use super::*;
        pub static COORD_TABLE: LazyLock<Vec<Vec<Vec<vec2i::Vec2i>>>> = LazyLock::new(|| vec![vec![Vec::new(); 7]; 7]);
        #[derive(Clone, Debug)]
        pub struct PoissonDiscPairData {
            pub coord_data: Vec<vec2i::Vec2i>,
            pub sectors: i32,
        }
        impl PoissonDiscPairData {
            pub fn new(rad1: i32, rad2: i32) -> Self {
                let mut s = Self { coord_data: Vec::new(), sectors: 0i32 };
                let idx1: i32 = rad1 - 2;
                let idx2: i32 = rad2 - 2;
                s.coord_data = (((*poisson_disc_pair_data::COORD_TABLE)[idx1 as usize])[idx2 as usize]).to_vec();
                s.sectors = (s.coord_data.len() as i32);
                s
            }
            // PoissonDiscPairData.getCoords(1): not translated, it creates a Vec2i with a constructor that isn't translated.
            // PoissonDiscPairData.getSector(1): not translated, it calls PoissonDiscPairData.getCoords, which isn't translated.
        }
        // PoissonDiscPairData.createPairData(5): not translated, an assignment inside an expression.
        pub fn uncompress_curve(code_size: i32, curve_code: i64) -> Vec<i8> {
            let mut wave: Vec<i8> = vec![0i8; (code_size + 2) as usize];
            for i in 0..=code_size {
                wave[(i + 1) as usize] = (((wave[i as usize] as i64) + ((curve_code >> i) & 1i64)) as i8);
            }
            return wave.to_vec();
        }
    }

    /// `SimpleBitmap` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java)
    pub mod simple_bitmap {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SimpleBitmap {
            pub height: i32,
            pub width: i32,
            pub bits: Option<Vec<i32>>,
            pub touched: bool,
        }
        impl SimpleBitmap {
            // SimpleBitmap.SimpleBitmap(2): not translated, unknown name `class_3532`.
            // SimpleBitmap.SimpleBitmap(3): not translated, it calls a constructor that isn't translated.
            // SimpleBitmap.SimpleBitmap(1): not translated, it calls a constructor that isn't translated.
            pub fn get_bits(&self) -> Option<Vec<i32>> {
                return self.bits.clone();
            }
            pub fn get_height(&self) -> i32 {
                return self.height;
            }
            pub fn get_width(&self) -> i32 {
                return self.width;
            }
            // SimpleBitmap.clear(0): not translated, `this` used as a value.
            // SimpleBitmap.isColliding(3): not translated, `--` inside an expression.
            // SimpleBitmap.blitOr(3): not translated, `--` inside an expression.
            // SimpleBitmap.blitSub(3): not translated, `--` inside an expression.
            // SimpleBitmap.blitAnd(3): not translated, `--` inside an expression.
            // SimpleBitmap.prepBlit(3): not translated, it calls BlitPreparationResult.failure, which isn't translated.
            pub fn set_pixel(&mut self, x: i32, y: i32, mut mode: i32) {
                if (((x >= 0) && (y >= 0)) && (x < self.width)) && (y < self.height) {
                    mode &= 1;
                    (self.bits.as_mut().expect("null"))[y as usize] = (((self.bits.as_ref().expect("null"))[y as usize]) & (!(mode << x))) | (mode << x);
                    self.touched = true;
                }
            }
            pub fn is_pixel_on(&self, x: i32, y: i32) -> bool {
                return ((((x >= 0) && (y >= 0)) && (x < self.width)) && (y < self.height)) && (((((self.bits.as_ref().expect("null"))[y as usize]) >> x) & 1) == 1);
            }
            pub fn is_row_blank(&self, y: i32) -> bool {
                return ((y >= 0) && (y < self.height)) && (((self.bits.as_ref().expect("null"))[y as usize]) == 0);
            }
            pub fn print(&self) {
                for y in 0..self.height {
                    let mut buff: String = String::new();
                    for x in 0..self.width {
                        buff.push_str(&(if self.is_pixel_on(x, y) { "\u{2588}" } else { "\u{2591}" }));
                    }
                    // log:
                }
            }
        }
    }

    /// `SimpleBitmap.BlitPreparationResult` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java)
    pub mod simple_bitmap_blit_preparation_result {
        use super::*;
        pub static FAILURE: LazyLock<simple_bitmap_blit_preparation_result::BlitPreparationResult> = LazyLock::new(|| simple_bitmap_blit_preparation_result::BlitPreparationResult::new(false, 0, 0, 0));
        #[derive(Clone, Debug)]
        pub struct BlitPreparationResult {
            pub success: bool,
            pub dst_offset_y: i32,
            pub src_offset_y: i32,
            pub run_h: i32,
        }
        impl BlitPreparationResult {
            pub fn new(success: bool, dst_offset_y: i32, src_offset_y: i32, run_h: i32) -> Self {
                let mut s = Self { success: success, dst_offset_y: dst_offset_y, src_offset_y: src_offset_y, run_h: run_h };
                s.success = success;
                s.dst_offset_y = dst_offset_y;
                s.src_offset_y = src_offset_y;
                s.run_h = run_h;
                s
            }
        }
        // SimpleBitmap.BlitPreparationResult.failure(0): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        pub fn success(dst_offset_y: i32, src_offset_y: i32, run_h: i32) -> simple_bitmap_blit_preparation_result::BlitPreparationResult {
            return simple_bitmap_blit_preparation_result::BlitPreparationResult::new(true, dst_offset_y, src_offset_y, run_h);
        }
    }

    /// `UniversalPoissonDiscProvider` (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java)
    pub mod universal_poisson_disc_provider {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_2960`. Its static parts are below.
        pub const CIRCLE_DATA_ID: &str = "GTCD";
        // UniversalPoissonDiscProvider.createCircleProvider(1): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // UniversalPoissonDiscProvider.getProvider(1): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // UniversalPoissonDiscProvider.getPoissonDiscs(2): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // UniversalPoissonDiscProvider.unloadWorld(1): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // UniversalPoissonDiscProvider.setChunkPoissonData(3): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // UniversalPoissonDiscProvider.getChunkPoissonData(2): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
        // UniversalPoissonDiscProvider.unloadChunkPoissonData(2): not translated, its class isn't translated (it uses Minecraft's `class_2960`).
    }

    /// `Vec2i` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java)
    pub mod vec2i {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Vec2i {
            pub x: i32,
            pub z: i32,
            pub tight: bool,
        }
        impl Vec2i {
            pub fn new() -> Self {
                let mut s = Self { x: 0i32, z: 0i32, tight: false };
                s
            }
            // Vec2i.Vec2i(2): not translated, it calls a constructor that isn't translated.
            // Vec2i.Vec2i(3): not translated, it calls Vec2i.set, which isn't translated.
            // Vec2i.Vec2i(1): not translated, it calls Vec2i.setTight, which isn't translated.
            // Vec2i.set(2): not translated, `this` used as a value.
            // Vec2i.set(1): not translated, it calls Vec2i.set, which isn't translated.
            // Vec2i.setTight(1): not translated, `this` used as a value.
            pub fn is_tight(&self) -> bool {
                return self.tight;
            }
            // Vec2i.add(2): not translated, `this` used as a value.
            // Vec2i.sub(2): not translated, `this` used as a value.
            // Vec2i.add(1): not translated, it calls Vec2i.add, which isn't translated.
            // Vec2i.sub(1): not translated, it calls Vec2i.sub, which isn't translated.
            pub fn len(&self) -> f64 {
                return (((self.x * self.x) + (self.z * self.z)) as f64).sqrt();
            }
            pub fn angle(&self) -> f64 {
                return (self.z as f64).atan2((self.x as f64));
            }
            // Vec2i.equals(1): not translated, it uses `Object`.
            pub fn hash_code(&self) -> i32 {
                return self.x ^ (self.z * 98764313);
            }
            pub fn to_string(&self) -> String {
                return format!("Coord {},{},{}", self.x, self.z, if self.tight { "T" } else { "L" });
            }
        }
        pub fn cross_product(c1: &vec2i::Vec2i, c2: &vec2i::Vec2i) -> i32 {
            return (c1.x * c2.z) - (c1.z * c2.x);
        }
    }

    /// `Vec2iPCA` (com/dtteam/dynamictrees/systems/poissondisc/Vec2iPCA.java)
    pub mod vec2i_pca {
        use super::*;
        // The class itself isn't translated: it extends `Vec2i`. Its static parts are below.
        // Vec2iPCA.Vec2iPCA(3): not translated, its class isn't translated (it extends `Vec2i`).
        // Vec2iPCA.set(2): not translated, its class isn't translated (it extends `Vec2i`).
        // Vec2iPCA.angle(0): not translated, its class isn't translated (it extends `Vec2i`).
    }

    /// `ActiveSeasonGrowthCalculator` (com/dtteam/dynamictrees/systems/season/ActiveSeasonGrowthCalculator.java)
    pub mod active_season_growth_calculator {
        use super::*;
        // The class itself isn't translated: it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season). Its static parts are below.
        // ActiveSeasonGrowthCalculator.clippedSineWave(4): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
        // ActiveSeasonGrowthCalculator.peakClimateOffset(1): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
        // ActiveSeasonGrowthCalculator.calcGrowthRate(2): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
        // ActiveSeasonGrowthCalculator.calcSeedDropRate(2): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
        // ActiveSeasonGrowthCalculator.calcFruitProductionRate(2): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
        // ActiveSeasonGrowthCalculator.getPeakFruitProductionSeasonValue(1): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
    }

    /// `ClimateHelper` (com/dtteam/dynamictrees/systems/season/ClimateHelper.java)
    pub mod climate_helper {
        use super::*;
        pub static CLIMATE_MULTIPLIER_TABLE: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64], vec![1.0f64, 1.0f64, 0.6f64, 0.9f64, 0.4f64], vec![1.0f64, 0.7f64, 1.0f64, 0.6f64, 0.1f64], vec![1.0f64, 0.6f64, 0.7f64, 1.0f64, 0.3f64], vec![1.0f64, 0.9f64, 0.4f64, 0.8f64, 1.0f64]]);
        #[derive(Clone, Debug)]
        pub struct ClimateHelper {
        }
        impl ClimateHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClimateHelper.climateMultiplier(3): not translated, it's overloaded with the same number of parameters.
        // ClimateHelper.climateMultiplier(3): not translated, it's overloaded with the same number of parameters.
        // ClimateHelper.getClimate(2): not translated, it uses ClimateZoneType, which isn't translated (its constants carry values).
    }

    /// `NormalSeasonManager` (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java)
    pub mod normal_season_manager {
        use super::*;
        // The class itself isn't translated: it implements `SeasonManager` (com.dtteam.dynamictrees.api.season). Its static parts are below.
        // NULL: not translated, it uses SeasonManager, which isn't translated (it's an interface).
        pub const ARID_THRESHHOLD: f32 = 1.0f32;
        pub const ARID_DOWNFALL_THRESHHOLD: f32 = 0.0f32;
        pub const TROPICAL_THRESHHOLD: f32 = 0.8f32;
        pub const COLD_THRESHHOLD: f32 = 0.2f32;
        // NormalSeasonManager.NormalSeasonManager(0): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.NormalSeasonManager(1): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.createProvider(1): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.getContext(1): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.flushMappings(0): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.getBiomeForDist(2): not translated, it uses Minecraft's `class_1959`.
        // NormalSeasonManager.setTropicalPredicate(1): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.setAridPredicate(1): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.setColdPredicate(1): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.getClimate(2): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.updateTick(2): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.getGrowthFactor(3): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.getSeedDropFactor(3): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.getFruitProductionFactor(3): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.getSeasonValue(2): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.getPeakFruitProductionSeasonValue(3): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.shouldSnowMelt(2): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
        // NormalSeasonManager.clearCache(1): not translated, its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)).
    }

    /// `NullSeasonGrowthCalculator` (com/dtteam/dynamictrees/systems/season/NullSeasonGrowthCalculator.java)
    pub mod null_season_growth_calculator {
        use super::*;
        // The class itself isn't translated: it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season). Its static parts are below.
        // NullSeasonGrowthCalculator.calcGrowthRate(2): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
        // NullSeasonGrowthCalculator.calcSeedDropRate(2): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
        // NullSeasonGrowthCalculator.calcFruitProductionRate(2): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
        // NullSeasonGrowthCalculator.getPeakFruitProductionSeasonValue(1): not translated, its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)).
    }

    /// `NullSeasonProvider` (com/dtteam/dynamictrees/systems/season/NullSeasonProvider.java)
    pub mod null_season_provider {
        use super::*;
        // The class itself isn't translated: it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season). Its static parts are below.
        // NullSeasonProvider.getSeasonValue(2): not translated, its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)).
        // NullSeasonProvider.updateTick(2): not translated, its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)).
        // NullSeasonProvider.shouldSnowMelt(2): not translated, its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)).
    }

    /// `SeasonCompatibilityHandler` (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java)
    pub mod season_compatibility_handler {
        use super::*;
        // SEASON_MANAGERS: not translated, it uses SeasonManager, which isn't translated (it's an interface).
        // activeSeasonManager: not translated, it uses SeasonManager, which isn't translated (it's an interface).
        pub const DISABLED: &str = "!";
        pub const ANY: &str = "*";
        #[derive(Clone, Debug)]
        pub struct SeasonCompatibilityHandler {
        }
        impl SeasonCompatibilityHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SeasonCompatibilityHandler.getSeasonManager(0): not translated, it uses SeasonManager, which isn't translated (it's an interface).
        // SeasonCompatibilityHandler.setSeasonManager(1): not translated, it uses SeasonManager, which isn't translated (it's an interface).
        // SeasonCompatibilityHandler.registerSeasonManager(2): not translated, it uses SeasonManager, which isn't translated (it's an interface).
        // SeasonCompatibilityHandler.registerBuiltInSeasonManagers(0): not translated, it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)).
        // SeasonCompatibilityHandler.reloadSeasonManager(0): not translated, it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `SeasonContext` (com/dtteam/dynamictrees/systems/season/SeasonContext.java)
    pub mod season_context {
        use super::*;
        // The class itself isn't translated: it uses SeasonProvider, which isn't translated (it's an interface). Its static parts are below.
        pub const UPDATE_RATE: i32 = 20;
        pub const SEASON_VALUE_CACHE_MULTIPLIER: f32 = 256.0f32;
        // SeasonContext.SeasonContext(2): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
        // SeasonContext.updateTick(2): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
        // SeasonContext.getSeasonProvider(0): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
        // SeasonContext.getCalculator(0): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
        // SeasonContext.getGrowthFactor(2): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
        // SeasonContext.getSeedDropFactor(2): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
        // SeasonContext.getFruitProductionFactor(2): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
        // SeasonContext.getPeakFruitProductionSeasonValue(2): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
        // SeasonContext.clearCache(0): not translated, its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)).
    }

    /// `SeasonHelper` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java)
    pub mod season_helper {
        use super::*;
        pub const SPRING_START: f32 = 0.0f32;
        pub const SPRING_MIDDLE: f32 = 0.5f32;
        pub const SUMMER_START: f32 = 1.0f32;
        pub const SUMMER_MIDDLE: f32 = 1.5f32;
        pub const AUTUMN_START: f32 = 2.0f32;
        pub const AUTUMN_MIDDLE: f32 = 2.5f32;
        pub const WINTER_START: f32 = 3.0f32;
        pub const WINTER_MIDDLE: f32 = 3.5f32;
        #[derive(Clone, Debug)]
        pub struct SeasonHelper {
        }
        impl SeasonHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SeasonHelper.updateTick(2): not translated, it uses Minecraft's `class_1937`.
        // SeasonHelper.globalSeasonalGrowthFactor(3): not translated, it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`).
        // SeasonHelper.globalSeasonalSeedDropFactor(3): not translated, it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`).
        // SeasonHelper.globalSeasonalFruitProductionFactor(3): not translated, it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`).
        // SeasonHelper.getPeakFruitProductionSeason(3): not translated, it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`).
        // SeasonHelper.getSeasonValue(2): not translated, it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`).
        pub fn is_season_between(mut test_value: f32, mut season_a: f32, mut season_b: f32) -> bool {
            test_value = test_value % 4.0f32;
            season_a %= 4.0f32;
            season_b %= 4.0f32;
            return if season_a <= season_b { (test_value > season_a) && (test_value < season_b) } else { (test_value < season_b) || (test_value > season_a) };
        }
        // SeasonHelper.shouldSnowMelt(2): not translated, it uses Minecraft's `class_1937`.
    }

    /// `SeedSaplingRecipe` (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java)
    pub mod seed_sapling_recipe {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_1792`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // SeedSaplingRecipe.SeedSaplingRecipe(2): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.SeedSaplingRecipe(1): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.addExtraIngredientForSaplingToSeed(1): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.addExtraIngredientForSeedToSapling(1): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.setCanCraftSaplingToSeed(1): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.setCanCraftSeedToSapling(1): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.setReplaceSaplingWhenPlaced(1): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.setReplaceSaplingWhenGrown(1): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.getSaplingItem(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.setSaplingItem(1): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.isValid(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.getSaplingBlock(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.getIngredientsForSaplingToSeed(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.getIngredientsForSeedToSapling(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.canCraftSaplingToSeed(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.canCraftSeedToSapling(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.shouldReplaceSaplingWhenPlaced(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
        // SeedSaplingRecipe.shouldReplaceSaplingWhenGrown(0): not translated, its class isn't translated (it uses Minecraft's `class_1792`).
    }

    /// `DepleteSubstance` (com/dtteam/dynamictrees/systems/substance/DepleteSubstance.java)
    pub mod deplete_substance {
        use super::*;
        // The class itself isn't translated: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance). Its static parts are below.
        // DepleteSubstance.apply(2): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // DepleteSubstance.getName(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // DepleteSubstance.setAmount(1): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // DepleteSubstance.isLingering(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
    }

    /// `FertilizeSubstance` (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java)
    pub mod fertilize_substance {
        use super::*;
        // The class itself isn't translated: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance). Its static parts are below.
        // FertilizeSubstance.apply(2): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // FertilizeSubstance.getName(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // FertilizeSubstance.setAmount(1): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // FertilizeSubstance.setGrow(1): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // FertilizeSubstance.setPulses(1): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // FertilizeSubstance.setDisplayParticles(1): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // FertilizeSubstance.setPulses(1): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // FertilizeSubstance.isLingering(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
    }

    /// `GrowthSubstance` (com/dtteam/dynamictrees/systems/substance/GrowthSubstance.java)
    pub mod growth_substance {
        use super::*;
        // The class itself isn't translated: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance). Its static parts are below.
        // GrowthSubstance.GrowthSubstance(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // GrowthSubstance.GrowthSubstance(2): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // GrowthSubstance.GrowthSubstance(3): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // GrowthSubstance.apply(2): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // GrowthSubstance.update(4): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // GrowthSubstance.getName(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // GrowthSubstance.isLingering(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
    }

    /// `HarvestSubstance` (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java)
    pub mod harvest_substance {
        use super::*;
        // The class itself isn't translated: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance). Its static parts are below.
        // HarvestSubstance.HarvestSubstance(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // HarvestSubstance.HarvestSubstance(5): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // HarvestSubstance.apply(2): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // HarvestSubstance.recalculateFruitPositions(3): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // HarvestSubstance.isCompatibleFruitBlock(2): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // HarvestSubstance.isCompatibleFruitBlock(1): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // HarvestSubstance.update(4): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // HarvestSubstance.getName(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // HarvestSubstance.isLingering(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
    }

    /// `LingeringSubstances` (com/dtteam/dynamictrees/systems/substance/LingeringSubstances.java)
    pub mod lingering_substances {
        use super::*;
        // LINGERING_SUBSTANCES: not translated, it uses `Class`.
        #[derive(Clone, Debug)]
        pub struct LingeringSubstances {
        }
        impl LingeringSubstances {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LingeringSubstances.registerLingeringSubstance(2): not translated, it's generic.
        // LingeringSubstances.indexOf(1): not translated, it's generic.
        // LingeringSubstances.fromIndex(1): not translated, it's generic.
    }

    /// `MegaSubstance` (com/dtteam/dynamictrees/systems/substance/MegaSubstance.java)
    pub mod mega_substance {
        use super::*;
        // The class itself isn't translated: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance). Its static parts are below.
        // MegaSubstance.apply(2): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // MegaSubstance.getName(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
        // MegaSubstance.isLingering(0): not translated, its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)).
    }

    /// `FabricModFileContainer` (com/dtteam/dynamictrees/treepack/FabricModFileContainer.java)
    pub mod fabric_mod_file_container {
        use super::*;
        // The class itself isn't translated: it extends `ModFileContainer`. Its static parts are below.
        // FabricModFileContainer.FabricModFileContainer(1): not translated, its class isn't translated (it extends `ModFileContainer`).
        // FabricModFileContainer.findResource(1): not translated, its class isn't translated (it extends `ModFileContainer`).
        // FabricModFileContainer.getModId(0): not translated, its class isn't translated (it extends `ModFileContainer`).
    }

    /// `BiomePopulatorsResourceLoader` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java)
    pub mod biome_populators_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading). Its static parts are below.
        // RESOURCE_PREPARER: not translated, it uses MultiJsonResourcePreparer, which isn't translated (it extends `AbstractResourcePreparer`).
        // LOGGER: the logger. Log calls become comments.
        pub const DEFAULT_POPULATOR: &str = "default";
        pub const SELECT: &str = "select";
        pub const METHOD: &str = "method";
        pub const APPLY: &str = "apply";
        pub const WHITE: &str = "white";
        pub const CAVE_ROOTED: &str = "cave_rooted";
        pub const ENTRY_APPLIERS: &str = "entries";
        // BiomePopulatorsResourceLoader.toLinkedList(1): not translated, it uses `LinkedList` (java.util).
        // BiomePopulatorsResourceLoader.collectBiomes(2): not translated, it uses IDTBiomeHolderSet, which isn't translated (it's an interface).
        pub fn warn_no_biomes_selected(json: &JJson) {
            if biome_populators_resource_loader::no_biomes_selected_warning_not_suppressed(json) {
                // log: "Could not get any biomes from selector:\n{}"
            }
        }
        pub fn no_biomes_selected_warning_not_suppressed(json: &JJson) -> bool {
            let suppress: Option<JJson> = json.get("suppress_none_selected");
            return ((suppress.is_none() || (!((suppress.as_ref().expect("null")).is_json_primitive()))) || (!(((suppress.as_ref().expect("null")).clone()).is_boolean()))) || (!(((suppress.as_ref().expect("null")).clone()).as_bool()));
        }
        // BiomePopulatorsResourceLoader.BiomePopulatorsResourceLoader(0): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.registerAppliers(0): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applySpecies(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applyDensity(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applyChance(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applyMultipass(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applyCaveRootedSpecies(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applyCaveRootedChance(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.getOperationOrWarn(1): not translated, it uses Result, which isn't translated (it's an interface).
        // BiomePopulatorsResourceLoader.getOperation(1): not translated, it uses Result, which isn't translated (it's an interface).
        // BiomePopulatorsResourceLoader.readCaveRootedPopulatorSection(3): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applyOnReload(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readPopulators(1): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readModPopulators(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readTreePackPopulators(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readDimensionalPopulators(1): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readDimensionalModPopulators(1): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readDimensionalTreePackPopulators(1): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readDimensionalPopulator(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readPopulator(3): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.readPopulatorSection(3): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applyCaveRootedPopulatorSection(3): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.applyWhite(4): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BiomePopulatorsResourceLoader.isDefaultPopulator(1): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
    }

    /// `FamilyResourceLoader` (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java)
    pub mod family_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // FamilyResourceLoader.FamilyResourceLoader(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FamilyResourceLoader.registerAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FamilyResourceLoader.registerMangroveAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FamilyResourceLoader.setCommonWarn(2): not translated, it uses Family, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // FamilyResourceLoader.applyLoadAppliers(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FamilyResourceLoader.setBranchProperties(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FamilyResourceLoader.postLoadOnLoad(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FamilyResourceLoader.addAcceptableSoilForRootSystem(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
    }

    /// `FeatureCancellationResourceLoader` (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java)
    pub mod feature_cancellation_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading). Its static parts are below.
        // RESOURCE_PREPARER: not translated, it uses MultiJsonResourcePreparer, which isn't translated (it extends `AbstractResourcePreparer`).
        // LOGGER: the logger. Log calls become comments.
        pub const CANCELLATION_FILE: &str = "feature_cancellers";
        pub const CANCELLATION_APPLIERS: &str = "cancellations";
        pub const CANCELLERS: &str = "cancellers";
        // FeatureCancellationResourceLoader.isCancellationFile(1): not translated, it uses Minecraft's `class_2960`.
        // FeatureCancellationResourceLoader.FeatureCancellationResourceLoader(0): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.registerAppliers(0): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.applyOnReload(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.readCancellers(1): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.readModCancellers(1): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.readTreePackCancellers(1): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.readCancellers(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.readCancellersInSection(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.applyCanceller(5): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FeatureCancellationResourceLoader.applyCancellationAppliers(3): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
    }

    /// `FruitResourceLoader` (com/dtteam/dynamictrees/treepack/loader/FruitResourceLoader.java)
    pub mod fruit_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation). Its static parts are below.
        // FruitResourceLoader.FruitResourceLoader(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FruitResourceLoader.registerAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FruitResourceLoader.applyLoadAppliers(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FruitResourceLoader.getBlockPropertiesJson(1): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FruitResourceLoader.createBlock(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FruitResourceLoader.createBlock(3): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FruitResourceLoader.getBlockRegistryName(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
    }

    /// `JoCodeResourceLoader` (com/dtteam/dynamictrees/treepack/loader/JoCodeResourceLoader.java)
    pub mod jo_code_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // JoCodeResourceLoader.JoCodeResourceLoader(0): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // JoCodeResourceLoader.applyOnReload(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // JoCodeResourceLoader.registerCodes(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // JoCodeResourceLoader.registerCodeForLine(2): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // JoCodeResourceLoader.registerCode(4): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // JoCodeResourceLoader.collectWorldGenCodes(3): not translated, its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
    }

    /// `JoCodeResourcePreparer` (com/dtteam/dynamictrees/treepack/loader/JoCodeResourcePreparer.java)
    pub mod jo_code_resource_preparer {
        use super::*;
        // The class itself isn't translated: it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation). Its static parts are below.
        // JoCodeResourcePreparer.JoCodeResourcePreparer(1): not translated, its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // JoCodeResourcePreparer.offerLine(2): not translated, its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // JoCodeResourcePreparer.shouldAddLine(1): not translated, its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // JoCodeResourcePreparer.processLine(1): not translated, its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // JoCodeResourcePreparer.withoutWhitespace(1): not translated, its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // JoCodeResourcePreparer.withoutTrailingComments(1): not translated, its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
    }

    /// `LeavesPropertiesResourceLoader` (com/dtteam/dynamictrees/treepack/loader/LeavesPropertiesResourceLoader.java)
    pub mod leaves_properties_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation). Its static parts are below.
        // LeavesPropertiesResourceLoader.LeavesPropertiesResourceLoader(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // LeavesPropertiesResourceLoader.registerAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // LeavesPropertiesResourceLoader.readDoesAge(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // LeavesPropertiesResourceLoader.applyLoadAppliers(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // LeavesPropertiesResourceLoader.readCustomBlockRegistryName(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // LeavesPropertiesResourceLoader.shouldGenerateBlocks(1): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // LeavesPropertiesResourceLoader.generateBlocks(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
    }

    /// `PodResourceLoader` (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java)
    pub mod pod_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation). Its static parts are below.
        // PodResourceLoader.PodResourceLoader(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // PodResourceLoader.registerAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // PodResourceLoader.readBlockShapes(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // PodResourceLoader.applyLoadAppliers(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // PodResourceLoader.getBlockPropertiesJson(1): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // PodResourceLoader.createBlock(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // PodResourceLoader.createBlock(3): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // PodResourceLoader.getBlockRegistryName(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
    }

    /// `SoilPropertiesResourceLoader` (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java)
    pub mod soil_properties_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // SoilPropertiesResourceLoader.SoilPropertiesResourceLoader(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SoilPropertiesResourceLoader.registerAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SoilPropertiesResourceLoader.setSubstituteWarn(2): not translated, it uses SoilProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // SoilPropertiesResourceLoader.registerAcceptableSoil(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SoilPropertiesResourceLoader.registerSpreadableAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SoilPropertiesResourceLoader.applyLoadAppliers(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SoilPropertiesResourceLoader.readCustomBlockRegistryName(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SoilPropertiesResourceLoader.postLoadOnLoad(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SoilPropertiesResourceLoader.generateSoilBlock(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
    }

    /// `SpeciesResourceLoader` (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java)
    pub mod species_resource_loader {
        use super::*;
        // The class itself isn't translated: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // SpeciesResourceLoader.SpeciesResourceLoader(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.registerAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.registerMangroveAppliers(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.setSeed(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.applySeasonalOffsets(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.setMegaSpecies(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.addAcceptableSoil(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.addAcceptableSoilForWorldGen(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.postLoadOnLoad(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.postLoadOnReload(2): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SpeciesResourceLoader.registerComposterChances(0): not translated, its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
    }

    /// `ModFileContainer` (com/dtteam/dynamictrees/treepack/ModFileContainer.java)
    pub mod mod_file_container {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModFileContainer {
        }
        impl ModFileContainer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ModFileContainer.findResource(1): not translated, it has no body.
            // ModFileContainer.getModId(0): not translated, it has no body.
        }
    }

    /// `Resettable` (com/dtteam/dynamictrees/treepack/Resettable.java)
    pub mod resettable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Resettable.reset(0): not translated, its class isn't translated (it's an interface).
        // Resettable.setPreReloadDefaults(0): not translated, its class isn't translated (it's an interface).
        // Resettable.setPostReloadDefaults(0): not translated, its class isn't translated (it's an interface).
    }

    /// `Resources` (com/dtteam/dynamictrees/treepack/Resources.java)
    pub mod resources {
        use super::*;
        pub const TREES: &str = "trees";
        // MANAGER: not translated, it uses TreeResourceManager, which isn't translated (it's an interface).
        // LEAVES_PROPERTIES_LOADER: not translated, it uses LeavesPropertiesResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SOIL_PROPERTIES_LOADER: not translated, it uses SoilPropertiesResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // FAMILY_LOADER: not translated, it uses FamilyResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // GEN_FEATURE_TEMPLATE_LOADER: not translated, it uses ConfigurationTemplateResourceLoader, which isn't translated (it's generic).
        // GROWTH_LOGIC_KIT_TEMPLATE_LOADER: not translated, it uses ConfigurationTemplateResourceLoader, which isn't translated (it's generic).
        // FRUIT_LOADER: not translated, it uses FruitResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // POD_LOADER: not translated, it uses PodResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // SPECIES_LOADER: not translated, it uses SpeciesResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)).
        // JO_CODE_LOADER: not translated, it uses JoCodeResourceLoader, which isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FEATURE_CANCELLATION_LOADER: not translated, it uses FeatureCancellationResourceLoader, which isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // BIOME_POPULATORS_LOADER: not translated, it uses BiomePopulatorsResourceLoader, which isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)).
        // FLAT_TREE_PACK_INFO: not translated, it uses Minecraft's `class_9224`.
        #[derive(Clone, Debug)]
        pub struct Resources {
        }
        impl Resources {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Resources.setupTreesResourceManager(0): not translated, it uses Services.EVENT, which isn't translated (it uses IEventHelper, which isn't translated (it's an interface)).
        // Resources.addDefaultLoaders(0): not translated, it uses Resources.MANAGER, which isn't translated (it uses TreeResourceManager, which isn't translated (it's an interface)).
        // Resources.registerModTreePacks(0): not translated, it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)).
        // Resources.addModTreePack(1): not translated, it calls ModFileContainer.findResource, which isn't translated.
        // Resources.registerFlatTreePack(0): not translated, it uses `File` (java.io).
        // Resources.getTreeFolder(0): not translated, it uses `File` (java.io).
    }

    /// `Resources.ReloadListener` (com/dtteam/dynamictrees/treepack/Resources.java)
    pub mod resources_reload_listener {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_3302`. Its static parts are below.
        // Resources.ReloadListener.ReloadListener(1): not translated, its class isn't translated (it implements Minecraft's `class_3302`).
        // Resources.ReloadListener.method_25931(6): not translated, its class isn't translated (it implements Minecraft's `class_3302`).
    }

    /// `TreePackResources` (com/dtteam/dynamictrees/treepack/TreePackResources.java)
    pub mod tree_pack_resources {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_3259`. Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // TreePackResources.TreePackResources(2): not translated, its class isn't translated (it extends Minecraft's `class_3259`).
        // TreePackResources.method_14405(2): not translated, its class isn't translated (it extends Minecraft's `class_3259`).
        // TreePackResources.getPathFromLocation(1): not translated, it uses Minecraft's `class_2960`.
        // TreePackResources.method_14408(4): not translated, its class isn't translated (it extends Minecraft's `class_3259`).
        // TreePackResources.method_14406(1): not translated, its class isn't translated (it extends Minecraft's `class_3259`).
    }

    /// `TreesResourceManager` (com/dtteam/dynamictrees/treepack/TreesResourceManager.java)
    pub mod trees_resource_manager {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `class_3300`. Its static parts are below.
        // TreesResourceManager.addLoader(1): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.addLoaders(1): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.addLoaderBefore(2): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.addLoaderAfter(2): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.registerAppliers(0): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.load(0): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.gatherData(0): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.setup(0): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.prepareReload(2): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.reload(1): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.reload(2): not translated, it's generic.
        // TreesResourceManager.addPack(1): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.method_14487(0): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.method_14486(1): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.getResourceOrThrow(1): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.method_14489(1): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.getResource(2): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.method_14488(2): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.method_41265(2): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
        // TreesResourceManager.method_29213(0): not translated, its class isn't translated (it implements Minecraft's `class_3300`).
    }

    /// `ChunkTreeHelper` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java)
    pub mod chunk_tree_helper {
        use super::*;
        pub const CHUNK_WIDTH: i32 = 16;
        pub const NONE: i8 = 0i8;
        pub const TREE: i8 = 1i8;
        pub const SURR: i8 = 2i8;
        #[derive(Clone, Debug)]
        pub struct ChunkTreeHelper {
        }
        impl ChunkTreeHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChunkTreeHelper.removeOrphanedBranchNodes(3): not translated, it uses Minecraft's `class_1937`.
        // ChunkTreeHelper.removeAllBranchesFromChunk(3): not translated, it uses Minecraft's `class_1937`.
        // ChunkTreeHelper.getEffectiveBlockBounds(3): not translated, it uses BlockPosBounds, which isn't translated (it implements `Iterable`).
        // ChunkTreeHelper.getTopFilledSegment(1): not translated, it uses Minecraft's `class_2818`.
        // ChunkTreeHelper.doTreeDestroy(3): not translated, it uses Minecraft's `class_1937`.
        // ChunkTreeHelper.cleanupNeighbors(2): not translated, it uses Minecraft's `class_1937`.
        // ChunkTreeHelper.cleanupBlock(2): not translated, it uses Minecraft's `class_1937`.
        // ChunkTreeHelper.cleanupVines(2): not translated, it uses Minecraft's `class_1937`.
        // ChunkTreeHelper.canCheckSurroundings(2): not translated, it uses Minecraft's `class_1936`.
        // ChunkTreeHelper.canCheckSurroundings(3): not translated, it uses Minecraft's `class_1936`.
        // ChunkTreeHelper.isSurroundedByLoadedChunks(2): not translated, it uses Minecraft's `class_1937`.
        // ChunkTreeHelper.canAccessStateSafely(2): not translated, it uses Minecraft's `class_1922`.
        // ChunkTreeHelper.getStateSafe(2): not translated, it uses Minecraft's `class_2680`.
    }

    /// `Family` (com/dtteam/dynamictrees/tree/family/Family.java)
    pub mod family {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // blockStateGenerators: not translated, it uses Minecraft's `class_2960`.
        // itemModelGenerators: not translated, it uses Minecraft's `class_2960`.
        // languageGenerators: not translated, it uses Minecraft's `class_2960`.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // NULL_FAMILY: not translated, its value couldn't be read: an anonymous class.
        // REGISTRY: not translated, it uses TypedRegistry, which isn't translated (it's generic).
        pub const BRANCH: &str = "branch";
        pub const BRANCH_TOP: &str = "branch_top";
        pub const STRIPPED_BRANCH: &str = "stripped_branch";
        pub const STRIPPED_BRANCH_TOP: &str = "stripped_branch_top";
        pub const ROOTS_SIDE: &str = "roots_side";
        pub const ROOTS_TOP: &str = "roots_top";
        pub const COVERED_ROOTS_BLOCK: &str = "covered_roots_block";
        // Family.Family(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.Family(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setupBlocks(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setCommonSpecies(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getCommonSpecies(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.addSpecies(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getSpecies(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getSpeciesForLocation(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getSpeciesForLocation(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.onTreeActivated(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.canStripBranch(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.stripBranch(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.isWood(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchName(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchNameSuffix(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.createBranchBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.createBranch(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.createBranchItem(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setBranch(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setStrippedBranch(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setupBranch(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setBranchItem(1): not translated, it's generic.
        // Family.getBranch(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchForPlacement(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchForRootsPlacement(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getStrippedBranch(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchItem(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.isThick(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getMaxBranchRadius(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setMaxBranchRadius(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getRootColor(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setStick(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getStick(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setPrimitiveLog(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setPrimitiveStrippedLog(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getPrimitiveLog(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getPrimitiveStrippedLog(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getLogDropsForBranch(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.isFireProof(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setIsFireProof(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getDefaultBranchHarvestTier(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getDefaultStrippedBranchHarvestTier(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getDefaultBranchMapColor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getDefaultBranchSoundType(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getDefaultBranchProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getLootVolumeMultiplier(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setLootVolumeMultiplier(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getRadiusForCellKit(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setPrimaryThickness(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setSecondaryThickness(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getPrimaryThickness(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getSecondaryThickness(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.hasStrippedBranch(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setHasStrippedBranch(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getMinRadiusForStripping(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setMinRadiusForStripping(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.reduceRadiusWhenStripping(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setReduceRadiusWhenStripping(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.addValidBranches(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchBlockIndex(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getValidBranchBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.isValidBranchBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getNumberOfValidBranchBlocks(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setBranchIsLadder(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.branchIsLadder(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getMaxSignalDepth(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setMaxSignalDepth(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.hasSurfaceRoot(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setHasSurfaceRoot(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.createSurfaceRoot(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getSurfaceRoot(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setSurfaceRoot(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.isAcceptableSoilForRootSystem(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.hasRootSystem(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.selectAnimationHandler(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.expandLeavesBlockBounds(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.isCompatibleDynamicLeaves(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.isCompatibleGenericLeaves(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getCommonLeaves(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setCommonLeaves(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.defaultBranchTags(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.defaultBranchItemTags(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.defaultStrippedBranchTags(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.addGeneratedBlockTags(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.tierTag(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.addGeneratedItemTags(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getSurfaceRootLoader(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchLoader(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getRootsLoader(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getBranchItemParentLocation(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getRootItemParentLocation(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.generateStateData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.generateItemModelData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.generateLangData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setOnlyIfLoaded(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.isOnlyIfLoaded(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setTextureOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getTexturePath(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setModelOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getModelPath(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.setLangOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.getLangOverride(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.addBranchTextures(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.addRootTextures(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.topBranchTextureLocations(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.topBranchTextureLocation(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.toLoadDataString(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Family.toReloadDataString(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
    }

    /// `Family.TreeActivationContext` (com/dtteam/dynamictrees/tree/family/Family.java)
    pub mod family_tree_activation_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_1937`. Its static parts are below.
        // Family.TreeActivationContext.TreeActivationContext(8): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // Family.TreeActivationContext.level(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // Family.TreeActivationContext.rootPos(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // Family.TreeActivationContext.hitPos(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // Family.TreeActivationContext.hitState(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // Family.TreeActivationContext.player(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // Family.TreeActivationContext.hand(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // Family.TreeActivationContext.heldItem(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
        // Family.TreeActivationContext.hitResult(0): not translated, its class isn't translated (it uses Minecraft's `class_1937`).
    }

    /// `NetherFungusFamily` (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java)
    pub mod nether_fungus_family {
        use super::*;
        // The class itself isn't translated: it extends `Family`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // NetherFungusFamily.NetherFungusFamily(1): not translated, its class isn't translated (it extends `Family`).
        // NetherFungusFamily.setCommonSpecies(1): not translated, its class isn't translated (it extends `Family`).
        // NetherFungusFamily.setPreReloadDefaults(0): not translated, its class isn't translated (it extends `Family`).
        // NetherFungusFamily.isFireProof(0): not translated, its class isn't translated (it extends `Family`).
        // NetherFungusFamily.expandLeavesBlockBounds(1): not translated, its class isn't translated (it extends `Family`).
        // NetherFungusFamily.defaultBranchTags(0): not translated, its class isn't translated (it extends `Family`).
        // NetherFungusFamily.defaultBranchItemTags(0): not translated, its class isn't translated (it extends `Family`).
        // NetherFungusFamily.defaultStrippedBranchTags(0): not translated, its class isn't translated (it extends `Family`).
    }

    /// `PalmFamily` (com/dtteam/dynamictrees/tree/family/PalmFamily.java)
    pub mod palm_family {
        use super::*;
        // The class itself isn't translated: it extends `Family`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // PalmFamily.PalmFamily(1): not translated, its class isn't translated (it extends `Family`).
        // PalmFamily.setCommonSpecies(1): not translated, its class isn't translated (it extends `Family`).
        // PalmFamily.setCommonLeaves(1): not translated, its class isn't translated (it extends `Family`).
        // PalmFamily.createBranchBlock(1): not translated, its class isn't translated (it extends `Family`).
    }

    /// `UndergroundRootsFamily` (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java)
    pub mod underground_roots_family {
        use super::*;
        // The class itself isn't translated: it extends `Family`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // UndergroundRootsFamily.UndergroundRootsFamily(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setCommonSpecies(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setDefaultSoil(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getDefaultSoil(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setupBlocks(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.createRoots(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.createRootsBlock(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.createRootsItem(2): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getRootsNameSuffix(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setRoots(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setRootsItem(1): not translated, it's generic.
        // UndergroundRootsFamily.getRoots(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getRootsItem(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getBranchForRootsPlacement(3): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.generateStateData(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.generateItemModelData(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getBranchItemParentLocation(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.defaultRootsTags(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getDefaultRootsHarvestTier(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.isAcceptableSoilForRootSystem(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.addAcceptableSoilsForRootSystem(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.hasRootSystem(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getPrimaryRootThickness(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getSecondaryRootThickness(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getSupportedRootThicknessExtra(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setPrimaryRootThickness(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setSecondaryRootThickness(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setSupportedRootThicknessExtra(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setPrimitiveRoots(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setPrimitiveRootsFilled(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.setPrimitiveRootsCovered(1): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getPrimitiveRoots(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getPrimitiveFilledRoots(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.getPrimitiveCoveredRoots(0): not translated, its class isn't translated (it extends `Family`).
        // UndergroundRootsFamily.addGeneratedBlockTags(1): not translated, its class isn't translated (it extends `Family`).
    }

    /// `NetherFungusSpecies` (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java)
    pub mod nether_fungus_species {
        use super::*;
        // The class itself isn't translated: it extends `Species`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // NetherFungusSpecies.NetherFungusSpecies(3): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.setDefaultGrowingParameters(0): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.setStandardSoils(0): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.setPreReloadDefaults(0): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.setPostReloadDefaults(0): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.isAcceptableSoilForWorldgen(3): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.defaultSeedComposterChance(0): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.defaultSaplingTags(0): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.defaultSeedTags(0): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.addSaplingTextures(3): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.falloverParticleFlingMultiplier(0): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.getFallingTreeStartSound(2): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.getFallingTreeEndSound(2): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.getFallingBranchEndSound(3): not translated, its class isn't translated (it extends `Species`).
        // NetherFungusSpecies.getFallingTreePitch(1): not translated, its class isn't translated (it extends `Species`).
    }

    /// `PalmSpecies` (com/dtteam/dynamictrees/tree/species/PalmSpecies.java)
    pub mod palm_species {
        use super::*;
        // The class itself isn't translated: it extends `Species`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // PalmSpecies.PalmSpecies(3): not translated, its class isn't translated (it extends `Species`).
        // PalmSpecies.postGrow(5): not translated, its class isn't translated (it extends `Species`).
        // PalmSpecies.transitionToTree(3): not translated, its class isn't translated (it extends `Species`).
        // PalmSpecies.postGeneration(1): not translated, its class isn't translated (it extends `Species`).
        // PalmSpecies.getFellingLeavesClusters(1): not translated, its class isn't translated (it extends `Species`).
    }

    /// `Species` (com/dtteam/dynamictrees/tree/species/Species.java)
    pub mod species {
        use super::*;
        // The class itself isn't translated: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry). Its static parts are below.
        // blockStateGenerators: not translated, it uses Minecraft's `class_2960`.
        // itemModelGenerators: not translated, it uses Minecraft's `class_2960`.
        // languageGenerators: not translated, it uses Minecraft's `class_2960`.
        // NULL_SPECIES: not translated, its value couldn't be read: an anonymous class.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // REGISTRY: not translated, it uses TypedRegistry, which isn't translated (it's generic).
        // upFirst: not translated, it uses Minecraft's `class_2350`.
        pub const DEFAULT_GROWTH_OFFSET: f32 = 0.0f32;
        pub const DEFAULT_FRUITING_OFFSET: f32 = 0.0f32;
        pub const DEFAULT_SEED_DROP_OFFSET: f32 = 1.0f32;
        pub const SAPLING: &str = "sapling";
        pub const SEED_PARENT: &str = "seed_parent";
        pub const SEED: &str = "seed";
        // Species.createDefaultType(1): not translated, it uses EntryType, which isn't translated (it's generic).
        // Species.createDefaultCodec(1): not translated, it uses Minecraft's `Codec`.
        // Species.read(1): not translated, it uses Minecraft's `DataResult`.
        // Species.Species(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.Species(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.Species(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.reset(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setPreReloadDefaults(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setPostReloadDefaults(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setDefaultGrowingParameters(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFamily(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setFamily(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setUnlocalizedName(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getLocalizedName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getUnlocalizedName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getTextComponent(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setLeavesProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getLeavesProperties(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getLeavesBlock(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getPrimitiveLeaves(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addValidLeafBlocks(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getLeafBlockIndex(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getValidLeavesProperties(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getValidLeafBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isValidLeafBlock(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.colorTreeQuads(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.leafColorMultiplier(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.defaultSeedComposterChance(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSeedStack(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.hasSeed(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSeed(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldGenerateSeed(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setShouldGenerateSeed(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setShouldGenerateSeedIfNull(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSeedName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeedName(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.generateSeed(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeed(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getVoluntaryDrops(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.createVoluntaryLootParams(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getLootTable(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getBranchesDrops(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getBranchesDrops(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getBranchesDrops(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.processVolume(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getDropsForBranchType(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.createBranchesLootParams(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.cleanDropsList(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getLogsAndSticks(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.handleVoluntaryDrops(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addPrimitiveSaplingRecipe(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldReplaceSaplingWhenPlaced(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldReplaceSaplingWhenGrown(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getPrimitiveSaplingRecipes(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setOverrideSaplingReplacementWhenCrouching(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.overrideSaplingReplacementWhenCrouching(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addPrimitiveSaplingItem(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSapling(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldGenerateSapling(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setShouldGenerateSapling(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setShouldGenerateSaplingIfNull(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.generateSapling(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSapling(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.selfOrLocationOverride(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldUseLocationOverride(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.plantSapling(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addAcceptableBlockForGrowth(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.canSaplingGrow(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setCanSaplingGrowNaturally(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.canSaplingGrowNaturally(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.canSaplingConsumeBoneMeal(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.canSaplingGrowAfterBoneMeal(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.saplingFireSpread(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.saplingFlammability(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.transitionToTree(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldTransitionToTree(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.transitionToTree(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSaplingShape(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSaplingShape(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSaplingRegName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSaplingModelName(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSaplingName(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setTintSapling(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.saplingColorMultiplier(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSaplingSound(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSaplingSound(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getPottedSapling(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.placeRootyDirtBlock(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.placeRootyDirtBlock(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.updateRootyDirtBlock(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getForceSoil(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setForceSoil(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSoilLongevity(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSoilLongevity(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isThick(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getMaxBranchRadius(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setMaxBranchRadius(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addAcceptableSoils(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addAcceptableSoilsForWorldGen(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.clearAcceptableSoils(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setStandardSoils(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.hasAcceptableSoil(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.moveGroundPosWorldgen(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isAcceptableSoil(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isAcceptableSoil(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isAcceptableSoil(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setAllowedWaterHeightForWorldgen(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getAllowedWaterHeightForWorldgen(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isAcceptableSoilForWorldgen(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isAcceptableSoilForWorldgen(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isWater(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.countWaterBlocksBelow(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setPlantableOnFluid(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isPlantableOnFluid(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.soilDestroyAction(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.doesRequireTileEntity(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.hasCommonOverride(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setCommonOverride(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldOverrideCommon(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setBasicGrowingParameters(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setTapering(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setUpProbability(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setLowestBranchHeight(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSignalEnergy(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setGrowthRate(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSignalEnergy(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getEnergy(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getGrowthRate(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getUpProbability(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getProbabilityForCurrentDir(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getLowestBranchHeight(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getTapering(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.update(8): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getEnds(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.handleRot(6): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setDoesRot(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.rot(8): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.postRot(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.rotChance(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.grow(8): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.sendGrowthSignal(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setGrowthLogicKit(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setGrowthLogicKit(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getGrowthLogicKit(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setCanBoneMealTree(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.canBoneMealTree(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.postGrow(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.handleDisease(5): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.biomeSuitability(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setClimateTolerance(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setPreferredClimate(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getPreferredClimate(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getClimateTolerance(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalFruitingOffset(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalFruitingOffset(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalGrowthOffset(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalGrowthOffset(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalSeedDropOffset(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalSeedDropOffset(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalGrowthOffset(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalSeedDropOffset(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setSeasonalFruitingOffset(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.seasonalGrowthFactor(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.seasonalSeedDropFactor(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.seasonalFruitProductionFactor(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.inheritSeasonalFruitingParametersToFruits(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.inheritSeasonalFruitingParametersToPods(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSeasonalTooltipFlags(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.showSeasonalTooltip(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isInFlowerHoldPeriod(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSubstanceEffect(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.applySubstance(6): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.onTreeActivated(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.consumePlayerItem(3): not translated, it uses Minecraft's `class_1657`.
        // Species.useDefaultWailaBody(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setAlwaysShowOnWaila(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.showSpeciesOnWaila(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getMegaSpecies(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getPreMegaSpecies(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isMegaSpecies(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setMegaSpecies(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setCanCraftMegaSeed(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.canCraftMegaSeed(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.selectAnimationHandler(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFellingLeavesClusters(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.canEncodeLeavesBlocks(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.encodeLeavesPos(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.encodeLeavesBlocks(4): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.leavesAreSolid(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.falloverParticleFlingMultiplier(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFallingTreeStartSound(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFallingTreeEndSound(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFallingTreePitch(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFallingBranchPitch(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFallingTreeHitWaterSound(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFallingBranchEndSound(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setBigTreeSoundThreshold(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.generate(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldGenerate(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getJoCode(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getRootsJoCode(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getJoCodes(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getRootsJoCodes(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addGenFeature(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addGenFeature(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.hasGenFeatures(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getGenFeatures(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.preGeneration(6): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.postGeneration(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getWorldGenTaperingFactor(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getWorldGenLeafMapHeight(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setWorldGenLeafMapHeight(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getWorldGenAgeIterations(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getNodeInflator(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getNodeInflator(2): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.coordHashCode(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.hasFruit(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.hasFruits(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addFruits(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getFruits(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.hasPod(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.hasPods(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addPods(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getPods(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getCommonSpecies(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isCommonSpecies(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isSeedCommon(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.generateStateData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.generateItemModelData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.generateLangData(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setOnlyIfLoaded(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.isOnlyIfLoaded(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setModelOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setTextureOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setLangOverrides(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getModelPath(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getTexturePath(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getLangOverride(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSaplingSmartModelLocation(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addSaplingTextures(3): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSeedParentModelLocation(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addGeneratedBlockTags(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.addGeneratedItemTags(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldGenerateVoluntaryDrops(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getVoluntaryDropsPath(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.createVoluntaryDrops(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.setDropSeeds(1): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.shouldDropSeeds(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.defaultSaplingTags(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.defaultSeedTags(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.toLoadDataString(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.toReloadDataString(0): not translated, its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.findSpecies(1): not translated, it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.findSpecies(1): not translated, it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.findSpeciesSloppy(1): not translated, it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // Species.getSpeciesDirectory(0): not translated, it uses Minecraft's `class_2960`.
    }

    /// `Species.BiomeSuitabilityEventResult` (com/dtteam/dynamictrees/tree/species/Species.java)
    pub mod species_biome_suitability_event_result {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct BiomeSuitabilityEventResult {
            pub handled: bool,
            pub suitability: f32,
        }
        impl BiomeSuitabilityEventResult {
            pub fn new(handled: bool, suitability: f32) -> Self {
                let mut s = Self { handled: handled, suitability: suitability };
                s.handled = handled;
                s.suitability = suitability;
                s
            }
            pub fn handled(&self) -> bool {
                return self.handled;
            }
            pub fn suitability(&self) -> f32 {
                return self.suitability;
            }
        }
    }

    /// `Species.CommonOverride` (com/dtteam/dynamictrees/tree/species/Species.java)
    pub mod species_common_override {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `Species.LogsAndSticks` (com/dtteam/dynamictrees/tree/species/Species.java)
    pub mod species_logs_and_sticks {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_1799`. Its static parts are below.
        // Species.LogsAndSticks.LogsAndSticks(2): not translated, its class isn't translated (it uses Minecraft's `class_1799`).
    }

    /// `SwampSpecies` (com/dtteam/dynamictrees/tree/species/SwampSpecies.java)
    pub mod swamp_species {
        use super::*;
        // The class itself isn't translated: it extends `Species`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        pub const MIN_RADIUS_FOR_SUNK_GENERATION: i32 = 5;
        // SwampSpecies.SwampSpecies(3): not translated, its class isn't translated (it extends `Species`).
        // SwampSpecies.generate(1): not translated, its class isn't translated (it extends `Species`).
    }

    /// `SwampSpecies.WaterSurfaceGenerationState` (com/dtteam/dynamictrees/tree/species/SwampSpecies.java)
    pub mod swamp_species_water_surface_generation_state {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum WaterSurfaceGenerationState {
            Rooted,
            Sunk,
            Disabled,
        }
        impl WaterSurfaceGenerationState {
            pub const VALUES: [WaterSurfaceGenerationState; 3] = [WaterSurfaceGenerationState::Rooted, WaterSurfaceGenerationState::Sunk, WaterSurfaceGenerationState::Disabled, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    WaterSurfaceGenerationState::Rooted => "ROOTED",
                    WaterSurfaceGenerationState::Sunk => "SUNK",
                    WaterSurfaceGenerationState::Disabled => "DISABLED",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "ROOTED" => WaterSurfaceGenerationState::Rooted,
                    "SUNK" => WaterSurfaceGenerationState::Sunk,
                    "DISABLED" => WaterSurfaceGenerationState::Disabled,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `UndergroundRootsSpecies` (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java)
    pub mod underground_roots_species {
        use super::*;
        // The class itself isn't translated: it extends `Species`. Its static parts are below.
        // TYPE: not translated, it uses EntryType, which isn't translated (it's generic).
        // UndergroundRootsSpecies.setMinWorldGenHeightOffset(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.setMaxWorldGenHeightOffset(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.setRootGrowthMultiplier(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.setUpdateSoilOnWaterRadius(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.UndergroundRootsSpecies(3): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.getFamily(0): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.placeRootyDirtBlock(3): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.replaceSoilBlock(4): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.postGrow(5): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.soilDestroyAction(4): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.rotChance(4): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.update(8): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.generate(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.getRootSignalEnergy(0): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.setRootSignalEnergy(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.getRootTapering(0): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.setRootTapering(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.sendGrowthSignal(5): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.getRootEnergy(2): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.setRootsGrowthLogicKit(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.setRootsGrowthLogicKit(1): not translated, its class isn't translated (it extends `Species`).
        // UndergroundRootsSpecies.getRootsGrowthLogicKit(0): not translated, its class isn't translated (it extends `Species`).
    }

    /// `TreeHelper` (com/dtteam/dynamictrees/tree/TreeHelper.java)
    pub mod tree_helper {
        use super::*;
        // NULL_TREE_PART: not translated, it uses TreePart, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct TreeHelper {
        }
        impl TreeHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TreeHelper.growPulse(2): not translated, it uses Minecraft's `class_1937`.
        // TreeHelper.ageVolume(4): not translated, it uses Minecraft's `class_1936`.
        // TreeHelper.ageVolume(6): not translated, it uses Minecraft's `class_1936`.
        // TreeHelper.getRootsJoCode(2): not translated, it uses Minecraft's `class_1937`.
        // TreeHelper.getJoCode(2): not translated, it uses Minecraft's `class_1937`.
        // TreeHelper.getJoCode(4): not translated, it uses Minecraft's `class_1937`.
        // TreeHelper.dereferenceTrunkShell(2): not translated, it uses Minecraft's `class_2338`.
        // TreeHelper.getCommonSpecies(2): not translated, it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // TreeHelper.getExactSpecies(2): not translated, it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // TreeHelper.getBestGuessSpecies(2): not translated, it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // TreeHelper.findRootNode(2): not translated, it uses Minecraft's `class_2338`.
        // TreeHelper.setCustomRootBlockDecay(1): not translated, it uses SoilBlockDecayer, which isn't translated (it's an interface).
        // TreeHelper.getCustomRootBlockDecay(0): not translated, it uses SoilBlockDecayer, which isn't translated (it's an interface).
        // TreeHelper.treeParticles(4): not translated, it uses Minecraft's `class_1937`.
        // TreeHelper.rootParticles(5): not translated, it uses Minecraft's `class_1937`.
        // TreeHelper.startAnalysisFromRoot(3): not translated, it uses Minecraft's `class_1936`.
        // TreeHelper.destroyTree(4): not translated, it uses Minecraft's `class_1937`.
        // TreeHelper.isTreePart(1): not translated, it's overloaded with the same number of parameters.
        // TreeHelper.isTreePart(1): not translated, it's overloaded with the same number of parameters.
        // TreeHelper.isTreePart(2): not translated, it uses Minecraft's `class_1936`.
        // TreeHelper.getTreePart(1): not translated, it uses TreePart, which isn't translated (it's an interface).
        // TreeHelper.getTreePart(1): not translated, it uses TreePart, which isn't translated (it's an interface).
        // TreeHelper.isBranch(1): not translated, it's overloaded with the same number of parameters.
        // TreeHelper.isBranch(1): not translated, it's overloaded with the same number of parameters.
        // TreeHelper.getBranch(1): not translated, it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.getBranch(1): not translated, it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.getBranch(1): not translated, it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.getRadius(2): not translated, it uses Minecraft's `class_1922`.
        // TreeHelper.getBranchOpt(1): not translated, it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.getBranchOpt(1): not translated, it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.getBranchOpt(1): not translated, it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.isLeaves(1): not translated, it's overloaded with the same number of parameters.
        // TreeHelper.isLeaves(1): not translated, it's overloaded with the same number of parameters.
        // TreeHelper.getLeaves(1): not translated, it uses DynamicLeavesBlock, which isn't translated (it extends Minecraft's `class_2397`).
        // TreeHelper.getLeaves(1): not translated, it uses DynamicLeavesBlock, which isn't translated (it extends Minecraft's `class_2397`).
        // TreeHelper.getLeaves(1): not translated, it uses DynamicLeavesBlock, which isn't translated (it extends Minecraft's `class_2397`).
        // TreeHelper.isRooty(1): not translated, it's overloaded with the same number of parameters.
        // TreeHelper.isRooty(1): not translated, it's overloaded with the same number of parameters.
        // TreeHelper.getRooty(1): not translated, it uses SoilBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.getRooty(1): not translated, it uses SoilBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.getRooty(1): not translated, it uses SoilBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
        // TreeHelper.getRootyOpt(1): not translated, it uses SoilBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)).
    }

    /// `CommonCollectors` (com/dtteam/dynamictrees/utility/CommonCollectors.java)
    pub mod common_collectors {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonCollectors {
        }
        impl CommonCollectors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonCollectors.toLinkedSet(0): not translated, it's generic.
        // CommonCollectors.toUnmodifiableSet(0): not translated, it's generic.
        // CommonCollectors.toUnmodifiableLinkedSet(0): not translated, it's generic.
        // CommonCollectors.toLinkedList(0): not translated, it's generic.
    }

    /// `CoordUtils` (com/dtteam/dynamictrees/utility/CoordUtils.java)
    pub mod coord_utils {
        use super::*;
        pub static COORD_XOR: JCell<i32> = JCell::new(0);
        // HORIZONTALS: not translated, it uses Minecraft's `class_2350`.
        pub static COORD_HASH_MAP: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![4111, 271, 3067], vec![7933711, 6144389, 9538033], vec![9973, 8287, 9721], vec![7211, 5437, 9613]]);
        #[derive(Clone, Debug)]
        pub struct CoordUtils {
        }
        impl CoordUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CoordUtils.getRandomDir(1): not translated, it uses Minecraft's `class_2350`.
        // CoordUtils.getRayTraceFruitPos(5): not translated, it uses Minecraft's `class_2338`.
        // CoordUtils.branchRayTrace(7): not translated, it uses Minecraft's `class_3965`.
        // CoordUtils.rayTraceBlocks(2): not translated, it uses Minecraft's `class_3965`.
        // CoordUtils.getRayTraceVector(3): not translated, it's generic.
        // CoordUtils.findWorldSurface(3): not translated, it uses Minecraft's `class_2338`.
        // CoordUtils.findWorldSurface(3): not translated, it uses Minecraft's `class_2338`.
        // CoordUtils.inRange(3): not translated, it uses Minecraft's `class_2338`.
        // CoordUtils.coordHashCode(4): not translated, it uses Minecraft's `class_2338`.
        // CoordUtils.coordHashCode(2): not translated, it uses Minecraft's `class_2338`.
        // CoordUtils.goHorSides(1): not translated, it uses Minecraft's `class_2338`.
        // CoordUtils.goHorSides(2): not translated, its body couldn't be read: an anonymous class.
    }

    /// `CoordUtils.CustomRayTraceContext` (com/dtteam/dynamictrees/utility/CoordUtils.java)
    pub mod coord_utils_custom_ray_trace_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `class_243`. Its static parts are below.
        // CoordUtils.CustomRayTraceContext.CustomRayTraceContext(4): not translated, its class isn't translated (it uses Minecraft's `class_243`).
        // CoordUtils.CustomRayTraceContext.getEndVector(0): not translated, its class isn't translated (it uses Minecraft's `class_243`).
        // CoordUtils.CustomRayTraceContext.getStartVector(0): not translated, its class isn't translated (it uses Minecraft's `class_243`).
        // CoordUtils.CustomRayTraceContext.getBlockShape(3): not translated, its class isn't translated (it uses Minecraft's `class_243`).
        // CoordUtils.CustomRayTraceContext.getFluidShape(3): not translated, its class isn't translated (it uses Minecraft's `class_243`).
    }

    /// `CoordUtils.Surround` (com/dtteam/dynamictrees/utility/CoordUtils.java)
    pub mod coord_utils_surround {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CoordUtils.Surround.Surround(2): not translated, its class isn't translated (its constants carry values).
        // CoordUtils.Surround.method_15434(0): not translated, its class isn't translated (its constants carry values).
        // CoordUtils.Surround.getOffset(0): not translated, its class isn't translated (its constants carry values).
        // CoordUtils.Surround.getOffsetPos(0): not translated, its class isn't translated (its constants carry values).
        // CoordUtils.Surround.getOpposite(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EntityUtils` (com/dtteam/dynamictrees/utility/EntityUtils.java)
    pub mod entity_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityUtils {
        }
        impl EntityUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EntityUtils.playerRayTrace(3): not translated, it uses Minecraft's `class_3965`.
        // EntityUtils.getHitDirection(1): not translated, it uses Minecraft's `class_2350`.
    }

    /// `ItemUtils` (com/dtteam/dynamictrees/utility/ItemUtils.java)
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
        // ItemUtils.spawnItemStack(3): not translated, it uses Minecraft's `class_1937`.
        // ItemUtils.spawnItemStack(4): not translated, it uses Minecraft's `class_1937`.
        // ItemUtils.damageAxe(5): not translated, it uses Minecraft's `class_1309`.
        // ItemUtils.getEnchantmentLevel(3): not translated, it uses Minecraft's `class_5321`.
        // ItemUtils.getEnchantment(2): not translated, it uses Minecraft's `class_6883`.
    }

    /// `MathUtils` (com/dtteam/dynamictrees/utility/MathUtils.java)
    pub mod math_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MathUtils {
        }
        impl MathUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MathUtils.selectRandomFromDistribution(2): not translated, it uses Minecraft's `class_5819`.
        pub fn short_degrees_dist(ang1: f32, ang2: f32) -> f32 {
            let max: f32 = 360.0f32;
            let da: f32 = (ang2 - ang1) % 360.0f32;
            return ((2.0f32 * da) % 360.0f32) - da;
        }
        pub fn angle_degrees_interpolate(ang1: f32, ang2: f32, t: f32) -> f32 {
            return ang1 + (math_utils::short_degrees_dist(ang1, ang2) * t);
        }
        // MathUtils.randomBetween(3): not translated, it uses Minecraft's `class_5819`.
    }

    /// `NullUtils` (com/dtteam/dynamictrees/utility/NullUtils.java)
    pub mod null_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NullUtils {
        }
        impl NullUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NullUtils.isNotNull(1): not translated, it uses `Object`.
        // NullUtils.consumeIfNonnull(2): not translated, it's generic.
        // NullUtils.applyIfNonnull(2): not translated, it's generic.
        // NullUtils.applyIfNonnull(3): not translated, it's generic.
    }

    /// `Optionals` (com/dtteam/dynamictrees/utility/Optionals.java)
    pub mod optionals {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Optionals {
        }
        impl Optionals {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Optionals.ofBlock(1): not translated, it's generic.
        // Optionals.ofBlock(1): not translated, it's generic.
        // Optionals.ofItem(1): not translated, it's generic.
        // Optionals.ofItem(1): not translated, it's generic.
        // Optionals.ifAllPresent(3): not translated, it's generic.
        // Optionals.ifAllPresent(4): not translated, it's generic.
    }

    /// `ResourceLocationUtils` (com/dtteam/dynamictrees/utility/ResourceLocationUtils.java)
    pub mod resource_location_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ResourceLocationUtils {
        }
        impl ResourceLocationUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ResourceLocationUtils.parse(2): not translated, it uses Minecraft's `class_2960`.
        // ResourceLocationUtils.namespace(2): not translated, it uses Minecraft's `class_2960`.
        // ResourceLocationUtils.prefix(2): not translated, it uses Minecraft's `class_2960`.
        // ResourceLocationUtils.suffix(2): not translated, it uses Minecraft's `class_2960`.
        // ResourceLocationUtils.surround(3): not translated, it uses Minecraft's `class_2960`.
        // ResourceLocationUtils.parseDTLocation(1): not translated, it uses Minecraft's `class_2960`.
        // ResourceLocationUtils.parseDTLocation(1): not translated, it uses Minecraft's `class_2960`.
    }

    /// `BiomeDatabase` (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java)
    pub mod biome_database {
        use super::*;
        // The class itself isn't translated: it uses IDTBiomeHolderSet, which isn't translated (it's an interface). Its static parts are below.
        // BiomeDatabase.getJsonEntry(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getEntry(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getEntry(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getEntry(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getAllEntries(0): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.reset(0): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.isPopulated(0): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getSpecies(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getChance(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getDensitySelector(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getForestness(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getHeightmap(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.getMultipass(1): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.setForestness(2): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.setHeightmap(2): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.setMultipass(2): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabase.copyOf(1): not translated, it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
    }

    /// `BiomeDatabase.BaseEntry` (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java)
    pub mod biome_database_base_entry {
        use super::*;
        // The class itself isn't translated: it implements `EntryReader`. Its static parts are below.
        // defaultMultipass: not translated, a function kept in a variable or field (only parameters can be).
        // BiomeDatabase.BaseEntry.BaseEntry(2): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getDatabase(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getBiomeKey(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getChanceSelector(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getDensitySelector(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getSpeciesSelector(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setChanceSelector(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setChanceSelector(2): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setDensitySelector(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setDensitySelector(2): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setSpeciesSelector(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setSpeciesSelector(2): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getFeatureCancellation(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getOrCreateFeatureCancellation(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setBlacklisted(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.isBlacklisted(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setForestness(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setHeightmap(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getForestness(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getHeightmap(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setMultipass(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.getMultipass(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.enableDefaultMultipass(0): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.setCustomMultipass(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.deserialiseCustomMultipass(1): not translated, its class isn't translated (it implements `EntryReader`).
        // BiomeDatabase.BaseEntry.reset(0): not translated, its class isn't translated (it implements `EntryReader`).
    }

    /// `BiomeDatabase.CaveRootedData` (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java)
    pub mod biome_database_cave_rooted_data {
        use super::*;
        // The class itself isn't translated: it uses SpeciesSelector, which isn't translated (it's an interface). Its static parts are below.
        // BiomeDatabase.CaveRootedData.shouldGenerateOnSurface(0): not translated, its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)).
        // BiomeDatabase.CaveRootedData.setGenerateOnSurface(1): not translated, its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)).
        // BiomeDatabase.CaveRootedData.getMaxDistToSurface(0): not translated, its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)).
        // BiomeDatabase.CaveRootedData.setMaxDistToSurface(1): not translated, its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)).
        // BiomeDatabase.CaveRootedData.getCaveRootedSpeciesSelector(0): not translated, its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)).
        // BiomeDatabase.CaveRootedData.getCaveRootedChanceSelector(0): not translated, its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)).
        // BiomeDatabase.CaveRootedData.setCaveRootedSpeciesSelector(2): not translated, its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)).
        // BiomeDatabase.CaveRootedData.setCaveRootedChanceSelector(2): not translated, its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)).
    }

    /// `BiomeDatabase.Entry` (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java)
    pub mod biome_database_entry {
        use super::*;
        // The class itself isn't translated: it extends `BaseEntry`. Its static parts are below.
        // BiomeDatabase.Entry.Entry(2): not translated, its class isn't translated (it extends `BaseEntry`).
        // BiomeDatabase.Entry.getCaveRootedData(0): not translated, its class isn't translated (it extends `BaseEntry`).
        // BiomeDatabase.Entry.hasCaveRootedData(0): not translated, its class isn't translated (it extends `BaseEntry`).
        // BiomeDatabase.Entry.getOrCreateCaveRootedData(0): not translated, its class isn't translated (it extends `BaseEntry`).
        // BiomeDatabase.Entry.setCaveRootedData(1): not translated, its class isn't translated (it extends `BaseEntry`).
        // BiomeDatabase.Entry.reset(0): not translated, its class isn't translated (it extends `BaseEntry`).
    }

    /// `BiomeDatabase.EntryReader` (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java)
    pub mod biome_database_entry_reader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomeDatabase.EntryReader.read(1): not translated, it uses Minecraft's `DataResult`.
        // BiomeDatabase.EntryReader.getBiomeKey(0): not translated, it has no body.
        // BiomeDatabase.EntryReader.getChanceSelector(0): not translated, it has no body.
        // BiomeDatabase.EntryReader.getDensitySelector(0): not translated, it has no body.
        // BiomeDatabase.EntryReader.getSpeciesSelector(0): not translated, it has no body.
        // BiomeDatabase.EntryReader.getFeatureCancellation(0): not translated, it has no body.
        // BiomeDatabase.EntryReader.isBlacklisted(0): not translated, it has no body.
        // BiomeDatabase.EntryReader.getForestness(0): not translated, it has no body.
        // BiomeDatabase.EntryReader.getHeightmap(0): not translated, it has no body.
        // BiomeDatabase.EntryReader.getMultipass(0): not translated, it has no body.
    }

    /// `BiomeDatabase.JsonEntry` (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java)
    pub mod biome_database_json_entry {
        use super::*;
        // The class itself isn't translated: it extends `Entry`. Its static parts are below.
        // BiomeDatabase.JsonEntry.JsonEntry(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.isForce(0): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setForce(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.copyTo(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setChanceSelector(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setChanceSelector(2): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setDensitySelector(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setDensitySelector(2): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setSpeciesSelector(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setSpeciesSelector(2): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setBlacklisted(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setForestness(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setHeightmap(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setMultipass(1): not translated, its class isn't translated (it extends `Entry`).
        // BiomeDatabase.JsonEntry.setCustomMultipass(1): not translated, its class isn't translated (it extends `Entry`).
    }

    /// `BiomeDatabase.Operation` (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java)
    pub mod biome_database_operation {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Operation {
            Replace,
            SpliceBefore,
            SpliceAfter,
        }
        impl Operation {
            pub const VALUES: [Operation; 3] = [Operation::Replace, Operation::SpliceBefore, Operation::SpliceAfter, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Operation::Replace => "REPLACE",
                    Operation::SpliceBefore => "SPLICE_BEFORE",
                    Operation::SpliceAfter => "SPLICE_AFTER",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "REPLACE" => Operation::Replace,
                    "SPLICE_BEFORE" => Operation::SpliceBefore,
                    "SPLICE_AFTER" => Operation::SpliceAfter,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BiomeDatabases` (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java)
    pub mod biome_databases {
        use super::*;
        // DEFAULT_DATABASE: not translated, it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // DIMENSIONAL_DATABASES: not translated, it uses Minecraft's `class_2960`.
        // BLACKLIST: not translated, it uses Minecraft's `class_2960`.
        #[derive(Clone, Debug)]
        pub struct BiomeDatabases {
        }
        impl BiomeDatabases {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BiomeDatabases.getDefault(0): not translated, it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabases.getDimensionalOrDefault(1): not translated, it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabases.getOrCreateDimensional(1): not translated, it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // BiomeDatabases.getDimensionalDatabases(0): not translated, it uses Minecraft's `class_2960`.
        // BiomeDatabases.isBlacklisted(1): not translated, it uses Minecraft's `class_2960`.
        // BiomeDatabases.populateBlacklistFromConfig(0): not translated, it uses DTConfigs.SERVER_CONFIG, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)).
        // BiomeDatabases.tryBlacklist(1): not translated, a try/catch (class_151).
        // BiomeDatabases.reset(0): not translated, it uses BiomeDatabases.DEFAULT_DATABASE, which isn't translated (it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface))).
    }

    /// `BiomeGenSettingsBuilderWrapper` (com/dtteam/dynamictrees/worldgen/BiomeGenSettingsBuilderWrapper.java)
    pub mod biome_gen_settings_builder_wrapper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BiomeGenSettingsBuilderWrapper {
        }
        impl BiomeGenSettingsBuilderWrapper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // BiomeGenSettingsBuilderWrapper.getPlainBuilder(0): not translated, it has no body.
            // BiomeGenSettingsBuilderWrapper.getFeatures(1): not translated, it has no body.
            // BiomeGenSettingsBuilderWrapper.getCarvers(1): not translated, it has no body.
        }
    }

    /// `BiomeRadiusCoordinator` (com/dtteam/dynamictrees/worldgen/BiomeRadiusCoordinator.java)
    pub mod biome_radius_coordinator {
        use super::*;
        // The class itself isn't translated: it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen). Its static parts are below.
        // BiomeRadiusCoordinator.BiomeRadiusCoordinator(2): not translated, its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)).
        // BiomeRadiusCoordinator.getRadiusAtCoords(2): not translated, its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)).
        // BiomeRadiusCoordinator.calcDensity(2): not translated, its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)).
        // BiomeRadiusCoordinator.calcRadius(3): not translated, its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)).
        // BiomeRadiusCoordinator.runPass(3): not translated, its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)).
    }

    /// `DynamicTreeGenerationContext` (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java)
    pub mod dynamic_tree_generation_context {
        use super::*;
        // The class itself isn't translated: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`). Its static parts are below.
        // DynamicTreeGenerationContext.DynamicTreeGenerationContext(8): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.levelContext(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.level(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.random(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.species(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.originPos(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.rootPos(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.biome(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.facing(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.radius(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.isWorldGen(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.secondChanceRegen(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
        // DynamicTreeGenerationContext.secondChance(0): not translated, its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
    }

    /// `FabricBiomeModifications` (com/dtteam/dynamictrees/worldgen/FabricBiomeModifications.java)
    pub mod fabric_biome_modifications {
        use super::*;
        // REMOVE_TREES_ID: not translated, it uses Minecraft's `class_2960`.
        // ADD_TREES_ID: not translated, it uses Minecraft's `class_2960`.
        // FEATURE_CANCELLER_EXCLUSIONS_KEY: not translated, it uses Minecraft's `class_6862`.
        #[derive(Clone, Debug)]
        pub struct FabricBiomeModifications {
        }
        impl FabricBiomeModifications {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FabricBiomeModifications.register(0): not translated, it uses FabricBiomeModifications.REMOVE_TREES_ID, which isn't translated (it uses Minecraft's `class_2960`).
        // FabricBiomeModifications.removeVanillaTrees(2): not translated, it uses Fabric's `BiomeSelectionContext`.
        // FabricBiomeModifications.addDynamicTrees(1): not translated, it uses Fabric's `BiomeModificationContext`.
    }

    /// `FeatureCancellationRegistry` (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java)
    pub mod feature_cancellation_registry {
        use super::*;
        // CANCELLATIONS: not translated, it uses Entry, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        #[derive(Clone, Debug)]
        pub struct FeatureCancellationRegistry {
        }
        impl FeatureCancellationRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FeatureCancellationRegistry.addCancellations(3): not translated, it uses IDTBiomeHolderSet, which isn't translated (it's an interface).
        // FeatureCancellationRegistry.getCancellations(0): not translated, it uses Entry, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
    }

    /// `FeatureCancellationRegistry.Entry` (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java)
    pub mod feature_cancellation_registry_entry {
        use super::*;
        // The class itself isn't translated: it uses IDTBiomeHolderSet, which isn't translated (it's an interface). Its static parts are below.
        // FeatureCancellationRegistry.Entry.Entry(3): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // FeatureCancellationRegistry.Entry.biomes(0): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // FeatureCancellationRegistry.Entry.operation(0): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
        // FeatureCancellationRegistry.Entry.cancellations(0): not translated, its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)).
    }

    /// `FeatureCancellers` (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellers.java)
    pub mod feature_cancellers {
        use super::*;
        // TREE_CANCELLER: not translated, it uses FeatureCanceller, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // ROOTED_TREE_CANCELLER: not translated, it uses FeatureCanceller, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        // FUNGUS_CANCELLER: not translated, it uses FeatureCanceller, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)).
        #[derive(Clone, Debug)]
        pub struct FeatureCancellers {
        }
        impl FeatureCancellers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FeatureCancellers.register(1): not translated, it uses Registry, which isn't translated (it's an interface).
    }

    /// `FungusFeatureCanceller` (com/dtteam/dynamictrees/worldgen/featurecancellation/FungusFeatureCanceller.java)
    pub mod fungus_feature_canceller {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FungusFeatureCanceller.FungusFeatureCanceller(2): not translated, its class isn't translated (it's generic).
        // FungusFeatureCanceller.shouldCancel(2): not translated, its class isn't translated (it's generic).
    }

    /// `MushroomFeatureCanceller` (com/dtteam/dynamictrees/worldgen/featurecancellation/MushroomFeatureCanceller.java)
    pub mod mushroom_feature_canceller {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MushroomFeatureCanceller.MushroomFeatureCanceller(2): not translated, its class isn't translated (it's generic).
        // MushroomFeatureCanceller.shouldCancel(2): not translated, its class isn't translated (it's generic).
        // MushroomFeatureCanceller.getConfigs(1): not translated, its class isn't translated (it's generic).
    }

    /// `TreeFeatureCanceller` (com/dtteam/dynamictrees/worldgen/featurecancellation/TreeFeatureCanceller.java)
    pub mod tree_feature_canceller {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TreeFeatureCanceller.TreeFeatureCanceller(2): not translated, its class isn't translated (it's generic).
        // TreeFeatureCanceller.shouldCancel(2): not translated, its class isn't translated (it's generic).
        // TreeFeatureCanceller.doesContainTrees(2): not translated, its class isn't translated (it's generic).
    }

    /// `CaveRootedTreeFeature` (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreeFeature.java)
    pub mod cave_rooted_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `DynamicTreeFeature`. Its static parts are below.
        // CaveRootedTreeFeature.method_13151(1): not translated, its class isn't translated (it extends `DynamicTreeFeature`).
        // CaveRootedTreeFeature.getSpeciesSelector(1): not translated, its class isn't translated (it extends `DynamicTreeFeature`).
        // CaveRootedTreeFeature.getChanceSelector(1): not translated, its class isn't translated (it extends `DynamicTreeFeature`).
        // CaveRootedTreeFeature.getDisc(3): not translated, its class isn't translated (it extends `DynamicTreeFeature`).
        // CaveRootedTreeFeature.getNextGroundPos(2): not translated, its class isn't translated (it extends `DynamicTreeFeature`).
    }

    /// `CaveRootedTreePlacement` (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreePlacement.java)
    pub mod cave_rooted_tree_placement {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_6797`. Its static parts are below.
        // INSTANCE: not translated, it uses CaveRootedTreePlacement, which isn't translated (it extends Minecraft's `class_6797`).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CaveRootedTreePlacement.CaveRootedTreePlacement(1): not translated, its class isn't translated (it extends Minecraft's `class_6797`).
        // CaveRootedTreePlacement.method_14452(3): not translated, its class isn't translated (it extends Minecraft's `class_6797`).
        // CaveRootedTreePlacement.method_39615(0): not translated, its class isn't translated (it extends Minecraft's `class_6797`).
    }

    /// `DTReplaceNyliumFungiBlockStateProvider` (com/dtteam/dynamictrees/worldgen/feature/DTReplaceNyliumFungiBlockStateProvider.java)
    pub mod dt_replace_nylium_fungi_block_state_provider {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_4651`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DTReplaceNyliumFungiBlockStateProvider.DTReplaceNyliumFungiBlockStateProvider(2): not translated, its class isn't translated (it extends Minecraft's `class_4651`).
        // DTReplaceNyliumFungiBlockStateProvider.method_28862(0): not translated, its class isn't translated (it extends Minecraft's `class_4651`).
        // DTReplaceNyliumFungiBlockStateProvider.method_23455(2): not translated, its class isn't translated (it extends Minecraft's `class_4651`).
    }

    /// `DynamicTreeFeature` (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java)
    pub mod dynamic_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_3031`. Its static parts are below.
        // DISC_PROVIDER: not translated, it uses UniversalPoissonDiscProvider, which isn't translated (it uses Minecraft's `class_2960`).
        // RANDOM: not translated, it uses RandomXOR, which isn't translated (it extends Minecraft's `class_7384`).
        // concreteBlocks: not translated, it uses Minecraft's `class_2248`.
        // DynamicTreeFeature.setup(0): not translated, it uses DynamicTreeFeature.concreteBlocks, which isn't translated (it uses Minecraft's `class_2248`).
        // DynamicTreeFeature.DynamicTreeFeature(0): not translated, its class isn't translated (it extends Minecraft's `class_3031`).
        // DynamicTreeFeature.method_13151(1): not translated, its class isn't translated (it extends Minecraft's `class_3031`).
        // DynamicTreeFeature.generateTrees(4): not translated, its class isn't translated (it extends Minecraft's `class_3031`).
        // DynamicTreeFeature.getNoiseBiome(2): not translated, it uses Minecraft's `class_6880`.
        // DynamicTreeFeature.validTreePos(2): not translated, it uses Minecraft's `class_3746`.
        // DynamicTreeFeature.isFoliage(2): not translated, it uses Minecraft's `class_3746`.
        // DynamicTreeFeature.generateTree(5): not translated, its class isn't translated (it extends Minecraft's `class_3031`).
        // DynamicTreeFeature.genTree(5): not translated, its class isn't translated (it extends Minecraft's `class_3031`).
        // DynamicTreeFeature.getSpeciesSelector(1): not translated, its class isn't translated (it extends Minecraft's `class_3031`).
        // DynamicTreeFeature.getChanceSelector(1): not translated, its class isn't translated (it extends Minecraft's `class_3031`).
        // DynamicTreeFeature.generateConcreteCircle(4): not translated, its class isn't translated (it extends Minecraft's `class_3031`).
    }

    /// `DynamicTreeFeature.GeneratorResult` (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java)
    pub mod dynamic_tree_feature_generator_result {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // DynamicTreeFeature.GeneratorResult.GeneratorResult(1): not translated, its class isn't translated (its constants carry values).
        // DynamicTreeFeature.GeneratorResult.getColor(0): not translated, its class isn't translated (its constants carry values).
        // DynamicTreeFeature.GeneratorResult.getColoredBlock(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `DelayedHolderSet` (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java)
    pub mod delayed_holder_set {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // DelayedHolderSet.DelayedHolderSet(1): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.method_40239(0): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.method_40247(0): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.method_40248(0): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.method_40243(1): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.method_40240(1): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.method_40241(1): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.method_46768(1): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.method_45925(0): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.iterator(0): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.forEach(1): not translated, its class isn't translated (it's generic).
        // DelayedHolderSet.spliterator(0): not translated, its class isn't translated (it's generic).
    }

    /// `DTBiomeHolderSet` (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java)
    pub mod dt_biome_holder_set {
        use super::*;
        // The class itself isn't translated: it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen). Its static parts are below.
        // DTBiomeHolderSet.getSet(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.containsKey(1): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.method_40239(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.method_40247(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.method_40248(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.method_40243(1): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.method_40240(1): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.method_40241(1): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.method_46768(1): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.method_45925(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.getIncludeComponents(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.getExcludeComponents(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.addHolderSet(2): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.addDelayedHolderSet(2): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.addNameRegexMatch(3): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.addTagsRegexMatch(3): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.addOr(2): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.clear(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
        // DTBiomeHolderSet.iterator(0): not translated, its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)).
    }

    /// `NameRegexMatchHolderSet` (com/dtteam/dynamictrees/worldgen/holderset/NameRegexMatchHolderSet.java)
    pub mod name_regex_match_holder_set {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NameRegexMatchHolderSet.NameRegexMatchHolderSet(2): not translated, its class isn't translated (it's generic).
        // NameRegexMatchHolderSet.getInput(1): not translated, its class isn't translated (it's generic).
    }

    /// `OrHolderSet` (com/dtteam/dynamictrees/worldgen/holderset/OrHolderSet.java)
    pub mod or_holder_set {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // OrHolderSet.OrHolderSet(1): not translated, its class isn't translated (it's generic).
        // OrHolderSet.method_40239(0): not translated, its class isn't translated (it's generic).
        // OrHolderSet.method_46768(1): not translated, its class isn't translated (it's generic).
    }

    /// `RegexMatchHolderSet` (com/dtteam/dynamictrees/worldgen/holderset/RegexMatchHolderSet.java)
    pub mod regex_match_holder_set {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegexMatchHolderSet.RegexMatchHolderSet(2): not translated, its class isn't translated (it's generic).
        // RegexMatchHolderSet.registryLookup(0): not translated, its class isn't translated (it's generic).
        // RegexMatchHolderSet.regex(0): not translated, its class isn't translated (it's generic).
        // RegexMatchHolderSet.getPattern(0): not translated, its class isn't translated (it's generic).
        // RegexMatchHolderSet.method_40239(0): not translated, its class isn't translated (it's generic).
        // RegexMatchHolderSet.method_46768(1): not translated, its class isn't translated (it's generic).
        // RegexMatchHolderSet.getInput(1): not translated, it has no body.
    }

    /// `StreamBackedHolderSet` (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java)
    pub mod stream_backed_holder_set {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // StreamBackedHolderSet.contents(0): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.contentsSet(0): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.method_40247(0): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.spliterator(0): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.iterator(0): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.method_40243(1): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.method_40240(1): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.method_40248(0): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.method_40241(1): not translated, its class isn't translated (it's generic).
        // StreamBackedHolderSet.method_45925(0): not translated, its class isn't translated (it's generic).
    }

    /// `TagsRegexMatchHolderSet` (com/dtteam/dynamictrees/worldgen/holderset/TagsRegexMatchHolderSet.java)
    pub mod tags_regex_match_holder_set {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TagsRegexMatchHolderSet.TagsRegexMatchHolderSet(2): not translated, its class isn't translated (it's generic).
        // TagsRegexMatchHolderSet.getInput(1): not translated, its class isn't translated (it's generic).
    }

    /// `IDTBiomeHolderSet` (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java)
    pub mod idt_biome_holder_set {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDTBiomeHolderSet.containsKey(1): not translated, it has no body.
        // IDTBiomeHolderSet.method_40239(0): not translated, it has no body.
        // IDTBiomeHolderSet.getIncludeComponents(0): not translated, it has no body.
        // IDTBiomeHolderSet.getExcludeComponents(0): not translated, it has no body.
        // IDTBiomeHolderSet.addHolderSet(2): not translated, it has no body.
        // IDTBiomeHolderSet.addDelayedHolderSet(2): not translated, it has no body.
        // IDTBiomeHolderSet.addNameRegexMatch(3): not translated, it has no body.
        // IDTBiomeHolderSet.addTagsRegexMatch(3): not translated, it has no body.
        // IDTBiomeHolderSet.addOr(2): not translated, it has no body.
        // IDTBiomeHolderSet.clear(0): not translated, it has no body.
    }

    /// `JoCode` (com/dtteam/dynamictrees/worldgen/JoCode.java)
    pub mod jo_code {
        use super::*;
        // LOGGER: the logger. Log calls become comments.
        pub const BASE_64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        pub const FORK_CODE: i8 = 6i8;
        pub const RETURN_CODE: i8 = 7i8;
        #[derive(Clone, Debug)]
        pub struct JoCode {
            pub instructions: Vec<i8>,
            pub careful: bool,
            pub dirmap: Vec<Vec<i8>>,
            pub facing_map: Vec<i8>,
            pub unfacing_map: Vec<i8>,
        }
        impl JoCode {
            // JoCode.JoCode(3): not translated, it uses Minecraft's `class_1937`.
            // JoCode.getCodeFromWorld(3): not translated, it uses Minecraft's `class_1937`.
            // JoCode.JoCode(2): not translated, it uses Minecraft's `class_1937`.
            // JoCode.JoCode(1): not translated, it calls JoCode.decode, which isn't translated.
            // JoCode.setCareful(1): not translated, `this` used as a value.
            pub fn get_code(&self, pos: i32) -> i32 {
                return ((self.unfacing_map[self.instructions[pos as usize] as usize]) as i32);
            }
            // JoCode.setFacing(1): not translated, it uses Minecraft's `class_2350`.
            // JoCode.rotate(1): not translated, it uses Minecraft's `class_2350`.
            // JoCode.generate(1): not translated, it uses DynamicTreeGenerationContext, which isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
            // JoCode.generateAndAgeLeaves(3): not translated, it uses DynamicTreeGenerationContext, which isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
            // JoCode.tryGenerateAgain(5): not translated, it uses DynamicTreeGenerationContext, which isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)).
            // JoCode.cleanupFrankentree(5): not translated, its body couldn't be read: an assert.
            // JoCode.generateFork(5): not translated, it uses Minecraft's `class_1936`.
            // JoCode.setBlockForGeneration(6): not translated, it uses Minecraft's `class_1936`.
            // JoCode.isFreeToSetBlock(3): not translated, it uses Minecraft's `class_1936`.
            // JoCode.isReplaceable(2): not translated, it uses Minecraft's `class_2680`.
            // JoCode.smother(2): not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
            // JoCode.isClearOfNearbyBranches(3): not translated, it uses Minecraft's `class_1936`.
            // JoCode.addSnow(4): not translated, it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`).
            // JoCode.toString(0): not translated, it calls JoCode.encode, which isn't translated.
            // JoCode.getTextComponent(0): not translated, it uses Minecraft's `class_2561`.
        }
        // JoCode.encode(1): not translated, it changes a for-each variable.
        // JoCode.decode(1): not translated, its constructors are overloaded with the same number of parameters.
    }

    /// `JoCode.CodeCompiler` (com/dtteam/dynamictrees/worldgen/JoCode.java)
    pub mod jo_code_code_compiler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CodeCompiler {
            pub instructions: Vec<i8>,
        }
        impl CodeCompiler {
            pub fn new() -> Self {
                let mut s = Self { instructions: Vec::new() };
                s.instructions = Vec::new();
                s
            }
            // JoCode.CodeCompiler.CodeCompiler(1): not translated, its constructors are overloaded with the same number of parameters.
            // JoCode.CodeCompiler.CodeCompiler(1): not translated, its constructors are overloaded with the same number of parameters.
            pub fn add_direction(&mut self, dir: i8) {
                if (dir as i32) >= 0 {
                    self.instructions.push((((dir as i32) & 7) as i8));
                }
            }
            pub fn add_instruction(&mut self, instruction: i8) {
                self.instructions.push(instruction);
            }
            pub fn add_return(&mut self) {
                self.instructions.push(7i8);
            }
            pub fn add_fork(&mut self) {
                self.instructions.push(6i8);
            }
            // JoCode.CodeCompiler.compile(0): not translated, `++` inside an expression.
        }
    }

    /// `JoCodeRegistry` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java)
    pub mod jo_code_registry {
        use super::*;
        // CODES: not translated, it uses Minecraft's `class_2960`.
        // ROOTS_CODES: not translated, it uses Minecraft's `class_2960`.
        #[derive(Clone, Debug)]
        pub struct JoCodeRegistry {
        }
        impl JoCodeRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // JoCodeRegistry.clear(0): not translated, it uses JoCodeRegistry.CODES, which isn't translated (it uses Minecraft's `class_2960`).
        // JoCodeRegistry.register(3): not translated, it uses Minecraft's `class_2960`.
        // JoCodeRegistry.registerRoot(3): not translated, it uses Minecraft's `class_2960`.
        // JoCodeRegistry.getCodes(2): not translated, it's overloaded with the same number of parameters.
        // JoCodeRegistry.getCodes(2): not translated, it's overloaded with the same number of parameters.
        // JoCodeRegistry.getCodes(3): not translated, it uses Minecraft's `class_2960`.
        // JoCodeRegistry.getRandomCode(3): not translated, it uses Minecraft's `class_2960`.
        // JoCodeRegistry.getRandomCode(4): not translated, it uses Minecraft's `class_2960`.
    }

    /// `OverworldGroundFinder` (com/dtteam/dynamictrees/worldgen/OverworldGroundFinder.java)
    pub mod overworld_ground_finder {
        use super::*;
        // The class itself isn't translated: it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen). Its static parts are below.
        // OverworldGroundFinder.findGround(3): not translated, its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)).
    }

    /// `RootsJoCode` (com/dtteam/dynamictrees/worldgen/RootsJoCode.java)
    pub mod roots_jo_code {
        use super::*;
        // The class itself isn't translated: it extends `JoCode`. Its static parts are below.
        // RootsJoCode.RootsJoCode(1): not translated, its class isn't translated (it extends `JoCode`).
        // RootsJoCode.RootsJoCode(3): not translated, its class isn't translated (it extends `JoCode`).
        // RootsJoCode.getCodeFromWorld(3): not translated, its class isn't translated (it extends `JoCode`).
        // RootsJoCode.generate(1): not translated, its class isn't translated (it extends `JoCode`).
        // RootsJoCode.setBlockForGeneration(6): not translated, its class isn't translated (it extends `JoCode`).
        // RootsJoCode.isFreeToSetBlock(3): not translated, its class isn't translated (it extends `JoCode`).
    }

    /// `DTCancelVanillaTreePoolElement` (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java)
    pub mod dt_cancel_vanilla_tree_pool_element {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_3784`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DTCancelVanillaTreePoolElement.DTCancelVanillaTreePoolElement(2): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // DTCancelVanillaTreePoolElement.create(2): not translated, it uses Minecraft's `class_3786`.
        // DTCancelVanillaTreePoolElement.isEnabled(0): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // DTCancelVanillaTreePoolElement.method_16601(2): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // DTCancelVanillaTreePoolElement.method_16627(4): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // DTCancelVanillaTreePoolElement.method_16628(3): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // DTCancelVanillaTreePoolElement.method_16626(11): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // DTCancelVanillaTreePoolElement.method_16624(0): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // DTCancelVanillaTreePoolElement.method_16622(1): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // DTCancelVanillaTreePoolElement.method_16757(0): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
    }

    /// `RegularTemplatePoolModifier` (com/dtteam/dynamictrees/worldgen/structure/RegularTemplatePoolModifier.java)
    pub mod regular_template_pool_modifier {
        use super::*;
        // The class itself isn't translated: it implements `TemplatePoolModifier`. Its static parts are below.
        // RegularTemplatePoolModifier.RegularTemplatePoolModifier(2): not translated, its class isn't translated (it implements `TemplatePoolModifier`).
        // RegularTemplatePoolModifier.replaceTemplate(2): not translated, its class isn't translated (it implements `TemplatePoolModifier`).
        // RegularTemplatePoolModifier.removeTemplate(1): not translated, its class isn't translated (it implements `TemplatePoolModifier`).
        // RegularTemplatePoolModifier.removeAllTemplates(0): not translated, its class isn't translated (it implements `TemplatePoolModifier`).
        // RegularTemplatePoolModifier.registerPool(1): not translated, its class isn't translated (it implements `TemplatePoolModifier`).
        // RegularTemplatePoolModifier.village(3): not translated, it uses TemplatePoolModifier, which isn't translated (it's an interface).
        // RegularTemplatePoolModifier.create(2): not translated, it uses TemplatePoolModifier, which isn't translated (it's an interface).
    }

    /// `TemplatePoolModifier` (com/dtteam/dynamictrees/worldgen/structure/TemplatePoolModifier.java)
    pub mod template_pool_modifier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TemplatePoolModifier.replaceTemplate(2): not translated, it has no body.
        // TemplatePoolModifier.removeTemplate(1): not translated, it has no body.
        // TemplatePoolModifier.removeAllTemplates(0): not translated, it has no body.
        // TemplatePoolModifier.registerPool(1): not translated, it has no body.
    }

    /// `TreePoolElement` (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java)
    pub mod tree_pool_element {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `class_3784`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TreePoolElement.TreePoolElement(2): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.TreePoolElement(3): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.create(1): not translated, it uses Minecraft's `class_3786`.
        // TreePoolElement.create(2): not translated, it uses Minecraft's `class_3786`.
        // TreePoolElement.fillDefaultJigsawNBT(0): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.method_16627(4): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.method_16628(3): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.method_16601(2): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.method_16626(11): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.getOffsetX(1): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.getOffsetY(1): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.getOffsetZ(1): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.method_16757(0): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.getSpecies(0): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
        // TreePoolElement.getOffset(0): not translated, its class isn't translated (it extends Minecraft's `class_3784`).
    }

    /// `VillageTreeReplacement` (com/dtteam/dynamictrees/worldgen/structure/VillageTreeReplacement.java)
    pub mod village_tree_replacement {
        use super::*;
        // LOGGER: the logger. Log calls become comments.
        // REPLACEMENT_TOWN_CENTER_ID: not translated, it calls DynamicTrees.location, which isn't translated.
        #[derive(Clone, Debug)]
        pub struct VillageTreeReplacement {
        }
        impl VillageTreeReplacement {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VillageTreeReplacement.replaceTreesFromVanillaVillages(2): not translated, it uses Minecraft's `class_7874`.
    }

    /// `SubterraneanGroundFinder` (com/dtteam/dynamictrees/worldgen/SubterraneanGroundFinder.java)
    pub mod subterranean_ground_finder {
        use super::*;
        // The class itself isn't translated: it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen). Its static parts are below.
        // NO_LAYERS: not translated, it uses Minecraft's `class_2338`.
        // SubterraneanGroundFinder.isReplaceable(2): not translated, its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)).
        // SubterraneanGroundFinder.getTopY(2): not translated, its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)).
        // SubterraneanGroundFinder.findSubterraneanLayerHeights(2): not translated, its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)).
        // SubterraneanGroundFinder.findGround(3): not translated, its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)).
    }

    /// `SurfaceGroundFinder` (com/dtteam/dynamictrees/worldgen/SurfaceGroundFinder.java)
    pub mod surface_ground_finder {
        use super::*;
        // The class itself isn't translated: it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen). Its static parts are below.
        // SurfaceGroundFinder.findGround(3): not translated, its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)).
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
        assert_eq!(ID, "dynamictrees");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
