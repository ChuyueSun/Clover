twostate predicate pre_original(n: int, number: int)
    {
    true
    }

    twostate predicate pre_gen(n: int, number: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, number: int)
    ensures pre_original(n, number) <==> pre_gen(n, number)
    {
    }

    twostate predicate post_original(n: int, number: int)
    requires pre_original(n, number)
    {
    true
    }

    twostate predicate post_gen(n: int, number: int)
    requires pre_original(n, number)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, number: int)
    requires pre_original(n, number)
    requires pre_gen(n, number)
    ensures post_original(n, number) <==> post_gen(n, number)
    {
    }