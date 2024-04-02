twostate predicate pre_original(a: int, b: int, maxValue: int)
    {
    true
    }

    twostate predicate pre_gen(a: int, b: int, maxValue: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: int, b: int, maxValue: int)
    ensures pre_original(a, b, maxValue) <==> pre_gen(a, b, maxValue)
    {
    }

    twostate predicate post_original(a: int, b: int, maxValue: int)
    requires pre_original(a, b, maxValue)
    {
    true
    }

    twostate predicate post_gen(a: int, b: int, maxValue: int)
    requires pre_original(a, b, maxValue)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: int, b: int, maxValue: int)
    requires pre_original(a, b, maxValue)
    requires pre_gen(a, b, maxValue)
    ensures post_original(a, b, maxValue) <==> post_gen(a, b, maxValue)
    {
    }