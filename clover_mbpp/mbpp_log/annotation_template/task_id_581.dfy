twostate predicate pre_original(baseEdge: int, height: int, area: int)
    {
    true
    }

    twostate predicate pre_gen(baseEdge: int, height: int, area: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(baseEdge: int, height: int, area: int)
    ensures pre_original(baseEdge, height, area) <==> pre_gen(baseEdge, height, area)
    {
    }

    twostate predicate post_original(baseEdge: int, height: int, area: int)
    requires pre_original(baseEdge, height, area)
    {
    true
    }

    twostate predicate post_gen(baseEdge: int, height: int, area: int)
    requires pre_original(baseEdge, height, area)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(baseEdge: int, height: int, area: int)
    requires pre_original(baseEdge, height, area)
    requires pre_gen(baseEdge, height, area)
    ensures post_original(baseEdge, height, area) <==> post_gen(baseEdge, height, area)
    {
    }