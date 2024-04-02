twostate predicate pre_original(n: int, decagonal: int)
    {
    true
    }

    twostate predicate pre_gen(n: int, decagonal: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, decagonal: int)
    ensures pre_original(n, decagonal) <==> pre_gen(n, decagonal)
    {
    }

    twostate predicate post_original(n: int, decagonal: int)
    requires pre_original(n, decagonal)
    {
    true
    }

    twostate predicate post_gen(n: int, decagonal: int)
    requires pre_original(n, decagonal)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, decagonal: int)
    requires pre_original(n, decagonal)
    requires pre_gen(n, decagonal)
    ensures post_original(n, decagonal) <==> post_gen(n, decagonal)
    {
    }