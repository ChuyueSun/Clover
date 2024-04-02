twostate predicate pre_original(size: int, volume: int)
    {
    true
    }

    twostate predicate pre_gen(size: int, volume: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(size: int, volume: int)
    ensures pre_original(size, volume) <==> pre_gen(size, volume)
    {
    }

    twostate predicate post_original(size: int, volume: int)
    requires pre_original(size, volume)
    {
    true
    }

    twostate predicate post_gen(size: int, volume: int)
    requires pre_original(size, volume)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(size: int, volume: int)
    requires pre_original(size, volume)
    requires pre_gen(size, volume)
    ensures post_original(size, volume) <==> post_gen(size, volume)
    {
    }