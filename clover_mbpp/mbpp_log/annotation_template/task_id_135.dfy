twostate predicate pre_original(n: int, hexNum: int)
    {
    true
    }

    twostate predicate pre_gen(n: int, hexNum: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, hexNum: int)
    ensures pre_original(n, hexNum) <==> pre_gen(n, hexNum)
    {
    }

    twostate predicate post_original(n: int, hexNum: int)
    requires pre_original(n, hexNum)
    {
    true
    }

    twostate predicate post_gen(n: int, hexNum: int)
    requires pre_original(n, hexNum)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, hexNum: int)
    requires pre_original(n, hexNum)
    requires pre_gen(n, hexNum)
    ensures post_original(n, hexNum) <==> post_gen(n, hexNum)
    {
    }