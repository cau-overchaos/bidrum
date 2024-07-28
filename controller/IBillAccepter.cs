namespace bidrumgodot.controller;

public delegate void NewCoinEventListener(int coins);
public interface IBillAccepter
{
    public int GetCoins();
    public void ConsumeCoins(int coins);
}