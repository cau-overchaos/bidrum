using System;
using System.Collections.Generic;
using bidrum.controller;
using Bidrum.DataStructLib;

namespace Bidrum.Controller
{
    public enum NoteAccuracy
    {
        Overchaos,
        Perfect,
        Great,
        Good,
        Bad,
        Miss
    }

    public struct JudgeResult
    {
        public NoteAccuracy Accuracy { get; set; }
        public ulong NoteId { get; set; }
    }

    public struct NoteForProcessing
    {
        public GameNote Note { get; set; }
        public uint Bpm { get; set; }
        public ulong Delay { get; set; }
        public ulong Id { get; set; }
        public ulong? HitTiming { get; set; }
    }

    public class TimingJudge
    {
        private ulong _badCount;
        private ulong _combo;
        private ulong _goodCount;
        private ulong _greatCount;
        private long _health;
        private ulong _maxCombo;
        private ulong _maxHealth;
        private ulong _missCount;
        private List<NoteForProcessing> _notes;
        private ulong _overchaosCount;
        private ulong _perfectCount;
        private ulong _score;

        public TimingJudge(GameChart chart)
        {
            _notes = new List<NoteForProcessing>();

            foreach (var j in chart.LeftFace)
            {
                _notes.Add(new NoteForProcessing
                {
                    Note = j,
                    Bpm = chart.Bpm,
                    Delay = chart.Delay,
                    Id = j.Id,
                    HitTiming = null
                });
            }

            foreach (var j in chart.RightFace)
            {
                _notes.Add(new NoteForProcessing
                {
                    Note = j,
                    Bpm = chart.Bpm,
                    Delay = chart.Delay,
                    Id = j.Id,
                    HitTiming = null
                });
            }

            _notes.Sort((a, b) => a.Note.TimingInMs(a.Bpm, a.Delay).CompareTo(b.Note.TimingInMs(b.Bpm, b.Delay)));

            _overchaosCount = 0;
            _perfectCount = 0;
            _greatCount = 0;
            _goodCount = 0;
            _badCount = 0;
            _missCount = 0;
            _combo = 0;
            _maxCombo = 0;
            _score = 0;
            _health = (long)Constants.DEFAULT_HEALTH;
            _maxHealth = Constants.DEFAULT_HEALTH;
        }

