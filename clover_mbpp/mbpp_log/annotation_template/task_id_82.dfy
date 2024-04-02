twostate predicate pre_original(radius: real, volume: real)
    {
    true
    }

    twostate predicate pre_gen(radius: real, volume: real)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(radius: real, volume: real)
    ensures pre_original(radius, volume) <==> pre_gen(radius, volume)
    {
    }

    twostate predicate post_original(radius: real, volume: real)
    requires pre_original(radius, volume)
    {
    true
    }

    twostate predicate post_gen(radius: real, volume: real)
    requires pre_original(radius, volume)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(radius: real, volume: real)
    requires pre_original(radius, volume)
    requires pre_gen(radius, volume)
    ensures post_original(radius, volume) <==> post_gen(radius, volume)
    {
    }