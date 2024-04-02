twostate predicate pre_original(s1: seq<int>, s2: seq<int>, s3: seq<int>, r: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(s1: seq<int>, s2: seq<int>, s3: seq<int>, r: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s1: seq<int>, s2: seq<int>, s3: seq<int>, r: seq<int>)
    ensures pre_original(s1, s2, s3, r) <==> pre_gen(s1, s2, s3, r)
    {
    }

    twostate predicate post_original(s1: seq<int>, s2: seq<int>, s3: seq<int>, r: seq<int>)
    requires pre_original(s1, s2, s3, r)
    {
    true
    }

    twostate predicate post_gen(s1: seq<int>, s2: seq<int>, s3: seq<int>, r: seq<int>)
    requires pre_original(s1, s2, s3, r)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s1: seq<int>, s2: seq<int>, s3: seq<int>, r: seq<int>)
    requires pre_original(s1, s2, s3, r)
    requires pre_gen(s1, s2, s3, r)
    ensures post_original(s1, s2, s3, r) <==> post_gen(s1, s2, s3, r)
    {
    }