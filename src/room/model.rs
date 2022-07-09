use crate::Id;
pub type RoomEdges<T> = [Option<T>; 12];

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct RoomSize(u8);

#[derive(Debug)]
pub struct Room {
    id: Id,
    title: String,
    description: String,
    size: RoomSize,
    edges: RoomEdges<Id>,
}

impl Room {
    pub fn new(
        id: u64,
        title: &str,
        description: &str,
        size: u8,
        edges: [Option<u64>; 12],
    ) -> Self {
        Room {
            id: Id(id),
            title: title.to_owned(),
            description: description.to_owned(),
            size: RoomSize(size),
            edges: [
                make_id(edges[0]),
                make_id(edges[1]),
                make_id(edges[2]),
                make_id(edges[3]),
                make_id(edges[4]),
                make_id(edges[5]),
                make_id(edges[6]),
                make_id(edges[7]),
                make_id(edges[8]),
                make_id(edges[9]),
                make_id(edges[10]),
                make_id(edges[11]),
            ],
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }
}

fn make_id(id: Option<u64>) -> Option<Id> {
    Some(Id(id?))
}
