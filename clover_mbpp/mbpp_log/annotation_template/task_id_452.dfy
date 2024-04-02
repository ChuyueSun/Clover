twostate predicate pre_original(costPrice: int, sellingPrice: int, loss: int)
    {
    true
    }

    twostate predicate pre_gen(costPrice: int, sellingPrice: int, loss: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(costPrice: int, sellingPrice: int, loss: int)
    ensures pre_original(costPrice, sellingPrice, loss) <==> pre_gen(costPrice, sellingPrice, loss)
    {
    }

    twostate predicate post_original(costPrice: int, sellingPrice: int, loss: int)
    requires pre_original(costPrice, sellingPrice, loss)
    {
    true
    }

    twostate predicate post_gen(costPrice: int, sellingPrice: int, loss: int)
    requires pre_original(costPrice, sellingPrice, loss)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(costPrice: int, sellingPrice: int, loss: int)
    requires pre_original(costPrice, sellingPrice, loss)
    requires pre_gen(costPrice, sellingPrice, loss)
    ensures post_original(costPrice, sellingPrice, loss) <==> post_gen(costPrice, sellingPrice, loss)
    {
    }