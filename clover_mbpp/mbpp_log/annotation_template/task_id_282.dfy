twostate predicate pre_original(a: array<int>, b: array<int>, result: array<int>)
    reads a, b, result
    {
    true
    }

    twostate predicate pre_gen(a: array<int>, b: array<int>, result: array<int>)
    reads a, b, result

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: array<int>, b: array<int>, result: array<int>)
    ensures pre_original(a, b, result) <==> pre_gen(a, b, result)
    {
    }

    twostate predicate post_original(a: array<int>, b: array<int>, result: array<int>)
    reads a, b, result
    requires pre_original(a, b, result)
    {
    true
    }

    twostate predicate post_gen(a: array<int>, b: array<int>, result: array<int>)
    reads a, b, result
    requires pre_original(a, b, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: array<int>, b: array<int>, result: array<int>)
    requires pre_original(a, b, result)
    requires pre_gen(a, b, result)
    ensures post_original(a, b, result) <==> post_gen(a, b, result)
    {
    }