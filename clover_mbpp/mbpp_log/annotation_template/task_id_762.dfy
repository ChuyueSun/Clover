twostate predicate pre_original(month: int, result: bool)
    {
    true
    }

    twostate predicate pre_gen(month: int, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(month: int, result: bool)
    ensures pre_original(month, result) <==> pre_gen(month, result)
    {
    }

    twostate predicate post_original(month: int, result: bool)
    requires pre_original(month, result)
    {
    true
    }

    twostate predicate post_gen(month: int, result: bool)
    requires pre_original(month, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(month: int, result: bool)
    requires pre_original(month, result)
    requires pre_gen(month, result)
    ensures post_original(month, result) <==> post_gen(month, result)
    {
    }