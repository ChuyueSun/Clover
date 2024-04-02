twostate predicate pre_original(radius: real, height: real, volume: real)
    {
    true
    }

    twostate predicate pre_gen(radius: real, height: real, volume: real)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(radius: real, height: real, volume: real)
    ensures pre_original(radius, height, volume) <==> pre_gen(radius, height, volume)
    {
    }

    twostate predicate post_original(radius: real, height: real, volume: real)
    requires pre_original(radius, height, volume)
    {
    true
    }

    twostate predicate post_gen(radius: real, height: real, volume: real)
    requires pre_original(radius, height, volume)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(radius: real, height: real, volume: real)
    requires pre_original(radius, height, volume)
    requires pre_gen(radius, height, volume)
    ensures post_original(radius, height, volume) <==> post_gen(radius, height, volume)
    {
    }