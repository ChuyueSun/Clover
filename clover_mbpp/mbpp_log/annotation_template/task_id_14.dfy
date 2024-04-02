twostate predicate pre_original(base: int, height: int, length: int, volume: int)
    {
    true
    }

    twostate predicate pre_gen(base: int, height: int, length: int, volume: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(base: int, height: int, length: int, volume: int)
    ensures pre_original(base, height, length, volume) <==> pre_gen(base, height, length, volume)
    {
    }

    twostate predicate post_original(base: int, height: int, length: int, volume: int)
    requires pre_original(base, height, length, volume)
    {
    true
    }

    twostate predicate post_gen(base: int, height: int, length: int, volume: int)
    requires pre_original(base, height, length, volume)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(base: int, height: int, length: int, volume: int)
    requires pre_original(base, height, length, volume)
    requires pre_gen(base, height, length, volume)
    ensures post_original(base, height, length, volume) <==> post_gen(base, height, length, volume)
    {
    }