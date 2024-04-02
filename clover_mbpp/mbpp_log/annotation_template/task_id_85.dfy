twostate predicate pre_original(radius: real, area: real)
    {
    true
    }

    twostate predicate pre_gen(radius: real, area: real)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(radius: real, area: real)
    ensures pre_original(radius, area) <==> pre_gen(radius, area)
    {
    }

    twostate predicate post_original(radius: real, area: real)
    requires pre_original(radius, area)
    {
    true
    }

    twostate predicate post_gen(radius: real, area: real)
    requires pre_original(radius, area)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(radius: real, area: real)
    requires pre_original(radius, area)
    requires pre_gen(radius, area)
    ensures post_original(radius, area) <==> post_gen(radius, area)
    {
    }