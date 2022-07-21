use stateright::actor::{Actor, Id};

use super::Request;

/// A client strategy that just deletes a single key in a map.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MapSingleDeleter {
    pub key: String,
    pub request_count: usize,
}

impl Actor for MapSingleDeleter {
    type Msg = Request;

    type State = ();

    fn on_start(
        &self,
        _id: stateright::actor::Id,
        o: &mut stateright::actor::Out<Self>,
    ) -> Self::State {
        for _ in 0..self.request_count {
            let msg = Request::DeleteMap(self.key.clone());
            o.send(Id::from(0), msg);
        }
    }
}

/// A client strategy that just deletes the first element in a list.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ListDeleter {
    pub index: usize,
    pub request_count: usize,
}

impl Actor for ListDeleter {
    type Msg = Request;

    type State = ();

    fn on_start(
        &self,
        _id: stateright::actor::Id,
        o: &mut stateright::actor::Out<Self>,
    ) -> Self::State {
        for _ in 0..self.request_count {
            let msg = Request::DeleteList(self.index);
            o.send(Id::from(0), msg);
        }
    }
}
