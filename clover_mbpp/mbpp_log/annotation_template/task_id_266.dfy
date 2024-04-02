twostate predicate pre_original(size: int, area: int)
    {
    true
    }

    twostate predicate pre_gen(size: int, area: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(size: int, area: int)
    ensures pre_original(size, area) <==> pre_gen(size, area)
    {
    }

    twostate predicate post_original(size: int, area: int)
    requires pre_original(size, area)
    {
    true
    }

    twostate predicate post_gen(size: int, area: int)
    requires pre_original(size, area)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(size: int, area: int)
    requires pre_original(size, area)
    requires pre_gen(size, area)
    ensures post_original(size, area) <==> post_gen(size, area)
    {
    }