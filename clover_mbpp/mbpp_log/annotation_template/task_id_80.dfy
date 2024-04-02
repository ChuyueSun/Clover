twostate predicate pre_original(n: int, t: int)
    {
    true
    }

    twostate predicate pre_gen(n: int, t: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, t: int)
    ensures pre_original(n, t) <==> pre_gen(n, t)
    {
    }

    twostate predicate post_original(n: int, t: int)
    requires pre_original(n, t)
    {
    true
    }

    twostate predicate post_gen(n: int, t: int)
    requires pre_original(n, t)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, t: int)
    requires pre_original(n, t)
    requires pre_gen(n, t)
    ensures post_original(n, t) <==> post_gen(n, t)
    {
    }