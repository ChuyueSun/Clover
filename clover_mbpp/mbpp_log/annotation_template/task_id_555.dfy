twostate predicate pre_original(n: int, diff: int)
    {
    true
    }

    twostate predicate pre_gen(n: int, diff: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, diff: int)
    ensures pre_original(n, diff) <==> pre_gen(n, diff)
    {
    }

    twostate predicate post_original(n: int, diff: int)
    requires pre_original(n, diff)
    {
    true
    }

    twostate predicate post_gen(n: int, diff: int)
    requires pre_original(n, diff)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, diff: int)
    requires pre_original(n, diff)
    requires pre_gen(n, diff)
    ensures post_original(n, diff) <==> post_gen(n, diff)
    {
    }