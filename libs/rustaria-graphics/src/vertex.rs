pub mod ty;
pub mod quad;


use crate::vertex::quad::Quad;


// Builder
pub struct VertexBuilder<V: Clone> {
    data: Vec<V>,
    indices: Vec<u32>,
}

impl<V: Clone> VertexBuilder<V> {
    pub fn point(&mut self, value: V) {
        self.indices.push(self.data.len() as u32);
        self.data.push(value);
    }

    pub fn quad(&mut self, value: impl Quad<V>) {
        let len = self.data.len() as u32;
        self.indices
            .extend_from_slice(&[len, 1 + len, 3 + len, 1 + len, 2 + len, 3 + len]);
        self.data.extend_from_slice(&value.quad());
    }
}