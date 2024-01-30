# peano-cell

`PeanoCell` is another [`GhostCell`](https://github.com/matthieu-m/ghost-cell) alternative.
It is inspired by [`TCell`](https://docs.rs/qcell/0.5.2/qcell/struct.TCell.html) and [`frankencell`](https://github.com/spencerwhite/frankencell).

Like TCell, PeanoCell also uses type marker to identify token <-> cell connection.
However, new owners (or tokens) can be easily created via `owner_builder.new_owner()`, there is no need to declare new type markers.

The main advantage over frankencell is that PeanoCell does not use unstable [generic const exprs](https://doc.rust-lang.org/beta/unstable-book/language-features/generic-const-exprs.html).
This was possible by replacing const generics with structs `Zero`, `Suc<Zero>`, and `Suc<Suc<...>>`
which correspond to the concept of [peano numbers](https://en.wikipedia.org/wiki/Peano_axioms)

Creating new owner builder is the only place which is not statically checked.
Everything else including
- owner creating,
- Aliasing Xor Mutability check when accessing a cell with one owner,
- owner <-> cell correspondence,

are checked statically.

To my knowledge, this is on of the closest cell we currently have to GhostCell, which does everything at compile-time.
The difference between PeanoCell and GhostCell is that PeanoCell doesn't require tokens to live inside closure.
You can pass tokens and even token builder wherever you want, see [owner_builder_passing example](https://github.com/sancho20021/peano-cell/blob/main/examples/owner_builder_passing.rs).

Currently, PeanoCell relies on qcell's `TCell`. This results in additional dynamic check when creating new owners.
Potentially, it can be implemented using `UnsafeCell` instead which will make everything except owner builder creation zero-cost.