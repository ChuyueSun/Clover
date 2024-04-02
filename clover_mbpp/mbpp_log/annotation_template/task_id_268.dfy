twostate predicate pre_original(n: int, star: int)
    {
    true
    }

    twostate predicate pre_gen(n: int, star: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, star: int)
    ensures pre_original(n, star) <==> pre_gen(n, star)
    {
    }

    twostate predicate post_original(n: int, star: int)
    requires pre_original(n, star)
    {
    true
    }

    twostate predicate post_gen(n: int, star: int)
    requires pre_original(n, star)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, star: int)
    requires pre_original(n, star)
    requires pre_gen(n, star)
    ensures post_original(n, star) <==> post_gen(n, star)
    {
    }