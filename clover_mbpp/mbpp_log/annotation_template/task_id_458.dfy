twostate predicate pre_original(length: int, width: int, area: int)
    {
    true
    }

    twostate predicate pre_gen(length: int, width: int, area: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(length: int, width: int, area: int)
    ensures pre_original(length, width, area) <==> pre_gen(length, width, area)
    {
    }

    twostate predicate post_original(length: int, width: int, area: int)
    requires pre_original(length, width, area)
    {
    true
    }

    twostate predicate post_gen(length: int, width: int, area: int)
    requires pre_original(length, width, area)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(length: int, width: int, area: int)
    requires pre_original(length, width, area)
    requires pre_gen(length, width, area)
    ensures post_original(length, width, area) <==> post_gen(length, width, area)
    {
    }