twostate predicate pre_original(a: array<int>, squared: array<int>)
    reads a, squared
    {
    true
    }

    twostate predicate pre_gen(a: array<int>, squared: array<int>)
    reads a, squared

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: array<int>, squared: array<int>)
    ensures pre_original(a, squared) <==> pre_gen(a, squared)
    {
    }

    twostate predicate post_original(a: array<int>, squared: array<int>)
    reads a, squared
    requires pre_original(a, squared)
    {
    true
    }

    twostate predicate post_gen(a: array<int>, squared: array<int>)
    reads a, squared
    requires pre_original(a, squared)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: array<int>, squared: array<int>)
    requires pre_original(a, squared)
    requires pre_gen(a, squared)
    ensures post_original(a, squared) <==> post_gen(a, squared)
    {
    }