twostate predicate pre_original(a: int, b: int, median: int)
    {
    true
    }

    twostate predicate pre_gen(a: int, b: int, median: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: int, b: int, median: int)
    ensures pre_original(a, b, median) <==> pre_gen(a, b, median)
    {
    }

    twostate predicate post_original(a: int, b: int, median: int)
    requires pre_original(a, b, median)
    {
    true
    }

    twostate predicate post_gen(a: int, b: int, median: int)
    requires pre_original(a, b, median)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: int, b: int, median: int)
    requires pre_original(a, b, median)
    requires pre_gen(a, b, median)
    ensures post_original(a, b, median) <==> post_gen(a, b, median)
    {
    }