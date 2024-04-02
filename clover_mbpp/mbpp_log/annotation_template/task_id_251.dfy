twostate predicate pre_original(s: seq<string>, x: string, v: seq<string>)
    {
    true
    }

    twostate predicate pre_gen(s: seq<string>, x: string, v: seq<string>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: seq<string>, x: string, v: seq<string>)
    ensures pre_original(s, x, v) <==> pre_gen(s, x, v)
    {
    }

    twostate predicate post_original(s: seq<string>, x: string, v: seq<string>)
    requires pre_original(s, x, v)
    {
    true
    }

    twostate predicate post_gen(s: seq<string>, x: string, v: seq<string>)
    requires pre_original(s, x, v)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: seq<string>, x: string, v: seq<string>)
    requires pre_original(s, x, v)
    requires pre_gen(s, x, v)
    ensures post_original(s, x, v) <==> post_gen(s, x, v)
    {
    }