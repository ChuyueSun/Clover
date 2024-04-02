twostate predicate pre_original(a: int, b: int, result: int)
    {
    true
    }

    twostate predicate pre_gen(a: int, b: int, result: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: int, b: int, result: int)
    ensures pre_original(a, b, result) <==> pre_gen(a, b, result)
    {
    }

    twostate predicate post_original(a: int, b: int, result: int)
    requires pre_original(a, b, result)
    {
    true
    }

    twostate predicate post_gen(a: int, b: int, result: int)
    requires pre_original(a, b, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: int, b: int, result: int)
    requires pre_original(a, b, result)
    requires pre_gen(a, b, result)
    ensures post_original(a, b, result) <==> post_gen(a, b, result)
    {
    }