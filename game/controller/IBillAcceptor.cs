namespace bidrum.controller;

public delegate void NewCoinEventListener(int coins);

public interface IBillAcceptor
{
    public int GetCoins();
    public void ConsumeCoins(int coins);
}