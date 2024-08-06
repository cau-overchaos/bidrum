using System;
using System.Collections.Generic;
using System.Linq;
using bidrum.controller;
using Bidrum.DataStructLib;

namespace Bidrum.Controller
{
    public enum NoteAccuracy
    {
        PPPPerfect,
        Perfect,
        Great,
        Good,
        Bad,
        Miss
    }

    public class NoteJudgement
    {
        public NoteJudgement(GameNote note, NoteAccuracy accuracy)
        {
            Accuracy = accuracy;
            Note = note;
        }

        public NoteAccuracy Accuracy { get; private set; }
        public GameNote Note { get; private set; }
    }

    /// <summary>
    /// Judges accuracy of notes
    /// </summary>
    public class TimingJudge
    {
        private uint _bpm;
        private int _delay;
        private List<GameNote> _unprocessedNotes;

        public TimingJudge(IEnumerable<GameNote> notes, uint bpm, int delay)
        {
            _unprocessedNotes = new List<GameNote>(notes.OrderBy(note => note.TimingInMs(bpm, delay)));
            _bpm = bpm;
            _delay = delay;
        }

        private int GetJudgementTimingWindowOf(NoteAccuracy accuracy)
        {
            switch (accuracy)
            {
                case NoteAccuracy.PPPPerfect:
                    return 10;
                case NoteAccuracy.Perfect:
                    return 40;
                case NoteAccuracy.Great:
                    return 80;
                case NoteAccuracy.Good:
                    return 160;
                case NoteAccuracy.Bad:
                    return 320;
                case NoteAccuracy.Miss:
                    return 1500;
                default:
                    throw new ArgumentOutOfRangeException(nameof(accuracy), accuracy, null);
            }
        }

        private NoteAccuracy CalculateNoteAccuracyFromTimeDifference(long difference)
        {
            long abs = Math.Abs(difference);

            foreach (NoteAccuracy accuracy in new[]
                     {
                         NoteAccuracy.PPPPerfect, NoteAccuracy.Perfect, NoteAccuracy.Great, NoteAccuracy.Good,
                         NoteAccuracy.Bad
                     })
            {
                if (abs <= GetJudgementTimingWindowOf(accuracy))
                    return accuracy;
            }

            return NoteAccuracy.Miss;
        }

        public List<NoteJudgement> Judge(JangguStateWithTick jangguState, long tick)
        {
            List<NoteJudgement> noteJudgements = new List<NoteJudgement>();
            bool processedLeftStick = false, processedRightStick = false;

            foreach (GameNote note in _unprocessedNotes)
            {
                if (processedLeftStick && processedRightStick)
                    break;

                long preciseTiming = note.TimingInMs(_bpm, _delay);
                long differenceWithNoteTiming = tick - preciseTiming;

                // Judge the missed notes
                if (differenceWithNoteTiming > GetJudgementTimingWindowOf(NoteAccuracy.Bad))
                {
                    noteJudgements.Add(new NoteJudgement(note, NoteAccuracy.Miss));
                    continue;
                }

                // Skip not-yet notes
                if (differenceWithNoteTiming < -GetJudgementTimingWindowOf(NoteAccuracy.Miss))
                {
                    continue;
                }

                // Process note
                JangguStickStateWithTick keydownData = jangguState.GetByStick(note.Stick);
                if (keydownData.IsKeydownNow &&
                    keydownData.Face == note.Face &&
                    !((note.Stick == JangguStick.Left && processedLeftStick) ||
                      (note.Stick == JangguStick.Right && processedRightStick)))
                {
                    if (note.Stick == JangguStick.Left)
                    {
                        processedLeftStick = true;
                    }
                    else
                    {
                        processedRightStick = true;
                    }

                    long hitTiming = keydownData.KeydownTimingTick;
                    long difference = Math.Abs(hitTiming - preciseTiming);
                    NoteAccuracy accuracy = CalculateNoteAccuracyFromTimeDifference(difference);
                    noteJudgements.Add(new NoteJudgement(note, accuracy));

                    // TO-DO: calc score here
                }
            }

            foreach (NoteJudgement judgement in noteJudgements)
            {
                _unprocessedNotes.Remove(judgement.Note);
                // TO-DO: process combo here
                // TO-DO: process health here
            }

            return noteJudgements;
        }
    }
}