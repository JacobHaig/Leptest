use leptos::*;

use styled::style;

struct Song {
    index: u32,
    image_url: String,
    title: String,
    artist: String,
    song_length: u32,
}

#[component]
fn Song_card(
    index: u32,
    image_url: String,
    title: String,
    artist: String,
    song_length: u32,
) -> impl IntoView {
    // convert song_length seconds into minutes and seconds format
    let minutes = song_length / 60;
    let seconds = song_length % 60;
    let song_length = format!("{}:{}", minutes, seconds);

    let styles = style!(
        .song_card {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            padding: 10px;
            border: 1px solid black;
            border-radius: 5px;
            // margin: 10px;
            background-color: green;
            color: white;
        }
        .song_card_image {
            width: 20px;
            height: 20px;
            border-radius: 5px;
            // margin-right: 10px;
        }
    );

    styled::view! {
        styles,
        <div class="song_card">
            <div class="song_card_index">{ index }</div>
            <img class="song_card_image" src= image_url />
            <div class="song_card_title">{ title }</div>
            <div class="song_card_artist">{ artist }</div>
            <div class="song_card_length">{ song_length }</div>
            // <button on:click= move |_| set_is_playing(!is_playing.get()) >{ if is_playing.get() { "Pause" } else { "Play" } }</button>
        </div>
    }
}
#[component]
fn Song_Panel() -> impl IntoView {
    // list of songs for testing
    let songs = vec![
        Song {
            index: 1,
            image_url: "https://imgs.search.brave.com/2-_CNDuVoC-zJ89WVorCJJhgQE90AMUDyv1rYDDgGUQ/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9jZG4t/aWNvbnMtcG5nLmZs/YXRpY29uLmNvbS8x/MjgvMjY1OS8yNjU5/MzYwLnBuZw".to_string(),
            title: "Song 1".to_string(),
            artist: "Artist 1".to_string(),
            song_length: 136,
        },
        Song {
            index: 2,
            image_url: "https://imgs.search.brave.com/2-_CNDuVoC-zJ89WVorCJJhgQE90AMUDyv1rYDDgGUQ/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9jZG4t/aWNvbnMtcG5nLmZs/YXRpY29uLmNvbS8x/MjgvMjY1OS8yNjU5/MzYwLnBuZw".to_string(),
            title: "Song 2".to_string(),
            artist: "Artist 2".to_string(),
            song_length: 10,
        },
        Song {
            index: 3,
            image_url: "https://imgs.search.brave.com/2-_CNDuVoC-zJ89WVorCJJhgQE90AMUDyv1rYDDgGUQ/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9jZG4t/aWNvbnMtcG5nLmZs/YXRpY29uLmNvbS8x/MjgvMjY1OS8yNjU5/MzYwLnBuZw".to_string(),
            title: "Song 3".to_string(),
            artist: "Artist 3".to_string(),
            song_length: 543,
        },
    ];

    let styles = style!(
        padding: 10px;
        background-color: red;
        color: white;
    );

    styled::view! {
        styles,
        <div>
            <p>"Song Panel"</p>

            <div>
            {  songs.iter().map(|song|
                view! {
                    <Song_card
                        index= song.index
                        image_url= song.image_url.clone()
                        title= song.title.clone()
                        artist= song.artist.clone()
                        song_length= song.song_length
                    />
                }).collect_view()
            }
        </div>
        </div>
    }
}

#[component]
fn Search() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    let set_name_fn = move |ev| {
        // event_target_value is a Leptos helper function it functions the same way as event.target.value
        // in JavaScript, but smooths out some of the typecasting necessary to make this work in Rust
        // console_log(&format!("{:?}", &ev.));
        set_name(event_target_value(&ev));
    };

    let styles = style!(
        padding: 10px;
        background-color: blue;
        color: white;
    );

    styled::view! {
        styles,
        <div>
            <p>"Search"</p>
            <input type="text"
                on:input=set_name_fn
                prop:value=name // the `prop:` syntax lets you update a DOM property, rather than an attribute.
            />
        </div>
    }
}

#[component]
fn Current_Song_Panel() -> impl IntoView {
    let styles = style!(
        // padding: 10px;
        background-color: blue;
        color: white;
        width: 100%;
        // position the panel at the bottom of the screen
        position: fixed;
        bottom: 0;
        left: 0;
        height: 50px;

    );

    styled::view! {
        styles,
        <div>
            <p>"Current Song Panel"</p>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <p>"Count: "{ count }</p>
            <button on:click= move |_| set_count(count.get() + 1) >"Increment"</button>
            <Search/>
            <Song_Panel/>
            <Current_Song_Panel/>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
