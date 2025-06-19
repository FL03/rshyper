/*
    appellation: impl_link_ext <module>
    authors: @FL03
*/
use crate::idx::{EdgeId, RawIndex};
use crate::rel::{Layout, Link, RawLayout};
use crate::{Domain, GraphType};

impl<S, K, Idx> RawLayout for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    type Kind = K;
    type Index = Idx;
    type Store = S;

    seal!();

    fn index(&self) -> &EdgeId<Idx> {
        self.id()
    }

    fn domain(&self) -> &S {
        self.domain()
    }

    fn domain_mut(&mut self) -> &mut S {
        self.domain_mut()
    }
}

impl<S, K, Idx> Layout for Link<S, K, Idx>
where
    S: Domain<Idx>,
    Idx: RawIndex,
    K: GraphType,
{
    fn new(id: EdgeId<Idx>, vertices: S) -> Self {
        Self::new(id, vertices)
    }
}
