use leptos::*;

// use styled::style;
use stylist;

#[component]
pub fn CurrentSongPanel() -> impl IntoView {
    view! {
        <div class="flex items-center gap-4 mt-8">
            <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-6 w-6"
                >
                    <path d="m12 19-7-7 7-7"></path>
                    <path d="M19 12H5"></path>
                </svg>
                <span class="sr-only">Previous</span>
            </button>

            <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-6 w-6"
                >
                    <polygon points="5 3 19 12 5 21 5 3"></polygon>
                </svg>
                <span class="sr-only">Play</span>
            </button>

            <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-6 w-6"
                >
                    <path d="M5 12h14"></path>
                    <path d="m12 5 7 7-7 7"></path>
                </svg>
                <span class="sr-only">Next</span>
            </button>
        </div>

        <div class="mt-8">
            <div class="h-2 bg-gray-700 rounded">
                <div class="h-full bg-green-500 rounded" style="width: 50%;"></div>
            </div>
            <p class="text-sm text-gray-500 mt-2">Current Song Progress</p>
        </div>
    }
}

// #[component]
// fn SidePanel() -> impl IntoView {
//     view! {
//         <aside class="w-64 p-6 space-y-6">
//             <div>
//                 <h1 class="text-2xl font-bold">Spotify</h1>
//             </div>
//             <nav>
//                 <ul class="space-y-4">
//                     <li>
//                         <a class="text-lg font-semibold" href="#"> Home </a>
//                     </li>
//                     <li>
//                         <a class="text-lg font-semibold" href="#"> Browse </a>
//                     </li>
//                     <li>
//                         <a class="text-lg font-semibold" href="#"> Your Library </a>
//                     </li>
//                 </ul>
//             </nav>

//             <div>
//                 <h2 class="text-lg font-semibold">Your Playlists</h2>
//                 <ul class="mt-2 space-y-2">
//                     <li>
//                         <a class="text-base" href="#"> Liked Songs </a>
//                     </li>
//                     <li>
//                         <a class="text-base" href="#"> Indie Pop </a>
//                     </li>
//                     <li>
//                         <a class="text-base" href="#"> Chill Vibes </a>
//                     </li>
//                 </ul>
//             </div>
//         </aside>
//     }
// }

// #[component]
// fn SearchSong() -> impl IntoView {
//     view! {
//         <div class="flex items-center gap-4 mb-8">
//             <form class="flex-1">
//                 <div class="relative">
//                     <svg
//                         xmlns="http://www.w3.org/2000/svg"
//                         width="24"
//                         height="24"
//                         viewBox="0 0 24 24"
//                         fill="none"
//                         stroke="currentColor"
//                         stroke-width="2"
//                         stroke-linecap="round"
//                         stroke-linejoin="round"
//                         class="absolute left-2.5 top-2.5 h-4 w-4 text-gray-500"
//                     >
//                         <circle cx="11" cy="11" r="8"></circle>
//                         <path d="m21 21-4.3-4.3"></path>
//                     </svg>
//                     <input
//                         class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 pl-8"
//                         placeholder="Search songs, artists, albums..."
//                         type="search"
//                     />
//                 </div>
//             </form>
//         </div>
//     }
// }

// #[component]
// fn SongCard() -> impl IntoView {
//     view! {
//         <div class="box-border h-94 w-64 p-4 border-0">

//             <div class="box-content h-80 w-54 border-2 border-gray-700 bg-gray-800">
//                 <img
//                     alt="Song Image"
//                     class="w-full h-48 object-cover"
//                     height="200"
//                     src="/placeholder.svg"
//                     width="200"
//                     style="aspect-ratio: 200 / 200; object-fit: cover;"
//                 />
//                 <div class="p-4">
//                     <h3 class="text-lg font-semibold">Song Title</h3>
//                     <p class="text-sm text-gray-500">Artist Name</p>
//                     <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10 mt-2">
//                         <svg
//                             xmlns="http://www.w3.org/2000/svg"
//                             width="24"
//                             height="24"
//                             viewBox="0 0 24 24"
//                             fill="none"
//                             stroke="currentColor"
//                             stroke-width="2"
//                             stroke-linecap="round"
//                             stroke-linejoin="round"
//                             class="h-6 w-6"
//                         >
//                             <polygon points="5 3 19 12 5 21 5 3"></polygon>
//                         </svg>
//                         <span class="sr-only">Play</span>
//                     </button>
//                 </div>
//             </div>
//         </div>
//     }
// }

// #[component]
// fn Everything() -> impl IntoView {
//     view! {
//         <div class="flex h-screen bg-gray-900 text-white">
//             <SidePanel/>

//             <main class="flex-1 p-6 flex flex-col">
//                 <SearchSong/>

//                 <div class="flex flex-wrap">
//                     <SongCard/>
//                     <SongCard/>
//                     <SongCard/>
//                     <SongCard/>
//                     <SongCard/>
//                     <SongCard/>
//                     <SongCard/>
//                     <SongCard/>
//                 </div>

