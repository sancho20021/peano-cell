use peano_cell::{new_owner_builder, PeanoCell, PeanoOwner};

struct S<T, Owner: 'static> {
    data: PeanoCell<T, Owner>,
    owner: PeanoOwner<Owner>,
}

impl<T, Owner: 'static> S<T, Owner> {
    fn new(data: T, owner: PeanoOwner<Owner>) -> Self {
        Self {
            data: PeanoCell::new(data),
            owner,
        }
    }

    fn get(&self) -> &T {
        self.data.borrow(&self.owner)
    }

    fn get_mut(&mut self) -> &mut T {
        self.data.borrow_mut(&mut self.owner)
    }
}

fn main() {
    // the only way to create new owners is via owner builder
    let owner_builder = new_owner_builder().unwrap();

    // dynamic check that builder prevents creation of two builders
    assert!(new_owner_builder().is_none());
    let (owner1, owner_builder) = owner_builder.new_owner();
    let (owner2, owner_builder) = owner_builder.new_owner();

    // can't create another owner of same type, does not compile:
    // let owner1_dup: PeanoOwner<Zero> = PeanoOwner(TCellOwner::new());
    let cell1a = owner1.cell(0);
    let cell1b = owner1.cell(1);
    println!(
        "owned by owner1: {}, {}",
        cell1a.borrow(&owner1),
        cell1b.borrow(&owner1)
    );

    let cell2a = owner2.cell(0);
    println!("owned by owner2: {}", cell2a.borrow(&owner2));

    // can't use owner1 to access cell2, does not compile:
    // println!("{}", cell2a.borrow(&owner1));
    // println!("{}", cell1a.borrow(&owner2));

    let (owner3, _owner_builder) = owner_builder.new_owner();
    let mut data_wrapper = S::new("hello", owner3);
    println!("{}", data_wrapper.get());
    *data_wrapper.get_mut() = "goodbye";
    println!("{}", data_wrapper.get());
}
