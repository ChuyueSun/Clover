twostate predicate pre_original(costPrice: int, sellingPrice: int, result: bool)
    {
    true
    }

    twostate predicate pre_gen(costPrice: int, sellingPrice: int, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(costPrice: int, sellingPrice: int, result: bool)
    ensures pre_original(costPrice, sellingPrice, result) <==> pre_gen(costPrice, sellingPrice, result)
    {
    }

    twostate predicate post_original(costPrice: int, sellingPrice: int, result: bool)
    requires pre_original(costPrice, sellingPrice, result)
    {
    true
    }

    twostate predicate post_gen(costPrice: int, sellingPrice: int, result: bool)
    requires pre_original(costPrice, sellingPrice, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(costPrice: int, sellingPrice: int, result: bool)
    requires pre_original(costPrice, sellingPrice, result)
    requires pre_gen(costPrice, sellingPrice, result)
    ensures post_original(costPrice, sellingPrice, result) <==> post_gen(costPrice, sellingPrice, result)
    {
    }