use crate::ty::quad::Quad;

// Builder
#[derive(Default)]
pub struct VertexBuilder<V: Clone> {
    pub data: Vec<V>,
    pub indices: Vec<u32>,
}

impl<V: Clone> VertexBuilder<V> {
    pub fn new() -> VertexBuilder<V> {
        VertexBuilder {
            data: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn point(&mut self, value: V) {
        self.indices.push(self.data.len() as u32);
        self.data.push(value);
    }

    pub fn quad(&mut self, value: impl Quad<Item = V>) {
        let len = self.data.len() as u32;
        self.indices
            .extend_from_slice(&[0 + len, 1 + len, 3 + len, 1 + len, 2 + len, 3 + len]);
        self.data.extend_from_slice(&value.quad());
    }
}
