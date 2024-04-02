twostate predicate pre_original(a: array<int>, b: array<int>, median: int)
    reads a, b
    {
    true
    }

    twostate predicate pre_gen(a: array<int>, b: array<int>, median: int)
    reads a, b

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: array<int>, b: array<int>, median: int)
    ensures pre_original(a, b, median) <==> pre_gen(a, b, median)
    {
    }

    twostate predicate post_original(a: array<int>, b: array<int>, median: int)
    reads a, b
    requires pre_original(a, b, median)
    {
    true
    }

    twostate predicate post_gen(a: array<int>, b: array<int>, median: int)
    reads a, b
    requires pre_original(a, b, median)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: array<int>, b: array<int>, median: int)
    requires pre_original(a, b, median)
    requires pre_gen(a, b, median)
    ensures post_original(a, b, median) <==> post_gen(a, b, median)
    {
    }