twostate predicate pre_original(first: seq<int>, second: seq<int>, result: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(first: seq<int>, second: seq<int>, result: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(first: seq<int>, second: seq<int>, result: seq<int>)
    ensures pre_original(first, second, result) <==> pre_gen(first, second, result)
    {
    }

    twostate predicate post_original(first: seq<int>, second: seq<int>, result: seq<int>)
    requires pre_original(first, second, result)
    {
    true
    }

    twostate predicate post_gen(first: seq<int>, second: seq<int>, result: seq<int>)
    requires pre_original(first, second, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(first: seq<int>, second: seq<int>, result: seq<int>)
    requires pre_original(first, second, result)
    requires pre_gen(first, second, result)
    ensures post_original(first, second, result) <==> post_gen(first, second, result)
    {
    }