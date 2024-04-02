twostate predicate pre_original(s: seq<seq<int>>, minSublist: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(s: seq<seq<int>>, minSublist: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: seq<seq<int>>, minSublist: seq<int>)
    ensures pre_original(s, minSublist) <==> pre_gen(s, minSublist)
    {
    }

    twostate predicate post_original(s: seq<seq<int>>, minSublist: seq<int>)
    requires pre_original(s, minSublist)
    {
    true
    }

    twostate predicate post_gen(s: seq<seq<int>>, minSublist: seq<int>)
    requires pre_original(s, minSublist)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: seq<seq<int>>, minSublist: seq<int>)
    requires pre_original(s, minSublist)
    requires pre_gen(s, minSublist)
    ensures post_original(s, minSublist) <==> post_gen(s, minSublist)
    {
    }