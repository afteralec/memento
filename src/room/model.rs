pub type RoomEdges<T> = [Option<T>; 12];

pub struct Room {
    id: i64,
    title: String,
    description: String,
    edges: RoomEdges<i64>,
}

impl Room {
    pub fn id(&self) -> i64 { self.id }
}
