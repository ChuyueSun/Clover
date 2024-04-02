twostate predicate pre_original(l: seq<string>, r: seq<char>)
    {
    true
    }

    twostate predicate pre_gen(l: seq<string>, r: seq<char>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(l: seq<string>, r: seq<char>)
    ensures pre_original(l, r) <==> pre_gen(l, r)
    {
    }

    twostate predicate post_original(l: seq<string>, r: seq<char>)
    requires pre_original(l, r)
    {
    true
    }

    twostate predicate post_gen(l: seq<string>, r: seq<char>)
    requires pre_original(l, r)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(l: seq<string>, r: seq<char>)
    requires pre_original(l, r)
    requires pre_gen(l, r)
    ensures post_original(l, r) <==> post_gen(l, r)
    {
    }