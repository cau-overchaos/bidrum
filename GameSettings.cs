namespace bidrum;

public class GameSettings
{
    /// <summary>
    /// Price of game
    /// </summary>
    public int Price { get; private set; } = 2;

    public bool Uncommericial { get; private set; } = false;

    public static GameSettings GetSettings()
    {
        return new GameSettings();
    }
}