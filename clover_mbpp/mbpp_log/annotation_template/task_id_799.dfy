twostate predicate pre_original(n: bv32, d: int, result: bv32)
    {
    true
    }

    twostate predicate pre_gen(n: bv32, d: int, result: bv32)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: bv32, d: int, result: bv32)
    ensures pre_original(n, d, result) <==> pre_gen(n, d, result)
    {
    }

    twostate predicate post_original(n: bv32, d: int, result: bv32)
    requires pre_original(n, d, result)
    {
    true
    }

    twostate predicate post_gen(n: bv32, d: int, result: bv32)
    requires pre_original(n, d, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: bv32, d: int, result: bv32)
    requires pre_original(n, d, result)
    requires pre_gen(n, d, result)
    ensures post_original(n, d, result) <==> post_gen(n, d, result)
    {
    }