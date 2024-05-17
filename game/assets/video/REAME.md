# Note on video assets
Video with frequent seeking, such as title background video (`title_bga.mkv`), needs small keyframe interval.

If keyframe interval is big, you'll experience freezing for keyframe interval when video is loaded. (e.g. when changing from title to tutorial dialog scene)

You must install ffmpeg and ffprobe tools to run commands in this docs.

To see timestamp of keyframe, run this command as suggested by [Stackoverflow Answer](https://stackoverflow.com/a/18088156)

```bash
ffprobe -loglevel error -select_streams v:0 -show_entries packet=pts_time,flags -of csv=print_section=0 INPUT_FILE_NAME | awk -F',' '/K/ {print $1}'
```

To identify fps and codec used for the video file, run following command.

```bash
ffprobe -i INPUT_FILE_NAME
```

## How to change keyframe interval
Let `X` be interval in frame and `N` be interval in seconds. For example, if the video is 30fps and you want 1keyframe/2second, `N` is 2 and `X` is 60.
To change keyframe interval, run following command. (The command differs by video codec used)

For more information, read [this StackOverflow answer](https://superuser.com/a/908325).

### libx264
```bash
ffmpeg -i INPUT_FILE_NAME -g X -keyint_min X OUTPUT_FILE_NAME
```

### libx265
```bash
ffmpeg -i INPUT_FILE_NAME -x265-params "keyint=X:min-keyint=X" OUTPUT_FILENAME
```

### libvpx-vp9
```bash
ffmpeg -i INPUT_FILE_NAME -g X OUTPUT_FILE_NAME
```