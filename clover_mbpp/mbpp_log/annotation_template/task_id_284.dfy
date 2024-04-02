twostate predicate pre_original(a: array<int>, n: int, result: bool)
    reads a
    {
    true
    }

    twostate predicate pre_gen(a: array<int>, n: int, result: bool)
    reads a

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: array<int>, n: int, result: bool)
    ensures pre_original(a, n, result) <==> pre_gen(a, n, result)
    {
    }

    twostate predicate post_original(a: array<int>, n: int, result: bool)
    reads a
    requires pre_original(a, n, result)
    {
    true
    }

    twostate predicate post_gen(a: array<int>, n: int, result: bool)
    reads a
    requires pre_original(a, n, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: array<int>, n: int, result: bool)
    requires pre_original(a, n, result)
    requires pre_gen(a, n, result)
    ensures post_original(a, n, result) <==> post_gen(a, n, result)
    {
    }