twostate predicate pre_original(a: array<int>, diff: int)
    reads a
    {
    true
    }

    twostate predicate pre_gen(a: array<int>, diff: int)
    reads a

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: array<int>, diff: int)
    ensures pre_original(a, diff) <==> pre_gen(a, diff)
    {
    }

    twostate predicate post_original(a: array<int>, diff: int)
    reads a
    requires pre_original(a, diff)
    {
    true
    }

    twostate predicate post_gen(a: array<int>, diff: int)
    reads a
    requires pre_original(a, diff)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: array<int>, diff: int)
    requires pre_original(a, diff)
    requires pre_gen(a, diff)
    ensures post_original(a, diff) <==> post_gen(a, diff)
    {
    }