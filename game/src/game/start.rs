use bidrum_data_struct_lib::song::GameSong;

use super::{
    display_result::display_result, game_common_context::GameCommonContext, game_player::play_song,
    select_song::select_song, tutorial,
};

pub(crate) fn start_game(common_context: &mut GameCommonContext) {
    let songs = GameSong::get_songs();
    let mut total_stages = 0;

    // TO-DO: do authentication here
    tutorial::do_tutorial(common_context);

    // player can play 3 songs with a credit
    while total_stages < 3 {
        // select song
        let selected = select_song(common_context, &songs);

        // play selected song
        let result = play_song(
            common_context,
            &selected.selected_song,
            selected.selected_level,
        );

        // display play result
        if let Some(result_unwrapped) = result {
            display_result(
                common_context,
                result_unwrapped,
                &selected.selected_song,
                selected.selected_level,
            );
        }

        total_stages += 1;
    }

    // TO-DO: call game over screen
}
