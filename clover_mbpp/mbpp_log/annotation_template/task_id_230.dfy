twostate predicate pre_original(s: string, ch: char, v: string)
    {
    true
    }

    twostate predicate pre_gen(s: string, ch: char, v: string)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: string, ch: char, v: string)
    ensures pre_original(s, ch, v) <==> pre_gen(s, ch, v)
    {
    }

    twostate predicate post_original(s: string, ch: char, v: string)
    requires pre_original(s, ch, v)
    {
    true
    }

    twostate predicate post_gen(s: string, ch: char, v: string)
    requires pre_original(s, ch, v)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: string, ch: char, v: string)
    requires pre_original(s, ch, v)
    requires pre_gen(s, ch, v)
    ensures post_original(s, ch, v) <==> post_gen(s, ch, v)
    {
    }