#[derive(Default)]
pub struct VertexBuilder<V: Clone> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

impl<V: Clone> VertexBuilder<V> {
    pub fn new() -> VertexBuilder<V> {
        VertexBuilder {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn point(&mut self, value: V) {
        self.indices.push(self.vertices.len() as u32);
        self.vertices.push(value);
    }

    pub fn quad(&mut self, value: impl Quad<Item = V>) {
        let len = self.vertices.len() as u32;
        self.indices
            .extend_from_slice(&[len, 1 + len, 3 + len, 1 + len, 2 + len, 3 + len]);
        self.vertices.extend_from_slice(&value.quad());
    }
}

pub trait Quad {
    type Item: Copy;
    fn quad(&self) -> [Self::Item; 4];
}

macro_rules! im {
    ($($V:ident: $N:tt),*) => {
	    impl<$($V: Quad),*> Quad for ($($V),*) {
			type Item = ($($V::Item),*);

			fn quad(&self) -> [Self::Item; 4] {
				#[allow(non_snake_case)]
				let ($($V),*) = ($(self.$N.quad()),*);
				[
					($($V[0]),*),
					($($V[1]),*),
					($($V[2]),*),
					($($V[3]),*),
				]
			}
		}
    };
}

im!(V0: 0, V1: 1);
im!(V0: 0, V1: 1, V2: 2);
im!(V0: 0, V1: 1, V2: 2, V3: 3);
im!(V0: 0, V1: 1, V2: 2, V3: 3, V4: 4);