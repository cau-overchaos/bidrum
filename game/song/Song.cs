using System.Collections.Generic;
using System.Text.Json.Serialization;
using bidrum.controller;

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
        [JsonIgnore] public ulong Id { get; set; }

        public Fraction Beat { get; set; }
        public JangguStick Stick { get; set; }
        public JangguFace Face { get; set; }

        public long TimingInMs(uint bpm, int delay)
        {
            return (long)((Beat) * (60000.0 / bpm) + delay);
        }
    }

    public class GameChart
    {
        public string Artist { get; set; }
        public int Delay { get; set; }
        public uint Bpm { get; set; }
        public GameNote[] Notes { get; set; }
    }
}