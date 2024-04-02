twostate predicate pre_original(a: array<int>, result: bool)
    reads a
    {
    true
    }

    twostate predicate pre_gen(a: array<int>, result: bool)
    reads a

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: array<int>, result: bool)
    ensures pre_original(a, result) <==> pre_gen(a, result)
    {
    }

    twostate predicate post_original(a: array<int>, result: bool)
    reads a
    requires pre_original(a, result)
    {
    true
    }

    twostate predicate post_gen(a: array<int>, result: bool)
    reads a
    requires pre_original(a, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: array<int>, result: bool)
    requires pre_original(a, result)
    requires pre_gen(a, result)
    ensures post_original(a, result) <==> post_gen(a, result)
    {
    }