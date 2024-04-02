twostate predicate pre_original(a: array<int>)
    reads a
    {
    true
    }

    twostate predicate pre_gen(a: array<int>)
    reads a

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: array<int>)
    ensures pre_original() <==> pre_gen()
    {
    }

    twostate predicate post_original(a: array<int>)
    reads a
    requires pre_original()
    {
    true
    }

    twostate predicate post_gen(a: array<int>)
    reads a
    requires pre_original()
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: array<int>)
    requires pre_original()
    requires pre_gen()
    ensures post_original() <==> post_gen()
    {
    }