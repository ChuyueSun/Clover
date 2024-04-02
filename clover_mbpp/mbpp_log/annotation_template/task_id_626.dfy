twostate predicate pre_original(radius: int, area: int)
    {
    true
    }

    twostate predicate pre_gen(radius: int, area: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(radius: int, area: int)
    ensures pre_original(radius, area) <==> pre_gen(radius, area)
    {
    }

    twostate predicate post_original(radius: int, area: int)
    requires pre_original(radius, area)
    {
    true
    }

    twostate predicate post_gen(radius: int, area: int)
    requires pre_original(radius, area)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(radius: int, area: int)
    requires pre_original(radius, area)
    requires pre_gen(radius, area)
    ensures post_original(radius, area) <==> post_gen(radius, area)
    {
    }