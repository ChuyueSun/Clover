twostate predicate pre_original(a: int, b: int, c: int, median: int)
    {
    true
    }

    twostate predicate pre_gen(a: int, b: int, c: int, median: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: int, b: int, c: int, median: int)
    ensures pre_original(a, b, c, median) <==> pre_gen(a, b, c, median)
    {
    }

    twostate predicate post_original(a: int, b: int, c: int, median: int)
    requires pre_original(a, b, c, median)
    {
    true
    }

    twostate predicate post_gen(a: int, b: int, c: int, median: int)
    requires pre_original(a, b, c, median)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: int, b: int, c: int, median: int)
    requires pre_original(a, b, c, median)
    requires pre_gen(a, b, c, median)
    ensures post_original(a, b, c, median) <==> post_gen(a, b, c, median)
    {
    }