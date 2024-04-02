twostate predicate pre_original(radius: real, height: real, area: real)
    {
    true
    }

    twostate predicate pre_gen(radius: real, height: real, area: real)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(radius: real, height: real, area: real)
    ensures pre_original(radius, height, area) <==> pre_gen(radius, height, area)
    {
    }

    twostate predicate post_original(radius: real, height: real, area: real)
    requires pre_original(radius, height, area)
    {
    true
    }

    twostate predicate post_gen(radius: real, height: real, area: real)
    requires pre_original(radius, height, area)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(radius: real, height: real, area: real)
    requires pre_original(radius, height, area)
    requires pre_gen(radius, height, area)
    ensures post_original(radius, height, area) <==> post_gen(radius, height, area)
    {
    }