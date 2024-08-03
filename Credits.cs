using System;

namespace bidrumgodot;

public class Credits
{
    public int Coins { get; set; }

    private int Price => GameSettings.GetSettings().Price;

    /// <summary>
    /// Available credits for playing game.
    ///
    /// For example, If 7 coins are not used yet and the price is 3 coins, return value will be 2. 
    /// </summary>
    /// <returns>Available credits for playing game</returns>
    public int AvailableCredits => Coins / Price;

    /// <summary>
    /// Leftover coins which cannot be a credit
    ///
    /// For example, If 7 coins are not used yet and the price is 3 coins, return value will be 1.
    /// </summary>
    /// <returns>Leftover coins</returns>
    private int LeftoverCoins()
    {
        return Coins % Price;
    }

    public void ConsumeCredit()
    {
        Coins -= Price;
    }

    public String CoinText()
    {
        if (GameSettings.GetSettings().Uncommericial)
        {
            return "FREE PLAY (UNCOMMERCIAL)";
        }
        if (Price == 0)
        {
            return "FREE PLAY";
        }
        if (Price == 1)
        {
            return $"CREDIT: {Coins}";
        }
        return $"CREDIT: {AvailableCredits} ({LeftoverCoins()}/{Price})";
    }
}