        public List<JudgeResult> Judge(JangguStateWithTick keydown, ulong tickInMilliseconds)
        {
            throw new NotImplementedException();
            var judgedNotes = new List<JudgeResult>();

            bool processedLeftStick = false;
            bool processedRightStick = false;

            foreach (var note in _notes)
            {
                if (processedLeftStick && processedRightStick) break;

                var preciseTiming = note.Note.TimingInMs(note.Bpm, note.Delay);
                var difference = (long)tickInMilliseconds - (long)preciseTiming;

                if (difference > Constants.BAD_TIMING)
                {
                    judgedNotes.Add(new JudgeResult
                    {
                        NoteId = note.Id,
                        Accuracy = NoteAccuracy.Miss
                    });
                    continue;
                }

                if (difference < -Constants.BAD_TIMING)
                {
                    continue;
                }

                var keydownData = note.Note.Stick switch
                {
                    JangguStick.Left => keydown.LeftStick,
                    JangguStick.Right => keydown.RightStick,
                    _ => throw new ArgumentOutOfRangeException()
                };

                if (keydown.GetByStick(note.Note.Stick).IsKeydownNow &&
                    keydownData.Face.HasValue && keydownData.Face.Value == note.Note.Face &&
                    (note.Note.Stick == JangguStick.Left ? !processedLeftStick : !processedRightStick))
                {
                    if (note.Note.Stick == JangguStick.Left)
                        processedLeftStick = true;
                    else
                        processedRightStick = true;

                    // note.HitTiming = (ulong)keydownData.KeydownTimingTick;
                }

                if (note.HitTiming.HasValue)
                {
                    var differenceAbs = Math.Abs((long)note.HitTiming.Value - (long)preciseTiming);
                    // _score += (ulong)((Math.Abs((Constants.BAD_TIMING - (double)differenceAbs).Clamp(Constants.OVERCHAOS_TIMING, Constants.BAD_TIMING)) / (Constants.BAD_TIMING - Constants.OVERCHAOS_TIMING)) * 1000.0);
                    var noteAccuracy = NoteAccuracyFromTimeDifference(differenceAbs);

                    judgedNotes.Add(new JudgeResult
                    {
                        NoteId = note.Id,
                        Accuracy = noteAccuracy
                    });
                }
            }

            foreach (var judgedNote in judgedNotes)
            {
                _notes.RemoveAll(x => x.Id == judgedNote.NoteId);

                bool isHealthZero = _health == 0;

                switch (judgedNote.Accuracy)
                {
                    case NoteAccuracy.Overchaos:
                        if (Constants.OVERCHAOS_COMBO == 0)
                        {
                            _maxCombo = Math.Max(_maxCombo, _combo);
                            _combo = 0;
                        }
                        else
                        {
                            _combo += Constants.OVERCHAOS_COMBO;
                        }

                        _health += Constants.OVERCHAOS_HEALTH;
                        _overchaosCount++;
                        break;
                    case NoteAccuracy.Perfect:
                        if (Constants.PERFECT_COMBO == 0)
                        {
                            _maxCombo = Math.Max(_maxCombo, _combo);
                            _combo = 0;
                        }
                        else
                        {
                            _combo += Constants.PERFECT_COMBO;
                        }

                        _health += Constants.PERFECT_HEALTH;
                        _perfectCount++;
                        break;
                    case NoteAccuracy.Great:
                        if (Constants.GREAT_COMBO == 0)
                        {
                            _maxCombo = Math.Max(_maxCombo, _combo);
                            _combo = 0;
                        }
                        else
                        {
                            _combo += Constants.GREAT_COMBO;
                        }

                        _health += Constants.GREAT_HEALTH;
                        _greatCount++;
                        break;
                    case NoteAccuracy.Good:
                        if (Constants.GOOD_COMBO == 0)
                        {
                            _maxCombo = Math.Max(_maxCombo, _combo);
                            _combo = 0;
                        }
                        else
                        {
                            _combo += Constants.GOOD_COMBO;
                        }

                        _health += Constants.GOOD_HEALTH;
                        _goodCount++;
                        break;
                    case NoteAccuracy.Bad:
                        if (Constants.BAD_COMBO == 0)
                        {
                            _maxCombo = Math.Max(_maxCombo, _combo);
                            _combo = 0;
                        }
                        else
                        {
                            _combo += Constants.BAD_COMBO;
                        }

                        _health += Constants.BAD_HEALTH;
                        _badCount++;
                        break;
                    case NoteAccuracy.Miss:
                        if (Constants.MISS_COMBO == 0)
                        {
                            _maxCombo = Math.Max(_maxCombo, _combo);
                            _combo = 0;
                        }
                        else
                        {
                            _combo += Constants.MISS_COMBO;
                        }

                        _health += Constants.MISS_HEALTH;
                        _missCount++;
                        break;
                }

                if (isHealthZero)
                {
                    _health = 0;
                }
                else
                {
                    _health = Math.Clamp(_health, 0, (long)_maxHealth);
                }
            }

            return judgedNotes;
        }

        public GameResult GetGameResult()
        {
            return new GameResult
            {
                OverchaosCount = _overchaosCount,
                PerfectCount = _perfectCount,
                GreatCount = _greatCount,
                GoodCount = _goodCount,
                BadCount = _badCount,
                MissCount = _missCount,
                Combo = _combo,
                MaxCombo = _maxCombo,
                Score = _score,
                Health = _health,
                MaxHealth = _maxHealth
            };
        }

        private NoteAccuracy NoteAccuracyFromTimeDifference(long differenceAbs)
        {
            if (differenceAbs <= Constants.OVERCHAOS_TIMING)
            {
                return NoteAccuracy.Overchaos;
            }
            else if (differenceAbs <= Constants.PERFECT_TIMING)
            {
                return NoteAccuracy.Perfect;
            }
            else if (differenceAbs <= Constants.GREAT_TIMING)
            {
                return NoteAccuracy.Great;
            }
            else if (differenceAbs <= Constants.GOOD_TIMING)
            {
                return NoteAccuracy.Good;
            }
            else if (differenceAbs <= Constants.BAD_TIMING)
            {
                return NoteAccuracy.Bad;
            }
            else
            {
                return NoteAccuracy.Miss;
            }
        }
    }
}