/*
    appellation: impl_serde <module>
    authors: @FL03
*/
use crate::hash_graph::HashGraph;
use crate::{GraphKind, HyperGraphAttributes, RawIndex};
use core::hash::Hash;
use serde::de::{Deserialize, DeserializeOwned, MapAccess, Visitor};
use serde::ser::Serialize;

const FIELDS: &'static [&'static str] = &["nodes", "surfaces", "position", "_attrs"];

impl<'a, N, E, A, K, Idx> Deserialize<'a> for HashGraph<N, E, A>
where
    A: HyperGraphAttributes<Kind = K, Idx = Idx> + DeserializeOwned,
    N: DeserializeOwned + Eq + Hash,
    E: DeserializeOwned + Eq + Hash,
    Idx: Eq + Hash + RawIndex + DeserializeOwned,
    K: GraphKind + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        deserializer.deserialize_struct(
            "HashGraph",
            FIELDS,
            HashGraphVisitor {
                _marker: core::marker::PhantomData,
            },
        )
    }
}

impl<N, E, A, K, Idx> Serialize for HashGraph<N, E, A>
where
    A: HyperGraphAttributes<Kind = K, Idx = Idx> + Serialize,
    N: Serialize + Eq + Hash,
    E: Serialize + Eq + Hash,
    Idx: Eq + Hash + RawIndex + Serialize,
    K: GraphKind + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("HashGraph", 4)?;
        state.serialize_field("nodes", &self.nodes)?;
        state.serialize_field("surfaces", &self.surfaces)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("_attrs", &self._attrs)?;
        state.end()
    }
}

struct HashGraphVisitor<N, E, A> {
    _marker: core::marker::PhantomData<(N, E, A)>,
}

impl<'de, N, E, A, K, Idx> Visitor<'de> for HashGraphVisitor<N, E, A>
where
    A: HyperGraphAttributes<Kind = K, Idx = Idx> + DeserializeOwned,
    N: DeserializeOwned + Eq + Hash,
    E: DeserializeOwned + Eq + Hash,
    Idx: Eq + Hash + RawIndex + DeserializeOwned,
    K: GraphKind + DeserializeOwned,
{
    type Value = HashGraph<N, E, A>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a HashGraph")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut nodes = None;
        let mut surfaces = None;
        let mut position = None;
        let mut attrs = None;

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "nodes" => {
                    if nodes.is_some() {
                        return Err(serde::de::Error::duplicate_field("nodes"));
                    }
                    nodes = Some(map.next_value()?);
                }
                "surfaces" => {
                    if surfaces.is_some() {
                        return Err(serde::de::Error::duplicate_field("surfaces"));
                    }
                    surfaces = Some(map.next_value()?);
                }
                "position" => {
                    if position.is_some() {
                        return Err(serde::de::Error::duplicate_field("position"));
                    }
                    position = Some(map.next_value()?);
                }
                "_attrs" => {
                    if attrs.is_some() {
                        return Err(serde::de::Error::duplicate_field("_attrs"));
                    }
                    attrs = Some(map.next_value()?);
                }
                _ => return Err(serde::de::Error::unknown_field(key, FIELDS)),
            }
        }

        let nodes = nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?;
        let surfaces = surfaces.ok_or_else(|| serde::de::Error::missing_field("surfaces"))?;
        let position = position.ok_or_else(|| serde::de::Error::missing_field("position"))?;
        let attrs = attrs.ok_or_else(|| serde::de::Error::missing_field("_attrs"))?;
        Ok(HashGraph {
            nodes,
            surfaces,
            position,
            _attrs: attrs,
        })
    }
}
