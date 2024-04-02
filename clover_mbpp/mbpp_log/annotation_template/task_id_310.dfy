twostate predicate pre_original(s: string, a: array<char>)
    reads a
    {
    true
    }

    twostate predicate pre_gen(s: string, a: array<char>)
    reads a

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: string, a: array<char>)
    ensures pre_original(s, a) <==> pre_gen(s, a)
    {
    }

    twostate predicate post_original(s: string, a: array<char>)
    reads a
    requires pre_original(s, a)
    {
    true
    }

    twostate predicate post_gen(s: string, a: array<char>)
    reads a
    requires pre_original(s, a)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: string, a: array<char>)
    requires pre_original(s, a)
    requires pre_gen(s, a)
    ensures post_original(s, a) <==> post_gen(s, a)
    {
    }