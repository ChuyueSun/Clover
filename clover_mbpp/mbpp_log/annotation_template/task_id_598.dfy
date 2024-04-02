twostate predicate pre_original(n: int, result: bool)
    {
    true
    }

    twostate predicate pre_gen(n: int, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, result: bool)
    ensures pre_original(n, result) <==> pre_gen(n, result)
    {
    }

    twostate predicate post_original(n: int, result: bool)
    requires pre_original(n, result)
    {
    true
    }

    twostate predicate post_gen(n: int, result: bool)
    requires pre_original(n, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, result: bool)
    requires pre_original(n, result)
    requires pre_gen(n, result)
    ensures post_original(n, result) <==> post_gen(n, result)
    {
    }