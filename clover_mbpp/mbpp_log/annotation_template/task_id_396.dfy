twostate predicate pre_original(s: string, result: bool)
    {
    true
    }

    twostate predicate pre_gen(s: string, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: string, result: bool)
    ensures pre_original(s, result) <==> pre_gen(s, result)
    {
    }

    twostate predicate post_original(s: string, result: bool)
    requires pre_original(s, result)
    {
    true
    }

    twostate predicate post_gen(s: string, result: bool)
    requires pre_original(s, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: string, result: bool)
    requires pre_original(s, result)
    requires pre_gen(s, result)
    ensures post_original(s, result) <==> post_gen(s, result)
    {
    }