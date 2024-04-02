twostate predicate pre_original(s: string, v: seq<char>)
    {
    true
    }

    twostate predicate pre_gen(s: string, v: seq<char>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: string, v: seq<char>)
    ensures pre_original(s, v) <==> pre_gen(s, v)
    {
    }

    twostate predicate post_original(s: string, v: seq<char>)
    requires pre_original(s, v)
    {
    true
    }

    twostate predicate post_gen(s: string, v: seq<char>)
    requires pre_original(s, v)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: string, v: seq<char>)
    requires pre_original(s, v)
    requires pre_gen(s, v)
    ensures post_original(s, v) <==> post_gen(s, v)
    {
    }