use qcell::{TCell, TCellOwner};
use std::{marker::PhantomData, sync::Once};

pub struct Zero;
pub struct Suc<N>(PhantomData<N>);

pub struct PeanoCell<T, Owner>(TCell<Owner, T>);

impl<T, Owner> PeanoCell<T, Owner> {
    pub const fn new(value: T) -> Self {
        PeanoCell(TCell::new(value))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}

impl<T, Owner> PeanoCell<T, Owner> {
    pub fn borrow<'a>(&'a self, owner: &'a PeanoOwner<Owner>) -> &'a T {
        self.0.ro(&owner.0)
    }

    pub fn borrow_mut<'a>(&'a self, owner: &'a mut PeanoOwner<Owner>) -> &'a mut T {
        self.0.rw(&mut owner.0)
    }
}

pub struct PeanoOwner<Owner: 'static>(TCellOwner<Owner>);

impl<Owner: 'static> PeanoOwner<Owner> {
    pub fn cell<T>(&self, value: T) -> PeanoCell<T, Owner> {
        PeanoCell::new(value)
    }
}

pub struct PeanoOwnerBuilder<N> {
    phantom_data: PhantomData<N>,
}

impl<N> PeanoOwnerBuilder<N> {
    pub fn new_owner(self) -> (PeanoOwner<N>, PeanoOwnerBuilder<Suc<N>>) {
        (
            PeanoOwner(TCellOwner::new()),
            PeanoOwnerBuilder {
                phantom_data: PhantomData,
            },
        )
    }
}

static INIT: Once = Once::new();

pub fn new_owner_builder() -> Option<PeanoOwnerBuilder<Zero>> {
    let mut builder = None;
    INIT.call_once(|| {
        builder = Some(PeanoOwnerBuilder {
            phantom_data: PhantomData,
        })
    });
    builder
}
