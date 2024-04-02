twostate predicate pre_original(a: real, h: real, k: real, directrix: real)
    {
    true
    }

    twostate predicate pre_gen(a: real, h: real, k: real, directrix: real)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: real, h: real, k: real, directrix: real)
    ensures pre_original(a, h, k, directrix) <==> pre_gen(a, h, k, directrix)
    {
    }

    twostate predicate post_original(a: real, h: real, k: real, directrix: real)
    requires pre_original(a, h, k, directrix)
    {
    true
    }

    twostate predicate post_gen(a: real, h: real, k: real, directrix: real)
    requires pre_original(a, h, k, directrix)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: real, h: real, k: real, directrix: real)
    requires pre_original(a, h, k, directrix)
    requires pre_gen(a, h, k, directrix)
    ensures post_original(a, h, k, directrix) <==> post_gen(a, h, k, directrix)
    {
    }