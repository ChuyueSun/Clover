twostate predicate pre_original(l: seq<, r: seq<(int, int)>)
    {
    true
    }

    twostate predicate pre_gen(l: seq<, r: seq<(int, int)>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(l: seq<, r: seq<(int, int)>)
    ensures pre_original(int, int)), r, int)>) <==> pre_gen(int, int)), r, int)>)
    {
    }

    twostate predicate post_original(l: seq<, r: seq<(int, int)>)
    requires pre_original(int, int)), r, int)>)
    {
    true
    }

    twostate predicate post_gen(l: seq<, r: seq<(int, int)>)
    requires pre_original(int, int)), r, int)>)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(l: seq<, r: seq<(int, int)>)
    requires pre_original(int, int)), r, int)>)
    requires pre_gen(int, int)), r, int)>)
    ensures post_original(int, int)), r, int)>) <==> post_gen(int, int)), r, int)>)
    {
    }