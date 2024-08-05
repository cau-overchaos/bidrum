public struct GameResult
{
    public ulong OverchaosCount { get; set; }
    public ulong PerfectCount { get; set; }
    public ulong GreatCount { get; set; }
    public ulong GoodCount { get; set; }
    public ulong BadCount { get; set; }
    public ulong MissCount { get; set; }
    public ulong Combo { get; set; }
    public ulong MaxCombo { get; set; }
    public ulong Score { get; set; }
    public long Health { get; set; }
    public ulong MaxHealth { get; set; }

    public ulong TotalJudgedNoteCount()
    {
        return OverchaosCount
            + PerfectCount
            + GreatCount
            + GoodCount
            + BadCount
            + MissCount;
    }
}
