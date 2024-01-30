use peano_cell::{new_owner_builder, PeanoOwner, PeanoOwnerBuilder, Suc};

fn create_two_owners<N>(
    owner_builder: PeanoOwnerBuilder<N>,
) -> (
    PeanoOwner<N>,
    PeanoOwner<Suc<N>>,
    PeanoOwnerBuilder<Suc<Suc<N>>>,
) {
    let (owner1, builder) = owner_builder.new_owner();
    let (owner2, builder) = builder.new_owner();
    (owner1, owner2, builder)
}

fn main() {
    let builder = new_owner_builder().unwrap();
    let (_owner1, _owner2, builder) = create_two_owners(builder);
    let (_owner3, _owner4, _builder) = create_two_owners(builder);
}
