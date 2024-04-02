twostate predicate pre_original(s1: string, s2: string, v: string)
    {
    true
    }

    twostate predicate pre_gen(s1: string, s2: string, v: string)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s1: string, s2: string, v: string)
    ensures pre_original(s1, s2, v) <==> pre_gen(s1, s2, v)
    {
    }

    twostate predicate post_original(s1: string, s2: string, v: string)
    requires pre_original(s1, s2, v)
    {
    true
    }

    twostate predicate post_gen(s1: string, s2: string, v: string)
    requires pre_original(s1, s2, v)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s1: string, s2: string, v: string)
    requires pre_original(s1, s2, v)
    requires pre_gen(s1, s2, v)
    ensures post_original(s1, s2, v) <==> post_gen(s1, s2, v)
    {
    }