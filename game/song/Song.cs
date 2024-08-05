using bidrum.controller;
using System;
using System.Collections.Generic;

namespace Bidrum.DataStructLib
{
    public enum GameSongCategory
    {
        Kpop,
        TraditionalKpop,
        Jpop,
        Variety
    }

    public class GameSong
    {
        public string Path { get; set; }
        public string Title { get; set; }
        public string Artist { get; set; }
        public GameSongCategory Category { get; set; }
        public string AudioFilename { get; set; }
        public string VideoFilename { get; set; }
        public string CoverImageFilename { get; set; }
        public List<uint> Levels { get; set; }
    }

    public class GameNote
    {
        public JangguStick Stick { get; set; }
        public ulong BeatIndex { get; set; }
        public long TickNomiator { get; set; }
        public long TickDenomiator { get; set; }
        public ulong Id { get; set; }
        public JangguFace Face { get; set; }

        public double TimingInMs(uint bpm, ulong delay)
        {
            return (double)BeatIndex + ((double)TickNomiator / TickDenomiator) * (60000.0 / bpm) + delay;
        }
    }

    public class GameChart
    {
        public string Artist { get; set; }
        public ulong Delay { get; set; }
        public uint Bpm { get; set; }
        public List<GameNote> LeftFace { get; set; }
        public List<GameNote> RightFace { get; set; }
    }
}
