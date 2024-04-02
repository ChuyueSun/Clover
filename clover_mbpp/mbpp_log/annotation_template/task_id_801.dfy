twostate predicate pre_original(a: int, b: int, c: int, count: int)
    {
    true
    }

    twostate predicate pre_gen(a: int, b: int, c: int, count: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: int, b: int, c: int, count: int)
    ensures pre_original(a, b, c, count) <==> pre_gen(a, b, c, count)
    {
    }

    twostate predicate post_original(a: int, b: int, c: int, count: int)
    requires pre_original(a, b, c, count)
    {
    true
    }

    twostate predicate post_gen(a: int, b: int, c: int, count: int)
    requires pre_original(a, b, c, count)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: int, b: int, c: int, count: int)
    requires pre_original(a, b, c, count)
    requires pre_gen(a, b, c, count)
    ensures post_original(a, b, c, count) <==> post_gen(a, b, c, count)
    {
    }