//                 <CurrentSongPanel/>
//             </main>
//         </div>
//     }
// }

// struct Song {
//     index: u32,
//     image_url: String,
//     title: String,
//     artist: String,
//     song_length: u32,
// }

// #[component]
// fn Song_card(
//     index: u32,
//     image_url: String,
//     title: String,
//     artist: String,
//     song_length: u32,
// ) -> impl IntoView {
//     // convert song_length seconds into minutes and seconds format
//     let minutes = song_length / 60;
//     let seconds = song_length % 60;
//     let song_length = format!("{}:{}", minutes, seconds);

//     let styles = style!(
//         .song_card {
//             display: flex;
//             flex-direction: row;
//             justify-content: space-between;
//             align-items: center;
//             padding: 10px;
//             border: 1px solid black;
//             border-radius: 5px;
//             // margin: 10px;
//             background-color: green;
//             color: white;
//         }
//         .song_card_image {
//             width: 20px;
//             height: 20px;
//             border-radius: 5px;
//             // margin-right: 10px;
//         }
//     );

//     styled::view! {
//         styles,
//         <div class="song_card">
//             <div class="song_card_index">{ index }</div>
//             <img class="song_card_image" src= image_url />
//             <div class="song_card_title">{ title }</div>
//             <div class="song_card_artist">{ artist }</div>
//             <div class="song_card_length">{ song_length }</div>
//             // <button on:click= move |_| set_is_playing(!is_playing.get()) >{ if is_playing.get() { "Pause" } else { "Play" } }</button>
//         </div>
//     }
// }
// #[component]
// fn Song_Panel() -> impl IntoView {
//     // list of songs for testing
//     let songs = vec![
//         Song {
//             index: 1,
//             image_url: "https://imgs.search.brave.com/2-_CNDuVoC-zJ89WVorCJJhgQE90AMUDyv1rYDDgGUQ/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9jZG4t/aWNvbnMtcG5nLmZs/YXRpY29uLmNvbS8x/MjgvMjY1OS8yNjU5/MzYwLnBuZw".to_string(),
//             title: "Song 1".to_string(),
//             artist: "Artist 1".to_string(),
//             song_length: 136,
//         },
//         Song {
//             index: 2,
//             image_url: "https://imgs.search.brave.com/2-_CNDuVoC-zJ89WVorCJJhgQE90AMUDyv1rYDDgGUQ/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9jZG4t/aWNvbnMtcG5nLmZs/YXRpY29uLmNvbS8x/MjgvMjY1OS8yNjU5/MzYwLnBuZw".to_string(),
//             title: "Song 2".to_string(),
//             artist: "Artist 2".to_string(),
//             song_length: 10,
//         },
//         Song {
//             index: 3,
//             image_url: "https://imgs.search.brave.com/2-_CNDuVoC-zJ89WVorCJJhgQE90AMUDyv1rYDDgGUQ/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9jZG4t/aWNvbnMtcG5nLmZs/YXRpY29uLmNvbS8x/MjgvMjY1OS8yNjU5/MzYwLnBuZw".to_string(),
//             title: "Song 3".to_string(),
//             artist: "Artist 3".to_string(),
//             song_length: 543,
//         },
//     ];

//     let styles = style!(
//         padding: 10px;
//         background-color: red;
//         color: white;
//     );

//     styled::view! {
//         styles,
//         <div>
//             <p>"Song Panel"</p>

//             <div>
//             {  songs.iter().map(|song|
//                 view! {
//                     <Song_card
//                         index= song.index
//                         image_url= song.image_url.clone()
//                         title= song.title.clone()
//                         artist= song.artist.clone()
//                         song_length= song.song_length
//                     />
//                 }).collect_view()
//             }
//         </div>
//         </div>
//     }
// }

// #[component]
// fn Search() -> impl IntoView {
//     let (name, set_name) = create_signal("Controlled".to_string());

//     let set_name_fn = move |ev| {
//         // event_target_value is a Leptos helper function it functions the same way as event.target.value
//         // in JavaScript, but smooths out some of the typecasting necessary to make this work in Rust
//         // console_log(&format!("{:?}", &ev.));
//         set_name(event_target_value(&ev));
//     };

//     let styles = style!(
//         padding: 10px;
//         background-color: blue;
//         color: white;
//     );

//     styled::view! {
//         styles,
//         <div>
//             <p>"Search"</p>
//             <input type="text"
//                 on:input=set_name_fn
//                 prop:value=name // the `prop:` syntax lets you update a DOM property, rather than an attribute.
//             />
//         </div>
//     }
// }

// #[component]
// fn Current_Song_Panel() -> impl IntoView {
//     let styles = style!(
//         // padding: 10px;
//         background-color: blue;
//         color: white;
//         width: 100%;
//         // position the panel at the bottom of the screen
//         position: fixed;
//         bottom: 0;
//         left: 0;
//         height: 50px;

//     );

//     styled::view! {
//         styles,
//         <div>
//             <p>"Current Song Panel"</p>
//         </div>
//     }
// }
