twostate predicate pre_original(degrees: real, radians: real)
    {
    true
    }

    twostate predicate pre_gen(degrees: real, radians: real)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(degrees: real, radians: real)
    ensures pre_original(degrees, radians) <==> pre_gen(degrees, radians)
    {
    }

    twostate predicate post_original(degrees: real, radians: real)
    requires pre_original(degrees, radians)
    {
    true
    }

    twostate predicate post_gen(degrees: real, radians: real)
    requires pre_original(degrees, radians)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(degrees: real, radians: real)
    requires pre_original(degrees, radians)
    requires pre_gen(degrees, radians)
    ensures post_original(degrees, radians) <==> post_gen(degrees, radians)
    {
    }