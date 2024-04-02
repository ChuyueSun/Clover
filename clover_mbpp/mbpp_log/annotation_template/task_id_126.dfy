twostate predicate pre_original(a: int, b: int, sum: int)
    {
    true
    }

    twostate predicate pre_gen(a: int, b: int, sum: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: int, b: int, sum: int)
    ensures pre_original(a, b, sum) <==> pre_gen(a, b, sum)
    {
    }

    twostate predicate post_original(a: int, b: int, sum: int)
    requires pre_original(a, b, sum)
    {
    true
    }

    twostate predicate post_gen(a: int, b: int, sum: int)
    requires pre_original(a, b, sum)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: int, b: int, sum: int)
    requires pre_original(a, b, sum)
    requires pre_gen(a, b, sum)
    ensures post_original(a, b, sum) <==> post_gen(a, b, sum)
    {
    }