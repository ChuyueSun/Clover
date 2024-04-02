twostate predicate pre_original(radius: real, circumference: real)
    {
    true
    }

    twostate predicate pre_gen(radius: real, circumference: real)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(radius: real, circumference: real)
    ensures pre_original(radius, circumference) <==> pre_gen(radius, circumference)
    {
    }

    twostate predicate post_original(radius: real, circumference: real)
    requires pre_original(radius, circumference)
    {
    true
    }

    twostate predicate post_gen(radius: real, circumference: real)
    requires pre_original(radius, circumference)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(radius: real, circumference: real)
    requires pre_original(radius, circumference)
    requires pre_gen(radius, circumference)
    ensures post_original(radius, circumference) <==> post_gen(radius, circumference)
    {
    }