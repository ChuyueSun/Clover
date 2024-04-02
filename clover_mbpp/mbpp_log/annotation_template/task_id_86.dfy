twostate predicate pre_original(n: nat, result: nat)
    {
    true
    }

    twostate predicate pre_gen(n: nat, result: nat)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: nat, result: nat)
    ensures pre_original(n, result) <==> pre_gen(n, result)
    {
    }

    twostate predicate post_original(n: nat, result: nat)
    requires pre_original(n, result)
    {
    true
    }

    twostate predicate post_gen(n: nat, result: nat)
    requires pre_original(n, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: nat, result: nat)
    requires pre_original(n, result)
    requires pre_gen(n, result)
    ensures post_original(n, result) <==> post_gen(n, result)
    {
    }