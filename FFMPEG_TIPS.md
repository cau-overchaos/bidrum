# FFmpeg Command Tips
- No audio (Only video): `ffmpeg -i input_filename -c copy -an output_filename`
- No video (Only audio): `ffmpeg -i input_filename -vn -acodec copy output_filename`
    - `-acodec copy` may be omitted sometimes

## Supported audio formats by kira
- mp3
- ogg
- flac
- wav