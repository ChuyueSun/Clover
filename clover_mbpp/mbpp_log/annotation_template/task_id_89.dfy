twostate predicate pre_original(n: int, m: int)
    {
    true
    }

    twostate predicate pre_gen(n: int, m: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, m: int)
    ensures pre_original(n, m) <==> pre_gen(n, m)
    {
    }

    twostate predicate post_original(n: int, m: int)
    requires pre_original(n, m)
    {
    true
    }

    twostate predicate post_gen(n: int, m: int)
    requires pre_original(n, m)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, m: int)
    requires pre_original(n, m)
    requires pre_gen(n, m)
    ensures post_original(n, m) <==> post_gen(n, m)
    {
    }