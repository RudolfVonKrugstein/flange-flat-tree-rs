use crate::FlatTree;
use crate::values::VecValues;

pub struct Builder<A> {
    nav_builder: super::navigator::Builder,
    values: Vec<A>,
}

impl<A> Builder<A> {
    pub fn new() -> Builder<A> {
        Builder {
            nav_builder: super::navigator::Builder::new(),
            values: Vec::new(),
        }
    }

    pub fn with_capacity(c: usize) -> Builder<A> {
        Builder {
            nav_builder: super::navigator::Builder::with_capacity(c),
            values: Vec::with_capacity(c),
        }
    }

    pub fn get(&self, index: usize) -> Option<&A> {
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut A> {
        self.values.get_mut(index)
    }

    pub fn start_element(&mut self, el: A) -> usize {
        self.values.push(el);
        self.nav_builder.start_element()
    }

    pub fn end_element(&mut self) -> usize {
        self.nav_builder.end_element()
    }

    pub fn start_end_element(&mut self, el: A) -> usize {
        self.values.push(el);
        self.nav_builder.start_end_element()
    }
}

impl<A> Builder<A> {
    pub fn build(self: Self) -> FlatTree<VecValues<A>>
    {

        FlatTree {
            nav: self.nav_builder.build(),
            values: VecValues::from_vec(self.values),
        }
    }
}
