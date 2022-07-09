use crate::Id;
pub type RoomEdges<T> = [Option<T>; 12];

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct RoomSize(u8);

impl RoomSize {
    pub fn new(size: u8) -> Self {
        if size > 4 {
            panic!("attempted to create room with invalid size {}", size)
        }

        RoomSize(size)
    }
}

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
            size: RoomSize::new(size),
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

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn size(&self) -> RoomSize {
        self.size
    }
}

fn make_id(id: Option<u64>) -> Option<Id> {
    Some(Id(id?))
}
